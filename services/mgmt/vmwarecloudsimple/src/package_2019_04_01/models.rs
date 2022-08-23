#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Resource provider available operation model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperation {
    #[doc = "Resource provider available operation display model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableOperationDisplay>,
    #[doc = "Indicating whether the operation is a data action or not"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "{resourceProviderNamespace}/{resourceType}/{read|write|delete|action}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The origin of operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<available_operation::Origin>,
    #[doc = "Available operation display property service specification model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableOperationDisplayPropertyServiceSpecification>,
}
impl AvailableOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod available_operation {
    use super::*;
    #[doc = "The origin of operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
    }
}
#[doc = "Resource provider available operation display model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperationDisplay {
    #[doc = "Description of the operation for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of the operation for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Name of the provider for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource type for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl AvailableOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operation display property service specification model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperationDisplayPropertyServiceSpecification {
    #[doc = "List of available operation display property service specification metrics"]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<AvailableOperationDisplayPropertyServiceSpecificationMetricsList>,
}
impl AvailableOperationDisplayPropertyServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operation display property service specification metrics item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOperationDisplayPropertyServiceSpecificationMetricsItem {
    #[doc = "Metric's aggregation type for e.g. (Average, Total)"]
    #[serde(rename = "aggregationType")]
    pub aggregation_type: available_operation_display_property_service_specification_metrics_item::AggregationType,
    #[doc = "Metric's description"]
    #[serde(rename = "displayDescription")]
    pub display_description: String,
    #[doc = "Human readable metric's name"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Metric's name/id"]
    pub name: String,
    #[doc = "Metric's unit"]
    pub unit: String,
}
impl AvailableOperationDisplayPropertyServiceSpecificationMetricsItem {
    pub fn new(
        aggregation_type: available_operation_display_property_service_specification_metrics_item::AggregationType,
        display_description: String,
        display_name: String,
        name: String,
        unit: String,
    ) -> Self {
        Self {
            aggregation_type,
            display_description,
            display_name,
            name,
            unit,
        }
    }
}
pub mod available_operation_display_property_service_specification_metrics_item {
    use super::*;
    #[doc = "Metric's aggregation type for e.g. (Average, Total)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AggregationType {
        Average,
        Total,
    }
}
#[doc = "List of available operation display property service specification metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperationDisplayPropertyServiceSpecificationMetricsList {
    #[doc = "Metric specifications of operation"]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<AvailableOperationDisplayPropertyServiceSpecificationMetricsItem>,
}
impl AvailableOperationDisplayPropertyServiceSpecificationMetricsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of available operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperationsListResponse {
    #[doc = "Link for next list of available operations"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Returns a list of available operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableOperation>,
}
impl azure_core::Continuable for AvailableOperationsListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailableOperationsListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "General error model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsrpError {
    #[doc = "Error properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CsrpErrorBody>,
}
impl azure_core::Continuable for CsrpError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CsrpError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsrpErrorBody {
    #[doc = "Error's code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error's details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CsrpErrorBody>,
    #[doc = "Error's message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error's target"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl CsrpErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Host name model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationHostName {
    #[doc = "Hostname"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of host name"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<customization_host_name::Type>,
}
impl CustomizationHostName {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod customization_host_name {
    use super::*;
    #[doc = "Type of host name"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "USER_DEFINED")]
        UserDefined,
        #[serde(rename = "PREFIX_BASED")]
        PrefixBased,
        #[serde(rename = "FIXED")]
        Fixed,
        #[serde(rename = "VIRTUAL_MACHINE_NAME")]
        VirtualMachineName,
        #[serde(rename = "CUSTOM_NAME")]
        CustomName,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationIpAddress {
    #[doc = "Argument when Custom ip type is selected"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub argument: Option<String>,
    #[doc = "Defined Ip Address when Fixed ip type is selected"]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Customization Specification ip type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<customization_ip_address::Type>,
}
impl CustomizationIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod customization_ip_address {
    use super::*;
    #[doc = "Customization Specification ip type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "CUSTOM")]
        Custom,
        #[serde(rename = "DHCP_IP")]
        DhcpIp,
        #[serde(rename = "FIXED_IP")]
        FixedIp,
        #[serde(rename = "USER_DEFINED")]
        UserDefined,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationIpSettings {
    #[doc = "The list of gateways"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gateway: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<CustomizationIpAddress>,
    #[doc = "Adapter subnet mask"]
    #[serde(rename = "subnetMask", default, skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
}
impl CustomizationIpSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationIdentity {
    #[doc = "Windows Text Identity. Prepared data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[doc = "Host name model"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<CustomizationHostName>,
    #[doc = "Identity type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<customization_identity::Type>,
    #[doc = "Windows Identity. User data customization"]
    #[serde(rename = "userData", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<customization_identity::UserData>,
}
impl CustomizationIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod customization_identity {
    use super::*;
    #[doc = "Identity type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "WINDOWS_TEXT")]
        WindowsText,
        #[serde(rename = "WINDOWS")]
        Windows,
        #[serde(rename = "LINUX")]
        Linux,
    }
    #[doc = "Windows Identity. User data customization"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct UserData {
        #[doc = "Is password predefined in customization policy"]
        #[serde(rename = "isPasswordPredefined", default, skip_serializing_if = "Option::is_none")]
        pub is_password_predefined: Option<bool>,
    }
    impl UserData {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationNicSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter: Option<CustomizationIpSettings>,
    #[doc = "NIC mac address"]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}
impl CustomizationNicSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of customization polices response model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationPoliciesListResponse {
    #[doc = "Link for next list of the Customization policy"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of the customization policies"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomizationPolicy>,
}
impl azure_core::Continuable for CustomizationPoliciesListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CustomizationPoliciesListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The virtual machine customization policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationPolicy {
    #[doc = "Customization policy azure id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Customization policy name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The properties of Customization policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomizationPolicyProperties>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CustomizationPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of Customization policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationPolicyProperties {
    #[doc = "Policy description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Private cloud id"]
    #[serde(rename = "privateCloudId", default, skip_serializing_if = "Option::is_none")]
    pub private_cloud_id: Option<String>,
    #[doc = "The specification for Customization Policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specification: Option<CustomizationSpecification>,
    #[doc = "The type of customization (Linux or Windows)"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<customization_policy_properties::Type>,
    #[doc = "Policy version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl CustomizationPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod customization_policy_properties {
    use super::*;
    #[doc = "The type of customization (Linux or Windows)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "LINUX")]
        Linux,
        #[serde(rename = "WINDOWS")]
        Windows,
    }
}
#[doc = "The specification for Customization Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<CustomizationIdentity>,
    #[doc = "Network interface settings"]
    #[serde(rename = "nicSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub nic_settings: Vec<CustomizationNicSetting>,
}
impl CustomizationSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dedicated cloud node model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCloudNode {
    #[doc = "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/dedicatedCloudNodes/{dedicatedCloudNodeName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure region"]
    pub location: String,
    #[doc = "{dedicatedCloudNodeName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of dedicated cloud node"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedCloudNodeProperties>,
    #[doc = "The purchase SKU for CloudSimple paid resources"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Tags model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "{resourceProviderNamespace}/{resourceType}"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DedicatedCloudNode {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            location,
            name: None,
            properties: None,
            sku: None,
            tags: None,
            type_: None,
        }
    }
}
#[doc = "List of dedicated nodes response model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedCloudNodeListResponse {
    #[doc = "Link for next list of DedicatedCloudNode"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Results of the DedicatedCloudNode list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DedicatedCloudNode>,
}
impl azure_core::Continuable for DedicatedCloudNodeListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DedicatedCloudNodeListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of dedicated cloud node"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCloudNodeProperties {
    #[doc = "Availability Zone id, e.g. \"az1\""]
    #[serde(rename = "availabilityZoneId")]
    pub availability_zone_id: String,
    #[doc = "Availability Zone name, e.g. \"Availability Zone 1\""]
    #[serde(rename = "availabilityZoneName", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    #[doc = "VMWare Cloud Rack Name"]
    #[serde(rename = "cloudRackName", default, skip_serializing_if = "Option::is_none")]
    pub cloud_rack_name: Option<String>,
    #[doc = "date time the resource was created"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "count of nodes to create"]
    #[serde(rename = "nodesCount")]
    pub nodes_count: i64,
    #[doc = "Placement Group id, e.g. \"n1\""]
    #[serde(rename = "placementGroupId")]
    pub placement_group_id: String,
    #[doc = "Placement Name, e.g. \"Placement Group 1\""]
    #[serde(rename = "placementGroupName", default, skip_serializing_if = "Option::is_none")]
    pub placement_group_name: Option<String>,
    #[doc = "Private Cloud Id"]
    #[serde(rename = "privateCloudId", default, skip_serializing_if = "Option::is_none")]
    pub private_cloud_id: Option<String>,
    #[doc = "Resource Pool Name"]
    #[serde(rename = "privateCloudName", default, skip_serializing_if = "Option::is_none")]
    pub private_cloud_name: Option<String>,
    #[doc = "The provisioning status of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "purchase id"]
    #[serde(rename = "purchaseId")]
    pub purchase_id: String,
    #[doc = "The purchase SKU for CloudSimple paid resources"]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<SkuDescription>,
    #[doc = "Node status, indicates is private cloud set up on this node or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<dedicated_cloud_node_properties::Status>,
    #[doc = "VMWare Cluster Name"]
    #[serde(rename = "vmwareClusterName", default, skip_serializing_if = "Option::is_none")]
    pub vmware_cluster_name: Option<String>,
}
impl DedicatedCloudNodeProperties {
    pub fn new(availability_zone_id: String, nodes_count: i64, placement_group_id: String, purchase_id: String) -> Self {
        Self {
            availability_zone_id,
            availability_zone_name: None,
            cloud_rack_name: None,
            created: None,
            nodes_count,
            placement_group_id,
            placement_group_name: None,
            private_cloud_id: None,
            private_cloud_name: None,
            provisioning_state: None,
            purchase_id,
            sku_description: None,
            status: None,
            vmware_cluster_name: None,
        }
    }
}
pub mod dedicated_cloud_node_properties {
    use super::*;
    #[doc = "Node status, indicates is private cloud set up on this node or not"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unused")]
        Unused,
        #[serde(rename = "used")]
        Used,
    }
}
#[doc = "Dedicated cloud service model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCloudService {
    #[doc = "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/dedicatedCloudServices/{dedicatedCloudServiceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure region"]
    pub location: String,
    #[doc = "{dedicatedCloudServiceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of dedicated cloud service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedCloudServiceProperties>,
    #[doc = "Tags model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "{resourceProviderNamespace}/{resourceType}"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DedicatedCloudService {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            location,
            name: None,
            properties: None,
            tags: None,
            type_: None,
        }
    }
}
#[doc = "List of dedicated cloud services"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedCloudServiceListResponse {
    #[doc = "Link for next list of DedicatedCloudNode"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Results of the DedicatedCloudService list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DedicatedCloudService>,
}
impl azure_core::Continuable for DedicatedCloudServiceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DedicatedCloudServiceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of dedicated cloud service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCloudServiceProperties {
    #[doc = "gateway Subnet for the account. It will collect the subnet address and always treat it as /28"]
    #[serde(rename = "gatewaySubnet")]
    pub gateway_subnet: String,
    #[doc = "indicates whether account onboarded or not in a given region"]
    #[serde(rename = "isAccountOnboarded", default, skip_serializing_if = "Option::is_none")]
    pub is_account_onboarded: Option<dedicated_cloud_service_properties::IsAccountOnboarded>,
    #[doc = "total nodes purchased"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<i64>,
    #[doc = "link to a service management web portal"]
    #[serde(rename = "serviceURL", default, skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
}
impl DedicatedCloudServiceProperties {
    pub fn new(gateway_subnet: String) -> Self {
        Self {
            gateway_subnet,
            is_account_onboarded: None,
            nodes: None,
            service_url: None,
        }
    }
}
pub mod dedicated_cloud_service_properties {
    use super::*;
    #[doc = "indicates whether account onboarded or not in a given region"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IsAccountOnboarded {
        #[serde(rename = "notOnBoarded")]
        NotOnBoarded,
        #[serde(rename = "onBoarded")]
        OnBoarded,
        #[serde(rename = "onBoardingFailed")]
        OnBoardingFailed,
        #[serde(rename = "onBoarding")]
        OnBoarding,
    }
}
#[doc = "Guest OS Customization properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestOsCustomization {
    #[doc = "List of dns servers to use"]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<Ipv4Address>,
    #[doc = "Virtual Machine hostname"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Password for login"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "id of customization policy"]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "Username for login"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl GuestOsCustomization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Guest OS nic customization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestOsnicCustomization {
    #[doc = "IP address allocation method"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation: Option<guest_osnic_customization::Allocation>,
    #[doc = "List of dns servers to use"]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<Ipv4Address>,
    #[doc = "Gateway addresses assigned to nic"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gateway: Vec<Ipv4Address>,
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Ipv4Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<Ipv4Address>,
    #[serde(rename = "primaryWinsServer", default, skip_serializing_if = "Option::is_none")]
    pub primary_wins_server: Option<Ipv4Address>,
    #[serde(rename = "secondaryWinsServer", default, skip_serializing_if = "Option::is_none")]
    pub secondary_wins_server: Option<Ipv4Address>,
}
impl GuestOsnicCustomization {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod guest_osnic_customization {
    use super::*;
    #[doc = "IP address allocation method"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Allocation {
        #[serde(rename = "static")]
        Static,
        #[serde(rename = "dynamic")]
        Dynamic,
    }
}
pub type Ipv4Address = String;
#[doc = "Operation error model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationError {
    #[doc = "Error's code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error's message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl OperationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation status response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResource {
    #[doc = "End time of the operation"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Operation error model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationError>,
    #[doc = "Operation Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Start time of the operation"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Operation status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl OperationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "General patch payload modal"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchPayload {
    #[doc = "Tags model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl PatchPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private cloud model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateCloud {
    #[doc = "Azure Id, e.g. \"/subscriptions/4da99247-a172-4ed6-8ae9-ebed2d12f839/providers/Microsoft.VMwareCloudSimple/privateClouds/cloud123\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Location where private cloud created, e.g \"westus\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Private cloud name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of private"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateCloudProperties>,
    #[doc = "Azure Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<private_cloud::Type>,
}
impl PrivateCloud {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_cloud {
    use super::*;
    #[doc = "Azure Resource type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.VMwareCloudSimple/privateClouds")]
        MicrosoftVMwareCloudSimplePrivateClouds,
    }
}
#[doc = "List of private clouds"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateCloudList {
    #[doc = "Link for next list of Private Clouds"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "the list of private clouds"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateCloud>,
}
impl azure_core::Continuable for PrivateCloudList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateCloudList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of private"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateCloudProperties {
    #[doc = "Availability Zone id, e.g. \"az1\""]
    #[serde(rename = "availabilityZoneId", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[doc = "Availability Zone name, e.g. \"Availability Zone 1\""]
    #[serde(rename = "availabilityZoneName", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    #[doc = "Number of clusters"]
    #[serde(rename = "clustersNumber", default, skip_serializing_if = "Option::is_none")]
    pub clusters_number: Option<i64>,
    #[doc = "User's emails who created cloud"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "When private cloud was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Array of DNS servers"]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
    #[doc = "Expiration date of PC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[doc = "Nsx Type, e.g. \"Advanced\""]
    #[serde(rename = "nsxType", default, skip_serializing_if = "Option::is_none")]
    pub nsx_type: Option<String>,
    #[doc = "Placement Group id, e.g. \"n1\""]
    #[serde(rename = "placementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub placement_group_id: Option<String>,
    #[doc = "Placement Group name"]
    #[serde(rename = "placementGroupName", default, skip_serializing_if = "Option::is_none")]
    pub placement_group_name: Option<String>,
    #[doc = "Id of a private cloud"]
    #[serde(rename = "privateCloudId", default, skip_serializing_if = "Option::is_none")]
    pub private_cloud_id: Option<String>,
    #[doc = "The list of Resource Pools"]
    #[serde(rename = "resourcePools", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_pools: Vec<ResourcePool>,
    #[doc = "Private Cloud state, e.g. \"operational\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Number of cores"]
    #[serde(rename = "totalCpuCores", default, skip_serializing_if = "Option::is_none")]
    pub total_cpu_cores: Option<i64>,
    #[doc = "Number of nodes"]
    #[serde(rename = "totalNodes", default, skip_serializing_if = "Option::is_none")]
    pub total_nodes: Option<i64>,
    #[doc = "Memory size"]
    #[serde(rename = "totalRam", default, skip_serializing_if = "Option::is_none")]
    pub total_ram: Option<i64>,
    #[doc = "Disk space in TB"]
    #[serde(rename = "totalStorage", default, skip_serializing_if = "Option::is_none")]
    pub total_storage: Option<f64>,
    #[doc = "Virtualization type e.g. \"vSphere\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "e.g. \"6.5u2\""]
    #[serde(rename = "vSphereVersion", default, skip_serializing_if = "Option::is_none")]
    pub v_sphere_version: Option<String>,
    #[doc = "FQDN for vcenter access"]
    #[serde(rename = "vcenterFqdn", default, skip_serializing_if = "Option::is_none")]
    pub vcenter_fqdn: Option<String>,
    #[doc = "Vcenter ip address"]
    #[serde(rename = "vcenterRefid", default, skip_serializing_if = "Option::is_none")]
    pub vcenter_refid: Option<String>,
    #[doc = "The list of Virtual Machine Templates"]
    #[serde(rename = "virtualMachineTemplates", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_machine_templates: Vec<VirtualMachineTemplate>,
    #[doc = "The list of Virtual Networks"]
    #[serde(rename = "virtualNetworks", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_networks: Vec<VirtualNetwork>,
    #[doc = "Is Vrops enabled/disabled"]
    #[serde(rename = "vrOpsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub vr_ops_enabled: Option<bool>,
}
impl PrivateCloudProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource pool model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePool {
    #[doc = "resource pool id (privateCloudId:vsphereId)"]
    pub id: String,
    #[doc = "Azure region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "{ResourcePoolName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Private Cloud Id"]
    #[serde(rename = "privateCloudId", default, skip_serializing_if = "Option::is_none")]
    pub private_cloud_id: Option<String>,
    #[doc = "Properties of resource pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourcePoolProperties>,
    #[doc = "{resourceProviderNamespace}/{resourceType}"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ResourcePool {
    pub fn new(id: String) -> Self {
        Self {
            id,
            location: None,
            name: None,
            private_cloud_id: None,
            properties: None,
            type_: None,
        }
    }
}
#[doc = "Properties of resource pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcePoolProperties {
    #[doc = "Hierarchical resource pool name"]
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}
impl ResourcePoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of resource pools response model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcePoolsListResponse {
    #[doc = "Link for next list of ResourcePoolsList"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Results of the Resource pools list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourcePool>,
}
impl azure_core::Continuable for ResourcePoolsListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourcePoolsListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The purchase SKU for CloudSimple paid resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The capacity of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    #[doc = "dedicatedCloudNode example: 8 x Ten-Core Intel® Xeon® Processor E5-2640 v4 2.40GHz 25MB Cache (90W); 12 x 64GB PC4-19200 2400MHz DDR4 ECC Registered DIMM, ..."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The name of the SKU for VMWare CloudSimple Node"]
    pub name: String,
    #[doc = "The tier of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            capacity: None,
            description: None,
            family: None,
            name,
            tier: None,
        }
    }
}
#[doc = "SKU availability model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuAvailability {
    #[doc = "CloudSimple Availability Zone id"]
    #[serde(rename = "dedicatedAvailabilityZoneId", default, skip_serializing_if = "Option::is_none")]
    pub dedicated_availability_zone_id: Option<String>,
    #[doc = "CloudSimple Availability Zone Name"]
    #[serde(rename = "dedicatedAvailabilityZoneName", default, skip_serializing_if = "Option::is_none")]
    pub dedicated_availability_zone_name: Option<String>,
    #[doc = "CloudSimple Placement Group Id"]
    #[serde(rename = "dedicatedPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub dedicated_placement_group_id: Option<String>,
    #[doc = "CloudSimple Placement Group name"]
    #[serde(rename = "dedicatedPlacementGroupName", default, skip_serializing_if = "Option::is_none")]
    pub dedicated_placement_group_name: Option<String>,
    #[doc = "indicates how many resources of a given SKU is available in a AZ->PG"]
    pub limit: i64,
    #[doc = "resource type e.g. DedicatedCloudNodes"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "sku id"]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "sku name"]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
}
impl SkuAvailability {
    pub fn new(limit: i64) -> Self {
        Self {
            dedicated_availability_zone_id: None,
            dedicated_availability_zone_name: None,
            dedicated_placement_group_id: None,
            dedicated_placement_group_name: None,
            limit,
            resource_type: None,
            sku_id: None,
            sku_name: None,
        }
    }
}
#[doc = "List of SKU availabilities"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuAvailabilityListResponse {
    #[doc = "Link for next list of DedicatedCloudNode"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Results of the DedicatedPlacementGroupSkuAvailability list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuAvailability>,
}
impl azure_core::Continuable for SkuAvailabilityListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SkuAvailabilityListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The purchase SKU for CloudSimple paid resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuDescription {
    #[doc = "SKU's id"]
    pub id: String,
    #[doc = "SKU's name"]
    pub name: String,
}
impl SkuDescription {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
#[doc = "Tags model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Usage model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[doc = "The current usage value"]
    #[serde(rename = "currentValue")]
    pub current_value: i64,
    #[doc = "limit of a given sku in a region for a subscription. The maximum permitted value for the usage quota. If there is no limit, this value will be -1"]
    pub limit: i64,
    #[doc = "User name model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
    #[doc = "The usages' unit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
}
impl Usage {
    pub fn new(current_value: i64, limit: i64) -> Self {
        Self {
            current_value,
            limit,
            name: None,
            unit: None,
        }
    }
}
pub mod usage {
    use super::*;
    #[doc = "The usages' unit"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountPerSecond,
        BytesPerSecond,
    }
}
#[doc = "List of usages"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResponse {
    #[doc = "Link for next list of DedicatedCloudNode"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of usages"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl azure_core::Continuable for UsageListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UsageListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User name model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "e.g. \"Virtual Machines\""]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
    #[doc = "resource type or resource type sku name, e.g. virtualMachines"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual disk model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualDisk {
    #[doc = "Disk's Controller id"]
    #[serde(rename = "controllerId")]
    pub controller_id: String,
    #[doc = "Disk's independence mode type"]
    #[serde(rename = "independenceMode")]
    pub independence_mode: virtual_disk::IndependenceMode,
    #[doc = "Disk's total size"]
    #[serde(rename = "totalSize")]
    pub total_size: i64,
    #[doc = "Disk's id"]
    #[serde(rename = "virtualDiskId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_disk_id: Option<String>,
    #[doc = "Disk's display name"]
    #[serde(rename = "virtualDiskName", default, skip_serializing_if = "Option::is_none")]
    pub virtual_disk_name: Option<String>,
}
impl VirtualDisk {
    pub fn new(controller_id: String, independence_mode: virtual_disk::IndependenceMode, total_size: i64) -> Self {
        Self {
            controller_id,
            independence_mode,
            total_size,
            virtual_disk_id: None,
            virtual_disk_name: None,
        }
    }
}
pub mod virtual_disk {
    use super::*;
    #[doc = "Disk's independence mode type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IndependenceMode {
        #[serde(rename = "persistent")]
        Persistent,
        #[serde(rename = "independent_persistent")]
        IndependentPersistent,
        #[serde(rename = "independent_nonpersistent")]
        IndependentNonpersistent,
    }
}
#[doc = "Virtual disk controller model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualDiskController {
    #[doc = "Controller's id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name of Controller"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "dik controller subtype (VMWARE_PARAVIRTUAL, BUS_PARALLEL, LSI_PARALLEL, LSI_SAS)"]
    #[serde(rename = "subType", default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[doc = "disk controller type (SCSI)"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl VirtualDiskController {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual machine model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[doc = "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/virtualMachines/{virtualMachineName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure region"]
    pub location: String,
    #[doc = "{virtualMachineName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of virtual machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineProperties>,
    #[doc = "Tags model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "{resourceProviderNamespace}/{resourceType}"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl VirtualMachine {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            location,
            name: None,
            properties: None,
            tags: None,
            type_: None,
        }
    }
}
#[doc = "List of virtual machines"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineListResponse {
    #[doc = "Link for next list of VirtualMachines"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Results of the VirtualMachine list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachine>,
}
impl azure_core::Continuable for VirtualMachineListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of virtual machine"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineProperties {
    #[doc = "The amount of memory"]
    #[serde(rename = "amountOfRam")]
    pub amount_of_ram: i64,
    #[doc = "The list of Virtual Disks' Controllers"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub controllers: Vec<VirtualDiskController>,
    #[doc = "Guest OS Customization properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization: Option<GuestOsCustomization>,
    #[doc = "The list of Virtual Disks"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VirtualDisk>,
    #[doc = "The DNS name of Virtual Machine in VCenter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dnsname: Option<String>,
    #[doc = "Expose Guest OS or not"]
    #[serde(rename = "exposeToGuestVM", default, skip_serializing_if = "Option::is_none")]
    pub expose_to_guest_vm: Option<bool>,
    #[doc = "The path to virtual machine folder in VCenter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[doc = "The name of Guest OS"]
    #[serde(rename = "guestOS", default, skip_serializing_if = "Option::is_none")]
    pub guest_os: Option<String>,
    #[doc = "The Guest OS type"]
    #[serde(rename = "guestOSType", default, skip_serializing_if = "Option::is_none")]
    pub guest_os_type: Option<virtual_machine_properties::GuestOsType>,
    #[doc = "The list of Virtual NICs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nics: Vec<VirtualNic>,
    #[doc = "The number of CPU cores"]
    #[serde(rename = "numberOfCores")]
    pub number_of_cores: i64,
    #[doc = "Password for login. Deprecated - use customization property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Private Cloud Id"]
    #[serde(rename = "privateCloudId")]
    pub private_cloud_id: String,
    #[doc = "The provisioning status of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The public ip of Virtual Machine"]
    #[serde(rename = "publicIP", default, skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[doc = "Resource pool model"]
    #[serde(rename = "resourcePool", default, skip_serializing_if = "Option::is_none")]
    pub resource_pool: Option<ResourcePool>,
    #[doc = "The status of Virtual machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<virtual_machine_properties::Status>,
    #[doc = "Virtual Machine Template Id"]
    #[serde(rename = "templateId", default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[doc = "Username for login. Deprecated - use customization property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The list of Virtual VSphere Networks"]
    #[serde(rename = "vSphereNetworks", default, skip_serializing_if = "Vec::is_empty")]
    pub v_sphere_networks: Vec<String>,
    #[doc = "The internal id of Virtual Machine in VCenter"]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "VMware tools version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vmwaretools: Option<String>,
}
impl VirtualMachineProperties {
    pub fn new(amount_of_ram: i64, number_of_cores: i64, private_cloud_id: String) -> Self {
        Self {
            amount_of_ram,
            controllers: Vec::new(),
            customization: None,
            disks: Vec::new(),
            dnsname: None,
            expose_to_guest_vm: None,
            folder: None,
            guest_os: None,
            guest_os_type: None,
            nics: Vec::new(),
            number_of_cores,
            password: None,
            private_cloud_id,
            provisioning_state: None,
            public_ip: None,
            resource_pool: None,
            status: None,
            template_id: None,
            username: None,
            v_sphere_networks: Vec::new(),
            vm_id: None,
            vmwaretools: None,
        }
    }
}
pub mod virtual_machine_properties {
    use super::*;
    #[doc = "The Guest OS type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GuestOsType {
        #[serde(rename = "linux")]
        Linux,
        #[serde(rename = "windows")]
        Windows,
        #[serde(rename = "other")]
        Other,
    }
    #[doc = "The status of Virtual machine"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "suspended")]
        Suspended,
        #[serde(rename = "poweredoff")]
        Poweredoff,
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "deallocating")]
        Deallocating,
        #[serde(rename = "deleting")]
        Deleting,
    }
}
#[doc = "List of virtual machine stop modes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineStopMode {
    #[doc = "mode indicates a type of stop operation - reboot, suspend, shutdown or power-off"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<virtual_machine_stop_mode::Mode>,
}
impl VirtualMachineStopMode {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_stop_mode {
    use super::*;
    #[doc = "mode indicates a type of stop operation - reboot, suspend, shutdown or power-off"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        #[serde(rename = "reboot")]
        Reboot,
        #[serde(rename = "suspend")]
        Suspend,
        #[serde(rename = "shutdown")]
        Shutdown,
        #[serde(rename = "poweroff")]
        Poweroff,
    }
}
#[doc = "Virtual machine template model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineTemplate {
    #[doc = "virtual machine template id (privateCloudId:vsphereId)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "{virtualMachineTemplateName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of virtual machine template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineTemplateProperties>,
    #[doc = "{resourceProviderNamespace}/{resourceType}"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl VirtualMachineTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of virtual machine templates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineTemplateListResponse {
    #[doc = "Link for next list of VirtualMachineTemplate"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Results of the VM template list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachineTemplate>,
}
impl azure_core::Continuable for VirtualMachineTemplateListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineTemplateListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of virtual machine template"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineTemplateProperties {
    #[doc = "The amount of memory"]
    #[serde(rename = "amountOfRam", default, skip_serializing_if = "Option::is_none")]
    pub amount_of_ram: Option<i64>,
    #[doc = "The list of Virtual Disk Controllers"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub controllers: Vec<VirtualDiskController>,
    #[doc = "The description of Virtual Machine Template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of Virtual Disks"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VirtualDisk>,
    #[doc = "Expose Guest OS or not"]
    #[serde(rename = "exposeToGuestVM", default, skip_serializing_if = "Option::is_none")]
    pub expose_to_guest_vm: Option<bool>,
    #[doc = "The Guest OS"]
    #[serde(rename = "guestOS", default, skip_serializing_if = "Option::is_none")]
    pub guest_os: Option<String>,
    #[doc = "The Guest OS types"]
    #[serde(rename = "guestOSType", default, skip_serializing_if = "Option::is_none")]
    pub guest_os_type: Option<String>,
    #[doc = "The list of Virtual NICs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nics: Vec<VirtualNic>,
    #[doc = "The number of CPU cores"]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i64>,
    #[doc = "path to folder"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The Private Cloud Id"]
    #[serde(rename = "privateCloudId")]
    pub private_cloud_id: String,
    #[doc = "The list of VSphere networks"]
    #[serde(rename = "vSphereNetworks", default, skip_serializing_if = "Vec::is_empty")]
    pub v_sphere_networks: Vec<String>,
    #[doc = "The tags from VSphere"]
    #[serde(rename = "vSphereTags", default, skip_serializing_if = "Vec::is_empty")]
    pub v_sphere_tags: Vec<String>,
    #[doc = "The VMware tools version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vmwaretools: Option<String>,
}
impl VirtualMachineTemplateProperties {
    pub fn new(private_cloud_id: String) -> Self {
        Self {
            amount_of_ram: None,
            controllers: Vec::new(),
            description: None,
            disks: Vec::new(),
            expose_to_guest_vm: None,
            guest_os: None,
            guest_os_type: None,
            nics: Vec::new(),
            number_of_cores: None,
            path: None,
            private_cloud_id,
            v_sphere_networks: Vec::new(),
            v_sphere_tags: Vec::new(),
            vmwaretools: None,
        }
    }
}
#[doc = "Virtual network model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetwork {
    #[doc = "can be used in vm creation/deletion"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignable: Option<bool>,
    #[doc = "virtual network id (privateCloudId:vsphereId)"]
    pub id: String,
    #[doc = "Azure region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "{VirtualNetworkName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of virtual network"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkProperties>,
    #[doc = "{resourceProviderNamespace}/{resourceType}"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl VirtualNetwork {
    pub fn new(id: String) -> Self {
        Self {
            assignable: None,
            id,
            location: None,
            name: None,
            properties: None,
            type_: None,
        }
    }
}
#[doc = "List of virtual networks"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkListResponse {
    #[doc = "Link for next list of VirtualNetwork"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Results of the VirtualNetwork list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetwork>,
}
impl azure_core::Continuable for VirtualNetworkListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualNetworkListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of virtual network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkProperties {
    #[doc = "The Private Cloud id"]
    #[serde(rename = "privateCloudId", default, skip_serializing_if = "Option::is_none")]
    pub private_cloud_id: Option<String>,
}
impl VirtualNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual NIC model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNic {
    #[doc = "Guest OS nic customization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization: Option<GuestOsnicCustomization>,
    #[doc = "NIC ip address"]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "NIC MAC address"]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Virtual network model"]
    pub network: VirtualNetwork,
    #[doc = "NIC type"]
    #[serde(rename = "nicType")]
    pub nic_type: virtual_nic::NicType,
    #[doc = "Is NIC powered on/off on boot"]
    #[serde(rename = "powerOnBoot", default, skip_serializing_if = "Option::is_none")]
    pub power_on_boot: Option<bool>,
    #[doc = "NIC id"]
    #[serde(rename = "virtualNicId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_nic_id: Option<String>,
    #[doc = "NIC name"]
    #[serde(rename = "virtualNicName", default, skip_serializing_if = "Option::is_none")]
    pub virtual_nic_name: Option<String>,
}
impl VirtualNic {
    pub fn new(network: VirtualNetwork, nic_type: virtual_nic::NicType) -> Self {
        Self {
            customization: None,
            ip_addresses: Vec::new(),
            mac_address: None,
            network,
            nic_type,
            power_on_boot: None,
            virtual_nic_id: None,
            virtual_nic_name: None,
        }
    }
}
pub mod virtual_nic {
    use super::*;
    #[doc = "NIC type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NicType {
        E1000,
        #[serde(rename = "E1000E")]
        E1000e,
        #[serde(rename = "PCNET32")]
        Pcnet32,
        #[serde(rename = "VMXNET")]
        Vmxnet,
        #[serde(rename = "VMXNET2")]
        Vmxnet2,
        #[serde(rename = "VMXNET3")]
        Vmxnet3,
    }
}
