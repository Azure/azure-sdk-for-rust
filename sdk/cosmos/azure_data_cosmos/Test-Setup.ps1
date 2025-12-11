# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

if ($IsWindows) {
    # TODO: Install emulator on Windows machines
    throw "Not yet supported on Windows."
} elseif (Get-Command "docker" -ErrorAction SilentlyContinue) {
    Write-Host "Docker detected. Using Cosmos DB Emulator in Docker."

    # Check if the emulator is already running
    $existingContainer = docker ps --filter "name=cosmosdb-emulator-test" --format "{{.Names}}"
    if ($existingContainer -eq "cosmosdb-emulator-test") {
        Write-Host "Cosmos DB Emulator container is already running."
    } else {
        LogGroupStart "Starting Cosmos DB Emulator in Docker"
        # Start Cosmos DB Emulator in Docker
        $containerName = "cosmosdb-emulator-test"
        Invoke-LoggedCommand "docker run -d -p 8081:8081 -p 10250:10250 -p 10251:10251 -p 10252:10252 -p 10253:10253 -p 10254:10254 --name $containerName mcr.microsoft.com/cosmosdb/linux/azure-cosmos-emulator"

        # Wait for the emulator to be ready by checking the logs for "Started"
        $maxRetries = 30
        $retryCount = 0
        $emulatorReady = $false
        while (-not $emulatorReady -and $retryCount -lt $maxRetries) {
            Start-Sleep -Seconds 2
            $logs = docker logs $containerName 2>&1
            if ($logs -match "Started") {
                $emulatorReady = $true
                break
            }
            $retryCount++
        }
        LogGroupEnd
    }

    # Set environment variables for the tests
    $env:AZURE_COSMOS_CONNECTION_STRING = "emulator"

    Write-Host "Cosmos DB Emulator is running in Docker."
} else {
    throw "Docker is not available. Cannot start Cosmos DB Emulator."
}