use std::{
    any::Any,
    ffi::{c_int, c_ulong, c_void},
    io::{self, Read, Seek, SeekFrom, Write},
    panic::{catch_unwind, AssertUnwindSafe},
    slice,
};

use crate::raw::{FPDF_FILEACCESS, FPDF_FILEWRITE};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
};

#[derive(Debug)]
pub(crate) enum SourceFailure {
    Io(String),
    Panic(String),
    Adapter(String),
}
pub(crate) trait SourceFailureCheck: Send + Sync {
    fn begin_source_operation(&self) -> usize;
    fn source_failure_since(&self, token: usize, operation: &str) -> Option<crate::PdfiumError>;
}

/// Stable context owned by the document until after FPDF_CloseDocument.
pub(crate) struct SourceContext<S> {
    pub(crate) interface: FPDF_FILEACCESS,
    file_len: u64,
    callback_count: AtomicUsize,
    source: Mutex<S>,
    failure: Mutex<Option<SourceFailure>>,
}
unsafe impl<S: Send> Send for SourceContext<S> {}
unsafe impl<S: Send> Sync for SourceContext<S> {}

impl<S: Read + Seek + Send + 'static> SourceContext<S> {
    pub(crate) fn new(source: S, len: c_ulong) -> std::sync::Arc<Self> {
        let mut context = std::sync::Arc::new(Self {
            interface: FPDF_FILEACCESS {
                m_FileLen: len,
                m_GetBlock: Some(read_block::<S>),
                m_Param: std::ptr::null_mut(),
            },
            source: Mutex::new(source),
            failure: Mutex::new(None),
            file_len: u64::from(len),
            callback_count: AtomicUsize::new(0),
        });
        std::sync::Arc::get_mut(&mut context)
            .unwrap()
            .interface
            .m_Param = std::sync::Arc::as_ptr(&context) as *mut c_void;
        context
    }
    fn fail(&self, failure: SourceFailure) {
        let mut slot = self.failure.lock().unwrap_or_else(|e| e.into_inner());
        if slot.is_none() {
            *slot = Some(failure);
        }
    }
}
impl<S: Read + Seek + Send + 'static> SourceFailureCheck for SourceContext<S> {
    fn begin_source_operation(&self) -> usize {
        self.callback_count.load(Ordering::SeqCst)
    }
    fn source_failure_since(&self, token: usize, operation: &str) -> Option<crate::PdfiumError> {
        if self.callback_count.load(Ordering::SeqCst) == token {
            return None;
        }
        self.failure
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .map(|failure| match failure {
                SourceFailure::Panic(message) => crate::PdfiumError::CallbackPanic {
                    operation: operation.into(),
                    message: message.clone(),
                },
                SourceFailure::Io(message) | SourceFailure::Adapter(message) => {
                    crate::PdfiumError::CallbackRead {
                        operation: operation.into(),
                        message: message.clone(),
                    }
                }
            })
    }
}

unsafe extern "C" fn read_block<S: Read + Seek + Send + 'static>(
    param: *mut c_void,
    position: c_ulong,
    buffer: *mut u8,
    size: c_ulong,
) -> c_int {
    if param.is_null() {
        return 0;
    }
    // SAFETY: m_Param points to the boxed context retained by the document.
    let context = unsafe { &*param.cast::<SourceContext<S>>() };
    context.callback_count.fetch_add(1, Ordering::SeqCst);
    if context
        .failure
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .is_some()
    {
        return 0;
    }
    let offset = match u64::try_from(position) {
        Ok(value) => value,
        Err(_) => {
            context.fail(SourceFailure::Adapter("source offset exceeds u64".into()));
            return 0;
        }
    };
    let size = match usize::try_from(size) {
        Ok(value) if value <= isize::MAX as usize => value,
        _ => {
            context.fail(SourceFailure::Adapter(
                "requested source range is too large".into(),
            ));
            return 0;
        }
    };
    if offset > context.file_len || (size as u128) > (context.file_len - offset) as u128 {
        context.fail(SourceFailure::Adapter(
            "requested source range is outside the declared file length".into(),
        ));
        return 0;
    }
    if size == 0 {
        return 1;
    }
    if buffer.is_null() {
        context.fail(SourceFailure::Adapter(
            "null buffer for nonempty source read".into(),
        ));
        return 0;
    }
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        let mut source = context.source.lock().unwrap_or_else(|e| e.into_inner());
        source.seek(SeekFrom::Start(offset))?;
        // SAFETY: PDFium supplies a writable buffer of `size` bytes for this callback.
        let bytes = unsafe { slice::from_raw_parts_mut(buffer, size) };
        source.read_exact(bytes)
    }));
    match outcome {
        Ok(Ok(())) => 1,
        Ok(Err(error)) => {
            context.fail(SourceFailure::Io(io_error_message(error)));
            0
        }
        Err(payload) => {
            context.fail(SourceFailure::Panic(panic_message(payload)));
            0
        }
    }
}

