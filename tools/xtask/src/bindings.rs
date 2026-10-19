use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const SOURCE_COMMIT: &str = "f51279699cb59754d1f49aef10eb8e825fe56532";
const BINDGEN_VERSION: &str = "bindgen 0.72.1";

#[derive(Clone, Deserialize, Serialize)]
struct Configuration {
    name: String,
    target: String,
    defines: Vec<String>,
    profile: String,
}

#[derive(Deserialize)]
struct Configurations {
    config: Vec<Configuration>,
}

#[derive(Clone, Deserialize)]
struct Lifecycle {
    name: String,
    phase: String,
}

#[derive(Deserialize)]
struct Lifecycles {
    lifecycle: Vec<Lifecycle>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Symbol {
    name: String,
    requirement: String,
    capability: Option<String>,
    phase: String,
}

#[derive(Serialize)]
struct Provenance {
    pdfium_version: &'static str,
    chromium_branch: &'static str,
    pdfium_source_commit: &'static str,
    source_url: &'static str,
    configuration: String,
    target: String,
    defines: Vec<String>,
    profile: String,
    bindgen: String,
    clang: String,
    public_headers: Vec<String>,
    generated_api_symbol_count: usize,
}

#[derive(Serialize)]
struct Toolchain {
    pdfium_version: &'static str,
    chromium_branch: &'static str,
    pdfium_source_commit: &'static str,
    bindgen: String,
    clang: String,
    configurations: Vec<ToolchainConfiguration>,
}

#[derive(Serialize)]
struct ToolchainConfiguration {
    name: String,
    target: String,
    defines: Vec<String>,
    symbol_count: usize,
}

pub fn check_generated(root: &Path) -> Result<(), Box<dyn Error>> {
    let generated = root.join("bindings/generated");
    let configurations = read_configurations(root)?;
    let mut manifests = BTreeMap::new();
    for configuration in &configurations {
        let directory = generated.join(&configuration.name);
        for file in ["symbols.json", "symbols.rs", "dispatch.rs", "api.rs"] {
            let path = directory.join(file);
            if !path.is_file() {
                return Err(format!(
                    "Generated runtime binding output is missing: {}",
                    path.display()
                )
                .into());
            }
        }
        let symbols: Vec<Symbol> =
            serde_json::from_slice(&fs::read(directory.join("symbols.json"))?)?;
        if symbols.len() < 300 {
            return Err(format!(
                "Expected at least 300 manifest symbols for '{}', found {}.",
                configuration.name,
                symbols.len()
            )
            .into());
        }
        let names = symbols
            .iter()
            .map(|symbol| symbol.name.as_str())
            .collect::<BTreeSet<_>>();
        if names.len() != symbols.len() {
            return Err(format!(
                "Symbol manifest for '{}' contains duplicate function names.",
                configuration.name
            )
            .into());
        }
        let dispatch = fs::read_to_string(directory.join("dispatch.rs"))?;
        if !dispatch
            .lines()
            .any(|line| line.trim() == "use super::types::*;")
        {
            return Err(format!(
                "Dispatch declarations for '{}' do not import canonical public types.",
                configuration.name
            )
            .into());
        }
        if dispatch.lines().any(|line| {
            let line = line.trim();
            line.starts_with("pub const F")
                || line.starts_with("pub type F")
                || line.starts_with("pub struct F")
                || line.starts_with("pub enum F")
                || line.starts_with("pub union F")
        }) {
            return Err(format!(
                "Dispatch declarations for '{}' duplicate public types or constants.",
                configuration.name
            )
            .into());
        }
        let api = fs::read_to_string(directory.join("api.rs"))?;
        for symbol in &symbols {
            let present = api.contains(&format!("pub unsafe fn {}(", symbol.name));
            if symbol.phase == "coordinator_owned" {
                if present {
                    return Err(format!(
                        "Coordinator-owned function '{}' has an ordinary raw wrapper in '{}'.",
                        symbol.name, configuration.name
                    )
                    .into());
                }
            } else if !present {
                return Err(format!(
                    "Function '{}' has no generated wrapper in '{}'.",
                    symbol.name, configuration.name
                )
                .into());
            }
            if symbol.requirement == "required" && symbol.capability.is_some() {
                return Err(format!(
                    "Required symbol '{}' unexpectedly names a capability in '{}'.",
                    symbol.name, configuration.name
                )
                .into());
            }
            if symbol.requirement == "optional" && symbol.capability.is_none() {
                return Err(format!(
                    "Optional symbol '{}' has no capability in '{}'.",
                    symbol.name, configuration.name
                )
                .into());
            }
            if !["required", "optional"].contains(&symbol.requirement.as_str()) {
                return Err(format!(
                    "Symbol '{}' has unknown requirement in '{}'.",
                    symbol.name, configuration.name
                )
                .into());
            }
            if ![
                "initialized",
                "pre_init",
                "pre_init_or_destroyed",
                "coordinator_owned",
            ]
            .contains(&symbol.phase.as_str())
            {
                return Err(format!(
                    "Symbol '{}' has unknown lifecycle phase in '{}'.",
                    symbol.name, configuration.name
                )
                .into());
            }
            if symbol.phase == "pre_init_or_destroyed"
                && (!has_method_in_impl(&api, "PreInitApi", &symbol.name)
                    || !has_method_in_impl(&api, "DestroyedApi", &symbol.name))
            {
                return Err(format!(
                    "Function '{}' must be available through both pre-init and destroyed facades in '{}'.",
                    symbol.name, configuration.name
                )
                .into());
            }
        }
        manifests.insert(configuration.name.clone(), symbols);
    }
    let base = manifests
        .get("windows-x64-default")
        .ok_or("Windows default manifest missing")?
        .iter()
        .map(|symbol| symbol.name.as_str())
        .collect::<BTreeSet<_>>();
    let v8 = manifests
        .get("windows-x64-v8-xfa")
        .ok_or("Windows V8/XFA manifest missing")?;
    let extras = v8
        .iter()
        .filter(|symbol| !base.contains(symbol.name.as_str()))
        .collect::<Vec<_>>();
    if extras.is_empty()
        || extras.iter().any(|symbol| {
            symbol.requirement != "optional" || symbol.capability.as_deref() != Some("V8Xfa")
        })
    {
        return Err(
            "Windows V8/XFA symbols are not classified as optional V8Xfa capability entries."
                .into(),
        );
    }
    println!("Runtime binding output checks passed for all six profiles.");
    Ok(())
}

fn has_method_in_impl(api: &str, facade: &str, method: &str) -> bool {
    let marker = format!("impl crate::raw::{facade}<'_> {{");
    let Some(start) = api.find(&marker) else {
        return false;
    };
    let rest = &api[start + marker.len()..];
    let end = rest.find("impl crate::raw::").unwrap_or(rest.len());
    rest[..end].contains(&format!("pub unsafe fn {method}("))
}

pub fn regenerate(root: &Path, args: &[String]) -> Result<(), Box<dyn Error>> {
    let source = option(args, "--pdfium-source")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target/pdfium-source-pinned"));
    validate_source(&source)?;
    let bindgen = executable_option(args, "--bindgen", "bindgen")?;
    let clang = executable_option(args, "--clang", "clang")?;
    let bindgen_version_output = command_text(&bindgen, &["--version"])?;
    validate_bindgen_version(&bindgen_version_output)?;
    let bindgen_version = bindgen_version_output.trim().to_owned();
    let clang_output = command_text(&clang, &["--version"])?;
    let clang_version = clang_output.lines().next().unwrap_or_default().to_owned();
    validate_clang_version(&clang_version)?;
    let resource = command_text(&clang, &["-print-resource-dir"])?;
    let resource_include = Path::new(resource.trim()).join("include");
    if !resource_include.join("stddef.h").is_file() {
        return Err(format!(
            "Clang resource headers were not found under {}",
            resource_include.display()
        )
        .into());
    }
    let configurations = read_configurations(root)?;
    if configurations.len() != 6 {
        return Err(format!(
            "Expected six supported configurations, found {}.",
            configurations.len()
        )
        .into());
    }
    let public = source.join("public");
    let mut headers = fs::read_dir(&public)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension().is_some_and(|ext| ext == "h")
                && path
                    .file_name()
                    .is_some_and(|name| name.to_string_lossy().starts_with("fpdf"))
        })
        .collect::<Vec<_>>();
    headers.sort();
    if headers.len() < 20 || !public.join("fpdfview.h").is_file() {
        return Err("PDFium checkout is missing expected official public C headers.".into());
    }
    let header_names = headers
        .iter()
        .filter_map(|header| header.file_name())
        .map(|name| format!("public/{}", name.to_string_lossy()))
        .collect::<Vec<_>>();
    let temporary = root
        .join("target")
        .join(format!("pdfium-bindgen-{}", std::process::id()));
    if temporary.exists() {
        fs::remove_dir_all(&temporary)?;
    }
    fs::create_dir_all(&temporary)?;
    let result = regenerate_into(
        root,
        &public,
        &headers,
        &header_names,
        &configurations,
        &temporary,
        &bindgen,
        &clang,
        &resource_include,
        &bindgen_version,
        &clang_version,
    );
    if result.is_ok() {
        let output = root.join("bindings/generated");
        for configuration in &configurations {
            let from = temporary.join(&configuration.name);
            let to = output.join(&configuration.name);
            fs::create_dir_all(&to)?;
            for file in [
                "types.rs",
                "dispatch.rs",
                "provenance.json",
                "symbols.json",
                "symbols.rs",
                "api.rs",
            ] {
                fs::copy(from.join(file), to.join(file))?;
            }
        }
        fs::copy(
            temporary.join("toolchain.json"),
            output.join("toolchain.json"),
        )?;
    }
    let _ = fs::remove_dir_all(&temporary);
    result
}

