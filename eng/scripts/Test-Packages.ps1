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

    Invoke-LoggedCommand "cargo +$Toolchain test --all-features $($targets -join ' ') --no-fail-fast"
    Write-Host "`n`n"

    Invoke-LoggedCommand "cargo +$Toolchain test --all-features --doc --no-fail-fast"
    Write-Host "`n`n"
  }
  finally {
    Pop-Location
  }
}
