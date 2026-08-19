# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'New-GoModules artifact provenance validation' {
BeforeAll {
    $PipelineDirectory = Split-Path -Parent $PSScriptRoot
    $ScriptPath = Join-Path $PipelineDirectory 'New-GoModules.ps1'
    $MatrixPath = Join-Path $PipelineDirectory 'build-matrix.json'
    $Matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json

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

    function New-TestArtifacts {
        param(
            [Parameter(Mandatory = $true)]
            [string]$Root
        )

        foreach ($row in $Matrix.targets) {
            $targetRoot = Join-Path $Root $row.id
            $headerPath = Join-Path $targetRoot $Matrix.header_filename
            $libraryPath = Join-Path $targetRoot $Matrix.static_lib_filename
            Write-TestFile -Path $headerPath -Content "int azure_cosmos_test(void);`n"
            Write-TestFile -Path $libraryPath -Content "archive-$($row.id)"

            $metadata = [ordered]@{
                schema_version = 3
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
            }
            $metadataPath = Join-Path $targetRoot 'rust-driver-native-interface-metadata.json'
            $metadata | ConvertTo-Json -Depth 6 | Set-Content $metadataPath -Encoding utf8
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
        $metadata | ConvertTo-Json -Depth 6 | Set-Content $path -Encoding utf8
    }
}

BeforeEach {
    $ArtifactRoot = Join-Path $TestDrive 'artifacts'
    $OutputRoot = Join-Path $TestDrive 'output'
    Remove-Item $ArtifactRoot, $OutputRoot -Recurse -Force -ErrorAction SilentlyContinue
    New-TestArtifacts -Root $ArtifactRoot
}

    It 'generates modules when all target artifacts agree with their metadata' {
        & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot

        Test-Path (Join-Path $OutputRoot 'windows/amd64/native/libazurecosmosdriver.a') |
            Should -BeTrue
        Test-Path (Join-Path $OutputRoot 'linux/amd64/native/libazurecosmosdriver.a') |
            Should -BeTrue
        Test-Path (Join-Path $OutputRoot 'linux/amd64-musl/native/libazurecosmosdriver.a') |
            Should -BeTrue
        Get-Content (Join-Path $OutputRoot 'linux/amd64-musl/go.mod') -Raw |
            Should -Match 'module github\.com/Azure/azure-cosmos-driver/linux/amd64-musl'
        Get-Content (Join-Path $OutputRoot 'linux/amd64-musl/link_linux_amd64.go') -Raw |
            Should -Not -Match 'cosmos_musl'
        Get-Content (Join-Path $OutputRoot 'linux/amd64-musl/link_linux_amd64.go') -Raw |
            Should -Match ([regex]::Escape('-lgcc_eh -lc'))
        Get-Content (Join-Path $OutputRoot 'linux/amd64-musl/link_linux_amd64.go') -Raw |
            Should -Not -Match ([regex]::Escape('-lunwind'))
    }

    It 'writes a consolidated provenance manifest binding the release identity' {
        & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot

        $provenancePath = Join-Path $OutputRoot 'provenance.json'
        Test-Path $provenancePath | Should -BeTrue

        $provenance = Get-Content $provenancePath -Raw | ConvertFrom-Json
        $provenance.schema_version | Should -Be 1
        $provenance.source_commit | Should -Be '0123456789abcdef0123456789abcdef01234567'
        $provenance.rust_driver_crate | Should -Be $Matrix.rust_driver_crate
        $provenance.rust_driver_version | Should -Be '0.7.0'
        $provenance.native_interface_crate | Should -Be $Matrix.native_interface_crate
        $provenance.native_interface_version | Should -Be '0.1.0'

        @($provenance.targets).Count | Should -Be @($Matrix.targets).Count
        $windowsEntry = @($provenance.targets | Where-Object { $_.id -eq 'windows-amd64' })
        $windowsEntry.Count | Should -Be 1
        $windowsEntry[0].static_library_sha256 | Should -Match '^[0-9a-f]{64}$'
        $windowsEntry[0].header_sha256 | Should -Match '^[0-9a-f]{64}$'
    }

    It 'generates only the selected standalone musl module' {
        & $ScriptPath `
            -ArtifactRoot $ArtifactRoot `
            -OutputRoot $OutputRoot `
            -TargetId 'linux-amd64-musl'

        Test-Path (Join-Path $OutputRoot 'linux/amd64-musl/go.mod') | Should -BeTrue
        Test-Path (Join-Path $OutputRoot 'linux/amd64/go.mod') | Should -BeFalse

        $provenance = Get-Content (Join-Path $OutputRoot 'provenance.json') -Raw | ConvertFrom-Json
        @($provenance.targets).Count | Should -Be 1
        @($provenance.targets)[0].id | Should -Be 'linux-amd64-musl'
    }

    It 'rejects an artifact ID that does not match its matrix row' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'windows-amd64' -Update {
            param($metadata)
            $metadata.artifact_id = 'linux-amd64-glibc'
        }

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw "*metadata 'artifact_id' mismatch*"
    }

    It 'rejects a target triple that does not match its matrix row' {
        Update-TestMetadata -Root $ArtifactRoot -TargetId 'windows-amd64' -Update {
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

    It 'rejects a static archive whose bytes do not match metadata' {
        Write-TestFile `
            -Path (Join-Path $ArtifactRoot 'windows-amd64/libazurecosmosdriver.a') `
            -Content 'tampered archive'

        { & $ScriptPath -ArtifactRoot $ArtifactRoot -OutputRoot $OutputRoot } |
            Should -Throw '*static library SHA256 mismatch*'
    }

    It 'rejects a header whose bytes do not match metadata' {
        Write-TestFile `
            -Path (Join-Path $ArtifactRoot 'windows-amd64/azurecosmosdriver.h') `
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
