# TextPage Segments Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add owned PDFium-grouped text segments with text and page-space bounds, and verify the agreed safe workflows alongside the existing complete raw function audit.

**Architecture:** Add a public owned `TextSegment` type and implement `TextPage::segments()` using the existing process-wide PDFium coordinator. Keep rectangle enumeration and character lookup in one serialized operation so PDFium's cached rectangles remain valid. Use isolated testable helpers for character-box containment and document-order text assembly; extend the authored native fixture for PDFium-level grouping verification.

**Tech Stack:** Rust 2021, existing generated PDFium bindings and dispatch, standard library, `cargo xtask` maintenance commands, ignored native integration test.

**Spec:** `docs/superpowers/specs/2026-10-19-pdfium-oxide-api-coverage-design.md`

## Global Constraints

- Keep the crate synchronous and add no async-runtime or UI/rendering-framework dependency.
- All PDFium calls, callback execution, error snapshots, and resource cleanup use the single process-wide serialized coordinator.
- No panic from a crate-owned callback adapter may unwind through C.
- TextPage retains its page/document ownership; returned segments own their data and retain no native handles.
- Raw bindings retain their existing ABI and unsafe obligations; do not add safe wrappers for every C function.
- Form filling, interactive form callbacks, async integration, hard cancellation, and additional target platforms remain outside this work.

## Review Focus

- PDFium's `CountRects` cache is overwritten by other text calls: hold the coordinator across count, rectangle, character-box, and Unicode calls; test native segment results with multiple lines/styles.
- Empty or malformed native counts cause invalid conversions or allocations: reject negative counts and use checked conversions; test empty and negative helper inputs.
- Character boxes on rectangle boundaries or overlapping style spans produce lost/reordered text: use inclusive full-box containment and document order; test edge and order cases.
- Text with no Unicode mapping or a character with no valid box causes an extraction-wide failure: omit only that character; test both cases.
- Returned bounds or strings accidentally borrow native state: return owned values without native pointers; verify values remain available after dropping `TextPage` in the native workflow.

## File Structure

- `src/text.rs`: public `TextSegment`, `TextPage::segments()`, and pure matching/assembly helpers.
- `src/lib.rs`: re-export `TextSegment`.
- `tests/document_workflows.rs`: ignored native workflow assertions using the authored fixture.
- `tools/xtask/src/main.rs`: expand the generated PDF fixture with multiple lines and font styles.
- `tools/xtask/src/symbol_audit.rs`: report per-profile declaration, raw wrapper, and coordinator-owned lifecycle counts after successful comparisons.
- `README.md`: document segment ownership, bounds, and PDFium grouping.
- `docs/superpowers/specs/2026-10-19-pdfium-oxide-api-coverage-design.md`: mark segment workflow implemented only after acceptance evidence is complete.

## Task 1: Define and test segment assembly rules

**Files:** Modify `src/text.rs`; test in `src/text.rs`.

**Interfaces:** Add `pub struct TextSegment { pub text: String, pub left: f64, pub bottom: f64, pub right: f64, pub top: f64 }`. Add private testable geometry and assembly helpers. Character inputs are supplied in document order with optional bounds and optional Unicode scalar; rectangle bounds use `f64` and inclusive edges.

- [ ] **Step 1: Write failing helper tests**

  Add `segments_includes_fully_contained_boxes_with_inclusive_edges` and `segments_omits_partial_invalid_and_unmapped_characters` tests. Assert that accepted Unicode scalars remain in document order. Add a checked native-count conversion helper and test zero, positive, negative, and `i32` overflow inputs.

- [ ] **Step 2: Run focused tests and confirm they fail**

  Run: `cargo test --offline --lib text::tests::segments_ -- --nocapture`

  Expected: the new tests fail because the helper does not yet exist.

- [ ] **Step 3: Implement containment and assembly helpers**

  Implement inclusive full-box containment against `left`, `bottom`, `right`, `top`, append accepted Unicode scalars in input order, and reject negative or unrepresentable native counts before allocation. A missing/invalid character box or missing Unicode value contributes no text.

- [ ] **Step 4: Run focused tests and confirm they pass**

  Run: `cargo test --offline --lib text::tests::segments_ -- --nocapture`

  Expected: all new segment assembly tests pass.

## Task 2: Implement the serialized PDFium segment operation

**Files:** Modify `src/text.rs`, `src/lib.rs`.

**Interfaces:** Add `TextPage::segments(&self) -> Result<Vec<TextSegment>, PdfiumError>` and export `TextSegment` from the crate root.

