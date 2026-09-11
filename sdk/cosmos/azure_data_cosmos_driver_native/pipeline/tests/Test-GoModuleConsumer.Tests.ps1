# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Test-GoModuleConsumer command flow' {
    BeforeAll {
        $PipelineDirectory = Split-Path -Parent $PSScriptRoot
        $ScriptPath = Join-Path $PipelineDirectory 'Test-GoModuleConsumer.ps1'

        function Write-TestFile {
            param(
                [Parameter(Mandatory = $true)]
                [string]$Path,

                [Parameter(Mandatory = $true)]
                [string]$Content
            )

            New-Item -ItemType Directory -Path (Split-Path -Parent $Path) -Force |
                Out-Null
            [IO.File]::WriteAllText(
                $Path,
                $Content,
                [Text.UTF8Encoding]::new($false)
            )
        }
    }

    BeforeEach {
        $GeneratedRoot = Join-Path $TestDrive 'generated'
        $GeneratedModule = Join-Path $GeneratedRoot 'windows/amd64'
        Write-TestFile `
            -Path (Join-Path $GeneratedModule 'go.mod') `
            -Content "module example.test/native/windows/amd64`n`ngo 1.25.0`n"
        Write-TestFile `
            -Path (Join-Path $GeneratedModule 'azurecosmosdriver.h') `
            -Content "const char *cosmos_version(void);`n"
        Write-TestFile `
            -Path (Join-Path $GeneratedModule 'libazurecosmosdriver.a') `
            -Content 'real-archive-fixture'

        $MatrixPath = Join-Path $TestDrive 'matrix.json'
        $matrix = [ordered]@{
            module_root = 'example.test/native'
            lib_basename = 'azurecosmosdriver'
            static_lib_filename = 'libazurecosmosdriver.a'
            header_filename = 'azurecosmosdriver.h'
            go_version = '1.25.0'
            targets = @(
                [ordered]@{
                    id = 'windows-amd64'
                    goos = 'windows'
                    goarch = 'amd64'
                    triple = 'x86_64-pc-windows-gnu'
                    module_path = 'windows/amd64'
                    c_compiler = 'pwsh'
                }
            )
        }
        Write-TestFile `
            -Path $MatrixPath `
            -Content ($matrix | ConvertTo-Json -Depth 6)

        $global:GoCalls = [Collections.Generic.List[string]]::new()
        $global:ConsumerCallsVersion = $false
        Mock go {
            $arguments = @($args)
            $global:GoCalls.Add($arguments -join ' ')
            $global:LASTEXITCODE = 0
            if ($arguments[0] -eq 'env') {
                return @('windows', 'amd64')
            }
            if ($arguments[0] -eq 'mod' -and $arguments[1] -eq 'vendor') {
                $vendoredModule = Join-Path (Get-Location) `
                    'vendor/example.test/native/windows/amd64'
                New-Item -ItemType Directory -Path $vendoredModule -Force |
                    Out-Null
                Copy-Item (Join-Path $GeneratedModule '*') $vendoredModule -Force
            }
            if ($arguments[0] -eq 'build') {
                $mainSource = Get-Content (Join-Path (Get-Location) 'main.go') -Raw
                $global:ConsumerCallsVersion =
                    $mainSource.Contains('C.cosmos_version()') -and
                    $mainSource.Contains('_ "example.test/native/windows/amd64"')
                $outputIndex = [Array]::IndexOf($arguments, '-o')
                New-Item -ItemType File -Path $arguments[$outputIndex + 1] -Force |
                    Out-Null
            }
        }
    }

    It 'builds direct and vendored consumers that call cosmos_version' {
        {
            & $ScriptPath `
                -GeneratedRoot $GeneratedRoot `
                -TargetId 'windows-amd64' `
                -CCompiler 'pwsh' `
                -MatrixPath $MatrixPath
        } | Should -Not -Throw

        $global:GoCalls | Should -Contain 'build -mod=mod -trimpath -o consumer-direct.exe .'
        $global:GoCalls | Should -Contain 'mod vendor'
        $global:GoCalls | Should -Contain 'build -mod=vendor -trimpath -o consumer-vendored.exe .'
        $global:ConsumerCallsVersion | Should -BeTrue
    }
}
