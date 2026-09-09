#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = 'PackageInfo')]
param(
  [Parameter(ParameterSetName = 'PackageInfo')]
  [Parameter(Mandatory, ParameterSetName = 'CheckPackageInfo')]
  [string] $PackageInfoDirectory,

  [Parameter(Position = 0, ParameterSetName = 'PackageName')]
  [Parameter(Mandatory, Position = 0, ParameterSetName = 'CheckPackageName')]
  [ValidateNotNullOrEmpty()]
  [Alias('PackageNames')]
  [string[]] $PackageName,

  [Parameter(ParameterSetName = 'ManifestDir')]
  [Parameter(Mandatory, ParameterSetName = 'CheckManifestDir')]
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

function Get-OutputDirectory(
  $Package
) {
  $packageDirectory = Split-Path -Path $Package.manifest_path -Parent
  return [System.IO.Path]::Combine($packageDirectory, 'api')
}

function Get-RepoRelativePath(
  [string] $Path
) {
  return [System.IO.Path]::GetRelativePath($RepoRoot, $Path).Replace('\', '/')
}

function Get-MissingRequiredApiFiles(
  [string] $OutputDirectory
) {
  $requiredFiles = @('API.md', 'API.metadata.yml')
  return @(
    foreach ($fileName in $requiredFiles) {
      $path = [System.IO.Path]::Combine($OutputDirectory, $fileName)
      if (!(Test-Path -Path $path -PathType Leaf)) {
        $fileName
      }
    }
  )
}

function Get-GenerateApiArguments(
  $Package
) {
  $outputDirectory = Get-OutputDirectory -Package $Package
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
  $outputDirectory = Get-OutputDirectory -Package $Package
  $generateApiManifestPath = [System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'generate_api', 'Cargo.toml')
  return 'cargo run --manifest-path "{0}" -- --manifest-path "{1}" --output "{2}" --no-docs --no-map' -f `
    (Get-RepoRelativePath -Path $generateApiManifestPath),
    (Get-RepoRelativePath -Path $Package.manifest_path),
    (Get-RepoRelativePath -Path $outputDirectory)
}

$packageInfoPath = $PackageInfoDirectory
if ($PackageInfoDirectory -and !(Test-Path -Path $PackageInfoDirectory -PathType Container)) {
  $packageInfoPath = $null
}

$packages = Get-PackagesToExport -SelectedPackageInfoDirectory $packageInfoPath

foreach ($package in $packages) {
  Write-Host "$($Check ? 'Checking' : 'Exporting') API files for '$($package.name)'"
  if ($Check) {
    $outputDirectory = Get-OutputDirectory -Package $package
    $missingFiles = Get-MissingRequiredApiFiles -OutputDirectory $outputDirectory
    if ($missingFiles) {
      LogError @"
API files are missing for '$($package.name)': $($missingFiles -join ', ').

Regenerate them locally with:
    $(Get-RegenerateCommand -Package $package)

Then add api/API.md and api/API.metadata.yml in a new commit to this pull request.
"@
      exit 1
    }
  }

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
