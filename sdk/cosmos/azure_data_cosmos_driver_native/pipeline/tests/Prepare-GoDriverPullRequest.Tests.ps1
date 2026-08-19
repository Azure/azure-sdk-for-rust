# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Prepare-GoDriverPullRequest generated tree synchronization' {
    BeforeAll {
        $PipelineDirectory = Split-Path -Parent $PSScriptRoot
        $ScriptPath = Join-Path $PipelineDirectory 'Prepare-GoDriverPullRequest.ps1'
        $MatrixPath = Join-Path $PipelineDirectory 'build-matrix.json'
        $Matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json

        function Write-TestFile {
            param(
                [Parameter(Mandatory = $true)]
                [string]$Path,

                [Parameter(Mandatory = $true)]
                [string]$Content
            )

            New-Item -ItemType Directory -Path (Split-Path -Parent $Path) -Force | Out-Null
            $normalizedContent = $Content.Replace("`r`n", "`n").Replace("`r", "`n")
            [IO.File]::WriteAllText($Path, $normalizedContent, [Text.UTF8Encoding]::new($false))
        }

        function Invoke-TestGit {
            param(
                [Parameter(Mandatory = $true)]
                [string]$Root,

                [Parameter(Mandatory = $true)]
                [string[]]$Arguments
            )

            & git -C $Root @Arguments
            if ($LASTEXITCODE -ne 0) {
                throw "git $($Arguments -join ' ') failed with exit code $LASTEXITCODE."
            }
        }

        function New-GeneratedFixture {
            param(
                [Parameter(Mandatory = $true)]
                [string]$Root
            )

            foreach ($modulePath in @($Matrix.targets.module_path | Sort-Object -Unique)) {
                $moduleRoot = Join-Path $Root $modulePath
                Write-TestFile -Path (Join-Path $moduleRoot 'go.mod') -Content @"
module $($Matrix.module_root)/$modulePath

go $($Matrix.go_version)
"@
                Write-TestFile `
                    -Path (Join-Path $moduleRoot $Matrix.header_filename) `
                    -Content "int azure_cosmos_test(void);`n"
            }

            $checksumLines = foreach ($target in $Matrix.targets) {
                $moduleRoot = Join-Path $Root $target.module_path
                $nativeRelativePath = if ($target.native_subdir) {
                    "native/$($target.native_subdir)"
                }
                else {
                    'native'
                }
                $nativeRoot = Join-Path $moduleRoot $nativeRelativePath
                $libraryPath = Join-Path $nativeRoot $Matrix.static_lib_filename
                Write-TestFile -Path $libraryPath -Content "archive-$($target.id)"
                Write-TestFile `
                    -Path (Join-Path $nativeRoot $Matrix.header_filename) `
                    -Content "int azure_cosmos_test(void);`n"

                $tag = "cgo && $($target.goos) && $($target.goarch)"
                if ($target.build_tag_extra) {
                    $tag += " && $($target.build_tag_extra)"
                }
                $linkSuffix = if ($target.native_subdir) { "_$($target.native_subdir)" } else { '' }
                $linkPath = Join-Path $moduleRoot "link_$($target.goos)_$($target.goarch)$linkSuffix.go"
                $runtimeFlags = if ($target.PSObject.Properties.Name -contains 'static_runtime_ldflags') {
                    " $(@($target.static_runtime_ldflags) -join ' ')"
                }
                else {
                    ''
                }
                $linkContent = @"
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//go:build $tag

package driver

// #cgo LDFLAGS: -L`${SRCDIR}/$nativeRelativePath -l$($Matrix.lib_basename)$runtimeFlags
// #include "$($Matrix.header_filename)"
import "C"
"@
                Write-TestFile -Path $linkPath -Content "$linkContent`n"

                $relativeLibraryPath = "$($target.module_path)/$nativeRelativePath/$($Matrix.static_lib_filename)"
                $hash = (Get-FileHash $libraryPath -Algorithm SHA256).Hash.ToLowerInvariant()
                "$hash  $relativeLibraryPath"
            }
            Write-TestFile -Path (Join-Path $Root 'SHA256SUMS') -Content ($checksumLines -join "`n")
            Write-TestFile -Path (Join-Path $Root 'provenance.json') -Content "{`n  `"schema_version`": 1`n}`n"
        }
    }

    BeforeEach {
        Mock go {
            $global:LASTEXITCODE = 0
        }

        $GeneratedRoot = Join-Path $TestDrive 'generated'
        $CheckoutRoot = Join-Path $TestDrive 'checkout'
        Remove-Item $GeneratedRoot, $CheckoutRoot -Recurse -Force -ErrorAction SilentlyContinue
        New-GeneratedFixture -Root $GeneratedRoot
        Write-TestFile `
            -Path (Join-Path $GeneratedRoot '_manifest/spdx_2.2/manifest.spdx.json') `
            -Content "{}`n"
        Write-TestFile `
            -Path (Join-Path $GeneratedRoot '_manifest/spdx_2.2/bsi.cose') `
            -Content "signed evidence`n"

        New-Item -ItemType Directory -Path $CheckoutRoot | Out-Null
        Invoke-TestGit -Root $CheckoutRoot -Arguments @('init', '--quiet')
        Invoke-TestGit -Root $CheckoutRoot -Arguments @('config', 'user.name', 'Pipeline Test')
        Invoke-TestGit -Root $CheckoutRoot -Arguments @('config', 'user.email', 'pipeline-test@example.com')
        Write-TestFile -Path (Join-Path $CheckoutRoot 'README.md') -Content "hand maintained`n"
        Write-TestFile `
            -Path (Join-Path $CheckoutRoot 'linux/arm64/retired-link.go') `
            -Content "package retired`n"
        Write-TestFile `
            -Path (Join-Path $CheckoutRoot 'windows/arm64/native/libazurecosmosdriver.a') `
            -Content 'retired archive'
        Invoke-TestGit -Root $CheckoutRoot -Arguments @('add', '-A')
        Invoke-TestGit -Root $CheckoutRoot -Arguments @('commit', '--quiet', '-m', 'Seed downstream checkout')
    }

    It 'publishes all 1ES evidence while preserving hand-maintained files' {
        & $ScriptPath `
            -GeneratedRoot $GeneratedRoot `
            -CheckoutRoot $CheckoutRoot `
            -MatrixPath $MatrixPath

        $stagedChanges = @(& git -C $CheckoutRoot diff --cached --name-status)
        $stagedChanges | Should -Contain "D`tlinux/arm64/retired-link.go"
        $stagedChanges | Should -Contain "D`twindows/arm64/native/libazurecosmosdriver.a"
        Get-Content (Join-Path $CheckoutRoot 'README.md') -Raw | Should -Be "hand maintained`n"
        $stagedChanges | Where-Object { $_ -match 'README\.md$' } | Should -BeNullOrEmpty
        Get-Content (Join-Path $CheckoutRoot '_manifest/spdx_2.2/manifest.spdx.json') -Raw |
            Should -Be "{}`n"
        Get-Content (Join-Path $CheckoutRoot '_manifest/spdx_2.2/bsi.cose') -Raw |
            Should -Be "signed evidence`n"
        $stagedChanges | Should -Contain "A`t_manifest/spdx_2.2/manifest.spdx.json"
        $stagedChanges | Should -Contain "A`t_manifest/spdx_2.2/bsi.cose"
        $stagedChanges | Should -Contain "A`tprovenance.json"
    }

    It 'rejects unexpected files outside the managed roots' {
        Write-TestFile -Path (Join-Path $GeneratedRoot 'unexpected.txt') -Content "unexpected`n"

        {
            & $ScriptPath `
                -GeneratedRoot $GeneratedRoot `
                -CheckoutRoot $CheckoutRoot `
                -MatrixPath $MatrixPath
        } | Should -Throw '*unexpected.txt*'
    }

    It 'requires the standard 1ES SPDX manifest' {
        Remove-Item (Join-Path $GeneratedRoot '_manifest/spdx_2.2/manifest.spdx.json')

        {
            & $ScriptPath `
                -GeneratedRoot $GeneratedRoot `
                -CheckoutRoot $CheckoutRoot `
                -MatrixPath $MatrixPath
        } | Should -Throw '*missing the required 1ES manifest*'
    }
}
