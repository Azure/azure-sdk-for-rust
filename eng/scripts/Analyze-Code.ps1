#Requires -Version 7.0

param(
  [string]$Toolchain = 'stable',
  [string]$PackageInfoPath,
  [switch]$CheckWasm = $true,
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

if ($CheckWasm) {
  Invoke-LoggedCommand "rustup target add --toolchain $Toolchain wasm32-unknown-unknown"
}

Invoke-LoggedCommand "cargo +$Toolchain check -p azure_core --all-features --all-targets --keep-going"

Invoke-LoggedCommand "cargo +$Toolchain fmt --all -- --check"

Invoke-LoggedCommand "cargo +$Toolchain clippy --workspace --all-features --all-targets --keep-going --no-deps"

if ($CheckWasm) {
  Invoke-LoggedCommand "cargo +$Toolchain clippy --target=wasm32-unknown-unknown --workspace --all-features --all-targets --keep-going --no-deps"
}

Invoke-LoggedCommand "cargo +$Toolchain doc --workspace --no-deps"

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
