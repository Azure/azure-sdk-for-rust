# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
#Requires -Version 7.4

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$scriptUnderTest = ([System.IO.Path]::Combine($PSScriptRoot, 'Write-CosmosE2eReport.ps1'))
$e2eRoot = (Resolve-Path ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', 'e2e_tests'))).Path
$implementation = Get-Content ([System.IO.Path]::Combine($e2eRoot, 'implementations', 'rust.json')) -Raw | ConvertFrom-Json
$implementationById = @{}
foreach ($entry in @($implementation.scenarios)) {
    $implementationById[[string]$entry.id] = [string]$entry.test
}
$selected = @(Get-ChildItem ([System.IO.Path]::Combine($e2eRoot, 'scenarios')) -Recurse -Filter '*.json' |
    ForEach-Object { Get-Content $_.FullName -Raw | ConvertFrom-Json } |
    Where-Object {
        'smokeTests' -in @($_.profiles) -and $_.backends.azureLive.applicability -ne 'notApplicable'
    })
$tempRoot = ([System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "cosmos-e2e-report-$([Guid]::NewGuid().ToString('N'))"))
$junit = ([System.IO.Path]::Combine($tempRoot, 'junit'))
$output = ([System.IO.Path]::Combine($tempRoot, 'output'))
New-Item -ItemType Directory -Path $junit -Force | Out-Null
try {
    $testCases = foreach ($scenario in $selected) {
        $name = $implementationById[[string]$scenario.id]
        "<testcase classname=`"e2e_test_cases`" name=`"$name`" time=`"0.1`" />"
    }
    "<testsuite>$($testCases -join '')</testsuite>" | Set-Content ([System.IO.Path]::Combine($junit, 'e2e.xml'))

    & pwsh -NoLogo -NoProfile -File $scriptUnderTest `
        -Profile smokeTests `
        -Backend azureLive `
        -Account sessionSingleRegion `
        -Runtime sdkDefault `
        -Client sdkDefault `
        -JUnitDirectory $junit `
        -OutputDirectory $output
    if ($LASTEXITCODE -ne 0) {
        throw "complete required coverage unexpectedly failed with exit code $LASTEXITCODE"
    }
    $report = Get-Content ([System.IO.Path]::Combine($output, 'coverage.json')) -Raw | ConvertFrom-Json
    if ($report.gate -ne 'passed' -or @($report.scenarios).Count -ne $selected.Count) {
        throw 'complete required coverage did not produce the expected passing report'
    }

    $required = @($selected | Where-Object { $_.backends.azureLive.applicability -eq 'required' })[0]
    $missingTest = $implementationById[[string]$required.id]
    $missingNeedle = 'name="' + $missingTest + '"'
    $remainingTestCases = @($testCases | Where-Object { -not $_.Contains($missingNeedle) })
    "<testsuite>$($remainingTestCases -join '')</testsuite>" |
        Set-Content ([System.IO.Path]::Combine($junit, 'e2e.xml'))
    & pwsh -NoLogo -NoProfile -File $scriptUnderTest `
        -Profile smokeTests `
        -Backend azureLive `
        -Account sessionSingleRegion `
        -Runtime sdkDefault `
        -Client sdkDefault `
        -JUnitDirectory $junit `
        -OutputDirectory $output
    if ($LASTEXITCODE -eq 0) {
        throw "missing required scenario '$($required.id)' did not fail the coverage gate"
    }
    $report = Get-Content ([System.IO.Path]::Combine($output, 'coverage.json')) -Raw | ConvertFrom-Json
    if ($report.gate -ne 'failed' -or -not (@($report.errors) -match [regex]::Escape([string]$required.id))) {
        throw 'missing required coverage was not identified in the report'
    }

    Write-Host 'Cosmos E2E report tests passed.'
}
finally {
    Remove-Item $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
}
exit 0
