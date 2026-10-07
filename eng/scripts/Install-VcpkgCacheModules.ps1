#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

# Az.Storage 9.7.2 requires Az.Accounts >= 5.5.2. These are fallback versions;
# compatible preinstalled modules are reused.
$modules = [ordered]@{
  'Az.Accounts' = '5.5.3'
  'Az.Storage' = '9.7.2'
}
$repositoryReady = $false
$operation = 'discovering Az modules'

try {
  foreach ($name in $modules.Keys) {
    $version = $modules[$name]
    $operation = "discovering $name >= $version"
    $module = Get-Module -Name $name -ListAvailable |
      Where-Object { $_.Version -ge [version]$version } |
      Sort-Object Version -Descending |
      Select-Object -First 1

    if (!$module) {
      if (!$repositoryReady) {
        $operation = 'preparing PSGallery'
        $repository = Get-PSRepository | Where-Object Name -EQ 'PSGallery'
        if (!$repository) {
          Register-PSRepository -Default -ErrorAction Stop
          $repository = Get-PSRepository -Name PSGallery -ErrorAction Stop
        }
        if (!$repository -or $repository.SourceLocation.TrimEnd('/') -ne 'https://www.powershellgallery.com/api/v2') {
          throw 'PSGallery must use https://www.powershellgallery.com/api/v2.'
        }
        $repositoryReady = $true
      }

      $operation = "installing $name $version from PSGallery"
      Write-Host "Installing $name $version from PSGallery (CurrentUser)."
      Install-Module -Name $name -RequiredVersion $version -Repository PSGallery -Scope CurrentUser -Force -ErrorAction Stop
      $module = Get-Module -Name $name -ListAvailable |
        Where-Object { $_.Version -eq [version]$version } |
        Select-Object -First 1
      if (!$module) {
        throw "$name $version is not discoverable after installation."
      }
    }

  }

  # Finish installation before importing: PowerShellGet -Force can reinstall
  # dependencies, whose assemblies would otherwise be locked on Windows.
  foreach ($name in $modules.Keys) {
    $operation = "rediscovering $name after installation"
    $module = Get-Module -Name $name -ListAvailable |
      Where-Object { $_.Version -ge [version]$modules[$name] } |
      Sort-Object Version -Descending |
      Select-Object -First 1
    if (!$module) {
      throw "$name >= $($modules[$name]) is not discoverable after installation."
    }
    $operation = "importing $name $($module.Version)"
    Import-Module -Name $module.Path -Global -ErrorAction Stop
    Write-Host "Verified import: $name $($module.Version)."
  }

  $operation = 'verifying Az cmdlets'
  $commands = @{
    'Az.Accounts\Connect-AzAccount' = @('ServicePrincipal', 'Tenant', 'FederatedToken')
    'Az.Accounts\Clear-AzContext' = @('Scope', 'Force')
    'Az.Accounts\Set-AzContext' = @('Subscription', 'Tenant')
    'Az.Storage\New-AzStorageContext' = @('StorageAccountName', 'UseConnectedAccount')
    'Az.Storage\New-AzStorageContainerSASToken' = @('Name', 'Permission', 'Context', 'ExpiryTime')
  }
  foreach ($name in $commands.Keys) {
    $command = Get-Command -Name $name -ErrorAction Stop
    foreach ($parameter in $commands[$name]) {
      if (!$command.Parameters.ContainsKey($parameter)) {
        throw "$name is missing required parameter $parameter."
      }
    }
  }
  Write-Host 'Az modules are ready for the vcpkg write-mode cache task.'
}
catch {
  Write-Host "Failed while ${operation}: $($_.Exception.Message)" -ForegroundColor Red
  exit 1
}
