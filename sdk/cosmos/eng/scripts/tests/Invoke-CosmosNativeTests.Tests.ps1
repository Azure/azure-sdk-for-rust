# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Cosmos native test runner' {
    BeforeAll {
        $ScriptPath = ([System.IO.Path]::Combine($PSScriptRoot, '..', 'Invoke-CosmosNativeTests.ps1'))
        function cargo {}
        function cmake {}
        function ctest {}
    }

    BeforeEach {
        $BuildDirectory = ([System.IO.Path]::Combine($TestDrive, 'native build'))
        $ResultsPath = ([System.IO.Path]::Combine($BuildDirectory, 'results.xml'))
        $OriginalTfBuild = $env:TF_BUILD
        $env:TF_BUILD = 'true'
        Mock cargo { $global:LASTEXITCODE = 0 }
        Mock cmake {
            New-Item -ItemType Directory -Force -Path $BuildDirectory | Out-Null
            $global:LASTEXITCODE = 0
        }
        Mock ctest {
            '<testsuite tests="1" failures="0"><testcase name="native" /></testsuite>' |
                Set-Content $ResultsPath
            $global:LASTEXITCODE = 0
        }
        Mock Write-Host {}
    }

    AfterEach {
        $env:TF_BUILD = $OriginalTfBuild
    }

    It 'runs both suites and publishes the C ABI report' {
        & $ScriptPath -BuildDirectory $BuildDirectory
        Should -Invoke cargo -Exactly 1
        Should -Invoke cmake -Exactly 2
        Should -Invoke ctest -Exactly 1
        Should -Invoke Write-Host -Exactly 1 -ParameterFilter {
            $Object -like '##vso[[]results.publish type=JUnit;*' -and $Object.EndsWith($ResultsPath)
        }
    }

    It 'stops before CMake when Rust tests fail' {
        Mock cargo { $global:LASTEXITCODE = 1 }
        { & $ScriptPath -BuildDirectory $BuildDirectory } | Should -Throw '*Native test setup failed*'
        Should -Invoke cmake -Exactly 0
        Should -Invoke ctest -Exactly 0
    }

    It 'stops before CTest when CMake fails' {
        Mock cmake { $global:LASTEXITCODE = 1 }
        { & $ScriptPath -BuildDirectory $BuildDirectory } | Should -Throw '*Native test setup failed*'
        Should -Invoke ctest -Exactly 0
    }

    It 'publishes failed C ABI results before failing the hook' {
        Mock ctest {
            '<testsuite tests="1" failures="1"><testcase name="native"><failure /></testcase></testsuite>' |
                Set-Content $ResultsPath
            $global:LASTEXITCODE = 8
        }
        { & $ScriptPath -BuildDirectory $BuildDirectory } | Should -Throw '*exit code 8*'
        Should -Invoke Write-Host -Exactly 1 -ParameterFilter {
            $Object -like '##vso[[]results.publish type=JUnit;*' -and $Object.EndsWith($ResultsPath)
        }
    }

    It 'rejects a missing report instead of publishing stale results' {
        New-Item -ItemType Directory -Force -Path $BuildDirectory | Out-Null
        Set-Content $ResultsPath 'stale results'
        Mock ctest { $global:LASTEXITCODE = 0 }
        { & $ScriptPath -BuildDirectory $BuildDirectory } | Should -Throw '*did not produce*'
        Test-Path $ResultsPath | Should -BeFalse
        Should -Invoke Write-Host -Exactly 0 -ParameterFilter {
            $Object -like '##vso[[]results.publish*'
        }
    }
}
