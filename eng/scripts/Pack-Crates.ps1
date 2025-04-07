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

# https://doc.rust-lang.org/cargo/reference/registry-web-api.html#publish
# https://github.com/rust-lang/cargo/blob/5c87c14f9a162daf10d4133fdaab35c72d67b018/crates/crates-io/lib.rs#L42
function Get-ApiMetadata($package) {
  $packagePath = Split-Path -Path $package.manifest_path -Parent
  $readmePath = $package.readme ? (Join-Path -Path $packagePath -ChildPath $package.readme) : $null

  $jsonBody = [ordered]@{
    'name'          = $package.name
    'vers'          = $package.version
    'deps'          = @()
    'features'      = $package.features
    'authors'       = $package.authors
    'description'   = $package.description
    'documentation' = $package.documentation
    'homepage'      = $package.homepage
    'readme'        = if ($readmePath -and (Test-Path -Path $readmePath)) {
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

function New-ApiPutFile($crateMetadata, $crateFilePath) {
  $metadataBytes = [Text.Encoding]::Utf8.GetBytes($crateMetadata)
  $metadataLengthBytes = [BitConverter]::GetBytes([UInt32]$metadataBytes.Length)
  $crateBytes = [IO.File]::ReadAllBytes($crateFilePath)
  $crateLengthBytes = [BitConverter]::GetBytes([UInt32]$crateBytes.Length)

  $bytes += $metadataLengthBytes + $metadataBytes + $crateLengthBytes + $crateBytes

  return $bytes
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
  [array]$packages = Get-PackagesToBuild

  $command = "cargo +nightly -Zpackage-workspace package --allow-dirty --locked"

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
    Write-Host "##[group] $RepoRoot/Cargo.lock"
    Get-Content "$RepoRoot/Cargo.lock"
    Write-Host "##[endgroup]"
  }

  Invoke-LoggedCommand -Command $command -GroupOutput

  if ($env:SYSTEM_DEBUG -eq 'true') {
    Write-Host "##[group] $RepoRoot/Cargo.lock"
    Get-Content "$RepoRoot/Cargo.lock"
    Write-Host "##[endgroup]"
  }

  if ($OutputPath) {
    foreach ($package in $packages | Where-Object { $_.OutputPackage }) {
      $packageName = $package.name
      $packageVersion = $package.version
      
      Write-Host "`nProcessing package '$packageName'"

      $sourcePath = "$RepoRoot/target/package/$packageName-$packageVersion.crate"
      $targetPath = "$OutputPath/$packageName"
      $targetContentsPath = "$targetPath/contents"
      $targetApiReviewFile = "$targetPath/$packageName.rust.json"
      $targetCrateFile = "$targetPath/$packageName-$packageVersion.crate"
      $targetJsonFile = "$targetPath/$packageName-$packageVersion.json"
      $targetBinFile = "$targetPath/$packageName.bin"

      Remove-Item -Path $targetPath -Recurse -Force -ErrorAction SilentlyContinue
      New-Item -ItemType Directory -Path $targetContentsPath -Force | Out-Null

      Write-Host "Copying crate file to '$targetCrateFile'"
      Copy-Item -Path $sourcePath -Destination $targetCrateFile -Force

      $crateMetadata = Get-ApiMetadata $package | ConvertTo-Json -Depth 10

      Write-Host "Writing crates.io request metadata to '$targetJsonFile'"
      $crateMetadata | Out-File -FilePath "$targetJsonFile" -Encoding utf8

      $uploadBytes = New-ApiPutFile $crateMetadata $sourcePath
      Write-Host "Writing crates.io request bundle to '$targetBinFile'"
      [IO.File]::WriteAllBytes($targetBinFile, $uploadBytes)

      Write-Host "Exctracting crate file to '$targetContentsPath'"
      New-Item -ItemType Directory -Path $targetContentsPath -Force | Out-Null
      tar -xf $sourcePath --directory $targetContentsPath --strip-components=1 | Out-Null
      
      Write-Host "Creating API review file"
      $apiReviewFile = Create-ApiViewFile $package
      
      Write-Host "Copying API review file to '$targetApiReviewFile'"
      Copy-Item -Path $apiReviewFile -Destination $targetApiReviewFile -Force
    }
  }
}
finally {
  Pop-Location
}
