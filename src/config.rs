use std::path::{Path, PathBuf};

use crate::error::PdfiumError;

#[cfg(target_os = "windows")]
const PDFIUM_LIBRARY_FILENAME: &str = "pdfium.dll";
#[cfg(target_os = "linux")]
const PDFIUM_LIBRARY_FILENAME: &str = "libpdfium.so";

/// PDFium's renderer selected during process-wide initialization.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Renderer {
    /// The renderer included in the standard PDFium build.
    Agg,
    /// The optional Skia renderer.
    Skia,
}

/// Build properties declared by the application for its external PDFium library.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PdfiumBuildProfile {
    /// The library is known to include AGG only.
    AggOnly,
    /// The library is known to include both AGG and Skia.
    AggAndSkia,
}

impl Default for PdfiumBuildProfile {
    fn default() -> Self {
        Self::AggOnly
    }
}

impl PdfiumBuildProfile {
    /// Returns whether this caller-supplied profile declares Skia support.
    pub const fn includes_skia(self) -> bool {
        matches!(self, Self::AggAndSkia)
    }
}

/// A runtime capability of the loaded PDFium binary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Capability {
    /// V8/XFA related exports are available in the loaded library.
    V8Xfa,
    /// Skia was declared by the application as included in the library build.
    Skia,
}

/// Capabilities established or declared for one loaded PDFium library.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Capabilities {
    v8_xfa: bool,
    skia: bool,
}

impl Capabilities {
    /// Returns whether the given capability is available.
    pub const fn supports(self, capability: Capability) -> bool {
        match capability {
            Capability::V8Xfa => self.v8_xfa,
            Capability::Skia => self.skia,
        }
    }

    pub(crate) const fn new(v8_xfa: bool, skia: bool) -> Self {
        Self { v8_xfa, skia }
    }
}

/// Immutable configuration used to load one external PDFium library.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PdfiumConfig {
    library_path: PathBuf,
    build_profile: PdfiumBuildProfile,
    renderer: Renderer,
}

impl PdfiumConfig {
    /// Creates a configuration from the `PDFIUM_LIBRARY_PATH` environment variable.
    ///
    /// The value must name an absolute path. This is an explicit convenience
    /// for applications that choose to configure PDFium through their
    /// environment; the loader does not search the environment on its own.
    pub fn from_env() -> Result<Self, PdfiumError> {
        let path = std::env::var_os("PDFIUM_LIBRARY_PATH").ok_or_else(|| {
            PdfiumError::Initialization(
                "PDFIUM_LIBRARY_PATH is not set; set it to an absolute path to the PDFium library or use PdfiumConfig::new".into(),
            )
        })?;
        if path.is_empty() {
            return Err(PdfiumError::Initialization(
                "PDFIUM_LIBRARY_PATH is empty; set it to an absolute path to the PDFium library or use PdfiumConfig::new".into(),
            ));
        }
        Self::new(PathBuf::from(path))
    }

    /// Creates a configuration from `PDFIUM_LIBRARY_PATH`, or from the standard
    /// PDFium library filename in the current working directory when the
    /// environment variable is unset.
    ///
    /// An empty, relative, or otherwise invalid environment value is an error
    /// and does not trigger the current-directory fallback. This checks only
    /// `pdfium.dll` on Windows or `libpdfium.so` on Linux;
    /// it does not search `PATH` or subdirectories. Because this can load native
    /// code from the current working directory, use it only when that directory
    /// is trusted.
    pub fn from_env_or_current_dir() -> Result<Self, PdfiumError> {
        if std::env::var_os("PDFIUM_LIBRARY_PATH").is_some() {
            return Self::from_env();
        }

        let current_dir = std::env::current_dir().map_err(|error| {
            PdfiumError::Initialization(format!(
                "could not determine the current directory while locating PDFium: {error}"
            ))
        })?;
        let library_path = current_dir.join(PDFIUM_LIBRARY_FILENAME);
        if !library_path.is_file() {
            return Err(PdfiumError::Initialization(format!(
                "PDFIUM_LIBRARY_PATH is not set and no PDFium library was found at '{}'; set PDFIUM_LIBRARY_PATH to an absolute path or place {PDFIUM_LIBRARY_FILENAME} in the current directory",
                library_path.display()
            )));
        }

        Self::new(library_path)
    }

    /// Creates a configuration for an explicit absolute library path.
    ///
    /// The default profile selects AGG and makes no claim that Skia is present.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, PdfiumError> {
        let library_path = path.into();
        if !library_path.is_absolute() {
            return Err(PdfiumError::InvalidLibraryPath { path: library_path });
        }

