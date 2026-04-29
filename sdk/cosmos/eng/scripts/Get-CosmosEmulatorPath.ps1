# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cSpell:ignore HOMEDIRECTORY TEAMPROJECTID

<#
.SYNOPSIS
Returns the path to the Cosmos DB Emulator executable, or $null if not found.

.DESCRIPTION
Searches for the Cosmos DB Emulator in the two locations where it is expected:
- The Azure DevOps agent pre-installed location (when running in Azure Pipelines)
- The temp directory used by the Cosmos-Emulator.ps1 install script

Returns the first path found, or $null if the emulator is not present.
#>
[CmdletBinding()]
Param()

$IsAzDo = ($null -ne $env:SYSTEM_TEAMPROJECTID)
if ($IsAzDo) {
    $AzDoEmulatorPath = Join-Path $env:AGENT_HOMEDIRECTORY "..\..\Program Files\Azure Cosmos DB Emulator\Microsoft.Azure.Cosmos.Emulator.exe"
    if (Test-Path $AzDoEmulatorPath) {
        return $AzDoEmulatorPath
    }
}

$TempEmulatorPath = [System.IO.Path]::Combine($env:TEMP, 'AzureCosmosEmulator', 'Azure Cosmos DB Emulator', 'Microsoft.Azure.Cosmos.Emulator.exe')
if (Test-Path $TempEmulatorPath) {
    return $TempEmulatorPath
}

return $null
