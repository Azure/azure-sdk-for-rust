#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
  [string]$OutputPath,
  [string[]]$PackageNames,
  [switch]$RequireDependencies,
  [switch]$NoVerify
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

  # We start with output packages, then recursively add unreleased dependencies to the list of packages that need to be built
  [array]$packagesToBuild = $packages | Where-Object { $PackageNames.Contains($_.name) }

  $toProcess = $packagesToBuild
  while ($toProcess.Length -gt 0) {
    $package = $toProcess[0]
    $toProcess = $toProcess -ne $package

    foreach ($dependency in $package.UnreleasedDependencies) {
      if (!$packagesToBuild.Contains($dependency) -and !$toProcess.Contains($dependency)) {
        if ($RequireDependencies -and $dependency.name -notin $PackageNames) { 
          Write-Warning "Package $($package.name) depends on unreleased or unspecified dependency: $($dependency.name)"
        }
        
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

    $package.OutputPackage = $PackageNames.Contains($package.name)
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

  Write-Host "Copying package '$packageName' to vendor directory '$LocalRegistryPath'"
  Copy-Item -Path $packagePath -Destination $LocalRegistryPath -Recurse

  #write an empty checksum file
  '{"files":{}}' | Out-File -FilePath "$LocalRegistryPath/$packageName-$packageVersion/.cargo-checksum.json" -Encoding utf8
}

function Create-ApiViewFile($package) {
  $packageName = $package.name
  $command = "cargo run --manifest-path $RepoRoot/eng/tools/generate_api_report/Cargo.toml -- --package $packageName"
  Invoke-LoggedCommand $command -GroupOutput | Out-Host

  $packagePath = Split-Path -Path $package.manifest_path -Parent

  "$packagePath/review/$packageName.rust.json"
}

Push-Location $RepoRoot
try {
  $localRegistryPath = Initialize-VendorDirectory

  [array]$packages = Get-PackagesToBuild

  if ($RequireDependencies) { 
    $unspecifiedPackages = $packages.name | Where-Object { $_ -notin $PackageNames }
    if ($unspecifiedPackages.Count -gt 0) { 
      Write-Error "Packages in -PackageNames require dependencies that are either not released or not listed for packing: $($unspecifiedPackages -join ', ')"
      exit 1
    }
  }

  Write-Host "Building packages in the following order:"
  foreach ($package in $packages) {
    $packageName = $package.name
    $type = if ($package.OutputPackage) { "output" } else { "dependency" }
    Write-Host "  $packageName ($type)"
  }

  foreach ($package in $packages) {
    Write-Host ""

    $packageName = $package.name
    $packageVersion = $package.version

    $command = "cargo publish --locked --dry-run --package $packageName --registry crates-io --config `"source.crates-io.replace-with='local'`" --config `"source.local.directory='$localRegistryPath'`" --allow-dirty"

    if ($NoVerify) {
      $command += " --no-verify"
    }

    Invoke-LoggedCommand -Command $command -GroupOutput

    # copy the package to the local registry
    Add-CrateToLocalRegistry `
      -LocalRegistryPath $localRegistryPath `
      -Package $package

    if ($OutputPath -and $package.OutputPackage) {
      $sourcePath = "$RepoRoot/target/package/$packageName-$packageVersion"
      $targetPath = "$OutputPath/$packageName"
      $targetApiReviewFile = "$targetPath/$packageName.rust.json"

      Write-Host "Copying package '$packageName' to '$targetPath'"
      New-Item -ItemType Directory -Path $targetPath -Force | Out-Null
      Copy-Item -Path "$sourcePath.crate" -Destination $targetPath

      Write-Host "Creating API review file"
      $apiReviewFile = Create-ApiViewFile $package
      
      Write-Host "Copying API review file to '$targetApiReviewFile'"
      Copy-Item -Path $apiReviewFile -Destination $targetApiReviewFile -Force
    }
  }

  Write-Host "Removing local registry"
  Remove-Item -Path $localRegistryPath -Recurse -Force | Out-Null
}
finally {
  Pop-Location
}
