# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# IMPORTANT: Do not invoke this file directly. Please instead run
# eng/common/TestResources/New-TestResources.ps1 from the repository root.
#
# The AllVersionsAndDeletes (full-fidelity) change feed is gated behind a
# subscription feature flag on the Microsoft.DocumentDB resource provider. The
# flag must be registered on the subscription (a one-time, subscription-wide,
# idempotent operation) before an account can be PATCHed to enable the feature.
# We register it here, before deployment, so it has the maximum time to
# propagate before test-resources-post.ps1 enables the feature on the account.

[CmdletBinding(SupportsShouldProcess = $true, ConfirmImpact = 'Medium')]
param (
    # Captures any arguments from New-TestResources.ps1 not declared here so no
    # parameter binding errors occur.
    [Parameter(ValueFromRemainingArguments = $true)]
    $RemainingArguments
)

# Only the dedicated AVAD job needs the feature. $templateFileParameters is set
# by New-TestResources.ps1 in the caller scope and carries the merged ARM
# template parameters, including the matrix-provided testCategory.
$testCategory = $null
if ((Get-Variable -Name 'templateFileParameters' -Scope 1 -ErrorAction Ignore) -and
    $templateFileParameters -and $templateFileParameters.ContainsKey('testCategory')) {
    $testCategory = $templateFileParameters['testCategory']
}

if ($testCategory -ne 'avad') {
    Write-Host "Skipping AllVersionsAndDeletes feature registration (testCategory = '$testCategory')."
    return
}

$providerNamespace = 'Microsoft.DocumentDB'
$featureName = 'AllVersionsAndDeletesChangeFeed'

Write-Host "Registering subscription feature '$providerNamespace/$featureName' for the AVAD change feed."
$feature = Get-AzProviderFeature -ProviderNamespace $providerNamespace -FeatureName $featureName -ErrorAction SilentlyContinue
if (-not $feature -or $feature.RegistrationState -ne 'Registered') {
    Register-AzProviderFeature -ProviderNamespace $providerNamespace -FeatureName $featureName | Out-Null

    # Re-register the provider so the newly registered feature takes effect.
    Register-AzResourceProvider -ProviderNamespace $providerNamespace | Out-Null

    # Poll briefly for the feature to reach Registered. Registration is durable
    # across runs, so a still-pending state here only affects the very first run;
    # test-resources-post.ps1 fails loudly if the PATCH is later rejected.
    $deadline = (Get-Date).AddMinutes(10)
    do {
        Start-Sleep -Seconds 20
        $feature = Get-AzProviderFeature -ProviderNamespace $providerNamespace -FeatureName $featureName -ErrorAction SilentlyContinue
        $state = if ($feature) { $feature.RegistrationState } else { 'Unknown' }
        Write-Host "  AllVersionsAndDeletesChangeFeed registration state: $state"
    } while ($state -ne 'Registered' -and (Get-Date) -lt $deadline)

    if ($state -ne 'Registered') {
        Write-Warning "Feature '$featureName' is still '$state' after waiting. Registration is subscription-wide and durable; a subsequent run should succeed once it propagates."
    }
}
else {
    Write-Host "Feature '$featureName' is already registered."
}
