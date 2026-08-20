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

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

$resolvedToolchain = Get-ResolvedRustToolchain -Toolchain $Toolchain

function Get-OutputPackages($workspacePackages) {
  $packages = @()
  $requestedPackageNames = @()
  switch ($PsCmdlet.ParameterSetName) {
    'Named' {
      Write-Verbose 'Getting named packages from workspace'
      $requestedPackageNames = $PackageNames
      $packages = $workspacePackages.Where({ $_.name -in $PackageNames })
    }

    'PackageInfo' {
      Write-Verbose "Getting packages from $PackageInfoDirectory"
      $requestedPackageNames = @(
        Get-PackagesFromPackageInfo $PackageInfoDirectory | Select-Object -ExpandProperty Name
      )
      $packages = $workspacePackages.Where({ $_.name -in $requestedPackageNames })
    }

    default {
      Write-Verbose 'Getting all workspace packages'
      return $workspacePackages

    }
  }

  Write-Verbose "Packages: $($packages.name -join ', ')"
  foreach ($name in $requestedPackageNames) {
    if (-not $workspacePackages.name.Contains($name)) {
      LogError "Package '$name' is not in the workspace or does not publish"
      exit 1
    }
  }

  return $packages
}

function Get-BaselineRevision($package) {
  $prefix = "$($package.name)@"
  $currentVersion = [AzureEngSemanticVersion]::ParseVersionString($package.version)
  if (!$currentVersion) {
    LogError "Package '$($package.name)' has invalid version '$($package.version)'"
    exit 1
  }

  $tags = @(Invoke-LoggedCommand "git tag -l '$prefix*'" -ExecutePath $RepoRoot)
  $latestVersion = $null
  $latestStableVersion = $null

  foreach ($tag in $tags) {
    $version = [AzureEngSemanticVersion]::ParseVersionString($tag.Substring($prefix.Length))
    if (!$version -or $version.CompareTo($currentVersion) -gt 0) {
      continue
    }

    if (!$latestVersion -or $version.CompareTo($latestVersion) -gt 0) {
      $latestVersion = $version
    }

    if (
      !$version.PrereleaseLabel `
        -and (!$latestStableVersion -or $version.CompareTo($latestStableVersion) -gt 0)
    ) {
      $latestStableVersion = $version
    }
  }

  $baselineVersion = if ($latestStableVersion) { $latestStableVersion } else { $latestVersion }
  if ($baselineVersion) {
    return "$prefix$($baselineVersion.RawVersion)"
  }

  return $null
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
  $baselineRevision = Get-BaselineRevision $package
  if (!$baselineRevision) {
    LogWarning "$packageName has not been published yet and will be ignored"
    continue
  }

  $output = Invoke-LoggedCommand "cargo +$resolvedToolchain semver-checks --manifest-path '$manifestPath' --baseline-rev '$baselineRevision'" -DoNotExitOnFailedExitCode -GroupOutput 2>&1
  if ($output -match 'error: no library targets found in package `(?<name>[\w_]+)`' -and $Matches['name'] -eq $packageName) {
    LogWarning "$packageName base version is a placeholder and will be ignored"
    continue
  }

  if ($output -match 'error: no crates with library targets selected') {
    LogWarning "$packageName is not a lib crate and will be ignored"
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
