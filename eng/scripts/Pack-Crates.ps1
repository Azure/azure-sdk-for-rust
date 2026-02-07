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
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

Write-Host @"
Packing crates with
    RUSTFLAGS: '${env:RUSTFLAGS}'
"@


function Get-PackagesToBuild() {
  $packages = Get-CargoPackages
  $outputPackageNames = Get-OutputPackageNames $packages

  # Force array in instances of a single package name
  if ($outputPackageNames -isnot [array]) {
    $outputPackageNames = @($outputPackageNames)
  }

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

function Get-OutputPackageNames($workspacePackages) {
  $names = @()
  switch ($PsCmdlet.ParameterSetName) {
    'Named' {
      $names = $PackageNames
    }

    'PackageInfo' {
      $names = Get-PackageNamesFromPackageInfo $PackageInfoDirectory
    }

    default {
      return $workspacePackages.name
    }
  }

  foreach ($name in $names) {
    if (-not $workspacePackages.name.Contains($name)) {
      Write-Error "Package '$name' is not in the workspace or does not publish"
      exit 1
    }
  }

  return $names
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

  # Some packages are not publishable, in cases where the script is not in a
  # release context, run cargo package` instead of `cargo publish --dry-run`.
  # The two are equivalent (https://doc.rust-lang.org/cargo/reference/publishing.html)
  # though `cargo publish` has additional checks like publishability.
  $subCommand = @("package")
  if ($Release) {
    $subCommand = @("publish", "--dry-run")
  }

  LogGroupStart "cargo $($subCommand -join ' ') --locked --allow-dirty $($packageParams -join ' ')"
  Write-Host "cargo $($subCommand -join ' ') --locked --allow-dirty $($packageParams -join ' ')"
  & cargo @subCommand --locked --allow-dirty @packageParams 2>&1 `
  | Tee-Object -Variable packResult `
  | ForEach-Object { Write-Host $_ -ForegroundColor Gray }
  LogGroupEnd

  Write-Host "Finished packing crates"
  if ($LASTEXITCODE) {
    Write-Host "cargo publish failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
  }

  if ($OutputPath) {
    $OutputPath = New-Item -ItemType Directory -Path $OutputPath -Force | Select-Object -ExpandProperty FullName

    foreach ($package in $packages) {
      $sourcePath = [System.IO.Path]::Combine($RepoRoot, "target", "package", "$($package.name)-$($package.version)")
      $targetPath = [System.IO.Path]::Combine($OutputPath, $package.name)
      $targetContentsPath = [System.IO.Path]::Combine($targetPath, "contents")
      $targetApiReviewFile = [System.IO.Path]::Combine($targetPath, "$($package.name).rust.json")

      if (Test-Path -Path $targetContentsPath) {
        Remove-Item -Path $targetContentsPath -Recurse -Force
      }

      Write-Host "Copying package contents '$($package.name)' to '$targetContentsPath'"
      New-Item -ItemType Directory -Path $targetContentsPath -Force | Out-Null
      Copy-Item -Path $sourcePath/* -Destination $targetContentsPath -Recurse

      Write-Host "Copying .crate file for '$($package.name)' to '$targetPath'"
      Copy-Item -Path "$sourcePath.crate" -Destination $targetPath -Force

      # Write-Host "Creating API review file"
      # $apiReviewFile = Create-ApiViewFile $package

      # Write-Host "Copying API review file to '$targetApiReviewFile'"
      # Copy-Item -Path $apiReviewFile -Destination $targetApiReviewFile -Force
    }
  }

  if ($OutBuildOrderFile) {
    $buildOrder = @()
    foreach ($line in $packResult) {
      if ($line -match '^\s*Packaging (\w*) ([\w\d\.-]*)') {
        $buildOrder += $matches[1]
      }
    }

    Write-Host "Build Order: $($buildOrder -join ', ')"
    ConvertTo-Json $buildOrder -Depth 100 | Set-Content $OutBuildOrderFile
  }
}
finally {
  Set-Location $originalLocation
}
