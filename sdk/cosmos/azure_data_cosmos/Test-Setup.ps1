# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cSpell: ignore HOMEDIRECTORY noui noexplorer disableratelimiting enableaadauthentication partitioncount LASTEXITCODE

# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

if($env:AGENT_HOMEDIRECTORY) {
    $AzDoEmulatorPath = Join-Path $env:AGENT_HOMEDIRECTORY "..\..\Program Files\Azure Cosmos DB Emulator\Microsoft.Azure.Cosmos.Emulator.exe"
}

if ($IsWindows) {
    $EmulatorPath = $null

    if($AzDoEmulatorPath -and (Test-Path $AzDoEmulatorPath)) {
        Write-Host "Detected Azure DevOps Agent environment with Cosmos DB Emulator. Skipping Cosmos DB Emulator install."
        $EmulatorPath = $AzDoEmulatorPath
    } else {
        LogGroupStart "Installing Cosmos DB Emulator"
        & "$PSScriptRoot\..\..\..\eng\common\scripts\Cosmos-Emulator.ps1" `
            -StartParameters "/noexplorer /noui /disableratelimiting /enableaadauthentication /partitioncount=50" `
            -Stage "Install"
        LogGroupEnd
    }

    LogGroupStart "Launching Cosmos DB Emulator"
    & "$PSScriptRoot\..\..\..\eng\common\scripts\Cosmos-Emulator.ps1" `
        -StartParameters "/noexplorer /noui /disableratelimiting /enableaadauthentication /partitioncount=50" `
        -Emulator:$EmulatorPath `
        -Stage "Launch"
    LogGroupEnd

    # Work around a temporary issue where Invoke-LoggedCommand, which calls us, needs LASTEXITCODE to be set
    $global:LASTEXITCODE = 0
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
        Invoke-LoggedCommand "docker run -d -e AZURE_COSMOS_EMULATOR_ARGS=`"/noexplorer /noui /disableratelimiting /enableaadauthentication`" -e AZURE_COSMOS_EMULATOR_PARTITION_COUNT=50 -p 8081:8081 -p 10250:10250 -p 10251:10251 -p 10252:10252 -p 10253:10253 -p 10254:10254 --name $containerName mcr.microsoft.com/cosmosdb/linux/azure-cosmos-emulator"

        # Wait for the emulator to be ready by polling the logs for a line with only "Started" on it
        $maxRetries = 30
        $retryCount = 0
        $emulatorStarted = $false
        while (-not $emulatorStarted -and $retryCount -lt $maxRetries) {
            $logs = docker logs $containerName 2>&1

            $lastLine = $logs | Select-Object -Last 1
            if ($lastLine -match "^\s*Started\s*$") {
                $emulatorStarted = $true
                Write-Host "Cosmos DB Emulator started successfully."
                break
            } elseif ($lastLine -match "^\s*Started (\d+/\d+) partitions\s*$") {
                $partitionsStarted = $matches[1]
                Write-Host "[Retry: $retryCount/$maxRetries] Emulator still starting, $partitionsStarted partitions started."
            } else {
                Write-Host "[Retry: $retryCount/$maxRetries] Emulator still starting"
            }
            $retryCount++
            Start-Sleep -Seconds 5
        }

        if (-not $emulatorStarted) {
            throw "Cosmos DB Emulator failed to start within the expected time."
        }

        LogGroupEnd
    }

    # Set environment variables for the tests
    $env:AZURE_COSMOS_CONNECTION_STRING = "emulator"

    Write-Host "Cosmos DB Emulator is running in Docker."
} else {
    throw "Docker is not available. Cannot start Cosmos DB Emulator."
}