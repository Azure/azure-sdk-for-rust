#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackagePath,
  [string[]]$PackageNames,
  [string]$Token
)

$ErrorActionPreference = 'Stop'
#Set-StrictMode -Version 2.0

. (Join-Path $PSScriptRoot '..' 'common' 'scripts' 'common.ps1')
. (Join-Path $EngCommonScriptsDir 'Helpers' 'CommandInvocation-Helpers.ps1')

$request = @{
  'Headers' = @{ 'Authorization' = $Token };
  'Uri'     = 'https://api.crates.io/api/v1/crates/new/';
  'Method'  = 'PUT';
}

foreach ($packageName in $packageNames) {
  $crateFile = Get-ChildItem "$PackagesPath/$packageName" -Filter '*.crate'

  Write-Host "Publishing package: '$packageName'"
  # https://doc.rust-lang.org/cargo/reference/registry-web-api.html#publish
  # Invoke-WebRequest @request -InFile $crateFile
}