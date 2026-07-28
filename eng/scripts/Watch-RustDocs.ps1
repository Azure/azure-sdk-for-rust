#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0

$ErrorActionPreference = 'Stop'

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'common.ps1'))
Set-StrictMode -Version 3

$resolvedToolchain = Get-ResolvedRustToolchain -Toolchain 'nightly'
if (!$resolvedToolchain) {
  LogErrorForFile $PSCommandPath "Failed to resolve Rust nightly toolchain."
  exit 1
}

$process = Start-PipedProcess `
  -FilePath 'cargo' `
  -ArgumentList @(
    'watch',
    # Ignore generated header files from some crates; should correspond to a .rs file change anyway in this repo.
    '-i',
    '*.h',
    '-s',
    "cargo +$resolvedToolchain doc --all-features --workspace --no-deps",
    '-s',
    'http-server --index --port 8080 ./target/doc'
  ) `
  -WorkingDirectory $RepoRoot `
  -Environment @{
    RUSTDOCFLAGS = '--cfg=docsrs --enable-index-page -Z unstable-options'
  } `
  -DoNotExitOnFailedExitCode

if ($process.ExitCode) {
  LogErrorForFile $PSCommandPath "Failed to watch Rust docs."
  exit $process.ExitCode
}
