# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Build-NativeMatrix target compiler configuration' {
    BeforeAll {
        $OriginalRustVersion = $env:RUST_VERSION
        $OriginalRustBinPath = $env:RUST_BIN_PATH
        $PipelineDirectory = Split-Path -Parent $PSScriptRoot
        $ScriptPath = Join-Path $PipelineDirectory 'Build-NativeMatrix.ps1'
        $MatrixPath = Join-Path $PipelineDirectory 'build-matrix.json'
        $MatrixSchemaPath = Join-Path $PipelineDirectory 'build-matrix.schema.json'
        $NativeInterfaceSourcePath = Join-Path (Split-Path -Parent $PipelineDirectory) 'src/lib.rs'
        $PipelinePath = Join-Path $PipelineDirectory 'native-driver.yml'
        $JobMatrixScriptPath = Join-Path $PipelineDirectory 'New-NativeJobMatrix.ps1'
        $BuildJobTemplatePath = Join-Path $PipelineDirectory 'native-driver-build-job.yml'
        $RepositoryRoot = (Resolve-Path (Join-Path $PipelineDirectory '../../../..')).Path
        $OneEsRedirectPath = Join-Path $RepositoryRoot 'eng/pipelines/templates/stages/1es-redirect.yml'
        $UseRustTemplatePath = Join-Path $RepositoryRoot 'eng/pipelines/templates/steps/use-rust.yml'
        $MicrosoftRustTemplatePath = Join-Path $RepositoryRoot 'eng/pipelines/templates/steps/use-ms-rust.yml'
        $MicrosoftRustConfigPath = Join-Path $RepositoryRoot 'eng/templates/ms-rust-toolchain.toml'
        $Matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
        $MicrosoftRustConfig = Get-Content $MicrosoftRustConfigPath -Raw
        $MicrosoftRustChannel = [regex]::Match(
            $MicrosoftRustConfig,
            '(?m)^\s*channel\s*=\s*"([^"]+)"\s*$'
        ).Groups[1].Value
        . ([System.IO.Path]::Combine(
            $RepositoryRoot,
            'eng',
            'scripts',
            'shared',
            'MicrosoftRust.ps1'
        ))
        $CargoLinkerVariable = 'CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER'
        $CcVariable = 'CC_x86_64_unknown_linux_gnu'
        function msrustup {}
    }

    BeforeEach {
        $global:ObservedCargoLinker = $null
        $global:ObservedCc = $null
        $global:ObservedBuildCargoLinker = $null
        $global:ObservedBuildCc = $null
        $global:ObservedCargoArguments = @()
        $global:ObservedRustcArguments = @()
        $global:MicrosoftRustManagerAvailable = $true
        $global:InstalledMicrosoftRustTargets = @(
            'x86_64-unknown-linux-gnu'
            'x86_64-unknown-linux-musl'
        )
        $global:MicrosoftRustRelease = '1.95.0'
        $env:RUST_VERSION = '1.95.0-ms-20260618.5'
        $global:MicrosoftRustSysroot = Join-Path $TestDrive 'ms-prod-1.95'
        $global:SelectedRustSysroot = $global:MicrosoftRustSysroot
        $global:MicrosoftRustBin = Join-Path $global:MicrosoftRustSysroot 'bin'
        $env:RUST_BIN_PATH = $global:MicrosoftRustBin
        $env:TEST_INSTALLER_RUST_SYSROOT = $global:MicrosoftRustSysroot
        $env:TEST_INSTALLER_RUST_RELEASE = $global:MicrosoftRustRelease
        $env:TEST_INSTALLER_CARGO_VERSION = 'cargo 1.0.0'
        $global:RustProxyBin = Join-Path $TestDrive 'cargo-home/bin'
        New-Item -ItemType Directory -Force -Path $global:RustProxyBin | Out-Null
        New-Item -ItemType Directory -Force -Path $global:MicrosoftRustBin | Out-Null
        $global:InstallerRustcExecutable = Join-Path $global:MicrosoftRustBin 'rustc.ps1'
        $global:InstallerCargoExecutable = Join-Path $global:MicrosoftRustBin 'cargo.ps1'
        Set-Content $global:InstallerRustcExecutable @'
$global:LASTEXITCODE = 0
if ($args -contains 'sysroot') {
    $env:TEST_INSTALLER_RUST_SYSROOT
    return
}
@(
    "rustc $env:TEST_INSTALLER_RUST_RELEASE (012345678 2026-08-01)"
    'binary: rustc'
    'commit-hash: 0123456789abcdef0123456789abcdef01234567'
    'commit-date: 2026-08-01'
    'host: x86_64-pc-windows-msvc'
    "release: $env:TEST_INSTALLER_RUST_RELEASE"
    'LLVM version: 21.1.0'
)
'@
        Set-Content $global:InstallerCargoExecutable @'
$global:LASTEXITCODE = 0
$env:TEST_INSTALLER_CARGO_VERSION
'@
        $global:RustcExecutable = Join-Path $global:RustProxyBin 'rustc'
        $global:CargoExecutable = Join-Path $global:RustProxyBin 'cargo'
        Set-Content $global:RustcExecutable ''
        Set-Content $global:CargoExecutable ''
        $global:CompilerLibraryPath = Join-Path $TestDrive 'libgcc_eh.a'
        Set-Content $global:CompilerLibraryPath 'archive'

        Mock git {
            $global:LASTEXITCODE = 0
            '0123456789abcdef0123456789abcdef01234567'
        }
        Mock Get-Command {
            if ($Name -eq 'msrustup' -and $global:MicrosoftRustManagerAvailable) {
                [pscustomobject]@{
                    Name = 'msrustup'
                    Source = $null
                }
            }
            elseif ($Name -eq 'rustc') {
                [pscustomobject]@{
                    Name = 'rustc'
                    Source = $global:RustcExecutable
                }
            }
            elseif ($Name -eq 'cargo') {
                [pscustomobject]@{
                    Name = 'cargo'
                    Source = $global:CargoExecutable
                }
            }
        } -ParameterFilter { $Name -in @('msrustup', 'rustc', 'cargo') }
        Mock msrustup {
            switch ("$args") {
                '--version' {
                    $global:LASTEXITCODE = 0
                    'msrustup 1.0.0'
                }
                default {
                    throw "Unexpected msrustup arguments: $args"
                }
            }
        }
        Mock rustc {
            $global:ObservedRustcArguments += ,@($args)
            $global:LASTEXITCODE = 0
            if ($args -contains 'sysroot') {
                $global:SelectedRustSysroot
                return
            }
            if ($args -contains 'target-libdir') {
                $targetIndex = [array]::IndexOf([object[]]$args, '--target') + 1
                $triple = [string]$args[$targetIndex]
                $libDir = Join-Path $global:SelectedRustSysroot "lib/rustlib/$triple/lib"
                if ($global:InstalledMicrosoftRustTargets -contains $triple) {
                    New-Item -ItemType Directory -Force -Path $libDir | Out-Null
                }
                elseif (Test-Path $libDir) {
                    Remove-Item -Recurse -Force $libDir
                }
                $libDir
                return
            }
            @(
                "rustc $global:MicrosoftRustRelease (012345678 2026-08-01)"
                'binary: rustc'
                'commit-hash: 0123456789abcdef0123456789abcdef01234567'
                'commit-date: 2026-08-01'
                'host: x86_64-pc-windows-msvc'
                "release: $global:MicrosoftRustRelease"
                'LLVM version: 21.1.0'
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
            $global:ObservedCargoArguments += ,@($args)
            $global:LASTEXITCODE = 0
            switch ($args[1]) {
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
                        'CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER',
                        'Process'
                    )
                    $global:ObservedCc = [Environment]::GetEnvironmentVariable(
                        'CC_x86_64_unknown_linux_gnu',
                        'Process'
                    )
                    'note: native-static-libs: -lsystem'
                }
                'auditable' {
                    $global:ObservedBuildCargoLinker = [Environment]::GetEnvironmentVariable(
                        'CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER',
                        'Process'
                    )
                    $global:ObservedBuildCc = [Environment]::GetEnvironmentVariable(
                        'CC_x86_64_unknown_linux_gnu',
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
                -TargetId 'linux-amd64-glibc' `
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
                'artifacts/linux-amd64-glibc/rust-driver-native-interface-metadata.json'
            $metadata = Get-Content $metadataPath -Raw | ConvertFrom-Json
            $metadata.schema_version | Should -Be 4
            $abiSource = Get-Content $NativeInterfaceSourcePath -Raw
            $abiMajor = [regex]::Match(
                $abiSource,
                '(?m)^\s*const ABI_VERSION_MAJOR:\s*u16\s*=\s*(\d+)\s*;'
            ).Groups[1].Value
            $abiMinor = [regex]::Match(
                $abiSource,
                '(?m)^\s*const ABI_VERSION_MINOR:\s*u16\s*=\s*(\d+)\s*;'
            ).Groups[1].Value
            $metadata.abi.version | Should -Be "$abiMajor.$abiMinor"
            $metadata.abi.compatibility | Should -Be 'same-major-minimum-minor'
            @($metadata.rustc_native_static_libs) | Should -Be @('-lsystem')
            @($metadata.native_static_libs) | Should -Be @('-lsystem')
            $metadata.toolchain.provider | Should -Be 'microsoft'
            $metadata.toolchain.manager.executable | Should -Be 'msrustup'
            $metadata.toolchain.channel | Should -Be $MicrosoftRustChannel
            $metadata.toolchain.selected_toolchain | Should -Be $MicrosoftRustChannel
            $metadata.toolchain.installer_package_version |
                Should -Be '1.95.0-ms-20260618.5'
            $metadata.toolchain.sysroot | Should -Be $global:MicrosoftRustSysroot
            $metadata.toolchain.rustc_executable | Should -Be $global:RustcExecutable
            $metadata.toolchain.cargo_executable | Should -Be $global:CargoExecutable
            $metadata.toolchain.installer_rustc_executable |
                Should -Be $global:InstallerRustcExecutable
            $metadata.toolchain.installer_cargo_executable |
                Should -Be $global:InstallerCargoExecutable
            $metadata.toolchain.rustc_verbose_version | Should -Match 'release: 1\.95\.0'
            $metadata.toolchain.rustc_release | Should -Be '1.95.0'
            $metadata.toolchain.cargo_version | Should -Be 'cargo 1.0.0'
            $metadata.toolchain.target | Should -Be 'x86_64-unknown-linux-gnu'
            $metadata.toolchain.linker.command | Should -Be 'pwsh'
            $metadata.toolchain.linker.executable | Should -Not -BeNullOrEmpty
            $metadata.toolchain.linker.version | Should -Be 'PowerShell 7.0.0'
            $global:ObservedRustcArguments.Count | Should -Be 3
            foreach ($invocation in $global:ObservedRustcArguments) {
                $invocation[0] | Should -Be "+$MicrosoftRustChannel"
            }
            $global:ObservedRustcArguments[0] | Should -Be @(
                "+$MicrosoftRustChannel"
                '--print'
                'sysroot'
            )
            $global:ObservedRustcArguments[1] | Should -Be @(
                "+$MicrosoftRustChannel"
                '-Vv'
            )
            $global:ObservedRustcArguments[2] | Should -Be @(
                "+$MicrosoftRustChannel"
                '--target'
                'x86_64-unknown-linux-gnu'
                '--print'
                'target-libdir'
            )
            foreach ($invocation in $global:ObservedCargoArguments) {
                $invocation[0] | Should -Be "+$MicrosoftRustChannel"
            }
        }
        finally {
            [Environment]::SetEnvironmentVariable($CargoLinkerVariable, $savedCargoLinker, 'Process')
            [Environment]::SetEnvironmentVariable($CcVariable, $savedCc, 'Process')
        }
    }

    It 'configures the same target compiler for the auditable build' {
        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
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
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'compiler-that-does-not-exist' `
                -SkipBuild
        } | Should -Throw "*C compiler 'compiler-that-does-not-exist' is not available*"

        Should -Invoke cargo -Exactly 0
    }

    It 'rejects a missing Microsoft Rust manager selection' {
        $global:MicrosoftRustManagerAvailable = $false

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild
        } | Should -Throw '*msrustup is not installed or is not available on PATH*'
    }

    It 'rejects a missing Microsoft Rust installer package identity' {
        $env:RUST_VERSION = $null

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild
        } | Should -Throw '*Microsoft Rust installer package version is required*'
    }

    It 'rejects an upstream Rust installer package identity' {
        $env:RUST_VERSION = '1.95.0'

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild
        } | Should -Throw "*does not identify Microsoft Rust release '1.95.0'*"
    }

    It 'rejects a missing RustInstaller bin path' {
        $env:RUST_BIN_PATH = $null

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild
        } | Should -Throw '*Microsoft Rust installer bin path is required*'
    }

    It 'accepts an installer alias path when compiler identities match' {
        $aliasPath = Join-Path $TestDrive 'toolchains/ms-prod-1.95/bin'
        New-Item -ItemType Directory -Force -Path $aliasPath | Out-Null
        Copy-Item $global:InstallerRustcExecutable (Join-Path $aliasPath 'rustc.ps1')
        Copy-Item $global:InstallerCargoExecutable (Join-Path $aliasPath 'cargo.ps1')
        $env:RUST_BIN_PATH = $aliasPath

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild
        } | Should -Not -Throw
    }

    It 'rejects a rustc selection outside the RustInstaller compiler' {
        $global:SelectedRustSysroot = Join-Path $TestDrive 'upstream-1.95'
        New-Item -ItemType Directory -Force -Path $global:SelectedRustSysroot | Out-Null

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild
        } | Should -Throw '*refusing a possible upstream fallback*'
    }

    It 'rejects a selected compiler identity that differs from RustInstaller' {
        $env:TEST_INSTALLER_RUST_RELEASE = '1.95.1'

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild
        } | Should -Throw '*identity does not match the RustInstaller compiler*'
    }

    It 'accepts an explicit installer package version for local builds' {
        $env:RUST_VERSION = $null

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -InstallerPackageVersion '1.95.0-ms-20260618.5' `
                -InstallerBinPath $global:MicrosoftRustBin `
                -SkipBuild
        } | Should -Not -Throw
    }

    It 'rejects a Microsoft compiler release that differs from the pinned channel' {
        $global:MicrosoftRustRelease = '1.96.0'
        $env:TEST_INSTALLER_RUST_RELEASE = $global:MicrosoftRustRelease

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild
        } | Should -Throw "*does not match pinned channel '$MicrosoftRustChannel'*"
    }

    It 'rejects an unpinned Microsoft Rust channel' {
        $toolchainConfig = Join-Path $TestDrive 'ms-rust-toolchain.toml'
        Set-Content $toolchainConfig @'
[toolchain]
channel = "ms-prod"
targets = ["x86_64-unknown-linux-gnu"]
'@

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-glibc' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -ToolchainConfigPath $toolchainConfig `
                -SkipBuild
        } | Should -Throw '*not an explicit pinned ms-prod channel*'
    }

    It 'fails closed when a required Microsoft Rust target is not installed' {
        $global:InstalledMicrosoftRustTargets = @('x86_64-unknown-linux-gnu')

        {
            & $ScriptPath `
                -TargetId 'linux-amd64-musl' `
                -OutputRoot (Join-Path $TestDrive 'artifacts') `
                -CCompiler 'pwsh' `
                -SkipBuild
        } | Should -Throw "*target 'x86_64-unknown-linux-musl' is not installed*"
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
            $generated.PublicationKind | Should -Be $target.publication_kind
            $generated.GoToolchainVersion | Should -Be $Matrix.go_toolchain_version
        }
    }

    It 'defines Windows ARM64 as a signed DLL publication target' {
        $target = $Matrix.targets | Where-Object id -EQ 'windows-arm64-msvc'

        $target.publication_kind | Should -Be 'windows-dll'
        $target.goos | Should -Be 'windows'
        $target.goarch | Should -Be 'arm64'
        $target.triple | Should -Be 'aarch64-pc-windows-msvc'
        $target.c_compiler | Should -Be 'link.exe'
        $target.module_path | Should -BeNullOrEmpty

        $buildJobTemplate = Get-Content $BuildJobTemplatePath -Raw
        $buildJobTemplate | Should -Match 'task:\s+EsrpCodeSigning@5'
        $buildJobTemplate | Should -Match 'OperationCode":\s+"SigntoolSign"'
        $buildJobTemplate | Should -Match 'OperationCode":\s+"SigntoolVerify"'
        $buildJobTemplate | Should -Match 'Finalize-WindowsArm64Artifact\.ps1'
        $buildJobTemplate | Should -Match ([regex]::Escape(
            "eq(variables['PublicationKind'], 'windows-dll')"
        ))
    }

    It 'requires module paths for static Go module targets' {
        $matrixJson = Get-Content $MatrixPath -Raw
        ($matrixJson | Test-Json -SchemaFile $MatrixSchemaPath) | Should -BeTrue

        $invalidMatrix = $matrixJson | ConvertFrom-Json
        $staticTarget = $invalidMatrix.targets |
            Where-Object publication_kind -EQ 'static-go-module' |
            Select-Object -First 1
        $staticTarget.PSObject.Properties.Remove('module_path')

        (
            $invalidMatrix |
                ConvertTo-Json -Depth 10 |
                Test-Json -SchemaFile $MatrixSchemaPath -ErrorAction SilentlyContinue
        ) | Should -BeFalse
    }

    It 'assigns symbolic pool and image values for shared matrix generation' {
        $outputPath = Join-Path $TestDrive 'native-driver-job-matrix.json'
        & $JobMatrixScriptPath -MatrixPath $MatrixPath -OutputPath $outputPath

        $jobMatrix = Get-Content $outputPath -Raw | ConvertFrom-Json
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
            'template: /eng/pipelines/templates/steps/use-ms-rust.yml@self'
        ))
        $buildJobTemplate | Should -Match 'TargetTriple:\s+\$\(Triple\)'
        $buildJobTemplate | Should -Match ([regex]::Escape(
            "C:\msys64\mingw64\bin"
        ))
        $buildJobTemplate | Should -Match ([regex]::Escape('-StaticOnly'))
    }

    It 'keeps Microsoft Rust installation opt-in and centrally pinned' {
        $template = Get-Content $MicrosoftRustTemplatePath -Raw
        $useRustTemplate = Get-Content $UseRustTemplatePath -Raw

        $template | Should -Match 'task:\s+RustInstaller@1'
        $template | Should -Match ([regex]::Escape(
            'https://pkgs.dev.azure.com/azure-sdk/internal/_packaging/ms-rust-tools/nuget/v3/index.json'
        ))
        $template | Should -Match ([regex]::Escape(
            'eng/templates/ms-rust-toolchain.toml'
        ))
        $template | Should -Not -Match 'RUSTUP_EXE'
        $template | Should -Match 'InstallToolchain:\s+false'
        $template | Should -Match 'additionalTargets:\s+\$\{\{\s*parameters\.TargetTriple\s*\}\}'
        $template | Should -Match 'New-MicrosoftRustInstallerConfiguration'
        $useRustTemplate | Should -Match '(?s)name:\s+InstallToolchain.*?default:\s+true'
        $MicrosoftRustChannel | Should -Match '^ms-prod-\d+(?:\.\d+)+$'
        $MicrosoftRustConfig | Should -Not -Match '"rust-std"'
        foreach ($target in $Matrix.targets) {
            $MicrosoftRustConfig | Should -Match ([regex]::Escape(
                "`"$($target.triple)`""
            ))
        }
    }

    It 'installs only the target assigned to the current matrix job' {
        $installerConfigPath = [System.IO.Path]::Combine(
            $TestDrive,
            'rust-toolchain.toml'
        )

        New-MicrosoftRustInstallerConfiguration `
            -Path $MicrosoftRustConfigPath `
            -Target 'x86_64-unknown-linux-gnu' `
            -OutputPath $installerConfigPath

        $installerConfig = Get-Content $installerConfigPath -Raw
        $installerConfig | Should -Match ([regex]::Escape(
            "channel = `"$MicrosoftRustChannel`""
        ))
        $installerConfig | Should -Not -Match '(?m)^\s*targets\s*='
        # The centralized toml keeps a documentation note about the deferred
        # Windows GNU triple, so assert it is absent from the active (non-comment)
        # configuration rather than from documentation lines.
        $installerConfigActive = (
            $installerConfig -split "`n" | Where-Object { $_ -notmatch '^\s*#' }
        ) -join "`n"
        $installerConfigActive | Should -Not -Match 'x86_64-pc-windows-gnu'
    }

    It 'rejects an installer target outside the centralized allowlist' {
        {
            New-MicrosoftRustInstallerConfiguration `
                -Path $MicrosoftRustConfigPath `
                -Target 'wasm32-unknown-unknown' `
                -OutputPath ([System.IO.Path]::Combine(
                    $TestDrive,
                    'rust-toolchain.toml'
                ))
        } | Should -Throw "*target 'wasm32-unknown-unknown' is not declared*"
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
        # The build template has mutually exclusive static and Windows
        # publication branches; the pipeline adds the combined Go artifact.
        ).Count | Should -Be 3
        $pipeline | Should -Match ([regex]::Escape(
            "eq(variables['Build.Reason'], 'Manual')"
        ))
    }

    AfterAll {
        $env:RUST_VERSION = $OriginalRustVersion
        $env:RUST_BIN_PATH = $OriginalRustBinPath
        Remove-Item Env:TEST_INSTALLER_RUST_SYSROOT -ErrorAction SilentlyContinue
        Remove-Item Env:TEST_INSTALLER_RUST_RELEASE -ErrorAction SilentlyContinue
        Remove-Item Env:TEST_INSTALLER_CARGO_VERSION -ErrorAction SilentlyContinue
    }
}
