
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
  $packageInfoFiles = Get-ChildItem -Path $packageInfoDirectory -Filter '*.json' -File -Recurse
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

function Get-CargoPackageByName(
  $WorkspacePackages,
  [string] $PackageName
) {
  $package = $WorkspacePackages | Where-Object -Property name -EQ -Value $PackageName | Select-Object -First 1
  if (!$package) {
    throw "Package '$PackageName' is not in the workspace."
  }

  return $package
}

function Resolve-CargoPackageNames(
  [string[]] $PackageName
) {
  return @(
    $PackageName `
    | ForEach-Object { $_ -split ',' } `
    | ForEach-Object { $_.Trim() } `
    | Where-Object { $_ } `
    | Select-Object -Unique
  )
}

function Resolve-CargoManifestPath(
  [string] $ManifestDir
) {
  $directoryPath = Resolve-Path -Path $ManifestDir -ErrorAction Stop
  $manifestPath = [System.IO.Path]::Combine($directoryPath, 'Cargo.toml')
  if (!(Test-Path -Path $manifestPath -PathType Leaf)) {
    throw "Cargo manifest '$manifestPath' does not exist."
  }
  return (Resolve-Path -Path $manifestPath -ErrorAction Stop).Path
}

function Get-NormalizedCargoManifestPath(
  [string] $ManifestPath
) {
  return [System.IO.Path]::GetFullPath((Resolve-Path -Path $ManifestPath -ErrorAction Stop).Path)
}

function Get-CargoManifestPaths(
  [string[]] $PackageName,
  [string[]] $ManifestDir,
  [string] $PackageInfoDirectory,
  [switch] $Workspace,
  $WorkspacePackages = $null
) {
  if ($ManifestDir) {
    return @($ManifestDir | ForEach-Object { Resolve-CargoManifestPath -ManifestDir $_ })
  }

  if ($PackageInfoDirectory) {
    if (!(Test-Path -Path $PackageInfoDirectory -PathType Container)) {
      throw "Package info path '$PackageInfoDirectory' does not exist."
    }

    if (!$WorkspacePackages) {
      $WorkspacePackages = Get-CargoPackages
    }

    return @(
      foreach ($packageInfo in (Get-PackagesFromPackageInfo $PackageInfoDirectory)) {
        $directoryPathProperty = $packageInfo.PSObject.Properties['DirectoryPath']
        $nameProperty = $packageInfo.PSObject.Properties['Name']
        if ($directoryPathProperty -and $directoryPathProperty.Value) {
          $directoryPath = $directoryPathProperty.Value
          if (![System.IO.Path]::IsPathRooted($directoryPath)) {
            $directoryPath = [System.IO.Path]::Combine($RepoRoot, $directoryPath)
          }
          Resolve-CargoManifestPath -ManifestDir $directoryPath
        }
        elseif ($nameProperty -and $nameProperty.Value) {
          $package = Get-CargoPackageByName -WorkspacePackages $WorkspacePackages -PackageName $nameProperty.Value
          $package.manifest_path
        }
        else {
          throw "Package info must contain either a DirectoryPath or Name property."
        }
      }
    ) | Select-Object -Unique
  }

  if ($PackageName) {
    if (!$WorkspacePackages) {
      $WorkspacePackages = Get-CargoPackages
    }

    return @(
      foreach ($name in (Resolve-CargoPackageNames -PackageName $PackageName)) {
        $package = Get-CargoPackageByName -WorkspacePackages $WorkspacePackages -PackageName $name
        $package.manifest_path
      }
    )
  }

  if ($Workspace -or (!$PackageName -and !$ManifestDir -and !$PackageInfoDirectory)) {
    return @([System.IO.Path]::Combine($RepoRoot, 'Cargo.toml'))
  }
}

function Get-CargoPackagesFromManifestPaths(
  [string[]] $ManifestPath,
  $WorkspacePackages = $null
) {
  if (!$WorkspacePackages) {
    $WorkspacePackages = Get-CargoPackages
  }

  $workspaceManifestPath = Get-NormalizedCargoManifestPath `
    -ManifestPath ([System.IO.Path]::Combine($RepoRoot, 'Cargo.toml'))
  $packagesByManifestPath = @{}
  foreach ($workspacePackage in $WorkspacePackages) {
    $normalizedPackagePath = Get-NormalizedCargoManifestPath -ManifestPath $workspacePackage.manifest_path
    $packagesByManifestPath[$normalizedPackagePath] = $workspacePackage
  }
  $packages = @()

  foreach ($path in $ManifestPath) {
    $normalizedPath = Get-NormalizedCargoManifestPath -ManifestPath $path
    if ($normalizedPath -eq $workspaceManifestPath) {
      $packages += $WorkspacePackages
      continue
    }

    $package = $packagesByManifestPath[$normalizedPath]
    if (!$package) {
      throw "Manifest '$path' is not a package in the workspace."
    }
    $packages += $package
  }

  return @($packages | Sort-Object -Property name -Unique)
}

function Get-CargoSelectedPackages(
  [string[]] $PackageName,
  [string[]] $ManifestDir,
  [string] $PackageInfoDirectory,
  [switch] $Workspace
) {
  $workspacePackages = Get-CargoPackages
  if ($Workspace -or (!$PackageName -and !$ManifestDir -and !$PackageInfoDirectory)) {
    return $workspacePackages
  }

  [string[]] $manifestPaths = Get-CargoManifestPaths `
    -PackageName $PackageName `
    -ManifestDir $ManifestDir `
    -PackageInfoDirectory $PackageInfoDirectory `
    -Workspace:$Workspace `
    -WorkspacePackages $workspacePackages

  return Get-CargoPackagesFromManifestPaths `
    -ManifestPath $manifestPaths `
    -WorkspacePackages $workspacePackages
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
