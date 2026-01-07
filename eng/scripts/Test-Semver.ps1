#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
  [Parameter(ParameterSetName = 'Named')]
  [string[]]$PackageNames,
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string]$PackageInfoDirectory,
  [switch]$IgnoreCgManifestVersion
)

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

function Get-OutputPackageNames($workspacePackages) {
  $names = @()
  switch ($PsCmdlet.ParameterSetName) {
    'Named' {
      $names = $PackageNames
    }

    'PackageInfo' {
      $names = Get-PackageNamesFromPackageInfo $PackageInfoDirectory
    }

    default {
      return $workspacePackages.name

    }
  }

  foreach ($name in $names) {
    if (-not $workspacePackages.name.Contains($name)) {
      Write-Error "Package '$name' is not in the workspace or does not publish"
      exit 1
    }
  }

  return $names
}

$packages = Get-CargoPackages
$outputPackageNames = Get-OutputPackageNames $packages

$versionParams = @()
if (!$IgnoreCgManifestVersion) {
  $versionParams = Get-VersionParamsFromCgManifest cargo-semver-checks
}

LogGroupStart "cargo install cargo-semver-checks --locked $($versionParams -join ' ')"
Write-Host "cargo install cargo-semver-checks --locked $($versionParams -join ' ')"
cargo install cargo-semver-checks --locked @versionParams
LogGroupEnd

$packageParams = @()
foreach ($packageName in $outputPackageNames) {
  $packageParams += "--package"
  $packageParams += $packageName
}

LogGroupStart "cargo semver-checks $($packageParams -join ' ')"
Write-Host "cargo semver-checks $($packageParams -join ' ')"
cargo semver-checks @packageParams
LogGroupEnd

if ($LASTEXITCODE -ne 0) {
  LogError "SemVer checks failed"
  exit $LASTEXITCODE
}
