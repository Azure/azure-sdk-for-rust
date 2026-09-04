#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = 'Workspace')]
param(
  [Parameter(Mandatory, ParameterSetName = 'PackageNames')]
  [ValidateNotNullOrEmpty()]
  [Alias('PackageNames')]
  [string[]] $PackageName,
  [Parameter(Mandatory, ParameterSetName = 'ManifestDir')]
  [string[]] $ManifestDir,

  [Parameter(Mandatory, ParameterSetName = 'Workspace')]
  [switch] $Workspace
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'common.ps1'))

$nightlyToolchain = Get-ResolvedRustToolchain -Toolchain 'nightly'
$stableToolchain = Get-ResolvedRustToolchain -Toolchain 'stable'
$nightlyCargo = (
  Invoke-LoggedCommand "rustup which --toolchain $nightlyToolchain cargo" |
    Select-Object -First 1
).Trim()
$stableRustc = (
  Invoke-LoggedCommand "rustup which --toolchain $stableToolchain rustc" |
    Select-Object -First 1
).Trim()

$manifestPaths = Get-CargoManifestPaths `
  -PackageName $PackageName `
  -ManifestDir $ManifestDir `
  -Workspace:$Workspace
$workspaceManifestPath = [System.IO.Path]::Combine($RepoRoot, 'Cargo.toml')
$packageArgs = @('--manifest-path', "'$workspaceManifestPath'")
if ($Workspace) {
  $packageArgs += '--workspace'
}
else {
  $packages = Get-CargoSelectedPackages `
    -PackageName $PackageName `
    -ManifestDir $ManifestDir
  foreach ($package in $packages) {
    $packageArgs += '--package', $package.name
  }
}

# Cargo owns SBOM generation, so nightly Cargo can drive the stable compiler without
# weakening the stable-toolchain validation performed by the rest of the pipeline.
$env:CARGO_BUILD_SBOM = 'true'
$env:RUSTC = $stableRustc
$env:RUSTUP_TOOLCHAIN = $stableToolchain

Invoke-LoggedCommand `
  "& `"$nightlyCargo`" -Z sbom build --locked --all-features --keep-going $($packageArgs -join ' ')" `
  -GroupOutput

$metadata = Invoke-LoggedCommand `
  "& `"$nightlyCargo`" metadata --manifest-path '$workspaceManifestPath' --no-deps --format-version 1 --locked" |
    ConvertFrom-Json
$sbomFiles = Get-ChildItem `
  -Path $metadata.target_directory `
  -Filter '*.cargo-sbom.json' `
  -File `
  -Recurse

if (!$sbomFiles) {
  LogError "Cargo completed without generating any SBOM precursor files."
  exit 1
}

Write-Host "Generated $($sbomFiles.Count) Cargo SBOM precursor files."
