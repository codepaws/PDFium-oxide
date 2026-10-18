//! A permissively licensed Rust interface to the PDFium C library.
//!
//! The raw bindings target the official PDFium public headers recorded in the
//! crate metadata. The native PDFium library is supplied by the application.

/// Upstream PDFium binary release used as this crate's initial ABI baseline.
pub const PDFIUM_VERSION: &str = "157.0.8100.0";

/// Chromium branch used to build the selected PDFium release.
pub const PDFIUM_CHROMIUM_BRANCH: &str = "chromium/8100";

/// Immutable upstream PDFium source commit used to generate the bindings.
pub const PDFIUM_SOURCE_COMMIT: &str = "f51279699cb59754d1f49aef10eb8e825fe56532";

mod callback;
mod config;
pub mod document;
mod error;
mod handles;
pub mod page;
pub mod raw;
pub mod render;
mod runtime;
pub mod text;

pub use config::{Capabilities, Capability, PdfiumBuildProfile, PdfiumConfig, Renderer};
pub use document::Document;
pub use error::PdfiumError;
pub use page::{Page, PageSize, Point, TextStyle};
pub use render::{
    AlphaSemantics, Bitmap, BitmapPixelView, BitmapSnapshot, ChannelOrder, PixelFormat,
};
pub use runtime::{Pdfium, PdfiumCall, PdfiumDestroyed, PdfiumInit};
pub use text::{TextPage, TextSegment};
