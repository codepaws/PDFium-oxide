$ErrorActionPreference = 'Stop'
$repositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$generatedRoot = Join-Path $repositoryRoot 'bindings\generated'
$configurations = @(
    'windows-x64-default',
    'windows-x64-v8-xfa',
    'linux-x64-default',
    'linux-x64-v8-xfa',
    'linux-arm64-default',
    'linux-arm64-v8-xfa'
)

foreach ($configuration in $configurations) {
    $directory = Join-Path $generatedRoot $configuration
    $symbolsPath = Join-Path $directory 'symbols.json'
    $symbolsRsPath = Join-Path $directory 'symbols.rs'
    $dispatchPath = Join-Path $directory 'dispatch.rs'
    $apiPath = Join-Path $directory 'api.rs'
    foreach ($path in @($symbolsPath, $symbolsRsPath, $dispatchPath, $apiPath)) {
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "Generated runtime binding output is missing: $path"
        }
    }

    $symbols = @(Get-Content -LiteralPath $symbolsPath -Raw | ConvertFrom-Json)
    if ($symbols.Count -lt 300) {
        throw "Expected a complete symbol manifest for '$configuration', found $($symbols.Count) entries."
    }
    $names = @($symbols | ForEach-Object { $_.name })
    if (($names | Select-Object -Unique).Count -ne $names.Count) {
        throw "Symbol manifest for '$configuration' contains duplicate function names."
    }

    $dispatch = Get-Content -LiteralPath $dispatchPath -Raw
    if ($dispatch -notmatch '(?m)^use super::types::\*;') {
        throw "Dispatch declarations for '$configuration' do not import the canonical public types."
    }
    if ($dispatch -match '(?m)^pub (?:const|type|struct|enum|union) F(?:PDF|ORM|X|S)') {
        throw "Dispatch declarations for '$configuration' duplicate public types or constants."
    }

    $api = Get-Content -LiteralPath $apiPath -Raw
    $coordinatorOwned = @($symbols | Where-Object { $_.phase -eq 'coordinator_owned' } | ForEach-Object { $_.name })
    foreach ($name in $coordinatorOwned) {
        if ($api -match "(?m)^\s*pub unsafe fn $([regex]::Escape($name))\(") {
            throw "Coordinator-owned function '$name' has an ordinary raw wrapper in '$configuration'."
        }
    }
    foreach ($phase in @('initialized', 'pre_init', 'pre_init_or_destroyed')) {
        foreach ($entry in @($symbols | Where-Object { $_.phase -eq $phase })) {
            if ($api -notmatch "(?m)^\s*pub unsafe fn $([regex]::Escape($entry.name))\(") {
                throw "Function '$($entry.name)' has no generated wrapper for phase '$phase' in '$configuration'."
            }
        }
    }
    foreach ($name in @($symbols | Where-Object { $_.phase -eq 'pre_init_or_destroyed' } | ForEach-Object { $_.name })) {
        if ($api -notmatch "(?s)impl crate::raw::PreInitApi<'_>.*?pub unsafe fn $([regex]::Escape($name))\(" -or
            $api -notmatch "(?s)impl crate::raw::DestroyedApi<'_>.*?pub unsafe fn $([regex]::Escape($name))\(") {
            throw "Function '$name' must be available through both pre-init and destroyed facades in '$configuration'."
        }
    }

    foreach ($entry in $symbols) {
        if ($entry.requirement -eq 'required' -and $entry.capability) {
            throw "Required symbol '$($entry.name)' unexpectedly names a capability in '$configuration'."
        }
        if ($entry.requirement -eq 'optional' -and -not $entry.capability) {
            throw "Optional symbol '$($entry.name)' is missing a capability in '$configuration'."
        }
        if ($entry.phase -notin @('initialized', 'pre_init', 'pre_init_or_destroyed', 'coordinator_owned')) {
            throw "Symbol '$($entry.name)' has an unknown lifecycle phase in '$configuration'."
        }
    }
}

$default = Get-Content -LiteralPath (Join-Path $generatedRoot 'windows-x64-default\symbols.json') -Raw | ConvertFrom-Json
$v8 = Get-Content -LiteralPath (Join-Path $generatedRoot 'windows-x64-v8-xfa\symbols.json') -Raw | ConvertFrom-Json
$baseNames = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
foreach ($entry in $default) { [void] $baseNames.Add($entry.name) }
$v8Extras = @($v8 | Where-Object { -not $baseNames.Contains($_.name) })
if ($v8Extras.Count -eq 0 -or @($v8Extras | Where-Object { $_.requirement -ne 'optional' -or $_.capability -ne 'V8Xfa' }).Count -gt 0) {
    throw 'Windows V8/XFA symbols are not classified as optional V8Xfa capability entries.'
}

'Runtime binding output checks passed for all six profiles.'
