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
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

$resolvedToolchain = Get-ResolvedRustToolchain -Toolchain $Toolchain

$taploCliVersionParams = Get-VersionParamsFromCgManifest taplo-cli
Invoke-LoggedCommand "cargo install taplo-cli --locked $($taploCliVersionParams -join ' ')" -GroupOutput

# Resolve which packages to format. Returns package objects from cargo metadata.
function Get-PackagesToFormat() {
  $allPackages = Get-CargoPackages

  switch ($PSCmdlet.ParameterSetName) {
    { $ManifestDir } {
      LogDebug "Formatting manifest(s) '$($ManifestDir -join "', '")'"
      [string[]] $manifestPaths = $ManifestDir | ForEach-Object {
        [System.IO.Path]::Combine((Resolve-Path $_), 'Cargo.toml')
      }
      return $allPackages | Where-Object { $manifestPaths -contains $_.manifest_path }
    }

    'PackageName' {
      LogDebug "Formatting package(s) '$($PackageName -join "', '")'"
      foreach ($name in $PackageName) {
        if (-not ($allPackages.name -contains $name)) {
          LogError "Package '$name' is not in the workspace"
          exit 1
        }
      }
      return $allPackages | Where-Object { $PackageName -contains $_.name }
    }

    'PackageInfo' {
      LogDebug "Formatting packages from '$PackageInfoDirectory'"
      [string[]] $names = Get-PackageNamesFromPackageInfo $PackageInfoDirectory
      return $allPackages | Where-Object { $names -contains $_.name }
    }

    default {
      LogDebug "Formatting all packages in workspace"
      return $allPackages
    }
  }
}

$originalLocation = Get-Location
try {
  Set-Location $RepoRoot

  $packages = Get-PackagesToFormat

  if ($packages) {
    foreach ($package in $packages) {
      Invoke-LoggedCommand "cargo +$resolvedToolchain fmt --manifest-path $($package.manifest_path)" -GroupOutput
      Invoke-LoggedCommand "taplo format $($package.manifest_path)" -GroupOutput
    }
  } else {
    Invoke-LoggedCommand "cargo +$resolvedToolchain fmt --all" -GroupOutput
    Invoke-LoggedCommand "taplo format" -GroupOutput
  }
}
finally {
  Set-Location $originalLocation
}
