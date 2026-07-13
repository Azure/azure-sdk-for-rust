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
