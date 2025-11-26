#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
  [Parameter(ParameterSetName = 'Named')]
  [string[]]$PackageNames,
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string]$PackageInfoDirectory,
  [switch]$IgnoreCgManfiestVersion
)

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

function Get-OutputPackageNames($workspacePackages) {
  $packablePackages = $workspacePackages | Where-Object -Property publish -NE -Value @()
  $packablePackageNames = $packablePackages.name

  $names = @()
  switch ($PsCmdlet.ParameterSetName) {
    'Named' {
      $names = $PackageNames
    }

    'PackageInfo' {
      $names = Get-PackageNamesFromPackageInfo $PackageInfoDirectory
    }

    default {
      return $packablePackageNames
    }
  }

  foreach ($name in $names) {
    if (-not $packablePackageNames.Contains($name)) {
      Write-Error "Package '$name' is not in the workspace or does not publish"
      exit 1
    }
  }

  return $names
}

$packages = Get-CargoPackages
$outputPackageNames = Get-OutputPackageNames $packages

# Read version from cgmanifest.json. If ignored the currently installed or
# "latest" version is used.
$versionParams = @()
if (!$IgnoreCgManfiestVersion) {
  $versionParams += '--version'
  $cgManfiest = Get-Content ([System.IO.Path]::Combine($PSScriptRoot, '..', 'cgmanifest.json')) `
  | ConvertFrom-Json
  $versionParams += $cgManfiest.
  registrations.
  Where({ $_.component.type -eq 'cargo' -and $_.component.cargo.name -eq 'cargo-semver-checks' }).
  component.cargo.version
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
