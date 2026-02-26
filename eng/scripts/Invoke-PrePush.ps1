#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

<#
.SYNOPSIS
Pre-push verification script that mirrors Azure Pipelines CI checks locally.

.DESCRIPTION
Runs the same analysis, testing, spell checking, and link verification steps
that the pull request CI pipeline performs. Use this script before pushing to
gain high confidence that CI will pass.

The script determines which packages have changed relative to a target branch
(default: main) and runs checks against those packages. You can also specify
packages explicitly.

All check steps run to completion even when earlier steps fail, so you can
discover every issue in a single invocation.

.PARAMETER PackageNames
Optional list of package names to check. If not specified, the script detects
packages changed on the current branch relative to the target branch.

.PARAMETER Toolchain
Rust toolchain to use for analysis and testing. Defaults to 'stable'.

.PARAMETER TargetBranch
Branch to diff against when detecting changed files and packages.
Defaults to 'main'.

.PARAMETER OutputDir
Optional directory that receives merged stdout/stderr output from each step as
separate log files. If not specified, nothing is written to disk. The directory
is created if it does not exist. A machine-readable summary.json is also
written so that automated tools can quickly determine which steps failed and
which log files to inspect.

.PARAMETER SkipAnalysis
Skip the Analyze-Code.ps1 step (cargo check, fmt, clippy, doc, audit, etc.).

.PARAMETER SkipTests
Skip the Test-Packages.ps1 step.

.PARAMETER SkipSpellCheck
Skip the spell check step.

.PARAMETER SkipLinkVerification
Skip the link verification step.

.EXAMPLE
./eng/scripts/Invoke-PrePush.ps1

Runs all checks for packages changed on the current branch vs origin/main.

.EXAMPLE
./eng/scripts/Invoke-PrePush.ps1 -PackageNames azure_core,azure_identity

Runs all checks for the specified packages only.

.EXAMPLE
./eng/scripts/Invoke-PrePush.ps1 -SkipTests -SkipLinkVerification

Runs analysis and spell check only for changed packages.

.EXAMPLE
./eng/scripts/Invoke-PrePush.ps1 -OutputDir ./target/prepush

Runs all checks and writes per-step log files and a summary.json to the
specified directory. Useful for automated tools that need to collect all
failures from a single run.
#>

#Requires -Version 7.0
param(
  [string[]]$PackageNames,
  [string]$Toolchain = 'stable',
  [string]$TargetBranch = 'main',
  [string]$OutputDir,
  [switch]$SkipAnalysis,
  [switch]$SkipTests,
  [switch]$SkipSpellCheck,
  [switch]$SkipLinkVerification
)

$ErrorActionPreference = 'Stop'

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))

# ---------------------------------------------------------------------------
# Output capture setup
# ---------------------------------------------------------------------------

if ($OutputDir) {
  if (!(Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
  }
  $OutputDir = (Resolve-Path $OutputDir).Path
}

# ---------------------------------------------------------------------------
# Invoke-Step helper
# ---------------------------------------------------------------------------
# Runs a named step. When $OutputDir is set the step's merged output streams
# are tee'd to a log file. Returns a result hashtable for the summary.

function Invoke-Step {
  param(
    [Parameter(Mandatory)]
    [string]$StepName,
    [Parameter(Mandatory)]
    [string]$DisplayName,
    [Parameter(Mandatory)]
    [scriptblock]$Action
  )

  Write-Host "`n`n=== $DisplayName ===`n"

  $result = @{ name = $StepName; status = 'passed' }
  $logFile = $null
  if ($OutputDir) {
    $logFile = ([System.IO.Path]::Combine($OutputDir, "$StepName.log"))
    $result['log'] = "$StepName.log"
  }

  $savedErrorPref = $ErrorActionPreference
  try {
    # Temporarily allow errors to flow through the pipeline instead of
    # terminating, so that child-script exit codes and error-stream writes
    # are captured rather than thrown as exceptions.
    $ErrorActionPreference = 'Continue'

    if ($logFile) {
      & $Action *>&1 | Tee-Object -FilePath $logFile | Out-Host
    }
    else {
      & $Action | Out-Host
    }

    $ErrorActionPreference = $savedErrorPref

    if ($LASTEXITCODE -and $LASTEXITCODE -ne 0) {
      $result['status'] = 'failed'
    }
  }
  catch {
    $ErrorActionPreference = $savedErrorPref
    $result['status'] = 'failed'
    $errorMsg = "Step '$DisplayName' threw an exception: $_"
    Write-Host $errorMsg
    if ($logFile) {
      $errorMsg | Out-File -FilePath $logFile -Append
    }
  }

  if ($result['status'] -eq 'failed') {
    LogError "$DisplayName failed."
  }

  return $result
}

# ---------------------------------------------------------------------------
# Banner
# ---------------------------------------------------------------------------

Write-Host @"
Pre-push verification
    Toolchain: '$Toolchain'
    TargetBranch: '$TargetBranch'
    PackageNames: '$($PackageNames -join ', ')'
    OutputDir: '$OutputDir'
    SkipAnalysis: $SkipAnalysis
    SkipTests: $SkipTests
    SkipSpellCheck: $SkipSpellCheck
    SkipLinkVerification: $SkipLinkVerification
"@

# Try to fetch the target branch ref for diffing. If this fails (e.g., no
# credentials or network access), warn and continue with the local ref.
Invoke-LoggedCommand "git fetch origin $TargetBranch --quiet" -GroupOutput -DoNotExitOnFailedExitCode
if ($LASTEXITCODE -and $LASTEXITCODE -ne 0) {
  LogWarning "Could not fetch origin/$TargetBranch (exit code $LASTEXITCODE). Falling back to local ref."
}

# ---------------------------------------------------------------------------
# Step 1: Generate PackageInfo directory (fatal — later steps depend on it)
# ---------------------------------------------------------------------------
# Mirror CI's save-package-properties.yml behavior for ServiceDirectory: auto.
# Generate-PR-Diff.ps1 and Save-Package-Properties.ps1 rely on Azure DevOps
# environment variables. We set them to local git equivalents.

$PackageInfoDirectory = ([System.IO.Path]::Combine($RepoRoot, 'target', 'packageinfo'))
$DiffDirectory = ([System.IO.Path]::Combine($RepoRoot, 'target', 'diff'))

# Clean previous runs
if (Test-Path $PackageInfoDirectory) {
  Remove-Item -Path $PackageInfoDirectory -Recurse -Force
}
if (Test-Path $DiffDirectory) {
  Remove-Item -Path $DiffDirectory -Recurse -Force
}

$env:SYSTEM_PULLREQUEST_SOURCECOMMITID = 'HEAD'
$env:SYSTEM_PULLREQUEST_TARGETBRANCH = $TargetBranch

$generatePrDiffScript = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'common', 'scripts', 'Generate-PR-Diff.ps1'))
$savePackagePropertiesScript = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'common', 'scripts', 'Save-Package-Properties.ps1'))

