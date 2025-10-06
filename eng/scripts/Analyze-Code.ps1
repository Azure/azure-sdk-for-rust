#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory,
  [string]$Toolchain = 'stable'
  [switch]$CheckWasm = $true,
  [switch]$Deny,
  [switch]$SkipPackageAnalysis
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. (Join-Path $PSScriptRoot '..' 'common' 'scripts' 'common.ps1')

Write-Host @"
Analyzing code with
    RUSTFLAGS: '${env:RUSTFLAGS}'
    RUSTDOCFLAGS: '${env:RUSTDOCFLAGS}'
"@

if ($CheckWasm) {
  # Temporary fix to exit immediately on failure. LogError should Write-Error
  # instead
  $command = "rustup target add wasm32-unknown-unknown"
  Invoke-LoggedCommand $command
  if ($LastExitCode) {
    Write-Error "Failed to execute $command"
  }
}

if ($Deny) {
  # Temporary fix to exit immediately on failure. LogError should Write-Error
  # instead
  $command = "cargo install cargo-deny --locked"
  Invoke-LoggedCommand $command
  if ($LastExitCode) {
    Write-Error "Failed to execute $command"
  }
}

# Temporary fix to exit immediately on failure. LogError should Write-Error
# instead
$command = "cargo check --package azure_core --all-features --all-targets --keep-going"
Invoke-LoggedCommand $command
if ($LastExitCode) {
  Write-Error "Failed to execute $command"
}

# Temporary fix to exit immediately on failure. LogError should Write-Error
# instead
$command = "cargo fmt --all -- --check"
Invoke-LoggedCommand $command
if ($LastExitCode) {
  Write-Error "Failed to execute $command"
}

# Temporary fix to exit immediately on failure. LogError should Write-Error
# instead
$command = "cargo clippy --workspace --all-features --all-targets --keep-going --no-deps"
Invoke-LoggedCommand $command
if ($LastExitCode) {
  Write-Error "Failed to execute $command"
}

if ($CheckWasm) {
  # Save the original RUSTFLAGS to restore later
  $OriginalRustFlags = $env:RUSTFLAGS
  # This is needed to ensure that the `getrandom` crate uses the `wasm_js` backend
  $env:RUSTFLAGS = ${env:RUSTFLAGS} + ' --cfg getrandom_backend="wasm_js"'

  # Temporary fix to exit immediately on failure. LogError should Write-Error
  # instead
  $command = "cargo clippy --target=wasm32-unknown-unknown --workspace --keep-going --no-deps"
  Invoke-LoggedCommand $command
  if ($LastExitCode) {
    Write-Error "Failed to execute $command"
  }

  # Restore the original RUSTFLAGS, since the getrandom config option can only be set for wasm32-unknown-unknown builds.
  $env:RUSTFLAGS = $OriginalRustFlags
}

if ($Deny) {
  # Temporary fix to exit immediately on failure. LogError should Write-Error
  # instead
  $command = "cargo deny --all-features check"
  Invoke-LoggedCommand $command
  if ($LastExitCode) {
    Write-Error "Failed to execute $command"
  }
}

# Temporary fix to exit immediately on failure. LogError should Write-Error
# instead
$command = "cargo doc --workspace --no-deps --all-features"
Invoke-LoggedCommand $command
if ($LastExitCode) {
  Write-Error "Failed to execute $command"
}

# Verify package dependencies
$verifyDependenciesScript = Join-Path $RepoRoot 'eng' 'scripts' 'verify-dependencies.rs' -Resolve

if (!$SkipPackageAnalysis) {
  if (!(Test-Path $PackageInfoDirectory)) {
    Write-Host "Analyzing workspace`n"
    # Temporary fix to exit immediately on failure. LogError should Write-Error
    # instead
    $command = "&$verifyDependenciesScript $RepoRoot/Cargo.toml"
    $result = Invoke-LoggedCommand $command
    if ($LastExitCode) {
      Write-Error "Failed to execute $command"
    }
    return $result
  }

  if ($Toolchain -eq 'nightly') {
    # Temporary fix to exit immediately on failure. LogError should Write-Error
    # instead
    $command = "cargo install --locked cargo-docs-rs"
    Invoke-LoggedCommand $command
    if ($LastExitCode) {
      Write-Error "Failed to execute $command"
    }
  }

  $packagesToTest = Get-ChildItem $PackageInfoDirectory -Filter "*.json" -Recurse
  | Get-Content -Raw
  | ConvertFrom-Json

  foreach ($package in $packagesToTest) {
    Write-Host "Analyzing package '$($package.Name)' in directory '$($package.DirectoryPath)'`n"
    # Temporary fix to exit immediately on failure. LogError should Write-Error
    # instead
    $command = "&$verifyDependenciesScript $($package.DirectoryPath)/Cargo.toml"
    Invoke-LoggedCommand $command
    if ($LastExitCode) {
      Write-Error "Failed to execute $command"
    }

    if ($Toolchain -eq 'nightly') {
      # Temporary fix to exit immediately on failure. LogError should Write-Error
      # instead
      $command = "cargo +nightly docs-rs --package "
      Invoke-LoggedCommand $command
      if ($LastExitCode) {
        Write-Error "Failed to execute $command"
      }
    }
  }
}
