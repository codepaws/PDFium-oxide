//! Safe page access.
use crate::{
    handles::ManagedResource,
    raw::{FPDF_DOCUMENT, FPDF_PAGE, FPDF_PAGEOBJECT, FS_MATRIX},
    runtime::{self, loader::LoadedLibrary, Phase},
    PdfiumError,
};
use std::{any::Any, ffi::CString, sync::Arc};

struct PageInner {
    resource: ManagedResource<FPDF_PAGE>,
    document: FPDF_DOCUMENT,
    source: Option<Arc<dyn crate::callback::SourceFailureCheck>>,
}
// PDFium page pointers are accessed only under the process coordinator.
unsafe impl Send for PageInner {}
// All page operations and finalization are serialized under the process coordinator.
unsafe impl Sync for PageInner {}
/// A clonable owner of a loaded PDFium page.
#[derive(Clone)]
pub struct Page(Arc<PageInner>);
/// Dimensions of a page in PDF points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageSize {
    pub width: f32,
    pub height: f32,
}
/// A point in PDF page coordinates, measured in points from the lower left.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
/// Styling for inserted page text.
#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
    /// One of PDF's 14 standard font names, such as `Helvetica`.
    pub standard_font: String,
    /// Positive font size in PDF points.
    pub font_size: f32,
    /// Red, green, blue, and alpha channels.
    pub fill_rgba: [u8; 4],
}

struct PendingObject<H, F: FnOnce(H)> {
    handle: Option<H>,
    close: Option<F>,
}
impl<H, F: FnOnce(H)> PendingObject<H, F> {
    fn new(handle: H, close: F) -> Self {
        Self {
            handle: Some(handle),
            close: Some(close),
        }
    }
    fn take_for_insert(&mut self) -> H {
        self.handle.take().expect("live page object")
    }
}
// The pinned API consumes the object on every executed insertion call,
// including a call that returns false. Disarm before crossing the C boundary.
fn insert_object_consuming<H, F: FnOnce(H), R>(
    object: &mut PendingObject<H, F>,
    native_insert: impl FnOnce(H) -> R,
) -> R {
    native_insert(object.take_for_insert())
}
impl<H, F: FnOnce(H)> Drop for PendingObject<H, F> {
    fn drop(&mut self) {
        if let (Some(handle), Some(close)) = (self.handle.take(), self.close.take()) {
            close(handle);
        }
    }
}

