# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

$script:ApiReviewHubToolchainReleases = @{}

function Get-ApiReviewHubToolInfo(
  [string] $SourceDir,
  [string] $ToolDir
) {
  $fileName = 'generate_api'
  if ($IsWindows) {
    $fileName = "$fileName.exe"
  }

  return [PSCustomObject]@{
    CargoManifestPath = [System.IO.Path]::Combine($SourceDir, 'eng', 'tools', 'Cargo.toml')
    BuiltExecutablePath = [System.IO.Path]::Combine($SourceDir, 'eng', 'tools', 'target', 'debug', $fileName)
    StagedExecutablePath = [System.IO.Path]::Combine($ToolDir, $fileName)
  }
}

function Get-ApiReviewHubRustVersionAliases(
  [string] $RustVersion
) {
  $aliases = @()
  $normalizedVersion = $RustVersion.Trim().ToLower()
  if (!$normalizedVersion) {
    return $aliases
  }

  $aliases += $normalizedVersion
  if ($normalizedVersion -match '^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)(?<suffix>-(?:nightly|beta))$' -and $Matches.patch -eq '0') {
    $aliases += "$($Matches.major).$($Matches.minor)$($Matches.suffix)"
  }

  if ($normalizedVersion -match '^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)$' -and $Matches.patch -eq '0') {
    $aliases += "$($Matches.major).$($Matches.minor)"
  }

  return @($aliases | Select-Object -Unique)
}

function Get-ApiReviewHubRustToolchainRelease(
  [string] $Toolchain,
  [string] $ExecuteDir = $RepoRoot
) {
  if ($script:ApiReviewHubToolchainReleases.ContainsKey($Toolchain)) {
    return $script:ApiReviewHubToolchainReleases[$Toolchain]
  }

  $releaseLine = Invoke-LoggedCommand "rustc +$Toolchain -Vv" -ExecutePath $ExecuteDir |
  Where-Object { $_ -match '^release:\s+' } |
  Select-Object -First 1
  if (!$releaseLine) {
    throw "Failed to determine release for Rust toolchain '$Toolchain'."
  }

  $release = ($releaseLine -replace '^release:\s+', '').Trim()
  $script:ApiReviewHubToolchainReleases[$Toolchain] = $release
  return $release
}

function Test-ApiReviewHubRustVersionMatch(
  [string] $RequestedVersion,
  [string] $ToolchainRelease
) {
  $requestedAliases = Get-ApiReviewHubRustVersionAliases -RustVersion $RequestedVersion
  $releaseAliases = Get-ApiReviewHubRustVersionAliases -RustVersion $ToolchainRelease

  foreach ($requestedAlias in $requestedAliases) {
    if ($releaseAliases -contains $requestedAlias) {
      return $true
    }
  }

  return $false
}

function Resolve-ApiReviewHubRustToolchain(
  [string] $Toolchain = 'nightly',
  [string] $ExecuteDir = $RepoRoot
) {
  if ([string]::IsNullOrWhiteSpace($Toolchain)) {
    $Toolchain = 'nightly'
  }

  $requestedToolchain = $Toolchain.Trim()
  if ($requestedToolchain -in @('active', 'stable', 'nightly', 'msrv')) {
    return Get-ResolvedRustToolchain -Toolchain $requestedToolchain -ExecutePath $ExecuteDir
  }

  $managedToolchains = @(
    (Get-ResolvedRustToolchain -Toolchain 'nightly' -ExecutePath $ExecuteDir),
    (Get-ResolvedRustToolchain -Toolchain 'stable' -ExecutePath $ExecuteDir),
    (Get-ResolvedRustToolchain -Toolchain 'msrv' -ExecutePath $ExecuteDir)
  ) | Select-Object -Unique

  foreach ($managedToolchain in $managedToolchains) {
    $release = Get-ApiReviewHubRustToolchainRelease -Toolchain $managedToolchain -ExecuteDir $ExecuteDir
    if (Test-ApiReviewHubRustVersionMatch -RequestedVersion $requestedToolchain -ToolchainRelease $release) {
      return $managedToolchain
    }
  }

  return $requestedToolchain
}