fn regenerate_into(
    root: &Path,
    public: &Path,
    headers: &[PathBuf],
    header_names: &[String],
    configurations: &[Configuration],
    temporary: &Path,
    bindgen: &Path,
    _clang: &Path,
    resource_include: &Path,
    bindgen_version: &str,
    clang_version: &str,
) -> Result<(), Box<dyn Error>> {
    let wrapper = temporary.join("wrapper.h");
    let mut wrapper_file = fs::File::create(&wrapper)?;
    for header in headers {
        use std::io::Write;
        writeln!(
            wrapper_file,
            "#include <{}>",
            header.file_name().unwrap().to_string_lossy()
        )?;
    }
    let mut generated_configurations = Vec::new();
    for configuration in configurations {
        let config_output = temporary.join(&configuration.name);
        fs::create_dir_all(&config_output)?;
        let generated = config_output.join("bindgen.rs");
        let stderr_path = config_output.join("clang-bindgen.stderr.txt");
        let mut command = Command::new(bindgen);
        command.arg(&wrapper).args([
            "--dynamic-loading",
            "pdfium",
            "--allowlist-function",
            "^(FPDF.*|FORM_.*|FSDK_.*)$",
            "--allowlist-type",
            "^(FPDF.*|FORM.*|FS_.*|FX.*|FSDK.*|UNSUPPORT.*|IPDF.*)$",
            "--allowlist-var",
            "^(FPDF.*|FORMTYPE.*|FXFA_.*)$",
            "--no-layout-tests",
            "--no-derive-debug",
            "--disable-header-comment",
            "--use-core",
            "--",
        ]);
        command.arg(format!("--target={}", configuration.target));
        if configuration.target.contains("linux") {
            command
                .args(["-nostdinc", "-isystem"])
                .arg(resource_include)
                .arg("-I")
                .arg(root.join("bindings/compat/linux"));
        } else if configuration.target.contains("windows") {
            if let Some(include) = std::env::var_os("WINDOWS_INCLUDE") {
                command.arg("-I").arg(include);
            } else if cfg!(target_os = "linux") {
                return Err("WINDOWS_INCLUDE must point to Windows SDK or MinGW headers when regenerating Windows bindings on Linux.".into());
            }
        }
        command.arg("-I").arg(public);
        for define in &configuration.defines {
            command.arg(format!("-D{define}"));
        }
        let output = command.output()?;
        fs::write(&generated, &output.stdout)?;
        fs::write(&stderr_path, &output.stderr)?;
        if !output.status.success() {
            return Err(format!(
                "Binding generation failed for '{}' target '{}': {}",
                configuration.name,
                configuration.target,
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }
        let generated_text = String::from_utf8(output.stdout)?;
        let marker = generated_text.find("pub struct pdfium {").ok_or_else(|| {
            format!(
                "bindgen did not emit the typed PDFium dispatch table for '{}'.",
                configuration.name
            )
        })?;
        let types = &generated_text[..marker];
        if !types.contains("FPDF_DOCUMENT") || !types.contains("FPDF_PAGE") {
            return Err(format!(
                "Generated type declarations for '{}' are missing core PDFium handles.",
                configuration.name
            )
            .into());
        }
        let dispatch_body = generated_text[marker..].replacen(
            "pub struct pdfium {",
            "pub(crate) struct pdfium {",
            1,
        );
        let dispatch = format!("use super::types::*;\n{dispatch_body}");
        let symbols = dispatch_symbols(&dispatch).map_err(|error| {
            format!(
                "Could not parse dispatch table for '{}': {error}",
                configuration.name
            )
        })?;
        if symbols.len() < 300 {
            return Err(format!(
                "Only {} public API symbols were generated for '{}'.",
                symbols.len(),
                configuration.name
            )
            .into());
        }
        let prefix = format!("// Generated from the official PDFium public C headers. Do not edit by hand.\n// Source: {SOURCE_COMMIT}; configuration: {}.\n", configuration.name);
        fs::write(config_output.join("types.rs"), format!("{prefix}{types}\n"))?;
        fs::write(
            config_output.join("dispatch.rs"),
            format!("{prefix}{dispatch}\n"),
        )?;
        let metadata = Provenance {
            pdfium_version: "157.0.8100.0",
            chromium_branch: "chromium/8100",
            pdfium_source_commit: SOURCE_COMMIT,
            source_url: "https://pdfium.googlesource.com/pdfium/",
            configuration: configuration.name.clone(),
            target: configuration.target.clone(),
            defines: configuration.defines.clone(),
            profile: configuration.profile.clone(),
            bindgen: bindgen_version.to_owned(),
            clang: clang_version.to_owned(),
            public_headers: header_names.to_vec(),
            generated_api_symbol_count: symbols.len(),
        };
        fs::write(
            config_output.join("provenance.json"),
            format!("{}\n", serde_json::to_string_pretty(&metadata)?),
        )?;
        generated_configurations.push(ToolchainConfiguration {
            name: configuration.name.clone(),
            target: configuration.target.clone(),
            defines: configuration.defines.clone(),
            symbol_count: symbols.len(),
        });
        let _ = fs::remove_file(generated);
        let _ = fs::remove_file(stderr_path);
    }
    generate_manifests(temporary, root)?;
    let toolchain = Toolchain {
        pdfium_version: "157.0.8100.0",
        chromium_branch: "chromium/8100",
        pdfium_source_commit: SOURCE_COMMIT,
        bindgen: bindgen_version.to_owned(),
        clang: clang_version.to_owned(),
        configurations: generated_configurations,
    };
    fs::write(
        temporary.join("toolchain.json"),
        format!("{}\n", serde_json::to_string_pretty(&toolchain)?),
    )?;
    println!(
        "Generated PDFium bindings for {} configurations.",
        configurations.len()
    );
    Ok(())
}

fn generate_manifests(root: &Path, repo: &Path) -> Result<(), Box<dyn Error>> {
    let lifecycle: Lifecycles = toml::from_str(&fs::read_to_string(
        repo.join("bindings/lifecycle-overrides.toml"),
    )?)?;
    let overrides = lifecycle
        .lifecycle
        .into_iter()
        .map(|item| (item.name, item.phase))
        .collect::<BTreeMap<_, _>>();
    let targets = ["windows-x64", "linux-x64", "linux-arm64"];
    let profiles = ["default", "v8-xfa"];
    let mut symbols_by_config = BTreeMap::<String, Vec<String>>::new();
    let mut all_names = BTreeSet::new();
    for target in targets {
        for profile in profiles {
            let config = format!("{target}-{profile}");
            let path = root.join(&config).join("dispatch.rs");
            let text = fs::read_to_string(&path)?;
            let symbols = dispatch_symbols(&text)?;
            if symbols.len() < 300 {
                return Err(format!("Invalid symbol table in {}", path.display()).into());
            }
            all_names.extend(symbols.iter().cloned());
            symbols_by_config.insert(config, symbols);
        }
    }
    for name in overrides.keys() {
        if !all_names.contains(name) {
            return Err(format!("Lifecycle override names unknown PDFium symbol '{name}'.").into());
        }
    }
    for target in targets {
        let base_name = format!("{target}-default");
        let base = symbols_by_config[&base_name]
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        for profile in profiles {
            let config = format!("{target}-{profile}");
            let entries = symbols_by_config[&config]
                .iter()
                .map(|name| {
                    let optional = profile == "v8-xfa" && !base.contains(name);
                    Symbol {
                        name: name.clone(),
                        requirement: if optional { "optional" } else { "required" }.into(),
                        capability: optional.then(|| "V8Xfa".into()),
                        phase: overrides
                            .get(name)
                            .cloned()
                            .unwrap_or_else(|| "initialized".into()),
                    }
                })
                .collect::<Vec<_>>();
            let directory = root.join(&config);
            fs::write(
                directory.join("symbols.json"),
                format!("{}\n", serde_json::to_string_pretty(&entries)?),
            )?;
            let required = entries
                .iter()
                .filter(|entry| entry.requirement == "required")
                .map(|entry| format!("    \"{}\",\n", entry.name))
                .collect::<String>();
            let optional = entries
                .iter()
                .filter(|entry| entry.requirement == "optional")
                .map(|entry| format!("    (\"{}\", crate::Capability::V8Xfa),\n", entry.name))
                .collect::<String>();
            let symbols_rs = format!("pub(crate) const REQUIRED_SYMBOLS: &[&str] = &[\n{required}];\npub(crate) const OPTIONAL_SYMBOLS: &[(&str, crate::Capability)] = &[\n{optional}];\n");
            fs::write(directory.join("symbols.rs"), format!("{symbols_rs}\n"))?;
            let dispatch = fs::read_to_string(directory.join("dispatch.rs"))?;
            fs::write(directory.join("api.rs"), generate_api(&dispatch, &entries)?)?;
        }
    }
    Ok(())
}

fn generate_api(dispatch: &str, entries: &[Symbol]) -> Result<String, Box<dyn Error>> {
    let methods = parse_methods(dispatch)?;
    for entry in entries
        .iter()
        .filter(|entry| entry.phase != "coordinator_owned")
    {
        if !methods.contains_key(&entry.name) {
            return Err(format!("missing dispatch method {}", entry.name).into());
        }
    }
    let initialized = entries
        .iter()
        .filter(|entry| entry.phase == "initialized")
        .collect::<Vec<_>>();
    let pre_init = entries
        .iter()
        .filter(|entry| entry.phase == "pre_init" || entry.phase == "pre_init_or_destroyed")
        .collect::<Vec<_>>();
    let pre_destroyed = entries
        .iter()
        .filter(|entry| entry.phase == "pre_init_or_destroyed")
        .collect::<Vec<_>>();
    let mut output = String::from("// Generated lifecycle-gated wrappers; do not edit by hand.\n");
    output.push_str(&emit_impl("RawApi", &initialized, &methods, "initialized"));
    output.push_str(&emit_impl("PreInitApi", &pre_init, &methods, "pre_init"));
    output.push_str(&emit_impl(
        "DestroyedApi",
        &pre_destroyed,
        &methods,
        "pre_init_or_destroyed",
    ));
    Ok(output)
}

type Method = (Vec<String>, String, Vec<String>);

fn parse_methods(source: &str) -> Result<BTreeMap<String, Method>, Box<dyn Error>> {
    let mut methods = BTreeMap::new();
    let mut cursor = 0;
    while let Some(found) = source[cursor..].find("pub unsafe fn ") {
        let start = cursor + found + "pub unsafe fn ".len();
        let name_end = source[start..]
            .find('(')
            .ok_or("unclosed function declaration")?
            + start;
        let name = source[start..name_end].trim().to_owned();
        if !(name.starts_with("FPDF") || name.starts_with("FORM_") || name.starts_with("FSDK_")) {
            cursor = name_end + 1;
            continue;
        }
        let close = matching(source, name_end, '(', ')')?;
        let params = split_parameters(&source[name_end + 1..close]);
        let body = source[close + 1..]
            .find('{')
            .ok_or_else(|| format!("method body missing for {name}"))?
            + close
            + 1;
        let mut ret = source[close + 1..body].trim().to_owned();
        if ret.starts_with("->") {
            ret = ret[2..].trim().to_owned();
        } else {
            ret = "()".into();
        }
        let mut args = Vec::new();
        for param in &params {
            let compact = param
                .chars()
                .filter(|character| !character.is_whitespace())
                .collect::<String>();
            if compact == "&self" || compact == "&mutself" {
                continue;
            }
            let argument = param
                .split_once(':')
                .map(|pair| pair.0.trim())
                .unwrap_or("")
                .trim_start_matches("mut ");
            if argument.is_empty() {
                return Err(format!("unsupported argument pattern in {name}: {param}").into());
            }
            args.push(argument.to_owned());
        }
        methods.insert(name, (params, ret, args));
        cursor = body + 1;
    }
    Ok(methods)
}

fn matching(text: &str, opening: usize, left: char, right: char) -> Result<usize, Box<dyn Error>> {
    let mut depth = 0;
    for (offset, ch) in text[opening..].char_indices() {
        if ch == left {
            depth += 1;
        }
        if ch == right {
            depth -= 1;
            if depth == 0 {
                return Ok(opening + offset);
            }
        }
    }
    Err("unbalanced function signature".into())
}

fn split_parameters(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    let (mut paren, mut bracket, mut brace, mut angle, mut start) =
        (0_i32, 0_i32, 0_i32, 0_i32, 0_usize);
    for (index, ch) in text.char_indices() {
        match ch {
            '(' => paren += 1,
            ')' => paren -= 1,
            '[' => bracket += 1,
            ']' => bracket -= 1,
            '{' => brace += 1,
            '}' => brace -= 1,
            '<' => angle += 1,
            '>' => angle -= 1,
            ',' if paren == 0 && bracket == 0 && brace == 0 && angle == 0 => {
                let value = text[start..index].trim();
                if !value.is_empty() {
                    result.push(value.to_owned());
                }
                start = index + 1;
            }
            _ => {}
        }
    }
    let last = text[start..].trim();
    if !last.is_empty() {
        result.push(last.to_owned());
    }
    result
}

fn emit_impl(
    class: &str,
    entries: &[&Symbol],
    methods: &BTreeMap<String, Method>,
    phase: &str,
) -> String {
    let mut output = format!("#[allow(non_snake_case)]\nimpl crate::raw::{class}<'_> {{");
    for entry in entries {
        let (params, ret, args) = &methods[&entry.name];
        let params = params
            .iter()
            .filter(|param| param.as_str() != "&self" && param.as_str() != "&mut self")
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        let callargs = args.join(", ");
        let expected = match phase {
            "initialized" => "Initialized",
            "pre_init" => "Prepared",
            _ => "Destroyed",
        };
        output.push_str(&format!("\n    /// Calls `{}` through PDFium's serialized process coordinator.\n    ///\n    /// # Safety\n    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.\n    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `{}`.\n    pub unsafe fn {}(&self{}{}) -> Result<crate::PdfiumCall<{}>, crate::PdfiumError> {{\n        crate::runtime::call(\"{}\", crate::runtime::Phase::{}, |context| unsafe {{ context.dispatch.{}({}) }})\n    }}", entry.name, entry.name, entry.name, if params.is_empty() { "" } else { ", " }, params, ret, entry.name, expected, entry.name, callargs));
    }
    output.push_str("\n}\n");
    output
}

fn dispatch_symbols(source: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = syn::parse_file(source)?;
    let table = file
        .items
        .into_iter()
        .find_map(|item| match item {
            syn::Item::Struct(item) if item.ident == "pdfium" => Some(item),
            _ => None,
        })
        .ok_or("dispatch table missing")?;
    let mut symbols = Vec::new();
    for field in table.fields {
        let Some(name) = field.ident else {
            continue;
        };
        let name = name.to_string();
        if name.starts_with("FPDF") || name.starts_with("FORM_") || name.starts_with("FSDK_") {
            symbols.push(name);
        }
    }
    if symbols.iter().collect::<BTreeSet<_>>().len() != symbols.len() {
        return Err("dispatch table contains duplicate symbols".into());
    }
    Ok(symbols)
}

#[cfg(test)]
mod tests {
    use super::{dispatch_symbols, parse_methods};

    #[test]
    fn dispatch_table_parser_accepts_indented_closing_brace() {
        let source = "pub(crate) struct pdfium {\n    pub FPDF_InitLibrary: fn(),\n    }\n";

        assert_eq!(dispatch_symbols(source).unwrap(), ["FPDF_InitLibrary"]);
    }

    #[test]
    fn dispatch_table_parser_accepts_closing_brace_with_trailing_comment() {
        let source = "pub(crate) struct pdfium {\n    pub FPDF_InitLibrary: fn(),\n} // end of bindgen dispatch table\n";

        assert_eq!(dispatch_symbols(source).unwrap(), ["FPDF_InitLibrary"]);
    }

    #[test]
    fn dispatch_table_parser_accepts_a_same_line_closing_brace() {
        let source = "pub(crate) struct pdfium { pub FPDF_InitLibrary: fn(), }\n";

        assert_eq!(dispatch_symbols(source).unwrap(), ["FPDF_InitLibrary"]);
    }

    #[test]
    fn dispatch_table_parser_ignores_braces_inside_field_documentation() {
        let source = "pub(crate) struct pdfium {\n    /// A documentation brace: }\n    pub FPDF_InitLibrary: fn(),\n}\n";

        assert_eq!(dispatch_symbols(source).unwrap(), ["FPDF_InitLibrary"]);
    }

    #[test]
    fn method_parser_accepts_bindgen_formatted_shared_receiver() {
        let source = "impl pdfium { pub unsafe fn FPDF_InitLibraryWithConfig(& self, config: FPDF_LIBRARY_CONFIG) {} }";

        let methods = parse_methods(source).unwrap();
        assert_eq!(methods["FPDF_InitLibraryWithConfig"].2, ["config"]);
    }
}

pub fn fetch_source(root: &Path, args: &[String]) -> Result<(), Box<dyn Error>> {
    let source = option(args, "--pdfium-source")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target/pdfium-source-pinned"));
    if !source.exists() {
        fs::create_dir_all(&source)?;
    }
    if !source.join(".git").exists() {
        run_git(&source, &["init"])?;
        run_git(
            &source,
            &[
                "remote",
                "add",
                "origin",
                "https://pdfium.googlesource.com/pdfium/",
            ],
        )?;
        run_git(&source, &["sparse-checkout", "init", "--cone"])?;
        run_git(&source, &["sparse-checkout", "set", "public"])?;
    }
    run_git(
        &source,
        &[
            "fetch",
            "--depth=1",
            "--filter=blob:none",
            "origin",
            SOURCE_COMMIT,
        ],
    )?;
    run_git(&source, &["checkout", "--detach", "FETCH_HEAD"])?;
    validate_source(&source)?;
    println!(
        "Fetched pinned PDFium public headers to {}",
        source.display()
    );
    Ok(())
}

pub fn check_generator(root: &Path, args: &[String]) -> Result<(), Box<dyn Error>> {
    let source = option(args, "--pdfium-source")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target/pdfium-source-pinned"));
    let scratch = root
        .join("target")
        .join(format!("xtask-generator-check-{}", std::process::id()));
    if scratch.exists() {
        fs::remove_dir_all(&scratch)?;
    }
    fs::create_dir_all(&scratch)?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        let sentinel = root.join("bindings/generated/toolchain.json");
        let before = fs::read(&sentinel)?;
        ensure_rejected(validate_source(&scratch), "non-Git PDFium source")?;
        ensure_rejected(
            validate_bindgen_version("bindgen 0.99.0"),
            "incorrect bindgen version",
        )?;
        ensure_rejected(
            validate_clang_version("gcc (no clang version)"),
            "unrecognized Clang version",
        )?;

        let wrong_source = scratch.join("wrong-source");
        fs::create_dir_all(&wrong_source)?;
        command_text(
            Path::new("git"),
            &["-C", &wrong_source.to_string_lossy(), "init", "--quiet"],
        )?;
        command_text(
            Path::new("git"),
            &[
                "-C",
                &wrong_source.to_string_lossy(),
                "config",
                "user.email",
                "pdfium-test@example.invalid",
            ],
        )?;
        command_text(
            Path::new("git"),
            &[
                "-C",
                &wrong_source.to_string_lossy(),
                "config",
                "user.name",
                "PDFium xtask test",
            ],
        )?;
        fs::write(wrong_source.join("revision.txt"), "not PDFium\n")?;
        command_text(
            Path::new("git"),
            &["-C", &wrong_source.to_string_lossy(), "add", "revision.txt"],
        )?;
        command_text(
            Path::new("git"),
            &[
                "-C",
                &wrong_source.to_string_lossy(),
                "commit",
                "--quiet",
                "-m",
                "wrong source",
            ],
        )?;
        ensure_rejected(
            validate_source(&wrong_source),
            "incorrect PDFium source revision",
        )?;

        if validate_source(&scratch).is_ok() {
            return Err("generator accepted a non-Git source directory".into());
        }
        if source.is_dir() {
            let commit = command_text(
                Path::new("git"),
                &["-C", &source.to_string_lossy(), "rev-parse", "HEAD"],
            )?;
            if commit.trim() == SOURCE_COMMIT {
                let dirty = scratch.join("dirty-source");
                let add = Command::new("git")
                    .args(["-C"])
                    .arg(&source)
                    .args(["worktree", "add", "--quiet", "--detach"])
                    .arg(&dirty)
                    .arg("HEAD")
                    .status()?;
                if !add.success() {
                    return Err("could not prepare dirty source fixture".into());
                }
                let dirty_result = (|| -> Result<(), Box<dyn Error>> {
                    let public = dirty.join("public");
                    let header = fs::read_dir(&public)?
                        .filter_map(Result::ok)
                        .map(|entry| entry.path())
                        .find(|path| {
                            path.file_name().is_some_and(|name| {
                                name.to_string_lossy().starts_with("fpdf")
                                    && name.to_string_lossy().ends_with(".h")
                            })
                        })
                        .ok_or("no PDFium header found")?;
                    fs::OpenOptions::new()
                        .append(true)
                        .open(header)?
                        .write_all(b"\n/* modified by xtask validation check */\n")?;
                    ensure_rejected(validate_source(&dirty), "modified public PDFium headers")
                })();
                let _ = Command::new("git")
                    .args(["-C"])
                    .arg(&source)
                    .args(["worktree", "remove", "--force"])
                    .arg(&dirty)
                    .status();
                dirty_result?;
            }
        }
        let missing = scratch.join("missing-tool");
        let missing_bindgen = ["--bindgen".into(), missing.to_string_lossy().into_owned()];
        ensure_rejected(
            executable_option(&missing_bindgen, "--bindgen", "bindgen"),
            "missing bindgen executable",
        )?;
        let missing_clang = ["--clang".into(), missing.to_string_lossy().into_owned()];
        ensure_rejected(
            executable_option(&missing_clang, "--clang", "clang"),
            "missing Clang executable",
        )?;
        if fs::read(sentinel)? != before {
            return Err(
                "generator validation changed generated binding output before preflight passed"
                    .into(),
            );
        }
        println!("Binding generator rejection-path checks passed.");
        Ok(())
    })();
    let _ = fs::remove_dir_all(&scratch);
    result
}

