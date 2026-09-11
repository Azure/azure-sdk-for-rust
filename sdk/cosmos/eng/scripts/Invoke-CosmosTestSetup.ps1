# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cSpell:ignore noui noexplorer disableratelimiting enableaadauthentication partitioncount LASTEXITCODE TEAMPROJECTID

# Load common ES scripts
. "$PSScriptRoot\..\..\..\..\eng\common\scripts\common.ps1"

# Work around a temporary issue where Invoke-LoggedCommand, which calls us, needs LASTEXITCODE to be set
$global:LASTEXITCODE = 0

function Test-CosmosE2eScenarioDocuments {
    $e2eTestRoot = ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', 'e2e_tests'))
    $scenarioSchema = ([System.IO.Path]::Combine($e2eTestRoot, 'schema', 'scenario.v1.json'))
    $profileSchema = ([System.IO.Path]::Combine($e2eTestRoot, 'schema', 'profile.v1.json'))

    Get-ChildItem ([System.IO.Path]::Combine($e2eTestRoot, 'scenarios')) -Recurse -Filter '*.json' | ForEach-Object {
        if (-not (Get-Content $_.FullName -Raw | Test-Json -SchemaFile $scenarioSchema)) {
            throw "Cosmos E2E scenario failed schema validation: $($_.FullName)"
        }
    }
    Get-ChildItem ([System.IO.Path]::Combine($e2eTestRoot, 'profiles')) -Filter '*.json' | ForEach-Object {
        if (-not (Get-Content $_.FullName -Raw | Test-Json -SchemaFile $profileSchema)) {
            throw "Cosmos E2E profile failed schema validation: $($_.FullName)"
        }
    }
}

function New-CosmosE2eEmulatorConfig {
    param(
        [Parameter(Mandatory)]
        [string] $ProfileId,

        [Parameter(Mandatory)]
        [bool] $GatewayV2Enabled,

        [Parameter(Mandatory)]
        [string] $OutputDirectory
    )

    if ($ProfileId -notmatch '^[a-zA-Z][a-zA-Z0-9]*$') {
        throw "Invalid AZURE_COSMOS_E2E_PROFILE value '$ProfileId'."
    }
    $e2eTestRoot = ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', 'e2e_tests'))
    $profilePath = ([System.IO.Path]::Combine($e2eTestRoot, 'profiles', "$ProfileId.json"))
    if (-not (Test-Path $profilePath)) {
        throw "E2E profile '$ProfileId' does not exist at '$profilePath'."
    }
    $profileDocument = Get-Content $profilePath -Raw | ConvertFrom-Json
    $accountDefinitions = @($profileDocument.accounts)
    $accountDefinition = if ($env:AZURE_COSMOS_E2E_ACCOUNT) {
        @($accountDefinitions | Where-Object { $_.id -eq $env:AZURE_COSMOS_E2E_ACCOUNT })
    }
    elseif ($accountDefinitions.Count -eq 1) {
        @($accountDefinitions[0])
    }
    else {
        throw "AZURE_COSMOS_E2E_ACCOUNT is required for profile '$ProfileId'."
    }
    if ($accountDefinition.Count -ne 1) {
        throw "Profile '$ProfileId' does not contain exactly one account named '$env:AZURE_COSMOS_E2E_ACCOUNT'."
    }
    $accountDefinition = $accountDefinition[0]
    $regions = @($accountDefinition.regions | ForEach-Object {
            $region = [ordered]@{
                name = [string]$_.name
                gatewayPort = 0
            }
            if ($GatewayV2Enabled) {
                $region.gateway20Port = 0
            }
            [pscustomobject]$region
        })
    $configuration = [ordered]@{
        account = [ordered]@{
            id = "e2e-$ProfileId-$($accountDefinition.id)"
            writeMode = [string]$accountDefinition.writeMode
            consistency = [string]$accountDefinition.consistency
            perPartitionFailover = [bool]$accountDefinition.perPartitionFailover
            throttling = $false
            regions = $regions
            replication = [ordered]@{
                minDelayMs = [uint64]$accountDefinition.replication.minDelayMs
                maxDelayMs = [uint64]$accountDefinition.replication.maxDelayMs
                maxBufferedReplications = 10000
            }
        }
        management = @{ port = 0 }
        databases = @()
    }
    $mode = if ($GatewayV2Enabled) { 'v2' } else { 'v1' }
    $path = ([System.IO.Path]::Combine($OutputDirectory, "azure-cosmos-e2e-$ProfileId-$($accountDefinition.id)-$mode.json"))
    $configuration | ConvertTo-Json -Depth 10 | Set-Content $path
    return [pscustomobject]@{
        Path = $path
        AccountId = $configuration.account.id
    }
}

