# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'New-GoModules artifact provenance validation' {
BeforeAll {
    $PipelineDirectory = Split-Path -Parent $PSScriptRoot
    $ScriptPath = Join-Path $PipelineDirectory 'New-GoModules.ps1'
    $MatrixPath = Join-Path $PipelineDirectory 'build-matrix.json'
    $Matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
    $StaticTargets = @($Matrix.targets | Where-Object publication_kind -EQ 'static-go-module')
    $RepositoryRoot = (Resolve-Path (Join-Path $PipelineDirectory '../../../..')).Path
    $MicrosoftRustConfigPath = Join-Path $RepositoryRoot 'eng/templates/ms-rust-toolchain.toml'
    $MicrosoftRustConfig = Get-Content $MicrosoftRustConfigPath -Raw
    $MicrosoftRustChannel = [regex]::Match(
        $MicrosoftRustConfig,
        '(?m)^\s*channel\s*=\s*"([^"]+)"\s*$'
    ).Groups[1].Value

    function Write-TestFile {
        param(
            [Parameter(Mandatory = $true)]
            [string]$Path,

            [Parameter(Mandatory = $true)]
            [string]$Content
        )

        $directory = Split-Path -Parent $Path
        New-Item -ItemType Directory -Path $directory -Force | Out-Null
        [IO.File]::WriteAllText($Path, $Content, [Text.UTF8Encoding]::new($false))
    }

    function Get-TestHash {
        param(
            [Parameter(Mandatory = $true)]
            [string]$Path
        )

        (Get-FileHash $Path -Algorithm SHA256).Hash.ToLowerInvariant()
    }

    function Get-TreeSnapshot {
        param(
            [Parameter(Mandatory = $true)]
            [string]$Root
        )

        @(
            Get-ChildItem $Root -Recurse -File |
                Sort-Object FullName |
                ForEach-Object {
                    $relativePath = [IO.Path]::GetRelativePath($Root, $_.FullName).Replace('\', '/')
                    "$relativePath|$(Get-TestHash -Path $_.FullName)"
                }
        )
    }

    function New-TestArtifacts {
        param(
            [Parameter(Mandatory = $true)]
            [string]$Root
        )

        foreach ($row in $StaticTargets) {
            $targetRoot = Join-Path $Root $row.id
            $headerPath = Join-Path $targetRoot $Matrix.header_filename
            $libraryPath = Join-Path $targetRoot $Matrix.static_lib_filename
            Write-TestFile -Path $headerPath -Content "int azure_cosmos_test(void);`n"
            Write-TestFile -Path $libraryPath -Content "archive-$($row.id)"

            $metadata = [ordered]@{
                schema_version = 4
                artifact_id = $row.id
                goos = $row.goos
                goarch = $row.goarch
                libc = $row.libc
                triple = $row.triple
                native_interface_crate = $Matrix.native_interface_crate
                native_interface_version = '0.1.0'
                rust_driver_crate = $Matrix.rust_driver_crate
                rust_driver_version = '0.7.0'
                source_commit = '0123456789abcdef0123456789abcdef01234567'
                static_library = [ordered]@{
                    filename = $Matrix.static_lib_filename
                    sha256 = Get-TestHash -Path $libraryPath
                }
                header = [ordered]@{
                    filename = $Matrix.header_filename
                    sha256 = Get-TestHash -Path $headerPath
                }
                rustc_native_static_libs = if ($row.libc -eq 'musl') {
                    @('-lunwind', '-lc')
                } else {
                    @('-lsystem')
                }
                native_static_libs = if ($row.libc -eq 'musl') {
                    @('-lgcc_eh', '-lc')
                } else {
                    @('-lsystem')
                }
                toolchain = [ordered]@{
                    provider = 'microsoft'
                    manager = [ordered]@{
                        executable = 'msrustup'
                        version = 'msrustup 1.0.0'
                    }
                    channel = $MicrosoftRustChannel
                    selected_toolchain = $MicrosoftRustChannel
                    installer_package_version = '1.95.0-ms-20260618.5'
                    sysroot = "/tools/$MicrosoftRustChannel"
                    rustc_executable = "/tools/$MicrosoftRustChannel/bin/rustc"
                    cargo_executable = "/tools/$MicrosoftRustChannel/bin/cargo"
                    installer_rustc_executable = "/packages/ms-rust/tools/bin/rustc"
                    installer_cargo_executable = "/packages/ms-rust/tools/bin/cargo"
                    rustc_verbose_version = @"
rustc 1.95.0 (012345678 2026-08-01)
binary: rustc
commit-hash: 0123456789abcdef0123456789abcdef01234567
commit-date: 2026-08-01
host: test-host
release: 1.95.0
LLVM version: 21.1.0
"@
                    rustc_release = '1.95.0'
                    cargo_version = 'cargo 1.95.0'
                    target = $row.triple
                    linker = [ordered]@{
                        command = $row.c_compiler
                        executable = "/tools/$($row.c_compiler)"
                        version = "$($row.c_compiler) 1.0.0"
                    }
                }
            }
            $metadataPath = Join-Path $targetRoot 'rust-driver-native-interface-metadata.json'
            $metadata | ConvertTo-Json -Depth 8 | Set-Content $metadataPath -Encoding utf8
        }
    }

    function Update-TestMetadata {
        param(
            [Parameter(Mandatory = $true)]
            [string]$Root,

            [Parameter(Mandatory = $true)]
            [string]$TargetId,

            [Parameter(Mandatory = $true)]
            [scriptblock]$Update
        )

        $path = Join-Path $Root $TargetId 'rust-driver-native-interface-metadata.json'
        $metadata = Get-Content $path -Raw | ConvertFrom-Json
        & $Update $metadata
        $metadata | ConvertTo-Json -Depth 8 | Set-Content $path -Encoding utf8
    }
}

BeforeEach {
    $ArtifactRoot = Join-Path $TestDrive 'artifacts'
    $OutputRoot = Join-Path $TestDrive 'output'
    Remove-Item $ArtifactRoot, $OutputRoot -Recurse -Force -ErrorAction SilentlyContinue
    New-TestArtifacts -Root $ArtifactRoot
}

    It 'generates modules when all target artifacts agree with their metadata' {
        Write-TestFile `
            -Path (Join-Path $OutputRoot 'linux/amd64/native/libazurecosmosdriver.a') `
            -Content 'obsolete nested archive'
        Write-TestFile `
            -Path (Join-Path $OutputRoot 'linux/amd64/libazurecosmosdriver.syso') `
            -Content 'obsolete syso archive'

        & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot

        @($StaticTargets).Count | Should -Be 5
        @($StaticTargets.module_path | Sort-Object -Unique).Count | Should -Be 5
        foreach ($row in $StaticTargets) {
            $moduleRoot = Join-Path $OutputRoot $row.module_path
            $linkSuffix = if ($row.native_subdir) { "_$($row.native_subdir)" } else { '' }
            $linkName = "link_$($row.goos)_$($row.goarch)$linkSuffix.go"
            $expectedFiles = @(
                'azurecosmosdriver.h'
                'go.mod'
                'libazurecosmosdriver.a'
                $linkName
            ) | Sort-Object
            $actualFiles = @(
                Get-ChildItem $moduleRoot -File |
                    ForEach-Object Name |
                    Sort-Object
            )
            ($actualFiles -join '|') | Should -Be ($expectedFiles -join '|')
            @(Get-ChildItem $moduleRoot -Directory).Count | Should -Be 0
            Test-Path (Join-Path $moduleRoot 'native/libazurecosmosdriver.a') |
                Should -BeFalse
            Test-Path (Join-Path $moduleRoot 'libazurecosmosdriver.syso') |
                Should -BeFalse

            $goMod = Get-Content (Join-Path $moduleRoot 'go.mod') -Raw
            $goMod | Should -Match ([regex]::Escape(
                "module $($Matrix.module_root)/$($row.module_path)"
            ))

            $tag = "cgo && $($row.goos) && $($row.goarch)"
            if ($row.build_tag_extra) {
                $tag += " && $($row.build_tag_extra)"
            }
            $linkContent = Get-Content (Join-Path $moduleRoot $linkName) -Raw
            $linkContent | Should -Match ([regex]::Escape("//go:build $tag"))
            $linkContent | Should -Match ([regex]::Escape(
                "-L`${SRCDIR} -l$($Matrix.lib_basename)"
            ))
            $linkContent | Should -Not -Match ([regex]::Escape('`${SRCDIR}/native'))
            foreach ($systemLibrary in @(
                (Get-Content (Join-Path $ArtifactRoot $row.id `
                    'rust-driver-native-interface-metadata.json') -Raw |
                    ConvertFrom-Json).native_static_libs
            )) {
                $linkContent | Should -Match ([regex]::Escape($systemLibrary))
            }
            foreach ($runtimeFlag in @($row.static_runtime_ldflags)) {
                $linkContent | Should -Match ([regex]::Escape($runtimeFlag))
            }
        }

        $generatedContents = Get-ChildItem $OutputRoot -Recurse -File |
            ForEach-Object { Get-Content $_.FullName -Raw }
        $generatedContents | Should -Not -Match ([regex]::Escape('`${SRCDIR}/native'))

        $secondOutputRoot = Join-Path $TestDrive 'second-output'
        & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $secondOutputRoot
        (Get-TreeSnapshot -Root $OutputRoot) |
            Should -Be (Get-TreeSnapshot -Root $secondOutputRoot)
    }

    It 'writes a consolidated provenance manifest binding the release identity' {
        & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot

        $provenancePath = Join-Path $OutputRoot 'provenance.json'
        Test-Path $provenancePath | Should -BeTrue

        $provenance = Get-Content $provenancePath -Raw | ConvertFrom-Json
        $provenance.schema_version | Should -Be 2
        $provenance.source_commit | Should -Be '0123456789abcdef0123456789abcdef01234567'
        $provenance.rust_driver_crate | Should -Be $Matrix.rust_driver_crate
        $provenance.rust_driver_version | Should -Be '0.7.0'
        $provenance.native_interface_crate | Should -Be $Matrix.native_interface_crate
        $provenance.native_interface_version | Should -Be '0.1.0'
        $provenance.rust_toolchain.provider | Should -Be 'microsoft'
        $provenance.rust_toolchain.manager | Should -Be 'msrustup'
        $provenance.rust_toolchain.manager_version | Should -Be 'msrustup 1.0.0'
        $provenance.rust_toolchain.channel | Should -Be $MicrosoftRustChannel
        $provenance.rust_toolchain.installer_package_version |
            Should -Be '1.95.0-ms-20260618.5'
        $provenance.rust_toolchain.rustc_release | Should -Be '1.95.0'
        $provenance.rust_toolchain.rustc_commit_hash |
            Should -Be '0123456789abcdef0123456789abcdef01234567'
        $provenance.rust_toolchain.cargo_version | Should -Be 'cargo 1.95.0'

        @($provenance.targets).Count | Should -Be @($StaticTargets).Count
        $linuxEntry = @($provenance.targets | Where-Object { $_.id -eq 'linux-amd64-glibc' })
        $linuxEntry.Count | Should -Be 1
        $linuxEntry[0].static_library_path | Should -Be 'linux/amd64/libazurecosmosdriver.a'
        $linuxEntry[0].static_library_sha256 | Should -Match '^[0-9a-f]{64}$'
        $linuxEntry[0].header_sha256 | Should -Match '^[0-9a-f]{64}$'
        $linuxEntry[0].toolchain.selected_toolchain | Should -Be $MicrosoftRustChannel
        $linuxEntry[0].toolchain.sysroot | Should -Be "/tools/$MicrosoftRustChannel"
        $linuxEntry[0].toolchain.rustc_executable |
            Should -Be "/tools/$MicrosoftRustChannel/bin/rustc"
        $linuxEntry[0].toolchain.cargo_executable |
            Should -Be "/tools/$MicrosoftRustChannel/bin/cargo"
        $linuxEntry[0].toolchain.installer_rustc_executable |
            Should -Be '/packages/ms-rust/tools/bin/rustc'
        $linuxEntry[0].toolchain.installer_cargo_executable |
            Should -Be '/packages/ms-rust/tools/bin/cargo'
        $linuxEntry[0].toolchain.rustc_verbose_version | Should -Match 'release: 1\.95\.0'
        $linuxEntry[0].toolchain.target | Should -Be 'x86_64-unknown-linux-gnu'
        $linuxEntry[0].toolchain.linker.command | Should -Be 'gcc'
        $linuxEntry[0].toolchain.linker.executable | Should -Be '/tools/gcc'
        $linuxEntry[0].toolchain.linker.version | Should -Be 'gcc 1.0.0'
    }

    It 'generates only the selected standalone musl module' {
        & $ScriptPath `
            -ArtifactRoot $ArtifactRoot `
            -OutputRoot $OutputRoot `
            -TargetId 'linux-amd64-musl'

        Test-Path (Join-Path $OutputRoot 'linux/amd64-musl/go.mod') | Should -BeTrue
        Test-Path (Join-Path $OutputRoot 'linux/amd64-musl/libazurecosmosdriver.a') |
            Should -BeTrue
        Test-Path (Join-Path $OutputRoot 'linux/amd64/go.mod') | Should -BeFalse

        $provenance = Get-Content (Join-Path $OutputRoot 'provenance.json') -Raw | ConvertFrom-Json
        @($provenance.targets).Count | Should -Be 1
        @($provenance.targets)[0].id | Should -Be 'linux-amd64-musl'
    }

    It 'rejects an artifact ID that does not match its matrix row' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'darwin-arm64' -Update {
            param($metadata)
            $metadata.artifact_id = 'linux-amd64-glibc'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*metadata 'artifact_id' mismatch*"
    }

    It 'rejects a target triple that does not match its matrix row' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'darwin-arm64' -Update {
            param($metadata)
            $metadata.triple = 'x86_64-unknown-linux-gnu'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*metadata 'triple' mismatch*"
    }

    It 'rejects targets built from different source commits' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.source_commit = 'fedcba9876543210fedcba9876543210fedcba98'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*release identity 'source_commit' mismatch*"
    }

    It 'rejects targets carrying different package versions' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.rust_driver_version = '0.8.0'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*release identity 'rust_driver_version' mismatch*"
    }

    It 'rejects metadata without Microsoft Rust provenance' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.PSObject.Properties.Remove('toolchain')
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*metadata is missing 'toolchain'*"
    }

    It 'rejects metadata produced through upstream rustup' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.toolchain.provider = 'upstream'
            $metadata.toolchain.manager.executable = 'rustup'
            $metadata.toolchain.channel = 'stable'
            $metadata.toolchain.selected_toolchain = 'stable'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*toolchain provider must be 'microsoft'*"
    }

    It 'rejects metadata without Microsoft Rust installer package identity' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.toolchain.installer_package_version = '1.95.0'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*does not identify Microsoft Rust release '1.95.0'*"
    }

    It 'rejects metadata without manager-resolved compiler identity' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.toolchain.PSObject.Properties.Remove('rustc_executable')
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*metadata is missing non-empty 'rustc_executable'*"
    }

    It 'rejects an unpinned Microsoft Rust identity' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.toolchain.channel = 'ms-prod'
            $metadata.toolchain.selected_toolchain = 'ms-prod'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw '*Microsoft Rust channel*not explicitly pinned*'
    }

    It 'rejects a Microsoft compiler release that differs from the pinned channel' {
        foreach ($row in $StaticTargets) {
            Update-TestMetadata -Root $ArtifactRoot -TargetId $row.id -Update {
                param($metadata)
                $metadata.toolchain.rustc_release = '1.96.0'
                $metadata.toolchain.rustc_verbose_version = $metadata.toolchain.rustc_verbose_version `
                    -replace 'rustc 1\.95\.0', 'rustc 1.96.0' `
                    -replace 'release: 1\.95\.0', 'release: 1.96.0'
            }
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*Microsoft Rust release '1.96.0' does not match pinned channel*"
    }

    It 'rejects a rustc identity that contradicts its release field' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.toolchain.rustc_verbose_version = $metadata.toolchain.rustc_verbose_version `
                -replace 'release: 1\.95\.0', 'release: 1.95.1'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*rustc release '1.95.0' does not match rustc identity release '1.95.1'*"
    }

    It 'rejects targets carrying mixed Microsoft Rust releases' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.toolchain.rustc_release = '1.95.1'
            $metadata.toolchain.installer_package_version = '1.95.1-ms-20260618.5'
            $metadata.toolchain.rustc_verbose_version = $metadata.toolchain.rustc_verbose_version `
                -replace 'release: 1\.95\.0', 'release: 1.95.1'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*release identity 'rustc_release' mismatch*"
    }

    It 'rejects targets carrying mixed Microsoft Rust installer packages' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.toolchain.installer_package_version = '1.95.0-ms-20260619.1'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*release identity 'rust_toolchain_installer_package_version' mismatch*"
    }

    It 'rejects targets built from different compiler commits' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-glibc' -Update {
            param($metadata)
            $metadata.toolchain.rustc_verbose_version = $metadata.toolchain.rustc_verbose_version `
                -replace `
                    'commit-hash: 0123456789abcdef0123456789abcdef01234567',
                    'commit-hash: fedcba9876543210fedcba9876543210fedcba98'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*release identity 'rustc_commit_hash' mismatch*"
    }

    It 'rejects a static archive whose bytes do not match metadata' {
        Write-TestFile `
            -Path (Join-Path $ArtifactRoot 'linux-amd64-glibc/libazurecosmosdriver.a') `
            -Content 'tampered archive'

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw '*static library SHA256 mismatch*'
    }

    It 'rejects a header whose bytes do not match metadata' {
        Write-TestFile `
            -Path (Join-Path $ArtifactRoot 'linux-amd64-glibc/azurecosmosdriver.h') `
            -Content "int changed_header(void);`n"

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw '*header SHA256 mismatch*'
    }

    It 'rejects different target headers even when each matches its own metadata' {
        $headerPath = Join-Path $ArtifactRoot 'linux-amd64-musl/azurecosmosdriver.h'
        Write-TestFile -Path $headerPath -Content "int musl_only_header(void);`n"
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'linux-amd64-musl' -Update {
            param($metadata)
            $metadata.header.sha256 = Get-TestHash -Path $headerPath
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw '*header SHA256 differs from the other selected targets*'
    }
}