fn ensure_rejected<T, E>(result: Result<T, E>, scenario: &str) -> Result<(), Box<dyn Error>> {
    if result.is_ok() {
        Err(format!("generator validation accepted {scenario}").into())
    } else {
        Ok(())
    }
}

fn validate_bindgen_version(version: &str) -> Result<(), Box<dyn Error>> {
    if version.trim() == BINDGEN_VERSION {
        Ok(())
    } else {
        Err(format!("Expected {BINDGEN_VERSION}, found '{}'.", version.trim()).into())
    }
}

fn validate_clang_version(version: &str) -> Result<(), Box<dyn Error>> {
    if version
        .split_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|words| words == ["clang", "version"])
    {
        Ok(())
    } else {
        Err(format!("Unrecognized Clang version output: {version}").into())
    }
}

fn validate_source(source: &Path) -> Result<(), Box<dyn Error>> {
    if !source.is_dir() {
        return Err(format!(
            "PDFium source checkout '{}' does not exist or is not a directory.",
            source.display()
        )
        .into());
    }
    let commit = command_text(
        Path::new("git"),
        &["-C", &source.to_string_lossy(), "rev-parse", "HEAD"],
    )?;
    if commit.trim() != SOURCE_COMMIT {
        return Err(format!(
            "PDFium source revision mismatch: expected {SOURCE_COMMIT}, got {}.",
            commit.trim()
        )
        .into());
    }
    let status = Command::new("git")
        .args(["-C"])
        .arg(source)
        .args([
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
            "--",
            "public/fpdf*.h",
        ])
        .output()?;
    if !status.status.success() {
        return Err("Unable to verify public PDFium header state.".into());
    }
    if !status.stdout.is_empty() {
        return Err("Public PDFium headers are modified, staged, deleted, or untracked; use a clean pinned checkout.".into());
    }
    let count = fs::read_dir(source.join("public"))?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_name().to_string_lossy().starts_with("fpdf")
                && entry.path().extension().is_some_and(|ext| ext == "h")
        })
        .count();
    if count < 20 {
        return Err("PDFium checkout is missing expected official public C headers.".into());
    }
    Ok(())
}

