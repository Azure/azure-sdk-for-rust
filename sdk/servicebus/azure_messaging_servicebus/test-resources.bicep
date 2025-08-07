@description('Base name of the resource to be created.')
@minLength(16)
param baseName string = resourceGroup().name

@description('The location where the resources will be created.')
param location string = resourceGroup().location

@description('Indicates if the tenant is a TME tenant. If true, local (SAS) authentication is enabled.')
param tenantIsTME bool = false

@description('The client OID to grant access to test resources.')
param testApplicationOid string

var serviceBusNamespaceName = 'sb-${baseName}'
var queueName = 'testqueue'
var topicName = 'testtopic'
var subscriptionName = 'testsubscription'

var serviceBusDataOwnerRoleId = '090c5cfd-751d-490a-894a-3ce6f1109419'
var serviceBusDataReceiverRoleId = '4f6d3b9b-027b-4f4c-9142-0e5a2a2247e0'
var serviceBusDataSenderRoleId = '69a216fc-b8fb-44d8-bc22-1f3c2cd27a39'
var azureContributorRoleId = 'b24988ac-6180-42a0-ab88-20f7382dd24c'

resource serviceBusNamespace 'Microsoft.ServiceBus/namespaces@2024-01-01' = {
  name: serviceBusNamespaceName
  location: location
  sku: {
    name: 'Standard'
    tier: 'Standard'
  }
  properties: {
    minimumTlsVersion: '1.2'
    publicNetworkAccess: 'Enabled'
    disableLocalAuth: !tenantIsTME // Disable local auth for non-TME tenants
    zoneRedundant: false
  }

  resource rootSharedAccessKey 'AuthorizationRules@2022-10-01-preview' = {
    name: 'RootManageSharedAccessKey'
    properties: {
      rights: [
        'Listen'
        'Manage'
        'Send'
      ]
    }
  }

  resource listenOnlyKey 'AuthorizationRules@2022-10-01-preview' = {
    name: 'ListenOnly'
    properties: {
      rights: [
        'Listen'
      ]
    }
  }

  resource sendOnlyKey 'AuthorizationRules@2022-10-01-preview' = {
    name: 'SendOnly'
    properties: {
      rights: [
        'Send'
      ]
    }
  }

  resource queue 'queues@2022-10-01-preview' = {
    name: queueName
    properties: {
      lockDuration: 'PT30S'
      maxSizeInMegabytes: 1024
      requiresDuplicateDetection: false
      requiresSession: false
      defaultMessageTimeToLive: 'P14D'
      deadLetteringOnMessageExpiration: false
      duplicateDetectionHistoryTimeWindow: 'PT10M'
      maxDeliveryCount: 10
      autoDeleteOnIdle: 'P10675199DT2H48M5.4775807S'
      enablePartitioning: false
      enableExpress: false
      status: 'Active'
    }
  }

  resource topic 'topics@2022-10-01-preview' = {
    name: topicName
    properties: {
      maxSizeInMegabytes: 1024
      requiresDuplicateDetection: false
      defaultMessageTimeToLive: 'P14D'
      duplicateDetectionHistoryTimeWindow: 'PT10M'
      enableBatchedOperations: true
      enablePartitioning: false
      enableExpress: false
      status: 'Active'
      autoDeleteOnIdle: 'P10675199DT2H48M5.4775807S'
      supportOrdering: true
    }

    resource subscription 'subscriptions@2022-10-01-preview' = {
      name: subscriptionName
      properties: {
        lockDuration: 'PT30S'
        requiresSession: false
        defaultMessageTimeToLive: 'P14D'
        deadLetteringOnMessageExpiration: false
        deadLetteringOnFilterEvaluationExceptions: true
        maxDeliveryCount: 10
        enableBatchedOperations: true
        autoDeleteOnIdle: 'P10675199DT2H48M5.4775807S'
        status: 'Active'
      }
    }
  }

  resource networkRuleSet 'networkrulesets@2022-10-01-preview' = {
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

// Role assignment for Service Bus Data Owner
resource roleAssignments_sbDataOwner 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(serviceBusNamespace.id, 'Azure Service Bus Data Owner')
  properties: {
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', serviceBusDataOwnerRoleId) // Azure Service Bus Data Owner
    principalId: testApplicationOid
  }
  dependsOn: [
    serviceBusNamespace::queue
    serviceBusNamespace::topic
  ]
}

// Role assignment for Contributor (for management operations)
resource roleAssignments_contributor 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(serviceBusNamespace.id, 'Azure Contributor')
  properties: {
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', azureContributorRoleId) // Azure Contributor
    principalId: testApplicationOid
  }
  dependsOn: [
    serviceBusNamespace::queue
    serviceBusNamespace::topic
  ]
}

// Role assignment for Service Bus Data Receiver
resource roleAssignments_sbDataReceiver 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(serviceBusNamespace.id, 'Azure Service Bus Data Receiver')
  properties: {
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', serviceBusDataReceiverRoleId) // Azure Service Bus Data Receiver
    principalId: testApplicationOid
  }
  dependsOn: [
    serviceBusNamespace::queue
    serviceBusNamespace::topic
  ]
}

// Role assignment for Service Bus Data Sender
resource roleAssignments_sbDataSender 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(serviceBusNamespace.id, 'Azure Service Bus Data Sender')
  properties: {
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', serviceBusDataSenderRoleId) // Azure Service Bus Data Sender
    principalId: testApplicationOid
  }
  dependsOn: [
    serviceBusNamespace::queue
    serviceBusNamespace::topic
  ]
}

// Outputs for test environment variables
output SERVICEBUS_NAMESPACE string = replace(
  replace(serviceBusNamespace.properties.serviceBusEndpoint, ':443/', ''),
  'https://',
  ''
)

output SERVICEBUS_NAMESPACE_NAME string = serviceBusNamespace.name

output SERVICEBUS_QUEUE_NAME string = serviceBusNamespace::queue.name

output SERVICEBUS_TOPIC_NAME string = serviceBusNamespace::topic.name

output SERVICEBUS_SUBSCRIPTION_NAME string = serviceBusNamespace::topic::subscription.name

// Connection strings contain secrets and should be retrieved via Azure CLI or portal
// output SERVICEBUS_CONNECTION_STRING string = serviceBusNamespace::rootSharedAccessKey.listKeys().primaryConnectionString
// output SERVICEBUS_LISTEN_ONLY_CONNECTION_STRING string = serviceBusNamespace::listenOnlyKey.listKeys().primaryConnectionString
// output SERVICEBUS_SEND_ONLY_CONNECTION_STRING string = serviceBusNamespace::sendOnlyKey.listKeys().primaryConnectionString

output RESOURCE_GROUP string = resourceGroup().name
