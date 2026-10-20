# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

. "$PSScriptRoot\..\eng\scripts\Invoke-CosmosTestSetup.ps1"
. "$PSScriptRoot\..\eng\scripts\Invoke-CosmosDefaultFeatureCheck.ps1"

if ($env:AZURE_COSMOS_NATIVE_TESTS -eq 'true') {
    & "$PSScriptRoot\..\eng\scripts\Invoke-CosmosNativeTests.ps1"
}
