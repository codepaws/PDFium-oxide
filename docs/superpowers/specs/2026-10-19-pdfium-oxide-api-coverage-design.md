# PDFium-oxide API Coverage Design

## Purpose

Define a verifiable meaning of complete API coverage for PDFium-oxide and specify the next safe workflow to add: owned text segments from `TextPage`. The target is complete raw C function reach plus the safe workflows already agreed in the crate specification and API follow-on plan. It does not require a safe Rust method for every C function, since many raw functions depend on caller-managed pointers, handles, and lifetimes that cannot be made generally safe.

## Scope and completion criteria

Coverage has two independently reported parts:

1. **Raw C function reach.** For each supported target/profile, every function declared by the pinned PDFium public headers is represented in the generated bindings and reachable through the crate's public unsafe raw layer, except the three library lifecycle functions owned by the runtime coordinator. The existing symbol audit remains the enforcement mechanism. It compares header declarations, dispatch entries, the runtime manifest, and public raw wrapper methods. The report must identify the pinned PDFium revision and target/profile; it must not imply that function-name reach alone validates every ABI type, constant, or semantic contract.
2. **Agreed safe workflows.** Every workflow in the ledger below has a public safe entry point, documented ownership/error behavior, and host contract tests plus native integration coverage where a matching PDFium binary is available. The ledger is the source of truth for this scope; newly proposed workflows require a separate scope decision.

This definition deliberately does not mean that every C function has an idiomatic safe wrapper. Unsafe raw access remains available for specialized operations and carries the caller's obligations.

## Safe workflow coverage ledger

| Workflow | Current status | Acceptance |
| --- | --- | --- |
| Configure and load PDFium | Implemented | Explicit path/bytes preparation and opt-in environment path; lifecycle is coordinator-owned. |
| Open documents | Implemented | Path, bytes, and persistent random-access source; source state survives for the native document lifetime and callback failures are contained and reported. |
| Inspect pages | Implemented | Page count, page access, and page dimensions with managed lifetimes and structured errors. |
| Extract page text | Implemented; spans specified below | Owned UTF-8 page text plus owned style/line spans with PDFium-provided bounds. |
| Render pages and access pixels | Implemented | Rendering, owned snapshots, and a borrowed pixel view whose guard prevents crate-mediated mutation while alive. |
| Save documents | Implemented | Save to a Rust writer with callback and native failure propagation. |
| Insert page text | Implemented | Validated in-place text insertion, with native object ownership handled correctly and no rollback guarantee. |
| Inspect text segments | Implemented | `TextPage::segments()` returns owned text and page-space bounds for PDFium's merged text rectangles. Host tests passed with `cargo test --offline --all-features`; the pinned Windows x64 default native workflow passed with `cargo xtask test-runtime-smoke --profile default`. |

The text-segment native result covers the Windows x64 default profile. The ordinary host suite ignores native tests; its passing result is separate from the native workflow result.

### Raw function-name reach evidence

`cargo xtask audit-bindings --pdfium-source D:/projects/detect/PDFium-oxide/target/pdfium-source-pinned` passed against pinned PDFium commit `f51279699cb59754d1f49aef10eb8e825fe56532` on Windows x64. Counts below are per generated configuration and follow successful header/dispatch, header/manifest, manifest/wrapper, and provenance comparisons.

| Target/profile | Header declarations | Public raw wrappers | Coordinator-owned lifecycle functions |
| --- | ---: | ---: | ---: |
| Linux ARM64 default | 469 | 466 | 3 |
| Linux ARM64 V8/XFA | 474 | 471 | 3 |
| Linux x64 default | 469 | 466 | 3 |
| Linux x64 V8/XFA | 474 | 471 | 3 |
| Windows x64 default | 471 | 468 | 3 |
| Windows x64 V8/XFA | 476 | 473 | 3 |

These counts establish function-name reach through the generated raw layer. They do not establish ABI type or constant parity or native behavior for every function.

### Deferred from this coverage target

