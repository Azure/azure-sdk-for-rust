#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Information regarding availability of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckAvailabilityResponse {
    #[doc = "<code>true</code> indicates name is valid and available. <code>false</code> indicates the name is invalid, unavailable, or both."]
    #[serde(rename = "isAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    #[doc = "<code>Invalid</code> indicates the name provided does not match Azure App Service naming requirements. <code>AlreadyExists</code> indicates that the name is already in use and is therefore unavailable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_availability_response::Reason>,
    #[doc = "If reason == invalid, provide the user with the reason why the given name is invalid, and provide the resource naming requirements so that the user can select a valid name. If reason == AlreadyExists, explain that resource name is already in use, and direct them to select a different name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_availability_response {
    use super::*;
    #[doc = "<code>Invalid</code> indicates the name provided does not match Azure App Service naming requirements. <code>AlreadyExists</code> indicates that the name is already in use and is therefore unavailable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
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
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Dimension of blobs, possibly be blob type or access tier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[doc = "Display name of dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "File path availability request content - availability is based on the name and the subnetId."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilePathAvailabilityRequest {
    #[doc = "File path to verify."]
    pub name: String,
    #[doc = "The Azure Resource URI for a delegated subnet. Must have the delegation Microsoft.NetApp/volumes"]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
}
impl FilePathAvailabilityRequest {
    pub fn new(name: String, subnet_id: String) -> Self {
        Self { name, subnet_id }
    }
}
#[doc = "Log Definition of a single resource metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of log specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of log specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric specification of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Name of metric specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of metric specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Display description of metric specification."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Unit could be Bytes or Count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Support metric aggregation type."]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "The supported time grain types for the metrics."]
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "The internal metric name."]
    #[serde(rename = "internalMetricName", default, skip_serializing_if = "Option::is_none")]
    pub internal_metric_name: Option<String>,
    #[doc = "Whether or not the service is using regional MDM accounts."]
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<bool>,
    #[doc = "The source MDM account."]
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[doc = "The source MDM namespace."]
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[doc = "Dimensions of blobs, including blob type and access tier."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
    #[doc = "Aggregation type could be Average."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "The property to decide fill gap with zero or not."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "The category this metric specification belong to, could be Capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Account Resource Id."]
    #[serde(rename = "resourceIdDimensionNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
    #[doc = "Whether the metric is internal."]
    #[serde(rename = "isInternal", default, skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Microsoft.NetApp REST API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The origin of operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Properties of operation, include metric specifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft NetApp."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Operation description."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Cloud Volume operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Storage operations supported by the Storage resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of operation, include metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "One property of operation, include metric specifications."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Quota availability request content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaAvailabilityRequest {
    #[doc = "Name of the resource to verify."]
    pub name: String,
    #[doc = "Resource type used for verification."]
    #[serde(rename = "type")]
    pub type_: quota_availability_request::Type,
    #[doc = "Resource group name."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
}
impl QuotaAvailabilityRequest {
    pub fn new(name: String, type_: quota_availability_request::Type, resource_group: String) -> Self {
        Self {
            name,
            type_,
            resource_group,
        }
    }
}
pub mod quota_availability_request {
    use super::*;
    #[doc = "Resource type used for verification."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "Microsoft.NetApp/netAppAccounts")]
        MicrosoftNetAppNetAppAccounts,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools")]
        MicrosoftNetAppNetAppAccountsCapacityPools,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumes,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes/snapshots")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumesSnapshots,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftNetAppNetAppAccounts => serializer.serialize_unit_variant("Type", 0u32, "Microsoft.NetApp/netAppAccounts"),
                Self::MicrosoftNetAppNetAppAccountsCapacityPools => {
                    serializer.serialize_unit_variant("Type", 1u32, "Microsoft.NetApp/netAppAccounts/capacityPools")
                }
                Self::MicrosoftNetAppNetAppAccountsCapacityPoolsVolumes => {
                    serializer.serialize_unit_variant("Type", 2u32, "Microsoft.NetApp/netAppAccounts/capacityPools/volumes")
                }
                Self::MicrosoftNetAppNetAppAccountsCapacityPoolsVolumesSnapshots => {
                    serializer.serialize_unit_variant("Type", 3u32, "Microsoft.NetApp/netAppAccounts/capacityPools/volumes/snapshots")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource name availability request content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNameAvailabilityRequest {
    #[doc = "Resource name to verify."]
    pub name: String,
    #[doc = "Resource type used for verification."]
    #[serde(rename = "type")]
    pub type_: resource_name_availability_request::Type,
    #[doc = "Resource group name."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
}
impl ResourceNameAvailabilityRequest {
    pub fn new(name: String, type_: resource_name_availability_request::Type, resource_group: String) -> Self {
        Self {
            name,
            type_,
            resource_group,
        }
    }
}
pub mod resource_name_availability_request {
    use super::*;
    #[doc = "Resource type used for verification."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "Microsoft.NetApp/netAppAccounts")]
        MicrosoftNetAppNetAppAccounts,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools")]
        MicrosoftNetAppNetAppAccountsCapacityPools,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumes,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes/snapshots")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumesSnapshots,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftNetAppNetAppAccounts => serializer.serialize_unit_variant("Type", 0u32, "Microsoft.NetApp/netAppAccounts"),
                Self::MicrosoftNetAppNetAppAccountsCapacityPools => {
                    serializer.serialize_unit_variant("Type", 1u32, "Microsoft.NetApp/netAppAccounts/capacityPools")
                }
                Self::MicrosoftNetAppNetAppAccountsCapacityPoolsVolumes => {
                    serializer.serialize_unit_variant("Type", 2u32, "Microsoft.NetApp/netAppAccounts/capacityPools/volumes")
                }
                Self::MicrosoftNetAppNetAppAccountsCapacityPoolsVolumesSnapshots => {
                    serializer.serialize_unit_variant("Type", 3u32, "Microsoft.NetApp/netAppAccounts/capacityPools/volumes/snapshots")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "One property of operation, include metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Metric specifications of operation."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
    #[doc = "Log specification of operation."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information regarding Subscription Quota Item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionQuotaItem {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "SubscriptionQuotaItem Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionQuotaItemProperties>,
}
impl SubscriptionQuotaItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Subscription Quota Items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionQuotaItemList {
    #[doc = "A list of SubscriptionQuotaItems"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionQuotaItem>,
}
impl azure_core::Continuable for SubscriptionQuotaItemList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SubscriptionQuotaItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SubscriptionQuotaItem Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionQuotaItemProperties {
    #[doc = "The current quota value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<i32>,
    #[doc = "The default quota value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
}
impl SubscriptionQuotaItemProperties {
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
#[doc = "Encryption settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountEncryption {
    #[doc = "Encryption Key Source. Possible values are: 'Microsoft.NetApp'."]
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<String>,
}
impl AccountEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetApp account properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountProperties {
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Active Directories"]
    #[serde(rename = "activeDirectories", default, skip_serializing_if = "Vec::is_empty")]
    pub active_directories: Vec<ActiveDirectory>,
    #[doc = "Encryption settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<AccountEncryption>,
}
impl AccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectory {
    #[doc = "Id of the Active Directory"]
    #[serde(rename = "activeDirectoryId", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    #[doc = "Username of Active Directory domain administrator"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Plain text password of Active Directory domain administrator, value is masked in the response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Name of the Active Directory domain"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "Comma separated list of DNS server IP addresses (IPv4 only) for the Active Directory domain"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,
    #[doc = "Status of the Active Directory"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<active_directory::Status>,
    #[doc = "Any details in regards to the Status of the Active Directory"]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "NetBIOS name of the SMB server. This name will be registered as a computer account in the AD and used to mount volumes"]
    #[serde(rename = "smbServerName", default, skip_serializing_if = "Option::is_none")]
    pub smb_server_name: Option<String>,
    #[doc = "The Organizational Unit (OU) within the Windows Active Directory"]
    #[serde(rename = "organizationalUnit", default, skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
    #[doc = "The Active Directory site the service will limit Domain Controller discovery to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[doc = "Users to be added to the Built-in Backup Operator active directory group. A list of unique usernames without domain specifier"]
    #[serde(rename = "backupOperators", default, skip_serializing_if = "Vec::is_empty")]
    pub backup_operators: Vec<String>,
    #[doc = "Users to be added to the Built-in Administrators active directory group. A list of unique usernames without domain specifier"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub administrators: Vec<String>,
    #[doc = "kdc server IP addresses for the active directory machine. This optional parameter is used only while creating kerberos volume."]
    #[serde(rename = "kdcIP", default, skip_serializing_if = "Option::is_none")]
    pub kdc_ip: Option<String>,
    #[doc = "Name of the active directory machine. This optional parameter is used only while creating kerberos volume"]
    #[serde(rename = "adName", default, skip_serializing_if = "Option::is_none")]
    pub ad_name: Option<String>,
    #[doc = "When LDAP over SSL/TLS is enabled, the LDAP client is required to have base64 encoded Active Directory Certificate Service's self-signed root CA certificate, this optional parameter is used only for dual protocol with LDAP user-mapping volumes."]
    #[serde(rename = "serverRootCACertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_root_ca_certificate: Option<String>,
    #[doc = "If enabled, AES encryption will be enabled for SMB communication."]
    #[serde(rename = "aesEncryption", default, skip_serializing_if = "Option::is_none")]
    pub aes_encryption: Option<bool>,
    #[doc = "Specifies whether or not the LDAP traffic needs to be signed."]
    #[serde(rename = "ldapSigning", default, skip_serializing_if = "Option::is_none")]
    pub ldap_signing: Option<bool>,
    #[doc = "Domain Users in the Active directory to be given SeSecurityPrivilege privilege (Needed for SMB Continuously available shares for SQL). A list of unique usernames without domain specifier"]
    #[serde(rename = "securityOperators", default, skip_serializing_if = "Vec::is_empty")]
    pub security_operators: Vec<String>,
    #[doc = "Specifies whether or not the LDAP traffic needs to be secured via TLS."]
    #[serde(rename = "ldapOverTLS", default, skip_serializing_if = "Option::is_none")]
    pub ldap_over_tls: Option<bool>,
    #[doc = " If enabled, NFS client local users can also (in addition to LDAP users) access the NFS volumes."]
    #[serde(rename = "allowLocalNfsUsersWithLdap", default, skip_serializing_if = "Option::is_none")]
    pub allow_local_nfs_users_with_ldap: Option<bool>,
    #[doc = "If enabled, Traffic between the SMB server to Domain Controller (DC) will be encrypted."]
    #[serde(rename = "encryptDCConnections", default, skip_serializing_if = "Option::is_none")]
    pub encrypt_dc_connections: Option<bool>,
    #[doc = "LDAP search scope "]
    #[serde(rename = "ldapSearchScope", default, skip_serializing_if = "Option::is_none")]
    pub ldap_search_scope: Option<LdapSearchScopeOpt>,
}
impl ActiveDirectory {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod active_directory {
    use super::*;
    #[doc = "Status of the Active Directory"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Created,
        Updating,
        InUse,
        Deleted,
        Error,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Created => serializer.serialize_unit_variant("Status", 0u32, "Created"),
                Self::Updating => serializer.serialize_unit_variant("Status", 1u32, "Updating"),
                Self::InUse => serializer.serialize_unit_variant("Status", 2u32, "InUse"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 3u32, "Deleted"),
                Self::Error => serializer.serialize_unit_variant("Status", 4u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Authorize request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizeRequest {
    #[doc = "Resource id of the remote volume"]
    #[serde(rename = "remoteVolumeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub remote_volume_resource_id: Option<String>,
}
impl AuthorizeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backup of a Volume"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Backup {
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Backup properties"]
    pub properties: BackupProperties,
}
impl Backup {
    pub fn new(location: String, properties: BackupProperties) -> Self {
        Self {
            location,
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "Backup patch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupPatch {
    #[doc = "Tags are a list of key-value pairs that describe the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "Backup properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupProperties>,
}
impl BackupPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Backup Policies"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupPoliciesList {
    #[doc = "A list of backup policies"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackupPolicy>,
}
impl azure_core::Continuable for BackupPoliciesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BackupPoliciesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backup policy information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicy {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Backup policy properties"]
    pub properties: BackupPolicyProperties,
}
impl BackupPolicy {
    pub fn new(tracked_resource: TrackedResource, properties: BackupPolicyProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties,
        }
    }
}
#[doc = "Backup policy properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupPolicyDetails {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Tags are a list of key-value pairs that describe the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "Backup policy properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupPolicyProperties>,
}
impl BackupPolicyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backup policy Details for create and update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupPolicyPatch {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Tags are a list of key-value pairs that describe the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "Backup policy properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupPolicyProperties>,
}
impl BackupPolicyPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backup policy properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupPolicyProperties {
    #[doc = "Backup Policy Resource ID"]
    #[serde(rename = "backupPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy_id: Option<String>,
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Daily backups count to keep"]
    #[serde(rename = "dailyBackupsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub daily_backups_to_keep: Option<i32>,
    #[doc = "Weekly backups count to keep"]
    #[serde(rename = "weeklyBackupsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub weekly_backups_to_keep: Option<i32>,
    #[doc = "Monthly backups count to keep"]
    #[serde(rename = "monthlyBackupsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub monthly_backups_to_keep: Option<i32>,
    #[doc = "Volumes using current backup policy"]
    #[serde(rename = "volumesAssigned", default, skip_serializing_if = "Option::is_none")]
    pub volumes_assigned: Option<i32>,
    #[doc = "The property to decide policy is enabled or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "A list of volumes assigned to this policy"]
    #[serde(rename = "volumeBackups", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_backups: Vec<VolumeBackups>,
}
impl BackupPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backup properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupProperties {
    #[doc = "UUID v4 used to identify the Backup"]
    #[serde(rename = "backupId", default, skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    #[doc = "The creation date of the backup"]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Size of backup"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "Label for backup"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Type of backup Manual or Scheduled"]
    #[serde(rename = "backupType", default, skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<backup_properties::BackupType>,
    #[doc = "Failure reason"]
    #[serde(rename = "failureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[doc = "Volume name"]
    #[serde(rename = "volumeName", default, skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
    #[doc = "Manual backup an already existing snapshot. This will always be false for scheduled backups and true/false for manual backups"]
    #[serde(rename = "useExistingSnapshot", default, skip_serializing_if = "Option::is_none")]
    pub use_existing_snapshot: Option<bool>,
}
impl BackupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_properties {
    use super::*;
    #[doc = "Type of backup Manual or Scheduled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupType")]
    pub enum BackupType {
        Manual,
        Scheduled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Manual => serializer.serialize_unit_variant("BackupType", 0u32, "Manual"),
                Self::Scheduled => serializer.serialize_unit_variant("BackupType", 1u32, "Scheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Backup status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupStatus {
    #[doc = "Backup health status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[doc = "Status of the backup mirror relationship"]
    #[serde(rename = "relationshipStatus", default, skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<backup_status::RelationshipStatus>,
    #[doc = "The status of the backup"]
    #[serde(rename = "mirrorState", default, skip_serializing_if = "Option::is_none")]
    pub mirror_state: Option<backup_status::MirrorState>,
    #[doc = "Reason for the unhealthy backup relationship"]
    #[serde(rename = "unhealthyReason", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_reason: Option<String>,
    #[doc = "Displays error message if the backup is in an error state"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Displays the last transfer size"]
    #[serde(rename = "lastTransferSize", default, skip_serializing_if = "Option::is_none")]
    pub last_transfer_size: Option<i64>,
    #[doc = "Displays the last transfer type"]
    #[serde(rename = "lastTransferType", default, skip_serializing_if = "Option::is_none")]
    pub last_transfer_type: Option<String>,
    #[doc = "Displays the total bytes transferred"]
    #[serde(rename = "totalTransferBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_transfer_bytes: Option<i64>,
}
impl BackupStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_status {
    use super::*;
    #[doc = "Status of the backup mirror relationship"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RelationshipStatus")]
    pub enum RelationshipStatus {
        Idle,
        Transferring,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RelationshipStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RelationshipStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RelationshipStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Idle => serializer.serialize_unit_variant("RelationshipStatus", 0u32, "Idle"),
                Self::Transferring => serializer.serialize_unit_variant("RelationshipStatus", 1u32, "Transferring"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the backup"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MirrorState")]
    pub enum MirrorState {
        Uninitialized,
        Mirrored,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MirrorState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MirrorState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MirrorState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Uninitialized => serializer.serialize_unit_variant("MirrorState", 0u32, "Uninitialized"),
                Self::Mirrored => serializer.serialize_unit_variant("MirrorState", 1u32, "Mirrored"),
                Self::Broken => serializer.serialize_unit_variant("MirrorState", 2u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of Backups"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupsList {
    #[doc = "A list of Backups"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Backup>,
}
impl azure_core::Continuable for BackupsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BackupsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Break replication request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BreakReplicationRequest {
    #[doc = "If replication is in status transferring and you want to force break the replication, set to true"]
    #[serde(rename = "forceBreakReplication", default, skip_serializing_if = "Option::is_none")]
    pub force_break_replication: Option<bool>,
}
impl BreakReplicationRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Capacity pool resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Pool properties"]
    pub properties: PoolProperties,
}
impl CapacityPool {
    pub fn new(tracked_resource: TrackedResource, properties: PoolProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties,
        }
    }
}
#[doc = "List of capacity pool resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityPoolList {
    #[doc = "List of Capacity pools"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CapacityPool>,
    #[doc = "URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CapacityPoolList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CapacityPoolList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Capacity pool patch resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityPoolPatch {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Tags are a list of key-value pairs that describe the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "Patchable pool properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolPatchProperties>,
}
impl CapacityPoolPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Daily Schedule properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DailySchedule {
    #[doc = "Daily snapshot count to keep"]
    #[serde(rename = "snapshotsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub snapshots_to_keep: Option<i32>,
    #[doc = "Indicates which hour in UTC timezone a snapshot should be taken"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[doc = "Indicates which minute snapshot should be taken"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
    #[doc = "Resource size in bytes, current storage usage for the volume in bytes"]
    #[serde(rename = "usedBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_bytes: Option<i64>,
}
impl DailySchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume Export Policy Rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportPolicyRule {
    #[doc = "Order index"]
    #[serde(rename = "ruleIndex", default, skip_serializing_if = "Option::is_none")]
    pub rule_index: Option<i32>,
    #[doc = "Read only access"]
    #[serde(rename = "unixReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub unix_read_only: Option<bool>,
    #[doc = "Read and write access"]
    #[serde(rename = "unixReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub unix_read_write: Option<bool>,
    #[doc = "Kerberos5 Read only access. To be use with swagger version 2020-05-01 or later"]
    #[serde(rename = "kerberos5ReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5_read_only: Option<bool>,
    #[doc = "Kerberos5 Read and write access. To be use with swagger version 2020-05-01 or later"]
    #[serde(rename = "kerberos5ReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5_read_write: Option<bool>,
    #[doc = "Kerberos5i Read only access. To be use with swagger version 2020-05-01 or later"]
    #[serde(rename = "kerberos5iReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5i_read_only: Option<bool>,
    #[doc = "Kerberos5i Read and write access. To be use with swagger version 2020-05-01 or later"]
    #[serde(rename = "kerberos5iReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5i_read_write: Option<bool>,
    #[doc = "Kerberos5p Read only access. To be use with swagger version 2020-05-01 or later"]
    #[serde(rename = "kerberos5pReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5p_read_only: Option<bool>,
    #[doc = "Kerberos5p Read and write access. To be use with swagger version 2020-05-01 or later"]
    #[serde(rename = "kerberos5pReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5p_read_write: Option<bool>,
    #[doc = "Allows CIFS protocol"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cifs: Option<bool>,
    #[doc = "Allows NFSv3 protocol. Enable only for NFSv3 type volumes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfsv3: Option<bool>,
    #[doc = "Allows NFSv4.1 protocol. Enable only for NFSv4.1 type volumes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfsv41: Option<bool>,
    #[doc = "Client ingress specification as comma separated string with IPv4 CIDRs, IPv4 host addresses and host names"]
    #[serde(rename = "allowedClients", default, skip_serializing_if = "Option::is_none")]
    pub allowed_clients: Option<String>,
    #[doc = "Has root access to volume"]
    #[serde(rename = "hasRootAccess", default, skip_serializing_if = "Option::is_none")]
    pub has_root_access: Option<bool>,
    #[doc = "This parameter specifies who is authorized to change the ownership of a file. restricted - Only root user can change the ownership of the file. unrestricted - Non-root users can change ownership of files that they own."]
    #[serde(rename = "chownMode", default, skip_serializing_if = "Option::is_none")]
    pub chown_mode: Option<export_policy_rule::ChownMode>,
}
impl ExportPolicyRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod export_policy_rule {
    use super::*;
    #[doc = "This parameter specifies who is authorized to change the ownership of a file. restricted - Only root user can change the ownership of the file. unrestricted - Non-root users can change ownership of files that they own."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ChownMode")]
    pub enum ChownMode {
        Restricted,
        Unrestricted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ChownMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ChownMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ChownMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Restricted => serializer.serialize_unit_variant("ChownMode", 0u32, "Restricted"),
                Self::Unrestricted => serializer.serialize_unit_variant("ChownMode", 1u32, "Unrestricted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ChownMode {
        fn default() -> Self {
            Self::Restricted
        }
    }
}
#[doc = "Hourly Schedule properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HourlySchedule {
    #[doc = "Hourly snapshot count to keep"]
    #[serde(rename = "snapshotsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub snapshots_to_keep: Option<i32>,
    #[doc = "Indicates which minute snapshot should be taken"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
    #[doc = "Resource size in bytes, current storage usage for the volume in bytes"]
    #[serde(rename = "usedBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_bytes: Option<i64>,
}
impl HourlySchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "LDAP search scope "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LdapSearchScopeOpt {
    #[doc = "This specifies the user DN, which overrides the base DN for user lookups."]
    #[serde(rename = "userDN", default, skip_serializing_if = "Option::is_none")]
    pub user_dn: Option<String>,
    #[doc = "This specifies the group DN, which overrides the base DN for group lookups."]
    #[serde(rename = "groupDN", default, skip_serializing_if = "Option::is_none")]
    pub group_dn: Option<String>,
    #[doc = "This specifies the custom LDAP search filter to be used when looking up group membership from LDAP server."]
    #[serde(rename = "groupMembershipFilter", default, skip_serializing_if = "Option::is_none")]
    pub group_membership_filter: Option<String>,
}
impl LdapSearchScopeOpt {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List Replications"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListReplications {
    #[doc = "A list of replications"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Replication>,
}
impl azure_core::Continuable for ListReplications {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListReplications {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Monthly Schedule properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonthlySchedule {
    #[doc = "Monthly snapshot count to keep"]
    #[serde(rename = "snapshotsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub snapshots_to_keep: Option<i32>,
    #[doc = "Indicates which days of the month snapshot should be taken. A comma delimited string."]
    #[serde(rename = "daysOfMonth", default, skip_serializing_if = "Option::is_none")]
    pub days_of_month: Option<String>,
    #[doc = "Indicates which hour in UTC timezone a snapshot should be taken"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[doc = "Indicates which minute snapshot should be taken"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
    #[doc = "Resource size in bytes, current storage usage for the volume in bytes"]
    #[serde(rename = "usedBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_bytes: Option<i64>,
}
impl MonthlySchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mount Target"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountTarget {
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Tags are a list of key-value pairs that describe the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "Mount target properties"]
    pub properties: MountTargetProperties,
}
impl MountTarget {
    pub fn new(location: String, properties: MountTargetProperties) -> Self {
        Self {
            location,
            id: None,
            name: None,
            type_: None,
            tags: None,
            properties,
        }
    }
}
#[doc = "Mount target properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountTargetProperties {
    #[doc = "UUID v4 used to identify the MountTarget"]
    #[serde(rename = "mountTargetId", default, skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
    #[doc = "UUID v4 used to identify the MountTarget"]
    #[serde(rename = "fileSystemId")]
    pub file_system_id: String,
    #[doc = "The mount target's IPv4 address"]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The SMB server's Fully Qualified Domain Name, FQDN"]
    #[serde(rename = "smbServerFqdn", default, skip_serializing_if = "Option::is_none")]
    pub smb_server_fqdn: Option<String>,
}
impl MountTargetProperties {
    pub fn new(file_system_id: String) -> Self {
        Self {
            mount_target_id: None,
            file_system_id,
            ip_address: None,
            smb_server_fqdn: None,
        }
    }
}
#[doc = "NetApp account resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetAppAccount {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "NetApp account properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
impl NetAppAccount {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties: None,
        }
    }
}
#[doc = "List of NetApp account resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetAppAccountList {
    #[doc = "Multiple NetApp accounts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetAppAccount>,
    #[doc = "URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetAppAccountList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetAppAccountList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NetApp account patch resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetAppAccountPatch {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Tags are a list of key-value pairs that describe the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "NetApp account properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
impl NetAppAccountPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets the status of the VolumeQuotaRule at the time the operation was called."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetappProvisioningState {
    Accepted,
    Creating,
    Patching,
    Deleting,
    Moving,
    Failed,
    Succeeded,
}
#[doc = "Application specific parameters for the placement of volumes in the volume group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlacementKeyValuePairs {
    #[doc = "Key for an application specific parameter for the placement of volumes in the volume group"]
    pub key: String,
    #[doc = "Value for an application specific parameter for the placement of volumes in the volume group"]
    pub value: String,
}
impl PlacementKeyValuePairs {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
#[doc = "Pool change request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolChangeRequest {
    #[doc = "Resource id of the pool to move volume to"]
    #[serde(rename = "newPoolResourceId")]
    pub new_pool_resource_id: String,
}
impl PoolChangeRequest {
    pub fn new(new_pool_resource_id: String) -> Self {
        Self { new_pool_resource_id }
    }
}
#[doc = "Patchable pool properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolPatchProperties {
    #[doc = "Provisioned size of the pool (in bytes). Allowed values are in 1TiB chunks (value must be multiply of 4398046511104)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "The qos type of the pool"]
    #[serde(rename = "qosType", default, skip_serializing_if = "Option::is_none")]
    pub qos_type: Option<pool_patch_properties::QosType>,
}
impl PoolPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pool_patch_properties {
    use super::*;
    #[doc = "The qos type of the pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "QosType")]
    pub enum QosType {
        Auto,
        Manual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for QosType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for QosType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for QosType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Auto => serializer.serialize_unit_variant("QosType", 0u32, "Auto"),
                Self::Manual => serializer.serialize_unit_variant("QosType", 1u32, "Manual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for QosType {
        fn default() -> Self {
            Self::Auto
        }
    }
}
#[doc = "Pool properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolProperties {
    #[doc = "UUID v4 used to identify the Pool"]
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[doc = "Provisioned size of the pool (in bytes). Allowed values are in 1TiB chunks (value must be multiply of 4398046511104)."]
    pub size: i64,
    #[doc = "The service level of the file system"]
    #[serde(rename = "serviceLevel")]
    pub service_level: ServiceLevel,
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Total throughput of pool in Mibps"]
    #[serde(rename = "totalThroughputMibps", default, skip_serializing_if = "Option::is_none")]
    pub total_throughput_mibps: Option<f64>,
    #[doc = "Utilized throughput of pool in Mibps"]
    #[serde(rename = "utilizedThroughputMibps", default, skip_serializing_if = "Option::is_none")]
    pub utilized_throughput_mibps: Option<f64>,
    #[doc = "The qos type of the pool"]
    #[serde(rename = "qosType", default, skip_serializing_if = "Option::is_none")]
    pub qos_type: Option<pool_properties::QosType>,
    #[doc = "If enabled (true) the pool can contain cool Access enabled volumes."]
    #[serde(rename = "coolAccess", default, skip_serializing_if = "Option::is_none")]
    pub cool_access: Option<bool>,
    #[doc = "Encryption type of the capacity pool, set encryption type for data at rest for this pool and all volumes in it. This value can only be set when creating new pool."]
    #[serde(rename = "encryptionType", default, skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<pool_properties::EncryptionType>,
}
impl PoolProperties {
    pub fn new(size: i64, service_level: ServiceLevel) -> Self {
        Self {
            pool_id: None,
            size,
            service_level,
            provisioning_state: None,
            total_throughput_mibps: None,
            utilized_throughput_mibps: None,
            qos_type: None,
            cool_access: None,
            encryption_type: None,
        }
    }
}
pub mod pool_properties {
    use super::*;
    #[doc = "The qos type of the pool"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "QosType")]
    pub enum QosType {
        Auto,
        Manual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for QosType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for QosType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for QosType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Auto => serializer.serialize_unit_variant("QosType", 0u32, "Auto"),
                Self::Manual => serializer.serialize_unit_variant("QosType", 1u32, "Manual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for QosType {
        fn default() -> Self {
            Self::Auto
        }
    }
    #[doc = "Encryption type of the capacity pool, set encryption type for data at rest for this pool and all volumes in it. This value can only be set when creating new pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EncryptionType")]
    pub enum EncryptionType {
        Single,
        Double,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EncryptionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EncryptionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EncryptionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Single => serializer.serialize_unit_variant("EncryptionType", 0u32, "Single"),
                Self::Double => serializer.serialize_unit_variant("EncryptionType", 1u32, "Double"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for EncryptionType {
        fn default() -> Self {
            Self::Single
        }
    }
}
#[doc = "Replication properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Replication {
    #[doc = "Indicates whether the local volume is the source or destination for the Volume Replication"]
    #[serde(rename = "endpointType", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<replication::EndpointType>,
    #[doc = "Schedule"]
    #[serde(rename = "replicationSchedule", default, skip_serializing_if = "Option::is_none")]
    pub replication_schedule: Option<replication::ReplicationSchedule>,
    #[doc = "The resource ID of the remote volume."]
    #[serde(rename = "remoteVolumeResourceId")]
    pub remote_volume_resource_id: String,
    #[doc = "The remote region for the other end of the Volume Replication."]
    #[serde(rename = "remoteVolumeRegion", default, skip_serializing_if = "Option::is_none")]
    pub remote_volume_region: Option<String>,
}
impl Replication {
    pub fn new(remote_volume_resource_id: String) -> Self {
        Self {
            endpoint_type: None,
            replication_schedule: None,
            remote_volume_resource_id,
            remote_volume_region: None,
        }
    }
}
pub mod replication {
    use super::*;
    #[doc = "Indicates whether the local volume is the source or destination for the Volume Replication"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EndpointType")]
    pub enum EndpointType {
        #[serde(rename = "src")]
        Src,
        #[serde(rename = "dst")]
        Dst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EndpointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EndpointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EndpointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Src => serializer.serialize_unit_variant("EndpointType", 0u32, "src"),
                Self::Dst => serializer.serialize_unit_variant("EndpointType", 1u32, "dst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationSchedule")]
    pub enum ReplicationSchedule {
        #[serde(rename = "_10minutely")]
        N10minutely,
        #[serde(rename = "hourly")]
        Hourly,
        #[serde(rename = "daily")]
        Daily,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationSchedule {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationSchedule {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationSchedule {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N10minutely => serializer.serialize_unit_variant("ReplicationSchedule", 0u32, "_10minutely"),
                Self::Hourly => serializer.serialize_unit_variant("ReplicationSchedule", 1u32, "hourly"),
                Self::Daily => serializer.serialize_unit_variant("ReplicationSchedule", 2u32, "daily"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Replication properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationObject {
    #[doc = "Id"]
    #[serde(rename = "replicationId", default, skip_serializing_if = "Option::is_none")]
    pub replication_id: Option<String>,
    #[doc = "Indicates whether the local volume is the source or destination for the Volume Replication"]
    #[serde(rename = "endpointType", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<replication_object::EndpointType>,
    #[doc = "Schedule"]
    #[serde(rename = "replicationSchedule", default, skip_serializing_if = "Option::is_none")]
    pub replication_schedule: Option<replication_object::ReplicationSchedule>,
    #[doc = "The resource ID of the remote volume."]
    #[serde(rename = "remoteVolumeResourceId")]
    pub remote_volume_resource_id: String,
    #[doc = "The remote region for the other end of the Volume Replication."]
    #[serde(rename = "remoteVolumeRegion", default, skip_serializing_if = "Option::is_none")]
    pub remote_volume_region: Option<String>,
}
impl ReplicationObject {
    pub fn new(remote_volume_resource_id: String) -> Self {
        Self {
            replication_id: None,
            endpoint_type: None,
            replication_schedule: None,
            remote_volume_resource_id,
            remote_volume_region: None,
        }
    }
}
pub mod replication_object {
    use super::*;
    #[doc = "Indicates whether the local volume is the source or destination for the Volume Replication"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EndpointType")]
    pub enum EndpointType {
        #[serde(rename = "src")]
        Src,
        #[serde(rename = "dst")]
        Dst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EndpointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EndpointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EndpointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Src => serializer.serialize_unit_variant("EndpointType", 0u32, "src"),
                Self::Dst => serializer.serialize_unit_variant("EndpointType", 1u32, "dst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationSchedule")]
    pub enum ReplicationSchedule {
        #[serde(rename = "_10minutely")]
        N10minutely,
        #[serde(rename = "hourly")]
        Hourly,
        #[serde(rename = "daily")]
        Daily,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationSchedule {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationSchedule {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationSchedule {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N10minutely => serializer.serialize_unit_variant("ReplicationSchedule", 0u32, "_10minutely"),
                Self::Hourly => serializer.serialize_unit_variant("ReplicationSchedule", 1u32, "hourly"),
                Self::Daily => serializer.serialize_unit_variant("ReplicationSchedule", 2u32, "daily"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Replication status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationStatus {
    #[doc = "Replication health check"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[doc = "Status of the mirror relationship"]
    #[serde(rename = "relationshipStatus", default, skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<replication_status::RelationshipStatus>,
    #[doc = "The status of the replication"]
    #[serde(rename = "mirrorState", default, skip_serializing_if = "Option::is_none")]
    pub mirror_state: Option<replication_status::MirrorState>,
    #[doc = "The progress of the replication"]
    #[serde(rename = "totalProgress", default, skip_serializing_if = "Option::is_none")]
    pub total_progress: Option<String>,
    #[doc = "Displays error message if the replication is in an error state"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl ReplicationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod replication_status {
    use super::*;
    #[doc = "Status of the mirror relationship"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RelationshipStatus")]
    pub enum RelationshipStatus {
        Idle,
        Transferring,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RelationshipStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RelationshipStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RelationshipStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Idle => serializer.serialize_unit_variant("RelationshipStatus", 0u32, "Idle"),
                Self::Transferring => serializer.serialize_unit_variant("RelationshipStatus", 1u32, "Transferring"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the replication"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MirrorState")]
    pub enum MirrorState {
        Uninitialized,
        Mirrored,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MirrorState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MirrorState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MirrorState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Uninitialized => serializer.serialize_unit_variant("MirrorState", 0u32, "Uninitialized"),
                Self::Mirrored => serializer.serialize_unit_variant("MirrorState", 1u32, "Mirrored"),
                Self::Broken => serializer.serialize_unit_variant("MirrorState", 2u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceIdentity {
    #[doc = "Object id of the identity resource"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of the resource"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of Identity. Supported values are: 'None', 'SystemAssigned'"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags are a list of key-value pairs that describe the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTags {}
impl ResourceTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Restore status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreStatus {
    #[doc = "Restore health status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[doc = "Status of the restore SnapMirror relationship"]
    #[serde(rename = "relationshipStatus", default, skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<restore_status::RelationshipStatus>,
    #[doc = "The status of the restore"]
    #[serde(rename = "mirrorState", default, skip_serializing_if = "Option::is_none")]
    pub mirror_state: Option<restore_status::MirrorState>,
    #[doc = "Reason for the unhealthy restore relationship"]
    #[serde(rename = "unhealthyReason", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_reason: Option<String>,
    #[doc = "Displays error message if the restore is in an error state"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Displays the total bytes transferred"]
    #[serde(rename = "totalTransferBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_transfer_bytes: Option<i64>,
}
impl RestoreStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restore_status {
    use super::*;
    #[doc = "Status of the restore SnapMirror relationship"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RelationshipStatus")]
    pub enum RelationshipStatus {
        Idle,
        Transferring,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RelationshipStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RelationshipStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RelationshipStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Idle => serializer.serialize_unit_variant("RelationshipStatus", 0u32, "Idle"),
                Self::Transferring => serializer.serialize_unit_variant("RelationshipStatus", 1u32, "Transferring"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the restore"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MirrorState")]
    pub enum MirrorState {
        Uninitialized,
        Mirrored,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MirrorState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MirrorState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MirrorState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Uninitialized => serializer.serialize_unit_variant("MirrorState", 0u32, "Uninitialized"),
                Self::Mirrored => serializer.serialize_unit_variant("MirrorState", 1u32, "Mirrored"),
                Self::Broken => serializer.serialize_unit_variant("MirrorState", 2u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The service level of the file system"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceLevel")]
pub enum ServiceLevel {
    Standard,
    Premium,
    Ultra,
    #[serde(rename = "StandardZRS")]
    StandardZrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceLevel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Standard => serializer.serialize_unit_variant("ServiceLevel", 0u32, "Standard"),
            Self::Premium => serializer.serialize_unit_variant("ServiceLevel", 1u32, "Premium"),
            Self::Ultra => serializer.serialize_unit_variant("ServiceLevel", 2u32, "Ultra"),
            Self::StandardZrs => serializer.serialize_unit_variant("ServiceLevel", 3u32, "StandardZRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ServiceLevel {
    fn default() -> Self {
        Self::Premium
    }
}
#[doc = "Snapshot of a Volume"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Snapshot properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
impl Snapshot {
    pub fn new(location: String) -> Self {
        Self {
            location,
            id: None,
            name: None,
            type_: None,
            properties: None,
        }
    }
}
#[doc = "Snapshot patch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotPatch {}
impl SnapshotPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Snapshot Policies"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotPoliciesList {
    #[doc = "A list of snapshot policies"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SnapshotPolicy>,
}
impl azure_core::Continuable for SnapshotPoliciesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SnapshotPoliciesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Snapshot policy information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotPolicy {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Snapshot policy properties"]
    pub properties: SnapshotPolicyProperties,
}
impl SnapshotPolicy {
    pub fn new(tracked_resource: TrackedResource, properties: SnapshotPolicyProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties,
        }
    }
}
#[doc = "Snapshot policy properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotPolicyDetails {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Tags are a list of key-value pairs that describe the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "Snapshot policy properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotPolicyProperties>,
}
impl SnapshotPolicyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Snapshot policy Details for create and update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotPolicyPatch {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Tags are a list of key-value pairs that describe the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "Snapshot policy properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotPolicyProperties>,
}
impl SnapshotPolicyPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Snapshot policy properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotPolicyProperties {
    #[doc = "Hourly Schedule properties"]
    #[serde(rename = "hourlySchedule", default, skip_serializing_if = "Option::is_none")]
    pub hourly_schedule: Option<HourlySchedule>,
    #[doc = "Daily Schedule properties"]
    #[serde(rename = "dailySchedule", default, skip_serializing_if = "Option::is_none")]
    pub daily_schedule: Option<DailySchedule>,
    #[doc = "Weekly Schedule properties, make a snapshot every week at a specific day or days"]
    #[serde(rename = "weeklySchedule", default, skip_serializing_if = "Option::is_none")]
    pub weekly_schedule: Option<WeeklySchedule>,
    #[doc = "Monthly Schedule properties"]
    #[serde(rename = "monthlySchedule", default, skip_serializing_if = "Option::is_none")]
    pub monthly_schedule: Option<MonthlySchedule>,
    #[doc = "The property to decide policy is enabled or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SnapshotPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volumes associated with snapshot policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotPolicyVolumeList {
    #[doc = "List of volumes"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Volume>,
}
impl SnapshotPolicyVolumeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Snapshot properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotProperties {
    #[doc = "UUID v4 used to identify the Snapshot"]
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[doc = "The creation date of the snapshot"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SnapshotProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Restore payload for Single File Snapshot Restore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotRestoreFiles {
    #[doc = "List of files to be restored"]
    #[serde(rename = "filePaths")]
    pub file_paths: Vec<String>,
    #[doc = "Destination folder where the files will be restored"]
    #[serde(rename = "destinationPath", default, skip_serializing_if = "Option::is_none")]
    pub destination_path: Option<String>,
}
impl SnapshotRestoreFiles {
    pub fn new(file_paths: Vec<String>) -> Self {
        Self {
            file_paths,
            destination_path: None,
        }
    }
}
#[doc = "List of Snapshots"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotsList {
    #[doc = "A list of Snapshots"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Snapshot>,
}
impl azure_core::Continuable for SnapshotsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SnapshotsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subvolume Information properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubvolumeInfo {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "This represents path associated with the subvolume"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubvolumeProperties>,
}
impl SubvolumeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the post subvolume and action is to get metadata of the subvolume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubvolumeModel {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties which represents actual subvolume model which is stored as a file in the system."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubvolumeModelProperties>,
}
impl SubvolumeModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties which represents actual subvolume model which is stored as a file in the system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubvolumeModelProperties {
    #[doc = "Path to the subvolume"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Path to the parent subvolume"]
    #[serde(rename = "parentPath", default, skip_serializing_if = "Option::is_none")]
    pub parent_path: Option<String>,
    #[doc = "Size of subvolume"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "Bytes used"]
    #[serde(rename = "bytesUsed", default, skip_serializing_if = "Option::is_none")]
    pub bytes_used: Option<i64>,
    #[doc = "Permissions of the subvolume"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[doc = "Creation time and date"]
    #[serde(rename = "creationTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "Most recent access time and date"]
    #[serde(rename = "accessedTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub accessed_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "Most recent modification time and date"]
    #[serde(rename = "modifiedTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub modified_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "Most recent change time and date"]
    #[serde(rename = "changedTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub changed_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SubvolumeModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters with which a subvolume can be updated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubvolumePatchParams {
    #[doc = "Truncate subvolume to the provided size in bytes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "path to the subvolume"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl SubvolumePatchParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subvolume Patch Request properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubvolumePatchRequest {
    #[doc = "Parameters with which a subvolume can be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubvolumePatchParams>,
}
impl SubvolumePatchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This represents path associated with the subvolume"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubvolumeProperties {
    #[doc = "Path to the subvolume"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Truncate subvolume to the provided size in bytes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "parent path to the subvolume"]
    #[serde(rename = "parentPath", default, skip_serializing_if = "Option::is_none")]
    pub parent_path: Option<String>,
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SubvolumeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Subvolumes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubvolumesList {
    #[doc = "A list of Subvolumes"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubvolumeInfo>,
    #[doc = "URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SubvolumesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SubvolumesList {
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
#[doc = "Vault information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Vault properties"]
    pub properties: VaultProperties,
}
impl Vault {
    pub fn new(location: String, properties: VaultProperties) -> Self {
        Self {
            location,
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "List of Vaults"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultList {
    #[doc = "A list of vaults"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,
}
impl azure_core::Continuable for VaultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl VaultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vault properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultProperties {
    #[doc = "Vault Name"]
    #[serde(rename = "vaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
}
impl VaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Availability Zone"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Volume properties"]
    pub properties: VolumeProperties,
}
impl Volume {
    pub fn new(tracked_resource: TrackedResource, properties: VolumeProperties) -> Self {
        Self {
            tracked_resource,
            etag: None,
            zones: Vec::new(),
            properties,
        }
    }
}
#[doc = "Volume Backup Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeBackupProperties {
    #[doc = "Backup Policy Resource ID"]
    #[serde(rename = "backupPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy_id: Option<String>,
    #[doc = "Policy Enforced"]
    #[serde(rename = "policyEnforced", default, skip_serializing_if = "Option::is_none")]
    pub policy_enforced: Option<bool>,
    #[doc = "Vault Resource ID"]
    #[serde(rename = "vaultId", default, skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<String>,
    #[doc = "Backup Enabled"]
    #[serde(rename = "backupEnabled", default, skip_serializing_if = "Option::is_none")]
    pub backup_enabled: Option<bool>,
}
impl VolumeBackupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume details using the backup policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeBackups {
    #[doc = "Volume name"]
    #[serde(rename = "volumeName", default, skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
    #[doc = "Total count of backups for volume"]
    #[serde(rename = "backupsCount", default, skip_serializing_if = "Option::is_none")]
    pub backups_count: Option<i32>,
    #[doc = "Policy enabled"]
    #[serde(rename = "policyEnabled", default, skip_serializing_if = "Option::is_none")]
    pub policy_enabled: Option<bool>,
}
impl VolumeBackups {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume group resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeGroup {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Volume group properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeGroupListProperties>,
}
impl VolumeGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume group resource for create"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeGroupDetails {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Volume group properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeGroupProperties>,
}
impl VolumeGroupDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of volume group resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeGroupList {
    #[doc = "List of volume Groups"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VolumeGroup>,
}
impl azure_core::Continuable for VolumeGroupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl VolumeGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume group properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeGroupListProperties {
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Volume group properties"]
    #[serde(rename = "groupMetaData", default, skip_serializing_if = "Option::is_none")]
    pub group_meta_data: Option<VolumeGroupMetaData>,
}
impl VolumeGroupListProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume group properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeGroupMetaData {
    #[doc = "Group Description"]
    #[serde(rename = "groupDescription", default, skip_serializing_if = "Option::is_none")]
    pub group_description: Option<String>,
    #[doc = "Application Type"]
    #[serde(rename = "applicationType", default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<volume_group_meta_data::ApplicationType>,
    #[doc = "Application specific identifier"]
    #[serde(rename = "applicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub application_identifier: Option<String>,
    #[doc = "Application specific placement rules for the volume group"]
    #[serde(rename = "globalPlacementRules", default, skip_serializing_if = "Vec::is_empty")]
    pub global_placement_rules: Vec<PlacementKeyValuePairs>,
    #[doc = "Application specific identifier of deployment rules for the volume group"]
    #[serde(rename = "deploymentSpecId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_spec_id: Option<String>,
    #[doc = "Number of volumes in volume group"]
    #[serde(rename = "volumesCount", default, skip_serializing_if = "Option::is_none")]
    pub volumes_count: Option<i64>,
}
impl VolumeGroupMetaData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod volume_group_meta_data {
    use super::*;
    #[doc = "Application Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApplicationType")]
    pub enum ApplicationType {
        #[serde(rename = "SAP-HANA")]
        SapHana,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApplicationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApplicationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApplicationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SapHana => serializer.serialize_unit_variant("ApplicationType", 0u32, "SAP-HANA"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Volume group properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeGroupProperties {
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Volume group properties"]
    #[serde(rename = "groupMetaData", default, skip_serializing_if = "Option::is_none")]
    pub group_meta_data: Option<VolumeGroupMetaData>,
    #[doc = "List of volumes from group"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<VolumeGroupVolumeProperties>,
}
impl VolumeGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeGroupVolumeProperties {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Tags are a list of key-value pairs that describe the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "Volume properties"]
    pub properties: VolumeProperties,
}
impl VolumeGroupVolumeProperties {
    pub fn new(properties: VolumeProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            tags: None,
            properties,
        }
    }
}
#[doc = "List of volume resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeList {
    #[doc = "List of volumes"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Volume>,
    #[doc = "URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VolumeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VolumeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume patch resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumePatch {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Tags are a list of key-value pairs that describe the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[doc = "Patchable volume properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumePatchProperties>,
}
impl VolumePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patchable volume properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumePatchProperties {
    #[doc = "The service level of the file system"]
    #[serde(rename = "serviceLevel", default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<ServiceLevel>,
    #[doc = "Maximum storage quota allowed for a file system in bytes. This is a soft quota used for alerting only. Minimum size is 100 GiB. Upper limit is 100TiB. Specified in bytes."]
    #[serde(rename = "usageThreshold", default, skip_serializing_if = "Option::is_none")]
    pub usage_threshold: Option<i64>,
    #[doc = "Set of export policy rules"]
    #[serde(rename = "exportPolicy", default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<volume_patch_properties::ExportPolicy>,
    #[serde(rename = "throughputMibps", default, skip_serializing_if = "Option::is_none")]
    pub throughput_mibps: Option<f64>,
    #[doc = "DataProtection type volumes include an object containing details of the replication"]
    #[serde(rename = "dataProtection", default, skip_serializing_if = "Option::is_none")]
    pub data_protection: Option<volume_patch_properties::DataProtection>,
    #[doc = "Specifies if default quota is enabled for the volume."]
    #[serde(rename = "isDefaultQuotaEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_default_quota_enabled: Option<bool>,
    #[doc = "Default user quota for volume in KiBs. If isDefaultQuotaEnabled is set, the minimum value of 4 KiBs applies ."]
    #[serde(rename = "defaultUserQuotaInKiBs", default, skip_serializing_if = "Option::is_none")]
    pub default_user_quota_in_ki_bs: Option<i64>,
    #[doc = "Default group quota for volume in KiBs. If isDefaultQuotaEnabled is set, the minimum value of 4 KiBs applies."]
    #[serde(rename = "defaultGroupQuotaInKiBs", default, skip_serializing_if = "Option::is_none")]
    pub default_group_quota_in_ki_bs: Option<i64>,
    #[doc = "UNIX permissions for NFS volume accepted in octal 4 digit format. First digit selects the set user ID(4), set group ID (2) and sticky (1) attributes. Second digit selects permission for the owner of the file: read (4), write (2) and execute (1). Third selects permissions for other users in the same group. the fourth for other users not in the group. 0755 - gives read/write/execute permissions to owner and read/execute to group and other users."]
    #[serde(rename = "unixPermissions", default, skip_serializing_if = "Option::is_none")]
    pub unix_permissions: Option<String>,
}
impl VolumePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod volume_patch_properties {
    use super::*;
    #[doc = "Set of export policy rules"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExportPolicy {
        #[doc = "Export policy rule"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub rules: Vec<ExportPolicyRule>,
    }
    impl ExportPolicy {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "DataProtection type volumes include an object containing details of the replication"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DataProtection {
        #[doc = "Volume Backup Properties"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub backup: Option<VolumeBackupProperties>,
        #[doc = "Volume Snapshot Properties"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot: Option<VolumeSnapshotProperties>,
    }
    impl DataProtection {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Volume properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[doc = "Unique FileSystem Identifier."]
    #[serde(rename = "fileSystemId", default, skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[doc = "A unique file path for the volume. Used when creating mount targets"]
    #[serde(rename = "creationToken")]
    pub creation_token: String,
    #[doc = "The service level of the file system"]
    #[serde(rename = "serviceLevel", default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<ServiceLevel>,
    #[doc = "Maximum storage quota allowed for a file system in bytes. This is a soft quota used for alerting only. Minimum size is 100 GiB. Upper limit is 100TiB. Specified in bytes."]
    #[serde(rename = "usageThreshold")]
    pub usage_threshold: i64,
    #[doc = "Set of export policy rules"]
    #[serde(rename = "exportPolicy", default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<volume_properties::ExportPolicy>,
    #[doc = "Set of protocol types, default NFSv3, CIFS for SMB protocol"]
    #[serde(rename = "protocolTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_types: Vec<String>,
    #[doc = "Azure lifecycle management"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "UUID v4 or resource identifier used to identify the Snapshot."]
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[doc = "UUID v4 or resource identifier used to identify the Backup."]
    #[serde(rename = "backupId", default, skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    #[doc = "Unique Baremetal Tenant Identifier."]
    #[serde(rename = "baremetalTenantId", default, skip_serializing_if = "Option::is_none")]
    pub baremetal_tenant_id: Option<String>,
    #[doc = "The Azure Resource URI for a delegated subnet. Must have the delegation Microsoft.NetApp/volumes"]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "Basic network, or Standard features available to the volume."]
    #[serde(rename = "networkFeatures", default, skip_serializing_if = "Option::is_none")]
    pub network_features: Option<volume_properties::NetworkFeatures>,
    #[doc = "Network Sibling Set ID for the the group of volumes sharing networking resources."]
    #[serde(rename = "networkSiblingSetId", default, skip_serializing_if = "Option::is_none")]
    pub network_sibling_set_id: Option<String>,
    #[doc = "Provides storage to network proximity information for the volume."]
    #[serde(rename = "storageToNetworkProximity", default, skip_serializing_if = "Option::is_none")]
    pub storage_to_network_proximity: Option<volume_properties::StorageToNetworkProximity>,
    #[doc = "List of mount targets"]
    #[serde(rename = "mountTargets", default, skip_serializing_if = "Vec::is_empty")]
    pub mount_targets: Vec<MountTargetProperties>,
    #[doc = "What type of volume is this. For destination volumes in Cross Region Replication, set type to DataProtection"]
    #[serde(rename = "volumeType", default, skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    #[doc = "DataProtection type volumes include an object containing details of the replication"]
    #[serde(rename = "dataProtection", default, skip_serializing_if = "Option::is_none")]
    pub data_protection: Option<volume_properties::DataProtection>,
    #[doc = "Restoring"]
    #[serde(rename = "isRestoring", default, skip_serializing_if = "Option::is_none")]
    pub is_restoring: Option<bool>,
    #[doc = "If enabled (true) the volume will contain a read-only snapshot directory which provides access to each of the volume's snapshots (default to true)."]
    #[serde(rename = "snapshotDirectoryVisible", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_directory_visible: Option<bool>,
    #[doc = "Describe if a volume is KerberosEnabled. To be use with swagger version 2020-05-01 or later"]
    #[serde(rename = "kerberosEnabled", default, skip_serializing_if = "Option::is_none")]
    pub kerberos_enabled: Option<bool>,
    #[doc = "The security style of volume, default unix, defaults to ntfs for dual protocol or CIFS protocol"]
    #[serde(rename = "securityStyle", default, skip_serializing_if = "Option::is_none")]
    pub security_style: Option<volume_properties::SecurityStyle>,
    #[doc = "Enables encryption for in-flight smb3 data. Only applicable for SMB/DualProtocol volume. To be used with swagger version 2020-08-01 or later"]
    #[serde(rename = "smbEncryption", default, skip_serializing_if = "Option::is_none")]
    pub smb_encryption: Option<bool>,
    #[doc = "Enables continuously available share property for smb volume. Only applicable for SMB volume"]
    #[serde(rename = "smbContinuouslyAvailable", default, skip_serializing_if = "Option::is_none")]
    pub smb_continuously_available: Option<bool>,
    #[serde(rename = "throughputMibps", default, skip_serializing_if = "Option::is_none")]
    pub throughput_mibps: Option<f64>,
    #[doc = "Source of key used to encrypt data in volume. Possible values (case-insensitive) are: 'Microsoft.NetApp'"]
    #[serde(rename = "encryptionKeySource", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_source: Option<volume_properties::EncryptionKeySource>,
    #[doc = "Specifies whether LDAP is enabled or not for a given NFS volume."]
    #[serde(rename = "ldapEnabled", default, skip_serializing_if = "Option::is_none")]
    pub ldap_enabled: Option<bool>,
    #[doc = "Specifies whether Cool Access(tiering) is enabled for the volume."]
    #[serde(rename = "coolAccess", default, skip_serializing_if = "Option::is_none")]
    pub cool_access: Option<bool>,
    #[doc = "Specifies the number of days after which data that is not accessed by clients will be tiered."]
    #[serde(rename = "coolnessPeriod", default, skip_serializing_if = "Option::is_none")]
    pub coolness_period: Option<i32>,
    #[doc = "UNIX permissions for NFS volume accepted in octal 4 digit format. First digit selects the set user ID(4), set group ID (2) and sticky (1) attributes. Second digit selects permission for the owner of the file: read (4), write (2) and execute (1). Third selects permissions for other users in the same group. the fourth for other users not in the group. 0755 - gives read/write/execute permissions to owner and read/execute to group and other users."]
    #[serde(rename = "unixPermissions", default, skip_serializing_if = "Option::is_none")]
    pub unix_permissions: Option<String>,
    #[doc = "When a volume is being restored from another volume's snapshot, will show the percentage completion of this cloning process. When this value is empty/null there is no cloning process currently happening on this volume. This value will update every 5 minutes during cloning."]
    #[serde(rename = "cloneProgress", default, skip_serializing_if = "Option::is_none")]
    pub clone_progress: Option<i32>,
    #[doc = "Specifies whether the volume is enabled for Azure VMware Solution (AVS) datastore purpose"]
    #[serde(rename = "avsDataStore", default, skip_serializing_if = "Option::is_none")]
    pub avs_data_store: Option<volume_properties::AvsDataStore>,
    #[doc = "Specifies if default quota is enabled for the volume."]
    #[serde(rename = "isDefaultQuotaEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_default_quota_enabled: Option<bool>,
    #[doc = "Default user quota for volume in KiBs. If isDefaultQuotaEnabled is set, the minimum value of 4 KiBs applies ."]
    #[serde(rename = "defaultUserQuotaInKiBs", default, skip_serializing_if = "Option::is_none")]
    pub default_user_quota_in_ki_bs: Option<i64>,
    #[doc = "Default group quota for volume in KiBs. If isDefaultQuotaEnabled is set, the minimum value of 4 KiBs applies."]
    #[serde(rename = "defaultGroupQuotaInKiBs", default, skip_serializing_if = "Option::is_none")]
    pub default_group_quota_in_ki_bs: Option<i64>,
    #[doc = "Maximum number of files allowed. Needs a service request in order to be changed. Only allowed to be changed if volume quota is more than 4TiB."]
    #[serde(rename = "maximumNumberOfFiles", default, skip_serializing_if = "Option::is_none")]
    pub maximum_number_of_files: Option<i64>,
    #[doc = "Volume Group Name"]
    #[serde(rename = "volumeGroupName", default, skip_serializing_if = "Option::is_none")]
    pub volume_group_name: Option<String>,
    #[doc = "Pool Resource Id used in case of creating a volume through volume group"]
    #[serde(rename = "capacityPoolResourceId", default, skip_serializing_if = "Option::is_none")]
    pub capacity_pool_resource_id: Option<String>,
    #[doc = "Proximity placement group associated with the volume"]
    #[serde(rename = "proximityPlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group: Option<String>,
    #[doc = "T2 network information"]
    #[serde(rename = "t2Network", default, skip_serializing_if = "Option::is_none")]
    pub t2_network: Option<String>,
    #[doc = "Volume spec name is the application specific designation or identifier for the particular volume in a volume group for e.g. data, log"]
    #[serde(rename = "volumeSpecName", default, skip_serializing_if = "Option::is_none")]
    pub volume_spec_name: Option<String>,
    #[doc = "Specifies if the volume is encrypted or not. Only available on volumes created or updated after 2022-01-01."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[doc = "Application specific placement rules for the particular volume"]
    #[serde(rename = "placementRules", default, skip_serializing_if = "Vec::is_empty")]
    pub placement_rules: Vec<PlacementKeyValuePairs>,
    #[doc = "Flag indicating whether subvolume operations are enabled on the volume"]
    #[serde(rename = "enableSubvolumes", default, skip_serializing_if = "Option::is_none")]
    pub enable_subvolumes: Option<volume_properties::EnableSubvolumes>,
}
impl VolumeProperties {
    pub fn new(creation_token: String, usage_threshold: i64, subnet_id: String) -> Self {
        Self {
            file_system_id: None,
            creation_token,
            service_level: None,
            usage_threshold,
            export_policy: None,
            protocol_types: Vec::new(),
            provisioning_state: None,
            snapshot_id: None,
            backup_id: None,
            baremetal_tenant_id: None,
            subnet_id,
            network_features: None,
            network_sibling_set_id: None,
            storage_to_network_proximity: None,
            mount_targets: Vec::new(),
            volume_type: None,
            data_protection: None,
            is_restoring: None,
            snapshot_directory_visible: None,
            kerberos_enabled: None,
            security_style: None,
            smb_encryption: None,
            smb_continuously_available: None,
            throughput_mibps: None,
            encryption_key_source: None,
            ldap_enabled: None,
            cool_access: None,
            coolness_period: None,
            unix_permissions: None,
            clone_progress: None,
            avs_data_store: None,
            is_default_quota_enabled: None,
            default_user_quota_in_ki_bs: None,
            default_group_quota_in_ki_bs: None,
            maximum_number_of_files: None,
            volume_group_name: None,
            capacity_pool_resource_id: None,
            proximity_placement_group: None,
            t2_network: None,
            volume_spec_name: None,
            encrypted: None,
            placement_rules: Vec::new(),
            enable_subvolumes: None,
        }
    }
}
pub mod volume_properties {
    use super::*;
    #[doc = "Set of export policy rules"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExportPolicy {
        #[doc = "Export policy rule"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub rules: Vec<ExportPolicyRule>,
    }
    impl ExportPolicy {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Basic network, or Standard features available to the volume."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NetworkFeatures")]
    pub enum NetworkFeatures {
        Basic,
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NetworkFeatures {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NetworkFeatures {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NetworkFeatures {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("NetworkFeatures", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("NetworkFeatures", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for NetworkFeatures {
        fn default() -> Self {
            Self::Basic
        }
    }
    #[doc = "Provides storage to network proximity information for the volume."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageToNetworkProximity")]
    pub enum StorageToNetworkProximity {
        Default,
        T1,
        T2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageToNetworkProximity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageToNetworkProximity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageToNetworkProximity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("StorageToNetworkProximity", 0u32, "Default"),
                Self::T1 => serializer.serialize_unit_variant("StorageToNetworkProximity", 1u32, "T1"),
                Self::T2 => serializer.serialize_unit_variant("StorageToNetworkProximity", 2u32, "T2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "DataProtection type volumes include an object containing details of the replication"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DataProtection {
        #[doc = "Volume Backup Properties"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub backup: Option<VolumeBackupProperties>,
        #[doc = "Replication properties"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replication: Option<ReplicationObject>,
        #[doc = "Volume Snapshot Properties"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot: Option<VolumeSnapshotProperties>,
    }
    impl DataProtection {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The security style of volume, default unix, defaults to ntfs for dual protocol or CIFS protocol"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecurityStyle")]
    pub enum SecurityStyle {
        #[serde(rename = "ntfs")]
        Ntfs,
        #[serde(rename = "unix")]
        Unix,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecurityStyle {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecurityStyle {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecurityStyle {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ntfs => serializer.serialize_unit_variant("SecurityStyle", 0u32, "ntfs"),
                Self::Unix => serializer.serialize_unit_variant("SecurityStyle", 1u32, "unix"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for SecurityStyle {
        fn default() -> Self {
            Self::Unix
        }
    }
    #[doc = "Source of key used to encrypt data in volume. Possible values (case-insensitive) are: 'Microsoft.NetApp'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EncryptionKeySource")]
    pub enum EncryptionKeySource {
        #[serde(rename = "Microsoft.NetApp")]
        MicrosoftNetApp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EncryptionKeySource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EncryptionKeySource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EncryptionKeySource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftNetApp => serializer.serialize_unit_variant("EncryptionKeySource", 0u32, "Microsoft.NetApp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for EncryptionKeySource {
        fn default() -> Self {
            Self::MicrosoftNetApp
        }
    }
    #[doc = "Specifies whether the volume is enabled for Azure VMware Solution (AVS) datastore purpose"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AvsDataStore")]
    pub enum AvsDataStore {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AvsDataStore {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AvsDataStore {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AvsDataStore {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("AvsDataStore", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("AvsDataStore", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AvsDataStore {
        fn default() -> Self {
            Self::Disabled
        }
    }
    #[doc = "Flag indicating whether subvolume operations are enabled on the volume"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnableSubvolumes")]
    pub enum EnableSubvolumes {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnableSubvolumes {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnableSubvolumes {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnableSubvolumes {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("EnableSubvolumes", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnableSubvolumes", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for EnableSubvolumes {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "Quota Rule of a Volume"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeQuotaRule {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Volume Quota Rule properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeQuotaRulesProperties>,
}
impl VolumeQuotaRule {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Patchable Quota Rule of a Volume"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeQuotaRulePatch {
    #[doc = "Volume Quota Rule properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeQuotaRulesProperties>,
}
impl VolumeQuotaRulePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Volume Quota Rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeQuotaRulesList {
    #[doc = "A list of Volume Quota Rules"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VolumeQuotaRule>,
}
impl azure_core::Continuable for VolumeQuotaRulesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl VolumeQuotaRulesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume Quota Rule properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeQuotaRulesProperties {
    #[doc = "Gets the status of the VolumeQuotaRule at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<NetappProvisioningState>,
    #[doc = "Size of quota"]
    #[serde(rename = "quotaSizeInKiBs", default, skip_serializing_if = "Option::is_none")]
    pub quota_size_in_ki_bs: Option<i64>,
    #[doc = "Type of quota"]
    #[serde(rename = "quotaType", default, skip_serializing_if = "Option::is_none")]
    pub quota_type: Option<volume_quota_rules_properties::QuotaType>,
    #[doc = "UserID/GroupID/SID based on the quota target type. UserID and groupID can be found by running ‘id’ or ‘getent’ command for the user or group and SID can be found by running <wmic useraccount where name='user-name' get sid>"]
    #[serde(rename = "quotaTarget", default, skip_serializing_if = "Option::is_none")]
    pub quota_target: Option<String>,
}
impl VolumeQuotaRulesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod volume_quota_rules_properties {
    use super::*;
    #[doc = "Type of quota"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "QuotaType")]
    pub enum QuotaType {
        DefaultUserQuota,
        DefaultGroupQuota,
        IndividualUserQuota,
        IndividualGroupQuota,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for QuotaType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for QuotaType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for QuotaType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DefaultUserQuota => serializer.serialize_unit_variant("QuotaType", 0u32, "DefaultUserQuota"),
                Self::DefaultGroupQuota => serializer.serialize_unit_variant("QuotaType", 1u32, "DefaultGroupQuota"),
                Self::IndividualUserQuota => serializer.serialize_unit_variant("QuotaType", 2u32, "IndividualUserQuota"),
                Self::IndividualGroupQuota => serializer.serialize_unit_variant("QuotaType", 3u32, "IndividualGroupQuota"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Volume relocation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeRelocationProperties {
    #[doc = "The id of the old volume that is being relocated"]
    #[serde(rename = "oldVolumeId", default, skip_serializing_if = "Option::is_none")]
    pub old_volume_id: Option<String>,
    #[doc = "The id of the bare metal tenant owned by the existing volume"]
    #[serde(rename = "oldBareMetalTenantId", default, skip_serializing_if = "Option::is_none")]
    pub old_bare_metal_tenant_id: Option<String>,
    #[doc = "Has relocation been requested for this volume"]
    #[serde(rename = "relocationRequested", default, skip_serializing_if = "Option::is_none")]
    pub relocation_requested: Option<bool>,
}
impl VolumeRelocationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "revert a volume to the snapshot"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeRevert {
    #[doc = "Resource id of the snapshot"]
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}
impl VolumeRevert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume Snapshot Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeSnapshotProperties {
    #[doc = "Snapshot Policy ResourceId"]
    #[serde(rename = "snapshotPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_policy_id: Option<String>,
}
impl VolumeSnapshotProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Weekly Schedule properties, make a snapshot every week at a specific day or days"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WeeklySchedule {
    #[doc = "Weekly snapshot count to keep"]
    #[serde(rename = "snapshotsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub snapshots_to_keep: Option<i32>,
    #[doc = "Indicates which weekdays snapshot should be taken, accepts a comma separated list of week day names in english"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    #[doc = "Indicates which hour in UTC timezone a snapshot should be taken"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[doc = "Indicates which minute snapshot should be taken"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
    #[doc = "Resource size in bytes, current storage usage for the volume in bytes"]
    #[serde(rename = "usedBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_bytes: Option<i64>,
}
impl WeeklySchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
