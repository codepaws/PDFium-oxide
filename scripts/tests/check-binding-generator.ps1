param(
    [Parameter(Mandatory = $true)]
    [string] $PdfiumSource,

    [Parameter(Mandatory = $true)]
    [string] $BindgenPath
)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..' '..')).Path
$generator = Join-Path $repoRoot (Join-Path 'scripts' 'regenerate-bindings.ps1')
$expectedCommit = 'f51279699cb59754d1f49aef10eb8e825fe56532'
$testRoot = Join-Path ([System.IO.Path]::GetTempPath()) ('pdfium-bindgen-test-' + [guid]::NewGuid().ToString('N'))
$outputDir = Join-Path $repoRoot (Join-Path 'bindings' 'generated')
$sentinel = Join-Path $outputDir '.validation-sentinel'

function Assert($condition, $message) {
    if (-not $condition) { throw $message }
}

try {
    New-Item -ItemType Directory -Force $testRoot, $outputDir | Out-Null
    Set-Content -Path $sentinel -Value 'keep' -Encoding ascii -NoNewline

    $missingPath = Join-Path $testRoot 'missing-source'
    $missingOutput = & pwsh -NoProfile -File $generator -PdfiumSource $missingPath 2>&1
    $missingExit = $LASTEXITCODE
    Assert ($missingExit -ne 0) 'Missing source path was accepted.'
    Assert (($missingOutput -join "`n") -match 'does not exist|not a directory|source checkout') 'Missing path error was not actionable.'
    Assert ((Get-Content -Raw $sentinel) -eq 'keep') 'Generator changed output before validating the source path.'

    $nonGitSource = Join-Path $testRoot 'non-git-source'
    New-Item -ItemType Directory -Force $nonGitSource | Out-Null
    $nonGitOutput = & pwsh -NoProfile -File $generator -PdfiumSource $nonGitSource 2>&1
    Assert ($LASTEXITCODE -ne 0) 'Non-Git source directory was accepted.'
    Assert (($nonGitOutput -join "`n") -match 'not a Git checkout') 'Non-Git path error was not actionable.'
    Assert ((Get-Content -Raw $sentinel) -eq 'keep') 'Generator changed output before validating the Git checkout.'

    $wrongSource = Join-Path $testRoot 'wrong-source'
    New-Item -ItemType Directory -Force $wrongSource | Out-Null
    git -C $wrongSource init --quiet
    git -C $wrongSource config user.email 'pdfium-test@example.invalid'
    git -C $wrongSource config user.name 'PDFium binding test'
    Set-Content -Path (Join-Path $wrongSource 'input.txt') -Value 'wrong revision' -Encoding ascii
    git -C $wrongSource add input.txt
    git -C $wrongSource commit --quiet -m 'test source revision'
    $wrongOutput = & pwsh -NoProfile -File $generator -PdfiumSource $wrongSource 2>&1
    $wrongExit = $LASTEXITCODE
    Assert ($wrongExit -ne 0) 'Wrong source revision was accepted.'
    Assert (($wrongOutput -join "`n") -match [regex]::Escape($expectedCommit)) 'Wrong revision error did not report the required full commit.'
    Assert ((Get-Content -Raw $sentinel) -eq 'keep') 'Generator changed output before validating the source commit.'

    $dirtySource = Join-Path $testRoot 'dirty-source'
    & git -C $PdfiumSource worktree add --quiet --detach --no-checkout $dirtySource HEAD
    Assert ($LASTEXITCODE -eq 0) 'Could not create an isolated PDFium source fixture.'
    $dirtyPublicDirectory = Join-Path $dirtySource 'public'
    New-Item -ItemType Directory -Force $dirtyPublicDirectory | Out-Null
    Get-ChildItem -LiteralPath (Join-Path $PdfiumSource 'public') -File -Filter 'fpdf*.h' | Copy-Item -Destination $dirtyPublicDirectory
    [System.IO.File]::AppendAllText((Join-Path $dirtySource (Join-Path 'public' 'fpdfview.h')), "`n/* test-only dirty header */")
    $dirtyOutput = & pwsh -NoProfile -File $generator -PdfiumSource $dirtySource 2>&1
    Assert ($LASTEXITCODE -ne 0) 'Modified public PDFium header was accepted.'
    Assert (($dirtyOutput -join "`n") -match 'public.*header.*modified|modified.*public.*header|working tree') 'Dirty-header error was not actionable.'
    Assert ((Get-Content -Raw $sentinel) -eq 'keep') 'Generator changed output before validating public headers.'

    & git -C $dirtySource add -- public/fpdfview.h
    Assert ($LASTEXITCODE -eq 0) 'Could not stage the dirty header fixture.'
    $stagedOutput = & pwsh -NoProfile -File $generator -PdfiumSource $dirtySource 2>&1
    Assert ($LASTEXITCODE -ne 0) 'Staged public PDFium header was accepted.'
    Assert (($stagedOutput -join "`n") -match 'public.*header.*modified|modified.*public.*header|working tree') 'Staged dirty-header error was not actionable.'

    & git -C $dirtySource checkout -- public/fpdfview.h
    Assert ($LASTEXITCODE -eq 0) 'Could not reset the staged header fixture.'
    Remove-Item -LiteralPath (Join-Path $dirtySource (Join-Path 'public' 'fpdfview.h'))
    $deletedOutput = & pwsh -NoProfile -File $generator -PdfiumSource $dirtySource 2>&1
    Assert ($LASTEXITCODE -ne 0) 'Deleted public PDFium header was accepted.'
    Assert (($deletedOutput -join "`n") -match 'expected official public C headers') 'Deleted-header error was not actionable.'

    & git -C $dirtySource checkout -- public/fpdfview.h
    Assert ($LASTEXITCODE -eq 0) 'Could not restore the deleted header fixture.'
    Set-Content -LiteralPath (Join-Path $dirtySource (Join-Path 'public' 'fpdf-test-untracked.h')) -Value '/* test-only header */'
    $untrackedOutput = & pwsh -NoProfile -File $generator -PdfiumSource $dirtySource 2>&1
    Assert ($LASTEXITCODE -ne 0) 'Untracked public PDFium header was accepted.'
    Assert (($untrackedOutput -join "`n") -match 'public.*header.*modified|modified.*public.*header|working tree') 'Untracked dirty-header error was not actionable.'
    Assert ((Get-Content -Raw $sentinel) -eq 'keep') 'Generator changed output before validating public headers.'

    $missingBindgen = Join-Path $testRoot 'missing-bindgen'
    $missingBindgenOutput = & pwsh -NoProfile -File $generator -PdfiumSource $PdfiumSource -BindgenPath $missingBindgen 2>&1
    Assert ($LASTEXITCODE -ne 0) 'Missing bindgen executable was accepted.'
    Assert (($missingBindgenOutput -join "`n") -match 'bindgen executable.*does not exist') 'Missing bindgen error was not actionable.'
    Assert ((Get-Content -Raw $sentinel) -eq 'keep') 'Generator changed output before validating bindgen.'

    $wrongBindgen = Join-Path $testRoot 'wrong-bindgen'
    if ($IsWindows) {
        $wrongBindgen += '.cmd'
        @('@echo off', 'echo bindgen 0.99.0') | Set-Content -Path $wrongBindgen -Encoding ascii
    }
    else {
        Set-Content -Path $wrongBindgen -Value "#!/bin/sh`necho 'bindgen 0.99.0'" -Encoding ascii
        & chmod +x $wrongBindgen
        Assert ($LASTEXITCODE -eq 0) 'Could not make wrong-bindgen fixture executable.'
    }

    $wrongBindgenOutput = & pwsh -NoProfile -File $generator -PdfiumSource $PdfiumSource -BindgenPath $wrongBindgen 2>&1
    Assert ($LASTEXITCODE -ne 0) 'Wrong bindgen version was accepted.'
    Assert (($wrongBindgenOutput -join "`n") -match 'Expected bindgen 0\.72\.1') 'Wrong bindgen error did not identify the pinned version.'
    Assert ((Get-Content -Raw $sentinel) -eq 'keep') 'Generator changed output before validating the bindgen version.'

    $missingClang = Join-Path $testRoot 'missing-clang'
    $missingClangOutput = & pwsh -NoProfile -File $generator -PdfiumSource $PdfiumSource -BindgenPath $BindgenPath -ClangPath $missingClang 2>&1
    Assert ($LASTEXITCODE -ne 0) 'Missing Clang executable was accepted.'
    Assert (($missingClangOutput -join "`n") -match 'Clang executable.*does not exist') 'Missing Clang error was not actionable.'
    Assert ((Get-Content -Raw $sentinel) -eq 'keep') 'Generator changed output before validating Clang.'

    'Binding generator validation checks passed.'
}
finally {
    $dirtySource = Join-Path $testRoot 'dirty-source'
    if (Test-Path -LiteralPath $dirtySource) {
        & git -C $PdfiumSource worktree remove --force $dirtySource 2>$null
    }
    Remove-Item -LiteralPath $sentinel -Force -ErrorAction SilentlyContinue
    Remove-Item -LiteralPath $testRoot -Recurse -Force -ErrorAction SilentlyContinue
}
