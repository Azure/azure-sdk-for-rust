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
var acrPull = subscriptionResourceId('Microsoft.Authorization/roleDefinitions', '7f951dda-4ed3-4680-a7ca-43fe172d538d')
var storageAccountContributor = subscriptionResourceId(
  'Microsoft.Authorization/roleDefinitions',
  '17d1049b-9a84-46fb-8f53-869881c3d3ab'
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

resource acrPullContainerInstance 'Microsoft.Authorization/roleAssignments@2022-04-01' = if (deployResources) {
  name: guid(resourceGroup().id, acrPull, 'containerInstance')
  properties: {
    principalId: deployResources ? usermgdid.properties.principalId : ''
    principalType: 'ServicePrincipal'
    roleDefinitionId: acrPull
  }
  scope: containerRegistry
}

resource blobRoleUserAssigned 'Microsoft.Authorization/roleAssignments@2022-04-01' = if (deployResources) {
  scope: saUserAssigned
  name: guid(resourceGroup().id, storageAccountContributor, usermgdid.id)
  properties: {
    principalId: deployResources ? usermgdid.properties.principalId : ''
    principalType: 'ServicePrincipal'
    roleDefinitionId: storageAccountContributor
  }
}

resource containerRegistry 'Microsoft.ContainerRegistry/registries@2023-01-01-preview' = if (deployResources) {
  location: location
  name: uniqueString(resourceGroup().id)
  properties: {
    adminUserEnabled: true
  }
  sku: {
    name: 'Basic'
  }
}

output AZURE_IDENTITY_ACR_LOGIN_SERVER string = deployResources ? containerRegistry.properties.loginServer : ''
output AZURE_IDENTITY_ACR_NAME string = deployResources ? containerRegistry.name : ''
output AZURE_IDENTITY_STORAGE_ID string = deployResources ? sa.id : ''
output AZURE_IDENTITY_STORAGE_NAME string = deployResources ? sa.name : ''
output AZURE_IDENTITY_STORAGE_NAME_USER_ASSIGNED string = deployResources ? saUserAssigned.name : ''
output AZURE_IDENTITY_USER_ASSIGNED_IDENTITY string = deployResources ? usermgdid.id : ''
output AZURE_IDENTITY_USER_ASSIGNED_IDENTITY_CLIENT_ID string = deployResources ? usermgdid.properties.clientId : ''
output AZURE_IDENTITY_USER_ASSIGNED_IDENTITY_NAME string = deployResources ? usermgdid.name : ''
output AZURE_IDENTITY_USER_ASSIGNED_IDENTITY_OBJECT_ID string = deployResources ? usermgdid.properties.principalId : ''
