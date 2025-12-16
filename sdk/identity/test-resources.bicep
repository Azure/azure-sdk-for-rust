// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

@minLength(6)
@maxLength(23)
@description('The base resource name.')
param baseName string = resourceGroup().name

@description('Whether to deploy resources. When set to false, this file deploys nothing.')
param deployResources bool = false

@description('The location of the resource. By default, this is the same as the resource group.')
param location string = resourceGroup().location

// https://learn.microsoft.com/azure/role-based-access-control/built-in-roles
var storageAccountContributor = subscriptionResourceId(
  'Microsoft.Authorization/roleDefinitions',
  '17d1049b-9a84-46fb-8f53-869881c3d3ab'
)

var blobDataOwnerRoleId = 'b7e6dc6d-f1e8-4753-8033-0f276bb0955b'
var queueDataContributorRoleId = '974c5e8b-45b9-4653-ba55-5f855dd0fb88'
var tableDataContributorRoleId = '0a9a7e1f-b9d0-4cc4-a60d-0319b160aaa3'

resource saSystemAssigned 'Microsoft.Storage/storageAccounts@2021-08-01' = if (deployResources) {
  kind: 'StorageV2'
  location: location
  name: 'sa${uniqueString(baseName)}'
  properties: {
    accessTier: 'Hot'
    allowSharedKeyAccess: false
  }
  sku: {
    name: 'Standard_LRS'
  }
}

resource saUserAssigned 'Microsoft.Storage/storageAccounts@2021-08-01' = if (deployResources) {
  kind: 'StorageV2'
  location: location
  name: 'sa2${uniqueString(baseName)}'
  properties: {
    accessTier: 'Hot'
    allowSharedKeyAccess: false
  }
  sku: {
    name: 'Standard_LRS'
  }
}

resource usermgdid 'Microsoft.ManagedIdentity/userAssignedIdentities@2018-11-30' = if (deployResources) {
  location: location
  name: baseName
}

resource storageRoleUserAssigned 'Microsoft.Authorization/roleAssignments@2022-04-01' = if (deployResources) {
  scope: saUserAssigned
  name: guid(resourceGroup().id, storageAccountContributor, usermgdid.id)
  properties: {
    principalId: deployResources ? usermgdid.properties.principalId : ''
    principalType: 'ServicePrincipal'
    roleDefinitionId: storageAccountContributor
  }
}


resource farm 'Microsoft.Web/serverfarms@2021-03-01' = if (deployResources) {
  kind: 'app'
  location: location
  name: '${baseName}_asp'
  properties: {}
  sku: {
    capacity: 1
    family: 'B'
    name: 'B1'
    size: 'B1'
    tier: 'Basic'
  }
}

resource functionApp 'Microsoft.Web/sites@2021-03-01' = if (deployResources) {
  identity: {
    type: 'SystemAssigned, UserAssigned'
    userAssignedIdentities: {
      '${deployResources ? usermgdid.id : ''}': {}
    }
  }
  kind: 'functionapp'
  location: location
  name: '${baseName}func'
  properties: {
    enabled: true
    httpsOnly: true
    keyVaultReferenceIdentity: 'SystemAssigned'
    serverFarmId: farm.id
    siteConfig: {
      alwaysOn: true
      appSettings: [
        {
          name: 'AZIDENTITY_STORAGE_NAME'
          value: deployResources ? saSystemAssigned.name : null
        }
        {
          name: 'AZIDENTITY_STORAGE_NAME_USER_ASSIGNED'
          value: deployResources ? saUserAssigned.name : null
        }
        {
          name: 'AZIDENTITY_USER_ASSIGNED_IDENTITY'
          value: deployResources ? usermgdid.id : null
        }
        {
          name: 'AZIDENTITY_USER_ASSIGNED_IDENTITY_CLIENT_ID'
          value: deployResources ? usermgdid.properties.clientId : null
        }
        {
          name: 'AZIDENTITY_USER_ASSIGNED_IDENTITY_OBJECT_ID'
          value: deployResources ? usermgdid.properties.principalId : null
        }
        {
          name: 'AzureWebJobsStorage'
          value: 'DefaultEndpointsProtocol=https;AccountName=${deployResources ? saSystemAssigned.name : ''};EndpointSuffix=${deployResources ? environment().suffixes.storage : ''};AccountKey=${deployResources ? saSystemAssigned.listKeys().keys[0].value : ''}'
        }
        {
          name: 'FUNCTIONS_EXTENSION_VERSION'
          value: '~4'
        }
        {
          name: 'FUNCTIONS_WORKER_RUNTIME'
          value: 'custom'
        }
        {
          name: 'WEBSITE_CONTENTAZUREFILECONNECTIONSTRING'
          value: 'DefaultEndpointsProtocol=https;AccountName=${deployResources ? saSystemAssigned.name : ''};EndpointSuffix=${deployResources ? environment().suffixes.storage : ''};AccountKey=${deployResources ? saSystemAssigned.listKeys().keys[0].value : ''}'
        }
        {
          name: 'WEBSITE_CONTENTSHARE'
          value: toLower('${baseName}-func')
        }
      ]
      http20Enabled: true
      minTlsVersion: '1.2'
    }
  }
}

output IDENTITY_STORAGE_ID string = deployResources ? saSystemAssigned.id : ''
output IDENTITY_STORAGE_NAME_SYSTEM_ASSIGNED string = deployResources ? saSystemAssigned.name : ''
output IDENTITY_STORAGE_NAME_USER_ASSIGNED string = deployResources ? saUserAssigned.name : ''
output IDENTITY_USER_ASSIGNED_IDENTITY string = deployResources ? usermgdid.id : ''
output IDENTITY_USER_ASSIGNED_IDENTITY_CLIENT_ID string = deployResources ? usermgdid.properties.clientId : ''
output IDENTITY_USER_ASSIGNED_IDENTITY_NAME string = deployResources ? usermgdid.name : ''
output IDENTITY_USER_ASSIGNED_IDENTITY_OBJECT_ID string = deployResources ? usermgdid.properties.principalId : ''
output IDENTITY_FUNCTIONAPP_NAME string = deployResources ? functionApp.name : ''
output IDENTITY_FUNCTIONAPP_DEFAULT_HOSTNAME string = deployResources ? functionApp.properties.defaultHostName : ''
