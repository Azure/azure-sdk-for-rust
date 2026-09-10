# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Build-NativeMatrix target compiler configuration' {
    BeforeAll {
        $PipelineDirectory = Split-Path -Parent $PSScriptRoot
        $ScriptPath = Join-Path $PipelineDirectory 'Build-NativeMatrix.ps1'
        $MatrixPath = Join-Path $PipelineDirectory 'build-matrix.json'
        $PipelinePath = Join-Path $PipelineDirectory 'native-driver.yml'
        $JobMatrixScriptPath = Join-Path $PipelineDirectory 'New-NativeJobMatrix.ps1'
        $BuildJobTemplatePath = Join-Path $PipelineDirectory 'native-driver-build-job.yml'
        $RepositoryRoot = (Resolve-Path (Join-Path $PipelineDirectory '../../../..')).Path
        $OneEsRedirectPath = Join-Path $RepositoryRoot 'eng/pipelines/templates/stages/1es-redirect.yml'
        $Matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
        $CargoLinkerVariable = 'CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER'
        $CcVariable = 'CC_x86_64_pc_windows_gnu'
    }

    BeforeEach {
        $global:ObservedCargoLinker = $null
        $global:ObservedCc = $null
        $global:ObservedBuildCargoLinker = $null
        $global:ObservedBuildCc = $null
        $global:CompilerLibraryPath = Join-Path $TestDrive 'libgcc_eh.a'
        Set-Content $global:CompilerLibraryPath 'archive'

        Mock git {
            $global:LASTEXITCODE = 0
            '0123456789abcdef0123456789abcdef01234567'
        }
        Mock rustup {
            $global:LASTEXITCODE = 0
            @(
                'x86_64-pc-windows-gnu'
                'x86_64-unknown-linux-musl'
            )
        }
        Mock pwsh {
            $global:LASTEXITCODE = 0
            if ($args[0] -eq '-print-file-name=libgcc_eh.a') {
                $global:CompilerLibraryPath
            }
            elseif ($args[0] -eq '--version') {
                'PowerShell 7.0.0'
            }
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
                    if ($args -contains 'x86_64-unknown-linux-musl') {
                        'note: native-static-libs: -lunwind -lc'
                        break
                    }
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
            $metadata.schema_version | Should -Be 3
            @($metadata.rustc_native_static_libs) | Should -Be @('-lsystem')
            @($metadata.native_static_libs) | Should -Be @('-lsystem')
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

    It 'uses the compiler unwind implementation for musl consumers' {
        & $ScriptPath `
            -TargetId 'linux-amd64-musl' `
            -OutputRoot (Join-Path $TestDrive 'artifacts') `
            -CCompiler 'pwsh' `
            -SkipBuild

        $metadataPath = Join-Path $TestDrive `
            'artifacts/linux-amd64-musl/rust-driver-native-interface-metadata.json'
        $metadata = Get-Content $metadataPath -Raw | ConvertFrom-Json

        @($metadata.rustc_native_static_libs) | Should -Be @('-lunwind', '-lc')
        @($metadata.native_static_libs) | Should -Be @('-lgcc_eh', '-lc')
    }

    It 'rejects a musl compiler without its configured unwind implementation' {
        $global:CompilerLibraryPath = 'libgcc_eh.a'

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-musl' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild
        } | Should -Throw "*cannot provide required library 'libgcc_eh.a'*"
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

    It 'generates one shared job-matrix row for every active build target' {
        $outputPath = Join-Path $TestDrive 'native-driver-job-matrix.json'
        & $JobMatrixScriptPath -MatrixPath $MatrixPath -OutputPath $outputPath

        $jobMatrix = Get-Content $outputPath -Raw | ConvertFrom-Json
        $generatedTargets = @($jobMatrix.matrix.Target.PSObject.Properties)
        $generatedTargets.Count | Should -Be $Matrix.targets.Count

        foreach ($target in $Matrix.targets) {
            $generated = $jobMatrix.matrix.Target.($target.id)
            $generated.TargetId | Should -Be $target.id
            $generated.Triple | Should -Be $target.triple
            $generated.CCompiler | Should -Be $target.c_compiler
            $generated.GoToolchainVersion | Should -Be $Matrix.go_toolchain_version
        }
    }

    It 'assigns symbolic pool and image values for shared matrix generation' {
        $outputPath = Join-Path $TestDrive 'native-driver-job-matrix.json'
        & $JobMatrixScriptPath -MatrixPath $MatrixPath -OutputPath $outputPath

        $jobMatrix = Get-Content $outputPath -Raw | ConvertFrom-Json
        $jobMatrix.matrix.Target.'windows-amd64'.Pool | Should -Be 'env:WINDOWSPOOL'
        $jobMatrix.matrix.Target.'windows-amd64'.OSVmImage | Should -Be 'env:WINDOWSVMIMAGE'
        $jobMatrix.matrix.Target.'linux-amd64-glibc'.Pool | Should -Be 'env:LINUXPOOL'
        $jobMatrix.matrix.Target.'darwin-arm64'.Pool | Should -Be 'env:MACPOOL'
        $jobMatrix.matrix.Target.'darwin-arm64'.OSVmImage | Should -Be 'env:MACVMIMAGEM1'
    }

    It 'uses the official 1ES wrapper and shared matrix generator' {
        $pipeline = Get-Content $PipelinePath -Raw

        $pipeline | Should -Match ([regex]::Escape(
            'template: /eng/pipelines/templates/stages/1es-redirect.yml'
        ))
        $pipeline | Should -Match 'Use1ESOfficial:\s+true'
        $pipeline | Should -Match 'EnableGoInternalModuleProxy:\s+true'
        $pipeline | Should -Match ([regex]::Escape(
            'template: /eng/common/pipelines/templates/jobs/generate-job-matrix.yml'
        ))
        $pipeline | Should -Not -Match 'parameters\.targets'
        $pipeline | Should -Not -Match '\$\{\{\s*each\s+t\s+in'

        $buildJobTemplate = Get-Content $BuildJobTemplatePath -Raw
        $buildJobTemplate | Should -Match 'name:\s+\$\(Pool\)'
        $buildJobTemplate | Should -Match ([regex]::Escape(
            'template: /eng/pipelines/templates/steps/use-rust.yml@self'
        ))
        $buildJobTemplate | Should -Match ([regex]::Escape(
            "C:\msys64\mingw64\bin"
        ))
        $buildJobTemplate | Should -Match ([regex]::Escape('-StaticOnly'))
    }

    It 'provisions Linux compilers from Ubuntu or checksum-pinned Microsoft prior art' {
        $buildJobTemplate = Get-Content $BuildJobTemplatePath -Raw

        $buildJobTemplate | Should -Match 'gcc-aarch64-linux-gnu'
        $buildJobTemplate | Should -Match 'libc6-dev-arm64-cross'
        $buildJobTemplate | Should -Match 'musl-dev musl-tools'
        $buildJobTemplate | Should -Match ([regex]::Escape(
            'microsoft/vscode-linux-build-agent/releases/download/'
        ))
        $buildJobTemplate | Should -Match '58cd59ee4038291fe8a7f4adccac0ecbe8d23cbad1cb650b381e45e7e1e22424'
        $buildJobTemplate | Should -Match 'sha256sum --check'
        $buildJobTemplate | Should -Match 'task:\s+GoTool@0'
        $buildJobTemplate | Should -Match 'GOTOOLCHAIN:\s+local'
    }

    It 'declares musl unwind replacements without changing other targets' {
        foreach ($targetId in @('linux-amd64-musl', 'linux-arm64-musl')) {
            $target = $Matrix.targets | Where-Object id -EQ $targetId
            $replacement = @($target.native_static_lib_replacements)

            $replacement.Count | Should -Be 1
            $replacement[0].rustc_flag | Should -Be '-lunwind'
            $replacement[0].consumer_flag | Should -Be '-lgcc_eh'
            $replacement[0].compiler_library | Should -Be 'libgcc_eh.a'
        }

        foreach ($target in @($Matrix.targets | Where-Object libc -NE 'musl')) {
            $target.PSObject.Properties.Name | Should -Not -Contain 'native_static_lib_replacements'
        }
    }

    It 'opts into the 1ES internal Go proxy without changing the shared default' {
        $pipeline = Get-Content $PipelinePath -Raw
        $oneEsRedirect = Get-Content $OneEsRedirectPath -Raw

        $pipeline | Should -Match 'EnableGoInternalModuleProxy:\s+true'
        $oneEsRedirect | Should -Match '(?s)name:\s+EnableGoInternalModuleProxy.*?default:\s+false'
        $oneEsRedirect | Should -Match '(?s)golang:.*?internalModuleProxy:.*?enabled:\s+true'
    }

    It 'publishes directly after the official 1ES build without a custom evidence gate' {
        $pipeline = Get-Content $PipelinePath -Raw
        $buildJobTemplate = Get-Content $BuildJobTemplatePath -Raw

        $pipeline | Should -Not -Match 'security_evidence'
        $pipeline | Should -Match '(?s)stage:\s+gomodules.*?dependsOn:\s+build'
        (
            [regex]::Matches(
                "$pipeline`n$buildJobTemplate",
                'SbomEnabled:\s+true'
            )
        ).Count | Should -Be 2
        $pipeline | Should -Match ([regex]::Escape(
            "eq(variables['Build.Reason'], 'Manual')"
        ))
    }
}
