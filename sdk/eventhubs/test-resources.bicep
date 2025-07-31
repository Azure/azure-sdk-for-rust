@description('Base name of the resource to be created.')
@minLength(16)
param baseName string = resourceGroup().name

@description('The location where the resources will be created.')
param location string = resourceGroup().location

@description('The suffix for the storage endpoint, typically based on the Azure environment.')
param storageEndpointSuffix string = environment().suffixes.storage

@description('Indicates if the tenant is a TME tenant. If true, local (SAS) authentication is enabled.')
param tenantIsTME bool = false

@description('The client OID to grant access to test resources.')
param testApplicationOid string

var eventhubNamespaceName = 'eh-${baseName}'
var storageAccountName = 'blb${baseName}'
var eventHubName = 'testeventhub'

var eventHubsDataOwnerRoleId = 'f526a384-b230-433a-b45c-95f59c4a2dec'
var blobDataOwnerRoleId = 'b7e6dc6d-f1e8-4753-8033-0f276bb0955b'
var azureContributorRoleId = 'b24988ac-6180-42a0-ab88-20f7382dd24c'

resource eventhubNamespace 'Microsoft.EventHub/namespaces@2024-05-01-preview' = {
  name: eventhubNamespaceName
  location: location
  sku: {
    name: 'Standard'
    tier: 'Standard'
    capacity: 5
  }
  properties: {
    geoDataReplication: {
      maxReplicationLagDurationInSeconds: 0
      locations: [
        {
          locationName: location
          roleType: 'Primary'
        }
      ]
    }
    minimumTlsVersion: '1.2'
    publicNetworkAccess: 'Enabled'
    disableLocalAuth: !tenantIsTME // Disable local auth for non-TME tenants
    zoneRedundant: false
    isAutoInflateEnabled: false
    //    maximumThroughputUnits: 0
    //    kafkaEnabled: true
  }
  resource authorization 'authorizationrules@2024-05-01-preview' = {
    name: 'RootManageSharedAccessKey'
    properties: {
      rights: [
        'Listen'
        'Manage'
        'Send'
      ]
    }
  }
  resource authorizedListenOnly 'AuthorizationRules@2017-04-01' = {
    name: 'ListenOnly'
    properties: {
      rights: [
        'Listen'
      ]
    }
  }

  resource authorizedSendOnly 'AuthorizationRules@2017-04-01' = {
    name: 'SendOnly'
    properties: {
      rights: [
        'Send'
      ]
    }
  }
  resource eventHub 'eventhubs@2024-05-01-preview' = {
    name: eventHubName
    properties: {
      messageTimestampDescription: {
        timestampType: 'LogAppend'
      }
      retentionDescription: {
        cleanupPolicy: 'Delete'
        retentionTimeInHours: 24
      }
      messageRetentionInDays: 1
      partitionCount: 4
      status: 'Active'
    }
    resource consumerGroup 'consumergroups@2024-05-01-preview' = {
      name: '$Default'
      properties: {}
    }

    resource defaultGroup 'consumergroups@2024-05-01-preview' = {
      name: 'defaultGroup'
      properties: {}
    }
  }

  resource eventhubNamespace_networkruleset_default 'networkrulesets@2024-05-01-preview' = {
    name: 'default'
    properties: {
      publicNetworkAccess: 'Enabled'
      defaultAction: 'Allow'
      virtualNetworkRules: []
      ipRules: []
      trustedServiceAccessEnabled: false
    }
  }
}

resource roleAssignments_ehDataOwner 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(eventhubNamespace.id, 'Azure Event Hubs Data Owner')

  properties: {
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', eventHubsDataOwnerRoleId) // Azure Event Hubs Data Owner
    principalId: testApplicationOid
  }
  dependsOn: [
    eventhubNamespace::eventHub
    storageAccount
  ]
}

resource roleAssignments_contributor 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(eventhubNamespace.id, 'Azure Contributor')

  properties: {
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', azureContributorRoleId) // Azure Contributor
    principalId: testApplicationOid
  }
  dependsOn: [
    eventhubNamespace::eventHub
    storageAccount
  ]
}

resource roleAssignments_storageDataOwner 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(eventhubNamespace.id, 'Storage Blob Data Owner')

  properties: {
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', blobDataOwnerRoleId) // Storage Blob Data Owner
    principalId: testApplicationOid
  }
  dependsOn: [
    eventhubNamespace::eventHub
    storageAccount
  ]
}

resource storageAccount 'Microsoft.Storage/storageAccounts@2024-01-01' = {
  name: storageAccountName
  location: location
  sku: {
    name: 'Standard_LRS'
  }
  kind: 'BlobStorage'
  properties: {
    allowCrossTenantReplication: false
    minimumTlsVersion: 'TLS1_2'
    allowBlobPublicAccess: false
    allowSharedKeyAccess: false
    networkAcls: {
      bypass: 'AzureServices'
      virtualNetworkRules: []
      ipRules: []
      defaultAction: 'Allow'
    }
    supportsHttpsTrafficOnly: true
    encryption: {
      services: {
        file: {
          keyType: 'Account'
          enabled: true
        }
        blob: {
          keyType: 'Account'
          enabled: true
        }
      }
      keySource: 'Microsoft.Storage'
    }
    accessTier: 'Hot'
  }
  resource storageAccount_default 'blobServices@2024-01-01' = {
    name: 'default'
    properties: {
      cors: {
        corsRules: []
      }
      deleteRetentionPolicy: {
        allowPermanentDelete: false
        enabled: false
      }
    }
    resource storageAccount_blobContainer 'containers@2019-04-01' = {
      name: 'container'
      properties: {}
    }
  }
}

// Outputs
output EVENTHUB_NAME string = eventhubNamespace::eventHub.name
output EVENTHUBS_NAMESPACE string = eventhubNamespace.name

output EVENTHUBS_HOST string = replace(
  replace(eventhubNamespace.properties.serviceBusEndpoint, ':443/', ''),
  'https://',
  ''
)
//output EVENTHUBS_CONNECTION_STRING string = listKeys(eventHubAuthRule.id, eventHubAuthRule.apiVersion).primaryConnectionString
output AZURE_STORAGE_ACCOUNT_NAME string = storageAccount.name
//output AZURE_STORAGE_ACCOUNT_KEY string = listKeys(storage.id, storage.apiVersion).keys[0].value
output STORAGE_ENDPOINT_SUFFIX string = storageEndpointSuffix
output AZURE_STORAGE_BLOB_ENDPOINT string = storageAccount.properties.primaryEndpoints.blob
output RESOURCE_GROUP string = resourceGroup().name
output AZURE_STORAGE_BLOB_CONTAINER string = storageAccount::storageAccount_default::storageAccount_blobContainer.name
