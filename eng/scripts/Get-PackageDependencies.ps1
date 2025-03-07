#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding()]

$ErrorActionPreference = 'Stop'

$packages = & (Join-Path $PSScriptRoot 'Get-PackageVersions.ps1')

$packages.packageDependencies | Select-Object -Property @(
  @{ Name = 'from'; Expression = { $_.name } },
  @{ Name = 'to'; Expression = { $_.dependant } },
  'kind',
  'req',
  @{ Name = 'local'; Expression = { $_.pathVersion } },
  @{ Name = 'index'; Expression = { $_.indexVersion } },
  @{ Name = 'byPath'; Expression = { !!$_.path } }
)
