#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "API key payload which is exposed in the request/response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiKey {
    #[doc = "Gets or sets the API key name."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "Gets or sets the API key value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ApiKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of the API key payload which is exposed in the response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiKeyCollection {
    #[doc = "Gets or sets the collection of API key."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<ApiKey>,
}
impl ApiKeyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Payload of the blockchain member which is exposed in the request/response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlockchainMember {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Payload of the blockchain member properties for a blockchain member."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlockchainMemberProperties>,
    #[doc = "Blockchain member Sku in payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl BlockchainMember {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of the blockchain member payload which is exposed in the request/response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlockchainMemberCollection {
    #[doc = "Gets or sets the collection of blockchain members."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BlockchainMember>,
    #[doc = "Gets or sets the URL, that the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BlockchainMemberCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BlockchainMemberCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Payload of the blockchain member nodes Sku for a blockchain member."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlockchainMemberNodesSku {
    #[doc = "Gets or sets the nodes capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl BlockchainMemberNodesSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Payload of the blockchain member properties for a blockchain member."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlockchainMemberProperties {
    #[doc = "Gets or sets the blockchain protocol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<blockchain_member_properties::Protocol>,
    #[doc = "Payload of the blockchain member nodes Sku for a blockchain member."]
    #[serde(rename = "validatorNodesSku", default, skip_serializing_if = "Option::is_none")]
    pub validator_nodes_sku: Option<BlockchainMemberNodesSku>,
    #[doc = "Gets or sets the blockchain member provision state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<blockchain_member_properties::ProvisioningState>,
    #[doc = "Gets the dns endpoint of the blockchain member."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,
    #[doc = "Gets the auth user name of the blockchain member."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Sets the basic auth password of the blockchain member."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Gets or sets the consortium for the blockchain member."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consortium: Option<String>,
    #[doc = "Gets the managed consortium management account address."]
    #[serde(rename = "consortiumManagementAccountAddress", default, skip_serializing_if = "Option::is_none")]
    pub consortium_management_account_address: Option<String>,
    #[doc = "Sets the managed consortium management account password."]
    #[serde(rename = "consortiumManagementAccountPassword", default, skip_serializing_if = "Option::is_none")]
    pub consortium_management_account_password: Option<String>,
    #[doc = "Gets the role of the member in the consortium."]
    #[serde(rename = "consortiumRole", default, skip_serializing_if = "Option::is_none")]
    pub consortium_role: Option<String>,
    #[doc = "Gets the display name of the member in the consortium."]
    #[serde(rename = "consortiumMemberDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub consortium_member_display_name: Option<String>,
    #[doc = "Gets the Ethereum root contract address of the blockchain."]
    #[serde(rename = "rootContractAddress", default, skip_serializing_if = "Option::is_none")]
    pub root_contract_address: Option<String>,
    #[doc = "Gets the public key of the blockchain member (default transaction node)."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "Gets or sets firewall rules"]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<FirewallRule>,
}
impl BlockchainMemberProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod blockchain_member_properties {
    use super::*;
    #[doc = "Gets or sets the blockchain protocol."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        NotSpecified,
        Parity,
        Quorum,
        Corda,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("Protocol", 0u32, "NotSpecified"),
                Self::Parity => serializer.serialize_unit_variant("Protocol", 1u32, "Parity"),
                Self::Quorum => serializer.serialize_unit_variant("Protocol", 2u32, "Quorum"),
                Self::Corda => serializer.serialize_unit_variant("Protocol", 3u32, "Corda"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the blockchain member provision state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotSpecified,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Stale,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Stale => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Stale"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Update the payload of the blockchain member properties for a blockchain member."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlockchainMemberPropertiesUpdate {
    #[serde(flatten)]
    pub transaction_node_properties_update: TransactionNodePropertiesUpdate,
    #[doc = "Sets the managed consortium management account password."]
    #[serde(rename = "consortiumManagementAccountPassword", default, skip_serializing_if = "Option::is_none")]
    pub consortium_management_account_password: Option<String>,
}
impl BlockchainMemberPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update the payload of the blockchain member which is exposed in the request/response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlockchainMemberUpdate {
    #[doc = "Tags of the service which is a list of key value pairs that describes the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Update the payload of the blockchain member properties for a blockchain member."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlockchainMemberPropertiesUpdate>,
}
impl BlockchainMemberUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Consortium payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Consortium {
    #[doc = "Gets or sets the blockchain member name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the protocol for the consortium."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<consortium::Protocol>,
}
impl Consortium {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod consortium {
    use super::*;
    #[doc = "Gets or sets the protocol for the consortium."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        NotSpecified,
        Parity,
        Quorum,
        Corda,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("Protocol", 0u32, "NotSpecified"),
                Self::Parity => serializer.serialize_unit_variant("Protocol", 1u32, "Parity"),
                Self::Quorum => serializer.serialize_unit_variant("Protocol", 2u32, "Quorum"),
                Self::Corda => serializer.serialize_unit_variant("Protocol", 3u32, "Corda"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of the consortium payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsortiumCollection {
    #[doc = "Gets or sets the collection of consortiums."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Consortium>,
}
impl ConsortiumCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Consortium approval"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsortiumMember {
    #[doc = "Gets the consortium member name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the consortium member display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets the consortium member subscription id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Gets the consortium member role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "Gets the consortium member status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets the consortium member join date."]
    #[serde(rename = "joinDate", default, with = "azure_core::date::rfc3339::option")]
    pub join_date: Option<time::OffsetDateTime>,
    #[doc = "Gets the consortium member modified date."]
    #[serde(rename = "dateModified", default, with = "azure_core::date::rfc3339::option")]
    pub date_modified: Option<time::OffsetDateTime>,
}
impl ConsortiumMember {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of consortium payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsortiumMemberCollection {
    #[doc = "Gets or sets the collection of consortiums."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConsortiumMember>,
    #[doc = "Gets or sets the URL, that the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConsortiumMemberCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConsortiumMemberCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ip range for firewall rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRule {
    #[doc = "Gets or sets the name of the firewall rules."]
    #[serde(rename = "ruleName", default, skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[doc = "Gets or sets the start IP address of the firewall rule range."]
    #[serde(rename = "startIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub start_ip_address: Option<String>,
    #[doc = "Gets or sets the end IP address of the firewall rule range."]
    #[serde(rename = "endIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub end_ip_address: Option<String>,
}
impl FirewallRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Name availability payload which is exposed in the response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[doc = "Gets or sets the value indicating whether the name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Gets or sets the message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets or sets the name availability reason."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<name_availability::Reason>,
}
impl NameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod name_availability {
    use super::*;
    #[doc = "Gets or sets the name availability reason."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        NotSpecified,
        AlreadyExists,
        Invalid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("Reason", 0u32, "NotSpecified"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::Invalid => serializer.serialize_unit_variant("Reason", 2u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Name availability request payload which is exposed in the request of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailabilityRequest {
    #[doc = "Gets or sets the name to check."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource to check."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation result payload which is exposed in the response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "Gets or sets the operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the operation start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the operation end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The core properties of the resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the service - e.g. \"Microsoft.Blockchain\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation payload which is exposed in the response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[doc = "Gets or sets the origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Gets or sets the operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets a value indicating whether the operation is a data action or not."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Operation display payload which is exposed in the response of the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ResourceProviderOperationDisplay>,
}
impl ResourceProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of operation payload which is exposed in the response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationCollection {
    #[doc = "Gets or sets the collection of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
    #[doc = "Gets or sets the URL, that the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceProviderOperationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceProviderOperationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation display payload which is exposed in the response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationDisplay {
    #[doc = "Gets or sets the name of the provider for display purposes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Gets or sets the name of the resource type for display purposes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Gets or sets the name of the operation for display purposes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Gets or sets the description of the provider for display purposes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ResourceProviderOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource type Sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeSku {
    #[doc = "Gets or sets the resource type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Gets or sets the Skus"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skus: Vec<SkuSetting>,
}
impl ResourceTypeSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of the resource type Sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeSkuCollection {
    #[doc = "Gets or sets the collection of resource type Sku."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceTypeSku>,
}
impl ResourceTypeSkuCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Blockchain member Sku in payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "Gets or sets Sku name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets Sku tier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku Setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuSetting {
    #[doc = "Gets or sets the Sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Gets or sets the locations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Gets or sets the required features."]
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
}
impl SkuSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The GEO location of the blockchain service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags of the service which is a list of key value pairs that describes the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Payload of the transaction node which is the request/response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionNode {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Gets or sets the transaction node location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Payload of transaction node properties payload in the transaction node payload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransactionNodeProperties>,
}
impl TransactionNode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of transaction node payload which is exposed in the request/response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionNodeCollection {
    #[doc = "Gets or sets the collection of transaction nodes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TransactionNode>,
    #[doc = "Gets or sets the URL, that the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TransactionNodeCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TransactionNodeCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Payload of transaction node properties payload in the transaction node payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionNodeProperties {
    #[doc = "Gets or sets the blockchain member provision state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<transaction_node_properties::ProvisioningState>,
    #[doc = "Gets or sets the transaction node dns endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,
    #[doc = "Gets or sets the transaction node public key."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "Gets or sets the transaction node dns endpoint basic auth user name."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Sets the transaction node dns endpoint basic auth password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Gets or sets the firewall rules."]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<FirewallRule>,
}
impl TransactionNodeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod transaction_node_properties {
    use super::*;
    #[doc = "Gets or sets the blockchain member provision state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotSpecified,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Update the payload of the transaction node properties in the transaction node payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionNodePropertiesUpdate {
    #[doc = "Sets the transaction node dns endpoint basic auth password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Gets or sets the firewall rules."]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<FirewallRule>,
}
impl TransactionNodePropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update the transaction node payload which is exposed in the request/response of the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionNodeUpdate {
    #[doc = "Update the payload of the transaction node properties in the transaction node payload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransactionNodePropertiesUpdate>,
}
impl TransactionNodeUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