$setupLogFile = $null
if ($OutputDir) {
  $setupLogFile = ([System.IO.Path]::Combine($OutputDir, 'setup.log'))
}

# Capture setup output while still allowing fatal exit on failure.
$setupBlock = {
  Write-Host "--- Generating PR diff ---`n"
  & $generatePrDiffScript -TargetPath $RepoRoot -ArtifactPath $DiffDirectory

  $diffFile = ([System.IO.Path]::Combine($DiffDirectory, 'diff.json'))
  if (!(Test-Path $diffFile)) {
    LogError "Failed to generate PR diff at $diffFile"
    exit 1
  }

  Write-Host "`n--- Saving package properties ---`n"
  & $savePackagePropertiesScript -PrDiff $diffFile -OutDirectory $PackageInfoDirectory
}

$savedErrorPref = $ErrorActionPreference
$ErrorActionPreference = 'Continue'
if ($setupLogFile) {
  & $setupBlock *>&1 | Tee-Object -FilePath $setupLogFile | Out-Host
}
else {
  & $setupBlock | Out-Host
}
$ErrorActionPreference = $savedErrorPref

$diffFile = ([System.IO.Path]::Combine($DiffDirectory, 'diff.json'))
if (!(Test-Path $diffFile)) {
  LogError "Failed to generate PR diff at $diffFile"
  exit 1
}

$packageInfoFiles = @()
if (Test-Path $PackageInfoDirectory) {
  $packageInfoFiles = @(Get-ChildItem $PackageInfoDirectory -Filter "*.json" -Recurse)
}

if ($packageInfoFiles.Count -eq 0) {
  Write-Host "No changed packages detected. Workspace-level checks will still run."
}
else {
  Write-Host "`nDetected changed packages:"
  foreach ($file in $packageInfoFiles) {
    $pkg = Get-Content -Raw $file | ConvertFrom-Json
    Write-Host "  $($pkg.Name)"
  }
}

# If the caller specified explicit package names, filter the PackageInfo files.
if ($PackageNames) {
  if (Test-Path $PackageInfoDirectory) {
    foreach ($file in $packageInfoFiles) {
      $pkg = Get-Content -Raw $file | ConvertFrom-Json
      if ($pkg.Name -notin $PackageNames) {
        Remove-Item $file.FullName
      }
    }
    # Refresh the list after filtering
    $packageInfoFiles = @(Get-ChildItem $PackageInfoDirectory -Filter "*.json" -Recurse -ErrorAction SilentlyContinue)
    if ($packageInfoFiles.Count -eq 0) {
      LogWarning "None of the specified packages ($($PackageNames -join ', ')) matched changed packages. Checks may be limited."
    }
  }
}

# ---------------------------------------------------------------------------
# Run check steps (continue on failure)
# ---------------------------------------------------------------------------

$stepResults = @()

