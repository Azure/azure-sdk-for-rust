#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
param(
  # Toolchain to install: 'stable', 'nightly', 'msrv' resolve to pinned versions via [Channels];
  # 'active' uses the current toolchain for the working directory, whether that comes
  # from a rustup directory override, rust-toolchain.toml, or the rustup default.
  # any other value is passed through as a literal toolchain string.
  [string] $Toolchain = 'active',
  [int] $MaxAttempts = 3,
  [bool] $SetDefault = $true,
  # A directory outside the repo used when $Toolchain is 'active' to check the
  # rustup version without rust-toolchain.toml influence.
  [string] $OutsideDirectory = ([System.IO.Path]::GetTempPath())
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

$toolchainArg = if ($Toolchain -eq 'active') {
  # Depending on the version of rustup currently installed, simply calling `rustup --version` will
  # install the active toolchain per rust-toolchain.toml if it's not already installed. We should
  # check the rust version outside of our repo's context to avoid any rustup-toolchain file influence.
  $rustupVersion = Invoke-LoggedCommand "rustup --version" -ExecutePath $OutsideDirectory -GroupOutput

  if ($rustupVersion -match 'rustup (\d+)\.(\d+)\.\d+') {
    $major = [int] $matches[1]
    $minor = [int] $matches[2]
    # You can't call 'rustup install' without a toolchain before rustup 1.28.0.
    if ($major -lt 1 -or ($major -eq 1 -and $minor -lt 28)) {
      Invoke-LoggedCommand "rustup self update" -GroupOutput
    }
  }

  ''
} else {
  Get-ResolvedRustToolchain -Toolchain $Toolchain
}
$installToolchainArg = if ($toolchainArg) { $toolchainArg } else { Get-ResolvedRustToolchain -Toolchain $Toolchain }

$attempts = 0
while ($true) {
  $attempts++

  $installArgs = @('--no-self-update', '--profile', 'default')
  if ($SetDefault) {
    # Use a directory override because it takes precedence over rust-toolchain.toml:
    # https://rust-lang.github.io/rustup/overrides.html#directory-overrides
    # This is easier to carry through pipelines than relying on an environment
    # variable that would need to be propagated across subsequent jobs.
    $installArgs += '--override'
  }
  $installArgs += $installToolchainArg
  Invoke-LoggedCommand "rustup install $($installArgs -join ' ')" -GroupOutput -DoNotExitOnFailedExitCode

  if ($LASTEXITCODE -eq 0) { break }

  if ($attempts -lt $MaxAttempts) {
    Write-Host "Install failed, attempt $attempts, retrying..."
  } else {
    LogError "Install failed after $attempts attempts."
    exit 1
  }

  # Failures to update are usually caused by file locks on Windows.
  # Sleep for a few seconds to give the blocking process a chance to release the lock.
  Start-Sleep -Seconds 3
}

Invoke-LoggedCommand "rustup show" -GroupOutput
