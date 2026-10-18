use serde::Deserialize;
use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const EXPECTED_COMMIT: &str = "f51279699cb59754d1f49aef10eb8e825fe56532";

#[derive(Deserialize)]
struct Provenance {
    target: String,
    #[serde(default)]
    defines: Vec<String>,
    generated_api_symbol_count: usize,
}

#[derive(Deserialize)]
struct ManifestEntry {
    name: String,
    phase: String,
}

#[derive(Deserialize)]
struct AstNode {
    #[serde(default)]
    kind: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    inner: Vec<AstNode>,
}

pub fn run(root: &Path, source: &Path) -> Result<(), Box<dyn Error>> {
    let source = if source.is_absolute() {
        source.to_path_buf()
    } else {
        env::current_dir()?.join(source)
    };
    if !source.is_dir() {
        return Err(format!(
            "PDFium source directory does not exist: {}",
            source.display()
        )
        .into());
    }
    let actual_commit = output(
        Command::new("git")
            .arg("-C")
            .arg(&source)
            .args(["rev-parse", "HEAD"]),
    )?;
    if actual_commit.trim() != EXPECTED_COMMIT {
        return Err(format!(
            "PDFium source checkout must be at pinned commit {EXPECTED_COMMIT}; found {}",
            actual_commit.trim()
        )
        .into());
    }

    let public = source.join("public");
    let mut headers = fs::read_dir(&public)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension() == Some(OsStr::new("h"))
                && path
                    .file_name()
                    .and_then(OsStr::to_str)
                    .is_some_and(|name| name.starts_with("fpdf"))
        })
        .collect::<Vec<_>>();
    headers.sort();
    if headers.is_empty() {
        return Err(format!("no fpdf*.h public headers found under {}", public.display()).into());
    }

    let temp = root
        .join("target")
        .join(format!("symbol-audit-{}", std::process::id()));
    fs::create_dir_all(&temp)?;
    let result = audit_configurations(root, &public, &headers, &temp);
    let _ = fs::remove_dir_all(&temp);
    result?;
    println!("Public header API audit passed for all generated configurations.");
    Ok(())
}

