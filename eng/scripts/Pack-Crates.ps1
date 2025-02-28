#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$OutputPath,
  [Parameter(ParameterSetName = 'Named')]
  [string[]]$PackageNames,
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string]$PackageInfoDirectory
)

$ErrorActionPreference = 'Stop'

. (Join-Path $PSScriptRoot '..' 'common' 'scripts' 'common.ps1')

if ($OutputPath) {
  $OutputPath = New-Item -ItemType Directory -Path $OutputPath -Force | Select-Object -ExpandProperty FullName
}

function Get-OutputPackageNames($workspacePackageNames) {
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
      return $workspacePackageNames
    }
  }

  foreach ($name in $names) {
    if (-not $workspacePackageNames.Contains($name)) {
      Write-Error "Package '$name' is not in the workspace"
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
  $packages = Get-CargoPackages
  $outputPackageNames = Get-OutputPackageNames $packages.name

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

function Initialize-VendorDirectory() {
  $path = "$RepoRoot/target/vendor"
  Invoke-LoggedCommand "cargo vendor $path" -GroupOutput | Out-Host
  return $path
}

function Add-CrateToLocalRegistry($LocalRegistryPath, $Package) {
  $packageName = $Package.name
  $packageVersion = $Package.version

  # create an index entry for the package
  $packagePath = "$RepoRoot/target/package/$packageName-$packageVersion"

  Write-Host "Copying package '$packageName' to '$destination'"
  Copy-Item -Path $packagePath -Destination $LocalRegistryPath -Recurse

  #write an empty checksum file
  '{"files":{}}' | Out-File -FilePath "$LocalRegistryPath/$packageName-$packageVersion/.cargo-checksum.json" -Encoding utf8
}

# For all dependencies with paths, but no versions, add the version from the path
function Add-PathVersions($packages) {
  # Install PSToml if it's not already installed
  if (-not (PowerShellGet\Get-InstalledModule -Name PSToml -ErrorAction SilentlyContinue)) {
    PowerShellGet\Install-Module -Name PSToml -Scope CurrentUser -Force
  }

  foreach ($package in $packages) {
    $dirty = $false
    $toml = Get-Content -Path $Package.manifest_path -Raw | ConvertFrom-Toml

    foreach ($name in $toml.dependencies.Keys) {
      # we want to look at the dependency as it was resolved by `cargo metadata`
      # this will resolve workspace depdencies, but retain their path/no-version state
      $dependency = $package.dependencies | Where-Object -Property name -EQ -Value $name | Select-Object -First 1
      # If the dependency is a path dependency, set the version to the version of the package in the workspace
      if ($dependency.path -and !$dependency.version) {
        $tomlDependency = $toml.dependencies.$name
        $dependencyVersion = $packages | Where-Object -Property name -EQ -Value $name | Select-Object -ExpandProperty version -First 1

        $tomlDependency.version = $dependencyVersion
        $dirty = $true
      }
    }
    if ($dirty) {
      $toml | ConvertTo-Toml -Depth 10 | Set-Content -Path $Package.manifest_path -Encoding utf8
    }
  }
}

Push-Location $RepoRoot
try {
  Write-Host "Setting RUSTFLAGS to '-Dwarnings'"
  $env:RUSTFLAGS = "-Dwarnings"

  $localRegistryPath = Initialize-VendorDirectory

  [array]$packages = Get-PackagesToBuild

  Add-PathVersions $packages

  Write-Host "Building packages in the following order:"
  foreach ($package in $packages) {
    $packageName = $package.name
    $type = if ($package.OutputPackage) { "output" } else { "dependency" }
    Write-Host "  $packageName ($type)"
  }
  Write-Host ""

  foreach ($package in $packages) {
    $packageName = $package.name
    $packageVersion = $package.version

    Invoke-LoggedCommand `
      -GroupOutput `
      -Command "cargo publish --dry-run --package $packageName --registry crates-io --config `"source.crates-io.replace-with='local'`" --config `"source.local.directory='$localRegistryPath'`" --allow-dirty"

    # copy the package to the local registry
    Add-CrateToLocalRegistry `
      -LocalRegistryPath $localRegistryPath `
      -Package $package

    if ($OutputPath -and $package.OutputPackage) {
      $packageOutputPath = "$OutputPath/$packageName"
      $targetPackagePath = "$RepoRoot/target/package/$packageName-$packageVersion"

      if (Test-Path -Path $packageOutputPath) {
        Remove-Item -Path $packageOutputPath -Recurse -Force
      }

      Write-Host "Copying package '$packageName' to '$packageOutputPath'"
      New-Item -ItemType Directory -Path $packageOutputPath -Force | Out-Null
      Copy-Item -Path $targetPackagePath/* -Destination $packageOutputPath -Recurse -Exclude "Cargo.toml.orig"
    }
  }

  Write-Host "Removing local registry"
  Remove-Item -Path $localRegistryPath -Recurse -Force | Out-Null
}
finally {
  Pop-Location
}
