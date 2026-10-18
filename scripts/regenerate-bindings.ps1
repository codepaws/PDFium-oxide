[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidateNotNullOrEmpty()]
    [string] $PdfiumSource,

    [string] $BindgenPath,
    [string] $ClangPath
)

$ErrorActionPreference = 'Stop'
$expectedCommit = 'f51279699cb59754d1f49aef10eb8e825fe56532'
$expectedBindgen = 'bindgen 0.72.1'
$repositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$manifestPath = Join-Path $repositoryRoot (Join-Path 'bindings' 'configurations.toml')
$outputRoot = Join-Path $repositoryRoot (Join-Path 'bindings' 'generated')
$temporaryRoot = Join-Path $repositoryRoot (Join-Path 'target' ('pdfium-bindgen-' + [guid]::NewGuid().ToString('N')))

if (-not (Test-Path -LiteralPath $PdfiumSource -PathType Container)) {
    throw "PDFium source checkout '$PdfiumSource' does not exist or is not a directory."
}
$sourceRoot = (Resolve-Path -LiteralPath $PdfiumSource).Path
$sourceRevision = (& git -C $sourceRoot rev-parse HEAD 2>$null | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or -not $sourceRevision) {
    throw "PDFium source '$sourceRoot' is not a Git checkout with a readable HEAD revision."
}
if ($sourceRevision -ne $expectedCommit) {
    throw "PDFium source revision mismatch: expected $expectedCommit, got $sourceRevision. Check out the pinned commit before regenerating bindings."
}
$publicDirectory = Join-Path $sourceRoot 'public'
$publicHeaders = @(Get-ChildItem -LiteralPath $publicDirectory -File -Filter 'fpdf*.h' | Sort-Object Name)
if ($publicHeaders.Count -lt 20 -or -not (Test-Path -LiteralPath (Join-Path $publicDirectory 'fpdfview.h'))) {
    throw "PDFium checkout '$sourceRoot' is missing the expected official public C headers."
}
$dirtyHeaderStatus = @(& git -C $sourceRoot status --porcelain=v1 --untracked-files=all -- 'public/fpdf*.h' 2>$null)
if ($LASTEXITCODE -ne 0) {
    throw "Unable to verify the state of public PDFium headers in '$sourceRoot'."
}
if ($dirtyHeaderStatus.Count -gt 0) {
    throw "Public PDFium headers in '$sourceRoot' have modified, staged, deleted, or untracked changes; use a clean checkout of $expectedCommit."
}
if (-not (Test-Path -LiteralPath $manifestPath -PathType Leaf)) {
    throw "Binding configuration manifest '$manifestPath' is missing."
}

if (-not $BindgenPath) {
    $bindgenCommand = Get-Command bindgen -ErrorAction SilentlyContinue
    if (-not $bindgenCommand) {
        throw 'bindgen-cli 0.72.1 was not found on PATH. Install it with: cargo install bindgen-cli --version 0.72.1 --locked'
    }
    $BindgenPath = $bindgenCommand.Source
}
if (-not (Test-Path -LiteralPath $BindgenPath -PathType Leaf)) {
    throw "bindgen executable '$BindgenPath' does not exist."
}
$bindgenVersion = (& $BindgenPath --version | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or $bindgenVersion -ne $expectedBindgen) {
    throw "Expected $expectedBindgen, found '$bindgenVersion'. Install the pinned generator with: cargo install bindgen-cli --version 0.72.1 --locked"
}

if (-not $ClangPath) {
    $clangCommand = Get-Command clang -ErrorAction SilentlyContinue
    if (-not $clangCommand) {
        throw 'Clang was not found on PATH. Install Clang with libclang available, then rerun binding generation.'
    }
    $ClangPath = $clangCommand.Source
}
if (-not (Test-Path -LiteralPath $ClangPath -PathType Leaf)) {
    throw "Clang executable '$ClangPath' does not exist."
}
$clangVersionOutput = @(& $ClangPath --version 2>&1)
if ($LASTEXITCODE -ne 0 -or $clangVersionOutput.Count -eq 0) {
    throw "Unable to read the Clang version from '$ClangPath'."
}
$clangVersion = [string] $clangVersionOutput[0]
if ($clangVersion -notmatch '^(Apple )?clang version ') {
    throw "Unrecognized Clang version output from '$ClangPath': $clangVersion"
}
$clangResourceDir = (& $ClangPath -print-resource-dir | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path -LiteralPath (Join-Path (Join-Path $clangResourceDir 'include') 'stddef.h'))) {
    throw "Clang resource headers were not found under '$clangResourceDir'."
}

