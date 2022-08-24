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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsForwardingRulesetProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DnsForwardingRuleset {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on DNS forwarding rulesets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsForwardingRulesetListResult {
    #[doc = "Enumeration of the DNS forwarding rulesets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DnsForwardingRuleset>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnsForwardingRulesetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsForwardingRulesetProperties {
    #[doc = "The reference to the DNS resolver outbound endpoints that are used to route DNS queries matching the forwarding rules in the ruleset to the target DNS servers."]
    #[serde(rename = "dnsResolverOutboundEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_resolver_outbound_endpoints: Vec<SubResource>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The resourceGuid for the DNS forwarding ruleset."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
}
impl DnsForwardingRulesetProperties {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsResolverProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DnsResolver {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on DNS resolvers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsResolverListResult {
    #[doc = "Enumeration of the DNS resolvers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DnsResolver>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnsResolverListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "Describes a forwarding rule within a DNS forwarding ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForwardingRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "ETag of the forwarding rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of a forwarding rule within a DNS forwarding ruleset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ForwardingRuleProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ForwardingRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to an enumeration operation on forwarding rules within a DNS forwarding ruleset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForwardingRuleListResult {
    #[doc = "Enumeration of the forwarding rules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ForwardingRule>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ForwardingRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "targetDnsServers", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InboundEndpointProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl InboundEndpoint {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on inbound endpoints for a DNS resolver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundEndpointListResult {
    #[doc = "Enumeration of the inbound endpoints for a DNS resolver."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<InboundEndpoint>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InboundEndpointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundEndpointProperties {
    #[doc = "IP configurations for the inbound endpoint."]
    #[serde(rename = "ipConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_configurations: Vec<IpConfiguration>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The Guid property of the resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<ResourceGuid>,
}
impl InboundEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpConfiguration {
    #[doc = "Reference to another ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubResource>,
    #[doc = "Private IP address of the IP configuration."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Private IP address allocation method."]
    #[serde(rename = "privateIpAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<ip_configuration::PrivateIpAllocationMethod>,
}
impl IpConfiguration {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OutboundEndpointProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl OutboundEndpoint {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "The response to an enumeration operation on outbound endpoints for a DNS resolver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEndpointListResult {
    #[doc = "Enumeration of the outbound endpoints for a DNS resolver."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OutboundEndpoint>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutboundEndpointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEndpointProperties {
    #[doc = "Reference to another ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubResource>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The Guid property of the resource."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<ResourceGuid>,
}
impl OutboundEndpointProperties {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ResourceGuid = String;
#[doc = "Reference to another ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to an enumeration operation on sub-resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResourceListResult {
    #[doc = "Enumeration of the sub-resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubResource>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SubResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SubResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a server to forward the DNS queries to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetDnsServer {
    #[doc = "DNS server IP address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "DNS server port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl TargetDnsServer {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkDnsForwardingRuleset>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkDnsForwardingRulesetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualNetworkDnsForwardingRulesetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual network link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLink {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "ETag of the virtual network link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of a virtual network link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkLinkProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl VirtualNetworkLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to an enumeration operation on virtual network links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLinkListResult {
    #[doc = "Enumeration of the virtual network links."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkLink>,
    #[doc = "The continuation token for the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkLinkProperties {
    #[doc = "Reference to another ARM resource."]
    #[serde(rename = "virtualNetwork", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network: Option<SubResource>,
    #[doc = "Metadata attached to the virtual network link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl VirtualNetworkLinkProperties {
    pub fn new() -> Self {
        Self::default()
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
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
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
