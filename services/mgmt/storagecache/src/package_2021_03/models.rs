#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "REST API operation description: see https://github.com/Azure/azure-rest-api-specs/blob/master/documentation/openapi-authoring-automated-guidelines.md#r3023-operationsapiimplementation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOperation {
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<api_operation::Display>,
    #[doc = "Origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The flag that indicates whether the operation applies to data plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Additional details about an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<api_operation::Properties>,
}
impl ApiOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Service provider: Microsoft.StorageCache"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Cache, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The description of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Additional details about an operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Specification of the all the metrics provided for a resource type."]
        #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
        pub service_specification: Option<properties::ServiceSpecification>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Specification of the all the metrics provided for a resource type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct ServiceSpecification {
            #[doc = "Details about operations related to metrics."]
            #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
            pub metric_specifications: Vec<MetricSpecification>,
        }
        impl ServiceSpecification {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "Result of the request to list Resource Provider operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOperationListResult {
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of Resource Provider operations supported by the Microsoft.StorageCache resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiOperation>,
}
impl azure_core::Continuable for ApiOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApiOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AscOperation {
    #[doc = "The operation Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Describes the format of Error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
    #[doc = "Additional operation-specific output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AscOperationProperties>,
}
impl AscOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional operation-specific output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AscOperationProperties {
    #[doc = "Additional operation-specific output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}
impl AscOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties pertaining to the BlobNfsTarget."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobNfsTarget {
    #[doc = "A fully qualified URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<UrlString>,
    #[doc = "Identifies the StorageCache usage model to be used for this storage target."]
    #[serde(rename = "usageModel", default, skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<String>,
}
impl BlobNfsTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Cache instance. Follows Azure Resource Manager standards: https://github.com/Azure/azure-resource-manager-rpc/blob/master/v1.0/resource-api-reference.md"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cache {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "A fully qualified URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UrlString>,
    #[doc = "Region name string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Schema for the name of resources served by this provider. Note that objects will contain an odata @id annotation as appropriate. This will contain the complete URL of the object. These names are case-preserving, but not case sensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "Type of the Cache; Microsoft.StorageCache/Cache"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Cache identity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<CacheIdentity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of the Cache."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<cache::Properties>,
    #[doc = "SKU for the Cache."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<cache::Sku>,
}
impl Cache {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cache {
    use super::*;
    #[doc = "Properties of the Cache."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The size of this Cache, in GB."]
        #[serde(rename = "cacheSizeGB", default, skip_serializing_if = "Option::is_none")]
        pub cache_size_gb: Option<i32>,
        #[doc = "An indication of Cache health. Gives more information about health than just that related to provisioning."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub health: Option<CacheHealth>,
        #[doc = "Array of IP addresses that can be used by clients mounting this Cache."]
        #[serde(rename = "mountAddresses", default, skip_serializing_if = "Vec::is_empty")]
        pub mount_addresses: Vec<String>,
        #[doc = "ARM provisioning state, see https://github.com/Azure/azure-resource-manager-rpc/blob/master/v1.0/Addendum.md#provisioningstate-property"]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "A fully qualified URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subnet: Option<UrlString>,
        #[doc = "Properties describing the software upgrade state of the Cache."]
        #[serde(rename = "upgradeStatus", default, skip_serializing_if = "Option::is_none")]
        pub upgrade_status: Option<CacheUpgradeStatus>,
        #[doc = "Cache network settings."]
        #[serde(rename = "networkSettings", default, skip_serializing_if = "Option::is_none")]
        pub network_settings: Option<CacheNetworkSettings>,
        #[doc = "Cache encryption settings."]
        #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Option::is_none")]
        pub encryption_settings: Option<CacheEncryptionSettings>,
        #[doc = "Cache security settings."]
        #[serde(rename = "securitySettings", default, skip_serializing_if = "Option::is_none")]
        pub security_settings: Option<CacheSecuritySettings>,
        #[doc = "Cache Directory Services settings."]
        #[serde(rename = "directoryServicesSettings", default, skip_serializing_if = "Option::is_none")]
        pub directory_services_settings: Option<CacheDirectorySettings>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "ARM provisioning state, see https://github.com/Azure/azure-resource-manager-rpc/blob/master/v1.0/Addendum.md#provisioningstate-property"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Cancelled,
            Creating,
            Deleting,
            Updating,
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
                    Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                    Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                    Self::Cancelled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Cancelled"),
                    Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
                    Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                    Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "SKU for the Cache."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Sku {
        #[doc = "SKU name for this Cache."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl Sku {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Active Directory settings used to join a cache to a domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheActiveDirectorySettings {
    #[doc = "Primary DNS IP address used to resolve the Active Directory domain controller's fully qualified domain name."]
    #[serde(rename = "primaryDnsIpAddress")]
    pub primary_dns_ip_address: String,
    #[doc = "Secondary DNS IP address used to resolve the Active Directory domain controller's fully qualified domain name."]
    #[serde(rename = "secondaryDnsIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub secondary_dns_ip_address: Option<String>,
    #[doc = "The fully qualified domain name of the Active Directory domain controller."]
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[doc = "The Active Directory domain's NetBIOS name."]
    #[serde(rename = "domainNetBiosName")]
    pub domain_net_bios_name: String,
    #[doc = "The NetBIOS name to assign to the HPC Cache when it joins the Active Directory domain as a server. Length must 1-15 characters from the class [-0-9a-zA-Z]."]
    #[serde(rename = "cacheNetBiosName")]
    pub cache_net_bios_name: String,
    #[doc = "True if the HPC Cache is joined to the Active Directory domain."]
    #[serde(rename = "domainJoined", default, skip_serializing_if = "Option::is_none")]
    pub domain_joined: Option<cache_active_directory_settings::DomainJoined>,
    #[doc = "Active Directory admin credentials used to join the HPC Cache to a domain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<cache_active_directory_settings::Credentials>,
}
impl CacheActiveDirectorySettings {
    pub fn new(primary_dns_ip_address: String, domain_name: String, domain_net_bios_name: String, cache_net_bios_name: String) -> Self {
        Self {
            primary_dns_ip_address,
            secondary_dns_ip_address: None,
            domain_name,
            domain_net_bios_name,
            cache_net_bios_name,
            domain_joined: None,
            credentials: None,
        }
    }
}
pub mod cache_active_directory_settings {
    use super::*;
    #[doc = "True if the HPC Cache is joined to the Active Directory domain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DomainJoined")]
    pub enum DomainJoined {
        Yes,
        No,
        Error,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DomainJoined {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DomainJoined {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DomainJoined {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Yes => serializer.serialize_unit_variant("DomainJoined", 0u32, "Yes"),
                Self::No => serializer.serialize_unit_variant("DomainJoined", 1u32, "No"),
                Self::Error => serializer.serialize_unit_variant("DomainJoined", 2u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Active Directory admin credentials used to join the HPC Cache to a domain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Credentials {
        #[doc = "Username of the Active Directory domain administrator. This value is stored encrypted and not returned on response."]
        pub username: String,
        #[doc = "Plain text password of the Active Directory domain administrator. This value is stored encrypted and not returned on response."]
        pub password: String,
    }
    impl Credentials {
        pub fn new(username: String, password: String) -> Self {
            Self { username, password }
        }
    }
}
#[doc = "Cache Directory Services settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheDirectorySettings {
    #[doc = "Active Directory settings used to join a cache to a domain."]
    #[serde(rename = "activeDirectory", default, skip_serializing_if = "Option::is_none")]
    pub active_directory: Option<CacheActiveDirectorySettings>,
    #[doc = "Settings for Extended Groups username and group download."]
    #[serde(rename = "usernameDownload", default, skip_serializing_if = "Option::is_none")]
    pub username_download: Option<CacheUsernameDownloadSettings>,
}
impl CacheDirectorySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cache encryption settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheEncryptionSettings {
    #[doc = "Describes a reference to Key Vault Key."]
    #[serde(rename = "keyEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyVaultKeyReference>,
}
impl CacheEncryptionSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An indication of Cache health. Gives more information about health than just that related to provisioning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheHealth {
    #[doc = "List of Cache health states."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<cache_health::State>,
    #[doc = "Describes explanation of state."]
    #[serde(rename = "statusDescription", default, skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[doc = "Outstanding conditions that need to be investigated and resolved."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}
impl CacheHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cache_health {
    use super::*;
    #[doc = "List of Cache health states."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Unknown,
        Healthy,
        Degraded,
        Down,
        Transitioning,
        Stopping,
        Stopped,
        Upgrading,
        Flushing,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("State", 0u32, "Unknown"),
                Self::Healthy => serializer.serialize_unit_variant("State", 1u32, "Healthy"),
                Self::Degraded => serializer.serialize_unit_variant("State", 2u32, "Degraded"),
                Self::Down => serializer.serialize_unit_variant("State", 3u32, "Down"),
                Self::Transitioning => serializer.serialize_unit_variant("State", 4u32, "Transitioning"),
                Self::Stopping => serializer.serialize_unit_variant("State", 5u32, "Stopping"),
                Self::Stopped => serializer.serialize_unit_variant("State", 6u32, "Stopped"),
                Self::Upgrading => serializer.serialize_unit_variant("State", 7u32, "Upgrading"),
                Self::Flushing => serializer.serialize_unit_variant("State", 8u32, "Flushing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Cache identity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheIdentity {
    #[doc = "The principal id of the cache."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id associated with the cache."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the cache"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<cache_identity::Type>,
}
impl CacheIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cache_identity {
    use super::*;
    #[doc = "The type of identity used for the cache"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[doc = "Cache network settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheNetworkSettings {
    #[doc = "The IPv4 maximum transmission unit configured for the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[doc = "Array of additional IP addresses used by this Cache."]
    #[serde(rename = "utilityAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub utility_addresses: Vec<String>,
    #[doc = "DNS servers for the cache to use.  It will be set from the network configuration if no value is provided."]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
    #[doc = "DNS search domain"]
    #[serde(rename = "dnsSearchDomain", default, skip_serializing_if = "Option::is_none")]
    pub dns_search_domain: Option<String>,
    #[doc = "NTP server IP Address or FQDN for the cache to use. The default is time.windows.com."]
    #[serde(rename = "ntpServer", default, skip_serializing_if = "Option::is_none")]
    pub ntp_server: Option<String>,
}
impl CacheNetworkSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cache security settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheSecuritySettings {
    #[doc = "NFS access policies defined for this cache."]
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<NfsAccessPolicy>,
}
impl CacheSecuritySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties describing the software upgrade state of the Cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheUpgradeStatus {
    #[doc = "Version string of the firmware currently installed on this Cache."]
    #[serde(rename = "currentFirmwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_firmware_version: Option<String>,
    #[doc = "True if there is a firmware update ready to install on this Cache. The firmware will automatically be installed after firmwareUpdateDeadline if not triggered earlier via the upgrade operation."]
    #[serde(rename = "firmwareUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub firmware_update_status: Option<cache_upgrade_status::FirmwareUpdateStatus>,
    #[doc = "Time at which the pending firmware update will automatically be installed on the Cache."]
    #[serde(rename = "firmwareUpdateDeadline", default, with = "azure_core::date::rfc3339::option")]
    pub firmware_update_deadline: Option<time::OffsetDateTime>,
    #[doc = "Time of the last successful firmware update."]
    #[serde(rename = "lastFirmwareUpdate", default, with = "azure_core::date::rfc3339::option")]
    pub last_firmware_update: Option<time::OffsetDateTime>,
    #[doc = "When firmwareUpdateAvailable is true, this field holds the version string for the update."]
    #[serde(rename = "pendingFirmwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub pending_firmware_version: Option<String>,
}
impl CacheUpgradeStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cache_upgrade_status {
    use super::*;
    #[doc = "True if there is a firmware update ready to install on this Cache. The firmware will automatically be installed after firmwareUpdateDeadline if not triggered earlier via the upgrade operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FirmwareUpdateStatus")]
    pub enum FirmwareUpdateStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FirmwareUpdateStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FirmwareUpdateStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FirmwareUpdateStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Available => serializer.serialize_unit_variant("FirmwareUpdateStatus", 0u32, "available"),
                Self::Unavailable => serializer.serialize_unit_variant("FirmwareUpdateStatus", 1u32, "unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Settings for Extended Groups username and group download."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheUsernameDownloadSettings {
    #[doc = "Whether or not Extended Groups is enabled."]
    #[serde(rename = "extendedGroups", default, skip_serializing_if = "Option::is_none")]
    pub extended_groups: Option<bool>,
    #[doc = "This setting determines how the cache gets username and group names for clients."]
    #[serde(rename = "usernameSource", default, skip_serializing_if = "Option::is_none")]
    pub username_source: Option<cache_username_download_settings::UsernameSource>,
    #[doc = "The URI of the file containing group information (in /etc/group file format). This field must be populated when 'usernameSource' is set to 'File'."]
    #[serde(rename = "groupFileURI", default, skip_serializing_if = "Option::is_none")]
    pub group_file_uri: Option<String>,
    #[doc = "The URI of the file containing user information (in /etc/passwd file format). This field must be populated when 'usernameSource' is set to 'File'."]
    #[serde(rename = "userFileURI", default, skip_serializing_if = "Option::is_none")]
    pub user_file_uri: Option<String>,
    #[doc = "The fully qualified domain name or IP address of the LDAP server to use."]
    #[serde(rename = "ldapServer", default, skip_serializing_if = "Option::is_none")]
    pub ldap_server: Option<String>,
    #[doc = "The base distinguished name for the LDAP domain."]
    #[serde(rename = "ldapBaseDN", default, skip_serializing_if = "Option::is_none")]
    pub ldap_base_dn: Option<String>,
    #[doc = "Whether or not the LDAP connection should be encrypted."]
    #[serde(rename = "encryptLdapConnection", default, skip_serializing_if = "Option::is_none")]
    pub encrypt_ldap_connection: Option<bool>,
    #[doc = "Determines if the certificates must be validated by a certificate authority. When true, caCertificateURI must be provided."]
    #[serde(rename = "requireValidCertificate", default, skip_serializing_if = "Option::is_none")]
    pub require_valid_certificate: Option<bool>,
    #[doc = "Determines if the certificate should be automatically downloaded. This applies to 'caCertificateURI' only if 'requireValidCertificate' is true."]
    #[serde(rename = "autoDownloadCertificate", default, skip_serializing_if = "Option::is_none")]
    pub auto_download_certificate: Option<bool>,
    #[doc = "The URI of the CA certificate to validate the LDAP secure connection. This field must be populated when 'requireValidCertificate' is set to true."]
    #[serde(rename = "caCertificateURI", default, skip_serializing_if = "Option::is_none")]
    pub ca_certificate_uri: Option<String>,
    #[doc = "Indicates whether or not the HPC Cache has performed the username download successfully."]
    #[serde(rename = "usernameDownloaded", default, skip_serializing_if = "Option::is_none")]
    pub username_downloaded: Option<cache_username_download_settings::UsernameDownloaded>,
    #[doc = "When present, these are the credentials for the secure LDAP connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<cache_username_download_settings::Credentials>,
}
impl CacheUsernameDownloadSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cache_username_download_settings {
    use super::*;
    #[doc = "This setting determines how the cache gets username and group names for clients."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UsernameSource")]
    pub enum UsernameSource {
        #[serde(rename = "AD")]
        Ad,
        #[serde(rename = "LDAP")]
        Ldap,
        File,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UsernameSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UsernameSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UsernameSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ad => serializer.serialize_unit_variant("UsernameSource", 0u32, "AD"),
                Self::Ldap => serializer.serialize_unit_variant("UsernameSource", 1u32, "LDAP"),
                Self::File => serializer.serialize_unit_variant("UsernameSource", 2u32, "File"),
                Self::None => serializer.serialize_unit_variant("UsernameSource", 3u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for UsernameSource {
        fn default() -> Self {
            Self::None
        }
    }
    #[doc = "Indicates whether or not the HPC Cache has performed the username download successfully."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UsernameDownloaded")]
    pub enum UsernameDownloaded {
        Yes,
        No,
        Error,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UsernameDownloaded {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UsernameDownloaded {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UsernameDownloaded {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Yes => serializer.serialize_unit_variant("UsernameDownloaded", 0u32, "Yes"),
                Self::No => serializer.serialize_unit_variant("UsernameDownloaded", 1u32, "No"),
                Self::Error => serializer.serialize_unit_variant("UsernameDownloaded", 2u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "When present, these are the credentials for the secure LDAP connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Credentials {
        #[doc = "The Bind Distinguished Name identity to be used in the secure LDAP connection. This value is stored encrypted and not returned on response."]
        #[serde(rename = "bindDn", default, skip_serializing_if = "Option::is_none")]
        pub bind_dn: Option<String>,
        #[doc = "The Bind password to be used in the secure LDAP connection. This value is stored encrypted and not returned on response."]
        #[serde(rename = "bindPassword", default, skip_serializing_if = "Option::is_none")]
        pub bind_password: Option<String>,
    }
    impl Credentials {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Caches. It contains a list of Caches and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CachesListResult {
    #[doc = "URL to get the next set of Cache list results, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of Caches."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cache>,
}
impl azure_core::Continuable for CachesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CachesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties pertaining to the ClfsTarget"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClfsTarget {
    #[doc = "A fully qualified URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<UrlString>,
}
impl ClfsTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response."]
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
#[doc = "An error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Outstanding conditions that will need to be resolved."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Condition {
    #[doc = "The time when the condition was raised."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The issue requiring attention."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Condition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a reference to Key Vault Key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultKeyReference {
    #[doc = "The URL referencing a key encryption key in Key Vault."]
    #[serde(rename = "keyUrl")]
    pub key_url: String,
    #[doc = "Describes a resource Id to source Key Vault."]
    #[serde(rename = "sourceVault")]
    pub source_vault: key_vault_key_reference::SourceVault,
}
impl KeyVaultKeyReference {
    pub fn new(key_url: String, source_vault: key_vault_key_reference::SourceVault) -> Self {
        Self { key_url, source_vault }
    }
}
pub mod key_vault_key_reference {
    use super::*;
    #[doc = "Describes a resource Id to source Key Vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct SourceVault {
        #[doc = "Resource Id."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl SourceVault {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Specifications of the Dimension of metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDimension {
    #[doc = "Name of the dimension"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the dimension"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Internal name of the dimension."]
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[doc = "To be exported to shoe box."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl MetricDimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about operation related to metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "The name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The unit that the metric is measured in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The type of metric aggregation."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Support metric aggregation type."]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "Type of metrics."]
    #[serde(rename = "metricClass", default, skip_serializing_if = "Option::is_none")]
    pub metric_class: Option<String>,
    #[doc = "Dimensions of the metric"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A namespace junction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceJunction {
    #[doc = "Namespace path on a Cache for a Storage Target."]
    #[serde(rename = "namespacePath", default, skip_serializing_if = "Option::is_none")]
    pub namespace_path: Option<String>,
    #[doc = "Path in Storage Target to which namespacePath points."]
    #[serde(rename = "targetPath", default, skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    #[doc = "NFS export where targetPath exists."]
    #[serde(rename = "nfsExport", default, skip_serializing_if = "Option::is_none")]
    pub nfs_export: Option<String>,
    #[doc = "Name of the access policy applied to this junction."]
    #[serde(rename = "nfsAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub nfs_access_policy: Option<String>,
}
impl NamespaceJunction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties pertaining to the Nfs3Target"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Nfs3Target {
    #[doc = "IP address or host name of an NFSv3 host (e.g., 10.0.44.44)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Identifies the StorageCache usage model to be used for this storage target."]
    #[serde(rename = "usageModel", default, skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<String>,
}
impl Nfs3Target {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of rules describing access policies applied to NFSv3 clients of the cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfsAccessPolicy {
    #[doc = "Name identifying this policy. Access Policy names are not case sensitive."]
    pub name: String,
    #[doc = "The set of rules describing client accesses allowed under this policy."]
    #[serde(rename = "accessRules")]
    pub access_rules: Vec<NfsAccessRule>,
}
impl NfsAccessPolicy {
    pub fn new(name: String, access_rules: Vec<NfsAccessRule>) -> Self {
        Self { name, access_rules }
    }
}
#[doc = "Rule to place restrictions on portions of the cache namespace being presented to clients."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfsAccessRule {
    #[doc = "Scope for this rule. The scope and filter determine which clients match the rule."]
    pub scope: nfs_access_rule::Scope,
    #[doc = "Filter applied to the scope for this rule. The filter's format depends on its scope. 'default' scope matches all clients and has no filter value. 'network' scope takes a filter in CIDR format (for example, 10.99.1.0/24). 'host' takes an IP address or fully qualified domain name as filter. If a client does not match any filter rule and there is no default rule, access is denied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[doc = "Access allowed by this rule."]
    pub access: nfs_access_rule::Access,
    #[doc = "Allow SUID semantics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suid: Option<bool>,
    #[doc = "For the default policy, allow access to subdirectories under the root export. If this is set to no, clients can only mount the path '/'. If set to yes, clients can mount a deeper path, like '/a/b'."]
    #[serde(rename = "submountAccess", default, skip_serializing_if = "Option::is_none")]
    pub submount_access: Option<bool>,
    #[doc = "Map root accesses to anonymousUID and anonymousGID."]
    #[serde(rename = "rootSquash", default, skip_serializing_if = "Option::is_none")]
    pub root_squash: Option<bool>,
    #[doc = "UID value that replaces 0 when rootSquash is true. 65534 will be used if not provided."]
    #[serde(rename = "anonymousUID", default, skip_serializing_if = "Option::is_none")]
    pub anonymous_uid: Option<String>,
    #[doc = "GID value that replaces 0 when rootSquash is true. This will use the value of anonymousUID if not provided."]
    #[serde(rename = "anonymousGID", default, skip_serializing_if = "Option::is_none")]
    pub anonymous_gid: Option<String>,
}
impl NfsAccessRule {
    pub fn new(scope: nfs_access_rule::Scope, access: nfs_access_rule::Access) -> Self {
        Self {
            scope,
            filter: None,
            access,
            suid: None,
            submount_access: None,
            root_squash: None,
            anonymous_uid: None,
            anonymous_gid: None,
        }
    }
}
pub mod nfs_access_rule {
    use super::*;
    #[doc = "Scope for this rule. The scope and filter determine which clients match the rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Scope")]
    pub enum Scope {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "network")]
        Network,
        #[serde(rename = "host")]
        Host,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Scope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Scope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Scope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("Scope", 0u32, "default"),
                Self::Network => serializer.serialize_unit_variant("Scope", 1u32, "network"),
                Self::Host => serializer.serialize_unit_variant("Scope", 2u32, "host"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Access allowed by this rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Access")]
    pub enum Access {
        #[serde(rename = "no")]
        No,
        #[serde(rename = "ro")]
        Ro,
        #[serde(rename = "rw")]
        Rw,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Access {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Access {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Access {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::No => serializer.serialize_unit_variant("Access", 0u32, "no"),
                Self::Ro => serializer.serialize_unit_variant("Access", 1u32, "ro"),
                Self::Rw => serializer.serialize_unit_variant("Access", 2u32, "rw"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type ResourceName = String;
#[doc = "A resource SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSku {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "A list of capabilities of this SKU, such as throughput or ops/sec."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceSkuCapabilities>,
    #[doc = "The set of locations where the SKU is available. This is the supported and registered Azure Geo Regions (e.g., West US, East US, Southeast Asia, etc.)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "The set of locations where the SKU is available."]
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<ResourceSkuLocationInfo>,
    #[doc = "The name of this SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The restrictions preventing this SKU from being used. This is empty if there are no restrictions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<Restriction>,
}
impl ResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A resource SKU capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCapabilities {
    #[doc = "Name of a capability, such as ops/sec."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Quantity, if the capability is measured by quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ResourceSkuCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource SKU location information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuLocationInfo {
    #[doc = "Location where this SKU is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Zones if any."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl ResourceSkuLocationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List Cache SKUs operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkusResult {
    #[doc = "The URI to fetch the next page of Cache SKUs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of SKUs available for the subscription."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceSku>,
}
impl azure_core::Continuable for ResourceSkusResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceSkusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The restrictions preventing this SKU from being used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Restriction {
    #[doc = "The type of restrictions. In this version, the only possible value for this is location."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The value of restrictions. If the restriction type is set to location, then this would be the different locations where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "The reason for the restriction. As of now this can be \"QuotaId\" or \"NotAvailableForSubscription\". \"QuotaId\" is set when the SKU has requiredQuotas parameter as the subscription does not belong to that quota. \"NotAvailableForSubscription\" is related to capacity at the datacenter."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<restriction::ReasonCode>,
}
impl Restriction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restriction {
    use super::*;
    #[doc = "The reason for the restriction. As of now this can be \"QuotaId\" or \"NotAvailableForSubscription\". \"QuotaId\" is set when the SKU has requiredQuotas parameter as the subscription does not belong to that quota. \"NotAvailableForSubscription\" is related to capacity at the datacenter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReasonCode")]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::QuotaId => serializer.serialize_unit_variant("ReasonCode", 0u32, "QuotaId"),
                Self::NotAvailableForSubscription => serializer.serialize_unit_variant("ReasonCode", 1u32, "NotAvailableForSubscription"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Type of the Storage Target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTarget {
    #[serde(flatten)]
    pub storage_target_resource: StorageTargetResource,
    #[doc = "Properties of the Storage Target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageTargetProperties>,
}
impl StorageTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Storage Target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTargetProperties {
    #[doc = "List of Cache namespace junctions to target for namespace associations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub junctions: Vec<NamespaceJunction>,
    #[doc = "Type of the Storage Target."]
    #[serde(rename = "targetType")]
    pub target_type: storage_target_properties::TargetType,
    #[doc = "ARM provisioning state, see https://github.com/Azure/azure-resource-manager-rpc/blob/master/v1.0/Addendum.md#provisioningstate-property"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_target_properties::ProvisioningState>,
    #[doc = "Properties pertaining to the Nfs3Target"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs3: Option<Nfs3Target>,
    #[doc = "Properties pertaining to the ClfsTarget"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clfs: Option<ClfsTarget>,
    #[doc = "Properties pertaining to the UnknownTarget"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown: Option<UnknownTarget>,
    #[doc = "Properties pertaining to the BlobNfsTarget."]
    #[serde(rename = "blobNfs", default, skip_serializing_if = "Option::is_none")]
    pub blob_nfs: Option<BlobNfsTarget>,
}
impl StorageTargetProperties {
    pub fn new(target_type: storage_target_properties::TargetType) -> Self {
        Self {
            junctions: Vec::new(),
            target_type,
            provisioning_state: None,
            nfs3: None,
            clfs: None,
            unknown: None,
            blob_nfs: None,
        }
    }
}
pub mod storage_target_properties {
    use super::*;
    #[doc = "Type of the Storage Target."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetType")]
    pub enum TargetType {
        #[serde(rename = "nfs3")]
        Nfs3,
        #[serde(rename = "clfs")]
        Clfs,
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "blobNfs")]
        BlobNfs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Nfs3 => serializer.serialize_unit_variant("TargetType", 0u32, "nfs3"),
                Self::Clfs => serializer.serialize_unit_variant("TargetType", 1u32, "clfs"),
                Self::Unknown => serializer.serialize_unit_variant("TargetType", 2u32, "unknown"),
                Self::BlobNfs => serializer.serialize_unit_variant("TargetType", 3u32, "blobNfs"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "ARM provisioning state, see https://github.com/Azure/azure-resource-manager-rpc/blob/master/v1.0/Addendum.md#provisioningstate-property"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Creating,
        Deleting,
        Updating,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Cancelled"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Resource used by a Cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTargetResource {
    #[doc = "Schema for the name of resources served by this provider. Note that objects will contain an odata @id annotation as appropriate. This will contain the complete URL of the object. These names are case-preserving, but not case sensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "Resource ID of the Storage Target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of the Storage Target; Microsoft.StorageCache/Cache/StorageTarget"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Region name string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl StorageTargetResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Storage Targets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTargetsResult {
    #[doc = "The URI to fetch the next page of Storage Targets."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of Storage Targets defined for the Cache."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageTarget>,
}
impl azure_core::Continuable for StorageTargetsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageTargetsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type UrlString = String;
#[doc = "Properties of an unknown type of Storage Target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnknownProperties {}
impl UnknownProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties pertaining to the UnknownTarget"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnknownTarget {
    #[doc = "Properties of an unknown type of Storage Target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<UnknownProperties>,
}
impl UnknownTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A usage model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageModel {
    #[doc = "Localized information describing this usage model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<usage_model::Display>,
    #[doc = "Non-localized keyword name for this usage model."]
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[doc = "The type of Storage Target to which this model is applicable (only nfs3 as of this version)."]
    #[serde(rename = "targetType", default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}
impl UsageModel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage_model {
    use super::*;
    #[doc = "Localized information describing this usage model."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "String to display for this usage model."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A list of Cache usage models."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageModelsResult {
    #[doc = "The URI to fetch the next page of Cache usage models."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of usage models available for the subscription."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageModel>,
}
impl azure_core::Continuable for UsageModelsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UsageModelsResult {
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
