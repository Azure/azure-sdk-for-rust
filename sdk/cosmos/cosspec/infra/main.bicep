// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Deploys one Azure Cosmos DB account per cosspec service profile, for the
// `live` backend. Each account has one region per region alias of its profile
// (ServiceProfile::regions()), with the profile's write regions:
//
//   SingleRegion: PrimaryReadWrite = primaryLocation
//   MultiRead:    PrimaryReadWrite = primaryLocation (writes),
//                 SecondaryReadOnly = secondaryLocation (reads)
//   MultiWrite:   PrimaryReadWrite = primaryLocation,
//                 SecondaryReadWrite = secondaryLocation (both write)
//
// Pick any locations with capacity. The `regions` output is each profile's
// `regions` table for the cosspec config. Keys aren't output: read them with
// `az cosmosdb keys list` and put them in the environment variables your
// cosspec config names in `key_env`.

@description('A short prefix for the account names. Account names are `<prefix>-<profile>`.')
@minLength(3)
@maxLength(30)
param namePrefix string = 'cosspec-${uniqueString(resourceGroup().id)}'

@description('The PrimaryReadWrite region of every account.')
param primaryLocation string = 'eastus'

@description('The SecondaryReadOnly or SecondaryReadWrite region of the multi-region accounts.')
param secondaryLocation string = 'westus'

@description('Optional Microsoft Entra principal (object ID) to grant data-plane access, for `auth = { kind = "entra" }`. Leave empty to skip.')
param principalId string = ''

var profiles = [
  {
    name: 'SingleRegion'
    suffix: 'single-region'
    locations: [ primaryLocation ]
    multiWrite: false
  }
  {
    name: 'MultiRead'
    suffix: 'multi-read'
    locations: [ primaryLocation, secondaryLocation ]
    multiWrite: false
  }
  {
    name: 'MultiWrite'
    suffix: 'multi-write'
    locations: [ primaryLocation, secondaryLocation ]
    multiWrite: true
  }
]

// Cosmos DB Built-in Data Contributor.
var dataContributorRoleId = '00000000-0000-0000-0000-000000000002'

resource accounts 'Microsoft.DocumentDB/databaseAccounts@2024-11-15' = [for profile in profiles: {
  name: toLower('${namePrefix}-${profile.suffix}')
  location: primaryLocation
  kind: 'GlobalDocumentDB'
  tags: {
    cosspecProfile: profile.name
  }
  properties: {
    databaseAccountOfferType: 'Standard'
    consistencyPolicy: {
      defaultConsistencyLevel: 'Session'
    }
    locations: [for (location, i) in profile.locations: {
      locationName: location
      failoverPriority: i
      isZoneRedundant: false
    }]
    enableMultipleWriteLocations: profile.multiWrite
    enableAutomaticFailover: false
    // The spec harness provisions databases and containers with account keys.
    disableLocalAuth: false
  }
}]

resource dataContributor 'Microsoft.DocumentDB/databaseAccounts/sqlRoleAssignments@2024-11-15' = [for (profile, i) in profiles: if (!empty(principalId)) {
  parent: accounts[i]
  name: guid(accounts[i].id, principalId, dataContributorRoleId)
  properties: {
    roleDefinitionId: '${accounts[i].id}/sqlRoleDefinitions/${dataContributorRoleId}'
    principalId: principalId
    scope: accounts[i].id
  }
}]

@description('The account endpoint for each service profile.')
output endpoints object = {
  SingleRegion: accounts[0].properties.documentEndpoint
  MultiRead: accounts[1].properties.documentEndpoint
  MultiWrite: accounts[2].properties.documentEndpoint
}

@description('The account name for each service profile.')
output accountNames object = {
  SingleRegion: accounts[0].name
  MultiRead: accounts[1].name
  MultiWrite: accounts[2].name
}

@description('The region alias map for each service profile: the `regions` table of its cosspec config.')
output regions object = {
  SingleRegion: { PrimaryReadWrite: primaryLocation }
  MultiRead: { PrimaryReadWrite: primaryLocation, SecondaryReadOnly: secondaryLocation }
  MultiWrite: { PrimaryReadWrite: primaryLocation, SecondaryReadWrite: secondaryLocation }
}
