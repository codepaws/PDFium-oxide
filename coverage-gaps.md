# Coverage Gap Closure Checklist

**Goal:** Improve meaningful Rust wrapper coverage by targeting behavior uncovered by the initial native-inclusive report, without adding artificial assertions or changing semantics just to raise a percentage.

**Reports refreshed:** 2026-10-19 with `cargo-llvm-cov 0.9.1` on Windows x64; generated bindings excluded; PDFium 157.0.8100.0. The all-features host report (native tests ignored) covers 1,824/2,688 lines (67.86%), 2,598/3,930 regions (66.11%), and 237/348 functions (68.10%). The default-profile native-inclusive report covers 2,377/2,688 lines (88.43%), 3,492/3,930 regions (88.85%), and 304/348 functions (87.36%). The lower host-only percentage reflects safe native workflows being intentionally ignored unless a matching PDFium library is provided.

## Per-area checklist

| Area | Initial line coverage | Current line coverage | Work completed | Remaining gap assessment |
|---|---:|---:|---|---|
| `src/document.rs` | 185/247 (74.9%) | 220/271 (81.2%) | Added valid path/password and load-error classification unit checks; native workflow now covers malformed PDFs, out-of-range pages, encrypted memory documents without a password, and source read failure propagation. | Negative native page count, null page for an in-range index, and failed native save without callback error require faulting PDFium; the pinned fixture does not trigger these reliably. |
| `src/page.rs` | 282/341 (82.7%) | 305/365 (83.6%) | Added standard-font acceptance and invalid font-size tests. Native workflow covers dimensions, insertion, text extraction and render. | Native failures from font/object creation or mutation, page/text handle load failures, and invalid native dimensions require controlled PDFium fault injection. |
| `src/runtime/mod.rs` | 620/679 (91.3%) | 703/777 (90.5%) | Added a gate-held sequence helper for multi-call mutations and a competing-operation serialization test; native workflow verifies insertion and saving. | Remaining lines depend on mutating process-global runtime state or native callbacks/failures; isolated tests would require a broad fake runtime and risk fragile coupling. |
| `src/callback.rs` | 287/339 (84.7%) | 300/353 (85.0%) | Added null source/writer interface tests and Unix 64-bit oversized callback block coverage. | Windows report cannot count the Unix-only size branch; some oversized values are unrepresentable on Windows. Existing callback error and panic handling has direct unit coverage. |
| `src/render.rs` | 191/225 (84.9%) | 232/263 (88.2%) | Added integer-boundary and poisoned-lock coverage plus `Bitmap::snapshot` re-entry and lock-failure tests; native workflow covers successful render, BGRA layout and snapshot metadata. | Bitmap allocation, creation, rendering and native buffer failures require native faults; OOM testing would be nondeterministic. |
| `src/runtime/loader.rs` | 105/131 (80.2%) | 112/138 (81.2%) | Existing tests enumerate every required and optional symbol failure; added empty optional-manifest case. Real PDFium load passes on all tested architectures/profiles. | Dispatch failure after required symbol validation is not expected because dispatch resolution uses the same validated required exports. |
| `src/text.rs` | 107/122 (87.7%) | 119/133 (89.5%) | Added negative count, invalid surrogate ordering, supplementary Unicode and trailing-buffer tests; native workflow covers successful extraction. | Negative native counts, text callback errors and inconsistent native buffer results require PDFium fault injection. |
| `src/config.rs` | 93/98 (94.9%) | 117/117 (100%) | Added AGG capability/native config and Skia profile transition checks. | Closed in the current report. |
| `src/handles.rs` | 85/87 (97.7%) | 101/103 (98.1%) | Added retained-value drop-order check; existing tests cover clone finalization and child-before-parent close. | Remaining branch requires manually running `Drop` twice on one still-live value, which is not a valid ownership flow. |
| `src/error.rs` | 168/168 (100%) | 168/168 (100%) | Existing variant display test covers all error variants. | No line gap. |

## Verification

- [x] Full Windows host suite: `cargo test --offline --all-features` (58 unit tests, 10 config tests, raw bindings and metadata tests, 2 doctests; native integration tests are ignored by default).
- [x] Default-profile native-inclusive and all-features host coverage reports regenerated on Windows x64.
- [x] Linux x64 `p3pro`: Rust 1.99.1 format/host checks and native smoke/document workflow under default and V8/XFA PDFium passed.
- [x] Linux ARM64 `spark`: Rust 1.99.1 format/host checks and native smoke/document workflow under default and V8/XFA PDFium passed.
- [x] Fresh Sol medium review of the grouped insert sequence found no confirmed actionable defect; it noted that the gate concurrency test is synthetic rather than a live concurrent PDFium save test.

**Scope limit:** The remaining items above need controlled native fault injection, an artificially invalid ownership flow, or platform conditions absent from the Windows coverage host. They are recorded rather than covered by tests that claim unrealistic guarantees.
