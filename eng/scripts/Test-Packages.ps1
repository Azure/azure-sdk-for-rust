#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

$activeToolchain = Get-ResolvedRustToolchain
$usesJsonTestOutput = Test-IsNightlyRustToolchain

# Helper function to run cargo test, capturing JSON output only when the active
# toolchain supports `--format json -Z unstable-options`.
function Invoke-CargoTest (
  [string]$TestParams,
  [string]$PackageName,
  [string]$ManifestPath,
  [string]$OutputFile
) {
  Write-Host "Running tests for $PackageName"
  $command = "cargo test $TestParams --manifest-path $ManifestPath --all-features --no-fail-fast"

  if ($usesJsonTestOutput) {
    $result = Invoke-LoggedCommand `
      "$command -- --format json -Z unstable-options" `
      -GroupOutput `
      -DoNotExitOnFailedExitCode

    LogGroupStart 'Test result JSON'
    $result | Tee-Object -FilePath $OutputFile
    LogGroupEnd
  }
  else {
    Invoke-LoggedCommand $command -GroupOutput -DoNotExitOnFailedExitCode
  }

  if ($LASTEXITCODE) {
    $message = "Tests failed for $PackageName."
    if ($usesJsonTestOutput) {
      $message += " For more information see the pipeline Tests tab."
    }
    LogError $message
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
    Active Rust toolchain: '$activeToolchain'
"@

$testResultsDir = ([System.IO.Path]::Combine($RepoRoot, 'test-results'))
if ($usesJsonTestOutput) {
  if (!(Test-Path $testResultsDir)) {
    New-Item -ItemType Directory -Path $testResultsDir | Out-Null
  }
  Write-Host "JSON test results will be saved to: $testResultsDir"
}
else {
  Write-Host "Skipping JSON test result capture because the active Rust toolchain is not nightly."
}

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

  $manifestPath = [System.IO.Path]::Combine($packageDirectory, 'Cargo.toml')
  $timestamp = Get-Date -Format "yyyyMMdd-HHmmss-fff"

  $docTestOutput = ([System.IO.Path]::Combine($testResultsDir, "$($package.Name)-doctest-$timestamp.json"))
  Invoke-CargoTest `
    -TestParams "--doc" `
    -PackageName $package.Name `
    -ManifestPath $manifestPath `
    -OutputFile $docTestOutput

  $allTargetsOutput = ([System.IO.Path]::Combine($testResultsDir, "$($package.Name)-alltargets-$timestamp.json"))
  Invoke-CargoTest `
    -TestParams "--lib --bins --tests --examples" `
    -PackageName $package.Name `
    -ManifestPath $manifestPath `
    -OutputFile $allTargetsOutput

  Invoke-LoggedCommand `
    "cargo test --benches --manifest-path $manifestPath --all-features --no-fail-fast" `
    -GroupOutput

  $cleanupScript = ([System.IO.Path]::Combine($packageDirectory, 'Test-Cleanup.ps1'))
  if (Test-Path $cleanupScript) {
    Write-Host "`n`nRunning test cleanup script for package: '$($package.Name)'`n"
    Invoke-LoggedCommand $cleanupScript -GroupOutput -DoNotExitOnFailedExitCode
    # We ignore the exit code of the cleanup script.
  }
}
