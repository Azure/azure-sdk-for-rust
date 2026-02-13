# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cSpell:ignore noui noexplorer disableratelimiting enableaadauthentication partitioncount

<#
.SYNOPSIS
Starts the Cosmos DB Linux Emulator using Docker.

.DESCRIPTION
This script starts the Cosmos DB Linux Emulator in a Docker container and waits for it to be fully started.
It monitors the container logs and waits for a line containing only "Started" (not "Starting x/y" which are intermediate outputs).

.PARAMETER ContainerName
The name to use for the Docker container. Default is "cosmosdb-emulator-test".

.PARAMETER PartitionCount
The number of partitions to configure for the emulator. Default is 50.

.PARAMETER MaxRetries
Maximum number of retries to wait for the emulator to start. Default is 30.

.PARAMETER RetryIntervalSeconds
Number of seconds to wait between retries. Default is 5.

.EXAMPLE
.\Start-CosmosEmulator.ps1

.EXAMPLE
.\Start-CosmosEmulator.ps1 -ContainerName "my-cosmos-emulator" -PartitionCount 100
#>

[CmdletBinding()]
param(
    [string]$ContainerName = "cosmosdb-emulator-test",
    [int]$PartitionCount = 50,
    [int]$MaxRetries = 30,
    [int]$RetryIntervalSeconds = 5
)

# Load common ES scripts
$commonScriptPath = ([System.IO.Path]::Combine($PSScriptRoot, "..", "common", "scripts", "common.ps1"))
. $commonScriptPath

# Check if Docker is available
if (-not (Get-Command "docker" -ErrorAction SilentlyContinue)) {
    LogError "Docker is not available. Please install Docker to use the Cosmos DB Emulator."
    exit 1
}

# Check if the emulator is already running
$existingContainer = docker ps --filter "name=$ContainerName" --format "{{.Names}}"
if ($existingContainer -eq $ContainerName) {
    Write-Host "Cosmos DB Emulator container '$ContainerName' is already running."
    return
}

# Remove any stopped container with the same name
$stoppedContainer = docker ps -a --filter "name=$ContainerName" --format "{{.Names}}"
if ($stoppedContainer -eq $ContainerName) {
    Write-Host "Removing stopped container '$ContainerName'..."
    docker rm -f $ContainerName | Out-Null
}

LogGroupStart "Starting Cosmos DB Emulator in Docker"

# Start Cosmos DB Emulator in Docker
Write-Host "Starting container '$ContainerName' with $PartitionCount partitions..."

$containerId = docker run -d `
    -e "AZURE_COSMOS_EMULATOR_ARGS=/noexplorer /noui /disableratelimiting /enableaadauthentication" `
    -e "AZURE_COSMOS_EMULATOR_PARTITION_COUNT=$PartitionCount" `
    -p 8081:8081 `
    -p 10250:10250 `
    -p 10251:10251 `
    -p 10252:10252 `
    -p 10253:10253 `
    -p 10254:10254 `
    --name $ContainerName `
    mcr.microsoft.com/cosmosdb/linux/azure-cosmos-emulator

if ($LASTEXITCODE -ne 0) {
    LogError "Failed to start Cosmos DB Emulator container."
    LogGroupEnd
    exit 1
}

Write-Host "Container started. Waiting for emulator to be ready..."

# Wait for the emulator to be ready by polling the logs for a line with only "Started" on it
$retryCount = 0
$emulatorStarted = $false

while (-not $emulatorStarted -and $retryCount -lt $MaxRetries) {
    $logs = docker logs $ContainerName 2>&1

    $lastLine = $logs | Select-Object -Last 1
    if ($lastLine -match "^\s*Started\s*$") {
        $emulatorStarted = $true
        Write-Host "Cosmos DB Emulator started successfully."
        break
    } elseif ($lastLine -match "^\s*Started (\d+/\d+) partitions\s*$") {
        $partitionsStarted = $matches[1]
        Write-Host "[Retry: $retryCount/$MaxRetries] Emulator still starting, $partitionsStarted partitions started."
    } else {
        Write-Host "[Retry: $retryCount/$MaxRetries] Emulator still starting..."
    }
    
    $retryCount++
    Start-Sleep -Seconds $RetryIntervalSeconds
}

if (-not $emulatorStarted) {
    LogError "Cosmos DB Emulator failed to start within the expected time ($($MaxRetries * $RetryIntervalSeconds) seconds)."
    Write-Host "Last 20 lines of container logs:"
    docker logs $ContainerName --tail 20 2>&1 | ForEach-Object { Write-Host $_ }
    LogGroupEnd
    exit 1
}

LogGroupEnd

Write-Host "Cosmos DB Emulator is running in Docker."
Write-Host "Container name: $ContainerName"
Write-Host "Endpoint: https://localhost:8081/"
