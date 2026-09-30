#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding()]
param(
  [ValidateNotNullOrEmpty()]
  [string] $Toolchain = 'nightly',

  [ValidateNotNullOrEmpty()]
  [string] $VariableName = 'ApiReviewRustToolchain'
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'shared', 'common.ps1'))
. ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'shared', 'ApiReviewHub.ps1'))

$resolvedToolchain = Resolve-ApiReviewHubRustToolchain -Toolchain $Toolchain -ExecuteDir $RepoRoot
Write-Host "Resolved API Review Hub Rust toolchain '$Toolchain' to '$resolvedToolchain'"
Write-Host "##vso[task.setvariable variable=$VariableName]$resolvedToolchain"
