# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Build-NativeMatrix target compiler configuration' {
    BeforeAll {
        $PipelineDirectory = Split-Path -Parent $PSScriptRoot
        $ScriptPath = Join-Path $PipelineDirectory 'Build-NativeMatrix.ps1'
        $MatrixPath = Join-Path $PipelineDirectory 'build-matrix.json'
        $PipelinePath = Join-Path $PipelineDirectory 'native-driver.yml'
        $Matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
        $CargoLinkerVariable = 'CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER'
        $CcVariable = 'CC_x86_64_pc_windows_gnu'
    }

    BeforeEach {
        $global:ObservedCargoLinker = $null
        $global:ObservedCc = $null
        $global:ObservedBuildCargoLinker = $null
        $global:ObservedBuildCc = $null

        Mock git {
            $global:LASTEXITCODE = 0
            '0123456789abcdef0123456789abcdef01234567'
        }
        Mock rustup {
            $global:LASTEXITCODE = 0
            'x86_64-pc-windows-gnu'
        }
        Mock cargo {
            $global:LASTEXITCODE = 0
            switch ($args[0]) {
                'metadata' {
                    [ordered]@{
                        packages = @(
                            [ordered]@{
                                name = 'azure_data_cosmos_driver_native'
                                version = '0.1.0'
                            },
                            [ordered]@{
                                name = 'azure_data_cosmos_driver'
                                version = '0.7.0'
                            }
                        )
                    } | ConvertTo-Json -Depth 4
                }
                'rustc' {
                    $global:ObservedCargoLinker = [Environment]::GetEnvironmentVariable(
                        'CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER',
                        'Process'
                    )
                    $global:ObservedCc = [Environment]::GetEnvironmentVariable(
                        'CC_x86_64_pc_windows_gnu',
                        'Process'
                    )
                    'note: native-static-libs: -lsystem'
                }
                'auditable' {
                    $global:ObservedBuildCargoLinker = [Environment]::GetEnvironmentVariable(
                        'CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER',
                        'Process'
                    )
                    $global:ObservedBuildCc = [Environment]::GetEnvironmentVariable(
                        'CC_x86_64_pc_windows_gnu',
                        'Process'
                    )
                    $global:LASTEXITCODE = 1
                }
                '--version' {
                    'cargo 1.0.0'
                }
            }
        }
    }

    It 'configures target-specific Cargo and C compiler variables while capturing system libraries' {
        $savedCargoLinker = [Environment]::GetEnvironmentVariable($CargoLinkerVariable, 'Process')
        $savedCc = [Environment]::GetEnvironmentVariable($CcVariable, 'Process')
        try {
            [Environment]::SetEnvironmentVariable($CargoLinkerVariable, 'original-linker', 'Process')
            [Environment]::SetEnvironmentVariable($CcVariable, 'original-cc', 'Process')

            & $ScriptPath `
                -TargetId 'windows-amd64' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild

            $global:ObservedCargoLinker | Should -Be 'pwsh'
            $global:ObservedCc | Should -Be 'pwsh'
            [Environment]::GetEnvironmentVariable($CargoLinkerVariable, 'Process') |
                Should -Be 'original-linker'
            [Environment]::GetEnvironmentVariable($CcVariable, 'Process') |
                Should -Be 'original-cc'

            $metadataPath = Join-Path $TestDrive `
                'artifacts/windows-amd64/rust-driver-native-interface-metadata.json'
            $metadata = Get-Content $metadataPath -Raw | ConvertFrom-Json
            $metadata.toolchains.c_compiler | Should -Be 'pwsh'
        }
        finally {
            [Environment]::SetEnvironmentVariable($CargoLinkerVariable, $savedCargoLinker, 'Process')
            [Environment]::SetEnvironmentVariable($CcVariable, $savedCc, 'Process')
        }
    }

    It 'configures the same target compiler for the auditable build' {
        {
            & $ScriptPath `
                -TargetId 'windows-amd64' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh'
        } | Should -Throw '*cargo-auditable build failed*'

        $global:ObservedBuildCargoLinker | Should -Be 'pwsh'
        $global:ObservedBuildCc | Should -Be 'pwsh'
    }

    It 'rejects an unavailable compiler before invoking Cargo' {
        {
            & $ScriptPath `
                -TargetId 'windows-amd64' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'compiler-that-does-not-exist' `
                -SkipBuild
        } | Should -Throw "*C compiler 'compiler-that-does-not-exist' is not available*"

        Should -Invoke cargo -Exactly 0
    }

    It 'keeps pipeline compiler parameters aligned with build-matrix.json' {
        $pipeline = Get-Content $PipelinePath -Raw
        $pipeline | Should -Match ([regex]::Escape('-CCompiler ${{ t.cc }}'))

        $pipelineCompilers = @{}
        foreach ($match in [regex]::Matches($pipeline, 'id:\s*([^,\s}]+).*?cc:\s*([^,\s}]+)')) {
            $pipelineCompilers[$match.Groups[1].Value] = $match.Groups[2].Value
        }

        $matrixTargets = @{}
        foreach ($target in @($Matrix.targets) + @($Matrix.optional_targets)) {
            $matrixTargets[$target.id] = $target
        }
        foreach ($targetId in $pipelineCompilers.Keys) {
            $matrixTargets.ContainsKey($targetId) | Should -BeTrue
            $pipelineCompilers[$targetId] | Should -Be $matrixTargets[$targetId].c_compiler
        }
        foreach ($target in $Matrix.targets) {
            $pipelineCompilers.ContainsKey($target.id) | Should -BeTrue
        }
    }
}
