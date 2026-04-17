#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = 'PackageInfo')]
param(
  [Parameter(ParameterSetName = 'PackageInfo')]
  [string]$PackageInfoDirectory,

  [Parameter(Position = 0, ParameterSetName = 'PackageName')]
  [ValidateNotNullOrEmpty()]
  [string[]]$PackageName,

  [string]$Toolchain = 'stable',
  [switch]$Deny,
  [switch]$SkipPackageAnalysis
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

Write-Host @"
Analyzing code with
    RUSTFLAGS: '${env:RUSTFLAGS}'
    RUSTDOCFLAGS: '${env:RUSTDOCFLAGS}'
    RUST_LOG: '${env:RUST_LOG}'
"@

if ($Deny) {
  Invoke-LoggedCommand "cargo install cargo-deny --locked" -GroupOutput
}

$packageArgs = if ($PackageName) {
  '--package ' + ($PackageName -join ' --package ')
}

$cargoAuditVersionParams = Get-VersionParamsFromCgManifest cargo-audit
Invoke-LoggedCommand "cargo install cargo-audit --locked $($cargoAuditVersionParams -join ' ')" -GroupOutput
Invoke-LoggedCommand "cargo audit" -GroupOutput

Invoke-LoggedCommand "cargo check --package azure_core $packageArgs --all-features --all-targets --keep-going" -GroupOutput

Invoke-LoggedCommand "cargo fmt $packageArgs -- --check" -GroupOutput

Invoke-LoggedCommand "cargo clippy $packageArgs --all-features --all-targets --keep-going --no-deps" -GroupOutput

if ($Deny) {
  Invoke-LoggedCommand "cargo deny --all-features check" -GroupOutput
}

Invoke-LoggedCommand "cargo doc --no-deps --all-features" -GroupOutput

# Verify package dependencies and keywords
$verifyDependenciesScript = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'verify-dependencies.rs'))
$verifyKeywordsScript = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'verify-keywords.rs'))
$checkApiSupersetManifest = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'check_api_superset', 'Cargo.toml'))

if (!$SkipPackageAnalysis) {
  $checkApiSupersetCrates = @('typespec', 'typespec_client_core', 'azure_core')

  if ($PSCmdlet.ParameterSetName -eq 'PackageInfo' -and !(Test-Path $PackageInfoDirectory)) {
    Write-Host "Analyzing workspace`n"
    $manifestPath = ([System.IO.Path]::Combine($RepoRoot, 'Cargo.toml'))
    Invoke-LoggedCommand "&$verifyDependenciesScript $manifestPath" -GroupOutput
    Invoke-LoggedCommand "&$verifyKeywordsScript $manifestPath" -GroupOutput

    Invoke-LoggedCommand "cargo run --manifest-path $checkApiSupersetManifest" -GroupOutput
    return
  }

  if ($Toolchain -eq 'nightly') {
    Invoke-LoggedCommand "cargo install --locked cargo-docs-rs" -GroupOutput
  }

  class Package {
    [string] $Name
    [string] $DirectoryPath

    Package([string] $name) {
      $this.Name = $name
    }

    static $Workspace = {
      $manifestPath = [System.IO.Path]::Combine($RepoRoot, 'Cargo.toml')
      cargo metadata --format-version 1 --no-deps --manifest-path $manifestPath | ConvertFrom-Json
    }.Invoke()

    [string] ManifestPath() {
      if ($this.DirectoryPath) {
        return [System.IO.Path]::Combine($this.DirectoryPath, 'Cargo.toml')
      }

      $manifestPath = [Package]::Workspace.packages.Where({ $_.name -eq $this.Name })
      if (!$manifestPath -or $manifestPath.Count -gt 1) {
        throw "Package $($this.Name) not found in workspace"
      }

      return $manifestPath
    }

    [string] ToString() {
      if ($this.DirectoryPath) {
        return "'$($this.Name)' in directory '$($this.DirectoryPath)'"
      }

      return "'$($this.Name)'"
    }
  }

  [Package[]] $packagesToTest = if ($PackageName) {
    foreach ($name in $PackageName) {
      [Package]::new($name)
    }
  }
  else {
    Get-ChildItem $PackageInfoDirectory -Filter "*.json" -Recurse
    | Get-Content -Raw
    | ConvertFrom-Json
    | ForEach-Object {
      $package = [Package]::new($_.Name)
      $package.DirectoryPath = $_.DirectoryPath
      $package
    }
  }

  $shouldCheckApiSuperset = $false
  foreach ($package in $packagesToTest) {
    Write-Host "Analyzing package $($package.ToString())`n"
    $packageManifestPath = $package.ManifestPath()
    Invoke-LoggedCommand "&$verifyDependenciesScript $packageManifestPath" -GroupOutput
    Invoke-LoggedCommand "&$verifyKeywordsScript $packageManifestPath" -GroupOutput

    if ($Toolchain -eq 'nightly') {
      Invoke-LoggedCommand "cargo +nightly docs-rs --package $($package.Name)" -GroupOutput
    }

    if ($checkApiSupersetCrates -contains $package.Name) {
      $shouldCheckApiSuperset = $true
    }
  }

  if ($shouldCheckApiSuperset) {
    Invoke-LoggedCommand "cargo run --manifest-path $checkApiSupersetManifest" -GroupOutput
  }
}
