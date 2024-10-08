#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An error message"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The body of an error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The body of an error message"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "The error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A description of what caused the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target resource of the error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Extra error information"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS forwarding ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsForwardingRuleset {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "ETag of the DNS forwarding ruleset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of a DNS forwarding ruleset."]
    pub properties: DnsForwardingRulesetProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DnsForwardingRuleset {
    pub fn new(tracked_resource: TrackedResource, properties: DnsForwardingRulesetProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on DNS forwarding rulesets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsForwardingRulesetListResult {
    #[doc = "Enumeration of the DNS forwarding rulesets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DnsForwardingRuleset>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnsForwardingRulesetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DnsForwardingRulesetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS forwarding ruleset PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsForwardingRulesetPatch {
    #[doc = "The reference to the DNS resolver outbound endpoints that are used to route DNS queries matching the forwarding rules in the ruleset to the target DNS servers."]
    #[serde(
        rename = "dnsResolverOutboundEndpoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_resolver_outbound_endpoints: Vec<SubResource>,
    #[doc = "Tags for DNS Resolver."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DnsForwardingRulesetPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a DNS forwarding ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsForwardingRulesetProperties {
    #[doc = "The reference to the DNS resolver outbound endpoints that are used to route DNS queries matching the forwarding rules in the ruleset to the target DNS servers."]
    #[serde(rename = "dnsResolverOutboundEndpoints")]
    pub dns_resolver_outbound_endpoints: Vec<SubResource>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The resourceGuid for the DNS forwarding ruleset."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
}
impl DnsForwardingRulesetProperties {
    pub fn new(dns_resolver_outbound_endpoints: Vec<SubResource>) -> Self {
        Self {
            dns_resolver_outbound_endpoints,
            provisioning_state: None,
            resource_guid: None,
        }
    }
}
#[doc = "Describes a DNS resolver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResolver {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "ETag of the DNS resolver."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of a DNS resolver."]
    pub properties: DnsResolverProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DnsResolver {
    pub fn new(tracked_resource: TrackedResource, properties: DnsResolverProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Describes a DNS resolver domain list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResolverDomainList {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "ETag of the DNS resolver domain list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of a DNS resolver domain list."]
    pub properties: DnsResolverDomainListProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DnsResolverDomainList {
    pub fn new(tracked_resource: TrackedResource, properties: DnsResolverDomainListProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on DNS resolver domain lists."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverDomainListListResult {
    #[doc = "Enumeration of the DNS resolver domain lists."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DnsResolverDomainList>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnsResolverDomainListListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DnsResolverDomainListListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS resolver domain list for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverDomainListPatch {
    #[doc = "Represents the updatable properties of a DNS resolver domain list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsResolverDomainListPatchProperties>,
    #[doc = "Tags for DNS resolver domain list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DnsResolverDomainListPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the updatable properties of a DNS resolver domain list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverDomainListPatchProperties {
    #[doc = "The domains in the domain list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub domains: Vec<String>,
}
impl DnsResolverDomainListPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a DNS resolver domain list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResolverDomainListProperties {
    #[doc = "The domains in the domain list."]
    pub domains: Vec<String>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The Guid property of the resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<ResourceGuid>,
}
impl DnsResolverDomainListProperties {
    pub fn new(domains: Vec<String>) -> Self {
        Self {
            domains,
            provisioning_state: None,
            resource_guid: None,
        }
    }
}
#[doc = "The response to an enumeration operation on DNS resolvers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverListResult {
    #[doc = "Enumeration of the DNS resolvers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DnsResolver>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnsResolverListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DnsResolverListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS resolver for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverPatch {
    #[doc = "Tags for DNS Resolver."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DnsResolverPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS resolver policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResolverPolicy {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "ETag of the DNS resolver policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of a DNS resolver policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsResolverPolicyProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DnsResolverPolicy {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on DNS resolver policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverPolicyListResult {
    #[doc = "Enumeration of the DNS resolver policies."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DnsResolverPolicy>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnsResolverPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DnsResolverPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS resolver policy for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverPolicyPatch {
    #[doc = "Tags for DNS resolver policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DnsResolverPolicyPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a DNS resolver policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverPolicyProperties {
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The Guid property of the resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<ResourceGuid>,
}
impl DnsResolverPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS resolver policy virtual network link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResolverPolicyVirtualNetworkLink {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "ETag of the DNS resolver policy virtual network link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of a DNS resolver policy virtual network link."]
    pub properties: DnsResolverPolicyVirtualNetworkLinkProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DnsResolverPolicyVirtualNetworkLink {
    pub fn new(tracked_resource: TrackedResource, properties: DnsResolverPolicyVirtualNetworkLinkProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on DNS resolver policy virtual network links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverPolicyVirtualNetworkLinkListResult {
    #[doc = "Enumeration of the DNS resolver policy virtual network links."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DnsResolverPolicyVirtualNetworkLink>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnsResolverPolicyVirtualNetworkLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DnsResolverPolicyVirtualNetworkLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS resolver policy virtual network link for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverPolicyVirtualNetworkLinkPatch {
    #[doc = "Tags for the DNS resolver policy virtual network link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DnsResolverPolicyVirtualNetworkLinkPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a DNS resolver policy virtual network link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResolverPolicyVirtualNetworkLinkProperties {
    #[doc = "Reference to another ARM resource."]
    #[serde(rename = "virtualNetwork")]
    pub virtual_network: SubResource,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DnsResolverPolicyVirtualNetworkLinkProperties {
    pub fn new(virtual_network: SubResource) -> Self {
        Self {
            virtual_network,
            provisioning_state: None,
        }
    }
}
#[doc = "Represents the properties of a DNS resolver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResolverProperties {
    #[doc = "Reference to another ARM resource."]
    #[serde(rename = "virtualNetwork")]
    pub virtual_network: SubResource,
    #[doc = "The current status of the DNS resolver. This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "dnsResolverState", default, skip_serializing_if = "Option::is_none")]
    pub dns_resolver_state: Option<dns_resolver_properties::DnsResolverState>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The Guid property of the resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<ResourceGuid>,
}
impl DnsResolverProperties {
    pub fn new(virtual_network: SubResource) -> Self {
        Self {
            virtual_network,
            dns_resolver_state: None,
            provisioning_state: None,
            resource_guid: None,
        }
    }
}
pub mod dns_resolver_properties {
    use super::*;
    #[doc = "The current status of the DNS resolver. This is a read-only property and any attempt to set this value will be ignored."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DnsResolverState")]
    pub enum DnsResolverState {
        Connected,
        Disconnected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DnsResolverState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DnsResolverState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DnsResolverState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connected => serializer.serialize_unit_variant("DnsResolverState", 0u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("DnsResolverState", 1u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a DNS security rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsSecurityRule {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "ETag of the DNS security rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of a DNS security rule."]
    pub properties: DnsSecurityRuleProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DnsSecurityRule {
    pub fn new(tracked_resource: TrackedResource, properties: DnsSecurityRuleProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "The action to take on DNS requests that match the DNS security rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsSecurityRuleAction {
    #[doc = "The type of action to take."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<dns_security_rule_action::ActionType>,
    #[doc = "The response code for block actions."]
    #[serde(rename = "blockResponseCode", default, skip_serializing_if = "Option::is_none")]
    pub block_response_code: Option<dns_security_rule_action::BlockResponseCode>,
}
impl DnsSecurityRuleAction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dns_security_rule_action {
    use super::*;
    #[doc = "The type of action to take."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Allow,
        Alert,
        Block,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("ActionType", 0u32, "Allow"),
                Self::Alert => serializer.serialize_unit_variant("ActionType", 1u32, "Alert"),
                Self::Block => serializer.serialize_unit_variant("ActionType", 2u32, "Block"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The response code for block actions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BlockResponseCode")]
    pub enum BlockResponseCode {
        #[serde(rename = "SERVFAIL")]
        Servfail,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BlockResponseCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BlockResponseCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BlockResponseCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Servfail => serializer.serialize_unit_variant("BlockResponseCode", 0u32, "SERVFAIL"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response to an enumeration operation on DNS security rules within a DNS resolver policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsSecurityRuleListResult {
    #[doc = "Enumeration of the DNS security rules."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DnsSecurityRule>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnsSecurityRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DnsSecurityRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS security rule for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsSecurityRulePatch {
    #[doc = "Represents the updatable properties of a DNS security rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsSecurityRulePatchProperties>,
    #[doc = "Tags for DNS security rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DnsSecurityRulePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the updatable properties of a DNS security rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsSecurityRulePatchProperties {
    #[doc = "The action to take on DNS requests that match the DNS security rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<DnsSecurityRuleAction>,
    #[doc = "DNS resolver policy domains lists that the DNS security rule applies to."]
    #[serde(
        rename = "dnsResolverDomainLists",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_resolver_domain_lists: Vec<SubResource>,
    #[doc = "The state of DNS security rule."]
    #[serde(rename = "dnsSecurityRuleState", default, skip_serializing_if = "Option::is_none")]
    pub dns_security_rule_state: Option<dns_security_rule_patch_properties::DnsSecurityRuleState>,
    #[doc = "The priority of the DNS security rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}
impl DnsSecurityRulePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dns_security_rule_patch_properties {
    use super::*;
    #[doc = "The state of DNS security rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DnsSecurityRuleState")]
    pub enum DnsSecurityRuleState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DnsSecurityRuleState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DnsSecurityRuleState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DnsSecurityRuleState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("DnsSecurityRuleState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("DnsSecurityRuleState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents the properties of a DNS security rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsSecurityRuleProperties {
    #[doc = "The priority of the DNS security rule."]
    pub priority: i32,
    #[doc = "The action to take on DNS requests that match the DNS security rule."]
    pub action: DnsSecurityRuleAction,
    #[doc = "DNS resolver policy domains lists that the DNS security rule applies to."]
    #[serde(rename = "dnsResolverDomainLists")]
    pub dns_resolver_domain_lists: Vec<SubResource>,
    #[doc = "The state of DNS security rule."]
    #[serde(rename = "dnsSecurityRuleState", default, skip_serializing_if = "Option::is_none")]
    pub dns_security_rule_state: Option<dns_security_rule_properties::DnsSecurityRuleState>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DnsSecurityRuleProperties {
    pub fn new(priority: i32, action: DnsSecurityRuleAction, dns_resolver_domain_lists: Vec<SubResource>) -> Self {
        Self {
            priority,
            action,
            dns_resolver_domain_lists,
            dns_security_rule_state: None,
            provisioning_state: None,
        }
    }
}
pub mod dns_security_rule_properties {
    use super::*;
    #[doc = "The state of DNS security rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DnsSecurityRuleState")]
    pub enum DnsSecurityRuleState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DnsSecurityRuleState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DnsSecurityRuleState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DnsSecurityRuleState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("DnsSecurityRuleState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("DnsSecurityRuleState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a forwarding rule within a DNS forwarding ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForwardingRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "ETag of the forwarding rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of a forwarding rule within a DNS forwarding ruleset."]
    pub properties: ForwardingRuleProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ForwardingRule {
    pub fn new(properties: ForwardingRuleProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            etag: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on forwarding rules within a DNS forwarding ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForwardingRuleListResult {
    #[doc = "Enumeration of the forwarding rules."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ForwardingRule>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ForwardingRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ForwardingRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a forwarding rule for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForwardingRulePatch {
    #[doc = "Represents the updatable properties of a forwarding rule within a DNS forwarding ruleset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ForwardingRulePatchProperties>,
}
impl ForwardingRulePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the updatable properties of a forwarding rule within a DNS forwarding ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForwardingRulePatchProperties {
    #[doc = "DNS servers to forward the DNS query to."]
    #[serde(
        rename = "targetDnsServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_dns_servers: Vec<TargetDnsServer>,
    #[doc = "Metadata attached to the forwarding rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The state of forwarding rule."]
    #[serde(rename = "forwardingRuleState", default, skip_serializing_if = "Option::is_none")]
    pub forwarding_rule_state: Option<forwarding_rule_patch_properties::ForwardingRuleState>,
}
impl ForwardingRulePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod forwarding_rule_patch_properties {
    use super::*;
    #[doc = "The state of forwarding rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ForwardingRuleState")]
    pub enum ForwardingRuleState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ForwardingRuleState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ForwardingRuleState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ForwardingRuleState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ForwardingRuleState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ForwardingRuleState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ForwardingRuleState {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Represents the properties of a forwarding rule within a DNS forwarding ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForwardingRuleProperties {
    #[doc = "The domain name for the forwarding rule."]
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[doc = "DNS servers to forward the DNS query to."]
    #[serde(rename = "targetDnsServers")]
    pub target_dns_servers: Vec<TargetDnsServer>,
    #[doc = "Metadata attached to the forwarding rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The state of forwarding rule."]
    #[serde(rename = "forwardingRuleState", default, skip_serializing_if = "Option::is_none")]
    pub forwarding_rule_state: Option<forwarding_rule_properties::ForwardingRuleState>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ForwardingRuleProperties {
    pub fn new(domain_name: String, target_dns_servers: Vec<TargetDnsServer>) -> Self {
        Self {
            domain_name,
            target_dns_servers,
            metadata: None,
            forwarding_rule_state: None,
            provisioning_state: None,
        }
    }
}
pub mod forwarding_rule_properties {
    use super::*;
    #[doc = "The state of forwarding rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ForwardingRuleState")]
    pub enum ForwardingRuleState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ForwardingRuleState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ForwardingRuleState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ForwardingRuleState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ForwardingRuleState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ForwardingRuleState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes an inbound endpoint for a DNS resolver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InboundEndpoint {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "ETag of the inbound endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of an inbound endpoint for a DNS resolver."]
    pub properties: InboundEndpointProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl InboundEndpoint {
    pub fn new(tracked_resource: TrackedResource, properties: InboundEndpointProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on inbound endpoints for a DNS resolver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundEndpointListResult {
    #[doc = "Enumeration of the inbound endpoints for a DNS resolver."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<InboundEndpoint>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InboundEndpointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InboundEndpointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an inbound endpoint for a DNS resolver for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundEndpointPatch {
    #[doc = "Tags for inbound endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl InboundEndpointPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of an inbound endpoint for a DNS resolver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InboundEndpointProperties {
    #[doc = "IP configurations for the inbound endpoint."]
    #[serde(rename = "ipConfigurations")]
    pub ip_configurations: Vec<IpConfiguration>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The Guid property of the resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<ResourceGuid>,
}
impl InboundEndpointProperties {
    pub fn new(ip_configurations: Vec<IpConfiguration>) -> Self {
        Self {
            ip_configurations,
            provisioning_state: None,
            resource_guid: None,
        }
    }
}
#[doc = "IP configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpConfiguration {
    #[doc = "Reference to another ARM resource."]
    pub subnet: SubResource,
    #[doc = "Private IP address of the IP configuration."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Private IP address allocation method."]
    #[serde(rename = "privateIpAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<ip_configuration::PrivateIpAllocationMethod>,
}
impl IpConfiguration {
    pub fn new(subnet: SubResource) -> Self {
        Self {
            subnet,
            private_ip_address: None,
            private_ip_allocation_method: None,
        }
    }
}
pub mod ip_configuration {
    use super::*;
    #[doc = "Private IP address allocation method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateIpAllocationMethod")]
    pub enum PrivateIpAllocationMethod {
        Static,
        Dynamic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateIpAllocationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateIpAllocationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateIpAllocationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Static => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 0u32, "Static"),
                Self::Dynamic => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 1u32, "Dynamic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PrivateIpAllocationMethod {
        fn default() -> Self {
            Self::Dynamic
        }
    }
}
#[doc = "Describes an outbound endpoint for a DNS resolver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundEndpoint {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "ETag of the outbound endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of an outbound endpoint for a DNS resolver."]
    pub properties: OutboundEndpointProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl OutboundEndpoint {
    pub fn new(tracked_resource: TrackedResource, properties: OutboundEndpointProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on outbound endpoints for a DNS resolver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEndpointListResult {
    #[doc = "Enumeration of the outbound endpoints for a DNS resolver."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OutboundEndpoint>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutboundEndpointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OutboundEndpointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an outbound endpoint for a DNS resolver for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEndpointPatch {
    #[doc = "Tags for outbound endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl OutboundEndpointPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of an outbound endpoint for a DNS resolver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundEndpointProperties {
    #[doc = "Reference to another ARM resource."]
    pub subnet: SubResource,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The Guid property of the resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<ResourceGuid>,
}
impl OutboundEndpointProperties {
    pub fn new(subnet: SubResource) -> Self {
        Self {
            subnet,
            provisioning_state: None,
            resource_guid: None,
        }
    }
}
#[doc = "The current provisioning state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
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
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for a Azure Resource Manager proxy resource. It will not have tags and a location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ResourceGuid = String;
#[doc = "Reference to another ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[doc = "Resource ID."]
    pub id: String,
}
impl SubResource {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The response to an enumeration operation on sub-resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResourceListResult {
    #[doc = "Enumeration of the sub-resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SubResource>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SubResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SubResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a server to forward the DNS queries to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetDnsServer {
    #[doc = "DNS server IP address."]
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[doc = "DNS server port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl TargetDnsServer {
    pub fn new(ip_address: String) -> Self {
        Self { ip_address, port: None }
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "Reference to DNS forwarding ruleset and associated virtual network link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkDnsForwardingRuleset {
    #[doc = "DNS Forwarding Ruleset Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The reference to the virtual network link that associates between the DNS forwarding ruleset and virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkLinkSubResourceProperties>,
}
impl VirtualNetworkDnsForwardingRuleset {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to an enumeration operation on Virtual Network DNS Forwarding Ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkDnsForwardingRulesetListResult {
    #[doc = "Enumeration of the Virtual Network DNS Forwarding Ruleset."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetworkDnsForwardingRuleset>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkDnsForwardingRulesetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworkDnsForwardingRulesetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual network link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkLink {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "ETag of the virtual network link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of a virtual network link."]
    pub properties: VirtualNetworkLinkProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl VirtualNetworkLink {
    pub fn new(properties: VirtualNetworkLinkProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            etag: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on virtual network links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLinkListResult {
    #[doc = "Enumeration of the virtual network links."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetworkLink>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworkLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual network link for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLinkPatch {
    #[doc = "Represents the updatable properties of the virtual network link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkLinkPatchProperties>,
}
impl VirtualNetworkLinkPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the updatable properties of the virtual network link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLinkPatchProperties {
    #[doc = "Metadata attached to the virtual network link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl VirtualNetworkLinkPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a virtual network link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkLinkProperties {
    #[doc = "Reference to another ARM resource."]
    #[serde(rename = "virtualNetwork")]
    pub virtual_network: SubResource,
    #[doc = "Metadata attached to the virtual network link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl VirtualNetworkLinkProperties {
    pub fn new(virtual_network: SubResource) -> Self {
        Self {
            virtual_network,
            metadata: None,
            provisioning_state: None,
        }
    }
}
#[doc = "The reference to the virtual network link that associates between the DNS forwarding ruleset and virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLinkSubResourceProperties {
    #[doc = "Reference to another ARM resource."]
    #[serde(rename = "virtualNetworkLink", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_link: Option<SubResource>,
}
impl VirtualNetworkLinkSubResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