if (-not $env:AZURE_COSMOS_E2E_TESTS_VALIDATED) {
    Test-CosmosE2eScenarioDocuments
    $env:AZURE_COSMOS_E2E_TESTS_VALIDATED = '1'
    Write-Host 'Validated Cosmos SDK E2E scenario documents.'
}

# Append COSMOS_RUSTFLAGS (from test-resources.bicep) to RUSTFLAGS if present
if ($env:COSMOS_RUSTFLAGS) {
    $env:RUSTFLAGS = "$($env:RUSTFLAGS) $($env:COSMOS_RUSTFLAGS)"
    Write-Host "RUSTFLAGS appended with COSMOS_RUSTFLAGS: $env:RUSTFLAGS"
}

# Byte-level binary-JSON codec fuzzing (cargo-fuzz). Triggered by
# AZURE_COSMOS_FUZZ=1 (matrix variable on the Linux + nightly fuzz leg in
# sdk/cosmos/fuzz-matrix.json). Replays the committed golden vectors through
# every codec fuzz target (libFuzzer -runs=0, no mutation) to prove they still
# decode without panicking. Linux-only; the leg carries ContinueOnError=true.
# Guarded so it runs once even though Test-Setup.ps1 fires per crate.
if ($env:AZURE_COSMOS_FUZZ -eq '1' -and -not $env:AZURE_COSMOS_FUZZ_RAN) {
    $env:AZURE_COSMOS_FUZZ_RAN = '1'
    if (-not $IsLinux) {
        Write-Host "AZURE_COSMOS_FUZZ=1 but not on Linux; cargo-fuzz is Linux-only. Skipping."
    }
    else {
        Write-Host "==> Cosmos binary-JSON fuzz: golden-vector corpus validation (-runs=0)"
        & "$PSScriptRoot\Run-BinaryJsonFuzz.ps1" -ValidateOnly -Toolchain ([Channels]::Nightly())
    }
    # Strip any test_category cfg COSMOS_RUSTFLAGS injected so the subsequent
    # cargo build/test runs only the always-on offline unit tests (no
    # emulator/live-gated tests, which would panic with no connection string).
    if ($env:RUSTFLAGS -match 'test_category') {
        $env:RUSTFLAGS = ($env:RUSTFLAGS -replace '--cfg[= ]test_category="[^"]*"', '' -replace '\s+', ' ').Trim()
        Write-Host "Stripped test_category from RUSTFLAGS on fuzz leg: '$env:RUSTFLAGS'"
    }
    # No live account is provisioned on the fuzz leg. Mark the test mode as
    # skipped so `resolve_test_env` treats the unset connection string as a skip
    # (not a fatal "required" panic) when the archetype's subsequent
    # `cargo test` builds the account-backed driver tests on this Azure
    # Pipelines job (SYSTEM_TEAMPROJECTID is set).
    $env:AZURE_COSMOS_TEST_MODE = 'skipped'
    return
}

# Hosted in-memory emulator path. The additional CI matrix sets one of the two
# flavors below so the existing emulator suites run against both Gateway V1
# and Gateway 2.0 over cleartext HTTP/2.
if ($env:AZURE_COSMOS_E2E_PROFILE -and
    $env:AZURE_COSMOS_EMULATOR_FLAVOR -notin @('inmemory-v1', 'inmemory-v2')) {
    throw 'AZURE_COSMOS_E2E_PROFILE requires AZURE_COSMOS_EMULATOR_FLAVOR to be inmemory-v1 or inmemory-v2.'
}

