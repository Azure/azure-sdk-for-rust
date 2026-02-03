// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

param baseName string = resourceGroup().name
param tenantId string = '72f988bf-86f1-41af-91ab-2d7cd011db47'
param testApplicationOid string
param location string = resourceGroup().location
@allowed(['standard', 'premium'])
param keyVaultSku string = 'premium'

var kvAdminDefinitionId = '00482a5a-887f-4fb3-b363-3b7fe8e74483'
var kvAdminAssignmentName = guid(resourceGroup().id, kvAdminDefinitionId, testApplicationOid)

resource kv 'Microsoft.KeyVault/vaults@2023-07-01' = {
  name: baseName
  location: location
  properties: {
    sku: {
      family: 'A'
      name: keyVaultSku
    }
    tenantId: tenantId
    enableRbacAuthorization: true
    softDeleteRetentionInDays: 7
  }
}

resource kvAdmin 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: kvAdminAssignmentName
  properties: {
    roleDefinitionId: resourceId('Microsoft.Authorization/roleDefinitions', kvAdminDefinitionId)
    principalId: testApplicationOid
  }
}

output AZURE_KEYVAULT_URL string = kv.properties.vaultUri
output KEYVAULT_TENANT_ID string = subscription().tenantId
