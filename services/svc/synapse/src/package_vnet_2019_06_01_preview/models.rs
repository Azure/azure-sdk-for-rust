#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Managed private endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedPrivateEndpoint {
    #[doc = "Fully qualified resource Id for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Compute/virtualMachines or Microsoft.Storage/storageAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of a managed private endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedPrivateEndpointProperties>,
}
impl ManagedPrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connection state of a managed private endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedPrivateEndpointConnectionState {
    #[doc = "The approval status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The managed private endpoint description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The actions required on the managed private endpoint"]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl ManagedPrivateEndpointConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of managed private endpoints"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedPrivateEndpointListResponse {
    #[doc = "List of managed private endpoints"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedPrivateEndpoint>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedPrivateEndpointListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedPrivateEndpointListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a managed private endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedPrivateEndpointProperties {
    #[doc = "The ARM resource ID of the resource to which the managed private endpoint is created"]
    #[serde(rename = "privateLinkResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_resource_id: Option<String>,
    #[doc = "The groupId to which the managed private endpoint is created"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The managed private endpoint provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The connection state of a managed private endpoint"]
    #[serde(rename = "connectionState", default, skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<ManagedPrivateEndpointConnectionState>,
    #[doc = "Denotes whether the managed private endpoint is reserved"]
    #[serde(rename = "isReserved", default, skip_serializing_if = "Option::is_none")]
    pub is_reserved: Option<bool>,
}
impl ManagedPrivateEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
