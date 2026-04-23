#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = 'ManifestDir')]
param(
  [Parameter(Position = 0, ParameterSetName = 'ManifestDir')]
  [string[]] $ManifestDir
)

$ErrorActionPreference = 'Stop'

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

Write-Host @"
Building crates with
    RUSTFLAGS: '${env:RUSTFLAGS}'
"@

[string[]] $manifestPath = if ($ManifestDir) {
  Join-Path $ManifestDir 'Cargo.toml' -Resolve
  LogDebug "Building manifest(s) '$( $manifestPath -join "'. '" )' and dependencies"
} else {
  "$RepoRoot/Cargo.toml"
  LogDebug "Building all packages in workspace"
}

$manifestArgs = '--manifest-path ' + ($manifestPath -join ' --manifest-path ')
Invoke-LoggedCommand "cargo build --keep-going --all-features $manifestArgs" -GroupOutput
