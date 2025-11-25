#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory,
  [string[]]$PackageNames
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

if ($PackageNames -and $PackageNames.Length -gt 0) {
  $packagesToTest = $packagesToTest | Where-Object { $PackageNames -contains $_.Name }
}

Write-Host "Testing packages:"
foreach ($package in $packagesToTest) {
  Write-Host "  '$($package.Name)' in '$($package.DirectoryPath)'"
}

foreach ($package in $packagesToTest) {
  Write-Host "Testing package '$($package.Name)', with crate types $($package.CrateTypes -join ', ') ..."

  # Launch a child process to test the package to isolate environment changes.
  # NOTE: This means we can only pass simple parameters (strings, arrays) to the child process.
  $Command = @(
    Join-Path $PSScriptRoot 'Test-Package.ps1'
    '-PackageName', $package.Name
    '-DirectoryPath', $package.DirectoryPath
    '-CrateTypes', ($package.CrateTypes -join ',')
  )
  Start-Process -FilePath pwsh -ArgumentList $Command -NoNewWindow -Wait
  if ($LASTEXITCODE -ne 0) {
    Write-Error "Testing package '$($package.Name)' failed."
    exit $LASTEXITCODE
  }
}
