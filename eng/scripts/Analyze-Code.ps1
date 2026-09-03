#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = 'PackageInfo')]
param(
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string]$PackageInfoDirectory,

  [Parameter(Position = 0, ParameterSetName = 'PackageName')]
  [ValidateNotNullOrEmpty()]
  [Alias('PackageNames')]
  [string[]]$PackageName,

  [Parameter(ParameterSetName = 'ManifestDir')]
  [string[]]$ManifestDir,

  [string]$Toolchain = 'stable',
  [switch]$Audit,
  [switch]$Deny,
  [switch]$SkipPackageAnalysis
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'common.ps1'))

$resolvedToolchain = Get-ResolvedRustToolchain -Toolchain $Toolchain
$isNightlyToolchain = Test-IsNightlyRustToolchain -Toolchain $Toolchain

Write-Host @"
Analyzing code with
    RUSTFLAGS: '${env:RUSTFLAGS}'
    RUSTDOCFLAGS: '${env:RUSTDOCFLAGS}'
    RUST_LOG: '${env:RUST_LOG}'
"@

if ($Audit) {
  $cargoAuditVersionParams = Get-VersionParamsFromCgManifest cargo-audit
  Invoke-LoggedCommand "cargo install cargo-audit --locked $($cargoAuditVersionParams -join ' ')" -GroupOutput
}

if ($Deny) {
  $cargoDenyVersionParams = Get-VersionParamsFromCgManifest cargo-deny
  Invoke-LoggedCommand "cargo install cargo-deny --locked $($cargoDenyVersionParams -join ' ')" -GroupOutput
}

$taploCliVersionParams = Get-VersionParamsFromCgManifest taplo-cli
Invoke-LoggedCommand "cargo install taplo-cli --locked $($taploCliVersionParams -join ' ')" -GroupOutput

$packageInfoPath = $PackageInfoDirectory
if ($PackageInfoDirectory -and !(Test-Path -Path $PackageInfoDirectory -PathType Container)) {
  $packageInfoPath = $null
}

$packagesToAnalyze = Get-CargoSelectedPackages `
  -PackageName $PackageName `
  -ManifestDir $ManifestDir `
  -PackageInfoDirectory $packageInfoPath
$workspaceManifestPath = [System.IO.Path]::Combine($RepoRoot, 'Cargo.toml')
$packageArgs = if ($PackageName -or $ManifestDir) {
  '--package ' + ($packagesToAnalyze.name -join ' --package ')
}

if ($Audit) {
  Invoke-LoggedCommand "cargo audit" -GroupOutput
}

Invoke-LoggedCommand "cargo check --manifest-path sdk/core/azure_core/Cargo.toml $packageArgs --all-features --all-targets --keep-going" -GroupOutput

if ($packageArgs) {
  Invoke-LoggedCommand "cargo fmt --manifest-path '$workspaceManifestPath' $packageArgs -- --check" -GroupOutput
}
else {
  Invoke-LoggedCommand "cargo fmt --manifest-path '$workspaceManifestPath' --all -- --check" -GroupOutput
}

Invoke-LoggedCommand "taplo format --check"

Invoke-LoggedCommand "cargo clippy --manifest-path '$workspaceManifestPath' $packageArgs --all-features --all-targets --keep-going --no-deps" -GroupOutput

if ($Deny) {
  Invoke-LoggedCommand "cargo deny --manifest-path '$workspaceManifestPath' --all-features check bans licenses sources" -GroupOutput
}

Invoke-LoggedCommand "cargo doc --manifest-path '$workspaceManifestPath' $packageArgs --no-deps --all-features" -GroupOutput

# Verify package dependencies and keywords
$verifyDependenciesScript = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'verify-dependencies.rs'))
$verifyKeywordsScript = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'verify-keywords.rs'))
$checkApiSupersetManifest = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'check_api_superset', 'Cargo.toml'))

if (!$SkipPackageAnalysis) {
  $checkApiSupersetCrates = @('typespec', 'typespec_client_core', 'azure_core')

  if (!$PackageName -and !$ManifestDir -and !$packageInfoPath) {
    Write-Host "Analyzing workspace`n"
    Invoke-LoggedCommand "&$verifyDependenciesScript $workspaceManifestPath" -GroupOutput
    Invoke-LoggedCommand "&$verifyKeywordsScript $workspaceManifestPath" -GroupOutput

    Invoke-LoggedCommand "cargo run --manifest-path $checkApiSupersetManifest" -GroupOutput
    return
  }

  # Ideally would want to install this with the others, but not replicate the conditions in which the tool is run.
  if ($isNightlyToolchain) {
    $cargoDocsRsVersionParams = Get-VersionParamsFromCgManifest cargo-docs-rs
    Invoke-LoggedCommand "cargo install cargo-docs-rs --locked $($cargoDocsRsVersionParams -join ' ')" -GroupOutput
  }

  $shouldCheckApiSuperset = $false
  foreach ($package in $packagesToAnalyze) {
    $packageManifestPath = $package.manifest_path
    Write-Host "Analyzing package '$($package.name)' from '$packageManifestPath'`n"
    Invoke-LoggedCommand "&$verifyDependenciesScript $packageManifestPath" -GroupOutput
    Invoke-LoggedCommand "&$verifyKeywordsScript $packageManifestPath" -GroupOutput

    if ($isNightlyToolchain) {
      Invoke-LoggedCommand "cargo +$resolvedToolchain docs-rs --manifest-path $packageManifestPath" -GroupOutput
    }

    if ($checkApiSupersetCrates -contains $package.name) {
      $shouldCheckApiSuperset = $true
    }
  }

  if ($shouldCheckApiSuperset) {
    Invoke-LoggedCommand "cargo run --manifest-path $checkApiSupersetManifest" -GroupOutput
  }
}
