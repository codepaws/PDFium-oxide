#!/usr/bin/env bash
set -euo pipefail

profile=all
force_download=0
while (($#)); do
  case "$1" in
    --profile)
      [[ $# -ge 2 ]] || { echo "--profile requires default, v8, or all" >&2; exit 2; }
      profile="$2"
      shift 2
      ;;
    --force-download) force_download=1; shift ;;
    -h|--help)
      cat <<'EOF'
Usage: scripts/test-runtime-smoke.sh [--profile default|v8|all] [--force-download]

Linux x64 or ARM64. Downloads and verifies the pinned PDFium default and V8/XFA
release archives, extracts them under target/pdfium-smoke, and runs smoke tests.
Requires curl, python3, sha256sum, tar, and cargo.
EOF
      exit 0
      ;;
    *) echo "Unknown argument: $1" >&2; exit 2 ;;
  esac
done
case "$profile" in
  default|v8|all) ;;
  *) echo "Invalid profile '$profile'; expected default, v8, or all." >&2; exit 2 ;;
esac
if [[ "$(uname -s)" != Linux ]]; then
  echo 'This Bash helper supports Linux x64 and ARM64; use scripts/test-runtime-smoke.ps1 for Windows x64.' >&2
  exit 2
fi
case "$(uname -m)" in
  x86_64) platform_arch=x64 ;;
  aarch64|arm64) platform_arch=arm64 ;;
  *) echo "Unsupported Linux architecture: $(uname -m)" >&2; exit 2 ;;
esac

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixture_root="$repo_root/target/pdfium-smoke"
metadata_file="$fixture_root/release.json"
release_api='https://api.github.com/repos/bblanchon/pdfium-binaries/releases/tags/chromium%2F8100'
previous_pdfium_path="${PDFIUM_PATH-}"
had_pdfium_path=0
[[ ${PDFIUM_PATH+x} ]] && had_pdfium_path=1
restore_pdfium_path() {
  if ((had_pdfium_path)); then export PDFIUM_PATH="$previous_pdfium_path"; else unset PDFIUM_PATH || true; fi
}
trap restore_pdfium_path EXIT
mkdir -p "$fixture_root"
curl --fail --silent --show-error --location \
  --header 'User-Agent: PDFium-oxide-smoke-test' \
  "$release_api" --output "$metadata_file"
python3 - "$metadata_file" <<'PY'
import json, sys
release = json.load(open(sys.argv[1], encoding="utf-8"))
if release.get("tag_name") != "chromium/8100" or release.get("name") != "PDFium 157.0.8100.0":
    raise SystemExit(f"Unexpected PDFium release: {release.get('name')} ({release.get('tag_name')})")
PY

run_profile() {
  local kind="$1" asset="$2" expected_hash="$3" feature="$4"
  local directory="$fixture_root/linux-$platform_arch-$kind" archive="$fixture_root/$asset"
  local asset_info asset_url asset_digest actual_hash library_path version
  asset_info="$(python3 - "$metadata_file" "$asset" "$expected_hash" <<'PY'
import json, sys
matches = [a for a in json.load(open(sys.argv[1], encoding="utf-8"))["assets"] if a["name"] == sys.argv[2]]
if len(matches) != 1:
    raise SystemExit(f"Could not uniquely find release asset {sys.argv[2]}")
asset = matches[0]
if asset.get("digest") != "sha256:" + sys.argv[3]:
    raise SystemExit(f"Release digest mismatch for {sys.argv[2]}: {asset.get('digest')}")
print(asset["browser_download_url"] + "\t" + asset["digest"])
PY
  )"
  IFS=$'\t' read -r asset_url asset_digest <<<"$asset_info"
  [[ "$asset_digest" == "sha256:$expected_hash" ]]
  if ((force_download)) || [[ ! -f "$archive" ]]; then
    curl --fail --silent --show-error --location "$asset_url" --output "$archive"
  fi
  actual_hash="$(sha256sum "$archive" | awk '{print tolower($1)}')"
  if [[ "$actual_hash" != "$expected_hash" ]]; then
    rm -f -- "$archive"
    echo "Archive checksum mismatch for $asset: got $actual_hash" >&2
    return 1
  fi

  mkdir -p "$directory"
  library_path="$directory/lib/libpdfium.so"
  if ((force_download)) || [[ ! -f "$library_path" ]]; then
    tar -tzf "$archive" | python3 -c '
import sys
for entry in sys.stdin:
    path = entry.strip().replace("\\", "/")
    if path.startswith("/") or (len(path) > 1 and path[1] == ":") or any(part == ".." for part in path.split("/")):
        raise SystemExit(f"Unsafe archive entry: {entry.strip()}")
'
    tar -xzf "$archive" -C "$directory"
  fi
  [[ -f "$library_path" ]] || { echo "Missing extracted library for $kind" >&2; return 1; }
  version="$(cat "$directory/VERSION")"
  grep -qx 'MAJOR=157' <<<"$version" || { echo "Wrong PDFium major version for $kind" >&2; return 1; }
  grep -qx 'BUILD=8100' <<<"$version" || { echo "Wrong PDFium build version for $kind" >&2; return 1; }

  export PDFIUM_PATH="$library_path"
  if [[ "$feature" == v8 ]]; then
    (cd "$repo_root" && cargo test --offline --features pdfium-v8 --test runtime_smoke -- --ignored)
    (cd "$repo_root" && cargo test --offline --features pdfium-v8 --test document_workflows -- --ignored)
  else
    (cd "$repo_root" && cargo test --offline --test runtime_smoke -- --ignored)
    (cd "$repo_root" && cargo test --offline --test document_workflows -- --ignored)
  fi
}

if [[ "$platform_arch" == x64 ]]; then
  default_hash=1c1894a12d4d96ee01400c3874444126254180c5c7d2ab628576b67f8120320a
  v8_hash=78a68e46d871a52599c98af31a30a433f5441e670c03bba1f6b7a8444d645782
else
  default_hash=58c87a95a40a24c914a9c755fae798843f46f06adf9e5e9f2c893302d4bdb040
  v8_hash=5fb4c71e22589d11a2f134cc243c08480f8adc3220ec38fc539e0d8c6b2dcbef
fi
case "$profile" in
  default) run_profile default "pdfium-linux-$platform_arch.tgz" "$default_hash" default ;;
  v8) run_profile v8 "pdfium-v8-linux-$platform_arch.tgz" "$v8_hash" v8 ;;
  all)
    run_profile default "pdfium-linux-$platform_arch.tgz" "$default_hash" default
    run_profile v8 "pdfium-v8-linux-$platform_arch.tgz" "$v8_hash" v8
    ;;
esac