fn read_configurations(root: &Path) -> Result<Vec<Configuration>, Box<dyn Error>> {
    let parsed: Configurations = toml::from_str(&fs::read_to_string(
        root.join("bindings/configurations.toml"),
    )?)?;
    Ok(parsed.config)
}

fn option<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|arg| arg == name)
        .and_then(|index| args.get(index + 1))
        .map(String::as_str)
}

fn executable_option(
    args: &[String],
    name: &str,
    fallback: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let path = option(args, name).unwrap_or(fallback);
    let resolved = if path.contains(std::path::MAIN_SEPARATOR) || Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        which(path).ok_or_else(|| format!("Required executable '{path}' was not found on PATH."))?
    };
    if !resolved.is_file() {
        return Err(format!("Executable '{}' does not exist.", resolved.display()).into());
    }
    Ok(resolved)
}

fn which(name: &str) -> Option<PathBuf> {
    let paths = std::env::split_paths(&std::env::var_os("PATH")?).collect::<Vec<_>>();
    let mut names = vec![name.to_owned()];
    if cfg!(windows) && Path::new(name).extension().is_none() {
        let extensions = std::env::var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".into());
        names.extend(
            extensions
                .split(';')
                .map(|extension| format!("{name}{extension}")),
        );
    }
    paths
        .iter()
        .flat_map(|directory| names.iter().map(|candidate| directory.join(candidate)))
        .find(|path| path.is_file())
}

fn command_text(executable: &Path, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new(executable).args(args).output()?;
    if !output.status.success() {
        return Err(format!(
            "command {} failed: {}",
            executable.display(),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn run_git(directory: &Path, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(args)
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(())
}
