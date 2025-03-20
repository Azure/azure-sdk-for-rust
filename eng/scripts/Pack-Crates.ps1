#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
  [string]$OutputPath,
  [Parameter(ParameterSetName = 'Named')]
  [string[]]$PackageNames,
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string]$PackageInfoDirectory,
  [switch]$Verify
)

$ErrorActionPreference = 'Stop'

. (Join-Path $PSScriptRoot '..' 'common' 'scripts' 'common.ps1')

Write-Host @"
Packing crates with
    RUSTFLAGS: '${env:RUSTFLAGS}'
"@

if ($OutputPath) {
  $OutputPath = New-Item -ItemType Directory -Path $OutputPath -Force | Select-Object -ExpandProperty FullName
}

function Get-OutputPackageNames($workspacePackages) {
  $packablePackages = $workspacePackages | Where-Object -Property publish -NE -Value @()
  $packablePackageNames = $packablePackages.name

  $names = @()
  switch ($PsCmdlet.ParameterSetName) {
    'Named' {
      $names = $PackageNames
    }

    'PackageInfo' {
      $packageInfoFiles = Get-ChildItem -Path $PackageInfoDirectory -Filter '*.json' -File
      foreach ($packageInfoFile in $packageInfoFiles) {
        $packageInfo = Get-Content -Path $packageInfoFile.FullName | ConvertFrom-Json
        $names += $packageInfo.name
      }
    }

    default {
      return $packablePackageNames
    }
  }

  foreach ($name in $names) {
    if (-not $packablePackageNames.Contains($name)) {
      Write-Error "Package '$name' is not in the workspace or does not publish"
      exit 1
    }
  }

  return $names
}

function Get-CargoMetadata() {
  cargo metadata --no-deps --format-version 1 --manifest-path "$RepoRoot/Cargo.toml" | ConvertFrom-Json -Depth 100 -AsHashtable
}

function Get-CargoPackages() {
  $metadata = Get-CargoMetadata

  # path based depdenencies are assumed to be unreleased package versions
  # they must be included in this build and build before packages that depend on them
  foreach ($package in $metadata.packages) {
    $package.UnreleasedDependencies = @()
    foreach ($dependency in $package.dependencies) {
      if ($dependency.path -and $dependency.kind -ne 'dev') {
        $dependencyPackage = $metadata.packages | Where-Object -Property name -EQ -Value $dependency.name | Select-Object -First 1
        $package.UnreleasedDependencies += $dependencyPackage
      }
    }
  }

  return $metadata.packages
}

function Get-PackagesToBuild() {
  [array]$packages = Get-CargoPackages
  [array]$outputPackageNames = Get-OutputPackageNames $packages

  # We start with output packages, then recursively add unreleased dependencies to the list of packages that need to be built
  [array]$packagesToBuild = $packages | Where-Object { $outputPackageNames.Contains($_.name) }

  $toProcess = $packagesToBuild
  while ($toProcess.Length -gt 0) {
    $package = $toProcess[0]
    $toProcess = $toProcess -ne $package

    foreach ($dependency in $package.UnreleasedDependencies) {
      if (!$packagesToBuild.Contains($dependency) -and !$toProcess.Contains($dependency)) {
        $packagesToBuild += $dependency
        $toProcess += $dependency
      }
    }
  }

  $buildOrder = @()

  # Then we order the packages to that dependencies are built first
  while ($packagesToBuild.Count -gt 0) {
    # Pick any package with no unreleased dependencies, add it to the build order and remove it from the list of other packages' unreleased dependencies
    $package = $packagesToBuild | Where-Object { $_.UnreleasedDependencies.Count -eq 0 } | Select-Object -First 1

    if (-not $package) {
      Write-Error "These packages cannot be built because they depend on unreleased dependencies that aren't being built." -ErrorAction Continue
      foreach ($package in $packagesToBuild) {
        Write-Error "  $($package.name) -> $($package.UnreleasedDependencies -join ', ')" -ErrorAction Continue
      }
      exit 1
    }

    $package.OutputPackage = $outputPackageNames.Contains($package.name)
    $buildOrder += $package
    $packagesToBuild = @($packagesToBuild -ne $package)

    foreach ($otherPackage in $packagesToBuild) {
      $otherPackage.UnreleasedDependencies = $otherPackage.UnreleasedDependencies -ne $package
    }
  }

  return $buildOrder
}

Push-Location $RepoRoot
try {
  [array]$packages = Get-PackagesToBuild

  $command = "cargo +nightly -Zpackage-workspace package --allow-dirty"

  Write-Host "Building packages:"
  foreach ($package in $packages) {
    $packageName = $package.name
    $type = if ($package.OutputPackage) { "output" } else { "dependency" }
    Write-Host "  $packageName ($type)"
    $command += " --package $packageName"
  }

  if (!$Verify) {
    $command += " --no-verify"
  }

  if ($env:SYSTEM_DEBUG -eq 'true') {
    Write-Host "##[group] $RepoRoot/Cargo.toml.lock"
    Get-Content "$RepoRoot/Cargo.toml.lock"
    Write-Host "##[endgroup]"
  }

  Invoke-LoggedCommand -Command $command -GroupOutput

  if ($env:SYSTEM_DEBUG -eq 'true') {
    Write-Host "##[group] $RepoRoot/Cargo.toml.lock"
    Get-Content "$RepoRoot/Cargo.toml.lock"
    Write-Host "##[endgroup]"
  }

  foreach ($package in $packages) {
    $packageName = $package.name
    $packageVersion = $package.version

    if ($OutputPath -and $package.OutputPackage) {
      $crateFilePath = "$RepoRoot/target/package/$packageName-$packageVersion.crate"
      $targetDirectory = "$OutputPath/$packageName"

      if (Test-Path $targetDirectory) {
        Write-Host "Removing existing directory '$targetDirectory'"
        Remove-Item -Path $targetDirectory -Recurse -Force
      }

      Write-Host "Copying crate '$crateFilePath' to '$OutputPath'"
      Copy-Item -Path $crateFilePath -Destination $OutputPath -Force

      Write-Host "Exctracting crate '$crateFilePath' to '$targetDirectory'"
      New-Item -ItemType Directory -Path $targetDirectory -Force | Out-Null
      tar -xf $crateFilePath --directory $targetDirectory --strip-components=1

      # Remove Cargo.toml.orig from the extracted directory
      Remove-Item -Path "$targetDirectory/Cargo.toml.orig" -Force -ErrorAction SilentlyContinue
    }
  }
}
finally {
  Pop-Location
}
