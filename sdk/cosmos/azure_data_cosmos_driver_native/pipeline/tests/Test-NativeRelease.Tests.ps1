# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Test-NativeRelease' {
    BeforeAll {
        $PipelineDirectory = Split-Path -Parent $PSScriptRoot
        $ScriptPath = [System.IO.Path]::Combine(
            $PipelineDirectory,
            'Test-NativeRelease.ps1'
        )

        function New-ReleaseFixture {
            param(
                [string] $Version = '0.2.1',
                [string] $ChangelogStatus = 'Unreleased',
                [string] $Header = 'int cosmos_test(void);'
            )

            $root = [System.IO.Path]::Combine(
                $TestDrive,
                [System.Guid]::NewGuid().ToString('N')
            )
            $crate = [System.IO.Path]::Combine(
                $root,
                'sdk',
                'cosmos',
                'azure_data_cosmos_driver_native'
            )
            $include = [System.IO.Path]::Combine($crate, 'include')
            New-Item -ItemType Directory -Force -Path $include | Out-Null
            @"
[package]
name = "azure_data_cosmos_driver_native"
version = "$Version"

[lib]
name = "azurecosmosdriver"
"@ | Set-Content ([System.IO.Path]::Combine($crate, 'Cargo.toml'))
            @"
# Release History

## $Version ($ChangelogStatus)

### Features Added

- Added the native release.
"@ | Set-Content ([System.IO.Path]::Combine($crate, 'CHANGELOG.md'))
            @"
#define AZURECOSMOSDRIVER_H_VERSION "$Version"
$Header
"@ | Set-Content (
                [System.IO.Path]::Combine($include, 'azurecosmosdriver.h')
            )
            $root
        }

        function New-TestGitInvoker {
            {
                param($RepositoryRoot, $Arguments)

                $command = $Arguments -join ' '
                $global:GitInvocations.Add($command)
                $response = $global:GitResponses[$command]
                if ($null -eq $response) {
                    throw "Unexpected git command: $command"
                }
                @($response)
            }
        }
    }

    BeforeEach {
        $global:GitResponses = @{}
        $global:GitInvocations = [System.Collections.Generic.List[string]]::new()
        $GitInvoker = New-TestGitInvoker
    }

    It 'accepts matching development versions without querying git' {
        $root = New-ReleaseFixture

        {
            & $ScriptPath -RepositoryRoot $root -GitInvoker $GitInvoker
        } | Should -Not -Throw

        $global:GitInvocations | Should -BeNullOrEmpty
    }

    It 'accepts the pre-0.2 bootstrap baseline without a changelog' {
        $root = New-ReleaseFixture -Version '0.1.0'
        Remove-Item (
            [System.IO.Path]::Combine(
                $root,
                'sdk',
                'cosmos',
                'azure_data_cosmos_driver_native',
                'CHANGELOG.md'
            )
        )

        {
            & $ScriptPath -RepositoryRoot $root -GitInvoker $GitInvoker
        } | Should -Not -Throw
    }

    It 'requires a changelog starting with version 0.2.0' {
        $root = New-ReleaseFixture -Version '0.2.0'
        Remove-Item (
            [System.IO.Path]::Combine(
                $root,
                'sdk',
                'cosmos',
                'azure_data_cosmos_driver_native',
                'CHANGELOG.md'
            )
        )

        {
            & $ScriptPath -RepositoryRoot $root -GitInvoker $GitInvoker
        } | Should -Throw "*must contain a '0.2.0' release heading*"
    }

    It 'rejects a header version that differs from Cargo' {
        $root = New-ReleaseFixture
        $headerPath = [System.IO.Path]::Combine(
            $root,
            'sdk',
            'cosmos',
            'azure_data_cosmos_driver_native',
            'include',
            'azurecosmosdriver.h'
        )
        (Get-Content $headerPath -Raw).Replace('0.2.1', '0.2.0') |
            Set-Content $headerPath

        {
            & $ScriptPath -RepositoryRoot $root
        } | Should -Throw "*Header version '0.2.0' does not match Cargo version '0.2.1'*"
    }

    It 'rejects a changelog without the Cargo version' {
        $root = New-ReleaseFixture
        $changelogPath = [System.IO.Path]::Combine(
            $root,
            'sdk',
            'cosmos',
            'azure_data_cosmos_driver_native',
            'CHANGELOG.md'
        )
        (Get-Content $changelogPath -Raw).Replace('0.2.1', '0.2.0') |
            Set-Content $changelogPath

        {
            & $ScriptPath -RepositoryRoot $root
        } | Should -Throw "*must contain a '0.2.1' release heading*"
    }

    It 'rejects a release tag that differs from Cargo' {
        $root = New-ReleaseFixture -ChangelogStatus '2026-09-28'

        {
            & $ScriptPath `
                -RepositoryRoot $root `
                -SourceBranch 'refs/tags/azure_data_cosmos_driver_native@0.2.0' `
                -SourceVersion 'current' `
                -GitInvoker $GitInvoker
        } | Should -Throw "*does not match expected tag*"
    }

    It 'rejects a release whose changelog is still unreleased' {
        $root = New-ReleaseFixture

        {
            & $ScriptPath `
                -RepositoryRoot $root `
                -SourceBranch 'refs/tags/azure_data_cosmos_driver_native@0.2.1' `
                -SourceVersion 'current' `
                -GitInvoker $GitInvoker
        } | Should -Throw "*must have a release date*"
    }

    It 'rejects a lightweight release tag' {
        $root = New-ReleaseFixture -ChangelogStatus '2026-09-28'
        $global:GitResponses['cat-file -t refs/tags/azure_data_cosmos_driver_native@0.2.1'] = 'commit'

        {
            & $ScriptPath `
                -RepositoryRoot $root `
                -SourceBranch 'refs/tags/azure_data_cosmos_driver_native@0.2.1' `
                -SourceVersion 'current' `
                -GitInvoker $GitInvoker
        } | Should -Throw "*must be annotated*"
    }

    It 'rejects a tag that points to another commit' {
        $root = New-ReleaseFixture -ChangelogStatus '2026-09-28'
        $global:GitResponses['cat-file -t refs/tags/azure_data_cosmos_driver_native@0.2.1'] = 'tag'
        $global:GitResponses['rev-list -n 1 azure_data_cosmos_driver_native@0.2.1'] = 'other'

        {
            & $ScriptPath `
                -RepositoryRoot $root `
                -SourceBranch 'refs/tags/azure_data_cosmos_driver_native@0.2.1' `
                -SourceVersion 'current' `
                -GitInvoker $GitInvoker
        } | Should -Throw "*points to 'other', not 'current'*"
    }

    It 'accepts a patch release when the FFI header is unchanged' {
        $root = New-ReleaseFixture -ChangelogStatus '2026-09-28'
        $global:GitResponses['cat-file -t refs/tags/azure_data_cosmos_driver_native@0.2.1'] = 'tag'
        $global:GitResponses['rev-list -n 1 azure_data_cosmos_driver_native@0.2.1'] = 'current'
        $global:GitResponses['tag --list azure_data_cosmos_driver_native@*'] = @(
            'azure_data_cosmos_driver_native@0.2.0'
            'azure_data_cosmos_driver_native@0.2.1'
        )
        $global:GitResponses['show azure_data_cosmos_driver_native@0.2.0:sdk/cosmos/azure_data_cosmos_driver_native/include/azurecosmosdriver.h'] = @(
            '#define AZURECOSMOSDRIVER_H_VERSION "0.2.0"'
            'int cosmos_test(void);'
        )

        {
            & $ScriptPath `
                -RepositoryRoot $root `
                -SourceBranch 'refs/tags/azure_data_cosmos_driver_native@0.2.1' `
                -SourceVersion 'current' `
                -GitInvoker $GitInvoker
        } | Should -Not -Throw
    }

    It 'rejects a patch release when the FFI header changed' {
        $root = New-ReleaseFixture -ChangelogStatus '2026-09-28'
        $global:GitResponses['cat-file -t refs/tags/azure_data_cosmos_driver_native@0.2.1'] = 'tag'
        $global:GitResponses['rev-list -n 1 azure_data_cosmos_driver_native@0.2.1'] = 'current'
        $global:GitResponses['tag --list azure_data_cosmos_driver_native@*'] = @(
            'azure_data_cosmos_driver_native@0.2.0'
        )
        $global:GitResponses['show azure_data_cosmos_driver_native@0.2.0:sdk/cosmos/azure_data_cosmos_driver_native/include/azurecosmosdriver.h'] = @(
            '#define AZURECOSMOSDRIVER_H_VERSION "0.2.0"'
        )

        {
            & $ScriptPath `
                -RepositoryRoot $root `
                -SourceBranch 'refs/tags/azure_data_cosmos_driver_native@0.2.1' `
                -SourceVersion 'current' `
                -GitInvoker $GitInvoker
        } | Should -Throw "*changes the C FFI header*"
    }

    It 'rejects stable releases until the stable FFI classifier exists' {
        $root = New-ReleaseFixture `
            -Version '1.0.0' `
            -ChangelogStatus '2026-09-28'

        {
            & $ScriptPath `
                -RepositoryRoot $root `
                -SourceBranch 'refs/tags/azure_data_cosmos_driver_native@1.0.0' `
                -SourceVersion 'current' `
                -GitInvoker $GitInvoker
        } | Should -Throw "*require an additive/breaking FFI classifier*"
    }

    It 'accepts a pre-1.0 minor release with an FFI change' {
        $root = New-ReleaseFixture `
            -Version '0.3.0' `
            -ChangelogStatus '2026-09-28'
        $global:GitResponses['cat-file -t refs/tags/azure_data_cosmos_driver_native@0.3.0'] = 'tag'
        $global:GitResponses['rev-list -n 1 azure_data_cosmos_driver_native@0.3.0'] = 'current'
        $global:GitResponses['tag --list azure_data_cosmos_driver_native@*'] = @(
            'azure_data_cosmos_driver_native@0.2.9'
        )

        {
            & $ScriptPath `
                -RepositoryRoot $root `
                -SourceBranch 'refs/tags/azure_data_cosmos_driver_native@0.3.0' `
                -SourceVersion 'current' `
                -GitInvoker $GitInvoker
        } | Should -Not -Throw
    }
}
