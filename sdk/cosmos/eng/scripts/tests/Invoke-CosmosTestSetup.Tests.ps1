# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

Describe 'Cosmos live profile regions' {
    BeforeEach {
        $scriptPath = ([System.IO.Path]::Combine($PSScriptRoot, '..', 'Invoke-CosmosTestSetup.ps1'))
        $parseErrors = $null
        $ast = [System.Management.Automation.Language.Parser]::ParseFile($scriptPath, [ref]$null, [ref]$parseErrors)
        if ($parseErrors.Count -ne 0) {
            throw "Setup script parse errors: $parseErrors"
        }
        $definition = $ast.Find({
                param($node)
                $node -is [System.Management.Automation.Language.FunctionDefinitionAst] -and
                $node.Name -eq 'Get-CosmosE2eLiveRegions'
            }, $false)
        if (-not $definition) {
            throw 'Missing live region validation function.'
        }
        # Load only the pure function, never setup imports, orchestration, or live calls.
        . ([scriptblock]::Create($definition.Extent.Text))

        function Assert-Rejected {
            param([AllowNull()][AllowEmptyString()][string] $Raw, [string[]] $ExpectedRegions)
            $rejected = $false
            try {
                Get-CosmosE2eLiveRegions -RawRegions $Raw -ProfileRegions $ExpectedRegions | Out-Null
            }
            catch {
                $rejected = $true
            }
            if (-not $rejected) {
                throw "Expected rejection for regions '$Raw' and profile '$($ExpectedRegions -join ';')'."
            }
        }
    }

    It 'projects both shared single-region profiles without changing their hosted definitions' {
        foreach ($profileId in @('smokeTests', 'coreOperations')) {
            $path = ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..', 'e2e_tests', 'profiles', "$profileId.json"))
            $profileDocument = Get-Content $path -Raw | ConvertFrom-Json
            if (@($profileDocument.accounts).Count -ne 1 -or @($profileDocument.accounts[0].regions).Count -ne 1 -or
                $profileDocument.accounts[0].regions[0].name -cne 'East US') {
                throw 'Shared profiles must retain the single hosted East US account.'
            }
            foreach ($live in @('East US 2', 'West US 3')) {
                $actual = @(Get-CosmosE2eLiveRegions $live @($profileDocument.accounts[0].regions.name))
                if ($actual.Count -ne 1 -or $actual[0] -cne $live) {
                    throw "Expected the configured live region '$live'."
                }
            }
        }
    }

    It 'accepts matching multi-region identity and order with normalized names' {
        $actual = @(Get-CosmosE2eLiveRegions ' eastus2 ; WEST US 3 ' @('East US 2', 'West US 3'))
        if (($actual -join ';') -cne 'eastus2;WEST US 3') {
            throw 'Region order must be preserved.'
        }
    }

    It 'rejects mismatched reordered and wrong-count regions' {
        foreach ($raw in @('West US 3;East US 2', 'East US;West US 3', 'East US 2')) {
            Assert-Rejected $raw @('East US 2', 'West US 3')
        }
        Assert-Rejected 'East US 2;West US 3' @('East US')
    }

    It 'rejects missing blank and empty region entries' {
        foreach ($raw in @($null, '', ' ', ';East US 2', 'East US 2;', 'East US 2;;West US 3', 'East US 2; ;West US 3')) {
            Assert-Rejected $raw @('East US 2', 'West US 3')
        }
    }

    It 'rejects duplicate region identities' {
        Assert-Rejected 'East US 2;eastus2' @('East US 2', 'West US 3')
        Assert-Rejected 'East US 2; East US 2 ' @('East US 2', 'West US 3')
    }

    It 'rejects missing empty and duplicate profile regions' {
        Assert-Rejected 'East US 2' @()
        Assert-Rejected 'East US 2' @('')
        Assert-Rejected 'East US 2;West US 3' @('East US', 'eastus')
    }

    It 'uses the Bicep single-region default in the AAD matrix' {
        $root = ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..'))
        $matrix = Get-Content ([System.IO.Path]::Combine($root, 'e2e-live-aad-matrix.json')) -Raw | ConvertFrom-Json
        $bicep = Get-Content ([System.IO.Path]::Combine($root, 'test-resources.bicep')) -Raw
        if ($matrix.matrix.AZURE_COSMOS_ACCOUNT_REGIONS[0] -cne 'East US 2' -or
            $bicep -notmatch "var singleRegionConfiguration =\s*\[\s*\{\s*locationName: 'East US 2'") {
            throw 'AAD region metadata must match the provisioned single-region default.'
        }
    }
}