# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Cosmos vNext setup' {
    BeforeAll {
        . ([System.IO.Path]::Combine($PSScriptRoot, '..', 'CosmosVNext.ps1'))
        function docker {}
        $variables = @(
            'AZURE_COSMOS_CONNECTION_STRING', 'AZURE_COSMOS_TEST_MODE',
            'AZURE_COSMOS_EMULATOR_SERVER_CERT_VALIDATION_DISABLED', 'RUSTFLAGS', 'RUST_TEST_THREADS',
            'SYSTEM_JOBID', 'AZURE_COSMOS_AUTH_MODE', 'ACCOUNT_HOST', 'CARGO_ENCODED_RUSTFLAGS',
            'AZURE_COSMOS_EMULATOR_FLAVOR', 'AZURE_COSMOS_EMULATOR_HOST',
            'AZURE_COSMOS_INMEMORY_EMULATOR_PID', 'AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT',
            'AZURE_COSMOS_INMEMORY_ACCOUNT_ENDPOINT', 'AZURE_COSMOS_INMEMORY_RUN_DIRECTORY',
            'AZURE_COSMOS_INMEMORY_RUN_ID'
        )
    }

    BeforeEach {
        $saved = @{}
        foreach ($variable in $variables) {
            $saved[$variable] = [Environment]::GetEnvironmentVariable($variable)
            [Environment]::SetEnvironmentVariable($variable, $null)
        }
        $env:SYSTEM_JOBID = 'vnext-test-job'
        $container = @{
            Config = @{
                Image = 'mcr.microsoft.com/cosmosdb/linux/azure-cosmos-emulator@sha256:4535ddbd27bba762f3e068f7585feb0022a2559ceccc9394b23aa38d91d77c38'
                Labels = @{ 'cosmos-vnext-owner' = 'vnext-test-job' }
                Env = @('PROTOCOL=https', 'THIN_CLIENT_ENABLED=true', 'THIN_CLIENT_PROTOCOL=https', 'ENABLE_EXPLORER=false')
            }
            State = @{ Running = $true }
            HostConfig = @{ PortBindings = @{
                '8081/tcp' = @(@{ HostIp = '127.0.0.1'; HostPort = '8081' })
                '8080/tcp' = @(@{ HostIp = '127.0.0.1'; HostPort = '8080' })
                '10250/tcp' = @(@{ HostIp = '127.0.0.1'; HostPort = '10250' })
            } }
        }
        Mock Invoke-CosmosVNextDocker {
            if ($Arguments[0] -eq 'ps') { return '' }
            if ($Arguments[0] -eq 'inspect' -and $Arguments.Count -eq 2) {
                return ConvertTo-Json -InputObject @($container) -Depth 10
            }
            return 'docker-output'
        }
        Mock Invoke-RestMethod { @{ ready = $true; protocol = 'https' } }
        Mock Start-Sleep {}
        Mock Write-Host {}
    }

    AfterEach {
        foreach ($variable in $variables) {
            [Environment]::SetEnvironmentVariable($variable, $saved[$variable])
        }
    }

    It 'selects exactly the intended categories for all four cells' {
        Get-CosmosVNextFlags v1 text | Should -Be '--cfg=test_category="emulator_vnext"'
        Get-CosmosVNextFlags v1 binary | Should -Be '--cfg=test_category="emulator_vnext" --cfg=test_category="emulator_vnext_binary"'
        Get-CosmosVNextFlags v2 text | Should -Be '--cfg=test_category="emulator_vnext_gateway_v2"'
        Get-CosmosVNextFlags v2 binary | Should -Be '--cfg=test_category="emulator_vnext_gateway_v2" --cfg=test_category="emulator_vnext_binary"'
    }

    It 'rejects missing or invalid axes before starting Docker' {
        { Start-CosmosVNext '' binary } | Should -Throw '*AZURE_COSMOS_VNEXT_GATEWAY*'
        { Start-CosmosVNext v1 invalid } | Should -Throw '*AZURE_COSMOS_VNEXT_ENCODING*'
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 0
    }

    It 'rejects a conflicting account and live categories' {
        $env:AZURE_COSMOS_CONNECTION_STRING = 'AccountEndpoint=https://remote.invalid/;'
        { Start-CosmosVNext v1 text } | Should -Throw '*conflicting connection string*'
        $env:AZURE_COSMOS_CONNECTION_STRING = $null
        $env:RUSTFLAGS = '--cfg=test_category="gateway_v2"'
        { Start-CosmosVNext v2 text } | Should -Throw '*clean test-category*'
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 0
    }

    It 'starts a pinned HTTPS emulator and requires test execution' {
        Start-CosmosVNext v2 binary
        $env:AZURE_COSMOS_TEST_MODE | Should -Be 'required'
        $env:AZURE_COSMOS_CONNECTION_STRING | Should -BeLike 'AccountEndpoint=https://localhost:8081/;*'
        $env:AZURE_COSMOS_EMULATOR_SERVER_CERT_VALIDATION_DISABLED | Should -Be 'true'
        $env:RUSTFLAGS | Should -Be '--cfg=test_category="emulator_vnext_gateway_v2" --cfg=test_category="emulator_vnext_binary"'
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 1 -ParameterFilter {
            $Arguments[0] -eq 'run' -and '127.0.0.1:10250:10250' -in $Arguments -and
            'THIN_CLIENT_PROTOCOL=https' -in $Arguments -and
            $Arguments[-1] -like '*@sha256:*'
        }
    }

    It 'rejects inherited AAD or invalid authentication before Docker' -ForEach @('aad', 'AAD', 'invalid') {
        $env:AZURE_COSMOS_AUTH_MODE = $_
        { Start-CosmosVNext v1 text } | Should -Throw '*AZURE_COSMOS_AUTH_MODE*'
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 0
    }

    It 'rejects an inherited account endpoint even with key authentication' {
        $env:AZURE_COSMOS_AUTH_MODE = 'key'
        $env:ACCOUNT_HOST = 'https://live-account.documents.azure.com/'
        { Start-CosmosVNext v2 binary } | Should -Throw '*ACCOUNT_HOST*'
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 0
    }

    It 'permits explicit key authentication' {
        $env:AZURE_COSMOS_AUTH_MODE = 'KEY'
        Start-CosmosVNext v1 text
        $env:AZURE_COSMOS_TEST_MODE | Should -Be 'required'
    }

    It 'rejects encoded Rust flags even when they contain no category' {
        $env:CARGO_ENCODED_RUSTFLAGS = '-Dwarnings'
        { Start-CosmosVNext v2 binary } | Should -Throw '*CARGO_ENCODED_RUSTFLAGS*'
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 0
    }

    It 'reuses the container after real per-package cleanup' {
        Mock Invoke-CosmosVNextDocker { 'cosmosdb-emulator-vnext' } -ParameterFilter { $Arguments[0] -eq 'ps' }
        $env:AZURE_COSMOS_EMULATOR_FLAVOR = 'vnext'
        Start-CosmosVNext v1 binary
        $connectionString = $env:AZURE_COSMOS_CONNECTION_STRING
        & ([System.IO.Path]::Combine($PSScriptRoot, '..', 'Invoke-CosmosTestCleanup.ps1'))
        $env:RUSTFLAGS | Should -BeNullOrEmpty
        $env:AZURE_COSMOS_TEST_MODE | Should -BeNullOrEmpty
        $env:AZURE_COSMOS_EMULATOR_SERVER_CERT_VALIDATION_DISABLED | Should -BeNullOrEmpty
        $env:AZURE_COSMOS_CONNECTION_STRING | Should -Be $connectionString
        Start-CosmosVNext v2 text
        $env:AZURE_COSMOS_TEST_MODE | Should -Be 'required'
        $env:RUSTFLAGS | Should -Be '--cfg=test_category="emulator_vnext_gateway_v2"'
        Should -Invoke Invoke-RestMethod -Exactly 2
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 0 -ParameterFilter { $Arguments[0] -in @('run', 'rm', 'pull') }
    }

    It 'rechecks readiness without restarting the owned container' {
        Mock Invoke-CosmosVNextDocker { 'cosmosdb-emulator-vnext' } -ParameterFilter { $Arguments[0] -eq 'ps' }
        Start-CosmosVNext v1 text
        $env:RUSTFLAGS = ''
        Start-CosmosVNext v1 text
        Should -Invoke Invoke-RestMethod -Exactly 2
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 0 -ParameterFilter { $Arguments[0] -in @('run', 'pull') }
    }

    It 'rejects a stopped, unowned or differently configured container' {
        Mock Invoke-CosmosVNextDocker { 'cosmosdb-emulator-vnext' } -ParameterFilter { $Arguments[0] -eq 'ps' }
        $container.State.Running = $false
        { Start-CosmosVNext v1 text } | Should -Throw '*Refusing to reuse*'
        $container.State.Running = $true
        $container.Config.Labels.'cosmos-vnext-owner' = 'other-job'
        { Start-CosmosVNext v1 text } | Should -Throw '*Refusing to reuse*'
        $container.Config.Labels.'cosmos-vnext-owner' = 'vnext-test-job'
        $container.HostConfig.PortBindings.'10250/tcp'[0].HostPort = '10251'
        { Start-CosmosVNext v1 text } | Should -Throw '*incompatible port bindings*'
    }

    It 'fails readiness instead of continuing into tests' {
        Mock Invoke-RestMethod { throw 'not ready' }
        { Start-CosmosVNext v2 binary } | Should -Throw '*after 36 probes*'
        Should -Invoke Invoke-RestMethod -Exactly 36
        $env:AZURE_COSMOS_CONNECTION_STRING | Should -BeNullOrEmpty
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 1 -ParameterFilter { $Arguments[0] -eq 'logs' }
    }

    It 'collects logs before removing only the owned container' {
        Mock Invoke-CosmosVNextDocker { 'cosmosdb-emulator-vnext' } -ParameterFilter { $Arguments[0] -eq 'ps' }
        Complete-CosmosVNext
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 1 -ParameterFilter { $Arguments[0] -eq 'logs' }
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 1 -ParameterFilter { $Arguments[0] -eq 'rm' }
    }

    It 'never removes another jobs container' {
        Mock Invoke-CosmosVNextDocker { 'cosmosdb-emulator-vnext' } -ParameterFilter { $Arguments[0] -eq 'ps' }
        $container.Config.Labels.'cosmos-vnext-owner' = 'other-job'
        { Complete-CosmosVNext } | Should -Throw '*Refusing to remove*'
        Should -Invoke Invoke-CosmosVNextDocker -Exactly 0 -ParameterFilter { $Arguments[0] -eq 'rm' }
    }

    It 'finalizes by job ownership in a separate PowerShell process' {
        Mock Invoke-CosmosVNextDocker { 'cosmosdb-emulator-vnext' } -ParameterFilter { $Arguments[0] -eq 'ps' }
        Start-CosmosVNext v1 text
        $containerPath = ([System.IO.Path]::Combine($TestDrive, 'container.json'))
        $finalizerPath = ([System.IO.Path]::Combine($TestDrive, 'finalize.ps1'))
        $helperPath = ([System.IO.Path]::Combine($PSScriptRoot, '..', 'CosmosVNext.ps1'))
        ConvertTo-Json -InputObject @($container) -Depth 10 | Set-Content $containerPath
        @'
param($HelperPath, $ContainerPath)
$ErrorActionPreference = 'Stop'
. $HelperPath
function Invoke-CosmosVNextDocker {
    param([string[]] $Arguments)
    switch ($Arguments[0]) {
        'ps' { 'cosmosdb-emulator-vnext' }
        'inspect' {
            if ($Arguments.Count -eq 2) { Get-Content $ContainerPath -Raw }
            else { 'identity' }
        }
        'logs' { 'captured logs' }
        'rm' { "removed:$env:SYSTEM_JOBID" }
        default { throw "Unexpected Docker operation: $($Arguments[0])" }
    }
}
Complete-CosmosVNext
'@ | Set-Content $finalizerPath
        $output = & pwsh -NoProfile -File $finalizerPath $helperPath $containerPath 2>&1
        $LASTEXITCODE | Should -Be 0
        $output | Should -Contain 'removed:vnext-test-job'
        $env:SYSTEM_JOBID = 'other-job'
        $output = & pwsh -NoProfile -File $finalizerPath $helperPath $containerPath 2>&1
        $LASTEXITCODE | Should -Not -Be 0
        ($output -join "`n") | Should -BeLike '*Refusing to remove unowned container*'
        ($output -join "`n") | Should -Not -BeLike '*removed:other-job*'
    }
}

Describe 'Cosmos vNext Docker exit codes' {
    BeforeAll {
        . ([System.IO.Path]::Combine($PSScriptRoot, '..', 'CosmosVNext.ps1'))
        function docker {}
    }

    It 'surfaces Docker failures immediately' {
        Mock docker { $global:LASTEXITCODE = 9; 'daemon failure' }
        { Invoke-CosmosVNextDocker @('inspect', 'test') } | Should -Throw '*exit 9*daemon failure*'
    }
}
