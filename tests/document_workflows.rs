use pdfium_oxide::{Pdfium, PdfiumConfig, PdfiumError, Point, TextStyle};
use std::{path::PathBuf, sync::Arc};

/// Requires the matching PDFium release selected through `PDFIUM_PATH`.
#[test]
#[ignore = "requires matching external PDFium library selected with PDFIUM_PATH"]
fn complete_document_text_and_render_workflow() {
    let library = std::env::var_os("PDFIUM_PATH").expect("set PDFIUM_PATH");
    let pdfium = Pdfium::load(PdfiumConfig::new(library).unwrap()).unwrap();
    let fixture =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/document-workflow.pdf");
    {
        let doc = pdfium.open_document(&fixture, None).unwrap();
        assert_eq!(doc.page_count().unwrap(), 1);
        assert!(matches!(doc.page(1), Err(PdfiumError::InvalidInput { .. })));
        let page = doc.page(0).unwrap();
        let size = page.size().unwrap();
        assert!((size.width - 200.0).abs() < 0.1);
        assert!((size.height - 100.0).abs() < 0.1);
        assert!(page.text().unwrap().contains("PDFium Workflow Fixture"));
        let mut saved = Vec::new();
        doc.save_to_writer(&mut saved).unwrap();
        let copied = pdfium.open_document_from_bytes(saved, None).unwrap();
        assert_eq!(copied.page_count().unwrap(), 1);
        assert!(copied
            .page(0)
            .unwrap()
            .text()
            .unwrap()
            .contains("PDFium Workflow Fixture"));
        drop(copied);

        struct FailingWriter;
        impl std::io::Write for FailingWriter {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("test writer failure"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        assert!(matches!(
            doc.save_to_writer(&mut FailingWriter),
            Err(PdfiumError::CallbackWrite { .. })
        ));
        struct PanickingWriter;
        impl std::io::Write for PanickingWriter {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                panic!("test callback panic")
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        assert!(matches!(
            doc.save_to_writer(&mut PanickingWriter),
            Err(PdfiumError::CallbackPanic { .. })
        ));

        // The page is owned only by the writer. Dropping it from PDFium's
        // synchronous write callback must defer FPDF_ClosePage until the
        // enclosing save call has returned to the coordinator epilogue.
        struct DropPageWriter(Option<pdfium_oxide::Page>);
        impl std::io::Write for DropPageWriter {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                drop(self.0.take());
                Err(std::io::Error::other("failure after dropping page"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let callback_page = doc.page(0).unwrap();
        let mut drop_page_writer = DropPageWriter(Some(callback_page));
        assert!(matches!(
            doc.save_to_writer(&mut drop_page_writer),
            Err(PdfiumError::CallbackWrite { ref message, .. })
                if message.contains("failure after dropping page")
        ));
        assert!(drop_page_writer.0.is_none());
        assert_eq!(doc.page_count().unwrap(), 1);

        struct DropPageThenPanicWriter(Option<pdfium_oxide::Page>);
        impl std::io::Write for DropPageThenPanicWriter {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                drop(self.0.take());
                panic!("panic after dropping page");
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let callback_page = doc.page(0).unwrap();
        let mut drop_page_then_panic = DropPageThenPanicWriter(Some(callback_page));
        assert!(matches!(
            doc.save_to_writer(&mut drop_page_then_panic),
            Err(PdfiumError::CallbackPanic { ref message, .. })
                if message.contains("panic after dropping page")
        ));
        assert!(drop_page_then_panic.0.is_none());
        assert_eq!(doc.page_count().unwrap(), 1);

        let bitmap = page.render(200, 100).unwrap();
        let bitmap_clone = bitmap.clone();
        let view = bitmap_clone.pixels().unwrap();
        assert_eq!((view.width, view.height, view.stride), (200, 100, 800));
        let view_bytes = view.to_vec();
        drop(view);
        let snap = bitmap.snapshot().unwrap();
        assert_eq!(view_bytes, snap.pixels);
        assert_eq!((snap.width, snap.height, snap.stride), (200, 100, 800));
        assert_eq!(snap.pixel_format, pdfium_oxide::PixelFormat::Bgra32);
        assert_eq!(snap.channel_order, pdfium_oxide::ChannelOrder::Bgra);
        assert_eq!(snap.alpha_semantics, pdfium_oxide::AlphaSemantics::Straight);
        assert_eq!(
            &snap.pixels[(75 * 800 + 40 * 4)..(75 * 800 + 40 * 4 + 4)],
            &[0, 0, 255, 255]
        );
        let blue = &snap.pixels[(75 * 800 + 100 * 4)..(75 * 800 + 100 * 4 + 4)];
        assert!(blue[0] > 240 && blue[1] < 10 && blue[2] < 10 && blue[3] == 255);
        let translucent_green = &snap.pixels[(75 * 800 + 160 * 4)..(75 * 800 + 160 * 4 + 4)];
        assert!(
            translucent_green[1] >= 247,
            "green channel should remain unpremultiplied: {translucent_green:?}"
        );
        assert!(
            translucent_green[3] >= 120 && translucent_green[3] <= 136,
            "half opacity expected: {translucent_green:?}"
        );
        assert_eq!(snap.pixels[0..4], [0, 0, 0, 0]);
        drop(snap);
        drop(bitmap_clone);
        drop(bitmap);

        // All native document workflows share this single ignored test so
        // their process-global PDFium lifecycle cannot race with each other.
        let edit_doc = pdfium.open_document(&fixture, None).unwrap();
        edit_doc
            .page(0)
            .unwrap()
            .insert_text(
                "Inserted PDFium text",
                Point { x: 20.0, y: 20.0 },
                TextStyle {
                    standard_font: "Helvetica".into(),
                    font_size: 12.0,
                    fill_rgba: [0, 0, 0, 255],
                },
            )
            .unwrap();
        let mut edited = Vec::new();
        edit_doc.save_to_writer(&mut edited).unwrap();
        let reloaded = pdfium.open_document_from_bytes(edited, None).unwrap();
        assert!(reloaded
            .page(0)
            .unwrap()
            .text()
            .unwrap()
            .contains("Inserted PDFium text"));
        drop(reloaded);
        drop(edit_doc);
    }
    let encrypted = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/document-workflow-encrypted.pdf");
    assert!(matches!(
        pdfium.open_document_from_bytes(b"not a PDF".to_vec(), None),
        Err(PdfiumError::DocumentLoad { .. })
    ));
    let bytes: Arc<[u8]> = std::fs::read(&fixture).unwrap().into();
    let memory_doc = pdfium
        .open_document_from_bytes(bytes.clone(), None)
        .unwrap();
    drop(bytes);
    let page = memory_doc.page(0).unwrap();
    let text = page.text_page().unwrap();
    drop(memory_doc);
    drop(page);
    assert!(text
        .extract_text()
        .unwrap()
        .contains("PDFium Workflow Fixture"));
    drop(text);
    assert!(matches!(
        pdfium.open_document(&encrypted, None),
        Err(PdfiumError::Password { .. })
    ));
    let encrypted_bytes = std::fs::read(&encrypted).unwrap();
    assert!(matches!(
        pdfium.open_document_from_bytes(encrypted_bytes.clone(), None),
        Err(PdfiumError::Password { .. })
    ));
    assert!(matches!(
        pdfium.open_document(&encrypted, Some("wrong")),
        Err(PdfiumError::Password { .. })
    ));
    let secured = pdfium.open_document(&encrypted, Some("secret")).unwrap();
    assert_eq!(secured.page_count().unwrap(), 1);
    drop(secured);
    struct TrackedSource {
        cursor: std::io::Cursor<Vec<u8>>,
        dropped: Arc<std::sync::atomic::AtomicBool>,
    }
    impl std::io::Read for TrackedSource {
        fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
            std::io::Read::read(&mut self.cursor, out)
        }
    }
    impl std::io::Seek for TrackedSource {
        fn seek(&mut self, from: std::io::SeekFrom) -> std::io::Result<u64> {
            std::io::Seek::seek(&mut self.cursor, from)
        }
    }
    impl Drop for TrackedSource {
        fn drop(&mut self) {
            self.dropped
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
    let source_bytes = std::fs::read(&fixture).unwrap();
    struct FailingSource;
    impl std::io::Read for FailingSource {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("test source read failure"))
        }
    }
    impl std::io::Seek for FailingSource {
        fn seek(&mut self, _: std::io::SeekFrom) -> std::io::Result<u64> {
            Ok(0)
        }
    }
    assert!(matches!(
        pdfium.open_document_from_source(FailingSource, source_bytes.len() as u64, None),
        Err(PdfiumError::CallbackRead { ref message, .. })
            if message.contains("test source read failure")
    ));
    let dropped = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let source_doc = pdfium
        .open_document_from_source(
            TrackedSource {
                cursor: std::io::Cursor::new(source_bytes.clone()),
                dropped: dropped.clone(),
            },
            source_bytes.len() as u64,
            None,
        )
        .unwrap();
    assert!(source_doc
        .page(0)
        .unwrap()
        .text()
        .unwrap()
        .contains("PDFium Workflow Fixture"));
    let source_clone = source_doc.clone();
    drop(source_doc);
    assert!(!dropped.load(std::sync::atomic::Ordering::SeqCst));
    drop(source_clone);
    assert!(dropped.load(std::sync::atomic::Ordering::SeqCst));
    // SAFETY: every managed document, page, text page, and bitmap above has been dropped.
    unsafe { pdfium.destroy_library() }.unwrap();
}