if ($env:AZURE_COSMOS_EMULATOR_FLAVOR -in @('inmemory-v1', 'inmemory-v2')) {
    $repoRoot = (Resolve-Path ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..', '..'))).Path
    $runDirectory = if ($env:AZURE_COSMOS_INMEMORY_RUN_DIRECTORY) {
        $env:AZURE_COSMOS_INMEMORY_RUN_DIRECTORY
    }
    else {
        ([System.IO.Path]::Combine(
                [System.IO.Path]::GetTempPath(),
                "azure-data-cosmos-emulator-$([System.Guid]::NewGuid().ToString('N'))"))
    }
    New-Item -ItemType Directory -Path $runDirectory -Force | Out-Null
    $env:AZURE_COSMOS_INMEMORY_RUN_DIRECTORY = $runDirectory
    $configuration = if ($env:AZURE_COSMOS_EMULATOR_FLAVOR -eq 'inmemory-v2') {
        [System.IO.Path]::Combine($repoRoot, 'sdk', 'cosmos', 'azure_data_cosmos_emulator', 'config', 'ci-gateway-v2.json')
    }
    else {
        [System.IO.Path]::Combine($repoRoot, 'sdk', 'cosmos', 'azure_data_cosmos_emulator', 'config', 'ci-gateway-v1.json')
    }
    $ready = $false
    $expectedGateway20 = $env:AZURE_COSMOS_EMULATOR_FLAVOR -eq 'inmemory-v2'
    $expectedAccountId = $null
    if ($env:AZURE_COSMOS_E2E_PROFILE) {
        $e2eConfiguration = New-CosmosE2eEmulatorConfig `
            -ProfileId $env:AZURE_COSMOS_E2E_PROFILE `
            -GatewayV2Enabled $expectedGateway20 `
            -OutputDirectory $runDirectory
        $configuration = $e2eConfiguration.Path
        $expectedAccountId = $e2eConfiguration.AccountId
    }
    else {
        $e2eConfiguration = New-CosmosE2eEmulatorConfig `
            -ProfileId 'hostedEmulatorSmoke' `
            -GatewayV2Enabled $expectedGateway20 `
            -OutputDirectory $runDirectory
        $configuration = $e2eConfiguration.Path
        $expectedAccountId = $e2eConfiguration.AccountId
    }
    $managementEndpoint = $env:AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT
    $accountEndpoint = $env:AZURE_COSMOS_INMEMORY_ACCOUNT_ENDPOINT
    if ($managementEndpoint -and $accountEndpoint) {
        $healthUrl = ([System.Uri]::new([System.Uri]$managementEndpoint, 'health')).AbsoluteUri
        try {
            $response = Invoke-WebRequest -Uri $healthUrl -UseBasicParsing -TimeoutSec 2 -ErrorAction Stop
            $health = $response.Content | ConvertFrom-Json
            $ready = $response.StatusCode -eq 200 -and $health.gateway20Enabled -eq $expectedGateway20
            if ($ready -and $expectedAccountId) {
                $accountUrl = ([System.Uri]::new([System.Uri]$managementEndpoint, 'account')).AbsoluteUri
                $accountResponse = Invoke-WebRequest -Uri $accountUrl -UseBasicParsing -TimeoutSec 2 -ErrorAction Stop
                $account = $accountResponse.Content | ConvertFrom-Json
                $ready = $accountResponse.StatusCode -eq 200 -and $account.id -eq $expectedAccountId
            }
        }
        catch {
            $ready = $false
        }
    }

    if (-not $ready) {
        if ($env:AZURE_COSMOS_INMEMORY_EMULATOR_PID) {
            Get-Process -Id ([int]$env:AZURE_COSMOS_INMEMORY_EMULATOR_PID) -ErrorAction SilentlyContinue |
            Stop-Process -Force -ErrorAction SilentlyContinue
            $env:AZURE_COSMOS_INMEMORY_EMULATOR_PID = $null
        }
    }

    if (-not $ready) {
        LogGroupStart "Testing and building hosted Cosmos DB in-memory emulator"
        Push-Location $repoRoot
        try {
            Invoke-LoggedCommand 'cargo test -p azure_data_cosmos_emulator --all-features'
            Invoke-LoggedCommand 'cargo build -p azure_data_cosmos_emulator'
        }
        finally {
            Pop-Location
        }
        LogGroupEnd

        $executableName = if ($IsWindows) {
            'azure_data_cosmos_emulator.exe'
        }
        else {
            'azure_data_cosmos_emulator'
        }
        $executable = [System.IO.Path]::Combine($repoRoot, 'target', 'debug', $executableName)
        $stdout = [System.IO.Path]::Combine($runDirectory, 'stdout.log')
        $stderr = [System.IO.Path]::Combine($runDirectory, 'stderr.log')
        Remove-Item $stdout, $stderr -Force -ErrorAction SilentlyContinue

        LogGroupStart "Starting hosted Cosmos DB in-memory emulator"
        $process = Start-Process `
            -FilePath $executable `
            -ArgumentList @('--config', $configuration) `
            -RedirectStandardOutput $stdout `
            -RedirectStandardError $stderr `
            -PassThru
        $env:AZURE_COSMOS_INMEMORY_EMULATOR_PID = $process.Id.ToString()
        Write-Host "Started hosted emulator process $($process.Id) using '$configuration'."

        $deadline = (Get-Date).AddSeconds(60)
        $readyRecord = $null
        while ((Get-Date) -lt $deadline) {
            if ($process.HasExited) {
                break
            }
            if (-not $readyRecord -and (Test-Path $stdout)) {
                $readyLine = Get-Content $stdout -ErrorAction SilentlyContinue | Select-Object -Last 1
                if ($readyLine) {
                    try {
                        $candidate = $readyLine | ConvertFrom-Json -ErrorAction Stop
                        if ($candidate.event -eq 'ready') {
                            $readyRecord = $candidate
                            $managementEndpoint = [string]$readyRecord.managementEndpoint
                            $accountEndpoint = [string]$readyRecord.accountEndpoint
                            # V1 omits this optional property; direct access throws under strict mode.
                            $hasGateway20 = @($readyRecord.regions | Where-Object {
                                    $gateway20Endpoint = $_.PSObject.Properties['gateway20Endpoint']
                                    $null -ne $gateway20Endpoint -and
                                    -not [string]::IsNullOrWhiteSpace([string]$gateway20Endpoint.Value)
                                }).Count -gt 0
                            if (-not $managementEndpoint -or -not $accountEndpoint -or $hasGateway20 -ne $expectedGateway20) {
                                throw 'Hosted emulator ready record does not match the requested gateway mode.'
                            }
                            $healthUrl = ([System.Uri]::new([System.Uri]$managementEndpoint, 'health')).AbsoluteUri
                        }
                    }
                    catch {
                        $readyRecord = $null
                        Write-Host "Waiting for a valid hosted emulator ready record: $($_.Exception.Message)"
                    }
                }
            }
            if ($readyRecord) {
                try {
                    $response = Invoke-WebRequest -Uri $healthUrl -UseBasicParsing -TimeoutSec 2 -ErrorAction Stop
                    $health = $response.Content | ConvertFrom-Json
                    if ($response.StatusCode -eq 200 -and $health.gateway20Enabled -eq $expectedGateway20) {
                        $ready = $true
                        break
                    }
                }
                catch {
                    Write-Host "Waiting for hosted in-memory emulator readiness: $($_.Exception.Message)"
                }
            }
            Start-Sleep -Seconds 1
        }
        if (-not $ready) {
            Get-Content $stdout, $stderr -ErrorAction SilentlyContinue | Write-Host
            if (-not $process.HasExited) {
                $process | Stop-Process -Force -ErrorAction SilentlyContinue
            }
            throw 'Hosted Cosmos DB in-memory emulator did not become ready within 60 seconds.'
        }
        LogGroupEnd
    }

    $env:AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT = $managementEndpoint
    $env:AZURE_COSMOS_INMEMORY_ACCOUNT_ENDPOINT = $accountEndpoint
    $emulatorKey = 'C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw=='
    $env:AZURE_COSMOS_CONNECTION_STRING = "AccountEndpoint=$accountEndpoint;AccountKey=$emulatorKey;"
    $env:AZURE_COSMOS_TEST_MODE = 'required'
    $env:RUSTFLAGS = $env:RUSTFLAGS -replace '\s*--cfg=test_category="[^"]*"', ''
    if ($env:AZURE_COSMOS_E2E_PROFILE) {
        $env:RUSTFLAGS = "$($env:RUSTFLAGS) --cfg=test_category=`"e2e`""
    }
    else {
        $env:RUSTFLAGS = "$($env:RUSTFLAGS) --cfg=test_category=`"emulator_inmemory`""
    }
    if ($expectedGateway20 -and -not $env:AZURE_COSMOS_E2E_PROFILE) {
        $env:RUSTFLAGS = "$($env:RUSTFLAGS) --cfg=test_category=`"emulator_inmemory_gateway_v2`""
    }
    $env:RUST_TEST_THREADS = '1'
    Write-Host "Hosted emulator is ready; RUSTFLAGS set to: $env:RUSTFLAGS"
    return
}

# Vnext (Linux) emulator path. Triggered by AZURE_COSMOS_EMULATOR_FLAVOR=vnext
# (set as a matrix variable on the cosmos vnext leg in
# sdk/cosmos/vnext-emulator-matrix.json). Manages the Docker container
# lifecycle locally so the pipeline doesn't need any service-specific steps.
# The corresponding teardown happens at the agent layer (1ES agents are
# ephemeral) — we deliberately do NOT remove the container in cleanup so
# subsequent crates in the same Test-Packages.ps1 loop reuse it.
if ($env:AZURE_COSMOS_EMULATOR_FLAVOR -eq 'vnext') {
    $vnextContainerName = 'cosmosdb-emulator-vnext'
    $vnextImage = 'mcr.microsoft.com/cosmosdb/linux/azure-cosmos-emulator:vnext-preview'
    # Well-known emulator master key — same value used by the legacy Windows
    # emulator and accepted by the vnext image. Safe to commit; not a secret.
    $vnextKey = 'C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw=='

    if (-not (Get-Command 'docker' -ErrorAction SilentlyContinue)) {
        throw "AZURE_COSMOS_EMULATOR_FLAVOR=vnext requires Docker but the docker CLI was not found on PATH."
    }

    # Idempotent: only start the container if it's not already running. This
    # matters because Test-Packages.ps1 calls Test-Setup.ps1 once per crate
    # (azure_data_cosmos and azure_data_cosmos_driver in the cosmos service).
    $existing = docker ps --filter "name=$vnextContainerName" --format "{{.Names}}" 2>$null
    if ($existing -eq $vnextContainerName) {
        Write-Host "Cosmos DB vnext emulator container '$vnextContainerName' is already running."
    }
    else {
        LogGroupStart "Starting Cosmos DB vnext emulator (Docker)"
        Invoke-LoggedCommand "docker pull $vnextImage"
        Invoke-LoggedCommand "docker run -d --name $vnextContainerName --publish 8081:8081 --publish 8080:8080 -e ENABLE_EXPLORER=false $vnextImage"

        # Best-effort readiness probe. Tests will surface a clearer failure
        # than a stuck step if the probe never succeeds.
        $deadline = (Get-Date).AddSeconds(180)
        $probeUrl = 'http://localhost:8080/ready'
        $ready = $false
        while ((Get-Date) -lt $deadline) {
            try {
                $resp = Invoke-WebRequest -Uri $probeUrl -UseBasicParsing -TimeoutSec 5 -ErrorAction Stop
                if ($resp.StatusCode -ge 200 -and $resp.StatusCode -lt 300) {
                    Write-Host "vnext emulator reports ready (HTTP $($resp.StatusCode))."
                    $ready = $true
                    break
                }
            }
            catch {
                Write-Host "Waiting for vnext emulator readiness: $($_.Exception.Message)"
            }
            Start-Sleep -Seconds 5
        }
        if (-not $ready) {
            Write-Warning "vnext emulator readiness probe did not succeed within 180s - proceeding anyway."
            docker logs --tail 50 $vnextContainerName 2>&1 | Write-Host
        }
        LogGroupEnd
    }

    if (-not $env:AZURE_COSMOS_CONNECTION_STRING) {
        $env:AZURE_COSMOS_CONNECTION_STRING = "AccountEndpoint=http://localhost:8081;AccountKey=$vnextKey;"
        Write-Host "Set AZURE_COSMOS_CONNECTION_STRING to vnext emulator endpoint."
    }
    $env:RUSTFLAGS = "$($env:RUSTFLAGS) --cfg=test_category=`"emulator_vnext`" --cfg=cosmos_aad_supported"
    Write-Host "RUSTFLAGS set to: $env:RUSTFLAGS"
    $env:RUST_TEST_THREADS = "1"
    return
}

# Skip emulator setup if AZURE_COSMOS_CONNECTION_STRING is already set
if ($env:AZURE_COSMOS_CONNECTION_STRING) {
    Write-Host "AZURE_COSMOS_CONNECTION_STRING is already set. Skipping Cosmos DB Emulator setup."
    return
}

$IsAzDo = ($null -ne $env:SYSTEM_TEAMPROJECTID)
if ($IsAzDo) {
    # We only run Cosmos DB tests on Windows agents in Azure DevOps
    if ($IsWindows) {
        $env:AZURE_COSMOS_TEST_MODE = "required"
    }
    else {
        $env:AZURE_COSMOS_TEST_MODE = "skipped"
        Write-Host "Skipping Cosmos DB Emulator setup on non-Windows Azure DevOps agents."
        return
    }
}


if ($IsWindows) {
    # Check for emulator in known locations
    $EmulatorPath = & "$PSScriptRoot\Get-CosmosEmulatorPath.ps1"
    if ($null -ne $EmulatorPath) {
        Write-Host "Found Cosmos DB Emulator at '$EmulatorPath'. Skipping Cosmos DB Emulator install."
    }
    else {
        LogGroupStart "Installing Cosmos DB Emulator"
        & "$PSScriptRoot\..\..\..\..\eng\common\scripts\Cosmos-Emulator.ps1" `
            -StartParameters "/noexplorer /noui /enablepreview /EnableSqlComputeEndpoint /SqlComputePort=9999 /disableratelimiting /partitioncount=50 /consistency=Strong /enableaadauthentication" `
            -Stage "Install"
        LogGroupEnd
    }

    LogGroupStart "Launching Cosmos DB Emulator"
    & "$PSScriptRoot\..\..\..\..\eng\common\scripts\Cosmos-Emulator.ps1" `
        -StartParameters "/noexplorer /noui /enablepreview /EnableSqlComputeEndpoint /SqlComputePort=9999 /disableratelimiting /partitioncount=50 /consistency=Strong /enableaadauthentication" `
        -Emulator:$EmulatorPath `
        -Stage "Launch"
    LogGroupEnd

    # Probe the emulator endpoint to verify it is responding
    LogGroupStart "Probing Cosmos DB Emulator endpoint"
    $emulatorUrl = "https://localhost:8081/"
    $maxProbeRetries = 30
    $probeRetry = 0
    $emulatorReady = $false
    while (-not $emulatorReady -and $probeRetry -lt $maxProbeRetries) {
        try {
            $response = Invoke-WebRequest -Uri $emulatorUrl -SkipCertificateCheck -UseBasicParsing -ErrorAction Stop
            Write-Host "Emulator responded with status $($response.StatusCode)."
            $emulatorReady = $true
        }
        catch {
            # Some exceptions (e.g. connection refused) have no Response property.
            $response = $null
            if ($_.Exception.PSObject.Properties['Response']) {
                $response = $_.Exception.Response
            }
            if ($null -ne $response -and $null -ne $response.StatusCode) {
                $statusCode = $response.StatusCode.value__
                if ($statusCode -ge 400 -and $statusCode -lt 500) {
                    # 4xx means the emulator is up but rejecting unauthenticated requests
                    Write-Host "Emulator responded with status $statusCode (expected auth failure). Emulator is ready."
                    $emulatorReady = $true
                    continue
                }
            }
            # No HTTP response or non-4xx status: treat as retryable failure
            $probeRetry++
            Write-Host "[Retry: $probeRetry/$maxProbeRetries] Emulator not yet responding. Exception: $($_.Exception.Message)"
            Start-Sleep -Seconds 5
        }
    }
    if (-not $emulatorReady) {
        LogError "Cosmos DB Emulator failed to respond at $emulatorUrl after $maxProbeRetries retries."
        exit 1
    }
    LogGroupEnd

    # Set environment variables for the tests
    $env:AZURE_COSMOS_CONNECTION_STRING = "emulator"
    $env:RUSTFLAGS = "$($env:RUSTFLAGS) --cfg=test_category=`"emulator`" --cfg=cosmos_aad_supported"
    Write-Host "RUSTFLAGS set to: $env:RUSTFLAGS"

    # Run tests single-threaded to avoid env var contamination from proxy tests.
    $env:RUST_TEST_THREADS = "1"
}
elseif (Get-Command "docker" -ErrorAction SilentlyContinue) {
    Write-Host "Docker detected. Using Cosmos DB Emulator in Docker."

    # Check if the emulator is already running
    $existingContainer = docker ps --filter "name=cosmosdb-emulator-test" --format "{{.Names}}"
    if ($existingContainer -eq "cosmosdb-emulator-test") {
        Write-Host "Cosmos DB Emulator container is already running."
    }
    else {
        LogGroupStart "Starting Cosmos DB Emulator in Docker"
        # Start Cosmos DB Emulator in Docker
        $containerName = "cosmosdb-emulator-test"
        Invoke-LoggedCommand "docker run -d -e AZURE_COSMOS_EMULATOR_IP_ADDRESS_OVERRIDE=127.0.0.1 -e AZURE_COSMOS_EMULATOR_ARGS=`"/noexplorer /noui /disableratelimiting /enableaadauthentication`" -e AZURE_COSMOS_EMULATOR_PARTITION_COUNT=50 -p 8081:8081 -p 10250:10250 -p 10251:10251 -p 10252:10252 -p 10253:10253 -p 10254:10254 --name $containerName mcr.microsoft.com/cosmosdb/linux/azure-cosmos-emulator"

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
            }
            elseif ($lastLine -match "^\s*Started (\d+/\d+) partitions\s*$") {
                $partitionsStarted = $matches[1]
                Write-Host "[Retry: $retryCount/$maxRetries] Emulator still starting, $partitionsStarted partitions started."
            }
            else {
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
    $env:RUSTFLAGS = "$($env:RUSTFLAGS) --cfg=test_category=`"emulator`" --cfg=cosmos_aad_supported"
    Write-Host "RUSTFLAGS set to: $env:RUSTFLAGS"

    # Run tests single-threaded to avoid env var contamination from proxy tests.
    $env:RUST_TEST_THREADS = "1"

    Write-Host "Cosmos DB Emulator is running in Docker."
}
else {
    # We're running a local build or we're on a macOS agent.
    # We can't run the emulator on the macOS agent, and we don't want to fail local builds because the emulator isn't installed.
    Write-Host "Cosmos DB Emulator is not available on this platform. Skipping test setup."
}