- Form filling and interactive form callbacks require a separate design for form environments, callback lifetimes, ownership, and saving behavior.
- Async runtime integration and hard cancellation are separate concerns. Async cancellation of a waiter does not stop a synchronous PDFium call; hard cancellation requires process isolation.
- Additional operating systems and architectures require a separate support decision and validation against available pinned PDFium binaries.

## TextPage segment API

Add an owned public `TextSegment` value and `TextPage::segments() -> Result<Vec<TextSegment>, PdfiumError>`. Each segment contains:

- `text: String`
- `left`, `bottom`, `right`, and `top` bounds as `f64`, in PDFium page user-space coordinates (PDF points for the crate's documented page-coordinate contract).

The method must use `FPDFText_CountRects` and `FPDFText_GetRect` over the full text page. PDFium groups these rectangles by its existing line and text-style rules; the wrapper must preserve that grouping and rectangle order. It must not invent line, word, or character-based spans.

For each returned rectangle, inspect character boxes and Unicode values across the text page in document order. Include a character when its complete character box is inside the rectangle, with rectangle edges treated as inclusive. Append the character's Unicode value to that segment. A character with a zero Unicode value contributes no text. A character whose box cannot be read is omitted from that segment. Do not reorder characters or synthesize separators. This matches the requested rule that segment text consists of the characters whose boxes fall inside the corresponding PDFium rectangle.

An empty page returns an empty vector. Each call returns independent owned strings and values; no PDFium pointers or temporary buffers escape. Preserve the coordinate values and orientation returned by PDFium without normalizing or flipping them.

## Native call sequencing and errors

The rectangle count, each rectangle retrieval, and the character-box/Unicode queries form one serialized PDFium operation. The whole sequence must run under the process-wide coordinator so another text operation cannot replace PDFium's cached rectangles between `FPDFText_CountRects` and `FPDFText_GetRect`. Callback-backed document failures must be checked and propagated using the existing source-error mechanism.

Reject negative rectangle or character counts and failed rectangle retrievals as structured operation errors. Propagate native error state using existing crate conventions. Validate conversions and allocation sizes before allocating or calling PDFium. If a PDFium character box or Unicode lookup fails for an individual character, omit that character rather than failing the entire extraction; failures in rectangle enumeration or retrieval fail the operation.

TextPage retains its page/document ownership as it does today. The returned segments do not retain native handles and remain usable after the TextPage is dropped.

## Verification requirements

### Raw function reach

- Run the existing symbol audit for every currently supported target/profile: Windows x64 default and V8/XFA, Linux x64 default and V8/XFA, and Linux ARM64 default and V8/XFA.
- Report function declaration and wrapper counts per profile and account for coordinator-owned lifecycle functions explicitly.
- Keep the audit scoped accurately: it proves declared function-name reach through the raw wrapper path, not all type/constant parity or behavioral correctness.

### Text segment behavior

- Host tests cover rectangle matching, inclusive containment, document-order preservation, omitted unmapped/invalid characters, empty input, checked conversions, and allocation/error behavior through testable helpers.
- Native integration fixtures include multiple lines and multiple text styles on a line, and verify that segment boundaries and order follow PDFium's `CountRects`/`GetRect` grouping rather than wrapper-generated splits.
- Native integration verifies the reported coordinates against the values obtained from PDFium and confirms segment values remain usable after dropping the `TextPage`.
- Public rustdoc and README examples document ownership, coordinate units, and PDFium's grouping behavior.

### Ledger verification

- Each implemented workflow has a corresponding public API and its documented contract.
- Host and native verification results are reported separately; unavailable native binaries are stated as an unverified environment limitation, not as passed coverage.
- The ledger status is updated only after the relevant acceptance evidence exists.

## Non-goals

- Safe wrappers for every raw C function.
- Form APIs, callback registration, asynchronous execution, or cancellation.
- Changing PDFium's text grouping, coordinate origin/orientation, or rectangle order.
- Claiming complete ABI/type/constant correctness solely from function-name symbol auditing.
- Expanding the supported platform matrix in this work.
