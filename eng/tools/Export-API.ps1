#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = 'PackageInfo')]
param(
  [Parameter(ParameterSetName = 'PackageInfo')]
  [Parameter(ParameterSetName = 'CheckPackageInfo')]
  [string] $PackageInfoDirectory,

  [Parameter(Position = 0, ParameterSetName = 'PackageName')]
  [Parameter(Position = 0, ParameterSetName = 'CheckPackageName')]
  [ValidateNotNullOrEmpty()]
  [Alias('PackageNames')]
  [string[]] $PackageName,

  [Parameter(ParameterSetName = 'ManifestDir')]
  [Parameter(ParameterSetName = 'CheckManifestDir')]
  [string[]] $ManifestDir,

  [Parameter(Mandatory, ParameterSetName = 'CheckWorkspace')]
  [Parameter(Mandatory, ParameterSetName = 'CheckPackageInfo')]
  [Parameter(Mandatory, ParameterSetName = 'CheckPackageName')]
  [Parameter(Mandatory, ParameterSetName = 'CheckManifestDir')]
  [switch] $Check,

  [Parameter(ParameterSetName = 'PackageInfo')]
  [Parameter(ParameterSetName = 'PackageName')]
  [Parameter(ParameterSetName = 'ManifestDir')]
  [switch] $IncludeComments,

  [Parameter(ParameterSetName = 'PackageInfo')]
  [Parameter(ParameterSetName = 'PackageName')]
  [Parameter(ParameterSetName = 'ManifestDir')]
  [switch] $IncludeSourceMap
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'shared', 'common.ps1'))

function Get-PackagesToExport(
  [string] $SelectedPackageInfoDirectory
) {
  return Get-CargoSelectedPackages `
    -PackageName $PackageName `
    -ManifestDir $ManifestDir `
    -PackageInfoDirectory $SelectedPackageInfoDirectory
}

function Get-GenerateApiArguments(
  $Package
) {
  $packageDirectory = Split-Path -Path $Package.manifest_path -Parent
  $outputDirectory = [System.IO.Path]::Combine($packageDirectory, 'api')
  $arguments = @(
    'run',
    '--manifest-path',
    ([System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'generate_api', 'Cargo.toml')),
    '--',
    '--manifest-path',
    $Package.manifest_path,
    '--output',
    $outputDirectory
  )

  if ($Check) {
    return $arguments + @('--no-docs', '--no-map', '--check')
  }

  if (!$IncludeComments) {
    $arguments += '--no-docs'
  }

  if (!$IncludeSourceMap) {
    $arguments += '--no-map'
  }

  return $arguments
}

function Get-RegenerateCommand(
  $Package
) {
  $packageDirectory = Split-Path -Path $Package.manifest_path -Parent
  $outputDirectory = [System.IO.Path]::Combine($packageDirectory, 'api')
  $generateApiManifestPath = [System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'generate_api', 'Cargo.toml')
  return 'cargo run --manifest-path "{0}" -- --manifest-path "{1}" --output "{2}" --no-docs --no-map' -f `
    $generateApiManifestPath, $Package.manifest_path, $outputDirectory
}

$packageInfoPath = $PackageInfoDirectory
if ($PackageInfoDirectory -and !(Test-Path -Path $PackageInfoDirectory -PathType Container)) {
  $packageInfoPath = $null
}

$packages = Get-PackagesToExport -SelectedPackageInfoDirectory $packageInfoPath

foreach ($package in $packages) {
  Write-Host "$($Check ? 'Checking' : 'Exporting') API files for '$($package.name)'"
  $process = Start-PipedProcess `
    -FilePath 'cargo' `
    -ArgumentList (Get-GenerateApiArguments -Package $package) `
    -WorkingDirectory $RepoRoot `
    -GroupOutput `
    -DoNotExitOnFailedExitCode
  if ($process.ExitCode) {
    if ($Check) {
      LogError @"
API files are out of date for '$($package.name)'.

Regenerate them locally with:
    $(Get-RegenerateCommand -Package $package)

Then add api/API.md and api/API.metadata.yml in a new commit to this pull request.
"@
    }

    exit $process.ExitCode
  }
}