fn invalid_insert(message: &str) -> PdfiumError {
    PdfiumError::InvalidInput {
        operation: "Page::insert_text".into(),
        message: message.into(),
    }
}
fn validate_text_insert(text: &str, position: Point, style: &TextStyle) -> Result<(), PdfiumError> {
    if text.is_empty() || text.contains('\0') {
        return Err(invalid_insert("text must be nonempty and contain no NUL"));
    }
    if !position.x.is_finite() || !position.y.is_finite() {
        return Err(invalid_insert("coordinates must be finite"));
    }
    if !style.font_size.is_finite() || style.font_size <= 0.0 {
        return Err(invalid_insert("font size must be finite and positive"));
    }
    const FONTS: [&str; 14] = [
        "Courier",
        "Courier-Bold",
        "Courier-Oblique",
        "Courier-BoldOblique",
        "Helvetica",
        "Helvetica-Bold",
        "Helvetica-Oblique",
        "Helvetica-BoldOblique",
        "Times-Roman",
        "Times-Bold",
        "Times-Italic",
        "Times-BoldItalic",
        "Symbol",
        "ZapfDingbats",
    ];
    if !FONTS.contains(&style.standard_font.as_str()) {
        return Err(invalid_insert("unsupported standard font"));
    }
    Ok(())
}
fn operation_error(function: &str, last_error: u64) -> PdfiumError {
    PdfiumError::Operation {
        function: function.into(),
        last_error,
    }
}
fn close_page(ctx: &LoadedLibrary, handle: FPDF_PAGE) {
    unsafe { ctx.dispatch.FPDF_ClosePage(handle) }
}
impl Page {
    /// Inserts text into this page and regenerates its native content stream.
    ///
    /// This mutates the page in place. Other safe crate operations, including
    /// saving the owning document with [`crate::Document::save_to_writer`],
    /// cannot interleave with the insertion sequence. Failure after object
    /// insertion can still leave the page changed; this is serialized, but it
    /// is not a transaction and provides no rollback guarantee.
    pub fn insert_text(
        &self,
        text: &str,
        position: Point,
        style: TextStyle,
    ) -> Result<(), PdfiumError> {
        validate_text_insert(text, position, &style)?;
        let font_name = CString::new(style.standard_font).expect("validated standard font");
        let mut wide: Vec<u16> = text.encode_utf16().collect();
        wide.push(0);
        let document = self.0.document;
        let source = self.0.source.as_deref();
        runtime::call_sequence(Phase::Initialized, source, |calls| {
            let (font_call, source_error) =
                calls.call("FPDFText_LoadStandardFont", |ctx| unsafe {
                    ctx.dispatch
                        .FPDFText_LoadStandardFont(document, font_name.as_ptr())
                })?;
            let font_call = runtime::source_checked_result(font_call, source_error, |handle| {
                if !handle.is_null() {
                    runtime::defer_cleanup(move |ctx| unsafe {
                        ctx.dispatch.FPDFFont_Close(handle)
                    });
                }
            })?;
            if font_call.return_value.is_null() {
                return Err(operation_error(
                    "FPDFText_LoadStandardFont",
                    font_call.last_error,
                ));
            }
            let parent: Arc<dyn Any + Send + Sync> = self.0.clone();
            let font = ManagedResource::new(
                font_call.return_value,
                |ctx, handle| unsafe { ctx.dispatch.FPDFFont_Close(handle) },
                vec![parent],
            );
            let (object_call, source_error) =
                calls.call("FPDFPageObj_CreateTextObj", |ctx| unsafe {
                    ctx.dispatch
                        .FPDFPageObj_CreateTextObj(document, font.handle(), style.font_size)
                })?;
            let object_handle = object_call.return_value;
            let parent = self.0.clone();
            let mut object = if object_handle.is_null() {
                None
            } else {
                Some(PendingObject::new(
                    object_handle,
                    move |handle: FPDF_PAGEOBJECT| {
                        runtime::defer_cleanup(move |ctx| unsafe {
                            ctx.dispatch.FPDFPageObj_Destroy(handle)
                        });
                        drop(parent);
                    },
                ))
            };
            if let Some(error) = source_error {
                return Err(error);
            }
            if object_handle.is_null() {
                return Err(operation_error(
                    "FPDFPageObj_CreateTextObj",
                    object_call.last_error,
                ));
            }
            let check = |name: &'static str,
                         call: crate::PdfiumCall<i32>,
                         callback: Option<PdfiumError>|
             -> Result<(), PdfiumError> {
                if let Some(error) = callback {
                    return Err(error);
                }
                if call.return_value == 0 {
                    return Err(operation_error(name, call.last_error));
                }
                Ok(())
            };
            let (call, callback) = calls.call("FPDFText_SetText", |ctx| unsafe {
                ctx.dispatch.FPDFText_SetText(object_handle, wide.as_ptr())
            })?;
            check("FPDFText_SetText", call, callback)?;
            let [r, g, b, a] = style.fill_rgba;
            let (call, callback) = calls.call("FPDFPageObj_SetFillColor", |ctx| unsafe {
                ctx.dispatch.FPDFPageObj_SetFillColor(
                    object_handle,
                    r.into(),
                    g.into(),
                    b.into(),
                    a.into(),
                )
            })?;
            check("FPDFPageObj_SetFillColor", call, callback)?;
            let matrix = FS_MATRIX {
                a: 1.0,
                b: 0.0,
                c: 0.0,
                d: 1.0,
                e: position.x,
                f: position.y,
            };
            let (call, callback) = calls.call("FPDFPageObj_SetMatrix", |ctx| unsafe {
                ctx.dispatch.FPDFPageObj_SetMatrix(object_handle, &matrix)
            })?;
            check("FPDFPageObj_SetMatrix", call, callback)?;
            let page = self.0.resource.handle();
            let (call, callback) = calls.call("FPDFPage_InsertObject", |ctx| {
                insert_object_consuming(object.as_mut().expect("live object"), |handle| unsafe {
                    ctx.dispatch.FPDFPage_InsertObject(page, handle)
                })
            })?;
            check("FPDFPage_InsertObject", call, callback)?;
            let (call, callback) = calls.call("FPDFPage_GenerateContent", |ctx| unsafe {
                ctx.dispatch.FPDFPage_GenerateContent(page)
            })?;
            check("FPDFPage_GenerateContent", call, callback)?;
            Ok(())
        })
    }
    /// Loads a text page that keeps this page alive.
    pub fn text_page(&self) -> Result<crate::TextPage, PdfiumError> {
        let page = self.0.resource.handle();
        let (call, callback_error) = runtime::call_with_source(
            "FPDFText_LoadPage",
            Phase::Initialized,
            |ctx| unsafe { ctx.dispatch.FPDFText_LoadPage(page) },
            self.0.source.as_deref(),
        )?;
        let parent_owner = self.0.clone();
        let source = self.0.source.clone();
        let call = runtime::source_checked_result(call, callback_error, move |handle| {
            if !handle.is_null() {
                let parent: Arc<dyn Any + Send + Sync> = parent_owner;
                drop(crate::TextPage::from_raw(handle, vec![parent], source));
            }
        })?;
        if call.return_value.is_null() {
            return Err(PdfiumError::Operation {
                function: "FPDFText_LoadPage".into(),
                last_error: call.last_error,
            });
        }
        let parent: Arc<dyn Any + Send + Sync> = self.0.clone();
        Ok(crate::TextPage::from_raw(
            call.return_value,
            vec![parent],
            self.0.source.clone(),
        ))
    }
    /// Extracts all text from the page.
    pub fn text(&self) -> Result<String, PdfiumError> {
        self.text_page()?.extract_text()
    }
    /// Renders the page scaled to the requested dimensions.
    pub fn render(&self, width: u32, height: u32) -> Result<crate::Bitmap, PdfiumError> {
        crate::render::render_page(
            self.0.resource.handle(),
            width,
            height,
            self.0.source.clone(),
        )
    }

