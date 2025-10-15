#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
  [string]$OutputPath,
  [Parameter(ParameterSetName = 'Named')]
  [string[]]$PackageNames,
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string]$PackageInfoDirectory,
  [switch]$Release,
  [switch]$NoVerify,
  [string]$OutBuildOrderFile
)

$ErrorActionPreference = 'Stop'

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))

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

function Get-CargoPackages() {
  $metadata = Get-CargoMetadata

  # Path based dependencies are assumed to be unreleased package versions. In
  # non-release builds these should be packed as well.
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
  $outputPackageNames = Get-OutputPackageNames $packages

  [array]$packagesToBuild = $packages | Where-Object { $outputPackageNames.Contains($_.name) }

  if ($Release) { 
    return $packagesToBuild
  }

  # If not releasing, expand dependencies into list of packages to build
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

  return $packagesToBuild
}

function Get-CargoMetadata() {
  cargo metadata --no-deps --format-version 1 --manifest-path "$RepoRoot/Cargo.toml" | ConvertFrom-Json -Depth 100 -AsHashtable
}

function Create-ApiViewFile($package) {
  $packageName = $package.name
  $command = "cargo run --manifest-path $RepoRoot/eng/tools/generate_api_report/Cargo.toml -- --package $packageName"
  Invoke-LoggedCommand $command -GroupOutput | Out-Host

  $packagePath = Split-Path -Path $package.manifest_path -Parent

  "$packagePath/review/$packageName.rust.json"
}

$originalLocation = Get-Location
try {
  Set-Location $RepoRoot

  [array]$packages = Get-PackagesToBuild
  $packageParams = @()
  foreach ($package in $packages) {
    $packageParams += "--package", $package.name
  }

  if ($NoVerify) {
    $packageParams += "--no-verify"
  }

  Write-Host "> cargo publish --locked --dry-run --allow-dirty $($packageParams -join ' ')"
  & cargo publish --locked --dry-run --allow-dirty @packageParams 2>&1 | Tee-Object -Variable packResult
  if ($LASTEXITCODE) {
    Write-Host "cargo publish failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
  }

  if ($OutputPath -and $package.OutputPackage) {
    $sourcePath = [System.IO.Path]::Combine($RepoRoot, "target", "package", "$packageName-$packageVersion")
    $targetPath = [System.IO.Path]::Combine($OutputPath, $packageName)
    $targetContentsPath = [System.IO.Path]::Combine($targetPath, "contents")
    $targetApiReviewFile = [System.IO.Path]::Combine($targetPath, "$packageName.rust.json")

    if (Test-Path -Path $targetContentsPath) {
      Remove-Item -Path $targetContentsPath -Recurse -Force
    }

    Write-Host "Copying package '$packageName' to '$targetContentsPath'"
    New-Item -ItemType Directory -Path $targetContentsPath -Force | Out-Null
    Copy-Item -Path $sourcePath/* -Destination $targetContentsPath -Recurse -Exclude "Cargo.toml.orig"

    Write-Host "Creating API review file"
    $apiReviewFile = Create-ApiViewFile $package
      
    Write-Host "Copying API review file to '$targetApiReviewFile'"
    Copy-Item -Path $apiReviewFile -Destination $targetApiReviewFile -Force
  }

  if ($OutBuildOrderFile) {
    $buildOrder = @()
    foreach ($line in $packResult) { 
      if ($line -match '^\s*Packaging (\w*) ([\w\d\.-]*)') {
        $buildOrder += $matches[1]
      }
    }

    Write-Host "Build Order: $($buildOrder -join ', ')"
    $buildOrder | ConvertTo-Json -Depth 100 | Set-Content $OutBuildOrderFile
  }
}
finally {
  Set-Location $originalLocation
}
