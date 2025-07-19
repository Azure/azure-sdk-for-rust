# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# IMPORTANT: Do not invoke this file directly. Please instead run eng/common/TestResources/New-TestResources.ps1 from the repository root.

[CmdletBinding(SupportsShouldProcess = $true, ConfirmImpact = 'Medium')]
param (
    [hashtable] $AdditionalParameters = @{},

    # Captures any arguments from eng/New-TestResources.ps1 not declared here (no parameter errors).
    [Parameter(ValueFromRemainingArguments = $true)]
    $RemainingArguments
)

if (-not (Test-Path "$PSScriptRoot/sshkey.pub")) {
    ssh-keygen -t rsa -b 4096 -f "$PSScriptRoot/sshkey" -N '' -C ''
}
$templateFileParameters['sshPubKey'] = Get-Content "$PSScriptRoot/sshkey.pub"
