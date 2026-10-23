# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Cosmos vNext setup' {
    BeforeAll {
        . ([System.IO.Path]::Combine($PSScriptRoot, '..', 'CosmosVNext.ps1'))
        function docker {}
        $variables = @(
            'AZURE_COSMOS_CONNECTION_STRING', 'AZURE_COSMOS_TEST_MODE',
            'AZURE_COSMOS_EMULATOR_SERVER_CERT_VALIDATION_DISABLED', 'RUSTFLAGS', 'RUST_TEST_THREADS',
            'SYSTEM_JOBID'
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
