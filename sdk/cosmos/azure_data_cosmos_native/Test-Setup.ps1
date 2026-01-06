# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cSpell:ignore HOMEDIRECTORY noui noexplorer disableratelimiting enableaadauthentication partitioncount LASTEXITCODE TEAMPROJECTID winget choco llvm

# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Test-CommandExists {
    param([Parameter(Mandatory=$true)][string]$Name)
    return $null -ne (Get-Command $Name -ErrorAction SilentlyContinue)
}

function Get-VersionFromCommand {
    param(
        [Parameter(Mandatory=$true)][string]$Command,
        [Parameter(Mandatory=$true)][string[]]$Arguments,
        [Parameter(Mandatory=$true)][string]$Regex
    )

    $output = & $Command @Arguments 2>$null
    if ($LASTEXITCODE -ne 0) {
        return $null
    }

    $m = [regex]::Match($output, $Regex)
    if ($m.Success -and $m.Groups.Count -gt 1) {
        return [Version]$m.Groups[1].Value
    }

    return $null
}

function Ensure-Rust {
    # We don't auto-install rustup here because it requires an interactive download.
    # CI images should already have Rust installed.
    if (-not (Test-CommandExists "rustc")) {
        Write-Host "Rust is not installed (rustc not found). Install Rust 1.85+ from https://rustup.rs/"
        return $false
    }

    $rustcVer = Get-VersionFromCommand -Command "rustc" -Arguments @("--version") -Regex "rustc\s+(\d+\.\d+\.\d+)"
    if ($null -eq $rustcVer) {
        Write-Host "Unable to determine rustc version."
        return $false
    }

    $min = [Version]"1.85.0"
    if ($rustcVer -lt $min) {
        Write-Host "Rust $rustcVer found, but $min+ is required. Please upgrade via rustup."
        return $false
    }

    return $true
}

function Ensure-CMake {
    $min = [Version]"3.10.0"

    if (Test-CommandExists "cmake") {
        $cmakeVer = Get-VersionFromCommand -Command "cmake" -Arguments @("--version") -Regex "cmake\s+version\s+(\d+\.\d+\.\d+)"
        if ($null -ne $cmakeVer -and $cmakeVer -ge $min) {
            return $true
        }
        Write-Host "CMake found but version is less than $min or could not be parsed."
    }

    if ($IsWindows) {
        if (Test-CommandExists "winget") {
            Write-Host "Installing CMake via winget..."
            # Id: Kitware.CMake
            Invoke-LoggedCommand "winget install --id Kitware.CMake -e --accept-source-agreements --accept-package-agreements"
        } elseif (Test-CommandExists "choco") {
            Write-Host "Installing CMake via Chocolatey..."
            Invoke-LoggedCommand "choco install cmake --yes --no-progress"
        } else {
            Write-Host "CMake is missing and neither winget nor choco is available to install it."
            return $false
        }
    } elseif (Test-CommandExists "brew") {
        Write-Host "Installing CMake via Homebrew..."
        Invoke-LoggedCommand "brew install cmake"
    } elseif (Test-CommandExists "apt-get") {
        Write-Host "Installing CMake via apt-get..."
        Invoke-LoggedCommand "sudo apt-get update"
        Invoke-LoggedCommand "sudo apt-get install -y cmake"
    } elseif (Test-CommandExists "dnf") {
        Write-Host "Installing CMake via dnf..."
        Invoke-LoggedCommand "sudo dnf install -y cmake"
    } elseif (Test-CommandExists "yum") {
        Write-Host "Installing CMake via yum..."
        Invoke-LoggedCommand "sudo yum install -y cmake"
    } else {
        Write-Host "CMake is missing and no supported package manager was detected."
        return $false
    }

    # Re-check
    if (-not (Test-CommandExists "cmake")) {
        Write-Host "CMake install completed but cmake is still not on PATH."
        return $false
    }

    $cmakeVer2 = Get-VersionFromCommand -Command "cmake" -Arguments @("--version") -Regex "cmake\s+version\s+(\d+\.\d+\.\d+)"
    if ($null -eq $cmakeVer2 -or $cmakeVer2 -lt $min) {
        Write-Host "CMake version $cmakeVer2 found, but $min+ is required."
        return $false
    }

    return $true
}

