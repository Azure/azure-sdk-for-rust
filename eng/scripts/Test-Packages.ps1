#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. "$PSScriptRoot/../common/scripts/common.ps1"

# Helper function to run cargo test with JSON output
function Invoke-CargoTestWithJsonOutput (
  [string]$TestParams,
  [string]$PackageName,
  [string]$OutputFile
) {
  Write-Host "Running tests for $PackageName"
  # Use nightly toolchain to enable JSON output format which can be converted
  # and uploaded to DevOps for display in the Tests tab
  # (requires -Z unstable-options)
  $result = Invoke-LoggedCommand `
    "cargo +nightly test $TestParams --package $PackageName --all-features --no-fail-fast -- --format json -Z unstable-options" `
    -GroupOutput `
    -DoNotExitOnFailedExitCode

  LogGroupStart 'Test result JSON'
  $result | Tee-Object -FilePath $OutputFile
  LogGroupEnd

  if ($LASTEXITCODE) {
    LogError "Tests failed for $PackageName. For more information see the pipeline Tests tab."
    exit $LASTEXITCODE
  }
}

Write-Host @"
Testing packages with
    PackageInfoDirectory: '$PackageInfoDirectory'
    RUSTFLAGS: '$env:RUSTFLAGS'
    RUSTDOCFLAGS: '$env:RUSTDOCFLAGS'
    RUST_LOG: '$env:RUST_LOG'
    AZURE_TEST_MODE: '$env:AZURE_TEST_MODE'
    SYSTEM_ACCESSTOKEN: $($env:SYSTEM_ACCESSTOKEN ? 'present' : 'not present')
    ARM_OIDC_TOKEN: $($env:ARM_OIDC_TOKEN ? 'present' : 'not present')
"@

$testResultsDir = ([System.IO.Path]::Combine($RepoRoot, 'test-results'))
if (!(Test-Path $testResultsDir)) {
  New-Item -ItemType Directory -Path $testResultsDir | Out-Null
}
Write-Host "Test results will be saved to: $testResultsDir"

if ($PackageInfoDirectory) {
  if (!(Test-Path $PackageInfoDirectory)) {
    LogError "Package info path '$PackageInfoDirectory' does not exist."
    exit 1
  }

  $packagesToTest = Get-ChildItem $PackageInfoDirectory -Filter "*.json" -Recurse
  | Get-Content -Raw
  | ConvertFrom-Json
}
else {
  $packagesToTest = Get-AllPackageInfoFromRepo
}

Write-Host "Testing packages:"
foreach ($package in $packagesToTest) {
  Write-Host "  '$($package.Name)' in '$($package.DirectoryPath)'"
}

foreach ($package in $packagesToTest) {
  $packageDirectory = ([System.IO.Path]::Combine($RepoRoot, $package.DirectoryPath))

  $setupScript = ([System.IO.Path]::Combine($packageDirectory, 'Test-Setup.ps1'))
  if (Test-Path $setupScript) {
    Write-Host "`n`nRunning test setup script for package: '$($package.Name)'`n"
    Invoke-LoggedCommand $setupScript -GroupOutput
    if (!$? -ne 0) {
      LogError "Test setup script failed for package: '$($package.Name)'"
      exit 1
    }
  }

  Write-Host "`n`nTesting package: '$($package.Name)'`n"

  Invoke-LoggedCommand "cargo build --all-features --keep-going" -GroupOutput
  Write-Host "`n`n"

  $timestamp = Get-Date -Format "yyyyMMdd-HHmmss-fff"

  $docTestOutput = ([System.IO.Path]::Combine($testResultsDir, "$($package.Name)-doctest-$timestamp.json"))
  Invoke-CargoTestWithJsonOutput `
    -TestParams "--doc" `
    -PackageName $package.Name `
    -OutputFile $docTestOutput

  $allTargetsOutput = ([System.IO.Path]::Combine($testResultsDir, "$($package.Name)-alltargets-$timestamp.json"))
  Invoke-CargoTestWithJsonOutput `
    -TestParams "--lib --bins --tests --examples" `
    -PackageName $package.Name `
    -OutputFile $allTargetsOutput

  Invoke-LoggedCommand `
    "cargo test --benches --package $($package.Name) --all-features --no-fail-fast" `
    -GroupOutput

  $cleanupScript = ([System.IO.Path]::Combine($packageDirectory, 'Test-Cleanup.ps1'))
  if (Test-Path $cleanupScript) {
    Write-Host "`n`nRunning test cleanup script for package: '$($package.Name)'`n"
    Invoke-LoggedCommand $cleanupScript -GroupOutput -DoNotExitOnFailedExitCode
    # We ignore the exit code of the cleanup script.
  }
}
