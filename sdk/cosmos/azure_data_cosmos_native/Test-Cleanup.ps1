# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cSpell:ignore TEAMPROJECTID HOMEDIRECTORY

# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

$IsAzDo = ($null -ne $env:SYSTEM_TEAMPROJECTID)
if($IsAzDo) {
    $AzDoEmulatorPath = Join-Path $env:AGENT_HOMEDIRECTORY "..\..\Program Files\Azure Cosmos DB Emulator\Microsoft.Azure.Cosmos.Emulator.exe"
}


if ($IsWindows) {
    if (Test-Path $AzDoEmulatorPath) {
        Write-Host "Detected Azure DevOps Agent environment with Cosmos DB Emulator. Stopping Cosmos DB Emulator."
        Invoke-LoggedCommand "& `"$AzDoEmulatorPath`" /shutdown" | Out-Null
    } else {
        Write-Host "Unable to confirm Cosmos DB Emulator location, skipping cleanup."
    }
} elseif (Get-Command docker -ErrorAction SilentlyContinue) {
    Write-Host "Docker detected. Stopping Cosmos DB Emulator container."

    $containerName = "cosmosdb-emulator-test"

    $containerStatus = docker ps -a --filter "name=$containerName" --format "{{.Status}}"
    if ($containerStatus) {
        Write-Host "Stopping and removing container $containerName..."
        Invoke-LoggedCommand "docker rm -f $containerName" | Out-Null
    }

    Write-Host "Cosmos DB Emulator container stopped and removed."
} else {
    Write-Host "Docker is not available. No Cosmos DB Emulator container to clean up."
}