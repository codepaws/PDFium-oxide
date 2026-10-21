# PDFium-oxide

PDFium-oxide is a Rust wrapper for [PDFium](https://pdfium.googlesource.com/pdfium/), Google's PDF rendering and document library. It provides checked, owned Rust workflows alongside access to the complete public PDFium C API.

## How it works

The application supplies a PDFium shared library (`pdfium.dll` or `libpdfium.so`). PDFium-oxide loads it at runtime, checks the exports needed by the selected build profile, and serializes native calls through one process-wide gate. Safe document, page, text, and bitmap handles retain their parent resources and close them when dropped. The raw C API remains `unsafe` because pointer, buffer, and lifetime requirements come from PDFium.

PDFium calls are synchronous. The crate does not sandbox documents or impose general CPU, memory, or time limits; an in-progress native call cannot be forcibly cancelled.

## Feature coverage

### Available

- [x] Generated bindings for every public declaration in the pinned PDFium headers: Windows x64, Linux x64, and Linux ARM64, with default and V8/XFA profiles.
- [x] Runtime loading, required-export checks, renderer/profile configuration, capability reporting, and process lifecycle management.
- [x] Open PDFs from a path, owned bytes, or a custom seekable source; inspect pages and dimensions; save through a Rust `Write` implementation.
- [x] Extract text and owned text segments with PDF-point bounds, grouped by PDFium's line and text-style rectangles.
- [x] Render pages to BGRA pixels, with borrowed pixel views and owned snapshots.
- [x] Insert text using PDF's 14 standard fonts.

### Deferred follow-on features

- [ ] Higher-level safe form workflows (the raw C calls remain available through the unsafe API).
- [ ] An asynchronous runtime or async-facing API.
- [ ] Hard cancellation of an in-progress PDFium call.

## Usage

Set `PDFIUM_LIBRARY_PATH` to the absolute path of the PDFium library, or put the platform library file in the current directory and use the opt-in fallback helper:

```rust,no_run
use pdfium_oxide::{Pdfium, PdfiumConfig};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdfium = Pdfium::load(PdfiumConfig::from_env_or_current_dir()?)?;
    let document = pdfium.open_document(Path::new("report.pdf"), None)?;
    let page = document.page(0)?;

    println!("Page size: {:?}", page.size()?);
    println!("Text: {}", page.text()?);
    let segments = page.text_page()?.segments()?;
    let image = page.render(1200, 1600)?.snapshot()?;
    println!(
        "{} text segments; {}×{} BGRA image",
        segments.len(), image.width, image.height
    );
    Ok(())
}
```

`TextPage::segments()` returns owned text and PDF-point bounds for each rectangle PDFium groups by line and text style. The returned segments remain usable after the native document handles are dropped.

## PDFium binary setup

The crate does **not** bundle or download PDFium at runtime. Download the matching archive from the [pinned `bblanchon/pdfium-binaries` release](https://github.com/bblanchon/pdfium-binaries/releases/tag/chromium%2F8100), choose the operating system, architecture, and profile you need, and extract the shared library. Use the V8/XFA archive only when your application needs those features and enable Cargo's `pdfium-v8` feature to select the matching declarations.

Set the library path in your environment:

```powershell
$env:PDFIUM_LIBRARY_PATH = 'C:\path\to\pdfium.dll'
```

```sh
export PDFIUM_LIBRARY_PATH=/path/to/libpdfium.so
```

`PdfiumConfig::from_env()` requires `PDFIUM_LIBRARY_PATH` to be set. The opt-in `PdfiumConfig::from_env_or_current_dir()` uses that variable when present; otherwise, it checks only `pdfium.dll` (Windows) or `libpdfium.so` (Linux) directly in the current working directory. It does not search `PATH` or subdirectories. An empty or invalid environment value remains an error. Since this can load native code from the current directory, use it only when that directory is trusted. You can also pass an explicit absolute path to `PdfiumConfig::new`.

For local native smoke tests, `cargo xtask test-runtime-smoke` downloads and verifies the pinned archive for the current supported host. This is a development helper; applications still provide their own PDFium binary.

## PDFium version and licensing

The checked-in bindings target PDFium **157.0.8100.0**, Chromium branch `chromium/8100`, source commit `f51279699cb59754d1f49aef10eb8e825fe56532`. This is the crate's pinned ABI baseline; it is not a promise that it is the newest upstream release.

**PDFium's binary has its own BSD-style three-clause license.** If you redistribute a PDFium binary with your application, include the required PDFium copyright notice, license conditions, disclaimer, and applicable third-party notices. See [the PDFium license](licenses/PDFIUM-LICENSE) and [redistribution notices](THIRD-PARTY-NOTICES.md). The Rust wrapper itself is dual-licensed under MIT or Apache-2.0.

## Development

The repository pins Rust 1.99.1. Host checks:

```sh
cargo +1.99.1 fmt --all -- --check
cargo +1.99.1 test --offline --locked --all-features
```

To run native tests for both PDFium profiles on the current supported host:

```sh
cargo +1.99.1 xtask test-runtime-smoke --profile all
```

Binding regeneration, symbol audits, and their prerequisites are documented in [the bindings guide](bindings/README.md). [CI](.github/workflows/ci.yml) checks host tests, native smoke tests, supported target/profile builds, and binding reproducibility.
