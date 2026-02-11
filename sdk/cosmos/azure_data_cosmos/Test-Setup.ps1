# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cSpell:ignore HOMEDIRECTORY noui noexplorer disableratelimiting enableaadauthentication partitioncount LASTEXITCODE TEAMPROJECTID

# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

# Work around a temporary issue where Invoke-LoggedCommand, which calls us, needs LASTEXITCODE to be set
$global:LASTEXITCODE = 0

# Skip emulator setup if AZURE_COSMOS_CONNECTION_STRING is already set
if ($env:AZURE_COSMOS_CONNECTION_STRING) {
    Write-Host "AZURE_COSMOS_CONNECTION_STRING is already set. Skipping Cosmos DB Emulator setup."
    return
}

$IsAzDo = ($null -ne $env:SYSTEM_TEAMPROJECTID)
if($IsAzDo) {
    $AzDoEmulatorPath = Join-Path $env:AGENT_HOMEDIRECTORY "..\..\Program Files\Azure Cosmos DB Emulator\Microsoft.Azure.Cosmos.Emulator.exe"

    # We only run Cosmos DB tests on Windows agents in Azure DevOps
    if ($IsWindows) {
        $env:AZURE_COSMOS_TEST_MODE = "required"
    } else {
        $env:AZURE_COSMOS_TEST_MODE = "skipped"
    }
}


if ($IsWindows) {
    $EmulatorPath = $null

    if($AzDoEmulatorPath -and (Test-Path $AzDoEmulatorPath)) {
        Write-Host "Detected Azure DevOps Agent environment with Cosmos DB Emulator. Skipping Cosmos DB Emulator install."
        $EmulatorPath = $AzDoEmulatorPath
    } else {
        LogGroupStart "Installing Cosmos DB Emulator"
        & "$PSScriptRoot\..\..\..\eng\common\scripts\Cosmos-Emulator.ps1" `
            -StartParameters "/noexplorer /noui /enablepreview /EnableSqlComputeEndpoint /SqlComputePort=9999 /disableratelimiting /partitioncount=50 /consistency=Strong" `
            -Stage "Install"
        LogGroupEnd
    }

    LogGroupStart "Launching Cosmos DB Emulator"
    & "$PSScriptRoot\..\..\..\eng\common\scripts\Cosmos-Emulator.ps1" `
        -StartParameters "/noexplorer /noui /disableratelimiting /enableaadauthentication /partitioncount=50" `
        -Emulator:$EmulatorPath `
        -Stage "Launch"
    LogGroupEnd

    # Set environment variables for the tests
    $env:AZURE_COSMOS_CONNECTION_STRING = "emulator"
} elseif (Get-Command "docker" -ErrorAction SilentlyContinue) {
    Write-Host "Docker detected. Using Cosmos DB Emulator in Docker."

    # Use the centralized script to start the emulator
    & "$PSScriptRoot\..\..\..\eng\scripts\Start-CosmosEmulator.ps1" -ContainerName "cosmosdb-emulator-test" -PartitionCount 50

    # Set environment variables for the tests
    $env:AZURE_COSMOS_CONNECTION_STRING = "emulator"
} else {
    # We're running a local build or we're on a macOS agent.
    # We can't run the emulator on the macOS agent, and we don't want to fail local builds because the emulator isn't installed.
    Write-Host "Cosmos DB Emulator is not available on this platform. Skipping test setup."
}