    pub(crate) fn from_raw(
        handle: FPDF_PAGE,
        document: FPDF_DOCUMENT,
        keep: Vec<Arc<dyn Any + Send + Sync>>,
        source: Option<Arc<dyn crate::callback::SourceFailureCheck>>,
    ) -> Self {
        Self(Arc::new(PageInner {
            resource: ManagedResource::new(handle, close_page, keep),
            document,
            source,
        }))
    }
    /// Returns page dimensions in PDF points.
    pub fn size(&self) -> Result<PageSize, PdfiumError> {
        let h = self.0.resource.handle();
        let (w, callback_error) = runtime::call_with_source(
            "FPDF_GetPageWidthF",
            Phase::Initialized,
            |ctx| unsafe { ctx.dispatch.FPDF_GetPageWidthF(h) },
            self.0.source.as_deref(),
        )?;
        if let Some(error) = callback_error {
            return Err(error);
        }
        let w = w.return_value;
        let (height, callback_error) = runtime::call_with_source(
            "FPDF_GetPageHeightF",
            Phase::Initialized,
            |ctx| unsafe { ctx.dispatch.FPDF_GetPageHeightF(h) },
            self.0.source.as_deref(),
        )?;
        if let Some(error) = callback_error {
            return Err(error);
        }
        let height = height.return_value;
        if !w.is_finite() || !height.is_finite() || w <= 0.0 || height <= 0.0 {
            return Err(PdfiumError::InvalidInput {
                operation: "FPDF_GetPageWidthF/HeightF".into(),
                message: "PDFium returned non-positive or non-finite dimensions".into(),
            });
        }
        Ok(PageSize { width: w, height })
    }
}

