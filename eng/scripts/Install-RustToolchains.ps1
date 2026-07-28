#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding(SupportsShouldProcess)]
param(
  [switch] $MSRV,
  [switch] $Nightly,
  [switch] $Tools
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

$installedToolchains = @{}

$rustupVersion = Invoke-LoggedCommand 'rustup --version' -GroupOutput
if (!($rustupVersion -match 'rustup (\d+)\.(\d+)\.\d+')) {
  LogError "Failed to determine rustup version. rustup 1.28.0 or newer is required. Run 'rustup self update' and rerun this script."
  exit 1
}

$major = [int] $matches[1]
$minor = [int] $matches[2]
# `rustup install` without an explicit toolchain requires rustup >= 1.28.0.
if ($major -lt 1 -or ($major -eq 1 -and $minor -lt 28)) {
  LogError "rustup 1.28.0 or newer is required; detected $($matches[0]). Run 'rustup self update' and rerun this script."
  exit 1
}

function Install-RustToolchain(
  [string] $Toolchain
) {
  $resolvedToolchain = Get-ResolvedRustToolchain -Toolchain $Toolchain
  if ($installedToolchains.ContainsKey($resolvedToolchain)) {
    return
  }

  $installedToolchains[$resolvedToolchain] = $true
  $command = "rustup install --no-self-update $resolvedToolchain"
  if ($PSCmdlet.ShouldProcess($resolvedToolchain, $command)) {
    Invoke-LoggedCommand $command -GroupOutput
  }
}

Install-RustToolchain -Toolchain 'stable'

if ($MSRV) {
  Install-RustToolchain -Toolchain 'msrv'
}

if ($Nightly) {
  Install-RustToolchain -Toolchain 'nightly'
}

if ($Tools) {
  $toolsDirectory = [System.IO.Path]::Combine($RepoRoot, 'eng', 'tools')
  $command = 'rustup install'
  if ($PSCmdlet.ShouldProcess('eng/tools', "$command (in $toolsDirectory)")) {
    Invoke-LoggedCommand $command -ExecutePath $toolsDirectory -GroupOutput
  }
}

Invoke-LoggedCommand 'rustup show' -GroupOutput
