# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

<#
.SYNOPSIS
    (Re)creates the fixed Cosmos DB accounts used by the azure-sdk-for-rust Cosmos live
    tests and outputs the fixed-accounts JSON (endpoints + keys) for the ADO secret.

.DESCRIPTION
    The Rust Cosmos live tests run against fixed, self-owned accounts in a permanent
    subscription/resource group (default: sdk-ci) instead of accounts deployed per-run
    into the (frequently rotating) ephemeral test tenant. This script is re-run
    whenever an account needs to be created or a key rotated, and:
      1. Ensures the resource group (default: sdk-ci) exists (created if missing).
      2. Creates (idempotently) one Cosmos account per entry in the definition file,
         with the requested consistency / multi-write / multi-region / automatic-failover
         configuration.
      3. Ensures the shared test database (definition.database, default: shared-test-db)
         exists on each account, matching what sdk/cosmos/test-resources.bicep used to
         create per-run.
      4. Reads each account's endpoint + primary (and optional secondary) key.
      5. Assembles the versioned account JSON (matching
         sdk/cosmos/pipeline/live-test-accounts.schema.json) and emits it (to stdout,
         and to -OutputPath if provided).

    This script does NOT touch any ADO variable group. Update the fixed-accounts secret
    (see sdk/cosmos/pipeline/README.md) manually with the JSON it outputs.

    Uses the Az PowerShell modules (Az.Accounts, Az.Resources, Az.CosmosDB).

.PARAMETER SubscriptionId
    Subscription hosting the resource group and the Cosmos accounts. Must be a
    permanent, self-owned subscription - NOT the ephemeral test tenant.

.PARAMETER ResourceGroupName
    Resource group for the accounts. Created if it does not exist. Defaults to 'sdk-ci'.

.PARAMETER Location
    Optional override for the primary/write region. When omitted, the primary region
    comes from the definition's regionDefaults (single source of truth; matches the
    regions sdk/cosmos/test-resources.bicep used to deploy).

.PARAMETER SecondaryLocation
    Optional override for the secondary region of multi-region accounts. When omitted,
    it comes from the definition's regionDefaults.multiRegion.

.PARAMETER DefinitionPath
    Path to the account definition JSON. Defaults to the file next to this script.

.PARAMETER AccountNamePrefix
    Prefix for the globally-unique Cosmos account names. Defaults to 'sdkci'.

.PARAMETER OutputPath
    Optional path to write the assembled JSON to. The JSON is always also written to
    stdout. NOTE: the JSON contains account keys - treat any file you write as a secret.

.EXAMPLE
    # Create/refresh accounts and write the JSON to a file, then update the secret manually
    ./New-CosmosLiveTestAccounts.ps1 -SubscriptionId <sub> -OutputPath ./accounts.json

.EXAMPLE
    # Dry run - creates nothing, prints the assembled JSON with keys stubbed
    ./New-CosmosLiveTestAccounts.ps1 -SubscriptionId <sub> -WhatIf

.NOTES
    Requires: PowerShell 7+, Az modules, and Contributor on the subscription.
    Idempotent: safe to re-run.
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [Parameter(Mandatory = $true)]
    [string] $SubscriptionId,

    [string] $ResourceGroupName = 'sdk-ci',

    [string] $Location,

    [string] $SecondaryLocation,

    [string] $DefinitionPath = ([System.IO.Path]::Combine($PSScriptRoot, 'cosmos-live-test-accounts.definition.json')),

    [ValidatePattern('^[a-z0-9]{1,10}$')]
    [string] $AccountNamePrefix = 'sdkci',

    [string] $OutputPath
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

function Write-Info([string]$msg) { Write-Host "==> $msg" -ForegroundColor Cyan }

# --- Prerequisites -----------------------------------------------------------
foreach ($m in @('Az.Accounts', 'Az.Resources', 'Az.CosmosDB')) {
    if (-not (Get-Module -ListAvailable -Name $m)) {
        throw "Required module '$m' is not installed. Install with: Install-Module $m -Scope CurrentUser"
    }
}

