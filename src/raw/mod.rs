//! Generated PDFium C API types and constants for the selected target profile.
//!
//! The checked-in dispatch declarations are private to the crate. Public raw
//! calls will be exposed through the process-wide coordinator so callers
//! cannot bypass PDFium's serialization contract.

/// Header configuration compiled into this crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeaderConfiguration {
    /// Windows x64 default non-V8 headers.
    WindowsX64Default,
    /// Windows x64 headers with V8 and XFA declarations.
    WindowsX64V8Xfa,
    /// Linux x64 default non-V8 headers.
    LinuxX64Default,
    /// Linux x64 headers with V8 and XFA declarations.
    LinuxX64V8Xfa,
    /// Linux ARM64 default non-V8 headers.
    LinuxArm64Default,
    /// Linux ARM64 headers with V8 and XFA declarations.
    LinuxArm64V8Xfa,
}

/// Header configuration compiled into this crate for its target platform.
pub const HEADER_CONFIGURATION: HeaderConfiguration = selected::HEADER_CONFIGURATION;

/// Source commit used for the selected generated declarations.
pub const PDFIUM_SOURCE_COMMIT: &str = crate::PDFIUM_SOURCE_COMMIT;

/// Version label of the binary release used to select this header baseline.
pub const PDFIUM_VERSION: &str = crate::PDFIUM_VERSION;

pub(crate) use selected::dispatch::pdfium as Dispatch;

/// Initialized raw functions. Every call is serialized through the process coordinator.
pub struct RawApi<'a> {
    pub(crate) _owner: &'a crate::Pdfium,
}
/// Functions permitted before PDFium initialization.
pub struct PreInitApi<'a> {
    pub(crate) _owner: &'a crate::PdfiumInit,
}
/// Functions permitted after terminal destruction.
pub struct DestroyedApi<'a> {
    pub(crate) _owner: &'a crate::PdfiumDestroyed,
}

#[cfg(all(
    target_os = "windows",
    target_arch = "x86_64",
    not(feature = "pdfium-v8")
))]
mod selected {
    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub mod types {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/windows-x64-default/types.rs"
        ));
    }

    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub(crate) mod dispatch {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/windows-x64-default/dispatch.rs"
        ));
    }

    pub(crate) mod symbols {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/windows-x64-default/symbols.rs"
        ));
    }

    pub const HEADER_CONFIGURATION: super::HeaderConfiguration =
        super::HeaderConfiguration::WindowsX64Default;
}

#[cfg(all(target_os = "windows", target_arch = "x86_64", feature = "pdfium-v8"))]
mod selected {
    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub mod types {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/windows-x64-v8-xfa/types.rs"
        ));
    }

    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub(crate) mod dispatch {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/windows-x64-v8-xfa/dispatch.rs"
        ));
    }

    pub(crate) mod symbols {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/windows-x64-v8-xfa/symbols.rs"
        ));
    }

    pub const HEADER_CONFIGURATION: super::HeaderConfiguration =
        super::HeaderConfiguration::WindowsX64V8Xfa;
}

#[cfg(all(
    target_os = "linux",
    target_arch = "x86_64",
    not(feature = "pdfium-v8")
))]
mod selected {
    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub mod types {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-x64-default/types.rs"
        ));
    }

    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub(crate) mod dispatch {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-x64-default/dispatch.rs"
        ));
    }

    pub(crate) mod symbols {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-x64-default/symbols.rs"
        ));
    }

    pub const HEADER_CONFIGURATION: super::HeaderConfiguration =
        super::HeaderConfiguration::LinuxX64Default;
}

#[cfg(all(target_os = "linux", target_arch = "x86_64", feature = "pdfium-v8"))]
mod selected {
    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub mod types {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-x64-v8-xfa/types.rs"
        ));
    }

    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub(crate) mod dispatch {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-x64-v8-xfa/dispatch.rs"
        ));
    }

    pub(crate) mod symbols {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-x64-v8-xfa/symbols.rs"
        ));
    }

    pub const HEADER_CONFIGURATION: super::HeaderConfiguration =
        super::HeaderConfiguration::LinuxX64V8Xfa;
}

#[cfg(all(
    target_os = "linux",
    target_arch = "aarch64",
    not(feature = "pdfium-v8")
))]
mod selected {
    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub mod types {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-arm64-default/types.rs"
        ));
    }

    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub(crate) mod dispatch {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-arm64-default/dispatch.rs"
        ));
    }

    pub(crate) mod symbols {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-arm64-default/symbols.rs"
        ));
    }

    pub const HEADER_CONFIGURATION: super::HeaderConfiguration =
        super::HeaderConfiguration::LinuxArm64Default;
}

#[cfg(all(target_os = "linux", target_arch = "aarch64", feature = "pdfium-v8"))]
mod selected {
    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub mod types {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-arm64-v8-xfa/types.rs"
        ));
    }

    #[allow(
        dead_code,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]
    pub(crate) mod dispatch {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-arm64-v8-xfa/dispatch.rs"
        ));
    }

    pub(crate) mod symbols {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bindings/generated/linux-arm64-v8-xfa/symbols.rs"
        ));
    }

    pub const HEADER_CONFIGURATION: super::HeaderConfiguration =
        super::HeaderConfiguration::LinuxArm64V8Xfa;
}

#[cfg(not(any(
    all(target_os = "windows", target_arch = "x86_64"),
    all(
        target_os = "linux",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )
)))]
compile_error!("pdfium-oxide bindings are generated for Windows x64, Linux x64, and Linux ARM64");

pub(crate) use selected::symbols;
pub use selected::types::*;

#[cfg(all(
    target_os = "windows",
    target_arch = "x86_64",
    not(feature = "pdfium-v8")
))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/bindings/generated/windows-x64-default/api.rs"
));
#[cfg(all(target_os = "windows", target_arch = "x86_64", feature = "pdfium-v8"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/bindings/generated/windows-x64-v8-xfa/api.rs"
));
#[cfg(all(
    target_os = "linux",
    target_arch = "x86_64",
    not(feature = "pdfium-v8")
))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/bindings/generated/linux-x64-default/api.rs"
));
#[cfg(all(target_os = "linux", target_arch = "x86_64", feature = "pdfium-v8"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/bindings/generated/linux-x64-v8-xfa/api.rs"
));
#[cfg(all(
    target_os = "linux",
    target_arch = "aarch64",
    not(feature = "pdfium-v8")
))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/bindings/generated/linux-arm64-default/api.rs"
));
#[cfg(all(target_os = "linux", target_arch = "aarch64", feature = "pdfium-v8"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/bindings/generated/linux-arm64-v8-xfa/api.rs"
));
