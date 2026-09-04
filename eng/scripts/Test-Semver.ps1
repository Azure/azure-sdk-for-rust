#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
  [Parameter(ParameterSetName = 'Named')]
  [Alias('PackageNames')]
  [string[]]$PackageName,
  [Parameter(ParameterSetName = 'ManifestDir')]
  [string[]]$ManifestDir,
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string]$PackageInfoDirectory,
  [string]$Toolchain = 'stable',
  [switch]$IgnoreCgManifestVersion
)

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'common.ps1'))

$resolvedToolchain = Get-ResolvedRustToolchain -Toolchain $Toolchain

function Get-OutputPackages($workspacePackages) {
  switch ($PSCmdlet.ParameterSetName) {
    'Named' {
      Write-Verbose 'Getting named packages from workspace'
      return Get-CargoSelectedPackages -PackageName $PackageName
    }

    'ManifestDir' {
      Write-Verbose 'Getting packages from manifest directories'
      return Get-CargoSelectedPackages -ManifestDir $ManifestDir
    }

    'PackageInfo' {
      Write-Verbose "Getting packages from $PackageInfoDirectory"
      return Get-CargoSelectedPackages -PackageInfoDirectory $PackageInfoDirectory
    }

    default {
      Write-Verbose 'Getting all workspace packages'
      return $workspacePackages
    }
  }
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

$materializedBaselineCommits = @{}

function Initialize-BaselineRevision($baselineRevision) {
  $commit = Invoke-LoggedCommand "git rev-parse '$baselineRevision^{commit}'" -ExecutePath $RepoRoot
  if ($materializedBaselineCommits.ContainsKey($commit)) {
    return
  }

  $tempDirectory = if ($env:AGENT_TEMPDIRECTORY) { $env:AGENT_TEMPDIRECTORY } else { [System.IO.Path]::GetTempPath() }
  $archivePath = [System.IO.Path]::Combine($tempDirectory, [System.IO.Path]::GetRandomFileName())
  try {
    # Sparse CI clones omit historical objects, which cargo-semver-checks cannot fetch from its local clone.
    Invoke-LoggedCommand "git archive --format=tar --output='$archivePath' '$baselineRevision'" -ExecutePath $RepoRoot -GroupOutput
    $materializedBaselineCommits[$commit] = $true
  }
  finally {
    Remove-Item $archivePath -Force -ErrorAction SilentlyContinue
  }
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

  # BUG: Skip checking Key Vault secrets and certificates because of a symlink until a fix is merged upstream (https://github.com/Azure/azure-sdk-for-rust/issues/5143).
  if ($packageName -in @('azure_security_keyvault_secrets', 'azure_security_keyvault_certificates')) {
    LogWarning "Skipping $packageName due to a bug in ``cargo-semver-checks`` (https://github.com/Azure/azure-sdk-for-rust/issues/5143)"
    continue
  }

  $manifestPath = $package.manifest_path
  $baselineRevision = Get-BaselineRevision $package
  if (!$baselineRevision) {
    LogWarning "$packageName has not been published yet and will be ignored"
    continue
  }

  Initialize-BaselineRevision $baselineRevision
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
