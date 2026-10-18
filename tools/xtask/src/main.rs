use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod bindings;
mod smoke;
mod symbol_audit;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        return Err("usage: cargo xtask <command> [options]\ncommands: audit-bindings, check-bindings, regenerate-bindings, check-generator, test-runtime-smoke, fetch-pdfium-source, create-fixture".into());
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("could not locate repository root from xtask manifest")?
        .to_path_buf();
    let arguments = args.collect::<Vec<_>>();
    match command.as_str() {
        "audit-bindings" => symbol_audit::run(
            &root,
            &option_path(
                &arguments,
                "--pdfium-source",
                root.join("target/pdfium-source-pinned"),
            )?,
        ),
        "check-bindings" => bindings::check_generated(&root),
        "regenerate-bindings" => bindings::regenerate(&root, &arguments),
        "check-generator" => bindings::check_generator(&root, &arguments),
        "test-runtime-smoke" => smoke::run(&root, &arguments),
        "fetch-pdfium-source" => bindings::fetch_source(&root, &arguments),
        "create-fixture" => create_fixture(&root),
        _ => Err(format!("unknown xtask command '{command}'").into()),
    }
}

fn create_fixture(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let content = b"BT /F1 18 Tf 20 70 Td (PDFium Workflow Fixture) Tj ET\n1 0 0 rg 20 15 40 30 re f\n0 0 1 rg 80 15 40 30 re f\n/GS1 gs 0 1 0 rg 140 15 40 30 re f\n";
    let objects: Vec<Vec<u8>> = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Resources << /Font << /F1 4 0 R >> /ExtGState << /GS1 6 0 R >> >> /Contents 5 0 R >>".to_vec(),
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec(),
        [format!("<< /Length {} >>\nstream\n", content.len()).as_bytes(), content, b"endstream"].concat(),
        b"<< /Type /ExtGState /ca 0.5 /CA 0.5 >>".to_vec(),
    ];
    let mut pdf = b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n".to_vec();
    let mut offsets = vec![0_usize];
    for (index, object) in objects.iter().enumerate() {
        offsets.push(pdf.len());
        write!(pdf, "{} 0 obj\n", index + 1)?;
        pdf.extend_from_slice(object);
        pdf.extend_from_slice(b"\nendobj\n");
    }
    let xref = pdf.len();
    write!(pdf, "xref\n0 {}\n0000000000 65535 f \n", offsets.len())?;
    for offset in offsets.iter().skip(1) {
        write!(pdf, "{offset:010} 00000 n \n")?;
    }
    write!(
        pdf,
        "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
        offsets.len()
    )?;
    let output = root.join("tests/fixtures/document-workflow.pdf");
    std::fs::write(&output, pdf)?;
    println!("Wrote {}", output.display());
    Ok(())
}

fn option_path(
    args: &[String],
    option: &str,
    default: PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    match args.iter().position(|arg| arg == option) {
        Some(index) => args
            .get(index + 1)
            .map(PathBuf::from)
            .ok_or_else(|| format!("{option} requires a path").into()),
        None => Ok(default),
    }
}
