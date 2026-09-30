#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding()]
param(
  [ValidateNotNullOrEmpty()]
  [string] $SourceDir,

  [Parameter(Mandatory)]
  [ValidateNotNullOrEmpty()]
  [string] $ToolDir,

  [ValidateNotNullOrEmpty()]
  [string] $ToolRef = 'main',

  [ValidateNotNullOrEmpty()]
  [string] $RustToolchain = 'nightly'
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
if ([string]::IsNullOrWhiteSpace($SourceDir)) {
  $SourceDir = $RepoRoot
}

. ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'shared', 'common.ps1'))
. ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'shared', 'ApiReviewHub.ps1'))

$sourcePath = (Resolve-Path -Path $SourceDir -ErrorAction Stop).Path
$repoPath = (Resolve-Path -Path $RepoRoot -ErrorAction Stop).Path
$resolvedSourceDir = (Invoke-LoggedCommand "git -C '$sourcePath' rev-parse --show-toplevel" | Select-Object -First 1).Trim()
$resolvedRepoDir = (Invoke-LoggedCommand "git -C '$repoPath' rev-parse --show-toplevel" | Select-Object -First 1).Trim()
$resolvedToolDir = [System.IO.Path]::GetFullPath($ToolDir)
$resolvedRustToolchain = Resolve-ApiReviewHubRustToolchain -Toolchain $RustToolchain -ExecuteDir $resolvedSourceDir
$toolInfo = Get-ApiReviewHubToolInfo -SourceDir $resolvedSourceDir -ToolDir $resolvedToolDir

if ($resolvedSourceDir -ne $resolvedRepoDir) {
  Invoke-LoggedCommand "git -C '$resolvedSourceDir' fetch --depth=1 origin '$ToolRef'" -GroupOutput
  Invoke-LoggedCommand "git -C '$resolvedSourceDir' checkout --force FETCH_HEAD" -GroupOutput
  Invoke-LoggedCommand "git -C '$resolvedSourceDir' clean -ffd" -GroupOutput
}
elseif ($PSBoundParameters.ContainsKey('ToolRef')) {
  LogWarning "Ignoring -ToolRef because the source directory is the current repository."
}

New-Item -ItemType Directory -Force -Path $resolvedToolDir | Out-Null

Invoke-LoggedCommand "cargo +$resolvedRustToolchain build --manifest-path '$($toolInfo.CargoManifestPath)' -p generate_api" -ExecutePath $resolvedSourceDir -GroupOutput

if (!(Test-Path -Path $toolInfo.BuiltExecutablePath -PathType Leaf)) {
  LogError "Expected generate_api executable was not built at '$($toolInfo.BuiltExecutablePath)'."
  exit 1
}

Copy-Item -Path $toolInfo.BuiltExecutablePath -Destination $toolInfo.StagedExecutablePath -Force

Write-Host "##vso[task.setvariable variable=ApiReviewRustToolPath]$($toolInfo.StagedExecutablePath)"
Write-Host "##vso[task.setvariable variable=ApiReviewRustToolchain]$resolvedRustToolchain"
