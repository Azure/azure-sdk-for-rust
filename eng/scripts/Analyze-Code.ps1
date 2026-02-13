#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory,
  [string]$Toolchain = 'stable',
  [switch]$CheckWasm = $true,
  [switch]$Deny,
  [switch]$SkipPackageAnalysis
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. (Join-Path $PSScriptRoot '..' 'common' 'scripts' 'common.ps1')
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

Write-Host @"
Analyzing code with
    RUSTFLAGS: '${env:RUSTFLAGS}'
    RUSTDOCFLAGS: '${env:RUSTDOCFLAGS}'
    RUST_LOG: '${env:RUST_LOG}'
"@

if ($CheckWasm) {
  Invoke-LoggedCommand "rustup target add wasm32-unknown-unknown" -GroupOutput
}

if ($Deny) {
  Invoke-LoggedCommand "cargo install cargo-deny --locked" -GroupOutput
}

$cargoAuditVersionParams = Get-VersionParamsFromCgManifest cargo-audit
Invoke-LoggedCommand "cargo install cargo-audit --locked $($cargoAuditVersionParams -join ' ')" -GroupOutput
Invoke-LoggedCommand "cargo audit" -GroupOutput

Invoke-LoggedCommand "cargo check --package azure_core --all-features --all-targets --keep-going" -GroupOutput

Invoke-LoggedCommand "cargo fmt --all -- --check" -GroupOutput

Invoke-LoggedCommand "cargo clippy --workspace --all-features --all-targets --keep-going --no-deps" -GroupOutput

if ($CheckWasm) {
  # Save the original RUSTFLAGS to restore later
  $OriginalRustFlags = $env:RUSTFLAGS
  # This is needed to ensure that the `getrandom` crate uses the `wasm_js` backend
  $env:RUSTFLAGS = ${env:RUSTFLAGS} + ' --cfg getrandom_backend="wasm_js"'

  Invoke-LoggedCommand "cargo clippy --target=wasm32-unknown-unknown --workspace --keep-going --no-deps" -GroupOutput

  # Restore the original RUSTFLAGS, since the getrandom config option can only be set for wasm32-unknown-unknown builds.
  $env:RUSTFLAGS = $OriginalRustFlags
}

if ($Deny) {
  Invoke-LoggedCommand "cargo deny --all-features check" -GroupOutput
}

Invoke-LoggedCommand "cargo doc --workspace --no-deps --all-features" -GroupOutput

# Verify package dependencies and keywords
$verifyDependenciesScript = Join-Path $RepoRoot 'eng' 'scripts' 'verify-dependencies.rs' -Resolve
$verifyKeywordsScript = Join-Path $RepoRoot 'eng' 'scripts' 'verify-keywords.rs' -Resolve

if (!$SkipPackageAnalysis) {
  if (!(Test-Path $PackageInfoDirectory)) {
    Write-Host "Analyzing workspace`n"
    Invoke-LoggedCommand "&$verifyDependenciesScript $RepoRoot/Cargo.toml" -GroupOutput
    Invoke-LoggedCommand "&$verifyKeywordsScript $RepoRoot/Cargo.toml" -GroupOutput
    return
  }

  if ($Toolchain -eq 'nightly') {
    Invoke-LoggedCommand "cargo install --locked cargo-docs-rs" -GroupOutput
  }

  $packagesToTest = Get-ChildItem $PackageInfoDirectory -Filter "*.json" -Recurse
  | Get-Content -Raw
  | ConvertFrom-Json

  foreach ($package in $packagesToTest) {
    Write-Host "Analyzing package '$($package.Name)' in directory '$($package.DirectoryPath)'`n"
    Invoke-LoggedCommand "&$verifyDependenciesScript $($package.DirectoryPath)/Cargo.toml" -GroupOutput
    Invoke-LoggedCommand "&$verifyKeywordsScript $($package.DirectoryPath)/Cargo.toml" -GroupOutput

    if ($Toolchain -eq 'nightly') {
      Invoke-LoggedCommand "cargo +nightly docs-rs --package $($package.Name)" -GroupOutput
    }
  }
}
