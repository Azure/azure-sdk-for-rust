#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

$resolvedToolchain = Get-ResolvedRustToolchain -Toolchain 'nightly'
if (!$resolvedToolchain) {
  LogErrorForFile $PSCommandPath "Failed to resolve Rust nightly toolchain."
  exit 1
}

$cargoArgs = @(
  'watch',
  '-s',
  "RUSTDOCFLAGS=""--cfg=docsrs --enable-index-page -Z unstable-options"" cargo +$resolvedToolchain doc --all-features --workspace --no-deps",
  '-s',
  'http-server --index --port 8080 ./target/doc'
)

Push-Location $RepoRoot
try {
  Write-Host "> cargo $($cargoArgs -join ' ')"
  & cargo @cargoArgs 2>&1 | ForEach-Object { $_ }

  if ($LASTEXITCODE -ne 0) {
    LogErrorForFile $PSCommandPath "Command failed to execute: cargo $($cargoArgs -join ' ')"
    exit $LASTEXITCODE
  }
}
finally {
  Pop-Location
}
