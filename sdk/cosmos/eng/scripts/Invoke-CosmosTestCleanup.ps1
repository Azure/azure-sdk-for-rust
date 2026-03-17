# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cSpell:ignore TEAMPROJECTID HOMEDIRECTORY

# Load common ES scripts
. "$PSScriptRoot\..\..\..\..\eng\common\scripts\common.ps1"

# Returns $true if the emulator is no longer reachable via HTTP.
function Test-EmulatorDown {
    try {
        Invoke-WebRequest -Uri "https://localhost:8081/" -TimeoutSec 1 -SkipCertificateCheck -UseBasicParsing -ErrorAction Stop | Out-Null
        # Got an HTTP response: emulator is still up.
        return $false
    } catch {
        $response = $_.Exception.Response
        if ($null -ne $response) {
            # An HTTP error response means the emulator process is still running.
            return $false
        }
        # No HTTP response at all: connection refused / timeout — emulator is down.
        return $true
    }
}

$ShutdownTimeout = 30

if ($IsWindows) {
    $EmulatorPath = & "$PSScriptRoot\Get-CosmosEmulatorPath.ps1"
    if ($null -eq $EmulatorPath) {
        Write-Host "Unable to confirm Cosmos DB Emulator location, skipping shutdown."
    } else {
        Write-Host "Shutting down Cosmos DB Emulator at '$EmulatorPath'."
        & $EmulatorPath /shutdown

        Write-Host "Waiting up to ${ShutdownTimeout}s for Cosmos DB Emulator to shut down..."
        $deadline = [DateTimeOffset]::Now.AddSeconds($ShutdownTimeout)
        $emulatorDown = $false
        while ([DateTimeOffset]::Now -lt $deadline) {
            # Check via /getstatus: exit code 3 means stopped.
            $statusProcess = Start-Process $EmulatorPath -ArgumentList "/getstatus" -PassThru -Wait
            if ($statusProcess.ExitCode -eq 3) {
                Write-Host "Cosmos DB Emulator has stopped (getstatus)."
                $emulatorDown = $true
                break
            }

            # Also check via HTTP: a connection failure means it is down.
            if (Test-EmulatorDown) {
                Write-Host "Cosmos DB Emulator has stopped (HTTP probe)."
                $emulatorDown = $true
                break
            }

            Start-Sleep -Seconds 2
        }

        if (-not $emulatorDown) {
            Write-Warning "Cosmos DB Emulator did not shut down within ${ShutdownTimeout} seconds."
        }
    }
} elseif (Get-Command docker -ErrorAction SilentlyContinue) {
    $containerName = "cosmosdb-emulator-test"
    $containerStatus = docker ps -a --filter "name=$containerName" --format "{{.Status}}"
    if (-not $containerStatus) {
        Write-Host "No Cosmos DB Emulator container found, skipping cleanup."
    } else {
        Write-Host "Stopping and removing Cosmos DB Emulator container '$containerName'."
        Invoke-LoggedCommand "docker rm -f $containerName"

        Write-Host "Waiting up to ${ShutdownTimeout}s for Cosmos DB Emulator to shut down..."
        $deadline = [DateTimeOffset]::Now.AddSeconds($ShutdownTimeout)
        $emulatorDown = $false
        while ([DateTimeOffset]::Now -lt $deadline) {
            # Check whether the container is still present.
            $remaining = docker ps -a --filter "name=$containerName" --format "{{.Names}}"
            if (-not $remaining) {
                Write-Host "Cosmos DB Emulator container has been removed."
                $emulatorDown = $true
                break
            }

            # Also check via HTTP: a connection failure means it is down.
            if (Test-EmulatorDown) {
                Write-Host "Cosmos DB Emulator has stopped (HTTP probe)."
                $emulatorDown = $true
                break
            }

            Start-Sleep -Seconds 2
        }

        if (-not $emulatorDown) {
            Write-Warning "Cosmos DB Emulator did not shut down within ${ShutdownTimeout} seconds."
        }
    }
} else {
    Write-Host "No Cosmos DB Emulator found to clean up."
}

# Clear env vars set by Test-Setup.ps1 so that subsequent packages in the same
# pipeline run get a clean environment and are not skipped by their own setup.
Write-Host "Clearing emulator environment variables."
$env:AZURE_COSMOS_CONNECTION_STRING = $null
$env:AZURE_COSMOS_TEST_MODE = $null
$env:AZURE_COSMOS_EMULATOR_HOST = $null
# Remove the --cfg=test_category="emulator" flag added by Test-Setup.ps1.
$env:RUSTFLAGS = $env:RUSTFLAGS -replace '\s*--cfg=test_category="emulator"', ''
Write-Host "RUSTFLAGS after cleanup: $env:RUSTFLAGS"
