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

$packageExemptions = @{
  # cargo-semver-checks resolves the latest compatible transitive dependencies in generated
  # packages. The current dependency is fixed in fe2o3-amqp 0.18.3, but the released baseline
  # still needs the 0.14 backport: https://github.com/Azure/azure-sdk-for-rust/issues/5451.
  azure_core_amqp = 'its baseline fe2o3-amqp dependency does not compile with the resolved tokio-util version'
  azure_messaging_eventhubs = 'its fe2o3-amqp dependency does not compile with the resolved tokio-util version'
  azure_messaging_eventhubs_checkpointstore_blob = 'its fe2o3-amqp dependency does not compile with the resolved tokio-util version'

  # cargo-semver-checks cannot process these packages because their baselines contain a symlink.
  azure_security_keyvault_certificates = 'of https://github.com/Azure/azure-sdk-for-rust/issues/5143'
  azure_security_keyvault_secrets = 'of https://github.com/Azure/azure-sdk-for-rust/issues/5143'
}

$finalExitCode = 0
foreach ($package in $outputPackages) {
  $currentPackageName = $package.name

  if ($packageExemptions.ContainsKey($currentPackageName)) {
    LogWarning "Skipping $currentPackageName because $($packageExemptions[$currentPackageName])"
    continue
  }

  $manifestPath = $package.manifest_path
  $baselineRevision = Get-BaselineRevision $package
  if (!$baselineRevision) {
    LogWarning "$currentPackageName has not been published yet and will be ignored"
    continue
  }

  Initialize-BaselineRevision $baselineRevision
  $output = Invoke-LoggedCommand "cargo +$resolvedToolchain semver-checks --manifest-path '$manifestPath' --baseline-rev '$baselineRevision'" -DoNotExitOnFailedExitCode -GroupOutput 2>&1
  if ($output -match 'error: no library targets found in package `(?<name>[\w_]+)`' -and $Matches['name'] -eq $currentPackageName) {
    LogWarning "$currentPackageName base version is a placeholder and will be ignored"
    continue
  }

  if ($output -match 'error: no crates with library targets selected') {
    LogWarning "$currentPackageName is not a lib crate and will be ignored"
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
