# PDFium-oxide

PDFium-oxide is a permissively licensed Rust interface to [PDFium](https://pdfium.googlesource.com/pdfium/), Google's PDF rendering and document library. It is intended to give Rust applications a focused, safe API over PDFium's C interface without requiring each application to maintain its own bindings or native-library integration.

The crate's PDFium ABI baseline is pinned to **PDFium 157.0.8100.0**, built from `chromium/8100`, at official PDFium source commit `f51279699cb59754d1f49aef10eb8e825fe56532`. This is the version selected for this crate, not a promise that it is the newest upstream release. The immutable source identity and binary release are recorded in `Cargo.toml`; the crate exposes the version, branch, and source commit as constants. Checked-in bindings cover the default profile and the V8/XFA profile. The project pins Rust 1.99.1 in `rust-toolchain.toml` and declares Rust 1.99 as its minimum supported version.

## Getting PDFium

PDFium-oxide does not redistribute PDFium binaries. Download the matching native library from the [bblanchon/pdfium-binaries release](https://github.com/bblanchon/pdfium-binaries/releases/tag/chromium%2F8100), selecting the archive for your operating system and architecture. For example, use the Windows x64 archive for `pdfium.dll`, Linux x64 for `libpdfium.so`, or Linux ARM64 for `libpdfium.so` on an ARM64 host. Keep the selected library available at the location configured by the application or loader.

Use the non-V8 archive unless an application specifically needs PDF JavaScript/XFA functionality. The binary distribution repository documents its platform variants and publishes release checksums and attestations.

## Platform scope

The initial native target matrix follows the binary releases: Windows x64, Linux x64, and Linux ARM64. Additional targets can be enabled when the binding and upstream binaries are verified together.

## Runtime loading

Pass an explicit absolute path to `PdfiumConfig::new`; the crate does not search the current directory or `PATH`. Applications that opt into environment configuration can call `PdfiumConfig::from_env`, which reads `PDFIUM_LIBRARY_PATH` and requires a non-empty absolute path. An unset or empty variable returns an actionable configuration error, while a relative value returns `InvalidLibraryPath`. The helper preserves non-Unicode paths and does not change loader resolution. `Pdfium::prepare` loads the library and checks every required export for the selected target profile without initializing it. `PdfiumInit::initialize` initializes the process-owned instance. `Pdfium::load` combines those steps. The first successful configuration owns PDFium for the lifetime of the process; subsequent compatible calls reuse it, while a different path, profile, or renderer is rejected.

The crate serializes native operations through one process-wide call gate. Multi-call safe operations such as `Page::insert_text` retain the gate for their full sequence, so another safe crate operation cannot observe an intermediate state. This does not make a mutation transactional: an error can still leave PDFium's document changed. A synchronous attempt to call back into PDFium on the same thread returns `ReentrantCall` before waiting on that gate. Raw pointer and resource contracts remain the caller's responsibility, so generated raw methods are `unsafe`. Each returned `PdfiumCall<T>` includes the C return value and a numeric `FPDF_GetLastError` snapshot taken while the call gate is held. PDFium defines that error value only for selected failing calls, so it is diagnostic state rather than a universal success indicator.

The `cargo xtask test-runtime-smoke` command downloads and verifies pinned PDFium archives for the current Windows x64 or Linux x64/ARM64 host, extracts them under ignored `target/`, and runs the native smoke and document-workflow tests. Use `--profile default`, `--profile v8`, or `--profile all`; use `--force-download` to fetch the archives again. Ordinary Cargo test runs skip native tests and do not require a PDFium library.

The selected Rust header profile describes available declarations; it does not establish which optional exports are present in the loaded binary. The loader checks those exports at runtime and exposes V8/XFA capability only when the complete optional export set is available. Skia availability is declared by the caller's build profile because it cannot be established from the same optional export check; selecting a profile that does not match the loaded library may cause PDFium initialization to fail. The application remains responsible for supplying a matching PDFium build. Do not initialize or call the same native PDFium library through a second loader or direct FFI while this crate owns it, since those calls bypass its process-wide lifecycle and serialization rules. `destroy_library` is an unsafe terminal operation and requires that all PDFium resources have been released and that no initialized calls will follow.

PDFium processes complex documents and this crate is not a sandbox. Treat input documents as untrusted: the crate does not provide general CPU, memory, or wall-clock limits, and cancellation is cooperative at operation boundaries rather than a guarantee that an in-progress native call can be stopped. Applications that need isolation or hard resource limits should run PDFium in a separately managed process with operating-system controls.

## Safe document workflow

```rust,no_run
use pdfium_oxide::{Pdfium, PdfiumConfig};
use std::path::Path;

let pdfium = Pdfium::load(PdfiumConfig::new("/absolute/path/to/libpdfium.so")?)?;
let document = pdfium.open_document(Path::new("report.pdf"), None)?;
let page = document.page(0)?;
let size = page.size()?;
let text = page.text()?;
let snapshot = page.render(1200, 1600)?.snapshot()?;
assert_eq!(snapshot.stride, snapshot.width as usize * 4);
// snapshot.pixels is an independent, top-to-bottom BGRA byte vector.
# Ok::<(), pdfium_oxide::PdfiumError>(())
```

`open_document_from_bytes` accepts any value convertible to `Arc<[u8]>` and retains the bytes until PDFium closes the document. Passwords are optional UTF-8 strings. `Document`, `Page`, `TextPage`, and `Bitmap` are clonable owners; pages retain their document and text pages retain their page. Drop native owners before calling the unsafe terminal `destroy_library` method. Rendering returns BGRA32 bytes with straight alpha semantics; applications receive a stable copy from `Bitmap::snapshot`.

The native workflow integration test is ignored during ordinary Cargo runs. The xtask smoke command runs it in a separate process from the loader-only smoke test and uses the authored fixtures in `tests/fixtures/`.

### Inspecting text segments

`TextPage::segments()` returns `Result<Vec<TextSegment>, PdfiumError>`. Each segment contains an owned string and `f64` bounds (`left`, `bottom`, `right`, `top`) in page space, measured in PDF points. Each segment corresponds to a rectangle grouped by PDFium's line and text-style rules. The method preserves PDFium's rectangle order and coordinates; it does not split the results into its own lines or words. The returned values remain usable after the text page, page, and document are dropped.

### Inserting page text

`Page::insert_text` adds text at a `Point` in PDF page coordinates using a `TextStyle` with one of PDF's 14 standard fonts, a positive font size, and an RGBA fill color. It changes the native page in place. Safe crate operations are serialized across its complete sequence, so a concurrent save runs before or after the insertion. Save the owning document with `Document::save_to_writer` to persist the change. An error after insertion may leave the page changed; the operation has no rollback guarantee.

### Saving through a Rust writer callback

`Document::save_to_writer` sends PDFium's output to a borrowed `std::io::Write` during one synchronous native call. The writer must not synchronously wait on another thread that calls PDFium; a direct same-thread re-entry is rejected. The method catches panics from the writer when built with `panic=unwind` and returns `PdfiumError::CallbackPanic`. With `panic=abort`, the process aborts and no panic conversion is possible. A writer error or failed save can leave partial bytes in the writer. The panic guarantee applies to this crate-owned adapter, not callbacks passed through the raw API or faults inside PDFium.

## Development and testing

Install the pinned Rust toolchain with `rustup toolchain install 1.99.1 --profile minimal --component rustfmt`, then run the host checks:

```sh
cargo +1.99.1 fmt --all -- --check
cargo +1.99.1 test --offline --locked --all-features
```

To run the native PDFium smoke and document-workflow tests for both profiles, use `cargo xtask test-runtime-smoke --profile all`. The command downloads the pinned binaries into `target/` and validates their published checksums before testing. CI also compiles all six supported target/profile combinations and checks binding reproducibility; see [the CI workflow](.github/workflows/ci.yml) and [binding maintenance notes](bindings/README.md).

## License

PDFium-oxide's original Rust code is dual-licensed under MIT or Apache-2.0, at your option; see [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE). These licenses apply to this repository's code only.

PDFium is separately licensed under its BSD-style three-clause license. The required copyright notice, conditions, and disclaimer are reproduced in [licenses/PDFIUM-LICENSE](licenses/PDFIUM-LICENSE), with the upstream source identified there. Applications that redistribute a PDFium binary must include the PDFium notice and applicable third-party notices with their binary distribution. See [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md) for the redistribution guidance.
