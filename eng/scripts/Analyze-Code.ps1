#Requires -Version 7.0

param(
  [string]$Toolchain = 'stable',
  [string]$PackageInfoPath,
  [switch]$SkipPackageAnalysis
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. (Join-Path $PSScriptRoot '..' 'common' 'scripts' 'common.ps1')
. (Join-Path $EngCommonScriptsDir 'Helpers' 'CommandInvocation-Helpers.ps1')

Write-Host "Analyzing code with
    Toolchain: '$Toolchain'`n"

$env:RUSTDOCFLAGS = "-D warnings"
$env:RUSTFLAGS = "-Dwarnings"

Invoke-LoggedCommand "cargo +$Toolchain check -p azure_core --no-default-features"

Invoke-LoggedCommand "cargo +$Toolchain fmt --all -- --check"

Invoke-LoggedCommand "cargo +$Toolchain clippy --all"

Invoke-LoggedCommand "cargo +$Toolchain doc --all --no-deps"

# Verify package dependencies

$verifyDependenciesScript = Join-Path $RepoRoot 'eng' 'scripts' 'verify-dependencies.rs' -Resolve

if (!$SkipPackageAnalysis) {
  if (!(Test-Path $PackageInfoPath)) {
    Write-Error "Package info path '$PackageInfoPath' does not exist."
    exit 1
  }

  $packagesToTest = Get-ChildItem $PackageInfoPath -Filter "*.json" -Recurse
  | Get-Content -Raw
  | ConvertFrom-Json

  Push-Location
  try {
    foreach ($package in $packagesToTest) {
      Set-Location (Join-Path $RepoRoot $package.DirectoryPath)
      Write-Host "Analyzing package: '$($package.Name)' in directory: '$($package.DirectoryPath)'`n"
      Invoke-LoggedCommand "cargo +nightly -Zscript $verifyDependenciesScript"
    }
  }
  finally {
    Pop-Location
  }
}
