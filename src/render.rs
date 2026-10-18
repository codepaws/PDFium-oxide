//! PDFium bitmap rendering and owned pixel snapshots.
use crate::{
    handles::ManagedResource,
    raw::{FPDF_BITMAP, FPDF_PAGE},
    runtime::{self, loader::LoadedLibrary, Phase},
    PdfiumError,
};
use std::{
    any::Any,
    ffi::c_void,
    ops::Deref,
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PixelFormat {
    Bgra32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelOrder {
    Bgra,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlphaSemantics {
    Straight,
}
struct PixelStorage(Mutex<Vec<u8>>);
struct BitmapInner {
    resource: ManagedResource<FPDF_BITMAP>,
    pixels: Arc<PixelStorage>,
    width: u32,
    height: u32,
    stride: usize,
    source: Option<Arc<dyn crate::callback::SourceFailureCheck>>,
}
// Bitmap handles are accessed only under the process gate; pixel allocation is stable and mutex protected.
unsafe impl Send for BitmapInner {}
// Native bitmap operations are serialized and backing storage has synchronized ownership.
unsafe impl Sync for BitmapInner {}
/// A clonable owner of a PDFium bitmap.
#[derive(Clone)]
pub struct Bitmap(Arc<BitmapInner>);
/// A borrowed, lock-backed view of a bitmap's BGRA pixel bytes.
///
/// The view holds the shared pixel-storage lock until it is dropped. It cannot
/// outlive its `Bitmap` borrow and is not `Send`.
///
/// The view cannot outlive the bitmap borrow:
///
/// ```compile_fail
/// # fn example() {
/// let view;
/// {
///     let bitmap = make_bitmap();
///     view = bitmap.pixels().unwrap();
/// }
/// drop(view);
/// # }
/// # fn make_bitmap() -> pdfium_oxide::Bitmap { unimplemented!() }
/// ```
///
/// The view is also not `Send`, even in a scoped thread:
///
/// ```compile_fail
/// # fn example(bitmap: &pdfium_oxide::Bitmap) {
/// let view = bitmap.pixels().unwrap();
/// std::thread::scope(|scope| {
///     scope.spawn(move || drop(view));
/// });
/// # }
/// ```
pub struct BitmapPixelView<'a> {
    guard: MutexGuard<'a, Vec<u8>>,
    /// Bitmap width in pixels.
    pub width: u32,
    /// Bitmap height in pixels.
    pub height: u32,
    /// Number of bytes between adjacent rows.
    pub stride: usize,
}
impl Deref for BitmapPixelView<'_> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.guard.as_slice()
    }
}
/// A stable independent copy of rendered pixels.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapSnapshot {
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub pixel_format: PixelFormat,
    pub channel_order: ChannelOrder,
    pub alpha_semantics: AlphaSemantics,
    pub pixels: Vec<u8>,
}
fn close_bitmap(ctx: &LoadedLibrary, handle: FPDF_BITMAP) {
    unsafe { ctx.dispatch.FPDFBitmap_Destroy(handle) }
}