pub(crate) enum WriterFailure {
    Panic(String),
    Io(io::Error),
    Adapter(String),
}

/// `interface` must stay at offset zero and at a stable address while PDFium calls it.
#[repr(C)]
pub(crate) struct WriteContext<'a, W: Write> {
    pub(crate) interface: FPDF_FILEWRITE,
    writer: &'a mut W,
    failure: Option<WriterFailure>,
}

impl<'a, W: Write> WriteContext<'a, W> {
    pub(crate) fn new(writer: &'a mut W) -> Self {
        Self {
            interface: FPDF_FILEWRITE {
                version: 1,
                WriteBlock: Some(write_block::<W>),
            },
            writer,
            failure: None,
        }
    }

    pub(crate) fn take_failure(&mut self) -> Option<WriterFailure> {
        self.failure.take()
    }

    pub(crate) fn interface_ptr(&mut self) -> *mut FPDF_FILEWRITE {
        (self as *mut Self).cast::<FPDF_FILEWRITE>()
    }
}

unsafe extern "C" fn write_block<W: Write>(
    this: *mut FPDF_FILEWRITE,
    data: *const c_void,
    size: c_ulong,
) -> c_int {
    if this.is_null() {
        return 0;
    }
    // SAFETY: only save_to_writer installs this monomorphized callback and passes
    // the address of the leading interface field in a live WriteContext<W>.
    let context = unsafe { &mut *this.cast::<WriteContext<'_, W>>() };
    if context.failure.is_some() {
        return 0;
    }
    let length = match checked_block_length(size) {
        Ok(length) => length,
        Err(()) => {
            context.failure = Some(WriterFailure::Adapter(
                "PDFium supplied an output block too large for Rust".into(),
            ));
            return 0;
        }
    };
    let bytes: &[u8] = if length == 0 {
        &[]
    } else if data.is_null() {
        context.failure = Some(WriterFailure::Adapter(
            "PDFium supplied a null pointer for a nonempty output block".into(),
        ));
        return 0;
    } else {
        // SAFETY: PDFium's callback contract supplies `length` readable bytes for
        // this synchronous invocation. The range is capped at isize::MAX above.
        unsafe { slice::from_raw_parts(data.cast::<u8>(), length) }
    };

    match catch_unwind(AssertUnwindSafe(|| context.writer.write_all(bytes))) {
        Ok(Ok(())) => 1,
        Ok(Err(error)) => {
            context.failure = Some(WriterFailure::Io(error));
            0
        }
        Err(payload) => {
            context.failure = Some(WriterFailure::Panic(panic_message(payload)));
            0
        }
    }
}

fn checked_block_length(size: c_ulong) -> Result<usize, ()> {
    let length = usize::try_from(size).map_err(|_| ())?;
    if length > isize::MAX as usize {
        return Err(());
    }
    Ok(length)
}

fn panic_message(payload: Box<dyn Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    if let Some(message) = payload.downcast_ref::<&'static str>() {
        return (*message).to_owned();
    }
    // Unknown panic payloads may have user-defined Drop implementations.
    std::mem::forget(payload);
    "Rust callback panicked with a non-string payload".into()
}

