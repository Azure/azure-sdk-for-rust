#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$Toolchain = 'stable',
  [string]$PackageInfoDirectory
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. "$PSScriptRoot\..\common\scripts\common.ps1"

Write-Host "Testing packages with
    Toolchain: '$Toolchain'
    PackageInfoDirectory: '$PackageInfoDirectory'"

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

Write-Host "Setting RUSTFLAGS to '-Dwarnings'"
$env:RUSTFLAGS = "-Dwarnings"

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

    Write-Host "`n`nTesting package: '$($package.Name)'`n"

    Invoke-LoggedCommand "cargo +$Toolchain build --keep-going" -GroupOutput
    Write-Host "`n`n"

    Invoke-LoggedCommand "cargo +$Toolchain test --doc --no-fail-fast" -GroupOutput
    Write-Host "`n`n"

    Invoke-LoggedCommand "cargo +$Toolchain test --all-targets --no-fail-fast --all-features" -GroupOutput
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
