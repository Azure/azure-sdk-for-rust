# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Test-NativeLink target validation' {
    BeforeAll {
        $PipelineDirectory = Split-Path -Parent $PSScriptRoot
        $ScriptPath = Join-Path $PipelineDirectory 'Test-NativeLink.ps1'

        function Write-TestJson {
            param(
                [Parameter(Mandatory = $true)]
                [string]$Path,

                [Parameter(Mandatory = $true)]
                [object]$Value
            )

            $Value | ConvertTo-Json -Depth 6 | Set-Content $Path -Encoding utf8
        }
    }

    BeforeEach {
        $ArtifactRoot = Join-Path $TestDrive 'artifacts'
        $TargetRoot = Join-Path $ArtifactRoot 'test-target'
        New-Item -ItemType Directory -Path $TargetRoot -Force | Out-Null
        Set-Content (Join-Path $TargetRoot 'libazurecosmosdriver.a') 'archive'
        Set-Content (Join-Path $TargetRoot 'azurecosmosdriver.h') 'const char *cosmos_version(void);'

        $MatrixPath = Join-Path $TestDrive 'matrix.json'
        Write-TestJson -Path $MatrixPath -Value ([ordered]@{
            lib_basename = 'azurecosmosdriver'
            static_lib_filename = 'libazurecosmosdriver.a'
            header_filename = 'azurecosmosdriver.h'
            go_version = '1.25.0'
            targets = @(
                [ordered]@{
                    id = 'test-target'
                    goos = 'windows'
                    goarch = 'amd64'
                    libc = $null
                    triple = 'x86_64-pc-windows-gnu'
                    build_tag_extra = $null
                }
            )
        })
        $MetadataPath = Join-Path $TargetRoot 'rust-driver-native-interface-metadata.json'
        Write-TestJson -Path $MetadataPath -Value ([ordered]@{
            schema_version = 3
            artifact_id = 'test-target'
            goos = 'windows'
            goarch = 'amd64'
            libc = $null
            triple = 'x86_64-pc-windows-gnu'
            rustc_native_static_libs = @('-lsystem')
            native_static_libs = @('-lsystem')
        })

        Mock go {
            $outputIndex = [Array]::IndexOf($args, '-o')
            New-Item -ItemType File -Path $args[$outputIndex + 1] -Force | Out-Null
            $global:LASTEXITCODE = 0
        }
    }

    It 'links a target whose metadata matches the matrix' {
        {
            & $ScriptPath `
                -TargetId 'test-target' `
                -ArtifactRoot $ArtifactRoot `
                -CCompiler 'pwsh' `
                -MatrixPath $MatrixPath
        } | Should -Not -Throw

        Should -Invoke go -Exactly 1
    }

    It 'rejects mismatched target metadata before invoking Go' {
        $metadata = Get-Content $MetadataPath -Raw | ConvertFrom-Json
        $metadata.goarch = 'arm64'
        Write-TestJson -Path $MetadataPath -Value $metadata

        {
            & $ScriptPath `
                -TargetId 'test-target' `
                -ArtifactRoot $ArtifactRoot `
                -CCompiler 'pwsh' `
                -MatrixPath $MatrixPath
        } | Should -Throw "*metadata 'goarch' does not match*"

        Should -Invoke go -Exactly 0
    }
}
