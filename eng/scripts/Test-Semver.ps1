#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
  [Parameter(ParameterSetName = 'Named')]
  [string[]]$PackageNames,
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string]$PackageInfoDirectory,
  [string]$Toolchain = 'stable',
  [switch]$IgnoreCgManifestVersion
)

Write-Host '[SKIP] Skipping semver-checks for now; see https://github.com/Azure/azure-sdk-for-rust/issues/4585'
exit 0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

$resolvedToolchain = Get-ResolvedRustToolchain -Toolchain $Toolchain

function Get-OutputPackages($workspacePackages) {
  $packages = @()
  switch ($PsCmdlet.ParameterSetName) {
    'Named' {
      Write-Verbose 'Getting named packages from workspace'
      $packages = $workspacePackages.Where({ $_.name -in $PackageNames })
    }

    'PackageInfo' {
      Write-Verbose "Getting packages from $PackageInfoDirectory"
      $packages = Get-PackagesFromPackageInfo $PackageInfoDirectory | ForEach-Object {
        [pscustomobject] @{
          name = $_.Name
          manifest_path = [System.IO.Path]::Combine($_.DirectoryPath, 'Cargo.toml')
        }
      }
    }

    default {
      Write-Verbose 'Getting all workspace packages'
      return $workspacePackages

    }
  }

  Write-Verbose "Packages: $($packages.name -join ', ')"
  foreach ($name in $packages.name) {
    if (-not $workspacePackages.name.Contains($name)) {
      Write-Error "Package '$name' is not in the workspace or does not publish"
      exit 1
    }
  }

  return $packages
}

$packages = Get-CargoPackages
$outputPackages = Get-OutputPackages $packages

$versionParams = @()
if (!$IgnoreCgManifestVersion) {
  $versionParams = Get-VersionParamsFromCgManifest cargo-semver-checks
}

Invoke-LoggedCommand "cargo install cargo-semver-checks --locked $($versionParams -join ' ')" -GroupOutput

$finalExitCode = 0
foreach ($package in $outputPackages) {
  $packageName = $package.name
  $manifestPath = $package.manifest_path
  $output = Invoke-LoggedCommand "cargo +$resolvedToolchain semver-checks --manifest-path $manifestPath" -DoNotExitOnFailedExitCode -GroupOutput 2>&1
  if ($output -match 'error: no library targets found in package `(?<name>[\w_]+)`' -and $Matches['name'] -eq $packageName) {
    LogWarning "$packageName base version is a placeholder and will be ignored"
    continue
  }

  if ($output -match 'error: no crates with library targets selected') {
    LogWarning "$packageName is not a lib crate and will be ignored"
    continue
  }

  if ($output -match 'not found in registry') {
    LogWarning "$packageName has not been published yet and will be ignored"
    continue
  }

  $finalExitCode = $finalExitCode -bor $LASTEXITCODE
  $output | Write-Host
}

if ($finalExitCode) {
  LogError "SemVer checks failed"
  exit $finalExitCode
}

# Explicitly return 0, to clear LASTEXITCODE in case there were any failures that were ignored due to the above conditions
exit 0
