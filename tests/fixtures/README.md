# Document workflow fixtures

`document-workflow.pdf` is authored for PDFium-oxide. It contains one 200 × 100 point page, the text “PDFium Workflow Fixture”, asymmetric red and blue rectangles, a half-opacity green rectangle, and untouched transparent background.

`document-workflow-encrypted.pdf` is the AES-256 encrypted derivative. Its user password is `secret`; owner password is `owner-secret`.

Regenerate the plain fixture with Cargo xtask, then use qpdf to create and check the encrypted derivative:

```sh
cargo xtask create-fixture
qpdf --encrypt secret owner-secret 256 -- tests/fixtures/document-workflow.pdf tests/fixtures/document-workflow-encrypted.pdf
qpdf --check tests/fixtures/document-workflow.pdf
qpdf --password=secret --check tests/fixtures/document-workflow-encrypted.pdf
```

The fixture is original test material and is not derived from a third-party PDF sample.
