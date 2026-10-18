use pdfium_oxide::{Pdfium, PdfiumConfig};

/// Run with `PDFIUM_PATH=<absolute path> cargo test --test runtime_smoke -- --ignored`.
#[test]
#[ignore = "requires a matching external PDFium binary selected with PDFIUM_PATH"]
fn loads_initializes_calls_and_destroys_a_real_pdfium_library() {
    let path =
        std::env::var_os("PDFIUM_PATH").expect("set PDFIUM_PATH to the downloaded PDFium library");
    let config = PdfiumConfig::new(path).expect("PDFIUM_PATH must be absolute");
    let prepared = Pdfium::prepare(config).expect("load and validate PDFium exports");
    #[cfg(feature = "pdfium-v8")]
    let _v8_flags = unsafe { prepared.raw().FPDF_GetRecommendedV8Flags() }
        .expect("pre-initialization V8 helper");
    let pdfium = prepared.initialize().expect("initialize PDFium");
    #[cfg(feature = "pdfium-v8")]
    assert!(pdfium
        .capabilities()
        .supports(pdfium_oxide::Capability::V8Xfa));
    #[cfg(not(feature = "pdfium-v8"))]
    assert!(!pdfium
        .capabilities()
        .supports(pdfium_oxide::Capability::V8Xfa));
    let outcome = unsafe { pdfium.raw().FPDF_GetLastError() }.expect("serialized raw call");
    let _captured_error = outcome.last_error;
    let _destroyed = unsafe { pdfium.destroy_library() }.expect("terminal PDFium destruction");
    #[cfg(feature = "pdfium-v8")]
    let _allocator = unsafe {
        _destroyed
            .raw()
            .FPDF_GetArrayBufferAllocatorSharedInstance()
    }
    .expect("permitted post-destruction allocator helper");
}