fn audit_configurations(
    root: &Path,
    public: &Path,
    headers: &[PathBuf],
    temp: &Path,
) -> Result<(), Box<dyn Error>> {
    let clang = env::var_os("CLANG").unwrap_or_else(|| "clang".into());
    let resource = output(Command::new(&clang).arg("-print-resource-dir"))?;
    let wrapper = temp.join("wrapper.h");
    {
        let mut file = fs::File::create(&wrapper)?;
        for header in headers {
            writeln!(
                file,
                "#include <{}>",
                header.file_name().unwrap().to_string_lossy()
            )?;
        }
    }

    let generated = root.join("bindings/generated");
    let mut configurations = fs::read_dir(&generated)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && path.join("provenance.json").is_file())
        .collect::<Vec<_>>();
    configurations.sort();
    if configurations.len() != 6 {
        return Err(format!(
            "expected six generated configurations, found {}",
            configurations.len()
        )
        .into());
    }

    for directory in configurations {
        let configuration = directory.file_name().unwrap().to_string_lossy();
        let provenance: Provenance =
            serde_json::from_slice(&fs::read(directory.join("provenance.json"))?)?;
        let mut command = Command::new(&clang);
        command.arg(format!("--target={}", provenance.target));
        if provenance.target.contains("linux") {
            command
                .args(["-nostdinc", "-isystem"])
                .arg(Path::new(resource.trim()).join("include"))
                .arg("-I")
                .arg(root.join("bindings/compat/linux"));
        } else if provenance.target.contains("windows") {
            if let Some(include) = std::env::var_os("WINDOWS_INCLUDE") {
                command.arg("-I").arg(include);
            }
            command.arg("-I").arg(root.join("bindings/compat/windows"));
        }
        command.arg("-I").arg(public);
        for define in provenance.defines {
            command.arg(format!("-D{define}"));
        }
        command
            .args(["-Xclang", "-ast-dump=json", "-fsyntax-only"])
            .arg(&wrapper);
        let output = command.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let tail = stderr
                .lines()
                .rev()
                .take(10)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join(" ");
            return Err(format!(
                "Clang AST extraction failed for '{configuration}' ({command:?}): {tail}"
            )
            .into());
        }

        let ast: AstNode = serde_json::from_slice(&output.stdout)?;
        let mut source_symbols = BTreeSet::new();
        collect_functions(&ast, &mut source_symbols);

        let dispatch = fs::read_to_string(directory.join("dispatch.rs"))?;
        let table = extract_dispatch(&dispatch).ok_or_else(|| {
            format!("Generated dispatch table was not found for '{configuration}'.")
        })?;
        let table_symbols = rust_symbols(table, r"pub ");
        compare(
            &configuration,
            "header/dispatch",
            &source_symbols,
            &table_symbols,
        )?;

        let manifest: Vec<ManifestEntry> =
            serde_json::from_slice(&fs::read(directory.join("symbols.json"))?)?;
        let manifest_symbols = manifest
            .iter()
            .map(|entry| entry.name.clone())
            .collect::<BTreeSet<_>>();
        if manifest_symbols.len() != manifest.len() {
            return Err(format!("Header/symbol-manifest mismatch for '{configuration}': duplicate names in manifest.").into());
        }
        compare(
            &configuration,
            "header/symbol-manifest",
            &source_symbols,
            &manifest_symbols,
        )?;

        let api = fs::read_to_string(directory.join("api.rs"))?;
        let wrapper_symbols = rust_symbols(&api, "pub unsafe fn ");
        let expected_wrappers = manifest
            .iter()
            .filter(|entry| entry.phase != "coordinator_owned")
            .map(|entry| entry.name.clone())
            .collect::<BTreeSet<_>>();
        compare(
            &configuration,
            "manifest/raw-wrapper",
            &expected_wrappers,
            &wrapper_symbols,
        )?;

        if provenance.generated_api_symbol_count != source_symbols.len() {
            return Err(format!(
                "'{configuration}' provenance reports {} symbols, but the headers declare {}.",
                provenance.generated_api_symbol_count,
                source_symbols.len()
            )
            .into());
        }
        println!(
            "  {configuration}: {} public API symbols",
            source_symbols.len()
        );
    }
    Ok(())
}

fn collect_functions(node: &AstNode, symbols: &mut BTreeSet<String>) {
    if node.kind == "FunctionDecl" && is_api_symbol(&node.name) {
        symbols.insert(node.name.clone());
    }
    for child in &node.inner {
        collect_functions(child, symbols);
    }
}

fn is_api_symbol(name: &str) -> bool {
    name.starts_with("FPDF") || name.starts_with("FORM_") || name.starts_with("FSDK_")
}

fn extract_dispatch(source: &str) -> Option<&str> {
    let start = source.find("pub(crate) struct pdfium {")?;
    let body_start = source[start..].find('{')? + start + 1;
    let end = source[body_start..].find("\n}")? + body_start;
    Some(&source[body_start..end])
}

fn rust_symbols(source: &str, declaration: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let name = if declaration == "pub " {
                let line = line.strip_prefix("pub ")?;
                line.split_once(':')?.0
            } else {
                let line = line.strip_prefix(declaration)?;
                line.split_once('(')?.0
            };
            is_api_symbol(name).then(|| name.to_owned())
        })
        .collect()
}

fn compare(
    configuration: &str,
    label: &str,
    expected: &BTreeSet<String>,
    actual: &BTreeSet<String>,
) -> Result<(), Box<dyn Error>> {
    let missing = expected.difference(actual).cloned().collect::<Vec<_>>();
    let extra = actual.difference(expected).cloned().collect::<Vec<_>>();
    if !missing.is_empty() || !extra.is_empty() {
        return Err(format!(
            "{label} API mismatch for '{configuration}'.\n  Missing: {}\n  Extra: {}",
            missing.join(", "),
            extra.join(", ")
        )
        .into());
    }
    Ok(())
}

fn output(command: &mut Command) -> Result<String, Box<dyn Error>> {
    let output = command.output()?;
    if !output.status.success() {
        return Err(format!(
            "command {:?} failed: {}",
            command,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?.trim_end().to_owned())
}
