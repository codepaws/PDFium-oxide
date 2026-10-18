# Generated PDFium bindings

The checked-in declarations use this crate's pinned PDFium binary baseline from [bblanchon/pdfium-binaries](https://github.com/bblanchon/pdfium-binaries/releases/tag/chromium%2F8100): PDFium **157.0.8100.0**, Chromium branch `chromium/8100`, official PDFium source commit `f51279699cb59754d1f49aef10eb8e825fe56532`.

`configurations.toml` lists the generated target and build profiles. The six configurations cover Windows x64, Linux x64, and Linux ARM64, with the default non-V8 profile and the V8/XFA profile. The bblanchon build script enables V8 and XFA together. The `pdfium-v8` Cargo feature selects the V8/XFA declarations for the current Rust target; it does not verify that the application supplies a library with those capabilities.

Each `generated/<configuration>/` directory contains:

- `types.rs`: public C types and constants for that target and profile.
- `dispatch.rs`: generated function pointer declarations and loader helpers. This module is private; safe and unsafe raw API calls go through the process-wide coordinator, which owns loading, serialization, error snapshots, and cleanup.
- `provenance.json`: exact PDFium commit, header paths, configuration defines, bindgen version, and Clang version used.

`generated/toolchain.json` records the shared toolchain and generated API symbol counts. The checked-in output is used directly by Cargo; normal builds do not require a PDFium checkout, native PDFium library, Clang, or bindgen.

## Regeneration

On Windows, install Clang with `libclang` available, then install the pinned generator:

```powershell
cargo install bindgen-cli --version 0.72.1 --locked
```

Fetch the exact public-header checkout, then regenerate all six configurations:

```sh
cargo xtask fetch-pdfium-source
cargo xtask regenerate-bindings
```

The command verifies the full source commit, public header inventory, bindgen version, and Clang version before writing generated output. It generates to a temporary directory first, then updates only `bindings/generated/`. The Linux `time.h` compatibility header supplies the LP64 `time_t` alias and opaque `struct tm` required by `fpdf_ext.h` when generating on a Windows host without a Linux sysroot.

After regeneration, inspect the generated diff and run:

```sh
cargo xtask check-generator --bindgen bindgen --clang clang
cargo xtask check-bindings
cargo xtask audit-bindings
cargo check --offline --all-targets --all-features
cargo check --offline --target x86_64-unknown-linux-gnu --all-features
cargo check --offline --target aarch64-unknown-linux-gnu --all-features
```

The header audit is run with `cargo xtask audit-bindings`. It parses the pinned
public headers separately for each target and define set, then compares that
inventory with the generated dispatch table, runtime symbol manifest, and raw
wrappers. A mismatch reports the configuration and sorted missing/extra names;
coordinator-owned lifecycle functions are accounted for by the runtime instead
of ordinary raw wrappers. Clang must be available on `PATH`; set `CLANG` to use
a specific executable. The pinned PDFium checkout defaults to
`target/pdfium-source-pinned` when `--pdfium-source` is omitted.
When regenerating Windows bindings on Linux, install MinGW headers and set
`WINDOWS_INCLUDE` to their include directory (Ubuntu's
`/usr/x86_64-w64-mingw32/include`); CI uses this setup. On Windows, Clang must
be installed with a Windows SDK or MinGW headers available.

Run the generator a second time with the same source and toolchain and confirm the generated diff is clean. Review all changed declarations against the official public headers before upgrading the PDFium baseline.
