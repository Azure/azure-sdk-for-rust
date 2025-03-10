#!/usr/bin/env pwsh
#Requires -Version 7.0

[CmdletBinding()]
param(
  [switch]$Publishable,
  [switch]$NoDev,
  [switch]$NoReq
)

$ErrorActionPreference = 'Stop'

$packages = & (Join-Path $PSScriptRoot 'Get-PackageVersions.ps1')

if ($Publishable) {
  $packages = $packages | Where-Object { $_.publish }
}

$dependencies = $packages | Select-Object -ExpandProperty packageDependencies

if ($NoDev) {
  $dependencies = $dependencies | Where-Object { $_.kind -ne 'dev' }
}

if ($NoReq) {
  $dependencies = $dependencies | Where-Object { $_.req -eq '*' }
}

$dependencies | Select-Object -Property @(
  @{ Name = 'from'; Expression = { $_.dependant } },
  @{ Name = 'to'; Expression = { $_.name } },
  'kind',
  @{ Name = 'path'; Expression = { !!$_.path } },
  'req',
  @{ Name = 'local'; Expression = { $_.pathVersion } },
  @{ Name = 'index'; Expression = { $_.indexVersion } }
)