# Step 2: Analyze code
if ($SkipAnalysis) {
  $stepResults += @{ name = 'analysis'; status = 'skipped' }
}
else {
  $analyzeArgs = @(
    '-PackageInfoDirectory', $PackageInfoDirectory,
    '-Toolchain', $Toolchain
  )

  if ($packageInfoFiles.Count -eq 0) {
    $analyzeArgs += '-SkipPackageAnalysis'
  }

  $analyzeScript = ([System.IO.Path]::Combine($PSScriptRoot, 'Analyze-Code.ps1'))

  $stepResults += Invoke-Step 'analysis' 'Running source analysis' {
    & $analyzeScript @analyzeArgs
  }
}

# Step 3: Test packages
if ($SkipTests) {
  $stepResults += @{ name = 'tests'; status = 'skipped' }
}
elseif ($packageInfoFiles.Count -eq 0) {
  Write-Host "`n`n=== Skipping tests (no changed packages detected) ===`n"
  $stepResults += @{ name = 'tests'; status = 'skipped' }
}
else {
  $testScript = ([System.IO.Path]::Combine($PSScriptRoot, 'Test-Packages.ps1'))

  $stepResults += Invoke-Step 'tests' 'Running tests' {
    & $testScript -PackageInfoDirectory $PackageInfoDirectory
  }
}

# Step 4: Spell check changed files
if ($SkipSpellCheck) {
  $stepResults += @{ name = 'spellcheck'; status = 'skipped' }
}
else {
  $spellCheckScript = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'common', 'scripts', 'check-spelling-in-changed-files.ps1'))

  $stepResults += Invoke-Step 'spellcheck' 'Running spell check on changed files' {
    & $spellCheckScript `
      -SourceCommittish HEAD `
      -TargetCommittish "origin/$TargetBranch" `
      -ExitWithError
  }
}

# Step 5: Link verification on changed markdown files
if ($SkipLinkVerification) {
  $stepResults += @{ name = 'linkcheck'; status = 'skipped' }
}
else {
  $changedMarkdownFiles = git -c core.quotepath=off diff "origin/$TargetBranch...HEAD" --name-only --diff-filter=d -- '*.md'

  if (!$changedMarkdownFiles) {
    Write-Host "`n`n=== Skipping link verification (no changed markdown files) ===`n"
    $stepResults += @{ name = 'linkcheck'; status = 'skipped' }
  }
  else {
    $resolvedMdFiles = @()
    foreach ($file in $changedMarkdownFiles) {
      $fullPath = ([System.IO.Path]::Combine($RepoRoot, $file))
      if (Test-Path $fullPath) {
        $resolvedMdFiles += (Resolve-Path $fullPath).Path
      }
    }

    if ($resolvedMdFiles.Count -eq 0) {
      Write-Host "`n`n=== Skipping link verification (no existing changed markdown files) ===`n"
      $stepResults += @{ name = 'linkcheck'; status = 'skipped' }
    }
    else {
      $verifyLinksScript = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'common', 'scripts', 'Verify-Links.ps1'))

      $stepResults += Invoke-Step 'linkcheck' 'Running link verification on changed markdown files' {
        & $verifyLinksScript `
          -urls $resolvedMdFiles `
          -rootUrl "file://$RepoRoot" `
          -checkLinkGuidance $true `
          -localBuildRepoPath $RepoRoot
      }
    }
  }
}

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------

$failed = @($stepResults | Where-Object { $_.status -eq 'failed' })
$passed = @($stepResults | Where-Object { $_.status -eq 'passed' })
$skipped = @($stepResults | Where-Object { $_.status -eq 'skipped' })

Write-Host "`n"
Write-Host "==========================================="
Write-Host "  Pre-push verification summary"
Write-Host "==========================================="
foreach ($step in $stepResults) {
  $icon = switch ($step.status) {
    'passed'  { 'PASS' }
    'failed'  { 'FAIL' }
    'skipped' { 'SKIP' }
  }
  $logNote = ''
  if ($step['log']) {
    $logNote = "  -> $($step['log'])"
  }
  Write-Host "  [$icon] $($step.name)$logNote"
}
Write-Host "-------------------------------------------"
Write-Host "  $($passed.Count) passed, $($failed.Count) failed, $($skipped.Count) skipped"
Write-Host "==========================================="

if ($OutputDir) {
  $overallResult = if ($failed.Count -gt 0) { 'failed' } else { 'passed' }
  $summaryObject = @{
    result = $overallResult
    steps = @($stepResults)
  }
  $summaryJson = $summaryObject | ConvertTo-Json -Depth 3
  $summaryPath = ([System.IO.Path]::Combine($OutputDir, 'summary.json'))
  $summaryJson | Out-File -FilePath $summaryPath -Encoding utf8
  Write-Host "`nSummary written to: $summaryPath"
}

if ($failed.Count -gt 0) {
  Write-Host "`nPre-push verification completed with failures.`n"
  exit 1
}

Write-Host "`nPre-push verification completed successfully.`n"
