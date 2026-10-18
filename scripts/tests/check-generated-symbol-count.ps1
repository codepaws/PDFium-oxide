param([string] $PdfiumSource)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
if (-not $PdfiumSource) { $PdfiumSource = Join-Path $repoRoot 'target\pdfium-source-pinned' }
$sourceRoot = (Resolve-Path -LiteralPath $PdfiumSource).Path
$expectedCommit = 'f51279699cb59754d1f49aef10eb8e825fe56532'
if ((& git -C $sourceRoot rev-parse HEAD | Out-String).Trim() -ne $expectedCommit) {
    throw "PDFium source checkout must be at pinned commit $expectedCommit."
}
$public = Join-Path $sourceRoot 'public'
$tempRoot = Join-Path $repoRoot ('target\symbol-audit-' + [guid]::NewGuid().ToString('N'))
$wrapper = Join-Path $tempRoot 'wrapper.h'

try {
    New-Item -ItemType Directory -Force $tempRoot | Out-Null
    $headers = Get-ChildItem -LiteralPath $public -File -Filter 'fpdf*.h' | Sort-Object Name
    Set-Content -Path $wrapper -Value @($headers | ForEach-Object { '#include <' + $_.Name + '>' }) -Encoding ascii
    $clang = (Get-Command clang -ErrorAction Stop).Source
    $resource = (& $clang -print-resource-dir | Out-String).Trim()
    foreach ($configurationDirectory in Get-ChildItem -LiteralPath (Join-Path $repoRoot 'bindings\generated') -Directory) {
        $configuration = $configurationDirectory.Name
        $generatedPath = $configurationDirectory.FullName
        $metadata = Get-Content -Raw (Join-Path $generatedPath 'provenance.json') | ConvertFrom-Json
        $arguments = @('--target=' + $metadata.target)
        if ($metadata.target -match 'linux') {
            $arguments += @('-nostdinc', '-isystem', (Join-Path $resource 'include'), '-I', (Join-Path $repoRoot 'bindings\compat\linux'))
        }
        $arguments += @('-I', $public)
        foreach ($define in $metadata.defines) { $arguments += '-D' + $define }
        $arguments += @('-Xclang', '-ast-dump=json', '-fsyntax-only', $wrapper)
        $astFile = Join-Path $tempRoot ($configuration + '.ast.json')
        $errorFile = Join-Path $tempRoot ($configuration + '.clang.stderr.txt')
        & $clang @arguments > $astFile 2> $errorFile
        if ($LASTEXITCODE -ne 0) { throw "Clang AST extraction failed for '$configuration': $((Get-Content $errorFile -Tail 10) -join ' ')" }

        $ast = Get-Content -LiteralPath $astFile -Raw | ConvertFrom-Json -Depth 100
        $stack = [System.Collections.Stack]::new()
        $stack.Push($ast)
        $sourceSymbols = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
        while ($stack.Count -gt 0) {
            $node = $stack.Pop()
            if ($node.kind -eq 'FunctionDecl' -and $node.name -match '^(FPDF|FORM_|FSDK_)') {
                [void] $sourceSymbols.Add($node.name)
            }
            foreach ($child in $node.inner) { $stack.Push($child) }
        }

        $dispatch = Get-Content -Raw (Join-Path $generatedPath 'dispatch.rs')
        $tableMatch = [regex]::Match($dispatch, '(?ms)^pub\(crate\) struct pdfium \{\r?\n(.*?)^\}')
        if (-not $tableMatch.Success) { throw "Generated dispatch table was not found for '$configuration'." }
        $tableSymbols = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
        foreach ($symbolMatch in [regex]::Matches($tableMatch.Groups[1].Value, '(?m)^\s+pub ((?:FPDF|FORM_|FSDK_)[A-Za-z0-9_]*)\s*:')) {
            [void] $tableSymbols.Add($symbolMatch.Groups[1].Value)
        }
        $missing = @($sourceSymbols | Where-Object { -not $tableSymbols.Contains($_) })
        $extra = @($tableSymbols | Where-Object { -not $sourceSymbols.Contains($_) })
        if ($missing.Count -or $extra.Count) {
            throw "Header/dispatch API mismatch for '$configuration'.`n  Missing from dispatch: $((@($missing | Sort-Object) -join ', '))`n  Extra in dispatch: $((@($extra | Sort-Object) -join ', '))"
        }

        # Independently verify both runtime metadata and generated wrappers against
        # the declaration inventory extracted from the pinned public headers.
        $symbolManifest = @(Get-Content -LiteralPath (Join-Path $generatedPath 'symbols.json') -Raw | ConvertFrom-Json)
        $manifestSymbols = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
        foreach ($entry in $symbolManifest) { [void] $manifestSymbols.Add([string] $entry.name) }
        $manifestMissing = @($sourceSymbols | Where-Object { -not $manifestSymbols.Contains($_) })
        $manifestExtra = @($manifestSymbols | Where-Object { -not $sourceSymbols.Contains($_) })
        if ($manifestMissing.Count -or $manifestExtra.Count -or $manifestSymbols.Count -ne $symbolManifest.Count) {
            throw "Header/symbol-manifest mismatch for '$configuration'.`n  Missing from manifest: $((@($manifestMissing | Sort-Object) -join ', '))`n  Extra in manifest: $((@($manifestExtra | Sort-Object) -join ', '))`n  Manifest entries: $($symbolManifest.Count); unique names: $($manifestSymbols.Count)."
        }

        $api = Get-Content -LiteralPath (Join-Path $generatedPath 'api.rs') -Raw
        $wrapperSymbols = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
        foreach ($wrapperMatch in [regex]::Matches($api, '(?m)^\s*pub unsafe fn ((?:FPDF|FORM_|FSDK_)[A-Za-z0-9_]*)\(')) {
            [void] $wrapperSymbols.Add($wrapperMatch.Groups[1].Value)
        }
        $expectedWrappers = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
        foreach ($entry in $symbolManifest) { if ($entry.phase -ne 'coordinator_owned') { [void] $expectedWrappers.Add([string] $entry.name) } }
        $wrapperMissing = @($expectedWrappers | Where-Object { -not $wrapperSymbols.Contains($_) })
        $wrapperExtra = @($wrapperSymbols | Where-Object { -not $expectedWrappers.Contains($_) })
        if ($wrapperMissing.Count -or $wrapperExtra.Count) {
            throw "Header/raw-wrapper mismatch for '$configuration'.`n  Missing wrappers: $((@($wrapperMissing | Sort-Object) -join ', '))`n  Extra wrappers: $((@($wrapperExtra | Sort-Object) -join ', '))`nCoordinator-owned entries are implemented by the runtime coordinator and intentionally excluded from api.rs."
        }
        if ($metadata.generated_api_symbol_count -ne $sourceSymbols.Count) {
            throw "'$configuration' provenance reports $($metadata.generated_api_symbol_count) symbols, but the headers declare $($sourceSymbols.Count)."
        }
        Remove-Variable ast, stack, sourceSymbols, tableSymbols
    }
    'Public header API audit passed for all six generated configurations.'
}
finally {
    Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
}