        Ok(Self {
            library_path,
            build_profile: PdfiumBuildProfile::default(),
            renderer: Renderer::Agg,
        })
    }

    /// Returns the explicit native library path.
    pub fn library_path(&self) -> &Path {
        &self.library_path
    }

    /// Returns the caller-supplied native build profile.
    pub const fn build_profile(&self) -> PdfiumBuildProfile {
        self.build_profile
    }

    /// Returns the renderer requested for PDFium initialization.
    pub const fn renderer(&self) -> Renderer {
        self.renderer
    }

    /// Sets the caller-supplied build profile.
    pub fn with_build_profile(
        mut self,
        build_profile: PdfiumBuildProfile,
    ) -> Result<Self, PdfiumError> {
        if self.renderer == Renderer::Skia && !build_profile.includes_skia() {
            return Err(PdfiumError::UnsupportedRenderer {
                renderer: self.renderer,
            });
        }
        self.build_profile = build_profile;
        Ok(self)
    }

    /// Selects the renderer after checking it against the declared build profile.
    pub fn with_renderer(mut self, renderer: Renderer) -> Result<Self, PdfiumError> {
        if renderer == Renderer::Skia && !self.build_profile.includes_skia() {
            return Err(PdfiumError::UnsupportedRenderer { renderer });
        }
        self.renderer = renderer;
        Ok(self)
    }

    /// Builds the exact native initialization struct for this profile.
    pub(crate) fn native_config(&self) -> crate::raw::FPDF_LIBRARY_CONFIG {
        crate::raw::FPDF_LIBRARY_CONFIG {
            version: 4,
            m_pUserFontPaths: std::ptr::null_mut(),
            m_pIsolate: std::ptr::null_mut(),
            m_v8EmbedderSlot: 0,
            m_pPlatform: std::ptr::null_mut(),
            m_RendererType: match self.renderer {
                Renderer::Agg => crate::raw::FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_AGG,
                Renderer::Skia => crate::raw::FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA,
            },
            m_FontLibraryType: crate::raw::FPDF_FONT_BACKEND_TYPE_FPDF_FONTBACKENDTYPE_FREETYPE,
            m_BrotliEnabled: 0,
            m_IsolatePerDocument: 0,
        }
    }

    pub(crate) fn set_canonical_path(&mut self, path: PathBuf) {
        self.library_path = path;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raw;

    #[test]
    fn native_config_initializes_all_fields_and_uses_version_four() {
        let path = std::env::current_exe().expect("test executable path");
        let config = PdfiumConfig::new(path)
            .expect("absolute path")
            .with_build_profile(PdfiumBuildProfile::AggAndSkia)
            .expect("AGG is in both profiles")
            .with_renderer(Renderer::Skia)
            .expect("declared Skia support");

        let native = config.native_config();

        assert_eq!(native.version, 4);
        assert!(native.m_pUserFontPaths.is_null());
        assert!(native.m_pIsolate.is_null());
        assert_eq!(native.m_v8EmbedderSlot, 0);
        assert!(native.m_pPlatform.is_null());
        assert_eq!(
            native.m_RendererType,
            raw::FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA
        );
        assert_eq!(
            native.m_FontLibraryType,
            raw::FPDF_FONT_BACKEND_TYPE_FPDF_FONTBACKENDTYPE_FREETYPE
        );
        assert_eq!(native.m_BrotliEnabled, 0);
        assert_eq!(native.m_IsolatePerDocument, 0);
    }

    #[test]
    fn agg_only_profile_reports_capabilities_and_encodes_agg_renderer() {
        let capabilities = Capabilities::new(false, false);
        assert!(!capabilities.supports(Capability::V8Xfa));
        assert!(!capabilities.supports(Capability::Skia));
        assert!(!PdfiumBuildProfile::AggOnly.includes_skia());

        let config = PdfiumConfig::new(std::env::current_exe().unwrap()).unwrap();
        let native = config.native_config();
        assert_eq!(
            native.m_RendererType,
            raw::FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_AGG
        );
    }

    #[test]
    fn skia_renderer_cannot_be_reclassified_as_agg_only() {
        let config = PdfiumConfig::new(std::env::current_exe().unwrap())
            .unwrap()
            .with_build_profile(PdfiumBuildProfile::AggAndSkia)
            .unwrap()
            .with_renderer(Renderer::Skia)
            .unwrap();

        assert!(matches!(
            config.with_build_profile(PdfiumBuildProfile::AggOnly),
            Err(PdfiumError::UnsupportedRenderer {
                renderer: Renderer::Skia
            })
        ));
    }
}