if (-not (Test-Path $DefinitionPath)) { throw "Definition file not found: $DefinitionPath" }
$definition = Get-Content -Raw -Path $DefinitionPath | ConvertFrom-Json

# Regions are defined once in the definition's regionDefaults so the provisioned topology
# matches sdk/cosmos/test-resources.bicep (the ARM template the old per-run flow used). The
# optional -Location / -SecondaryLocation params override the primary/secondary region for
# ad-hoc runs.
if (-not ($definition.PSObject.Properties.Name -contains 'regionDefaults')) {
    throw "Definition '$DefinitionPath' is missing 'regionDefaults' (singleRegion / multiRegion)."
}
$singleRegionList = @($definition.regionDefaults.singleRegion)
$multiRegionList = @($definition.regionDefaults.multiRegion)
if ($singleRegionList.Count -lt 1 -or $multiRegionList.Count -lt 2) {
    throw "regionDefaults must provide singleRegion (>=1) and multiRegion (>=2) entries."
}
if ($Location) {
    $singleRegionList[0] = $Location
    $multiRegionList[0] = $Location
}
if ($SecondaryLocation) {
    $multiRegionList[1] = $SecondaryLocation
}
$primaryRegion = $multiRegionList[0]

$databaseName = if ($definition.PSObject.Properties.Name -contains 'database' -and $definition.database) {
    [string]$definition.database
}
else {
    'shared-test-db'
}

Write-Info "Selecting subscription $SubscriptionId"
$null = Set-AzContext -Subscription $SubscriptionId

# --- Resource group (create if missing) --------------------------------------
if (-not (Get-AzResourceGroup -Name $ResourceGroupName -ErrorAction SilentlyContinue)) {
    if ($PSCmdlet.ShouldProcess($ResourceGroupName, 'Create resource group')) {
        Write-Info "Creating resource group $ResourceGroupName in $primaryRegion"
        $null = New-AzResourceGroup -Name $ResourceGroupName -Location $primaryRegion
    }
}
else {
    Write-Info "Resource group $ResourceGroupName already exists"
}

# --- Helper: build the -LocationObject list from a region list --------------
# First region is the write region (failoverPriority 0); the rest are read regions.
function New-LocationObjects([string[]] $regionList) {
    $locations = @()
    for ($i = 0; $i -lt $regionList.Count; $i++) {
        $locations += New-AzCosmosDBLocationObject -LocationName $regionList[$i] -FailoverPriority $i -IsZoneRedundant $false
    }
    return , $locations
}

# --- Create / update each account, then collect endpoint + keys --------------
$secret = [ordered]@{
    version  = 1
    accounts = [ordered]@{}
}

