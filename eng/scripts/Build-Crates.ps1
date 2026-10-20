#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = 'ManifestDir')]
param(
  [Parameter(Position = 0, ParameterSetName = 'ManifestDir')]
  [string[]] $ManifestDir
)

$ErrorActionPreference = 'Stop'

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'common.ps1'))

Write-Host @"
Building crates with
    RUSTFLAGS: '${env:RUSTFLAGS}'
"@

[string[]] $manifestPath = Get-CargoManifestPaths -ManifestDir $ManifestDir
if ($ManifestDir) {
  LogDebug "Building manifest(s) '$( $manifestPath -join "'. '" )' and dependencies"
}
else {
  LogDebug "Building all packages in workspace"
}

foreach ($path in $manifestPath) {
  Invoke-LoggedCommand "cargo build --manifest-path '$path' --keep-going --all-features" -GroupOutput
}
