use pdfium_oxide::{PdfiumBuildProfile, PdfiumConfig, PdfiumError, Renderer};
use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

struct PdfiumPathEnv {
    _lock: MutexGuard<'static, ()>,
    previous: Option<OsString>,
}

impl PdfiumPathEnv {
    fn set(value: Option<OsString>) -> Self {
        let lock = ENV_LOCK.get_or_init(|| Mutex::new(()));
        let guard = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let previous = std::env::var_os("PDFIUM_LIBRARY_PATH");
        match value {
            Some(value) => std::env::set_var("PDFIUM_LIBRARY_PATH", value),
            None => std::env::remove_var("PDFIUM_LIBRARY_PATH"),
        }
        Self {
            _lock: guard,
            previous,
        }
    }
}

impl Drop for PdfiumPathEnv {
    fn drop(&mut self) {
        match self.previous.take() {
            Some(value) => std::env::set_var("PDFIUM_LIBRARY_PATH", value),
            None => std::env::remove_var("PDFIUM_LIBRARY_PATH"),
        }
    }
}

#[test]
fn config_rejects_relative_library_paths() {
    let error = PdfiumConfig::new("pdfium.dll").unwrap_err();

    assert!(matches!(error, PdfiumError::InvalidLibraryPath { .. }));
}

#[test]
fn config_accepts_an_absolute_library_path() {
    let path = std::env::current_exe()
        .expect("test executable path")
        .parent()
        .expect("test executable directory")
        .join("pdfium.dll");

    let config = PdfiumConfig::new(&path).expect("absolute paths are valid config");

    assert_eq!(config.library_path(), path);
}

#[test]
fn default_profile_selects_agg() {
    let path = std::env::current_exe().expect("test executable path");
    let config = PdfiumConfig::new(path).expect("absolute path");

    assert_eq!(config.renderer(), Renderer::Agg);
    assert_eq!(config.build_profile(), PdfiumBuildProfile::AggOnly);
}

#[test]
fn skia_requires_an_explicit_build_profile_declaration() {
    let path = std::env::current_exe().expect("test executable path");
    let config = PdfiumConfig::new(path).expect("absolute path");

    let error = config.with_renderer(Renderer::Skia).unwrap_err();

    assert!(matches!(error, PdfiumError::UnsupportedRenderer { .. }));
}

#[test]
fn skia_is_allowed_when_the_profile_declares_it() {
    let path = std::env::current_exe().expect("test executable path");
    let config = PdfiumConfig::new(path)
        .expect("absolute path")
        .with_build_profile(PdfiumBuildProfile::AggAndSkia)
        .expect("profile supports AGG");

    let config = config
        .with_renderer(Renderer::Skia)
        .expect("declared profile supports Skia");

    assert_eq!(config.renderer(), Renderer::Skia);
}

#[test]
fn from_env_rejects_an_unset_library_path_with_actionable_error() {
    let _env = PdfiumPathEnv::set(None);

    let error = PdfiumConfig::from_env().unwrap_err();

    assert!(
        matches!(error, PdfiumError::Initialization(message) if message.contains("PDFIUM_LIBRARY_PATH"))
    );
}

#[test]
fn from_env_rejects_an_empty_library_path_with_actionable_error() {
    let _env = PdfiumPathEnv::set(Some(OsString::new()));

    let error = PdfiumConfig::from_env().unwrap_err();

    assert!(
        matches!(error, PdfiumError::Initialization(message) if message.contains("PDFIUM_LIBRARY_PATH"))
    );
}

#[test]
fn from_env_rejects_a_relative_library_path() {
    let _env = PdfiumPathEnv::set(Some(OsString::from("pdfium.dll")));

    let error = PdfiumConfig::from_env().unwrap_err();

    assert!(matches!(error, PdfiumError::InvalidLibraryPath { .. }));
}

#[test]
fn from_env_accepts_an_absolute_library_path() {
    let path = std::env::current_exe()
        .expect("test executable path")
        .parent()
        .expect("test executable directory")
        .join("pdfium.dll");
    let _env = PdfiumPathEnv::set(Some(path.clone().into_os_string()));

    let config = PdfiumConfig::from_env().expect("absolute environment path");

    assert_eq!(config.library_path(), path);
}

#[cfg(unix)]
#[test]
fn from_env_accepts_a_non_unicode_absolute_library_path() {
    use std::os::unix::ffi::OsStringExt;

    let path = OsString::from_vec(b"/tmp/pdfium-\xff.so".to_vec());
    let expected = std::path::PathBuf::from(&path);
    let _env = PdfiumPathEnv::set(Some(path));

    let config = PdfiumConfig::from_env().expect("absolute non-Unicode environment path");

    assert_eq!(config.library_path(), expected);
}

#[cfg(windows)]
#[test]
fn from_env_accepts_a_non_unicode_absolute_library_path() {
    use std::os::windows::ffi::OsStringExt;

    let path = OsString::from_wide(&[b'C' as u16, b':' as u16, b'\\' as u16, 0xD800]);
    let expected = std::path::PathBuf::from(&path);
    let _env = PdfiumPathEnv::set(Some(path));

    let config = PdfiumConfig::from_env().expect("absolute non-Unicode environment path");

    assert_eq!(config.library_path(), expected);
}