pub(crate) fn io_error_message(error: io::Error) -> String {
    let formatted = catch_unwind(AssertUnwindSafe(|| error.to_string()));
    let formatting_panicked = match formatted {
        Ok(message) => Some(message),
        Err(payload) => {
            std::mem::forget(payload);
            None
        }
    };
    let dropped = catch_unwind(AssertUnwindSafe(|| drop(error)));
    let drop_succeeded = match dropped {
        Ok(()) => true,
        Err(payload) => {
            std::mem::forget(payload);
            false
        }
    };
    match (formatting_panicked, drop_succeeded) {
        (Some(message), true) => message,
        _ => "callback returned an error that panicked while formatting or dropping".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::ErrorKind,
        panic::{catch_unwind, panic_any},
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
    };

    #[test]
    fn source_callback_reads_exact_ranges_and_zero_size() {
        use std::io::Cursor;
        let source = SourceContext::new(Cursor::new(b"abcdef".to_vec()), 6);
        let callback = source.interface.m_GetBlock.unwrap();
        let mut buffer = [0u8; 3];
        let token = source.begin_source_operation();
        assert_eq!(
            unsafe { callback(source.interface.m_Param, 2, buffer.as_mut_ptr(), 3) },
            1
        );
        assert_eq!(&buffer, b"cde");
        assert_eq!(
            unsafe { callback(source.interface.m_Param, 6, std::ptr::null_mut(), 0) },
            1
        );
        assert!(source.source_failure_since(token, "test").is_none());
    }

    #[test]
    fn source_callback_rejects_null_context_without_dereferencing_it() {
        use std::io::Cursor;
        let source = SourceContext::new(Cursor::new(b"a".to_vec()), 1);
        let callback = source.interface.m_GetBlock.unwrap();
        let mut byte = 0;
        assert_eq!(
            unsafe { callback(std::ptr::null_mut(), 0, &mut byte, 1) },
            0
        );
        assert_eq!(byte, 0);
        assert!(source
            .source_failure_since(source.begin_source_operation(), "null")
            .is_none());
    }

    #[test]
    fn source_callback_rejects_ranges_and_keeps_the_first_error() {
        use std::io::Cursor;
        let source = SourceContext::new(Cursor::new(b"abcdef".to_vec()), 6);
        let callback = source.interface.m_GetBlock.unwrap();
        let mut buffer = [0u8; 2];
        let first_token = source.begin_source_operation();
        assert_eq!(
            unsafe { callback(source.interface.m_Param, 5, buffer.as_mut_ptr(), 2) },
            0
        );
        let first = source
            .source_failure_since(first_token, "first")
            .unwrap()
            .to_string();
        assert_eq!(
            unsafe { callback(source.interface.m_Param, 0, buffer.as_mut_ptr(), 1) },
            0
        );
        assert!(first.contains("outside the declared file length"));
        assert_eq!(
            source
                .source_failure_since(first_token, "later")
                .unwrap()
                .to_string()
                .replace("later", "first"),
            first
        );
    }

    #[test]
    fn source_callback_contains_read_panics_and_io_errors() {
        struct Panics;
        impl Read for Panics {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                panic!("source read panic")
            }
        }
        impl Seek for Panics {
            fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
                Ok(0)
            }
        }
        let source = SourceContext::new(Panics, 1);
        let callback = source.interface.m_GetBlock.unwrap();
        let mut byte = 0;
        let token = source.begin_source_operation();
        assert_eq!(
            unsafe { callback(source.interface.m_Param, 0, &mut byte, 1) },
            0
        );
        assert!(matches!(
            source.source_failure_since(token, "test"),
            Some(crate::PdfiumError::CallbackPanic { message, .. })
                if message.contains("source read panic")
        ));

        struct Fails;
        impl Read for Fails {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Ok(0)
            }
        }
        impl Seek for Fails {
            fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
                Err(io::Error::other("seek failure"))
            }
        }
        let source = SourceContext::new(Fails, 1);
        let token = source.begin_source_operation();
        assert_eq!(
            unsafe {
                source.interface.m_GetBlock.unwrap()(source.interface.m_Param, 0, &mut byte, 1)
            },
            0
        );
        assert!(matches!(
            source.source_failure_since(token, "test"),
            Some(crate::PdfiumError::CallbackRead { message, .. })
                if message.contains("seek failure")
        ));
    }

    fn invoke<W: Write>(context: &mut WriteContext<'_, W>, bytes: &[u8]) -> c_int {
        unsafe {
            write_block::<W>(
                context.interface_ptr(),
                bytes.as_ptr().cast(),
                bytes.len() as c_ulong,
            )
        }
    }

    #[test]
    fn handles_zero_null_and_unrepresentable_callback_blocks() {
        let mut output = Vec::new();
        let mut context = WriteContext::new(&mut output);
        assert_eq!(
            unsafe { write_block::<Vec<u8>>(context.interface_ptr(), std::ptr::null(), 0) },
            1
        );
        assert_eq!(
            unsafe { write_block::<Vec<u8>>(context.interface_ptr(), std::ptr::null(), 1) },
            0
        );
        assert!(matches!(
            context.take_failure(),
            Some(WriterFailure::Adapter(_))
        ));

        let too_large = (isize::MAX as u128) + 1;
        if let Ok(too_large) = c_ulong::try_from(too_large) {
            assert!(checked_block_length(too_large).is_err());
        }
    }

    #[cfg(all(unix, target_pointer_width = "64"))]
    #[test]
    fn rejects_callback_block_lengths_larger_than_isize() {
        let too_large = c_ulong::try_from(isize::MAX as u128 + 1).unwrap();
        assert!(checked_block_length(too_large).is_err());
    }

    #[test]
    fn writer_callback_rejects_null_interface() {
        assert_eq!(
            unsafe { write_block::<Vec<u8>>(std::ptr::null_mut(), b"x".as_ptr().cast(), 1) },
            0
        );
    }

    #[test]
    fn writes_successful_blocks_and_keeps_first_io_failure() {
        let mut output = Vec::new();
        let mut context = WriteContext::new(&mut output);
        assert_eq!(invoke(&mut context, b"callback bytes"), 1);
        assert_eq!(output, b"callback bytes");

        struct Fails(usize);
        impl Write for Fails {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                self.0 += 1;
                Err(io::Error::new(ErrorKind::Other, "blocked"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let mut writer = Fails(0);
        let mut context = WriteContext::new(&mut writer);
        assert_eq!(invoke(&mut context, b"x"), 0);
        assert_eq!(invoke(&mut context, b"y"), 0);
        assert!(matches!(context.take_failure(), Some(WriterFailure::Io(_))));
        drop(context);
        assert_eq!(
            writer.0, 1,
            "callback must stop calling the writer after failure"
        );
    }

    #[test]
    fn catches_writer_panics_and_forgets_unknown_panicking_payloads() {
        struct Bomb(Arc<AtomicBool>);
        impl Drop for Bomb {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
                panic!("payload destructor");
            }
        }
        struct Panics(Arc<AtomicBool>);
        impl Write for Panics {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                panic_any(Bomb(self.0.clone()))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let dropped = Arc::new(AtomicBool::new(false));
        let mut writer = Panics(dropped.clone());
        let mut context = WriteContext::new(&mut writer);
        let result = catch_unwind(AssertUnwindSafe(|| invoke(&mut context, b"x")));
        assert_eq!(result.unwrap(), 0);
        assert!(!dropped.load(Ordering::SeqCst));
        assert!(matches!(
            context.take_failure(),
            Some(WriterFailure::Panic(_))
        ));
    }

    #[test]
    fn contains_panics_from_custom_io_error_display_and_drop() {
        struct ErrorBomb(Arc<AtomicBool>);
        impl std::fmt::Display for ErrorBomb {
            fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                panic!("error display")
            }
        }
        impl std::fmt::Debug for ErrorBomb {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ErrorBomb")
            }
        }
        impl std::error::Error for ErrorBomb {}
        impl Drop for ErrorBomb {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
                panic!("error destructor");
            }
        }
        let dropped = Arc::new(AtomicBool::new(false));
        let message =
            io_error_message(io::Error::new(ErrorKind::Other, ErrorBomb(dropped.clone())));
        assert!(message.contains("panicked"));
        assert!(message.contains("callback returned an error"));
        assert!(dropped.load(Ordering::SeqCst));
    }
}
