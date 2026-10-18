//! Safe document loading and page access.
use crate::{
    handles::ManagedResource,
    raw::FPDF_DOCUMENT,
    runtime::{self, loader::LoadedLibrary, Phase},
    Pdfium, PdfiumError,
};
use std::{
    any::Any,
    ffi::CString,
    io::{Read, Seek, Write},
    os::raw::c_ulong,
    path::Path,
    ptr,
};

struct DocumentInner {
    resource: ManagedResource<FPDF_DOCUMENT>,
    source: Option<Arc<dyn crate::callback::SourceFailureCheck>>,
}
#[allow(dead_code)] // Retained solely to keep PDFium's memory-document input alive.
struct ByteBacking(Arc<[u8]>);
// PDFium pointers are only accessed under runtime's process gate; ownership is immutable.
unsafe impl Send for DocumentInner {}
// All native access and finalization are serialized by runtime's process gate.
unsafe impl Sync for DocumentInner {}

/// A clonable owner of a PDFium document.
#[derive(Clone)]
pub struct Document(Arc<DocumentInner>);
use std::sync::Arc;
fn close_document(ctx: &LoadedLibrary, handle: FPDF_DOCUMENT) {
    unsafe { ctx.dispatch.FPDF_CloseDocument(handle) }
}

impl Pdfium {
    /// Opens a document from a UTF-8 path.
    pub fn open_document(
        &self,
        path: impl AsRef<Path>,
        password: Option<&str>,
    ) -> Result<Document, PdfiumError> {
        let path = path_to_cstring(path.as_ref())?;
        let password = password_to_cstring(password, "FPDF_LoadDocument")?;
        let call = runtime::call("FPDF_LoadDocument", Phase::Initialized, |ctx| unsafe {
            ctx.dispatch.FPDF_LoadDocument(
                path.as_ptr(),
                password.as_ref().map_or(ptr::null(), |p| p.as_ptr()),
            )
        })?;
        if call.return_value.is_null() {
            return Err(load_error("FPDF_LoadDocument", call.last_error));
        }
        Ok(Document(Arc::new(DocumentInner {
            resource: ManagedResource::new(call.return_value, close_document, vec![]),
            source: None,
        })))
    }

    /// Opens bytes while retaining their storage through the native document lifetime.
    pub fn open_document_from_bytes(
        &self,
        bytes: impl Into<Arc<[u8]>>,
        password: Option<&str>,
    ) -> Result<Document, PdfiumError> {
        let bytes = bytes.into();
        let password = password_to_cstring(password, "FPDF_LoadMemDocument64")?;
        let call = runtime::call("FPDF_LoadMemDocument64", Phase::Initialized, |ctx| unsafe {
            ctx.dispatch.FPDF_LoadMemDocument64(
                bytes.as_ptr().cast(),
                bytes.len(),
                password.as_ref().map_or(ptr::null(), |p| p.as_ptr()),
            )
        })?;
        if call.return_value.is_null() {
            return Err(load_error("FPDF_LoadMemDocument64", call.last_error));
        }
        let backing: Arc<dyn Any + Send + Sync> = Arc::new(ByteBacking(bytes));
        Ok(Document(Arc::new(DocumentInner {
            resource: ManagedResource::new(call.return_value, close_document, vec![backing]),
            source: None,
        })))
    }

    /// Opens a document through a persistent random-access source.
    ///
    /// Source callbacks run synchronously while the process-wide PDFium gate is held.
    /// A source must not wait for another thread that calls PDFium.
    pub fn open_document_from_source<S: Read + Seek + Send + 'static>(
        &self,
        source: S,
        file_len: u64,
        password: Option<&str>,
    ) -> Result<Document, PdfiumError> {
        const OP: &str = "FPDF_LoadCustomDocument";
        let native_len = checked_file_len(file_len)?;
        let password = password_to_cstring(password, OP)?;
        let context = crate::callback::SourceContext::new(source, native_len);
        let interface = &context.interface as *const crate::raw::FPDF_FILEACCESS
            as *mut crate::raw::FPDF_FILEACCESS;
        let (call, callback_error) = runtime::call_with_source(
            OP,
            Phase::Initialized,
            |ctx| unsafe {
                ctx.dispatch.FPDF_LoadCustomDocument(
                    interface,
                    password.as_ref().map_or(ptr::null(), |p| p.as_ptr()),
                )
            },
            Some(context.as_ref() as &dyn crate::callback::SourceFailureCheck),
        )?;
        let owner: Arc<dyn crate::callback::SourceFailureCheck> = context.clone();
        let call = runtime::source_checked_result(call, callback_error, |handle| {
            if !handle.is_null() {
                runtime::defer_cleanup(move |ctx| unsafe {
                    ctx.dispatch.FPDF_CloseDocument(handle)
                });
            }
        })?;
        if call.return_value.is_null() {
            return Err(load_error(OP, call.last_error));
        }
        let retained: Arc<dyn Any + Send + Sync> = context;
        Ok(Document(Arc::new(DocumentInner {
            resource: ManagedResource::new(call.return_value, close_document, vec![retained]),
            source: Some(owner),
        })))
    }
}
fn invalid(operation: &str, message: &str) -> PdfiumError {
    PdfiumError::InvalidInput {
        operation: operation.into(),
        message: message.into(),
    }
}