$configText = Get-Content -LiteralPath $manifestPath -Raw
$configBlocks = [regex]::Matches($configText, '(?ms)^\[\[config\]\]\s*\r?\n(.*?)(?=^\[\[config\]\]|\z)')
if ($configBlocks.Count -ne 6) {
    throw "Expected six supported target/profile configurations in '$manifestPath', found $($configBlocks.Count)."
}
$configurations = @()
foreach ($blockMatch in $configBlocks) {
    $block = $blockMatch.Groups[1].Value
    $nameMatch = [regex]::Match($block, '(?m)^name\s*=\s*"([a-z0-9-]+)"\s*$')
    $targetMatch = [regex]::Match($block, '(?m)^target\s*=\s*"([a-z0-9_-]+)"\s*$')
    $definesMatch = [regex]::Match($block, '(?m)^defines\s*=\s*\[(.*?)\]\s*$')
    $profileMatch = [regex]::Match($block, '(?m)^profile\s*=\s*"([^"]+)"\s*$')
    if (-not ($nameMatch.Success -and $targetMatch.Success -and $definesMatch.Success -and $profileMatch.Success)) {
        throw "Malformed configuration block in '$manifestPath': each block requires name, target, defines, and profile."
    }
    $defines = @([regex]::Matches($definesMatch.Groups[1].Value, '"([A-Z0-9_]+)"') | ForEach-Object { $_.Groups[1].Value })
    $configurations += [pscustomobject]@{
        Name = $nameMatch.Groups[1].Value
        Target = $targetMatch.Groups[1].Value
        Defines = $defines
        Profile = $profileMatch.Groups[1].Value
    }
}
$expectedNames = @('windows-x64-default', 'windows-x64-v8-xfa', 'linux-x64-default', 'linux-x64-v8-xfa', 'linux-arm64-default', 'linux-arm64-v8-xfa')
if (($configurations.Name -join ',') -ne ($expectedNames -join ',')) {
    throw "The configuration manifest must list the six supported configurations in this order: $($expectedNames -join ', ')."
}

