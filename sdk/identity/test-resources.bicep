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

resource appServicePlan 'Microsoft.Web/serverfarms@2023-12-01' = if (deployResources) {
  name: 'asp-${baseName}'
  location: location
  sku: {
    name: 'P1V2'
    tier: 'PremiumV2'
  }
  kind: 'linux'
  properties: {
    reserved: true
  }
}

resource functionApp 'Microsoft.Web/sites@2023-12-01' = if (deployResources) {
  name: 'func-${baseName}'
  location: location
  kind: 'functionapp,linux'
  identity: {
    type: 'UserAssigned'
    userAssignedIdentities: {
      '${usermgdid.id}': {}
    }
  }
  properties: {
    serverFarmId: appServicePlan.id
    reserved: true
    siteConfig: {
      linuxFxVersion: 'CUSTOM'
      appSettings: [
        {
          name: 'AzureWebJobsStorage__blobServiceUri'
          value: 'https://${saUserAssigned.name}.blob.${environment().suffixes.storage}'
        }
        {
          name: 'AzureWebJobsStorage__queueServiceUri'
          value: 'https://${saUserAssigned.name}.queue.${environment().suffixes.storage}'
        }
        {
          name: 'AzureWebJobsStorage__tableServiceUri'
          value: 'https://${saUserAssigned.name}.table.${environment().suffixes.storage}'
        }
        {
          name: 'AzureWebJobsStorage__credential'
          value: 'managedidentity'
        }
        {
          name: 'AzureWebJobsStorage__clientId'
          value: usermgdid.properties.clientId
        }
        {
          name: 'FUNCTIONS_WORKER_RUNTIME'
          value: 'custom'
        }
        {
          name: 'FUNCTIONS_EXTENSION_VERSION'
          value: '~4'
        }
      ]
    }
  }
}

resource blobRoleAssignment 'Microsoft.Authorization/roleAssignments@2022-04-01' = if (deployResources) {
  name: guid(saUserAssigned.id, usermgdid.id, blobDataOwnerRoleId)
  scope: saUserAssigned
  properties: {
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', blobDataOwnerRoleId)
    principalId: usermgdid.properties.principalId
    principalType: 'ServicePrincipal'
  }
}

resource queueRoleAssignment 'Microsoft.Authorization/roleAssignments@2022-04-01' = if (deployResources) {
  name: guid(saUserAssigned.id, usermgdid.id, queueDataContributorRoleId)
  scope: saUserAssigned
  properties: {
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', queueDataContributorRoleId)
    principalId: usermgdid.properties.principalId
    principalType: 'ServicePrincipal'
  }
}

resource tableRoleAssignment 'Microsoft.Authorization/roleAssignments@2022-04-01' = if (deployResources) {
  name: guid(saUserAssigned.id, usermgdid.id, tableDataContributorRoleId)
  scope: saUserAssigned
  properties: {
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', tableDataContributorRoleId)
    principalId: usermgdid.properties.principalId
    principalType: 'ServicePrincipal'
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
