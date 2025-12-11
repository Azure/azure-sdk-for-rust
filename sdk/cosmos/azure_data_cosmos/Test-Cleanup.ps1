# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

if (Get-Command docker -ErrorAction SilentlyContinue) {
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