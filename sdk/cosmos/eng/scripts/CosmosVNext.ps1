# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

function Invoke-CosmosVNextDocker {
    param([string[]] $Arguments)

    # Capture output so callers can validate inspect results and include Docker errors.
    $output = & docker @Arguments 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw "docker $($Arguments[0]) failed (exit $LASTEXITCODE): $output"
    }
    return ($output -join "`n")
}

function Get-CosmosVNextFlags {
    param([string] $Gateway, [string] $Encoding)

    if ($Gateway -cnotin @('v1', 'v2') -or $Encoding -cnotin @('text', 'binary')) {
        throw 'Set AZURE_COSMOS_VNEXT_GATEWAY to v1/v2 and AZURE_COSMOS_VNEXT_ENCODING to text/binary.'
    }
    $category = if ($Gateway -eq 'v1') { 'emulator_vnext' } else { 'emulator_vnext_gateway_v2' }
    $flags = "--cfg=test_category=`"$category`""
    if ($Encoding -eq 'binary') {
        $flags += ' --cfg=test_category="emulator_vnext_binary"'
    }
    return $flags
}

function Start-CosmosVNext {
    param([string] $Gateway, [string] $Encoding)

    $flags = Get-CosmosVNextFlags -Gateway $Gateway -Encoding $Encoding
    $name = 'cosmosdb-emulator-vnext'
    $image = 'mcr.microsoft.com/cosmosdb/linux/azure-cosmos-emulator@sha256:4535ddbd27bba762f3e068f7585feb0022a2559ceccc9394b23aa38d91d77c38'
    $endpoint = 'https://localhost:8081/'
    # Public emulator key, not a cloud credential.
    $key = 'C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw=='
    $connectionString = "AccountEndpoint=$endpoint;AccountKey=$key;"
    if ($env:AZURE_COSMOS_CONNECTION_STRING -and $env:AZURE_COSMOS_CONNECTION_STRING -cne $connectionString) {
        throw 'vNext jobs must use their owned local emulator; remove the conflicting connection string.'
    }
    if ($env:RUSTFLAGS -match 'test_category') {
        throw 'vNext setup requires clean test-category flags; do not combine live and emulator categories.'
    }
    if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
        throw 'vNext setup requires Docker.'
    }

    $owner = if ($env:SYSTEM_JOBID) { $env:SYSTEM_JOBID } else { "local-$PID" }
    $existing = Invoke-CosmosVNextDocker @('ps', '-a', '--filter', "name=^/$name$", '--format', '{{.Names}}')
    if ($existing) {
        $container = @(Invoke-CosmosVNextDocker @('inspect', $name) | ConvertFrom-Json)[0]
        if ($container.Config.Labels.'cosmos-vnext-owner' -ne $owner -or
            $container.Config.Image -ne $image -or -not $container.State.Running) {
            throw "Refusing to reuse stopped, unowned, or mismatched container '$name'."
        }
        foreach ($entry in @('PROTOCOL=https', 'THIN_CLIENT_ENABLED=true', 'THIN_CLIENT_PROTOCOL=https', 'ENABLE_EXPLORER=false')) {
            if ($entry -notin $container.Config.Env) {
                throw "vNext container '$name' has incompatible configuration."
            }
        }
        foreach ($port in @('8081', '8080', '10250')) {
            $binding = @($container.HostConfig.PortBindings."$port/tcp")
            if ($binding.Count -ne 1 -or $binding[0].HostIp -ne '127.0.0.1' -or $binding[0].HostPort -ne $port) {
                throw "vNext container '$name' has incompatible port bindings."
            }
        }
    }
    else {
        Write-Host "Pulling vNext image $image"
        Invoke-CosmosVNextDocker @('pull', $image) | Write-Host
        Invoke-CosmosVNextDocker @(
            'run', '-d', '--name', $name, '--label', "cosmos-vnext-owner=$owner",
            '-p', '127.0.0.1:8081:8081', '-p', '127.0.0.1:8080:8080', '-p', '127.0.0.1:10250:10250',
            '-e', 'PROTOCOL=https', '-e', 'THIN_CLIENT_ENABLED=true', '-e', 'THIN_CLIENT_PROTOCOL=https',
            '-e', 'ENABLE_EXPLORER=false', '-e', 'ENABLE_TELEMETRY=false', $image
        ) | Write-Host
    }

    Write-Host "vNext gateway=$Gateway encoding=$Encoding image=$image"
    Invoke-CosmosVNextDocker @('inspect', '--format', '{{.Image}}', $name) | Write-Host
    $ready = $false
    for ($attempt = 0; $attempt -lt 36; $attempt++) {
        try {
            $health = Invoke-RestMethod -Uri 'http://localhost:8080/ready' -TimeoutSec 5 -ErrorAction Stop
            if ($health.ready -eq $true -and $health.protocol -eq 'https') {
                $ready = $true
                break
            }
        }
        catch {
            Write-Host "Waiting for vNext readiness: $($_.Exception.Message)"
        }
        Start-Sleep -Seconds 5
    }
    if (-not $ready) {
        Invoke-CosmosVNextDocker @('logs', '--tail', '100', $name) | Write-Host
        throw 'vNext did not become ready with HTTPS after 36 probes.'
    }

    $env:AZURE_COSMOS_CONNECTION_STRING = $connectionString
    $env:AZURE_COSMOS_EMULATOR_SERVER_CERT_VALIDATION_DISABLED = 'true'
    $env:AZURE_COSMOS_TEST_MODE = 'required'
    $env:RUSTFLAGS = "$($env:RUSTFLAGS) $flags".Trim()
    $env:RUST_TEST_THREADS = '1'
}

function Complete-CosmosVNext {
    $name = 'cosmosdb-emulator-vnext'
    $existing = Invoke-CosmosVNextDocker @('ps', '-a', '--filter', "name=^/$name$", '--format', '{{.Names}}')
    if (-not $existing) { return }
    $owner = if ($env:SYSTEM_JOBID) { $env:SYSTEM_JOBID } else { "local-$PID" }
    $container = @(Invoke-CosmosVNextDocker @('inspect', $name) | ConvertFrom-Json)[0]
    if ($container.Config.Labels.'cosmos-vnext-owner' -ne $owner) {
        throw "Refusing to remove unowned container '$name'."
    }
    try {
        Invoke-CosmosVNextDocker @('inspect', '--format', '{{.Config.Image}} {{.Image}} {{.State.Status}}', $name) | Write-Host
        Invoke-CosmosVNextDocker @('logs', $name) | Write-Host
    }
    finally {
        Invoke-CosmosVNextDocker @('rm', '-f', $name) | Write-Host
    }
}