foreach ($acct in $definition.accounts) {
    $selector = $acct.name
    $accountName = ("{0}-{1}" -f $AccountNamePrefix, $selector).ToLower()
    if ($accountName.Length -gt 44) {
        throw "Generated account name '$accountName' exceeds 44 chars. Shorten AccountNamePrefix or the selector '$selector'."
    }

    $multiRegion = [bool]$acct.enableMultipleRegions
    $multiWrite = [bool]$acct.enableMultipleWriteLocations
    $autoFailover = [bool]$acct.enableAutomaticFailover
    $regionList = if ($multiRegion) { $multiRegionList } else { $singleRegionList }
    $locations = New-LocationObjects $regionList

    $existing = Get-AzCosmosDBAccount -ResourceGroupName $ResourceGroupName -Name $accountName -ErrorAction SilentlyContinue
    if (-not $existing) {
        if ($PSCmdlet.ShouldProcess($accountName, "Create Cosmos account [$selector]")) {
            Write-Info "Creating Cosmos account '$accountName' (selector=$selector, consistency=$($acct.defaultConsistencyLevel), multiWrite=$multiWrite, multiRegion=$multiRegion, autoFailover=$autoFailover)"
            $params = @{
                ResourceGroupName            = $ResourceGroupName
                Name                         = $accountName
                LocationObject               = $locations
                DefaultConsistencyLevel      = $acct.defaultConsistencyLevel
                EnableAutomaticFailover      = $autoFailover
                EnableMultipleWriteLocations = $multiWrite
                ApiKind                      = 'GlobalDocumentDB'
                Capabilities                 = @('EnableNoSQLVectorSearch', 'EnableNoSQLFullTextSearch')
            }
            if ($acct.PSObject.Properties.Name -contains 'enableContinuousBackup' -and $acct.enableContinuousBackup) {
                $params['BackupPolicyType'] = 'Continuous'
                $params['ContinuousTier'] = 'Continuous7Days'
            }
            $null = New-AzCosmosDBAccount @params
        }
    }
    else {
        Write-Info "Cosmos account '$accountName' already exists (selector=$selector); leaving configuration as-is"
    }

    # Ensure the shared test database exists (mirrors the `database` resource that
    # sdk/cosmos/test-resources.bicep used to create per-run).
    if (-not $WhatIfPreference) {
        $existingDb = Get-AzCosmosDBSqlDatabase -ResourceGroupName $ResourceGroupName -AccountName $accountName -Name $databaseName -ErrorAction SilentlyContinue
        if (-not $existingDb) {
            if ($PSCmdlet.ShouldProcess("$accountName/$databaseName", 'Create shared test database')) {
                Write-Info "Creating database '$databaseName' on account '$accountName'"
                $null = New-AzCosmosDBSqlDatabase -ResourceGroupName $ResourceGroupName -AccountName $accountName -Name $databaseName
            }
        }
    }

    # Read endpoint + keys. Under -WhatIf (dry run) never read real keys - stub them so a
    # preview never emits secrets, even for already-provisioned accounts.
    if ($WhatIfPreference) {
        $endpoint = "https://$accountName.documents.azure.com:443/"
        $primary = 'WHATIF_KEY'
        $secondary = 'WHATIF_SECONDARY_KEY'
    }
    else {
        $account = Get-AzCosmosDBAccount -ResourceGroupName $ResourceGroupName -Name $accountName
        $endpoint = $account.DocumentEndpoint
        $keys = Get-AzCosmosDBAccountKey -ResourceGroupName $ResourceGroupName -Name $accountName -Type 'Keys'
        $primary = $keys.PrimaryMasterKey
        $secondary = $keys.SecondaryMasterKey
    }

    $entry = [ordered]@{
        endpoint     = $endpoint
        key          = $primary
        database     = $databaseName
        consistency  = $acct.defaultConsistencyLevel
        testCategory = $acct.testCategory
        multiWrite   = $multiWrite
        multiRegion  = $multiRegion
    }
    if ($acct.PSObject.Properties.Name -contains 'includeSecondaryKey' -and $acct.includeSecondaryKey) {
        $entry['secondaryKey'] = $secondary
    }
    $entry['regions'] = [string[]]$regionList

    $secret.accounts[$selector] = $entry
}

# --- Emit the assembled JSON -------------------------------------------------
$secretJson = $secret | ConvertTo-Json -Depth 8

if ($OutputPath) {
    if ($PSCmdlet.ShouldProcess($OutputPath, 'Write accounts JSON to file')) {
        Set-Content -Path $OutputPath -Value $secretJson -NoNewline
        Write-Info "Wrote accounts JSON to '$OutputPath' ($($secret.accounts.Count) accounts). Contains keys - treat as secret."
    }
}

Write-Info "Assembled $($secret.accounts.Count) accounts. Update the fixed-accounts ADO secret manually with this JSON (see sdk/cosmos/pipeline/README.md)."
# Emit the JSON to stdout so it can be captured/redirected.
Write-Output $secretJson