New-Item -ItemType Directory -Force $temporaryRoot | Out-Null
try {
    $wrapperPath = Join-Path $temporaryRoot 'wrapper.h'
    $wrapperLines = @($publicHeaders | ForEach-Object { '#include <' + $_.Name + '>' })
    Set-Content -LiteralPath $wrapperPath -Value $wrapperLines -Encoding ascii
    $headerNames = @($publicHeaders | ForEach-Object { 'public/' + $_.Name })
    $generatedConfigurations = @()

    foreach ($configuration in $configurations) {
        $configOutput = Join-Path $temporaryRoot $configuration.Name
        New-Item -ItemType Directory -Force $configOutput | Out-Null
        $generatedRustPath = Join-Path $configOutput 'bindgen.rs'
        $clangStderrPath = Join-Path $configOutput 'clang-bindgen.stderr.txt'
        $bindgenArguments = @(
            $wrapperPath,
            '--dynamic-loading', 'pdfium',
            '--allowlist-function', '^(FPDF.*|FORM_.*|FSDK_.*)$',
            '--allowlist-type', '^(FPDF.*|FORM.*|FS_.*|FX.*|FSDK.*|UNSUPPORT.*|IPDF.*)$',
            '--allowlist-var', '^(FPDF.*|FORMTYPE.*|FXFA_.*)$',
            '--no-layout-tests',
            '--no-derive-debug',
            '--disable-header-comment',
            '--use-core',
            '--'
        )
        $clangArguments = @('--target=' + $configuration.Target)
        if ($configuration.Target -match 'linux') {
            $clangArguments += @('-nostdinc', '-isystem', (Join-Path $clangResourceDir 'include'), '-I', (Join-Path (Join-Path $repositoryRoot 'bindings') (Join-Path 'compat' 'linux')))
        }
        $clangArguments += @('-I', (Join-Path $sourceRoot 'public'))
        foreach ($define in $configuration.Defines) {
            $clangArguments += '-D' + $define
        }
        $bindgenArguments += $clangArguments
        & $BindgenPath @bindgenArguments > $generatedRustPath 2> $clangStderrPath
        if ($LASTEXITCODE -ne 0) {
            $diagnosticTail = (Get-Content -LiteralPath $clangStderrPath -Tail 20 | Out-String).Trim()
            throw "Binding generation failed for '$($configuration.Name)' using target '$($configuration.Target)'.`n$diagnosticTail"
        }
        $generatedRust = Get-Content -LiteralPath $generatedRustPath -Raw
        $dispatchMarker = [regex]::Match($generatedRust, '(?m)^pub struct pdfium \{')
        if (-not $dispatchMarker.Success) {
            throw "bindgen did not emit the typed PDFium dispatch table for '$($configuration.Name)'."
        }
        $typesRust = $generatedRust.Substring(0, $dispatchMarker.Index)
        if ($typesRust -notmatch '\bFPDF_DOCUMENT\b' -or $typesRust -notmatch '\bFPDF_PAGE\b') {
            throw "Generated type declarations for '$($configuration.Name)' are missing core PDFium handles."
        }
        $dispatchRust = "use super::types::*;`n" + $generatedRust.Substring($dispatchMarker.Index).Replace('pub struct pdfium {', 'pub(crate) struct pdfium {')
        $tableMatch = [regex]::Match($dispatchRust, '(?ms)^pub\(crate\) struct pdfium \{\r?\n(.*?)^\}')
        if (-not $tableMatch.Success) {
            throw "Generated dispatch table was malformed for '$($configuration.Name)'."
        }
        $symbolNames = @([regex]::Matches($tableMatch.Groups[1].Value, '(?m)^\s+pub ((?:FPDF|FORM_|FSDK_)[A-Za-z0-9_]*)\s*:') | ForEach-Object { $_.Groups[1].Value })
        if ($symbolNames.Count -lt 300) {
            throw "Only $($symbolNames.Count) public API symbols were generated for '$($configuration.Name)'; expected a complete public C API binding."
        }
        if (($symbolNames | Select-Object -Unique).Count -ne $symbolNames.Count) {
            throw "Duplicate dispatch symbols were generated for '$($configuration.Name)'."
        }

        $configMetadata = [ordered]@{
            pdfium_version = '157.0.8100.0'
            chromium_branch = 'chromium/8100'
            pdfium_source_commit = $expectedCommit
            source_url = 'https://pdfium.googlesource.com/pdfium/'
            configuration = $configuration.Name
            target = $configuration.Target
            defines = $configuration.Defines
            profile = $configuration.Profile
            bindgen = $bindgenVersion
            clang = $clangVersion
            public_headers = $headerNames
            generated_api_symbol_count = $symbolNames.Count
        }
        $configJson = $configMetadata | ConvertTo-Json -Depth 8
        $typesWithProvenance = "// Generated from the official PDFium public C headers. Do not edit by hand.`n// Source: $expectedCommit; configuration: $($configuration.Name).`n" + $typesRust
        $dispatchWithProvenance = "// Generated from the official PDFium public C headers. Do not edit by hand.`n// Source: $expectedCommit; configuration: $($configuration.Name).`n" + $dispatchRust
        Set-Content -LiteralPath (Join-Path $configOutput 'types.rs') -Value $typesWithProvenance -Encoding utf8
        Set-Content -LiteralPath (Join-Path $configOutput 'dispatch.rs') -Value $dispatchWithProvenance -Encoding utf8
        Set-Content -LiteralPath (Join-Path $configOutput 'provenance.json') -Value $configJson -Encoding utf8
        Remove-Item -LiteralPath $generatedRustPath, $clangStderrPath -Force
        $generatedConfigurations += [pscustomobject]@{
            name = $configuration.Name
            target = $configuration.Target
            defines = $configuration.Defines
            symbol_count = $symbolNames.Count
        }
    }

    & (Join-Path $PSScriptRoot 'generate-runtime-manifests.ps1') -GeneratedRoot $temporaryRoot
    if ($LASTEXITCODE -ne 0) { throw 'Runtime manifest and API wrapper generation failed.' }

    $toolchainMetadata = [ordered]@{
        pdfium_version = '157.0.8100.0'
        chromium_branch = 'chromium/8100'
        pdfium_source_commit = $expectedCommit
        bindgen = $bindgenVersion
        clang = $clangVersion
        configurations = $generatedConfigurations
    }
    $toolchainJson = $toolchainMetadata | ConvertTo-Json -Depth 8

    New-Item -ItemType Directory -Force $outputRoot | Out-Null
    foreach ($configuration in $configurations) {
        $destination = Join-Path $outputRoot $configuration.Name
        New-Item -ItemType Directory -Force $destination | Out-Null
        $generatedConfigPath = Join-Path $temporaryRoot $configuration.Name
        Copy-Item -LiteralPath (Join-Path $generatedConfigPath 'types.rs') -Destination $destination -Force
        Copy-Item -LiteralPath (Join-Path $generatedConfigPath 'dispatch.rs') -Destination $destination -Force
        Copy-Item -LiteralPath (Join-Path $generatedConfigPath 'provenance.json') -Destination $destination -Force
        Copy-Item -LiteralPath (Join-Path $generatedConfigPath 'symbols.json') -Destination $destination -Force
        Copy-Item -LiteralPath (Join-Path $generatedConfigPath 'symbols.rs') -Destination $destination -Force
        Copy-Item -LiteralPath (Join-Path $generatedConfigPath 'api.rs') -Destination $destination -Force
    }
    Set-Content -LiteralPath (Join-Path $outputRoot 'toolchain.json') -Value $toolchainJson -Encoding utf8
    Write-Output "Generated public bindings for $($configurations.Count) target/profile configurations at PDFium commit $expectedCommit."
}
finally {
    if (Test-Path -LiteralPath $temporaryRoot) {
        Remove-Item -LiteralPath $temporaryRoot -Recurse -Force
    }
}