fn checked_file_len(file_len: u64) -> Result<c_ulong, PdfiumError> {
    c_ulong::try_from(file_len).map_err(|_| {
        invalid(
            "FPDF_LoadCustomDocument",
            "file length exceeds PDFium's c_ulong range",
        )
    })
}

fn path_to_cstring(path: &Path) -> Result<CString, PdfiumError> {
    let path = path
        .to_str()
        .ok_or_else(|| invalid("FPDF_LoadDocument", "path is not valid UTF-8"))?;
    CString::new(path).map_err(|_| invalid("FPDF_LoadDocument", "path contains an interior NUL"))
}

fn password_to_cstring(
    password: Option<&str>,
    operation: &str,
) -> Result<Option<CString>, PdfiumError> {
    password
        .map(CString::new)
        .transpose()
        .map_err(|_| invalid(operation, "password contains an interior NUL"))
}

fn page_index_to_native(index: usize) -> Result<i32, PdfiumError> {
    i32::try_from(index)
        .map_err(|_| invalid("FPDF_LoadPage", "page index exceeds PDFium's integer range"))
}

fn load_error(operation: &str, code: u64) -> PdfiumError {
    if code == crate::raw::FPDF_ERR_PASSWORD as u64 {
        PdfiumError::Password {
            operation: operation.into(),
            last_error: code,
        }
    } else {
        PdfiumError::DocumentLoad {
            operation: operation.into(),
            last_error: code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        checked_file_len, load_error, page_index_to_native, password_to_cstring, path_to_cstring,
    };
    use crate::PdfiumError;
    use std::os::raw::c_ulong;
    use std::path::Path;

    #[test]
    fn rejects_interior_nuls_in_document_path_and_password() {
        assert!(matches!(
            path_to_cstring(Path::new("file\0name.pdf")),
            Err(PdfiumError::InvalidInput { .. })
        ));
        assert!(matches!(
            password_to_cstring(Some("secret\0suffix"), "FPDF_LoadDocument"),
            Err(PdfiumError::InvalidInput { .. })
        ));
    }

    #[test]
    fn accepts_valid_document_paths_and_absent_passwords() {
        assert_eq!(
            path_to_cstring(Path::new("document.pdf"))
                .unwrap()
                .to_bytes(),
            b"document.pdf"
        );
        assert!(password_to_cstring(None, "FPDF_LoadDocument")
            .unwrap()
            .is_none());
    }

    #[test]
    fn native_load_errors_preserve_password_and_other_failure_categories() {
        let password_code = crate::raw::FPDF_ERR_PASSWORD as u64;
        assert!(matches!(
            load_error("FPDF_LoadDocument", password_code),
            PdfiumError::Password { ref operation, last_error }
                if operation == "FPDF_LoadDocument" && last_error == password_code
        ));

        assert!(matches!(
            load_error("FPDF_LoadMemDocument64", 7),
            PdfiumError::DocumentLoad { ref operation, last_error }
                if operation == "FPDF_LoadMemDocument64" && last_error == 7
        ));
    }

    #[test]
    fn rejects_non_utf8_document_paths() {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            let path = Path::new(std::ffi::OsStr::from_bytes(b"invalid-\xff.pdf"));
            assert!(matches!(
                path_to_cstring(path),
                Err(PdfiumError::InvalidInput { .. })
            ));
        }
        #[cfg(windows)]
        {
            use std::{ffi::OsString, os::windows::ffi::OsStringExt};
            let invalid_path = OsString::from_wide(&[0xD800]);
            let path = Path::new(&invalid_path);
            assert!(matches!(
                path_to_cstring(path),
                Err(PdfiumError::InvalidInput { .. })
            ));
        }
    }

    #[test]
    fn page_index_conversion_checks_pdfiums_integer_range() {
        assert_eq!(page_index_to_native(i32::MAX as usize).unwrap(), i32::MAX);
        assert!(matches!(
            page_index_to_native(usize::MAX),
            Err(PdfiumError::InvalidInput { .. })
        ));
    }

    #[test]
    fn source_file_length_conversion_checks_native_range() {
        assert_eq!(checked_file_len(c_ulong::MAX as u64).unwrap(), c_ulong::MAX);
        if (c_ulong::MAX as u128) < (u64::MAX as u128) {
            assert!(matches!(
                checked_file_len(c_ulong::MAX as u64 + 1),
                Err(PdfiumError::InvalidInput { .. })
            ));
        }
    }
}
impl Document {
    /// Writes a copy of this document to a caller-provided writer.
    ///
    /// The writer is borrowed only during this synchronous call. PDFium invokes
    /// the adapter while holding the crate's process-wide native-call gate; a
    /// writer must not synchronously wait for another thread that calls PDFium.
    /// Same-thread PDFium re-entry is rejected with [`PdfiumError::ReentrantCall`].
    /// A failed save may leave partial bytes in the writer. Writer panics are
    /// converted to [`PdfiumError::CallbackPanic`] when built with unwind
    /// panics; with `panic=abort`, the process aborts instead. This guarantee
    /// covers this crate-owned adapter only, not raw callbacks or native faults.
    pub fn save_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), PdfiumError> {
        const OPERATION: &str = "FPDF_SaveAsCopy";
        let mut context = crate::callback::WriteContext::new(writer);
        let file_write = context.interface_ptr();
        let document = self.0.resource.handle();
        let (call, callback_error) = runtime::call_with_source(
            OPERATION,
            Phase::Initialized,
            |ctx| unsafe { ctx.dispatch.FPDF_SaveAsCopy(document, file_write, 0) },
            self.0.source.as_deref(),
        )?;
        if let Some(error) = callback_error {
            return Err(error);
        }
        match context.take_failure() {
            Some(crate::callback::WriterFailure::Panic(message)) => {
                return Err(PdfiumError::CallbackPanic {
                    operation: OPERATION.into(),
                    message,
                });
            }
            Some(crate::callback::WriterFailure::Io(error)) => {
                return Err(PdfiumError::CallbackWrite {
                    operation: OPERATION.into(),
                    message: crate::callback::io_error_message(error),
                });
            }
            Some(crate::callback::WriterFailure::Adapter(message)) => {
                return Err(PdfiumError::CallbackWrite {
                    operation: OPERATION.into(),
                    message,
                });
            }
            None => {}
        }
        if call.return_value == 0 {
            return Err(PdfiumError::Operation {
                function: OPERATION.into(),
                last_error: call.last_error,
            });
        }
        Ok(())
    }

    /// Returns the number of pages in the document.
    pub fn page_count(&self) -> Result<usize, PdfiumError> {
        let handle = self.0.resource.handle();
        let (call, callback_error) = runtime::call_with_source(
            "FPDF_GetPageCount",
            Phase::Initialized,
            |ctx| unsafe { ctx.dispatch.FPDF_GetPageCount(handle) },
            self.0.source.as_deref(),
        )?;
        if let Some(error) = callback_error {
            return Err(error);
        }
        usize::try_from(call.return_value)
            .map_err(|_| invalid("FPDF_GetPageCount", "PDFium returned a negative page count"))
    }
    /// Loads a page by zero-based index.
    pub fn page(&self, index: usize) -> Result<crate::Page, PdfiumError> {
        let native_index = page_index_to_native(index)?;
        if index >= self.page_count()? {
            return Err(invalid("FPDF_LoadPage", "page index is out of range"));
        }
        let handle = self.0.resource.handle();
        let (call, callback_error) = runtime::call_with_source(
            "FPDF_LoadPage",
            Phase::Initialized,
            |ctx| unsafe { ctx.dispatch.FPDF_LoadPage(handle, native_index) },
            self.0.source.as_deref(),
        )?;
        let parent_owner = self.0.clone();
        let source = self.0.source.clone();
        let call = runtime::source_checked_result(call, callback_error, move |handle| {
            if !handle.is_null() {
                let parent: Arc<dyn Any + Send + Sync> = parent_owner;
                drop(crate::Page::from_raw(
                    handle,
                    self.0.resource.handle(),
                    vec![parent],
                    source,
                ));
            }
        })?;
        if call.return_value.is_null() {
            return Err(PdfiumError::Operation {
                function: "FPDF_LoadPage".into(),
                last_error: call.last_error,
            });
        }
        let parent: Arc<dyn Any + Send + Sync> = self.0.clone();
        Ok(crate::Page::from_raw(
            call.return_value,
            self.0.resource.handle(),
            vec![parent],
            self.0.source.clone(),
        ))
    }
}
