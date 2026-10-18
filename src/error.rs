use std::fmt;
use std::path::PathBuf;

use crate::config::{Capability, Renderer};

/// Errors returned while configuring, loading, or calling PDFium.
#[derive(Debug)]
#[non_exhaustive]
pub enum PdfiumError {
    /// The library path is not absolute.
    InvalidLibraryPath { path: PathBuf },
    /// The caller selected a renderer that the declared build profile excludes.
    UnsupportedRenderer { renderer: Renderer },
    /// The native library could not be loaded.
    LibraryLoad { path: PathBuf, message: String },
    /// A required function export is missing from the native library.
    MissingSymbol { path: PathBuf, symbol: String },
    /// The loaded library does not provide the requested optional capability.
    UnsupportedCapability(Capability),
    /// Initialization or configuration validation failed.
    Initialization(String),
    /// The process-wide coordinator already owns a different library configuration.
    AlreadyInitialized { path: PathBuf },
    /// PDFium has been terminally destroyed through the coordinator.
    LibraryDestroyed,
    /// A native call attempted to re-enter PDFium from a callback on the same thread.
    ReentrantCall,
    /// A function was invoked in a phase prohibited by the pinned PDFium headers.
    LifecycleViolation {
        function: String,
        phase: &'static str,
    },
    /// The native call's error snapshot, when meaningful for that function.
    Operation { function: String, last_error: u64 },
    /// Invalid caller input rejected before or after a native operation.
    InvalidInput { operation: String, message: String },
    /// PDFium could not load a document.
    DocumentLoad { operation: String, last_error: u64 },
    /// PDFium rejected a document password (missing and incorrect are indistinguishable).
    Password { operation: String, last_error: u64 },
    /// A checked Rust-side allocation failed.
    Allocation { requested_bytes: usize },
    /// PDFium returned invalid UTF-16 text.
    TextDecoding { message: String },
    /// A crate-owned Rust callback panicked and its failure was caught at the C boundary.
    CallbackPanic { operation: String, message: String },
    /// A crate-owned Rust callback could not complete its write operation.
    CallbackWrite { operation: String, message: String },
    /// A crate-owned random-access source failed while PDFium read document data.
    CallbackRead { operation: String, message: String },
}

impl fmt::Display for PdfiumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLibraryPath { path } => {
                write!(
                    f,
                    "PDFium library path must be absolute: {}",
                    path.display()
                )
            }
            Self::UnsupportedRenderer { renderer } => {
                write!(
                    f,
                    "renderer {renderer:?} is not declared by the PDFium build profile"
                )
            }
            Self::LibraryLoad { path, message } => {
                write!(
                    f,
                    "failed to load PDFium library {}: {message}",
                    path.display()
                )
            }
            Self::MissingSymbol { path, symbol } => write!(
                f,
                "PDFium library {} is missing required export {symbol}",
                path.display()
            ),
            Self::UnsupportedCapability(capability) => {
                write!(
                    f,
                    "the loaded PDFium library does not support {capability:?}"
                )
            }
            Self::Initialization(message) => write!(f, "PDFium initialization failed: {message}"),
            Self::AlreadyInitialized { path } => write!(
                f,
                "PDFium is already configured with a different library or profile: {}",
                path.display()
            ),
            Self::LibraryDestroyed => f.write_str("PDFium has been destroyed"),
            Self::ReentrantCall => f.write_str(
                "PDFium calls cannot be re-entered synchronously from a PDFium callback",
            ),
            Self::LifecycleViolation { function, phase } => {
                write!(f, "{function} cannot be called during the {phase} phase")
            }
            Self::Operation {
                function,
                last_error,
            } => write!(
                f,
                "PDFium operation {function} failed with native error snapshot {last_error}"
            ),
            Self::InvalidInput { operation, message } => {
                write!(f, "invalid input for {operation}: {message}")
            }
            Self::DocumentLoad {
                operation,
                last_error,
            } => write!(
                f,
                "{operation} failed to load document (PDFium error {last_error})"
            ),
            Self::Password {
                operation,
                last_error,
            } => write!(
                f,
                "{operation} rejected the document password (PDFium error {last_error})"
            ),
            Self::Allocation { requested_bytes } => {
                write!(f, "could not allocate {requested_bytes} bytes")
            }
            Self::TextDecoding { message } => write!(f, "could not decode PDF text: {message}"),
            Self::CallbackPanic { operation, message } => {
                write!(f, "Rust callback for {operation} panicked: {message}")
            }
            Self::CallbackWrite { operation, message } => {
                write!(f, "Rust callback for {operation} failed: {message}")
            }
            Self::CallbackRead { operation, message } => {
                write!(f, "source callback for {operation} failed: {message}")
            }
        }
    }
}