- [ ] **Step 1: Add tests for empty and count/conversion failures**

  Add `segments_returns_empty_for_empty_text` and `segments_rejects_negative_rectangle_count` tests for zero character/rectangle counts and negative native counts returning a structured error before allocation.

- [ ] **Step 2: Run focused tests and confirm the new contract is unmet**

  Run: `cargo test --offline --lib text::tests::segments_ -- --nocapture`

  Expected: FAIL because `segments()` and its count validation are not implemented.

- [ ] **Step 3: Implement `TextPage::segments()`**

  Under one `runtime::call_sequence` coordinator operation, call `FPDFText_CountChars`, then `FPDFText_CountRects(handle, 0, char_count)`. For every rectangle index in PDFium order, call `FPDFText_GetRect`; inspect all characters in document order using `FPDFText_GetCharBox` and `FPDFText_GetUnicode`; assemble text using Task 1 helpers. Treat a failed rectangle retrieval as an operation error. Omit an individual character if its box or Unicode lookup is unavailable. Copy all coordinates and text into owned values. Apply the existing callback-source error check after the serialized sequence, and use checked conversions and fallible reservations for counts and output storage.

- [ ] **Step 4: Run host tests and formatting**

  Run: `cargo test --offline --all-features`

  Run: `cargo fmt --all -- --check`

  Expected: all host tests pass and formatting check succeeds.

## Task 3: Verify PDFium grouping and owned results natively

**Files:** Modify `tools/xtask/src/main.rs`, regenerate `tests/fixtures/document-workflow.pdf`, modify `tests/document_workflows.rs`.

**Interfaces:** Keep the existing `cargo xtask create-fixture` command. Extend its authored page content/resources with text on multiple lines and at least two font styles on one line. Native assertions use public `TextPage::segments()` and validate PDFium-provided rectangles, text ordering, grouping, and owned-result lifetime.

- [ ] **Step 1: Add ignored native assertions for segment order, grouping, bounds, and ownership**

  Assert the fixture yields PDFium's distinct line/style spans in rectangle order; each segment's bounds have `left <= right` and `bottom <= top`; and segment text matches characters whose boxes are within the reported rectangle. Drop the `TextPage`, `Page`, and `Document`, then assert the returned segment values remain readable.

- [ ] **Step 2: Run native test to confirm the assertions expose missing behavior**

  Run: `cargo test --offline --test document_workflows complete_document_text_and_render_workflow -- --ignored --exact --nocapture`

  Expected: FAIL until `segments()` and multi-style fixture content are present.

- [ ] **Step 3: Extend and regenerate the authored fixture**

  Update `create_fixture` to include the extra standard font resource and text content that forms at least two lines and two distinct styles on one line, then run `cargo xtask create-fixture`. Keep the fixture deterministic and authored by the repository's generator.

- [ ] **Step 4: Run the native workflow**

  Run: `cargo xtask test-runtime-smoke --profile default`

  Expected: smoke and ignored native document workflow pass using the pinned matching binary. If the host binary is unavailable, report native integration as unverified rather than passed.

## Task 4: Document and reconcile coverage evidence

**Files:** Modify `README.md`, `tools/xtask/src/symbol_audit.rs`, and the coverage design spec.

- [ ] **Step 1: Document the safe segment contract**

  Add a concise API section explaining that `TextPage::segments()` returns owned strings and PDFium-grouped rectangles with `f64` page-space bounds in PDF points; PDFium's line/style grouping and rectangle order are preserved.

- [ ] **Step 2: Report raw audit counts and run the six-profile audit**

  After the header/manifest/wrapper comparisons pass, print each configuration's header declaration count, public raw wrapper count, and coordinator-owned lifecycle count. Keep the existing mismatch checks unchanged.

  Run: `cargo xtask audit-bindings`

  Expected: the audit reports all six generated configurations pass and prints those three counts for each. Treat this as raw function-name reach evidence, not full ABI type/constant validation.

- [ ] **Step 3: Update the coverage ledger from evidence**

  Mark `Inspect text segments` implemented only if host tests pass and the native workflow passed. Otherwise retain an explicit unverified native status. Record raw audit evidence independently from safe workflow evidence; do not claim all raw API semantics or all ABI types are behaviorally verified.

- [ ] **Step 4: Run final local checks**

  Run: `cargo fmt --all -- --check`

  Run: `cargo test --offline --all-features`

  Run: `cargo xtask audit-bindings`

  Expected: all three commands pass. Native coverage is reported separately according to Task 3's result.
