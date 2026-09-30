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

  [string] $WorkspacePath,

  [ValidateNotNullOrEmpty()]
  [string] $ToolPath,

  [ValidateNotNullOrEmpty()]
  [string] $RustToolchain = 'nightly',

  [switch] $Check,

  [switch] $Review,

  [Parameter(ParameterSetName = 'PackageName')]
  [Parameter(ParameterSetName = 'ManifestDir')]
  [ValidateNotNullOrEmpty()]
  [string] $OutputDir,

  [Parameter(ParameterSetName = 'PackageName')]
  [Parameter(ParameterSetName = 'ManifestDir')]
  [ValidateNotNullOrEmpty()]
  [string] $WorkingDir
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'shared', 'common.ps1'))
. ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'shared', 'ApiReviewHub.ps1'))

function Get-OutputDir(
  $Package,
  [string] $SelectedOutputDir
) {
  if ($SelectedOutputDir) {
    return $SelectedOutputDir
  }

  return Split-Path -Path $Package.manifest_path -Parent
}

function Get-RepoRelativePath(
  [string] $Path
) {
  return [System.IO.Path]::GetRelativePath($RepoRoot, $Path).Replace('\', '/')
}

function Get-MissingRequiredApiFiles(
  [string] $SelectedOutputDir
) {
  $requiredFiles = @('api.md')
  return @(
    foreach ($fileName in $requiredFiles) {
      $path = [System.IO.Path]::Combine($SelectedOutputDir, $fileName)
      if (!(Test-Path -Path $path -PathType Leaf)) {
        $fileName
      }
    }
  )
}

function Get-GenerateApiArguments(
  $Package,
  [string] $SourceDir,
  [string] $SelectedOutputDir,
  [string] $SelectedWorkingDir
) {
  $arguments = @(
    '--root',
    $SourceDir,
    '--manifest-path',
    $Package.manifest_path
  )

  if ($Check) {
    $arguments += '--check'
  }

  if ($SelectedOutputDir) {
    $arguments += @('--output-dir', (Get-OutputDir -Package $Package -SelectedOutputDir $SelectedOutputDir))
  }

  if ($SelectedWorkingDir) {
    $arguments += @('--working-dir', $SelectedWorkingDir)
  }

  if ($Review) {
    $arguments += '--review'
  }

  return $arguments
}

function Get-GenerateApiInvocation(
  $Package,
  [string] $SourceDir,
  [string] $ResolvedToolPath,
  [string] $FallbackRustToolchain,
  [string] $SelectedOutputDir,
  [string] $SelectedWorkingDir
) {
  $arguments = Get-GenerateApiArguments `
    -Package $Package `
    -SourceDir $SourceDir `
    -SelectedOutputDir $SelectedOutputDir `
    -SelectedWorkingDir $SelectedWorkingDir
  if ($ResolvedToolPath) {
    return [PSCustomObject]@{
      FilePath     = $ResolvedToolPath
      ArgumentList = $arguments
    }
  }

  $cargoArguments = @()
  if ($FallbackRustToolchain) {
    $cargoArguments += "+$FallbackRustToolchain"
  }

  $cargoArguments += @(
    'run',
    '--manifest-path',
    ([System.IO.Path]::Combine($SourceDir, 'eng', 'tools', 'Cargo.toml')),
    '-p',
    'generate_api',
    '--'
  ) + $arguments

  return [PSCustomObject]@{
    FilePath     = 'cargo'
    ArgumentList = $cargoArguments
  }
}

function Get-RegenerateCommand(
  $Package,
  [string] $SelectedOutputDir,
  [string] $SelectedWorkingDir
) {
  $generateApiManifestPath = [System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'generate_api', 'Cargo.toml')
  $command = 'cargo run --manifest-path "{0}" -- --root "{1}" --manifest-path "{2}"' -f `
  (Get-RepoRelativePath -Path $generateApiManifestPath),
  $RepoRoot,
  (Get-RepoRelativePath -Path $Package.manifest_path)
  if ($SelectedOutputDir) {
    $command += ' --output-dir "{0}"' -f (Get-OutputDir -Package $Package -SelectedOutputDir $SelectedOutputDir)
  }

  if ($SelectedWorkingDir) {
    $command += ' --working-dir "{0}"' -f $SelectedWorkingDir
  }

  if ($Review) {
    $command += ' --review'
  }

  return $command
}

if ($OutputDir -and @($PackageName).Count -gt 1) {
  LogError '-OutputDir can only be used with a single -PackageName value.'
  exit 1
}

if ($OutputDir -and @($ManifestDir).Count -gt 1) {
  LogError '-OutputDir can only be used with a single -ManifestDir value.'
  exit 1
}

if ([string]::IsNullOrWhiteSpace($WorkspacePath)) {
  $WorkspacePath = ([System.IO.Path]::Combine($RepoRoot, 'Cargo.toml'))
}

$workspace = Get-CargoWorkspaceInfo -WorkspacePath $WorkspacePath
$sourceDir = $workspace.WorkspaceDir

$packageInfoDir = $PackageInfoDirectory
if ($PackageInfoDirectory -and !(Test-Path -Path $PackageInfoDirectory -PathType Container)) {
  $packageInfoDir = $null
}

$packages = Get-CargoSelectedPackages `
  -PackageName $PackageName `
  -ManifestDir $ManifestDir `
  -PackageInfoDirectory $packageInfoDir `
  -WorkspacePath $workspace.ManifestPath

$resolvedOutputDir = if ($OutputDir) {
  [System.IO.Path]::Combine($PWD.Path, $OutputDir)
}
$resolvedWorkingDir = if ($WorkingDir) {
  [System.IO.Path]::Combine($PWD.Path, $WorkingDir)
}

$resolvedToolPath = if ($ToolPath) { [System.IO.Path]::GetFullPath($ToolPath) } else { $null }
$fallbackRustToolchain = $null
if ($resolvedToolPath -and !(Test-Path -Path $resolvedToolPath -PathType Leaf)) {
  LogWarning "Expected generate_api executable at '$resolvedToolPath'. Falling back to 'cargo run'."
  $resolvedToolPath = $null
  $fallbackRustToolchain = Resolve-ApiReviewHubRustToolchain -Toolchain $RustToolchain -ExecuteDir $sourceDir
}

foreach ($package in $packages) {
  if (!(Test-CargoPackagePublishable $package)) {
    LogWarning "Skipping API files for non-publishable package '$($package.name)'."
    continue
  }

  Write-Host "$($Check ? 'Checking' : 'Exporting') API files for '$($package.name)'"
  if ($Check) {
    $packageOutputDir = Get-OutputDir -Package $package -SelectedOutputDir $resolvedOutputDir
    $missingFiles = Get-MissingRequiredApiFiles -SelectedOutputDir $packageOutputDir
    if ($missingFiles) {
      LogError @"
API files are missing for '$($package.name)': $($missingFiles -join ', ').

Regenerate them locally with:
    $(Get-RegenerateCommand -Package $package -SelectedOutputDir $resolvedOutputDir -SelectedWorkingDir $resolvedWorkingDir)

Then add api.md in a new commit to this pull request.
"@
      exit 1
    }
  }

  $invocation = Get-GenerateApiInvocation `
    -Package $package `
    -SourceDir $sourceDir `
    -ResolvedToolPath $resolvedToolPath `
    -FallbackRustToolchain $fallbackRustToolchain `
    -SelectedOutputDir $resolvedOutputDir `
    -SelectedWorkingDir $resolvedWorkingDir
  $process = Start-PipedProcess `
    -FilePath $invocation.FilePath `
    -ArgumentList $invocation.ArgumentList `
    -WorkingDirectory $sourceDir `
    -GroupOutput `
    -DoNotExitOnFailedExitCode
  if ($process.ExitCode) {
    if ($Check) {
      LogError @"
API files are out of date for '$($package.name)'.

Regenerate them locally with:
    $(Get-RegenerateCommand -Package $package -SelectedOutputDir $resolvedOutputDir -SelectedWorkingDir $resolvedWorkingDir)

Then add api.md in a new commit to this pull request.
"@
    }

    exit $process.ExitCode
  }
}