function Ensure-CCompiler {
    # For building C tests: require at least one usable C compiler.
    if ($IsWindows) {
        # Prefer MSVC (cl.exe). If not present, attempt to install VS Build Tools is too heavy for this script.
        if (Test-CommandExists "cl") {
            return $true
        }

        Write-Host "MSVC compiler (cl.exe) not found. Please install Visual Studio 2022+ with the 'Desktop development with C++' workload."
        return $false
    }

    if ((Test-CommandExists "cc") -or (Test-CommandExists "clang") -or (Test-CommandExists "gcc")) {
        return $true
    }

    if (Test-CommandExists "brew") {
        # macOS: Xcode command line tools provide clang/cc
        Write-Host "No C compiler detected. On macOS, install Xcode Command Line Tools: xcode-select --install"
        return $false
    }

    if (Test-CommandExists "apt-get") {
        Write-Host "Installing C compiler toolchain via apt-get..."
        Invoke-LoggedCommand "sudo apt-get update"
        Invoke-LoggedCommand "sudo apt-get install -y build-essential"
        return ((Test-CommandExists "cc") -or (Test-CommandExists "gcc"))
    }

    if (Test-CommandExists "dnf") {
        Write-Host "Installing C compiler toolchain via dnf..."
        Invoke-LoggedCommand "sudo dnf groupinstall -y 'Development Tools'"
        return ((Test-CommandExists "cc") -or (Test-CommandExists "gcc"))
    }

    if (Test-CommandExists "yum") {
        Write-Host "Installing C compiler toolchain via yum..."
        Invoke-LoggedCommand "sudo yum groupinstall -y 'Development Tools'"
        return ((Test-CommandExists "cc") -or (Test-CommandExists "gcc"))
    }

    Write-Host "No C compiler found and no supported package manager detected to install one."
    return $false
}

function Ensure-TestPrerequisites {
    LogGroupStart "Ensuring test prerequisites (Rust, CMake, C compiler)"

    $ok = $true

    if (-not (Ensure-Rust)) { $ok = $false }
    if (-not (Ensure-CMake)) { $ok = $false }
    if (-not (Ensure-CCompiler)) { $ok = $false }

    LogGroupEnd

    if (-not $ok) {
        # In CI we should fail loudly. Locally, keep it best-effort.
        if ($null -ne $env:SYSTEM_TEAMPROJECTID) {
            throw "Required test prerequisites are missing. See log output above for details."
        }

        Write-Host "One or more prerequisites are missing. Tests may fail."
    }
}

# Always try to ensure prerequisites before test environment setup.
Ensure-TestPrerequisites

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

    # Set environment variables for the tests
    $env:AZURE_COSMOS_CONNECTION_STRING = "emulator"
} elseif (Get-Command "docker" -ErrorAction SilentlyContinue) {
    # Docker may be installed but not running; only attempt emulator setup if it is usable.
    $dockerOk = $false
    try {
        $null = docker info 2>$null
        if ($LASTEXITCODE -eq 0) {
            $dockerOk = $true
        }
    } catch {
        $dockerOk = $false
    }

    if (-not $dockerOk) {
        Write-Host "Docker CLI found but Docker daemon is not reachable. Skipping Cosmos DB Emulator setup."
    } else {
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
    }
} else {
    # We're running a local build or we're on a macOS agent.
    # We can't run the emulator on the macOS agent, and we don't want to fail local builds because the emulator isn't installed.
    Write-Host "Cosmos DB Emulator is not available on this platform. Skipping test setup."
}

# Work around a temporary issue where Invoke-LoggedCommand, which calls us, needs LASTEXITCODE to be set
$global:LASTEXITCODE = 0