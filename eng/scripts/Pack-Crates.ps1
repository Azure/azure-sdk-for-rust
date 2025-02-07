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
. (Join-Path $EngCommonScriptsDir 'Helpers' 'CommandInvocation-Helpers.ps1')

$cargoLocalRegistryVersion = "0.2.7"

if ($OutputPath) {
  $OutputPath = New-Item -ItemType Directory -Path $OutputPath -Force | Select-Object -ExpandProperty FullName
}

function Initialize-LocalRegistry() {
  $localRegistryPath = "$RepoRoot/target/local-registry"

  Invoke-LoggedCommand `
    -GroupOutput `
    -Command "cargo install cargo-local-registry@$cargoLocalRegistryVersion --locked" | Out-Host

  Invoke-LoggedCommand `
    -GroupOutput `
    -Command "cargo local-registry --sync . '$localRegistryPath'" | Out-NUll

  return $localRegistryPath
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

function Add-CrateToLocalRegistry($LocalRegistryPath, $Package, $CrateFilePath) {
  $packageName = $Package.name
  $packageVersion = $Package.version

  # create an index entry for the package
  $indexEntry = [ordered]@{
    name     = $packageName
    vers     = $packageVersion
    deps     = @($Package.dependencies | ForEach-Object {
        [ordered]@{
          name             = $_.rename ? $_.rename : $_.name
          req              = $_.req
          features         = $_.features
          optional         = $_.optional
          default_features = $_.uses_default_features
          target           = $_.target
          kind             = $_.kind
          package          = $_.rename ? $_.name : $null
        }
      })
    cksum    = (Get-FileHash -Path $CrateFilePath -Algorithm SHA256).Hash.ToLower()
    features = $Package.features
    yanked   = $false
  }

  $indexFolder = if ($packageName.Length -lt 3) {
    "$LocalRegistryPath/index/$($packageName.Length)"
  }
  elseif ($packageName.Length -eq 3) {
    "$LocalRegistryPath/index/$($packageName.Length)/$($packageName[0])"
  }
  else {
    "$LocalRegistryPath/index/$($packageName.Substring(0, 2))/$($packageName.Substring(2, 2))"
  }

  Write-Host "Copying package '$packageName' to local registry"
  Copy-Item -Path $CrateFilePath -Destination $LocalRegistryPath

  New-Item -ItemType Directory -Path $indexFolder -Force | Out-Null
  $indexEntry | ConvertTo-Json -Depth 100 -Compress | Out-File -Path "$indexFolder/$packageName" -Encoding utf8 -Append
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

function Get-ApiMetadata($package) {
  $packagePath = Split-Path -Path $package.manifest_path -Parent
  $readmePath = Join-Path -Path $packagePath -ChildPath $package.readme
  $jsonBody = [ordered]@{
    'name'          = $package.name
    'vers'          = $package.version
    'deps'          = @()
    'features'      = $package.features
    'authors'       = $package.authors
    'description'   = $package.description
    'documentation' = $package.documentation
    'homepage'      = $package.homepage
    'readme'        = if ($package.readme -and (Test-Path -Path $readmePath)) {
      Get-Content -Path $readmePath -Raw
    }
    else {
      $null
    }
    'readme_file'   = $package.readme
    'keywords'      = $package.keywords
    'categories'    = $package.categories
    'license'       = $package.license
    'license_file'  = $package.license_file
    'repository'    = $package.repository
    'links'         = $package.links
    'rust_version'  = $package.rust_version
  }

  foreach ($dependency in $package.dependencies) {
    $jsonBody.deps += @{
      'name'                  = $dependency.name
      'version_req'           = $dependency.req
      'features'              = $dependency.features
      'optional'              = $dependency.optional
      'default_features'      = $dependency.default_features
      'target'                = $dependency.target
      'kind'                  = $dependency.kind
      'explicit_name_in_toml' = $dependency.rename
    }
  }

  return $jsonBody
}

function New-ApiPutFile($crateMetadata, $CrateFilePath) {
  $metadataBytes = [Text.Encoding]::Utf8.GetBytes($crateMetadata)
  $metadataLengthBytes = [BitConverter]::GetBytes([UInt32]$metadataBytes.Length)
  $crateBytes = [IO.File]::ReadAllBytes($CrateFilePath)
  $crateLengthBytes = [BitConverter]::GetBytes([UInt32]$crateBytes.Length)

  $bytes += $metadataLengthBytes + $metadataBytes + $crateLengthBytes + $crateBytes

  return $bytes
}

Push-Location $RepoRoot
try {
  $localRegistryPath = Initialize-LocalRegistry

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
      -Command "cargo package --package $packageName --config `"source.crates-io.replace-with='local'`" --config `"source.local.local-registry='$localRegistryPath'`" --allow-dirty"

    $crateFile = "$RepoRoot/target/package/$packageName-$packageVersion.crate"

    if (-not (Test-Path -Path $crateFile)) {
      Write-Error "Building the package '$packageName' didn't produce a crate file in the expected location: '$crateFile'"
      exit 1
    }

    # copy the package to the local registry
    Add-CrateToLocalRegistry `
      -LocalRegistryPath $localRegistryPath `
      -Package $package `
      -CrateFilePath $crateFile

    if ($OutputPath -and $package.OutputPackage) {
      $packageOutputPath = "$OutputPath/$packageName"
      if (Test-Path -Path $packageOutputPath) {
        Remove-Item -Path $packageOutputPath -Recurse -Force
      }

      Write-Host "Copying package '$packageName' to '$packageOutputPath'"

      New-Item -ItemType Directory -Path $packageOutputPath -Force | Out-Null
      Copy-Item -Path $crateFile -Destination $packageOutputPath
      # Copy package's Cargo.toml to the output directory
      Copy-Item -Path "$RepoRoot/target/package/$packageName-$packageVersion/Cargo.toml" -Destination $packageOutputPath
      # Write package metadata to the output directory

      $metadataFile = "$packageOutputPath/cargo-metadata.json"
      $uploadFile = "$packageOutputPath/cargo-put.bin"

      $crateMetadata = Get-ApiMetadata $package | ConvertTo-Json -Depth 10

      Write-Host "Writing crates.io request metadata to '$metadataFile'"
      $crateMetadata | Out-File -FilePath "$metadataFile" -Encoding utf8

      $uploadBytes = New-ApiPutFile $crateMetadata $crateFile
      Write-Host "Writing crates.io request bundle to '$uploadFile'"
      [IO.File]::WriteAllBytes($uploadFile, $uploadBytes)
      
      Get-ApiMetadata $package | ConvertTo-Json -Depth 100 | Out-File -FilePath "$packageOutputPath/cargo-metadata.json" -Encoding utf8
    }
  }
}
finally {
  Pop-Location
}

