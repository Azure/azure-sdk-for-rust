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
var blobReader = subscriptionResourceId(
  'Microsoft.Authorization/roleDefinitions',
  '2a2b9908-6ea1-4ae2-8e65-a410df84e7d1'
)

resource sa 'Microsoft.Storage/storageAccounts@2021-08-01' = if (deployResources) {
  kind: 'StorageV2'
  location: location
  name: 'sa${uniqueString(baseName)}'
  properties: {
    accessTier: 'Hot'
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
  }
  sku: {
    name: 'Standard_LRS'
  }
}

resource usermgdid 'Microsoft.ManagedIdentity/userAssignedIdentities@2018-11-30' = if (deployResources) {
  location: location
  name: baseName
}

resource blobRoleUserAssigned 'Microsoft.Authorization/roleAssignments@2022-04-01' = if (deployResources) {
  scope: saUserAssigned
  name: guid(resourceGroup().id, blobReader, usermgdid.id)
  properties: {
    principalId: deployResources ? usermgdid.properties.principalId : ''
    principalType: 'ServicePrincipal'
    roleDefinitionId: blobReader
  }
}

resource blobRoleFunc 'Microsoft.Authorization/roleAssignments@2022-04-01' = if (deployResources) {
  name: guid(resourceGroup().id, blobReader, 'azfunc')
  properties: {
    principalId: deployResources ? azfunc.identity.principalId : ''
    roleDefinitionId: blobReader
    principalType: 'ServicePrincipal'
  }
  scope: sa
}

resource farm 'Microsoft.Web/serverfarms@2021-03-01' = if (deployResources) {
  kind: 'linux'
  location: location
  name: '${baseName}_asp'
  properties: {
    reserved: true
  }
  sku: {
    capacity: 1
    family: 'B'
    name: 'B1'
    size: 'B1'
    tier: 'Basic'
  }
}

resource azfunc 'Microsoft.Web/sites@2021-03-01' = if (deployResources) {
  identity: {
    type: 'SystemAssigned, UserAssigned'
    userAssignedIdentities: {
      '${deployResources ? usermgdid.id : ''}': {}
    }
  }
  kind: 'functionapp,linux'
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
          name: 'AzureWebJobsStorage'
          value: 'DefaultEndpointsProtocol=https;AccountName=${deployResources ? sa.name : ''};EndpointSuffix=${deployResources ? environment().suffixes.storage : ''};AccountKey=${deployResources ? sa.listKeys().keys[0].value : ''}'
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
          value: 'DefaultEndpointsProtocol=https;AccountName=${deployResources ? sa.name : ''};EndpointSuffix=${deployResources ? environment().suffixes.storage : ''};AccountKey=${deployResources ? sa.listKeys().keys[0].value : ''}'
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

output AZURE_IDENTITY_FUNCTION_NAME string = deployResources ? azfunc.name : ''
output AZURE_IDENTITY_STORAGE_ID string = deployResources ? sa.id : ''
output AZURE_IDENTITY_STORAGE_NAME string = deployResources ? sa.name : ''
output AZURE_IDENTITY_STORAGE_NAME_USER_ASSIGNED string = deployResources ? saUserAssigned.name : ''
output AZURE_IDENTITY_USER_ASSIGNED_IDENTITY string = deployResources ? usermgdid.id : ''
output AZURE_IDENTITY_USER_ASSIGNED_IDENTITY_CLIENT_ID string = deployResources ? usermgdid.properties.clientId : ''
output AZURE_IDENTITY_USER_ASSIGNED_IDENTITY_NAME string = deployResources ? usermgdid.name : ''
output AZURE_IDENTITY_USER_ASSIGNED_IDENTITY_OBJECT_ID string = deployResources ? usermgdid.properties.principalId : ''
