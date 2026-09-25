# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Finalize-WindowsArm64Artifact signed payload validation' {
    BeforeAll {
        $PipelineDirectory = Split-Path -Parent $PSScriptRoot
        $ScriptPath = Join-Path $PipelineDirectory 'Finalize-WindowsArm64Artifact.ps1'

        function Write-Fixture {
            param([string] $Root)

            New-Item -ItemType Directory -Force $Root | Out-Null
            Set-Content (Join-Path $Root 'azurecosmosdriver.dll') 'signed-dll'
            @'
uint32_t cosmos_abi_version(void);
void cosmos_bytes_free_ref(void);
void cosmos_driver_options_config_default_init(void);
void cosmos_operation_options_default_init(void);
void cosmos_runtime_options_default_init(void);
const char *cosmos_version(void);
'@ | Set-Content (Join-Path $Root 'azurecosmosdriver.h')
            Set-Content (Join-Path $Root 'azurecosmosdriver.pdb') 'symbols'
            $headerHash = (Get-FileHash (Join-Path $Root 'azurecosmosdriver.h') -Algorithm SHA256).Hash.ToLowerInvariant()
            $pdbHash = (Get-FileHash (Join-Path $Root 'azurecosmosdriver.pdb') -Algorithm SHA256).Hash.ToLowerInvariant()
            [ordered]@{
                schema_version = 4
                artifact_id = 'windows-arm64-msvc'
                publication_kind = 'windows-dll'
                goos = 'windows'
                goarch = 'arm64'
                triple = 'aarch64-pc-windows-msvc'
                dynamic_library = [ordered]@{
                    filename = 'azurecosmosdriver.dll'
                    sha256 = $null
                    authenticode_verified = $false
                }
                header = [ordered]@{
                    filename = 'azurecosmosdriver.h'
                    sha256 = $headerHash
                }
                symbols = [ordered]@{
                    filename = 'azurecosmosdriver.pdb'
                    sha256 = $pdbHash
                    runtime_dependency = $false
                }
            } | ConvertTo-Json -Depth 6 |
                Set-Content (Join-Path $Root 'rust-driver-native-interface-metadata.json')
        }
    }

    BeforeEach {
        $global:WindowsArtifactTestSigner = [System.Security.Cryptography.X509Certificates.X509Certificate2]::new()
        Mock Get-AuthenticodeSignature {
            [pscustomobject]@{
                Status = [System.Management.Automation.SignatureStatus]::Valid
                StatusMessage = 'Signature verified.'
                SignerCertificate = $global:WindowsArtifactTestSigner
                TimeStamperCertificate = $global:WindowsArtifactTestSigner
            }
        }
    }

    It 'binds final signed bytes and PE inventory into metadata' {
        $root = Join-Path $TestDrive 'artifact'
        Write-Fixture $root
        $dumpbin = Join-Path $TestDrive 'dumpbin.ps1'
        @'
$global:LASTEXITCODE = 0
switch ($args[0]) {
    '/headers' {
        '            AA64 machine (ARM64)'
        '            Format: RSDS, azurecosmosdriver.pdb'
    }
    '/exports' {
        '          1    0 00001000 COSMOS_BUILD_IDENTIFIER'
        '          2    1 00001010 cosmos_abi_version'
        '          3    2 00001020 cosmos_bytes_free_ref'
        '          4    3 00001030 cosmos_driver_options_config_default_init'
        '          5    4 00001040 cosmos_operation_options_default_init'
        '          6    5 00001050 cosmos_runtime_options_default_init'
        '          7    6 00001060 cosmos_version'
    }
    '/imports' { '    KERNEL32.dll' }
}
'@ | Set-Content $dumpbin

        & $ScriptPath -ArtifactRoot $root -Dumpbin $dumpbin

        $metadata = Get-Content (Join-Path $root 'rust-driver-native-interface-metadata.json') -Raw |
            ConvertFrom-Json
        $metadata.dynamic_library.authenticode_verified | Should -BeTrue
        $metadata.dynamic_library.pe_machine_code | Should -Be '0xAA64'
        $metadata.dynamic_library.sha256 | Should -Match '^[0-9a-f]{64}$'
        @($metadata.dynamic_library.required_exports).Count | Should -Be 7
        @($metadata.dynamic_library.required_exports) |
            Should -Contain 'COSMOS_BUILD_IDENTIFIER'
        @($metadata.dynamic_library.header_exports).Count | Should -Be 6
        $metadata.symbols.runtime_dependency | Should -BeFalse
        Test-Path (Join-Path $root 'SHA256SUMS') | Should -BeTrue
    }

    It 'fails closed when the PDB is missing' {
        $root = Join-Path $TestDrive 'missing-pdb'
        Write-Fixture $root
        Remove-Item (Join-Path $root 'azurecosmosdriver.pdb')

        {
            & $ScriptPath -ArtifactRoot $root -Dumpbin 'unused'
        } | Should -Throw '*Required Windows ARM64 artifact was not found*'
    }

    It 'fails closed when the build identifier export is missing' {
        $root = Join-Path $TestDrive 'missing-build-identifier'
        Write-Fixture $root
        $dumpbin = Join-Path $TestDrive 'dumpbin-missing-build-identifier.ps1'
        @'
$global:LASTEXITCODE = 0
switch ($args[0]) {
    '/headers' {
        '            AA64 machine (ARM64)'
        '            Format: RSDS, azurecosmosdriver.pdb'
    }
    '/exports' {
        '          1    0 00001000 cosmos_abi_version'
        '          2    1 00001010 cosmos_bytes_free_ref'
        '          3    2 00001020 cosmos_driver_options_config_default_init'
        '          4    3 00001030 cosmos_operation_options_default_init'
        '          5    4 00001040 cosmos_runtime_options_default_init'
        '          6    5 00001050 cosmos_version'
    }
}
'@ | Set-Content $dumpbin

        {
            & $ScriptPath -ArtifactRoot $root -Dumpbin $dumpbin
        } | Should -Throw '*Required DLL export is missing: COSMOS_BUILD_IDENTIFIER*'
    }
}
