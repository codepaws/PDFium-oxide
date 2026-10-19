use flate2::read::GzDecoder;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Component, Path};
use std::process::Command;

const RELEASE_TAG: &str = "chromium/8100";
const RELEASE_NAME: &str = "PDFium 157.0.8100.0";
const API_URL: &str =
    "https://api.github.com/repos/bblanchon/pdfium-binaries/releases/tags/chromium%2F8100";

#[derive(Deserialize)]
struct Release {
    name: String,
    tag_name: String,
    assets: Vec<Asset>,
}
#[derive(Deserialize)]
struct Asset {
    name: String,
    digest: Option<String>,
    browser_download_url: String,
}

struct Profile {
    name: &'static str,
    asset: String,
    sha256: &'static str,
    feature: bool,
}

pub fn run(root: &Path, args: &[String]) -> Result<(), Box<dyn Error>> {
    let mut profile = "all";
    let mut force = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--profile" => {
                index += 1;
                profile = args
                    .get(index)
                    .ok_or("--profile requires default, v8, or all")?;
            }
            "--force-download" => force = true,
            "--help" | "-h" => {
                println!("Usage: cargo xtask test-runtime-smoke [--profile default|v8|all] [--force-download]");
                return Ok(());
            }
            other => return Err(format!("unknown test-runtime-smoke argument '{other}'").into()),
        }
        index += 1;
    }
    if !["default", "v8", "all"].contains(&profile) {
        return Err(format!("invalid profile '{profile}'; expected default, v8, or all").into());
    }
    let (os, arch) = host()?;
    let profiles = profiles(os, arch);
    let profiles = profiles
        .into_iter()
        .filter(|item| profile == "all" || item.name == profile)
        .collect::<Vec<_>>();
    let client = reqwest::blocking::Client::builder()
        .user_agent("PDFium-oxide-smoke-test")
        .build()?;
    let release: Release = client.get(API_URL).send()?.error_for_status()?.json()?;
    if release.name != RELEASE_NAME || release.tag_name != RELEASE_TAG {
        return Err(format!(
            "unexpected PDFium release metadata: {} ({})",
            release.name, release.tag_name
        )
        .into());
    }
    let fixture_root = root.join("target/pdfium-smoke");
    fs::create_dir_all(&fixture_root)?;
    for item in profiles {
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == item.asset)
            .ok_or_else(|| format!("release asset '{}' was not found", item.asset))?;
        let expected_digest = format!("sha256:{}", item.sha256);
        if asset.digest.as_deref() != Some(&expected_digest) {
            return Err(format!(
                "release digest mismatch for '{}': expected {}, got {:?}",
                item.asset, expected_digest, asset.digest
            )
            .into());
        }
        let archive = fixture_root.join(&item.asset);
        if force || !archive.is_file() {
            let mut response = client
                .get(&asset.browser_download_url)
                .send()?
                .error_for_status()?;
            let mut bytes = Vec::new();
            response.read_to_end(&mut bytes)?;
            fs::write(&archive, bytes)?;
        }
        let bytes = fs::read(&archive)?;
        let actual = format!("{:x}", Sha256::digest(&bytes));
        if actual != item.sha256 {
            let _ = fs::remove_file(&archive);
            return Err(format!(
                "archive checksum mismatch for '{}': expected {}, got {}",
                item.asset, item.sha256, actual
            )
            .into());
        }
        let directory = fixture_root.join(format!("{os}-{arch}-{}", item.name));
        let expected_file = if os == "windows" {
            directory.join("bin/pdfium.dll")
        } else {
            directory.join("lib/libpdfium.so")
        };
        if force || !expected_file.is_file() {
            extract_archive(&archive, &directory)?;
        }
        if !expected_file.is_file() {
            return Err(format!(
                "extracted PDFium library is missing: {}",
                expected_file.display()
            )
            .into());
        }
        let version = fs::read_to_string(directory.join("VERSION"))?;
        if !version.lines().any(|line| line == "MAJOR=157")
            || !version.lines().any(|line| line == "BUILD=8100")
        {
            return Err(format!(
                "extracted PDFium version does not match {RELEASE_NAME} for profile '{}'.",
                item.name
            )
            .into());
        }
        run_cargo_tests(root, &expected_file, item.feature)?;
    }
    Ok(())
}

fn host() -> Result<(&'static str, &'static str), Box<dyn Error>> {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        return Err("native smoke supports Windows x64 and Linux x64/ARM64".into());
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        other => return Err(format!("unsupported host architecture: {other}").into()),
    };
    if os == "windows" && arch != "x64" {
        return Err("Windows native smoke currently supports x64 only".into());
    }
    Ok((os, arch))
}

fn profiles(os: &str, arch: &str) -> Vec<Profile> {
    let platform = match (os, arch) {
        ("windows", "x64") => "win-x64",
        ("linux", "x64") => "linux-x64",
        _ => "linux-arm64",
    };
    let (default_hash, v8_hash) = match platform {
        "win-x64" => (
            "51621bcc1d9be9ea448f4b84b0516862b928e3968d3fca899af1a6d6e2848d6d",
            "10a926574aa43ef673ba1a89bc64c559307ab541ca6169299d77797a8472dd7b",
        ),
        "linux-x64" => (
            "1c1894a12d4d96ee01400c3874444126254180c5c7d2ab628576b67f8120320a",
            "78a68e46d871a52599c98af31a30a433f5441e670c03bba1f6b7a8444d645782",
        ),
        _ => (
            "58c87a95a40a24c914a9c755fae798843f46f06adf9e5e9f2c893302d4bdb040",
            "5fb4c71e22589d11a2f134cc243c08480f8adc3220ec38fc539e0d8c6b2dcbef",
        ),
    };
    [
        Profile {
            name: "default",
            asset: format!("pdfium-{platform}.tgz"),
            sha256: default_hash,
            feature: false,
        },
        Profile {
            name: "v8",
            asset: format!("pdfium-v8-{platform}.tgz"),
            sha256: v8_hash,
            feature: true,
        },
    ]
    .into_iter()
    .collect()
}

fn extract_archive(archive_path: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    let file = fs::File::open(archive_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        if path.as_os_str().is_empty()
            || path
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(format!("unsafe archive entry '{}'", path.display()).into());
        }
        entry.unpack_in(destination)?;
    }
    Ok(())
}

fn run_cargo_tests(root: &Path, library: &Path, v8: bool) -> Result<(), Box<dyn Error>> {
    for test in ["runtime_smoke", "document_workflows"] {
        let mut command = Command::new("cargo");
        command.current_dir(root).args(["test", "--offline"]);
        if v8 {
            command.args(["--features", "pdfium-v8"]);
        }
        command.args(["--test", test, "--", "--ignored"]);
        command.env("PDFIUM_PATH", library);
        let status = command.status()?;
        if !status.success() {
            return Err(format!(
                "native {test} test failed (profile: {}).",
                if v8 { "v8" } else { "default" }
            )
            .into());
        }
    }
    Ok(())
}
