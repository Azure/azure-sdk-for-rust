// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

param baseName string = resourceGroup().name
param testApplicationOid string
param location string = resourceGroup().location

var blobDataOwnerRoleId = 'b7e6dc6d-f1e8-4753-8033-0f276bb0955b'
var queueDataContributorRoleId = '974c5e8b-45b9-4653-ba55-5f855dd0fb88'
var encryption = {
  keySource: 'Microsoft.Storage'
  services: {
    blob: {
      enabled: true
    }
    file: {
      enabled: true
    }
  }
}
var networkAcls = {
  bypass: 'AzureServices'
  defaultAction: 'Allow'
  ipRules: []
  virtualNetworkRules: []
}

resource blobDataOwner 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(blobDataOwnerRoleId, resourceGroup().id)
  properties: {
    roleDefinitionId: resourceId('Microsoft.Authorization/roleDefinitions', blobDataOwnerRoleId)
    principalId: testApplicationOid
  }
}

resource queueDataContributor 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(queueDataContributorRoleId, resourceGroup().id)
  properties: {
    roleDefinitionId: resourceId('Microsoft.Authorization/roleDefinitions', queueDataContributorRoleId)
    principalId: testApplicationOid
  }
}

resource storage 'Microsoft.Storage/storageAccounts@2024-01-01' = {
  name: '${baseName}prim'
  location: location
  kind: 'StorageV2'
  sku: {
    name: 'Standard_RAGRS'
  }
  properties: {
    accessTier: 'Hot'
    allowSharedKeyAccess: false
    encryption: encryption
    networkAcls: networkAcls
    supportsHttpsTrafficOnly: true
  }
}

resource blobServices 'Microsoft.Storage/storageAccounts/blobServices@2024-01-01' = {
  parent: storage
  name: 'default'
  properties: {
    deleteRetentionPolicy: {
      enabled: true
      days: 1 // Number of days to retain deleted blobs (1-365)
      allowPermanentDelete: false
    }
    containerDeleteRetentionPolicy: {
      enabled: true
      days: 1 // Number of days to retain deleted containers (1-365)
    }
  }
}

output AZURE_STORAGE_ACCOUNT_NAME string = storage.name
