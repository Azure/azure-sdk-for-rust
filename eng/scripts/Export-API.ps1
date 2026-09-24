#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = 'PackageInfo')]
param(
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string] $PackageInfoDirectory,

  [Parameter(Position = 0, ParameterSetName = 'PackageName')]
  [ValidateNotNullOrEmpty()]
  [Alias('PackageNames')]
  [string[]] $PackageName,

  [Parameter(ParameterSetName = 'ManifestDir')]
  [string[]] $ManifestDir,

  [switch] $Check,

  [ValidateNotNullOrEmpty()]
  [string] $OutputPath
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
  if ($OutputPath) {
    if ([System.IO.Path]::IsPathRooted($OutputPath)) {
      return $OutputPath
    }

    return [System.IO.Path]::Combine($PWD.Path, $OutputPath)
  }

  return Split-Path -Path $Package.manifest_path -Parent
}

function Get-RepoRelativePath(
  [string] $Path
) {
  return [System.IO.Path]::GetRelativePath($RepoRoot, $Path).Replace('\', '/')
}

function Get-MissingRequiredApiFiles(
  [string] $OutputDirectory
) {
  $requiredFiles = @('api.md')
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
  $arguments = @(
    'run',
    '--manifest-path',
    ([System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'generate_api', 'Cargo.toml')),
    '--',
    '--manifest-path',
    $Package.manifest_path
  )

  if ($Check) {
    $arguments += '--check'
  }

  if ($OutputPath) {
    $arguments += @('--output', (Get-OutputDirectory -Package $Package))
  }

  return $arguments
}

function Get-RegenerateCommand(
  $Package
) {
  $generateApiManifestPath = [System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'generate_api', 'Cargo.toml')
  $command = 'cargo run --manifest-path "{0}" -- --manifest-path "{1}"' -f `
    (Get-RepoRelativePath -Path $generateApiManifestPath),
    (Get-RepoRelativePath -Path $Package.manifest_path)
  if ($OutputPath) {
    $command += ' --output "{0}"' -f (Get-OutputDirectory -Package $Package)
  }

  return $command
}

$packageInfoPath = $PackageInfoDirectory
if ($PackageInfoDirectory -and !(Test-Path -Path $PackageInfoDirectory -PathType Container)) {
  $packageInfoPath = $null
}

$packages = Get-PackagesToExport -SelectedPackageInfoDirectory $packageInfoPath

foreach ($package in $packages) {
  if (!(Test-CargoPackagePublishable $package)) {
    LogWarning "Skipping API files for non-publishable package '$($package.name)'."
    continue
  }

  Write-Host "$($Check ? 'Checking' : 'Exporting') API files for '$($package.name)'"
  if ($Check) {
    $outputDirectory = Get-OutputDirectory -Package $package
    $missingFiles = Get-MissingRequiredApiFiles -OutputDirectory $outputDirectory
    if ($missingFiles) {
      LogError @"
API files are missing for '$($package.name)': $($missingFiles -join ', ').

Regenerate them locally with:
    $(Get-RegenerateCommand -Package $package)

Then add api.md in a new commit to this pull request.
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

Then add api.md in a new commit to this pull request.
"@
    }

    exit $process.ExitCode
  }
}
