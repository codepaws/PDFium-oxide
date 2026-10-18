param([Parameter(Mandatory=$true)][string] $GeneratedRoot)
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path -LiteralPath $GeneratedRoot).Path
$overridePath = Join-Path (Split-Path $PSScriptRoot -Parent) 'bindings\lifecycle-overrides.toml'
$overrideText = Get-Content -LiteralPath $overridePath -Raw
$overrides = @{}
$overrideNames = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
foreach ($block in [regex]::Matches($overrideText, '(?ms)^\[\[lifecycle\]\]\s*\r?\n(.*?)(?=^\[\[lifecycle\]\]|\z)')) {
    $name = [regex]::Match($block.Groups[1].Value, '(?m)^name\s*=\s*"([A-Za-z0-9_]+)"').Groups[1].Value
    $phase = [regex]::Match($block.Groups[1].Value, '(?m)^phase\s*=\s*"([a-z_]+)"').Groups[1].Value
    if (-not $name -or $phase -notin @('pre_init','pre_init_or_destroyed','coordinator_owned')) { throw "Malformed lifecycle override: $($block.Value)" }
    if (-not $overrideNames.Add($name)) { throw "Duplicate lifecycle override for $name" }
    $overrides[$name] = $phase
}
$targets = @('windows-x64','linux-x64','linux-arm64')
$allNames = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
$symbolCache = @{}
foreach ($target in $targets) {
    foreach ($profile in @('default','v8-xfa')) {
        $config = "$target-$profile"
        $dispatchPath = Join-Path $root "$config\dispatch.rs"
        $dispatch = Get-Content -LiteralPath $dispatchPath -Raw
        $table = [regex]::Match($dispatch, '(?ms)^pub\(crate\) struct pdfium \{(.*?)^\}')
        if (-not $table.Success) { throw "Dispatch table missing in $dispatchPath" }
        $symbols = @([regex]::Matches($table.Groups[1].Value, '(?m)^\s*pub ((?:FPDF|FORM_|FSDK_)[A-Za-z0-9_]*)\s*:') | ForEach-Object { $_.Groups[1].Value })
        if ($symbols.Count -lt 300 -or ($symbols | Select-Object -Unique).Count -ne $symbols.Count) { throw "Invalid symbol table in $dispatchPath" }
        $symbolCache[$config] = $symbols
        foreach ($symbol in $symbols) { [void]$allNames.Add($symbol) }
    }
}
foreach ($name in $overrides.Keys) { if (-not $allNames.Contains($name)) { throw "Lifecycle override names unknown PDFium symbol '$name'" } }
foreach ($target in $targets) {
    $base = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
    foreach ($symbol in $symbolCache["$target-default"]) { [void]$base.Add($symbol) }
    foreach ($profile in @('default','v8-xfa')) {
        $config = "$target-$profile"
        $entries = @()
        foreach ($symbol in $symbolCache[$config]) {
            $optional = $profile -eq 'v8-xfa' -and -not $base.Contains($symbol)
            $phase = if ($overrides.ContainsKey($symbol)) { $overrides[$symbol] } else { 'initialized' }
            $entries += [pscustomobject]@{ name=$symbol; requirement=$(if($optional){'optional'}else{'required'}); capability=$(if($optional){'V8Xfa'}else{$null}); phase=$phase }
        }
        $directory = Join-Path $root $config
        $entries | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $directory 'symbols.json') -Encoding utf8
        $rs = "pub(crate) const REQUIRED_SYMBOLS: &[&str] = &[`n"
        foreach ($entry in $entries) { if ($entry.requirement -eq 'required') { $rs += "    `"$($entry.name)`",`n" } }
        $rs += "];`npub(crate) const OPTIONAL_SYMBOLS: &[(&str, crate::Capability)] = &[`n"
        foreach ($entry in $entries) { if ($entry.requirement -eq 'optional') { $rs += "    (`"$($entry.name)`", crate::Capability::V8Xfa),`n" } }
        $rs += "];`n"
        Set-Content -LiteralPath (Join-Path $directory 'symbols.rs') -Value $rs -Encoding utf8
        & python (Join-Path $PSScriptRoot 'generate-runtime-api.py') (Join-Path $directory 'dispatch.rs') (Join-Path $directory 'symbols.json')
        if ($LASTEXITCODE -ne 0) { throw "Wrapper generation failed for $config" }
    }
}
