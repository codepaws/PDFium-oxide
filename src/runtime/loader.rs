use crate::{Capabilities, PdfiumConfig, PdfiumError};
use std::ffi::c_void;
use std::path::Path;

pub(crate) struct LoadedLibrary {
    pub(crate) dispatch: crate::raw::Dispatch,
}

fn validate_symbols(
    path: &Path,
    required: &[&str],
    optional: &[(&str, crate::Capability)],
    mut has_symbol: impl FnMut(&str) -> bool,
) -> Result<(Capabilities, std::collections::HashSet<String>), PdfiumError> {
    let mut available = std::collections::HashSet::new();
    for symbol in required {
        if !has_symbol(symbol) {
            return Err(PdfiumError::MissingSymbol {
                path: path.to_owned(),
                symbol: (*symbol).to_owned(),
            });
        }
        available.insert((*symbol).to_owned());
    }
    let mut v8_xfa = !optional.is_empty();
    for (symbol, _capability) in optional {
        if has_symbol(symbol) {
            available.insert((*symbol).to_owned());
        } else {
            v8_xfa = false;
        }
    }
    Ok((Capabilities::new(v8_xfa, false), available))
}

pub(crate) fn load(
    path: &Path,
    _config: &PdfiumConfig,
) -> Result<
    (
        LoadedLibrary,
        Capabilities,
        std::collections::HashSet<String>,
    ),
    PdfiumError,
> {
    #[cfg(target_os = "windows")]
    let library: libloading::Library = {
        use libloading::os::windows::{
            Library as Native, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
        };
        unsafe {
            Native::load_with_flags(
                path,
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
            )
        }
        .map(Into::into)
        .map_err(|e| PdfiumError::LibraryLoad {
            path: path.to_owned(),
            message: e.to_string(),
        })?
    };
    #[cfg(target_os = "linux")]
    let library: libloading::Library = {
        use libloading::os::unix::{Library as Native, RTLD_LOCAL, RTLD_NOW};
        unsafe { Native::open(Some(path), RTLD_NOW | RTLD_LOCAL) }
            .map(Into::into)
            .map_err(|e| PdfiumError::LibraryLoad {
                path: path.to_owned(),
                message: e.to_string(),
            })?
    };
    let optional = crate::raw::symbols::OPTIONAL_SYMBOLS;
    let (mut capabilities, available) = validate_symbols(
        path,
        crate::raw::symbols::REQUIRED_SYMBOLS,
        optional,
        |symbol| unsafe { library.get::<*const c_void>(symbol.as_bytes()).is_ok() },
    )?;
    capabilities = Capabilities::new(
        capabilities.supports(crate::Capability::V8Xfa),
        _config.build_profile().includes_skia(),
    );
    let dispatch = unsafe { crate::raw::Dispatch::from_library(library) }.map_err(|e| {
        PdfiumError::LibraryLoad {
            path: path.to_owned(),
            message: e.to_string(),
        }
    })?;
    Ok((LoadedLibrary { dispatch }, capabilities, available))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_each_missing_required_symbol() {
        let names = ["required_a", "required_b", "required_c"];
        for missing in names {
            let result = validate_symbols(Path::new("/tmp/pdfium"), &names, &[], |name| {
                name != missing
            });
            assert!(
                matches!(result, Err(PdfiumError::MissingSymbol { symbol, .. }) if symbol == missing)
            );
        }
    }
    #[test]
    fn every_export_in_the_selected_required_manifest_is_enforced() {
        let required = crate::raw::symbols::REQUIRED_SYMBOLS;
        let optional = crate::raw::symbols::OPTIONAL_SYMBOLS;
        for missing in required {
            let result = validate_symbols(Path::new("/tmp/pdfium"), required, optional, |name| {
                name != *missing
            });
            assert!(
                matches!(result, Err(PdfiumError::MissingSymbol { ref symbol, .. }) if symbol == missing)
            );
        }
    }
    #[test]
    fn every_selected_optional_export_is_independently_optional() {
        let required = crate::raw::symbols::REQUIRED_SYMBOLS;
        let optional = crate::raw::symbols::OPTIONAL_SYMBOLS;
        for (missing, capability) in optional {
            let result = validate_symbols(Path::new("/tmp/pdfium"), required, optional, |name| {
                name != *missing
            })
            .unwrap();
            assert!(!result.0.supports(*capability));
            assert!(!result.1.contains(*missing));
            assert!(result
                .1
                .iter()
                .any(|name| required.contains(&name.as_str())));
        }
    }
    #[test]
    fn optional_symbols_only_enable_capability_when_complete() {
        let optional = [
            ("v8_a", crate::Capability::V8Xfa),
            ("v8_b", crate::Capability::V8Xfa),
        ];
        let complete =
            validate_symbols(Path::new("/tmp/pdfium"), &["base"], &optional, |_| true).unwrap();
        assert!(complete.0.supports(crate::Capability::V8Xfa));
        for missing in ["v8_a", "v8_b"] {
            let incomplete =
                validate_symbols(Path::new("/tmp/pdfium"), &["base"], &optional, |name| {
                    name != missing
                })
                .unwrap();
            assert!(!incomplete.0.supports(crate::Capability::V8Xfa));
            assert!(!incomplete.1.contains(missing));
        }
    }

    #[test]
    fn empty_optional_manifest_does_not_advertise_v8_xfa() {
        let (capabilities, available) =
            validate_symbols(Path::new("/tmp/pdfium"), &["base"], &[], |_| true).unwrap();
        assert!(!capabilities.supports(crate::Capability::V8Xfa));
        assert_eq!(
            available,
            std::collections::HashSet::from(["base".to_owned()])
        );
    }

    #[test]
    fn missing_library_returns_path_context() {
        let path =
            std::env::temp_dir().join(format!("pdfium-does-not-exist-{}.dll", std::process::id()));
        let config = PdfiumConfig::new(path.clone()).unwrap();
        let error = match load(&path, &config) {
            Err(error) => error,
            Ok(_) => panic!("unexpectedly loaded missing library"),
        };
        assert!(matches!(error, PdfiumError::LibraryLoad { path: failed, .. } if failed == path));
    }
}
