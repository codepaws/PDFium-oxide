use pdfium_oxide::{PdfiumBuildProfile, PdfiumConfig, PdfiumError, Renderer};
use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
static CURRENT_DIR_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

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

struct CurrentDirGuard {
    _lock: MutexGuard<'static, ()>,
    previous: std::path::PathBuf,
}

impl CurrentDirGuard {
    fn set(path: &std::path::Path) -> Self {
        let lock = CURRENT_DIR_LOCK.get_or_init(|| Mutex::new(()));
        let guard = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let previous = std::env::current_dir().expect("current directory");
        std::env::set_current_dir(path).expect("set current directory");
        Self {
            _lock: guard,
            previous,
        }
    }
}

impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.previous).expect("restore current directory");
    }
}

fn temporary_directory(name: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "pdfium-oxide-{name}-{}-{nonce}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path).expect("create temporary directory");
    path
}

#[cfg(target_os = "windows")]
const PDFIUM_FILENAME: &str = "pdfium.dll";
#[cfg(target_os = "linux")]
const PDFIUM_FILENAME: &str = "libpdfium.so";

#[test]
fn from_env_or_current_dir_uses_platform_library_when_env_is_unset() {
    let directory = temporary_directory("cwd-fallback");
    let library = directory.join(PDFIUM_FILENAME);
    std::fs::write(&library, b"test placeholder").expect("create placeholder library");
    let _cwd = CurrentDirGuard::set(&directory);
    let _env = PdfiumPathEnv::set(None);

    let config =
        PdfiumConfig::from_env_or_current_dir().expect("find library in current directory");

    assert_eq!(config.library_path(), library);
    drop(_cwd);
    std::fs::remove_dir_all(directory).expect("remove temporary directory");
}

#[test]
fn from_env_or_current_dir_prefers_environment_path() {
    let directory = temporary_directory("env-precedence");
    std::fs::write(directory.join(PDFIUM_FILENAME), b"cwd placeholder")
        .expect("create current directory placeholder");
    let _cwd = CurrentDirGuard::set(&directory);
    let env_path = std::env::current_exe().expect("test executable path");
    let _env = PdfiumPathEnv::set(Some(env_path.clone().into_os_string()));

    let config = PdfiumConfig::from_env_or_current_dir().expect("use environment path");

    assert_eq!(config.library_path(), env_path);
    drop(_cwd);
    std::fs::remove_dir_all(directory).expect("remove temporary directory");
}

#[test]
fn from_env_or_current_dir_does_not_fall_back_for_invalid_environment_path() {
    let directory = temporary_directory("invalid-env");
    std::fs::write(directory.join(PDFIUM_FILENAME), b"cwd placeholder")
        .expect("create current directory placeholder");
    let _cwd = CurrentDirGuard::set(&directory);
    let _env = PdfiumPathEnv::set(Some(OsString::from("relative-pdfium")));

    let error = PdfiumConfig::from_env_or_current_dir().unwrap_err();

    assert!(matches!(error, PdfiumError::InvalidLibraryPath { .. }));
    drop(_cwd);
    std::fs::remove_dir_all(directory).expect("remove temporary directory");
}

#[test]
fn from_env_or_current_dir_reports_missing_library_and_search_location() {
    let directory = temporary_directory("missing-library");
    let _cwd = CurrentDirGuard::set(&directory);
    let _env = PdfiumPathEnv::set(None);

    let error = PdfiumConfig::from_env_or_current_dir().unwrap_err();

    assert!(matches!(error, PdfiumError::Initialization(message)
        if message.contains(PDFIUM_FILENAME) && message.contains("PDFIUM_LIBRARY_PATH")));
    drop(_cwd);
    std::fs::remove_dir_all(directory).expect("remove temporary directory");
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
