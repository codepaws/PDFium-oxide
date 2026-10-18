use pdfium_oxide::{PDFIUM_CHROMIUM_BRANCH, PDFIUM_SOURCE_COMMIT, PDFIUM_VERSION};

#[test]
fn baseline_constants_match_the_pinned_binary_release() {
    assert_eq!(PDFIUM_VERSION, "157.0.8100.0");
    assert_eq!(PDFIUM_CHROMIUM_BRANCH, "chromium/8100");
    assert_eq!(
        PDFIUM_SOURCE_COMMIT,
        "f51279699cb59754d1f49aef10eb8e825fe56532"
    );
}
