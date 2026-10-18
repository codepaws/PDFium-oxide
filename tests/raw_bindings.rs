use pdfium_oxide::raw::{self, FPDF_DOCUMENT};

#[cfg(all(target_os = "windows", not(feature = "pdfium-v8")))]
const EXPECTED_CONFIGURATION: raw::HeaderConfiguration =
    raw::HeaderConfiguration::WindowsX64Default;
#[cfg(all(target_os = "windows", feature = "pdfium-v8"))]
const EXPECTED_CONFIGURATION: raw::HeaderConfiguration = raw::HeaderConfiguration::WindowsX64V8Xfa;
#[cfg(all(
    target_os = "linux",
    target_arch = "x86_64",
    not(feature = "pdfium-v8")
))]
const EXPECTED_CONFIGURATION: raw::HeaderConfiguration = raw::HeaderConfiguration::LinuxX64Default;
#[cfg(all(target_os = "linux", target_arch = "x86_64", feature = "pdfium-v8"))]
const EXPECTED_CONFIGURATION: raw::HeaderConfiguration = raw::HeaderConfiguration::LinuxX64V8Xfa;
#[cfg(all(
    target_os = "linux",
    target_arch = "aarch64",
    not(feature = "pdfium-v8")
))]
const EXPECTED_CONFIGURATION: raw::HeaderConfiguration =
    raw::HeaderConfiguration::LinuxArm64Default;
#[cfg(all(target_os = "linux", target_arch = "aarch64", feature = "pdfium-v8"))]
const EXPECTED_CONFIGURATION: raw::HeaderConfiguration = raw::HeaderConfiguration::LinuxArm64V8Xfa;

#[test]
fn exposes_only_the_selected_header_profile_and_public_types() {
    assert_eq!(raw::HEADER_CONFIGURATION, EXPECTED_CONFIGURATION);
    let document: Option<FPDF_DOCUMENT> = None;
    assert!(document.is_none());
    assert_eq!(
        raw::PDFIUM_SOURCE_COMMIT,
        "f51279699cb59754d1f49aef10eb8e825fe56532"
    );
}
