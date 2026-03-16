#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory,
  [string]$Toolchain = 'stable',
  [switch]$Deny,
  [switch]$SkipPackageAnalysis
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

Write-Host @"
Analyzing code with
    RUSTFLAGS: '${env:RUSTFLAGS}'
    RUSTDOCFLAGS: '${env:RUSTDOCFLAGS}'
    RUST_LOG: '${env:RUST_LOG}'
"@

if ($Deny) {
  Invoke-LoggedCommand "cargo install cargo-deny --locked" -GroupOutput
}

$cargoAuditVersionParams = Get-VersionParamsFromCgManifest cargo-audit
$requiredVersion = $cargoAuditVersionParams[1]
$installedVersion = $null
$cargoAuditCmd = (Get-Command cargo-audit -ErrorAction SilentlyContinue)
if ($cargoAuditCmd) {
  $versionOutput = cargo audit --version 2>$null
  if ($versionOutput -match '(\d+\.\d+\.\d+)') {
    $installedVersion = $Matches[1]
  } else {
    LogWarning "Unable to parse cargo-audit version from output: '$versionOutput'. Will reinstall cargo-audit $requiredVersion."
  }
}
if ($installedVersion -ne $requiredVersion) {
  Invoke-LoggedCommand "cargo install cargo-audit --locked $($cargoAuditVersionParams -join ' ')" -GroupOutput
} else {
  Write-Host "cargo-audit $requiredVersion is already installed, skipping install."
}
Invoke-LoggedCommand "cargo audit" -GroupOutput

Invoke-LoggedCommand "cargo check --package azure_core --all-features --all-targets --keep-going" -GroupOutput

Invoke-LoggedCommand "cargo fmt --all -- --check" -GroupOutput

Invoke-LoggedCommand "cargo clippy --workspace --all-features --all-targets --keep-going --no-deps" -GroupOutput

if ($Deny) {
  Invoke-LoggedCommand "cargo deny --all-features check" -GroupOutput
}

Invoke-LoggedCommand "cargo doc --workspace --no-deps --all-features" -GroupOutput

# Verify package dependencies and keywords
$verifyDependenciesScript = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'verify-dependencies.rs'))
$verifyKeywordsScript = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'verify-keywords.rs'))
$checkApiSupersetManifest = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'check_api_superset', 'Cargo.toml'))

if (!$SkipPackageAnalysis) {
  $checkApiSupersetCrates = @('typespec', 'typespec_client_core', 'azure_core')

  if (!(Test-Path $PackageInfoDirectory)) {
    Write-Host "Analyzing workspace`n"
    $manifestPath = ([System.IO.Path]::Combine($RepoRoot, 'Cargo.toml'))
    Invoke-LoggedCommand "&$verifyDependenciesScript $manifestPath" -GroupOutput
    Invoke-LoggedCommand "&$verifyKeywordsScript $manifestPath" -GroupOutput

    Invoke-LoggedCommand "cargo run --manifest-path $checkApiSupersetManifest" -GroupOutput
    return
  }

  if ($Toolchain -eq 'nightly') {
    Invoke-LoggedCommand "cargo install --locked cargo-docs-rs" -GroupOutput
  }

  $packagesToTest = Get-ChildItem $PackageInfoDirectory -Filter "*.json" -Recurse
  | Get-Content -Raw
  | ConvertFrom-Json

  $shouldCheckApiSuperset = $false
  foreach ($package in $packagesToTest) {
    Write-Host "Analyzing package '$($package.Name)' in directory '$($package.DirectoryPath)'`n"
    $packageManifestPath = ([System.IO.Path]::Combine($package.DirectoryPath, 'Cargo.toml'))
    Invoke-LoggedCommand "&$verifyDependenciesScript $packageManifestPath" -GroupOutput
    Invoke-LoggedCommand "&$verifyKeywordsScript $packageManifestPath" -GroupOutput

    if ($Toolchain -eq 'nightly') {
      Invoke-LoggedCommand "cargo +nightly docs-rs --package $($package.Name)" -GroupOutput
    }

    if ($checkApiSupersetCrates -contains $package.Name) {
      $shouldCheckApiSuperset = $true
    }
  }

  if ($shouldCheckApiSuperset) {
    Invoke-LoggedCommand "cargo run --manifest-path $checkApiSupersetManifest" -GroupOutput
  }
}
