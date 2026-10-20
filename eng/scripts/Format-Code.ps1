#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = 'ManifestDir')]
param(
  [Parameter(Position = 0, ParameterSetName = 'ManifestDir')]
  [string[]] $ManifestDir,

  [Parameter(ParameterSetName = 'PackageName')]
  [Alias('PackageNames')]
  [string[]] $PackageName,

  [Parameter(ParameterSetName = 'PackageInfo')]
  [string] $PackageInfoDirectory,

  [string] $Toolchain = 'stable'
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'common.ps1'))

$resolvedToolchain = Get-ResolvedRustToolchain -Toolchain $Toolchain

$taploCliVersionParams = Get-VersionParamsFromCgManifest taplo-cli
Invoke-LoggedCommand "cargo install taplo-cli --locked $($taploCliVersionParams -join ' ')" -GroupOutput

# Resolve which packages to format. Returns package objects from cargo metadata.
function Get-PackagesToFormat() {
  switch ($PSCmdlet.ParameterSetName) {
    { $ManifestDir } {
      LogDebug "Formatting manifest(s) '$($ManifestDir -join "', '")'"
      return Get-CargoSelectedPackages -ManifestDir $ManifestDir
    }

    'PackageName' {
      LogDebug "Formatting package(s) '$($PackageName -join "', '")'"
      return Get-CargoSelectedPackages -PackageName $PackageName
    }

    'PackageInfo' {
      LogDebug "Formatting packages from '$PackageInfoDirectory'"
      return Get-CargoSelectedPackages -PackageInfoDirectory $PackageInfoDirectory
    }

    default {
      LogDebug "Formatting all packages in workspace"
      return Get-CargoSelectedPackages
    }
  }
}

$originalLocation = Get-Location
try {
  Set-Location $RepoRoot

  $workspaceManifestPath = [System.IO.Path]::Combine($RepoRoot, 'Cargo.toml')

  $packages = Get-PackagesToFormat

  if ($packages) {
    foreach ($package in $packages) {
      Invoke-LoggedCommand "cargo +$resolvedToolchain fmt --manifest-path '$($package.manifest_path)'" -GroupOutput
      Invoke-LoggedCommand "taplo format '$($package.manifest_path)'" -GroupOutput
    }
  } else {
    Invoke-LoggedCommand "cargo +$resolvedToolchain fmt --manifest-path '$workspaceManifestPath' --all" -GroupOutput
    Invoke-LoggedCommand "taplo format" -GroupOutput
  }
}
finally {
  Set-Location $originalLocation
}
