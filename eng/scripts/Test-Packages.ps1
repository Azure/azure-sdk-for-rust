#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. "$PSScriptRoot/../common/scripts/common.ps1"

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

if ($PackageInfoDirectory) {
  if (!(Test-Path $PackageInfoDirectory)) {
    Write-Error "Package info path '$PackageInfoDirectory' does not exist."
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
  Push-Location ([System.IO.Path]::Combine($RepoRoot, $package.DirectoryPath))
  try {
    $packageDirectory = ([System.IO.Path]::Combine($RepoRoot, $package.DirectoryPath))

    $setupScript = Join-Path $packageDirectory "Test-Setup.ps1"
    if (Test-Path $setupScript) {
      Write-Host "`n`nRunning test setup script for package: '$($package.Name)'`n"
      Invoke-LoggedCommand $setupScript -GroupOutput
      if (!$? -ne 0) {
        Write-Error "Test setup script failed for package: '$($package.Name)'"
        exit 1
      }
    }

    $featuresList = Join-Path $packageDirectory "test-features.txt"
    $featuresArg = ""
    if (Test-Path $featuresList) {
      $features = Get-Content $featuresList | Where-Object { $_ -and -not $_.StartsWith("#") } | ForEach-Object { $_.Trim() }
      if ($features.Count -gt 0) {
        $featuresArg = "--features " + ($features -join ",")
      }
    }

    Write-Host "`n`nTesting package: '$($package.Name)'`n"

    Invoke-LoggedCommand "cargo build --keep-going $featuresArg" -GroupOutput
    Write-Host "`n`n"

    Invoke-LoggedCommand "cargo test --doc --no-fail-fast $featuresArg" -GroupOutput
    Write-Host "`n`n"

    Invoke-LoggedCommand "cargo test --all-targets --no-fail-fast $featuresArg" -GroupOutput
    Write-Host "`n`n"

    $cleanupScript = Join-Path $packageDirectory "Test-Cleanup.ps1"
    if (Test-Path $cleanupScript) {
      Write-Host "`n`nRunning test cleanup script for package: '$($package.Name)'`n"
      Invoke-LoggedCommand $cleanupScript -GroupOutput
      # We ignore the exit code of the cleanup script.
    }
  }
  finally {
    Pop-Location
  }
}
