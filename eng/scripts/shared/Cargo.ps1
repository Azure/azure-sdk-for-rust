
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

function Get-ActiveRustToolchain(
  [string]$ExecutePath
) {
  $activeToolchain = (Invoke-LoggedCommand "rustup show active-toolchain" -ExecutePath $ExecutePath | Select-Object -First 1).Trim()
  if (!$activeToolchain) {
    throw "Failed to determine the active Rust toolchain."
  }

  return ($activeToolchain -split '\s+')[0]
}

function Get-ResolvedRustToolchain(
  [string]$Toolchain = 'active',
  [string]$ExecutePath
) {
  if ($Toolchain -eq 'active') {
    return Get-ActiveRustToolchain -ExecutePath $ExecutePath
  }

  return [Channels]::Resolve($Toolchain)
}

function Test-IsNightlyRustToolchain(
  [string]$Toolchain = 'active',
  [string]$ExecutePath
) {
  return (Get-ResolvedRustToolchain -Toolchain $Toolchain -ExecutePath $ExecutePath) -match '^nightly(?:$|[-])'
}

function Get-CargoMetadata() {
  cargo metadata --no-deps --format-version 1 --manifest-path "$RepoRoot/Cargo.toml" | ConvertFrom-Json -Depth 100 -AsHashtable
}

function Test-ShouldPackDependency(
  $dependency,
  $dependencyPackage
) {
  if (!$dependency['path'] -or !$dependencyPackage) {
    return $false
  }

  if ($dependency['kind'] -ne 'dev') {
    return $true
  }

  # `cargo package` verification can resolve publishable workspace
  # dev-dependencies from crates.io after rewriting path dependencies. Pack
  # them alongside the requested crate, but skip helper crates that set
  # `publish = false` because they cannot be packaged for upload anyway.
  return $null -eq $dependencyPackage.publish
}

function Get-CargoPackages() {
  $metadata = Get-CargoMetadata

  # Path based dependencies are assumed to be unreleased package versions. In
  # non-release builds these should be packed as well.
  foreach ($package in $metadata.packages) {
    $package.UnreleasedDependencies = @()
    foreach ($dependency in $package.dependencies) {
      $dependencyPackage = $metadata.packages | Where-Object -Property name -EQ -Value $dependency.name | Select-Object -First 1
      if (Test-ShouldPackDependency $dependency $dependencyPackage) {
        $package.UnreleasedDependencies += $dependencyPackage
      }
    }
  }

  return $metadata.packages
}

function Get-PackagesFromPackageInfo($packageInfoDirectory) {
  $packages = @()
  $packageInfoFiles = Get-ChildItem -Path $packageInfoDirectory -Filter '*.json' -File
  foreach ($packageInfoFile in $packageInfoFiles) {
    $packageInfo = Get-Content -Path $packageInfoFile.FullName | ConvertFrom-Json
    $packages += $packageInfo
  }

  return $packages
}

function Get-PackageNamesFromPackageInfo($packageInfoDirectory) {
  $packages = Get-PackagesFromPackageInfo($packageInfoDirectory)
  $packages.name
}

function Get-VersionParamsFromCgManifest(
  $packageName,
  $cgManifestPath = ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', 'cgmanifest.json'))
) {
  $cgManifest = Get-Content $cgManifestPath `
  | ConvertFrom-Json
  $components = $cgManifest.
  registrations.
  Where({ $_.component.type -eq 'cargo' -and $_.component.cargo.name -eq $packageName })
  if (!$components) {
    Write-Error "Component '$packageName' not found in cgmanifest.json"
  }

  $versions = $components.component.cargo.version
  if (!$versions) {
    Write-Error "No versions found for package '$packageName' in cgmanifest.json"
  }

  if ($versions -is [Array] -and $versions.Count -ne 1) {
    Write-Error "Multiple versions found for package '$packageName' in cgmanifest.json"
  }

  return @('--version', $versions)
}
