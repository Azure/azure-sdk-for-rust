
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

function Get-RustupExecutable() {
  # msrustup manages MS Rust toolchains outside of rustup's registry, so rustup
  # cannot resolve a custom channel like 'ms-prod-1.95' from rust-toolchain.toml.
  # Jobs using MS Rust set RUSTUP_EXE to 'msrustup' to route toolchain queries there.
  if ($env:RUSTUP_EXE) {
    return $env:RUSTUP_EXE
  }

  return 'rustup'
}

function Get-ActiveRustToolchain(
  [string]$ExecutePath
) {
  $rustup = Get-RustupExecutable
  $output = Invoke-LoggedCommand "$rustup show active-toolchain" -ExecutePath $ExecutePath

  # msrustup prefixes its output with INFO lines, so take the first line that
  # actually names a toolchain.
  $activeToolchain = $output
  | Where-Object { $_ -match '\S' -and $_ -notmatch '^\s*(INFO|WARN)\b' }
  | Select-Object -First 1

  if (!$activeToolchain) {
    throw "Failed to determine the active Rust toolchain."
  }

  return ($activeToolchain.Trim() -split '\s+')[0]
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

  # `cargo package` can verify path-only dev-dependencies without packing them
  # as separate crates first, so only non-dev path dependencies need to be
  # expanded into the package set.
  return $dependency['kind'] -ne 'dev'
}

function Get-CargoPackages() {
  $metadata = Get-CargoMetadata

  # Path based non-dev dependencies are assumed to be unreleased package
  # versions. In non-release builds these should be packed as well.
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
