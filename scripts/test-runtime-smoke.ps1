[CmdletBinding()]
param(
    [ValidateSet('default', 'v8', 'all')]
    [string] $Profile = 'all',
    [switch] $ForceDownload
)

$ErrorActionPreference = 'Stop'

$releaseTag = 'chromium/8100'
$releaseName = 'PDFium 157.0.8100.0'
$releaseApi = 'https://api.github.com/repos/bblanchon/pdfium-binaries/releases/tags/chromium%2F8100'
$repositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$fixtureRoot = Join-Path $repositoryRoot 'target\pdfium-smoke'
$previousPdfiumPath = $env:PDFIUM_PATH
$release = Invoke-RestMethod -Uri $releaseApi -Headers @{ 'User-Agent' = 'PDFium-oxide-smoke-test' }
if ($release.name -ne $releaseName -or $release.tag_name -ne $releaseTag) {
    throw "Unexpected PDFium release metadata: $($release.name) ($($release.tag_name)). Expected $releaseName ($releaseTag)."
}

$profiles = @(
    [pscustomobject]@{ Name = 'default'; Asset = 'pdfium-win-x64.tgz'; Feature = $false; Sha256 = '51621bcc1d9be9ea448f4b84b0516862b928e3968d3fca899af1a6d6e2848d6d' },
    [pscustomobject]@{ Name = 'v8'; Asset = 'pdfium-v8-win-x64.tgz'; Feature = $true; Sha256 = '10a926574aa43ef673ba1a89bc64c559307ab541ca6169299d77797a8472dd7b' }
)
if ($Profile -ne 'all') { $profiles = @($profiles | Where-Object { $_.Name -eq $Profile }) }

try {
foreach ($item in $profiles) {
    $asset = @($release.assets | Where-Object { $_.name -eq $item.Asset })
    if ($asset.Count -ne 1) { throw "Could not uniquely find release asset '$($item.Asset)'." }
    if ($asset[0].digest -ne "sha256:$($item.Sha256)") {
        throw "Release digest mismatch for '$($item.Asset)': expected sha256:$($item.Sha256), got '$($asset[0].digest)'."
    }

    $profileDirectory = Join-Path $fixtureRoot $item.Name
    $archivePath = Join-Path $fixtureRoot $item.Asset
    New-Item -ItemType Directory -Force -Path $profileDirectory | Out-Null
    if ($ForceDownload -or -not (Test-Path -LiteralPath $archivePath -PathType Leaf)) {
        Invoke-WebRequest -Uri $asset[0].browser_download_url -OutFile $archivePath
    }
    $actualHash = (Get-FileHash -LiteralPath $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actualHash -ne $item.Sha256) {
        Remove-Item -LiteralPath $archivePath -Force
        throw "Downloaded archive hash mismatch for '$($item.Asset)': expected $($item.Sha256), got $actualHash."
    }

    $versionPath = Join-Path $profileDirectory 'VERSION'
    if ($ForceDownload -or -not (Test-Path -LiteralPath (Join-Path $profileDirectory 'bin\pdfium.dll') -PathType Leaf)) {
        $entries = @(& tar -tzf $archivePath)
        if ($LASTEXITCODE -ne 0) { throw "Unable to inspect archive '$archivePath'." }
        foreach ($entry in $entries) {
            $normalized = $entry.Replace('\', '/')
            if ($normalized.StartsWith('/') -or $normalized -match '(^|/)\.\.(/|$)' -or $normalized -match '^[A-Za-z]:') {
                throw "Unsafe archive entry '$entry' in '$($item.Asset)'."
            }
        }
        & tar -xzf $archivePath -C $profileDirectory
        if ($LASTEXITCODE -ne 0) { throw "Unable to extract archive '$($item.Asset)'." }
    }
    if (-not (Test-Path -LiteralPath (Join-Path $profileDirectory 'bin\pdfium.dll') -PathType Leaf)) {
        throw "Extracted PDFium DLL is missing for profile '$($item.Name)'."
    }
    $versionText = Get-Content -LiteralPath $versionPath -Raw
    if ($versionText -notmatch '(?m)^MAJOR=157$' -or $versionText -notmatch '(?m)^BUILD=8100$') {
        throw "Extracted PDFium version does not match $releaseName for profile '$($item.Name)'."
    }

    $env:PDFIUM_PATH = (Resolve-Path (Join-Path $profileDirectory 'bin\pdfium.dll')).Path
    if ($item.Feature) {
        & cargo test --offline --features pdfium-v8 --test runtime_smoke -- --ignored
        if ($LASTEXITCODE -eq 0) { & cargo test --offline --features pdfium-v8 --test document_workflows -- --ignored }
    } else {
        & cargo test --offline --test runtime_smoke -- --ignored
        if ($LASTEXITCODE -eq 0) { & cargo test --offline --test document_workflows -- --ignored }
    }
    if ($LASTEXITCODE -ne 0) { throw "Runtime smoke test failed for profile '$($item.Name)'." }
}
}
finally {
    if ($null -eq $previousPdfiumPath) {
        Remove-Item Env:\PDFIUM_PATH -ErrorAction SilentlyContinue
    } else {
        $env:PDFIUM_PATH = $previousPdfiumPath
    }
}
