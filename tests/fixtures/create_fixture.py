"""Generate the original PDFium-oxide document workflow fixture."""
from pathlib import Path

root = Path(__file__).parent
stream = (
    b"BT /F1 18 Tf 20 70 Td (PDFium Workflow Fixture) Tj ET\n"
    b"1 0 0 rg 20 15 40 30 re f\n"
    b"0 0 1 rg 80 15 40 30 re f\n"
    b"/GS1 gs 0 1 0 rg 140 15 40 30 re f\n"
)
objects = [
    b"<< /Type /Catalog /Pages 2 0 R >>",
    b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
    b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Resources << /Font << /F1 4 0 R >> /ExtGState << /GS1 6 0 R >> >> /Contents 5 0 R >>",
    b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
    b"<< /Length " + str(len(stream)).encode() + b" >>\nstream\n" + stream + b"endstream",
    b"<< /Type /ExtGState /ca 0.5 /CA 0.5 >>",
]
pdf = bytearray(b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n")
offsets = [0]
for number, obj in enumerate(objects, 1):
    offsets.append(len(pdf))
    pdf.extend(f"{number} 0 obj\n".encode() + obj + b"\nendobj\n")
xref = len(pdf)
pdf.extend(f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode())
for offset in offsets[1:]:
    pdf.extend(f"{offset:010d} 00000 n \n".encode())
pdf.extend(
    f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
)
(root / "document-workflow.pdf").write_bytes(pdf)
