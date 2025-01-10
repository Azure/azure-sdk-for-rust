#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$Toolchain = 'stable',
  [bool]$UnitTests = $true,
  [bool]$FunctionalTests = $true,
  [string]$PackageInfoPath
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. "$PSScriptRoot\..\common\scripts\common.ps1"
. (Join-Path $EngCommonScriptsDir "Helpers" CommandInvocation-Helpers.ps1)

Write-Host "Testing packages with
    Toolchain: '$Toolchain'
    UnitTests: '$UnitTests'
    FunctionalTests: '$FunctionalTests'
    PackageInfoPath: '$PackageInfoPath'"

if ($PackageInfoPath) {
  if (!(Test-Path $PackageInfoPath)) {
    Write-Error "Package info path '$PackageInfoPath' does not exist."
    exit 1
  }

  $packagesToTest = Get-ChildItem $PackageInfoPath -Filter "*.json" -Recurse
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
    Write-Host "PackageDirectory" $packageDirectory
    $packageName = $package.Name
    $setupScript = $packageDirectory + "/Test-Setup.ps1"
    Write-Host "Checking for setup in $($packageDirectory+'/Test-Setup.ps1')"
    if (Test-Path ($packageDirectory + "/Test-Setup.ps1")) {
      Write-Host "`n`nRunning test setup script for package: '$($package.Name)': $setupScript`n"
      Invoke-LoggedCommand "$setupScript -packageName $packageName"
      if (!$? -ne 0) {
        Write-Error "Test setup script failed for package: '$($package.Name)'"
        exit 1
      }
    }

    Write-Host "`n`nTesting package: '$($package.Name)'`n"

    Invoke-LoggedCommand "cargo +$Toolchain build --keep-going"
    Write-Host "`n`n"

    $targets = @()
    if ($UnitTests) {
      $targets += "--lib"
    }

    if ($FunctionalTests) {
      $targets += "--bins"
      $targets += "--examples"
      $targets += "--tests"
      $targets += "--benches"
    }

    Invoke-LoggedCommand "cargo +$Toolchain test $($targets -join ' ') --no-fail-fast"
    Write-Host "`n`n"

    Invoke-LoggedCommand "cargo +$Toolchain test --doc --no-fail-fast"
    Write-Host "`n`n"

    if (Test-Path ($packageDirectory + '/Test-Cleanup.ps1')) {
      Write-Host "`n`nRunning test cleanup script for package: '$($package.Name)': $($packageDirectory+'/Test-Cleanup.ps1')`n"
      $cleanupScript = $packageDirectory + '/Test-Cleanup.ps1'
      Write-Host Cleanup script: $cleanupScript
      Invoke-LoggedCommand "$cleanupScript -packageName $packageName"

      # We ignore the exit code of the cleanup script.
    }
  }
  finally {
    Pop-Location
  }
}