#[cfg(test)]
mod insertion_tests {
    use super::{insert_object_consuming, validate_text_insert, PendingObject, Point, TextStyle};
    use crate::PdfiumError;
    use std::sync::{Arc, Mutex};

    fn style() -> TextStyle {
        TextStyle {
            standard_font: "Helvetica".into(),
            font_size: 12.0,
            fill_rgba: [1, 2, 3, 255],
        }
    }

    #[test]
    fn rejects_invalid_text_inputs_before_native_calls() {
        let point = Point { x: 10.0, y: 20.0 };
        for (text, position, selected_style) in [
            ("", point, style()),
            ("a\0b", point, style()),
            (
                "a",
                Point {
                    x: f32::NAN,
                    y: 20.0,
                },
                style(),
            ),
            (
                "a",
                Point {
                    x: 10.0,
                    y: f32::INFINITY,
                },
                style(),
            ),
            (
                "a",
                point,
                TextStyle {
                    font_size: 0.0,
                    ..style()
                },
            ),
            (
                "a",
                point,
                TextStyle {
                    font_size: f32::NAN,
                    ..style()
                },
            ),
            (
                "a",
                point,
                TextStyle {
                    standard_font: "Custom Font".into(),
                    ..style()
                },
            ),
        ] {
            assert!(matches!(
                validate_text_insert(text, position, &selected_style),
                Err(PdfiumError::InvalidInput { .. })
            ));
        }
        assert!(validate_text_insert("hello", point, &style()).is_ok());
    }

    #[test]
    fn accepts_each_pdf_standard_font_name() {
        let point = Point { x: 0.0, y: 0.0 };
        for font in [
            "Courier",
            "Courier-Bold",
            "Courier-Oblique",
            "Courier-BoldOblique",
            "Helvetica",
            "Helvetica-Bold",
            "Helvetica-Oblique",
            "Helvetica-BoldOblique",
            "Times-Roman",
            "Times-Bold",
            "Times-Italic",
            "Times-BoldItalic",
            "Symbol",
            "ZapfDingbats",
        ] {
            let selected_style = TextStyle {
                standard_font: font.into(),
                ..style()
            };
            assert!(
                validate_text_insert("x", point, &selected_style).is_ok(),
                "{font}"
            );
        }
    }

    #[test]
    fn rejects_negative_infinite_and_negative_font_sizes() {
        let point = Point { x: 0.0, y: 0.0 };
        for font_size in [-1.0, f32::NEG_INFINITY, f32::INFINITY] {
            let selected_style = TextStyle {
                font_size,
                ..style()
            };
            assert!(matches!(
                validate_text_insert("x", point, &selected_style),
                Err(PdfiumError::InvalidInput { .. })
            ));
        }
    }

    #[test]
    fn pending_object_is_consumed_by_any_native_insert_attempt() {
        let closed = Arc::new(Mutex::new(Vec::new()));
        {
            let observed = closed.clone();
            let _failed_before_insert =
                PendingObject::new(7, move |handle| observed.lock().unwrap().push(handle));
        }
        for native_result in [false, true] {
            let observed = closed.clone();
            let mut pending =
                PendingObject::new(9, move |handle| observed.lock().unwrap().push(handle));
            let called = insert_object_consuming(&mut pending, |handle| {
                assert_eq!(handle, 9);
                native_result
            });
            assert_eq!(called, native_result);
            drop(pending);
        }
        assert_eq!(*closed.lock().unwrap(), [7]);
    }
}
