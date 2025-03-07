#!/usr/bin/env pwsh
#Requires -Version 7.0

[CmdletBinding()]

$ErrorActionPreference = 'Stop'

$packages = & (Join-Path $PSScriptRoot 'Get-PackageVersions.ps1')

$packages.packageDependencies | Select-Object -Property @(
  @{ Name = 'from'; Expression = { $_.dependant } },
  @{ Name = 'to'; Expression = { $_.name } },
  'kind',
  @{ Name = 'path'; Expression = { !!$_.path } },
  'req',
  @{ Name = 'local'; Expression = { $_.pathVersion } },
  @{ Name = 'index'; Expression = { $_.indexVersion } }
)