fn checked_layout(width: u32, height: u32) -> Result<(i32, i32, usize, usize), PdfiumError> {
    if width == 0 || height == 0 {
        return Err(invalid("FPDFBitmap_CreateEx", "dimensions must be nonzero"));
    }
    let w = i32::try_from(width)
        .map_err(|_| invalid("FPDFBitmap_CreateEx", "width exceeds PDFium integer range"))?;
    let h = i32::try_from(height)
        .map_err(|_| invalid("FPDFBitmap_CreateEx", "height exceeds PDFium integer range"))?;
    let stride = (width as usize)
        .checked_mul(4)
        .ok_or(PdfiumError::Allocation {
            requested_bytes: usize::MAX,
        })?;
    i32::try_from(stride)
        .map_err(|_| invalid("FPDFBitmap_CreateEx", "stride exceeds PDFium integer range"))?;
    let len = stride
        .checked_mul(height as usize)
        .ok_or(PdfiumError::Allocation {
            requested_bytes: usize::MAX,
        })?;
    Ok((w, h, stride, len))
}
fn invalid(operation: &str, message: &str) -> PdfiumError {
    PdfiumError::InvalidInput {
        operation: operation.into(),
        message: message.into(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_zero_and_overflow_dimensions() {
        assert!(checked_layout(0, 1).is_err());
        assert!(checked_layout(1, 0).is_err());
        assert!(checked_layout(u32::MAX, 1).is_err());
        assert!(checked_layout(i32::MAX as u32, 1).is_err());
        assert_eq!(checked_layout(3, 2).unwrap(), (3, 2, 12, 24));
    }

    #[test]
    fn layout_reports_pdfium_integer_limit_errors_and_row_major_bgra_size() {
        assert!(matches!(
            checked_layout(i32::MAX as u32 + 1, 1),
            Err(PdfiumError::InvalidInput { .. })
        ));
        assert!(matches!(
            checked_layout(1, i32::MAX as u32 + 1),
            Err(PdfiumError::InvalidInput { .. })
        ));
        let (width, height, stride, len) = checked_layout(2, 3).unwrap();
        assert_eq!((width, height, stride, len), (2, 3, 8, 24));
    }

    #[test]
    fn poisoned_pixel_storage_returns_operation_error() {
        let pixels = Arc::new(PixelStorage(Mutex::new(vec![0; 4])));
        let other = pixels.clone();
        let _ = std::thread::spawn(move || {
            let _guard = other.0.lock().unwrap();
            panic!("poison pixel mutex");
        })
        .join();
        assert!(
            matches!(lock_pixels(&pixels, "Bitmap::pixels"), Err(PdfiumError::Operation { function, .. }) if function == "Bitmap::pixels")
        );
    }

    #[test]
    fn snapshot_rejects_callback_reentry_before_native_buffer_access() {
        let bitmap = fake_bitmap(vec![1, 2, 3, 4], 1, 1, 4);
        crate::runtime::with_reentry_for_test(|| {
            assert!(matches!(bitmap.snapshot(), Err(PdfiumError::ReentrantCall)));
        });
    }

    #[test]
    fn snapshot_reports_poisoned_pixel_storage() {
        let bitmap = fake_bitmap(vec![0; 4], 1, 1, 4);
        let storage = bitmap.0.pixels.clone();
        let _ = std::thread::spawn(move || {
            let _guard = storage.0.lock().unwrap();
            panic!("poison snapshot pixel mutex");
        })
        .join();
        assert!(matches!(
            bitmap.snapshot(),
            Err(PdfiumError::Operation { function, .. }) if function == "Bitmap::snapshot"
        ));
    }

    fn fake_bitmap(bytes: Vec<u8>, width: u32, height: u32, stride: usize) -> Bitmap {
        fn no_close(_: &LoadedLibrary, _: FPDF_BITMAP) {}
        let pixels = Arc::new(PixelStorage(Mutex::new(bytes)));
        let retained: Arc<dyn Any + Send + Sync> = pixels.clone();
        let resource = ManagedResource::new(std::ptr::null_mut(), no_close, vec![retained]);
        Bitmap(Arc::new(BitmapInner {
            resource,
            width,
            height,
            stride,
            pixels,
            source: None,
        }))
    }

    #[test]
    fn borrowed_view_exposes_layout_and_bytes_and_excludes_clones() {
        let bitmap = fake_bitmap(vec![1, 2, 3, 4], 1, 1, 4);
        let clone = bitmap.clone();
        let view = bitmap.pixels().unwrap();
        assert_eq!((view.width, view.height, view.stride), (1, 1, 4));
        assert_eq!(&*view, &[1, 2, 3, 4]);
        assert!(clone.0.pixels.0.try_lock().is_err());
        drop(view);
        assert!(clone.0.pixels.0.try_lock().is_ok());
    }

    #[test]
    fn callback_reentry_is_rejected_before_waiting_for_pixel_lock() {
        let pixels = Arc::new(PixelStorage(Mutex::new(vec![0; 4])));
        let held = pixels.0.lock().unwrap();
        let (send, receive) = std::sync::mpsc::channel();
        let worker_pixels = pixels.clone();
        let worker = std::thread::spawn(move || {
            crate::runtime::with_reentry_for_test(|| {
                send.send(matches!(
                    lock_pixels(&worker_pixels, "test"),
                    Err(PdfiumError::ReentrantCall)
                ))
                .unwrap();
            });
        });

        let result = receive.recv_timeout(std::time::Duration::from_secs(1));
        drop(held);
        worker.join().unwrap();
        assert!(result.unwrap());
    }
}
pub(crate) fn render_page(
    page: FPDF_PAGE,
    width: u32,
    height: u32,
    source: Option<Arc<dyn crate::callback::SourceFailureCheck>>,
) -> Result<Bitmap, PdfiumError> {
    let (w, h, stride, len) = checked_layout(width, height)?;
    let mut data = Vec::new();
    data.try_reserve_exact(len)
        .map_err(|_| PdfiumError::Allocation {
            requested_bytes: len,
        })?;
    data.resize(len, 0);
    let pixels = Arc::new(PixelStorage(Mutex::new(data)));
    let native_stride = stride as i32;
    // Acquire pixel storage before the PDFium gate. Keep this order consistent
    // with snapshots and any future native writes into the shared buffer.
    let mut bytes = lock_pixels(&pixels, "FPDFBitmap_CreateEx")?;
    let (bitmap, callback_error) = runtime::call_with_source(
        "FPDFBitmap_CreateEx",
        Phase::Initialized,
        {
            |ctx| unsafe {
                ctx.dispatch.FPDFBitmap_CreateEx(
                    w,
                    h,
                    crate::raw::FPDFBitmap_BGRA as i32,
                    bytes.as_mut_ptr().cast::<c_void>(),
                    native_stride,
                )
            }
        },
        source.as_deref(),
    )?;
    drop(bytes);
    let bitmap = runtime::source_checked_result(bitmap, callback_error, |handle| {
        if !handle.is_null() {
            runtime::defer_cleanup(move |ctx| unsafe { ctx.dispatch.FPDFBitmap_Destroy(handle) });
        }
    })?;
    if bitmap.return_value.is_null() {
        return Err(PdfiumError::Operation {
            function: "FPDFBitmap_CreateEx".into(),
            last_error: bitmap.last_error,
        });
    }
    let handle = bitmap.return_value;
    let retained: Arc<dyn Any + Send + Sync> = pixels.clone();
    let resource = ManagedResource::new(handle, close_bitmap, vec![retained]);
    let _bytes = match lock_pixels(&pixels, "FPDF_RenderPageBitmap") {
        Ok(bytes) => bytes,
        Err(error) => {
            drop(resource);
            return Err(error);
        }
    };
    let render = runtime::call_with_source(
        "FPDF_RenderPageBitmap",
        Phase::Initialized,
        |ctx| unsafe {
            ctx.dispatch
                .FPDF_RenderPageBitmap(handle, page, 0, 0, w, h, 0, 0)
        },
        source.as_deref(),
    );
    let (_, callback_error) = match render {
        Ok(result) => result,
        Err(error) => {
            drop(resource);
            return Err(error);
        }
    };
    if let Some(error) = callback_error {
        drop(resource);
        return Err(error);
    }
    drop(_bytes);
    Ok(Bitmap(Arc::new(BitmapInner {
        resource,
        pixels,
        width,
        height,
        stride,
        source,
    })))
}
impl Bitmap {
    /// Borrows the bitmap's pixel buffer, preventing crate-managed mutation
    /// while the view remains alive. Cloned `Bitmap` handles share this lock.
    pub fn pixels(&self) -> Result<BitmapPixelView<'_>, PdfiumError> {
        let guard = lock_pixels(&self.0.pixels, "Bitmap::pixels")?;
        Ok(BitmapPixelView {
            guard,
            width: self.0.width,
            height: self.0.height,
            stride: self.0.stride,
        })
    }

    /// Returns an independent copy of the bitmap's top-to-bottom BGRA bytes.
    pub fn snapshot(&self) -> Result<BitmapSnapshot, PdfiumError> {
        let handle = self.0.resource.handle();
        let n =
            self.0
                .stride
                .checked_mul(self.0.height as usize)
                .ok_or(PdfiumError::Allocation {
                    requested_bytes: usize::MAX,
                })?;
        let mut pixels = Vec::new();
        pixels
            .try_reserve_exact(n)
            .map_err(|_| PdfiumError::Allocation { requested_bytes: n })?;
        let shared_pixels = lock_pixels(&self.0.pixels, "Bitmap::snapshot")?;
        let (copied, callback_error) = runtime::call_with_source(
            "FPDFBitmap_GetBuffer",
            Phase::Initialized,
            |ctx| unsafe {
                let source = ctx.dispatch.FPDFBitmap_GetBuffer(handle);
                if source.is_null() {
                    return Err(PdfiumError::Operation {
                        function: "FPDFBitmap_GetBuffer".into(),
                        last_error: 0,
                    });
                }
                pixels.extend_from_slice(std::slice::from_raw_parts(source.cast::<u8>(), n));
                Ok(())
            },
            self.0.source.as_deref(),
        )?;
        drop(shared_pixels);
        if let Some(error) = callback_error {
            return Err(error);
        }
        copied.return_value?;
        Ok(BitmapSnapshot {
            width: self.0.width,
            height: self.0.height,
            stride: self.0.stride,
            pixel_format: PixelFormat::Bgra32,
            channel_order: ChannelOrder::Bgra,
            alpha_semantics: AlphaSemantics::Straight,
            pixels,
        })
    }
}

fn lock_pixels<'a>(
    pixels: &'a PixelStorage,
    operation: &str,
) -> Result<MutexGuard<'a, Vec<u8>>, PdfiumError> {
    // Reject calls made from a PDFium callback before waiting on a lock that a
    // callback's enclosing operation may already hold.
    runtime::check_reentry()?;
    pixels.0.lock().map_err(|_| PdfiumError::Operation {
        function: operation.into(),
        last_error: 0,
    })
}
