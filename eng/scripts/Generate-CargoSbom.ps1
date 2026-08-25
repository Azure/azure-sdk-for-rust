#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = 'Workspace')]
param(
  [Parameter(Mandatory, ParameterSetName = 'PackageNames')]
  [ValidateNotNullOrEmpty()]
  [string] $PackageNames,

  [Parameter(Mandatory, ParameterSetName = 'Workspace')]
  [switch] $Workspace
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

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

$packageArgs = if ($Workspace) {
  '--workspace'
}
else {
  '--package ' + (($PackageNames -split ',') -join ' --package ')
}

# Cargo owns SBOM generation, so nightly Cargo can drive the stable compiler without
# weakening the stable-toolchain validation performed by the rest of the pipeline.
$env:CARGO_BUILD_SBOM = 'true'
$env:RUSTC = $stableRustc
$env:RUSTUP_TOOLCHAIN = $stableToolchain

Invoke-LoggedCommand `
  "& `"$nightlyCargo`" -Z sbom build --locked --all-features --keep-going $packageArgs" `
  -GroupOutput

$metadata = Invoke-LoggedCommand `
  "& `"$nightlyCargo`" metadata --no-deps --format-version 1 --locked" |
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