impl std::error::Error for PdfiumError {}

#[cfg(test)]
mod tests {
    use super::PdfiumError;
    use crate::{Capability, Renderer};
    use std::path::PathBuf;

    #[test]
    fn display_formats_every_error_variant() {
        let path = PathBuf::from("pdfium.dll");
        let cases = [
            (
                PdfiumError::InvalidLibraryPath { path: path.clone() },
                "PDFium library path must be absolute: pdfium.dll",
            ),
            (
                PdfiumError::UnsupportedRenderer {
                    renderer: Renderer::Skia,
                },
                "renderer Skia is not declared by the PDFium build profile",
            ),
            (
                PdfiumError::LibraryLoad {
                    path: path.clone(),
                    message: "bad image".into(),
                },
                "failed to load PDFium library pdfium.dll: bad image",
            ),
            (
                PdfiumError::MissingSymbol {
                    path: path.clone(),
                    symbol: "FPDF_LoadDocument".into(),
                },
                "PDFium library pdfium.dll is missing required export FPDF_LoadDocument",
            ),
            (
                PdfiumError::UnsupportedCapability(Capability::V8Xfa),
                "the loaded PDFium library does not support V8Xfa",
            ),
            (
                PdfiumError::Initialization("missing path".into()),
                "PDFium initialization failed: missing path",
            ),
            (
                PdfiumError::AlreadyInitialized { path: path.clone() },
                "PDFium is already configured with a different library or profile: pdfium.dll",
            ),
            (PdfiumError::LibraryDestroyed, "PDFium has been destroyed"),
            (
                PdfiumError::ReentrantCall,
                "PDFium calls cannot be re-entered synchronously from a PDFium callback",
            ),
            (
                PdfiumError::LifecycleViolation {
                    function: "FPDF_InitLibrary".into(),
                    phase: "initialized",
                },
                "FPDF_InitLibrary cannot be called during the initialized phase",
            ),
            (
                PdfiumError::Operation {
                    function: "FPDF_LoadDocument".into(),
                    last_error: 4,
                },
                "PDFium operation FPDF_LoadDocument failed with native error snapshot 4",
            ),
            (
                PdfiumError::InvalidInput {
                    operation: "Bitmap::create".into(),
                    message: "invalid size".into(),
                },
                "invalid input for Bitmap::create: invalid size",
            ),
            (
                PdfiumError::DocumentLoad {
                    operation: "FPDF_LoadDocument".into(),
                    last_error: 4,
                },
                "FPDF_LoadDocument failed to load document (PDFium error 4)",
            ),
            (
                PdfiumError::Password {
                    operation: "FPDF_LoadDocument".into(),
                    last_error: 4,
                },
                "FPDF_LoadDocument rejected the document password (PDFium error 4)",
            ),
            (
                PdfiumError::Allocation {
                    requested_bytes: 1024,
                },
                "could not allocate 1024 bytes",
            ),
            (
                PdfiumError::TextDecoding {
                    message: "invalid UTF-16".into(),
                },
                "could not decode PDF text: invalid UTF-16",
            ),
            (
                PdfiumError::CallbackPanic {
                    operation: "save".into(),
                    message: "test panic".into(),
                },
                "Rust callback for save panicked: test panic",
            ),
            (
                PdfiumError::CallbackWrite {
                    operation: "save".into(),
                    message: "disk full".into(),
                },
                "Rust callback for save failed: disk full",
            ),
            (
                PdfiumError::CallbackRead {
                    operation: "load".into(),
                    message: "unexpected EOF".into(),
                },
                "source callback for load failed: unexpected EOF",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }
}
