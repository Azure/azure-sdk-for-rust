// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

param baseName string = resourceGroup().name
param testApplicationOid string
param location string = resourceGroup().location

var blobDataContributorRoleId = 'ba92f5b4-2d11-453d-a403-e96b0029c9fe'
var blobDataOwnerRoleId = 'b7e6dc6d-f1e8-4753-8033-0f276bb0955b'
var networkAcls = {
  bypass: 'AzureServices'
  defaultAction: 'Allow'
  ipRules: []
  virtualNetworkRules: []
}

resource blobDataContributor 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(blobDataContributorRoleId, resourceGroup().id)
  properties: {
    roleDefinitionId: resourceId('Microsoft.Authorization/roleDefinitions', blobDataContributorRoleId)
    principalId: testApplicationOid
  }
}

resource blobDataOwner 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(blobDataOwnerRoleId, resourceGroup().id)
  properties: {
    roleDefinitionId: resourceId('Microsoft.Authorization/roleDefinitions', blobDataOwnerRoleId)
    principalId: testApplicationOid
  }
}

resource storage 'Microsoft.Storage/storageAccounts@2024-01-01' = {
  name: '${baseName}blob'
  location: location
  kind: 'BlockBlobStorage'
  sku: {
    name: 'Premium_LRS'
  }
  properties: {
    allowSharedKeyAccess: false
    networkAcls: networkAcls
    supportsHttpsTrafficOnly: true
  }
}

output AZURE_STORAGE_ACCOUNT_NAME string = storage.name
