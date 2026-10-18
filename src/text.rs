//! Owned text-page handles and UTF-16 text extraction.
use crate::{
    handles::ManagedResource,
    raw::FPDF_TEXTPAGE,
    runtime::{self, loader::LoadedLibrary, Phase},
    PdfiumError,
};
use std::{any::Any, sync::Arc};

struct TextPageInner {
    resource: ManagedResource<FPDF_TEXTPAGE>,
    source: Option<Arc<dyn crate::callback::SourceFailureCheck>>,
}
// Text pointers are accessed only under the serialized PDFium coordinator.
unsafe impl Send for TextPageInner {}
// All native text operations and finalization are serialized under the coordinator.
unsafe impl Sync for TextPageInner {}
/// Clonable PDFium text page retaining its source page.
#[derive(Clone)]
pub struct TextPage(Arc<TextPageInner>);
fn close_text_page(ctx: &LoadedLibrary, handle: FPDF_TEXTPAGE) {
    unsafe { ctx.dispatch.FPDFText_ClosePage(handle) }
}
impl TextPage {
    pub(crate) fn from_raw(
        handle: FPDF_TEXTPAGE,
        keep: Vec<Arc<dyn Any + Send + Sync>>,
        source: Option<Arc<dyn crate::callback::SourceFailureCheck>>,
    ) -> Self {
        Self(Arc::new(TextPageInner {
            resource: ManagedResource::new(handle, close_text_page, keep),
            source,
        }))
    }
    /// Copies all page text into an owned UTF-8 string.
    pub fn extract_text(&self) -> Result<String, PdfiumError> {
        let handle = self.0.resource.handle();
        let (count, callback_error) = runtime::call_with_source(
            "FPDFText_CountChars",
            Phase::Initialized,
            |ctx| unsafe { ctx.dispatch.FPDFText_CountChars(handle) },
            self.0.source.as_deref(),
        )?;
        if let Some(error) = callback_error {
            return Err(error);
        }
        let count = count.return_value;
        let count = usize::try_from(count).map_err(|_| PdfiumError::InvalidInput {
            operation: "FPDFText_CountChars".into(),
            message: "PDFium returned a negative character count".into(),
        })?;
        if count == 0 {
            return Ok(String::new());
        }
        let capacity = count.checked_add(1).ok_or(PdfiumError::Allocation {
            requested_bytes: usize::MAX,
        })?;
        let requested_bytes =
            capacity
                .checked_mul(std::mem::size_of::<u16>())
                .ok_or(PdfiumError::Allocation {
                    requested_bytes: usize::MAX,
                })?;
        let mut units = Vec::<u16>::new();
        units
            .try_reserve_exact(capacity)
            .map_err(|_| PdfiumError::Allocation { requested_bytes })?;
        units.resize(capacity, 0);
        let native_count =
            i32::try_from(count).map_err(|_| PdfiumError::Allocation { requested_bytes })?;
        let (returned, callback_error) = runtime::call_with_source(
            "FPDFText_GetText",
            Phase::Initialized,
            |ctx| unsafe {
                ctx.dispatch
                    .FPDFText_GetText(handle, 0, native_count, units.as_mut_ptr())
            },
            self.0.source.as_deref(),
        )?;
        if let Some(error) = callback_error {
            return Err(error);
        }
        let returned = returned.return_value;
        decode_text(count, &units, returned)
    }
}

fn decode_text(
    character_count: usize,
    units: &[u16],
    returned: i32,
) -> Result<String, PdfiumError> {
    if character_count == 0 {
        return Ok(String::new());
    }
    let returned = usize::try_from(returned)
        .map_err(|_| text_decode_error("PDFium returned a negative UTF-16 unit count"))?;
    if returned == 0 {
        return Err(text_decode_error(
            "PDFium returned no text for a nonempty page",
        ));
    }
    if returned > units.len() {
        return Err(text_decode_error(
            "PDFium returned more UTF-16 units than the output buffer",
        ));
    }
    if units[returned - 1] != 0 {
        return Err(text_decode_error(
            "PDFium omitted the required UTF-16 terminator",
        ));
    }
    let content = &units[..returned - 1];
    let max_bytes = content
        .len()
        .checked_mul(3)
        .ok_or(PdfiumError::Allocation {
            requested_bytes: usize::MAX,
        })?;
    let mut output = String::new();
    output
        .try_reserve_exact(max_bytes)
        .map_err(|_| PdfiumError::Allocation {
            requested_bytes: max_bytes,
        })?;
    for item in char::decode_utf16(content.iter().copied()) {
        output.push(item.map_err(|error| text_decode_error(&error.to_string()))?);
    }
    Ok(output)
}
fn text_decode_error(message: &str) -> PdfiumError {
    PdfiumError::TextDecoding {
        message: message.into(),
    }
}
#[cfg(test)]
mod tests {
    use super::decode_text;
    use crate::PdfiumError;
    #[test]
    fn validates_counts_terminator_truncation_and_utf16() {
        assert_eq!(decode_text(0, &[0], 0).unwrap(), "");
        assert_eq!(decode_text(1, &[65, 0], 2).unwrap(), "A");
        assert_eq!(decode_text(1, &[65, 0, 0], 2).unwrap(), "A");
        assert!(decode_text(1, &[65, 0], 0).is_err());
        assert!(decode_text(1, &[65, 0], 3).is_err());
        assert!(decode_text(1, &[65, 66], 2).is_err());
        assert!(decode_text(1, &[0xd800, 0], 2).is_err());
    }

    #[test]
    fn rejects_negative_unit_counts_and_invalid_surrogate_pairs() {
        assert!(matches!(
            decode_text(1, &[65, 0], -1),
            Err(PdfiumError::TextDecoding { .. })
        ));
        assert!(matches!(
            decode_text(2, &[0xdc00, 0], 2),
            Err(PdfiumError::TextDecoding { .. })
        ));
        assert!(matches!(
            decode_text(2, &[65, 0xdc00, 0], 3),
            Err(PdfiumError::TextDecoding { .. })
        ));
    }

    #[test]
    fn decodes_supplementary_unicode_and_ignores_trailing_buffer_units() {
        assert_eq!(decode_text(2, &[0xd83d, 0xde03, 0, 65], 3).unwrap(), "😃");
    }
}
