#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The parameters to list SAS credentials of a storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSasParameters {
    #[doc = "The signed services accessible with the account SAS. Possible values include: Blob (b), Queue (q), Table (t), File (f)."]
    #[serde(rename = "signedServices")]
    pub signed_services: account_sas_parameters::SignedServices,
    #[doc = "The signed resource types that are accessible with the account SAS. Service (s): Access to service-level APIs; Container (c): Access to container-level APIs; Object (o): Access to object-level APIs for blobs, queue messages, table entities, and files."]
    #[serde(rename = "signedResourceTypes")]
    pub signed_resource_types: account_sas_parameters::SignedResourceTypes,
    #[doc = "The signed permissions for the account SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p)."]
    #[serde(rename = "signedPermission")]
    pub signed_permission: account_sas_parameters::SignedPermission,
    #[doc = "An IP address or a range of IP addresses from which to accept requests."]
    #[serde(rename = "signedIp", default, skip_serializing_if = "Option::is_none")]
    pub signed_ip: Option<String>,
    #[doc = "The protocol permitted for a request made with the account SAS."]
    #[serde(rename = "signedProtocol", default, skip_serializing_if = "Option::is_none")]
    pub signed_protocol: Option<account_sas_parameters::SignedProtocol>,
    #[doc = "The time at which the SAS becomes valid."]
    #[serde(rename = "signedStart", with = "azure_core::date::rfc3339::option")]
    pub signed_start: Option<time::OffsetDateTime>,
    #[doc = "The time at which the shared access signature becomes invalid."]
    #[serde(rename = "signedExpiry", with = "azure_core::date::rfc3339")]
    pub signed_expiry: time::OffsetDateTime,
    #[doc = "The key to sign the account SAS token with."]
    #[serde(rename = "keyToSign", default, skip_serializing_if = "Option::is_none")]
    pub key_to_sign: Option<String>,
}
impl AccountSasParameters {
    pub fn new(
        signed_services: account_sas_parameters::SignedServices,
        signed_resource_types: account_sas_parameters::SignedResourceTypes,
        signed_permission: account_sas_parameters::SignedPermission,
        signed_expiry: time::OffsetDateTime,
    ) -> Self {
        Self {
            signed_services,
            signed_resource_types,
            signed_permission,
            signed_ip: None,
            signed_protocol: None,
            signed_start: None,
            signed_expiry,
            key_to_sign: None,
        }
    }
}
pub mod account_sas_parameters {
    use super::*;
    #[doc = "The signed services accessible with the account SAS. Possible values include: Blob (b), Queue (q), Table (t), File (f)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SignedServices")]
    pub enum SignedServices {
        #[serde(rename = "b")]
        B,
        #[serde(rename = "q")]
        Q,
        #[serde(rename = "t")]
        T,
        #[serde(rename = "f")]
        F,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SignedServices {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SignedServices {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SignedServices {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::B => serializer.serialize_unit_variant("SignedServices", 0u32, "b"),
                Self::Q => serializer.serialize_unit_variant("SignedServices", 1u32, "q"),
                Self::T => serializer.serialize_unit_variant("SignedServices", 2u32, "t"),
                Self::F => serializer.serialize_unit_variant("SignedServices", 3u32, "f"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The signed resource types that are accessible with the account SAS. Service (s): Access to service-level APIs; Container (c): Access to container-level APIs; Object (o): Access to object-level APIs for blobs, queue messages, table entities, and files."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SignedResourceTypes")]
    pub enum SignedResourceTypes {
        #[serde(rename = "s")]
        S,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "o")]
        O,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SignedResourceTypes {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SignedResourceTypes {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SignedResourceTypes {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::S => serializer.serialize_unit_variant("SignedResourceTypes", 0u32, "s"),
                Self::C => serializer.serialize_unit_variant("SignedResourceTypes", 1u32, "c"),
                Self::O => serializer.serialize_unit_variant("SignedResourceTypes", 2u32, "o"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The signed permissions for the account SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SignedPermission")]
    pub enum SignedPermission {
        #[serde(rename = "r")]
        R,
        #[serde(rename = "d")]
        D,
        #[serde(rename = "w")]
        W,
        #[serde(rename = "l")]
        L,
        #[serde(rename = "a")]
        A,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "u")]
        U,
        #[serde(rename = "p")]
        P,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SignedPermission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SignedPermission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SignedPermission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::R => serializer.serialize_unit_variant("SignedPermission", 0u32, "r"),
                Self::D => serializer.serialize_unit_variant("SignedPermission", 1u32, "d"),
                Self::W => serializer.serialize_unit_variant("SignedPermission", 2u32, "w"),
                Self::L => serializer.serialize_unit_variant("SignedPermission", 3u32, "l"),
                Self::A => serializer.serialize_unit_variant("SignedPermission", 4u32, "a"),
                Self::C => serializer.serialize_unit_variant("SignedPermission", 5u32, "c"),
                Self::U => serializer.serialize_unit_variant("SignedPermission", 6u32, "u"),
                Self::P => serializer.serialize_unit_variant("SignedPermission", 7u32, "p"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The protocol permitted for a request made with the account SAS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedProtocol {
        #[serde(rename = "https,http")]
        HttpsHttp,
        #[serde(rename = "https")]
        Https,
    }
}
#[doc = "Settings properties for Active Directory (AD)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryProperties {
    #[doc = "Specifies the primary domain that the AD DNS server is authoritative for."]
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[doc = "Specifies the NetBIOS domain name."]
    #[serde(rename = "netBiosDomainName")]
    pub net_bios_domain_name: String,
    #[doc = "Specifies the Active Directory forest to get."]
    #[serde(rename = "forestName")]
    pub forest_name: String,
    #[doc = "Specifies the domain GUID."]
    #[serde(rename = "domainGuid")]
    pub domain_guid: String,
    #[doc = "Specifies the security identifier (SID)."]
    #[serde(rename = "domainSid")]
    pub domain_sid: String,
    #[doc = "Specifies the security identifier (SID) for Azure Storage."]
    #[serde(rename = "azureStorageSid")]
    pub azure_storage_sid: String,
}
impl ActiveDirectoryProperties {
    pub fn new(
        domain_name: String,
        net_bios_domain_name: String,
        forest_name: String,
        domain_guid: String,
        domain_sid: String,
        azure_storage_sid: String,
    ) -> Self {
        Self {
            domain_name,
            net_bios_domain_name,
            forest_name,
            domain_guid,
            domain_sid,
            azure_storage_sid,
        }
    }
}
#[doc = "Settings for Azure Files identity based authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFilesIdentityBasedAuthentication {
    #[doc = "Indicates the directory service used."]
    #[serde(rename = "directoryServiceOptions")]
    pub directory_service_options: azure_files_identity_based_authentication::DirectoryServiceOptions,
    #[doc = "Settings properties for Active Directory (AD)."]
    #[serde(rename = "activeDirectoryProperties", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_properties: Option<ActiveDirectoryProperties>,
}
impl AzureFilesIdentityBasedAuthentication {
    pub fn new(directory_service_options: azure_files_identity_based_authentication::DirectoryServiceOptions) -> Self {
        Self {
            directory_service_options,
            active_directory_properties: None,
        }
    }
}
pub mod azure_files_identity_based_authentication {
    use super::*;
    #[doc = "Indicates the directory service used."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DirectoryServiceOptions")]
    pub enum DirectoryServiceOptions {
        None,
        #[serde(rename = "AADDS")]
        Aadds,
        #[serde(rename = "AD")]
        Ad,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DirectoryServiceOptions {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DirectoryServiceOptions {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DirectoryServiceOptions {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("DirectoryServiceOptions", 0u32, "None"),
                Self::Aadds => serializer.serialize_unit_variant("DirectoryServiceOptions", 1u32, "AADDS"),
                Self::Ad => serializer.serialize_unit_variant("DirectoryServiceOptions", 2u32, "AD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The storage account blob inventory policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobInventoryPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The storage account blob inventory policy properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlobInventoryPolicyProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl BlobInventoryPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that defines the blob inventory rule. Each definition consists of a set of filters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobInventoryPolicyDefinition {
    #[doc = "An object that defines the blob inventory rule filter conditions."]
    pub filters: BlobInventoryPolicyFilter,
}
impl BlobInventoryPolicyDefinition {
    pub fn new(filters: BlobInventoryPolicyFilter) -> Self {
        Self { filters }
    }
}
#[doc = "An object that defines the blob inventory rule filter conditions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobInventoryPolicyFilter {
    #[doc = "An array of strings for blob prefixes to be matched."]
    #[serde(rename = "prefixMatch", default, skip_serializing_if = "Vec::is_empty")]
    pub prefix_match: Vec<String>,
    #[doc = "An array of predefined enum values. Valid values include blockBlob, appendBlob, pageBlob. Hns accounts does not support pageBlobs."]
    #[serde(rename = "blobTypes")]
    pub blob_types: Vec<String>,
    #[doc = "Includes blob versions in blob inventory when value set to true."]
    #[serde(rename = "includeBlobVersions", default, skip_serializing_if = "Option::is_none")]
    pub include_blob_versions: Option<bool>,
    #[doc = "Includes blob snapshots in blob inventory when value set to true."]
    #[serde(rename = "includeSnapshots", default, skip_serializing_if = "Option::is_none")]
    pub include_snapshots: Option<bool>,
}
impl BlobInventoryPolicyFilter {
    pub fn new(blob_types: Vec<String>) -> Self {
        Self {
            prefix_match: Vec::new(),
            blob_types,
            include_blob_versions: None,
            include_snapshots: None,
        }
    }
}
#[doc = "The storage account blob inventory policy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobInventoryPolicyProperties {
    #[doc = "Returns the last modified date and time of the blob inventory policy."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The storage account blob inventory policy rules."]
    pub policy: BlobInventoryPolicySchema,
}
impl BlobInventoryPolicyProperties {
    pub fn new(policy: BlobInventoryPolicySchema) -> Self {
        Self {
            last_modified_time: None,
            policy,
        }
    }
}
#[doc = "An object that wraps the blob inventory rule. Each rule is uniquely defined by name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobInventoryPolicyRule {
    #[doc = "Rule is enabled when set to true."]
    pub enabled: bool,
    #[doc = "A rule name can contain any combination of alpha numeric characters. Rule name is case-sensitive. It must be unique within a policy."]
    pub name: String,
    #[doc = "An object that defines the blob inventory rule. Each definition consists of a set of filters."]
    pub definition: BlobInventoryPolicyDefinition,
}
impl BlobInventoryPolicyRule {
    pub fn new(enabled: bool, name: String, definition: BlobInventoryPolicyDefinition) -> Self {
        Self { enabled, name, definition }
    }
}
#[doc = "The storage account blob inventory policy rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobInventoryPolicySchema {
    #[doc = "Policy is enabled if set to true."]
    pub enabled: bool,
    #[doc = "Container name where blob inventory files are stored. Must be pre-created."]
    pub destination: String,
    #[doc = "The valid value is Inventory"]
    #[serde(rename = "type")]
    pub type_: blob_inventory_policy_schema::Type,
    #[doc = "The storage account blob inventory policy rules. The rule is applied when it is enabled."]
    pub rules: Vec<BlobInventoryPolicyRule>,
}
impl BlobInventoryPolicySchema {
    pub fn new(enabled: bool, destination: String, type_: blob_inventory_policy_schema::Type, rules: Vec<BlobInventoryPolicyRule>) -> Self {
        Self {
            enabled,
            destination,
            type_,
            rules,
        }
    }
}
pub mod blob_inventory_policy_schema {
    use super::*;
    #[doc = "The valid value is Inventory"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Inventory,
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
                Self::Inventory => serializer.serialize_unit_variant("Type", 0u32, "Inventory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Blob restore parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobRestoreParameters {
    #[doc = "Restore blob to the specified time."]
    #[serde(rename = "timeToRestore", with = "azure_core::date::rfc3339")]
    pub time_to_restore: time::OffsetDateTime,
    #[doc = "Blob ranges to restore."]
    #[serde(rename = "blobRanges")]
    pub blob_ranges: Vec<BlobRestoreRange>,
}
impl BlobRestoreParameters {
    pub fn new(time_to_restore: time::OffsetDateTime, blob_ranges: Vec<BlobRestoreRange>) -> Self {
        Self {
            time_to_restore,
            blob_ranges,
        }
    }
}
#[doc = "Blob range"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobRestoreRange {
    #[doc = "Blob start range. This is inclusive. Empty means account start."]
    #[serde(rename = "startRange")]
    pub start_range: String,
    #[doc = "Blob end range. This is exclusive. Empty means account end."]
    #[serde(rename = "endRange")]
    pub end_range: String,
}
impl BlobRestoreRange {
    pub fn new(start_range: String, end_range: String) -> Self {
        Self { start_range, end_range }
    }
}
#[doc = "Blob restore status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobRestoreStatus {
    #[doc = "The status of blob restore progress. Possible values are: - InProgress: Indicates that blob restore is ongoing. - Complete: Indicates that blob restore has been completed successfully. - Failed: Indicates that blob restore is failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<blob_restore_status::Status>,
    #[doc = "Failure reason when blob restore is failed."]
    #[serde(rename = "failureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[doc = "Id for tracking blob restore request."]
    #[serde(rename = "restoreId", default, skip_serializing_if = "Option::is_none")]
    pub restore_id: Option<String>,
    #[doc = "Blob restore parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BlobRestoreParameters>,
}
impl BlobRestoreStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod blob_restore_status {
    use super::*;
    #[doc = "The status of blob restore progress. Possible values are: - InProgress: Indicates that blob restore is ongoing. - Complete: Indicates that blob restore has been completed successfully. - Failed: Indicates that blob restore is failed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        InProgress,
        Complete,
        Failed,
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
                Self::InProgress => serializer.serialize_unit_variant("Status", 0u32, "InProgress"),
                Self::Complete => serializer.serialize_unit_variant("Status", 1u32, "Complete"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Gets a boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken or is invalid and cannot be used."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Gets the reason that a storage account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "Gets an error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "Gets the reason that a storage account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        AccountNameInvalid,
        AlreadyExists,
    }
}
#[doc = "The custom domain assigned to this storage account. This can be set via Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    #[doc = "Gets or sets the custom domain name assigned to the storage account. Name is the CNAME source."]
    pub name: String,
    #[doc = "Indicates whether indirect CName validation is enabled. Default value is false. This should only be set on updates."]
    #[serde(rename = "useSubDomainName", default, skip_serializing_if = "Option::is_none")]
    pub use_sub_domain_name: Option<bool>,
}
impl CustomDomain {
    pub fn new(name: String) -> Self {
        Self {
            name,
            use_sub_domain_name: None,
        }
    }
}
#[doc = "Object to define the number of days after creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateAfterCreation {
    #[doc = "Value indicating the age in days after creation"]
    #[serde(rename = "daysAfterCreationGreaterThan")]
    pub days_after_creation_greater_than: f64,
}
impl DateAfterCreation {
    pub fn new(days_after_creation_greater_than: f64) -> Self {
        Self {
            days_after_creation_greater_than,
        }
    }
}
#[doc = "Object to define the number of days after object last modification Or last access. Properties daysAfterModificationGreaterThan and daysAfterLastAccessTimeGreaterThan are mutually exclusive."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DateAfterModification {
    #[doc = "Value indicating the age in days after last modification"]
    #[serde(rename = "daysAfterModificationGreaterThan", default, skip_serializing_if = "Option::is_none")]
    pub days_after_modification_greater_than: Option<f64>,
    #[doc = "Value indicating the age in days after last blob access. This property can only be used in conjunction with last access time tracking policy"]
    #[serde(rename = "daysAfterLastAccessTimeGreaterThan", default, skip_serializing_if = "Option::is_none")]
    pub days_after_last_access_time_greater_than: Option<f64>,
}
impl DateAfterModification {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The encryption settings on the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    #[doc = "A list of services that support encryption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<EncryptionServices>,
    #[doc = "The encryption keySource (provider). Possible values (case-insensitive):  Microsoft.Storage, Microsoft.Keyvault"]
    #[serde(rename = "keySource")]
    pub key_source: encryption::KeySource,
    #[doc = "A boolean indicating whether or not the service applies a secondary layer of encryption with platform managed keys for data at rest."]
    #[serde(rename = "requireInfrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
    pub require_infrastructure_encryption: Option<bool>,
    #[doc = "Properties of key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyvaultproperties: Option<KeyVaultProperties>,
}
impl Encryption {
    pub fn new(key_source: encryption::KeySource) -> Self {
        Self {
            services: None,
            key_source,
            require_infrastructure_encryption: None,
            keyvaultproperties: None,
        }
    }
}
pub mod encryption {
    use super::*;
    #[doc = "The encryption keySource (provider). Possible values (case-insensitive):  Microsoft.Storage, Microsoft.Keyvault"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeySource")]
    pub enum KeySource {
        #[serde(rename = "Microsoft.Storage")]
        MicrosoftStorage,
        #[serde(rename = "Microsoft.Keyvault")]
        MicrosoftKeyvault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeySource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeySource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeySource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftStorage => serializer.serialize_unit_variant("KeySource", 0u32, "Microsoft.Storage"),
                Self::MicrosoftKeyvault => serializer.serialize_unit_variant("KeySource", 1u32, "Microsoft.Keyvault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for KeySource {
        fn default() -> Self {
            Self::MicrosoftStorage
        }
    }
}
#[doc = "The Encryption Scope resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionScope {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the encryption scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EncryptionScopeProperties>,
}
impl EncryptionScope {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key vault properties for the encryption scope. This is a required field if encryption scope 'source' attribute is set to 'Microsoft.KeyVault'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionScopeKeyVaultProperties {
    #[doc = "The object identifier for a key vault key object. When applied, the encryption scope will use the key referenced by the identifier to enable customer-managed key support on this encryption scope."]
    #[serde(rename = "keyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_uri: Option<String>,
}
impl EncryptionScopeKeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of encryption scopes requested, and if paging is required, a URL to the next page of encryption scopes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionScopeListResult {
    #[doc = "List of encryption scopes requested."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EncryptionScope>,
    #[doc = "Request URL that can be used to query next page of encryption scopes. Returned when total number of requested encryption scopes exceeds the maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EncryptionScopeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EncryptionScopeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the encryption scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionScopeProperties {
    #[doc = "The provider for the encryption scope. Possible values (case-insensitive):  Microsoft.Storage, Microsoft.KeyVault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<encryption_scope_properties::Source>,
    #[doc = "The state of the encryption scope. Possible values (case-insensitive):  Enabled, Disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<encryption_scope_properties::State>,
    #[doc = "Gets the creation date and time of the encryption scope in UTC."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the last modification date and time of the encryption scope in UTC."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The key vault properties for the encryption scope. This is a required field if encryption scope 'source' attribute is set to 'Microsoft.KeyVault'."]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<EncryptionScopeKeyVaultProperties>,
}
impl EncryptionScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_scope_properties {
    use super::*;
    #[doc = "The provider for the encryption scope. Possible values (case-insensitive):  Microsoft.Storage, Microsoft.KeyVault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        #[serde(rename = "Microsoft.Storage")]
        MicrosoftStorage,
        #[serde(rename = "Microsoft.KeyVault")]
        MicrosoftKeyVault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Source {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Source {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Source {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftStorage => serializer.serialize_unit_variant("Source", 0u32, "Microsoft.Storage"),
                Self::MicrosoftKeyVault => serializer.serialize_unit_variant("Source", 1u32, "Microsoft.KeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state of the encryption scope. Possible values (case-insensitive):  Enabled, Disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("State", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("State", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A service that allows server-side encryption to be used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionService {
    #[doc = "A boolean indicating whether or not the service encrypts the data as it is stored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Gets a rough estimate of the date/time when the encryption was last enabled by the user. Only returned when encryption is enabled. There might be some unencrypted blobs which were written after this time, as it is just a rough estimate."]
    #[serde(rename = "lastEnabledTime", with = "azure_core::date::rfc3339::option")]
    pub last_enabled_time: Option<time::OffsetDateTime>,
    #[doc = "Encryption key type to be used for the encryption service. 'Account' key type implies that an account-scoped encryption key will be used. 'Service' key type implies that a default service key is used."]
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<encryption_service::KeyType>,
}
impl EncryptionService {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_service {
    use super::*;
    #[doc = "Encryption key type to be used for the encryption service. 'Account' key type implies that an account-scoped encryption key will be used. 'Service' key type implies that a default service key is used."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyType")]
    pub enum KeyType {
        Service,
        Account,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Service => serializer.serialize_unit_variant("KeyType", 0u32, "Service"),
                Self::Account => serializer.serialize_unit_variant("KeyType", 1u32, "Account"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of services that support encryption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionServices {
    #[doc = "A service that allows server-side encryption to be used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<EncryptionService>,
    #[doc = "A service that allows server-side encryption to be used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<EncryptionService>,
    #[doc = "A service that allows server-side encryption to be used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<EncryptionService>,
    #[doc = "A service that allows server-side encryption to be used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<EncryptionService>,
}
impl EncryptionServices {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The URIs that are used to perform a retrieval of a public blob, queue, table, web or dfs object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Endpoints {
    #[doc = "Gets the blob endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[doc = "Gets the queue endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[doc = "Gets the table endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "Gets the file endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[doc = "Gets the web endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
    #[doc = "Gets the dfs endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dfs: Option<String>,
    #[doc = "The URIs that are used to perform a retrieval of a public blob, queue, table, web or dfs object via a microsoft routing endpoint."]
    #[serde(rename = "microsoftEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_endpoints: Option<StorageAccountMicrosoftEndpoints>,
    #[doc = "The URIs that are used to perform a retrieval of a public blob, file, web or dfs object via a internet routing endpoint."]
    #[serde(rename = "internetEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub internet_endpoints: Option<StorageAccountInternetEndpoints>,
}
impl Endpoints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the storage resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error response body contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
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
#[doc = "Error response body contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Statistics related to replication for storage account's Blob, Table, Queue and File services. It is only available when geo-redundant replication is enabled for the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GeoReplicationStats {
    #[doc = "The status of the secondary location. Possible values are: - Live: Indicates that the secondary location is active and operational. - Bootstrap: Indicates initial synchronization from the primary location to the secondary location is in progress.This typically occurs when replication is first enabled. - Unavailable: Indicates that the secondary location is temporarily unavailable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<geo_replication_stats::Status>,
    #[doc = "All primary writes preceding this UTC date/time value are guaranteed to be available for read operations. Primary writes following this point in time may or may not be available for reads. Element may be default value if value of LastSyncTime is not available, this can happen if secondary is offline or we are in bootstrap."]
    #[serde(rename = "lastSyncTime", with = "azure_core::date::rfc3339::option")]
    pub last_sync_time: Option<time::OffsetDateTime>,
    #[doc = "A boolean flag which indicates whether or not account failover is supported for the account."]
    #[serde(rename = "canFailover", default, skip_serializing_if = "Option::is_none")]
    pub can_failover: Option<bool>,
}
impl GeoReplicationStats {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod geo_replication_stats {
    use super::*;
    #[doc = "The status of the secondary location. Possible values are: - Live: Indicates that the secondary location is active and operational. - Bootstrap: Indicates initial synchronization from the primary location to the secondary location is in progress.This typically occurs when replication is first enabled. - Unavailable: Indicates that the secondary location is temporarily unavailable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Live,
        Bootstrap,
        Unavailable,
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
                Self::Live => serializer.serialize_unit_variant("Status", 0u32, "Live"),
                Self::Bootstrap => serializer.serialize_unit_variant("Status", 1u32, "Bootstrap"),
                Self::Unavailable => serializer.serialize_unit_variant("Status", 2u32, "Unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "IP rule with specific IP or IP range in CIDR format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    #[doc = "Specifies the IP or IP range in CIDR format. Only IPV4 address is allowed."]
    pub value: String,
    #[doc = "The action of IP ACL rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ip_rule::Action>,
}
impl IpRule {
    pub fn new(value: String) -> Self {
        Self { value, action: None }
    }
}
pub mod ip_rule {
    use super::*;
    #[doc = "The action of IP ACL rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
    }
    impl Default for Action {
        fn default() -> Self {
            Self::Allow
        }
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type")]
    pub type_: identity::Type,
}
impl Identity {
    pub fn new(type_: identity::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[doc = "Properties of key vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultProperties {
    #[doc = "The name of KeyVault key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyname: Option<String>,
    #[doc = "The version of KeyVault key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyversion: Option<String>,
    #[doc = "The Uri of KeyVault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyvaulturi: Option<String>,
    #[doc = "The object identifier of the current versioned Key Vault Key in use."]
    #[serde(rename = "currentVersionedKeyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub current_versioned_key_identifier: Option<String>,
    #[doc = "Timestamp of last rotation of the Key Vault Key."]
    #[serde(rename = "lastKeyRotationTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_key_rotation_timestamp: Option<time::OffsetDateTime>,
}
impl KeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List SAS credentials operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListAccountSasResponse {
    #[doc = "List SAS credentials of storage account."]
    #[serde(rename = "accountSasToken", default, skip_serializing_if = "Option::is_none")]
    pub account_sas_token: Option<String>,
}
impl ListAccountSasResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of blob inventory policies returned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListBlobInventoryPolicy {
    #[doc = "List of blob inventory policies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BlobInventoryPolicy>,
}
impl azure_core::Continuable for ListBlobInventoryPolicy {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListBlobInventoryPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List service SAS credentials operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListServiceSasResponse {
    #[doc = "List service SAS credentials of specific resource."]
    #[serde(rename = "serviceSasToken", default, skip_serializing_if = "Option::is_none")]
    pub service_sas_token: Option<String>,
}
impl ListServiceSasResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Get Storage Account ManagementPolicies operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The Storage Account ManagementPolicy properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementPolicyProperties>,
}
impl ManagementPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Actions are applied to the filtered blobs when the execution condition is met."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementPolicyAction {
    #[doc = "Management policy action for base blob."]
    #[serde(rename = "baseBlob", default, skip_serializing_if = "Option::is_none")]
    pub base_blob: Option<ManagementPolicyBaseBlob>,
    #[doc = "Management policy action for snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<ManagementPolicySnapShot>,
    #[doc = "Management policy action for blob version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ManagementPolicyVersion>,
}
impl ManagementPolicyAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Management policy action for base blob."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementPolicyBaseBlob {
    #[doc = "Object to define the number of days after object last modification Or last access. Properties daysAfterModificationGreaterThan and daysAfterLastAccessTimeGreaterThan are mutually exclusive."]
    #[serde(rename = "tierToCool", default, skip_serializing_if = "Option::is_none")]
    pub tier_to_cool: Option<DateAfterModification>,
    #[doc = "Object to define the number of days after object last modification Or last access. Properties daysAfterModificationGreaterThan and daysAfterLastAccessTimeGreaterThan are mutually exclusive."]
    #[serde(rename = "tierToArchive", default, skip_serializing_if = "Option::is_none")]
    pub tier_to_archive: Option<DateAfterModification>,
    #[doc = "Object to define the number of days after object last modification Or last access. Properties daysAfterModificationGreaterThan and daysAfterLastAccessTimeGreaterThan are mutually exclusive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<DateAfterModification>,
    #[doc = "This property enables auto tiering of a blob from cool to hot on a blob access. This property requires tierToCool.daysAfterLastAccessTimeGreaterThan."]
    #[serde(rename = "enableAutoTierToHotFromCool", default, skip_serializing_if = "Option::is_none")]
    pub enable_auto_tier_to_hot_from_cool: Option<bool>,
}
impl ManagementPolicyBaseBlob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that defines the Lifecycle rule. Each definition is made up with a filters set and an actions set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementPolicyDefinition {
    #[doc = "Actions are applied to the filtered blobs when the execution condition is met."]
    pub actions: ManagementPolicyAction,
    #[doc = "Filters limit rule actions to a subset of blobs within the storage account. If multiple filters are defined, a logical AND is performed on all filters. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<ManagementPolicyFilter>,
}
impl ManagementPolicyDefinition {
    pub fn new(actions: ManagementPolicyAction) -> Self {
        Self { actions, filters: None }
    }
}
#[doc = "Filters limit rule actions to a subset of blobs within the storage account. If multiple filters are defined, a logical AND is performed on all filters. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementPolicyFilter {
    #[doc = "An array of strings for prefixes to be match."]
    #[serde(rename = "prefixMatch", default, skip_serializing_if = "Vec::is_empty")]
    pub prefix_match: Vec<String>,
    #[doc = "An array of predefined enum values. Currently blockBlob supports all tiering and delete actions. Only delete actions are supported for appendBlob."]
    #[serde(rename = "blobTypes")]
    pub blob_types: Vec<String>,
    #[doc = "An array of blob index tag based filters, there can be at most 10 tag filters"]
    #[serde(rename = "blobIndexMatch", default, skip_serializing_if = "Vec::is_empty")]
    pub blob_index_match: Vec<TagFilter>,
}
impl ManagementPolicyFilter {
    pub fn new(blob_types: Vec<String>) -> Self {
        Self {
            prefix_match: Vec::new(),
            blob_types,
            blob_index_match: Vec::new(),
        }
    }
}
#[doc = "The Storage Account ManagementPolicy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementPolicyProperties {
    #[doc = "Returns the date and time the ManagementPolicies was last modified."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The Storage Account ManagementPolicies Rules. See more details in: https://docs.microsoft.com/en-us/azure/storage/common/storage-lifecycle-managment-concepts."]
    pub policy: ManagementPolicySchema,
}
impl ManagementPolicyProperties {
    pub fn new(policy: ManagementPolicySchema) -> Self {
        Self {
            last_modified_time: None,
            policy,
        }
    }
}
#[doc = "An object that wraps the Lifecycle rule. Each rule is uniquely defined by name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementPolicyRule {
    #[doc = "Rule is enabled if set to true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "A rule name can contain any combination of alpha numeric characters. Rule name is case-sensitive. It must be unique within a policy."]
    pub name: String,
    #[doc = "The valid value is Lifecycle"]
    #[serde(rename = "type")]
    pub type_: management_policy_rule::Type,
    #[doc = "An object that defines the Lifecycle rule. Each definition is made up with a filters set and an actions set."]
    pub definition: ManagementPolicyDefinition,
}
impl ManagementPolicyRule {
    pub fn new(name: String, type_: management_policy_rule::Type, definition: ManagementPolicyDefinition) -> Self {
        Self {
            enabled: None,
            name,
            type_,
            definition,
        }
    }
}
pub mod management_policy_rule {
    use super::*;
    #[doc = "The valid value is Lifecycle"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Lifecycle,
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
                Self::Lifecycle => serializer.serialize_unit_variant("Type", 0u32, "Lifecycle"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Storage Account ManagementPolicies Rules. See more details in: https://docs.microsoft.com/en-us/azure/storage/common/storage-lifecycle-managment-concepts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementPolicySchema {
    #[doc = "The Storage Account ManagementPolicies Rules. See more details in: https://docs.microsoft.com/en-us/azure/storage/common/storage-lifecycle-managment-concepts."]
    pub rules: Vec<ManagementPolicyRule>,
}
impl ManagementPolicySchema {
    pub fn new(rules: Vec<ManagementPolicyRule>) -> Self {
        Self { rules }
    }
}
#[doc = "Management policy action for snapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementPolicySnapShot {
    #[doc = "Object to define the number of days after creation."]
    #[serde(rename = "tierToCool", default, skip_serializing_if = "Option::is_none")]
    pub tier_to_cool: Option<DateAfterCreation>,
    #[doc = "Object to define the number of days after creation."]
    #[serde(rename = "tierToArchive", default, skip_serializing_if = "Option::is_none")]
    pub tier_to_archive: Option<DateAfterCreation>,
    #[doc = "Object to define the number of days after creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<DateAfterCreation>,
}
impl ManagementPolicySnapShot {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Management policy action for blob version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementPolicyVersion {
    #[doc = "Object to define the number of days after creation."]
    #[serde(rename = "tierToCool", default, skip_serializing_if = "Option::is_none")]
    pub tier_to_cool: Option<DateAfterCreation>,
    #[doc = "Object to define the number of days after creation."]
    #[serde(rename = "tierToArchive", default, skip_serializing_if = "Option::is_none")]
    pub tier_to_archive: Option<DateAfterCreation>,
    #[doc = "Object to define the number of days after creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<DateAfterCreation>,
}
impl ManagementPolicyVersion {
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
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network rule set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    #[doc = "Specifies whether traffic is bypassed for Logging/Metrics/AzureServices. Possible values are any combination of Logging|Metrics|AzureServices (For example, \"Logging, Metrics\"), or None to bypass none of those traffics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass: Option<network_rule_set::Bypass>,
    #[doc = "Sets the virtual network rules"]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[doc = "Sets the IP ACL rules"]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
    #[doc = "Specifies the default action of allow or deny when no other rules match."]
    #[serde(rename = "defaultAction")]
    pub default_action: network_rule_set::DefaultAction,
}
impl NetworkRuleSet {
    pub fn new(default_action: network_rule_set::DefaultAction) -> Self {
        Self {
            bypass: None,
            virtual_network_rules: Vec::new(),
            ip_rules: Vec::new(),
            default_action,
        }
    }
}
pub mod network_rule_set {
    use super::*;
    #[doc = "Specifies whether traffic is bypassed for Logging/Metrics/AzureServices. Possible values are any combination of Logging|Metrics|AzureServices (For example, \"Logging, Metrics\"), or None to bypass none of those traffics."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Bypass")]
    pub enum Bypass {
        None,
        Logging,
        Metrics,
        AzureServices,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Bypass {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Bypass {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Bypass {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Bypass", 0u32, "None"),
                Self::Logging => serializer.serialize_unit_variant("Bypass", 1u32, "Logging"),
                Self::Metrics => serializer.serialize_unit_variant("Bypass", 2u32, "Metrics"),
                Self::AzureServices => serializer.serialize_unit_variant("Bypass", 3u32, "AzureServices"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Bypass {
        fn default() -> Self {
            Self::AzureServices
        }
    }
    #[doc = "Specifies the default action of allow or deny when no other rules match."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultAction {
        Allow,
        Deny,
    }
    impl Default for DefaultAction {
        fn default() -> Self {
            Self::Allow
        }
    }
}
#[doc = "List storage account object replication policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectReplicationPolicies {
    #[doc = "The replication policy between two storage accounts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ObjectReplicationPolicy>,
}
impl azure_core::Continuable for ObjectReplicationPolicies {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ObjectReplicationPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The replication policy between two storage accounts. Multiple rules can be defined in one policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectReplicationPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The Storage Account ObjectReplicationPolicy properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ObjectReplicationPolicyProperties>,
}
impl ObjectReplicationPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Filters limit replication to a subset of blobs within the storage account. A logical OR is performed on values in the filter. If multiple filters are defined, a logical AND is performed on all filters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectReplicationPolicyFilter {
    #[doc = "Optional. Filters the results to replicate only blobs whose names begin with the specified prefix."]
    #[serde(rename = "prefixMatch", default, skip_serializing_if = "Vec::is_empty")]
    pub prefix_match: Vec<String>,
    #[doc = "Blobs created after the time will be replicated to the destination. It must be in datetime format 'yyyy-MM-ddTHH:mm:ssZ'. Example: 2020-02-19T16:05:00Z"]
    #[serde(rename = "minCreationTime", default, skip_serializing_if = "Option::is_none")]
    pub min_creation_time: Option<String>,
}
impl ObjectReplicationPolicyFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Storage Account ObjectReplicationPolicy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectReplicationPolicyProperties {
    #[doc = "A unique id for object replication policy."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "Indicates when the policy is enabled on the source account."]
    #[serde(rename = "enabledTime", with = "azure_core::date::rfc3339::option")]
    pub enabled_time: Option<time::OffsetDateTime>,
    #[doc = "Required. Source account name."]
    #[serde(rename = "sourceAccount")]
    pub source_account: String,
    #[doc = "Required. Destination account name."]
    #[serde(rename = "destinationAccount")]
    pub destination_account: String,
    #[doc = "The storage account object replication rules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<ObjectReplicationPolicyRule>,
}
impl ObjectReplicationPolicyProperties {
    pub fn new(source_account: String, destination_account: String) -> Self {
        Self {
            policy_id: None,
            enabled_time: None,
            source_account,
            destination_account,
            rules: Vec::new(),
        }
    }
}
#[doc = "The replication policy rule between two containers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectReplicationPolicyRule {
    #[doc = "Rule Id is auto-generated for each new rule on destination account. It is required for put policy on source account."]
    #[serde(rename = "ruleId", default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[doc = "Required. Source container name."]
    #[serde(rename = "sourceContainer")]
    pub source_container: String,
    #[doc = "Required. Destination container name."]
    #[serde(rename = "destinationContainer")]
    pub destination_container: String,
    #[doc = "Filters limit replication to a subset of blobs within the storage account. A logical OR is performed on values in the filter. If multiple filters are defined, a logical AND is performed on all filters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<ObjectReplicationPolicyFilter>,
}
impl ObjectReplicationPolicyRule {
    pub fn new(source_container: String, destination_container: String) -> Self {
        Self {
            rule_id: None,
            source_container,
            destination_container,
            filters: None,
        }
    }
}
#[doc = "Storage REST API operation definition."]
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
        #[doc = "Service provider: Microsoft Storage."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Storage operations. It contains a list of operations and a URL link to get the next set of results."]
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
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connection associated with the specified storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionRequired", default, skip_serializing_if = "Option::is_none")]
    pub action_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
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
#[doc = "The restriction because of which SKU cannot be used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Restriction {
    #[doc = "The type of restrictions. As of now only possible value for this is location."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "The reason for the restriction. As of now this can be \"QuotaId\" or \"NotAvailableForSubscription\". Quota Id is set when the SKU has requiredQuotas parameter as the subscription does not belong to that quota. The \"NotAvailableForSubscription\" is related to capacity at DC."]
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
    #[doc = "The reason for the restriction. As of now this can be \"QuotaId\" or \"NotAvailableForSubscription\". Quota Id is set when the SKU has requiredQuotas parameter as the subscription does not belong to that quota. The \"NotAvailableForSubscription\" is related to capacity at DC."]
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
#[doc = "Routing preference defines the type of network, either microsoft or internet routing to be used to deliver the user data, the default option is microsoft routing"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoutingPreference {
    #[doc = "Routing Choice defines the kind of network routing opted by the user."]
    #[serde(rename = "routingChoice", default, skip_serializing_if = "Option::is_none")]
    pub routing_choice: Option<routing_preference::RoutingChoice>,
    #[doc = "A boolean flag which indicates whether microsoft routing storage endpoints are to be published"]
    #[serde(rename = "publishMicrosoftEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub publish_microsoft_endpoints: Option<bool>,
    #[doc = "A boolean flag which indicates whether internet routing storage endpoints are to be published"]
    #[serde(rename = "publishInternetEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub publish_internet_endpoints: Option<bool>,
}
impl RoutingPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod routing_preference {
    use super::*;
    #[doc = "Routing Choice defines the kind of network routing opted by the user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoutingChoice")]
    pub enum RoutingChoice {
        MicrosoftRouting,
        InternetRouting,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoutingChoice {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoutingChoice {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoutingChoice {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftRouting => serializer.serialize_unit_variant("RoutingChoice", 0u32, "MicrosoftRouting"),
                Self::InternetRouting => serializer.serialize_unit_variant("RoutingChoice", 1u32, "InternetRouting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The capability information in the specified SKU, including file encryption, network ACLs, change notification, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapability {
    #[doc = "The name of capability, The capability information in the specified SKU, including file encryption, network ACLs, change notification, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A string value to indicate states of given capability. Possibly 'true' or 'false'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters to list service SAS credentials of a specific resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSasParameters {
    #[doc = "The canonical path to the signed resource."]
    #[serde(rename = "canonicalizedResource")]
    pub canonicalized_resource: String,
    #[doc = "The signed services accessible with the service SAS. Possible values include: Blob (b), Container (c), File (f), Share (s)."]
    #[serde(rename = "signedResource", default, skip_serializing_if = "Option::is_none")]
    pub signed_resource: Option<service_sas_parameters::SignedResource>,
    #[doc = "The signed permissions for the service SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p)."]
    #[serde(rename = "signedPermission", default, skip_serializing_if = "Option::is_none")]
    pub signed_permission: Option<service_sas_parameters::SignedPermission>,
    #[doc = "An IP address or a range of IP addresses from which to accept requests."]
    #[serde(rename = "signedIp", default, skip_serializing_if = "Option::is_none")]
    pub signed_ip: Option<String>,
    #[doc = "The protocol permitted for a request made with the account SAS."]
    #[serde(rename = "signedProtocol", default, skip_serializing_if = "Option::is_none")]
    pub signed_protocol: Option<service_sas_parameters::SignedProtocol>,
    #[doc = "The time at which the SAS becomes valid."]
    #[serde(rename = "signedStart", with = "azure_core::date::rfc3339::option")]
    pub signed_start: Option<time::OffsetDateTime>,
    #[doc = "The time at which the shared access signature becomes invalid."]
    #[serde(rename = "signedExpiry", with = "azure_core::date::rfc3339::option")]
    pub signed_expiry: Option<time::OffsetDateTime>,
    #[doc = "A unique value up to 64 characters in length that correlates to an access policy specified for the container, queue, or table."]
    #[serde(rename = "signedIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub signed_identifier: Option<String>,
    #[doc = "The start of partition key."]
    #[serde(rename = "startPk", default, skip_serializing_if = "Option::is_none")]
    pub start_pk: Option<String>,
    #[doc = "The end of partition key."]
    #[serde(rename = "endPk", default, skip_serializing_if = "Option::is_none")]
    pub end_pk: Option<String>,
    #[doc = "The start of row key."]
    #[serde(rename = "startRk", default, skip_serializing_if = "Option::is_none")]
    pub start_rk: Option<String>,
    #[doc = "The end of row key."]
    #[serde(rename = "endRk", default, skip_serializing_if = "Option::is_none")]
    pub end_rk: Option<String>,
    #[doc = "The key to sign the account SAS token with."]
    #[serde(rename = "keyToSign", default, skip_serializing_if = "Option::is_none")]
    pub key_to_sign: Option<String>,
    #[doc = "The response header override for cache control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rscc: Option<String>,
    #[doc = "The response header override for content disposition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rscd: Option<String>,
    #[doc = "The response header override for content encoding."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsce: Option<String>,
    #[doc = "The response header override for content language."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rscl: Option<String>,
    #[doc = "The response header override for content type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsct: Option<String>,
}
impl ServiceSasParameters {
    pub fn new(canonicalized_resource: String) -> Self {
        Self {
            canonicalized_resource,
            signed_resource: None,
            signed_permission: None,
            signed_ip: None,
            signed_protocol: None,
            signed_start: None,
            signed_expiry: None,
            signed_identifier: None,
            start_pk: None,
            end_pk: None,
            start_rk: None,
            end_rk: None,
            key_to_sign: None,
            rscc: None,
            rscd: None,
            rsce: None,
            rscl: None,
            rsct: None,
        }
    }
}
pub mod service_sas_parameters {
    use super::*;
    #[doc = "The signed services accessible with the service SAS. Possible values include: Blob (b), Container (c), File (f), Share (s)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SignedResource")]
    pub enum SignedResource {
        #[serde(rename = "b")]
        B,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "f")]
        F,
        #[serde(rename = "s")]
        S,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SignedResource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SignedResource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SignedResource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::B => serializer.serialize_unit_variant("SignedResource", 0u32, "b"),
                Self::C => serializer.serialize_unit_variant("SignedResource", 1u32, "c"),
                Self::F => serializer.serialize_unit_variant("SignedResource", 2u32, "f"),
                Self::S => serializer.serialize_unit_variant("SignedResource", 3u32, "s"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The signed permissions for the service SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SignedPermission")]
    pub enum SignedPermission {
        #[serde(rename = "r")]
        R,
        #[serde(rename = "d")]
        D,
        #[serde(rename = "w")]
        W,
        #[serde(rename = "l")]
        L,
        #[serde(rename = "a")]
        A,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "u")]
        U,
        #[serde(rename = "p")]
        P,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SignedPermission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SignedPermission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SignedPermission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::R => serializer.serialize_unit_variant("SignedPermission", 0u32, "r"),
                Self::D => serializer.serialize_unit_variant("SignedPermission", 1u32, "d"),
                Self::W => serializer.serialize_unit_variant("SignedPermission", 2u32, "w"),
                Self::L => serializer.serialize_unit_variant("SignedPermission", 3u32, "l"),
                Self::A => serializer.serialize_unit_variant("SignedPermission", 4u32, "a"),
                Self::C => serializer.serialize_unit_variant("SignedPermission", 5u32, "c"),
                Self::U => serializer.serialize_unit_variant("SignedPermission", 6u32, "u"),
                Self::P => serializer.serialize_unit_variant("SignedPermission", 7u32, "p"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The protocol permitted for a request made with the account SAS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedProtocol {
        #[serde(rename = "https,http")]
        HttpsHttp,
        #[serde(rename = "https")]
        Https,
    }
}
#[doc = "One property of operation, include metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Metric specifications of operation."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU of the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The SKU name. Required for account creation; optional for update. Note that in older versions, SKU name was called accountType."]
    pub name: SkuName,
    #[doc = "The SKU tier. This is based on the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<Tier>,
}
impl Sku {
    pub fn new(name: SkuName) -> Self {
        Self { name, tier: None }
    }
}
#[doc = "Storage SKU and its properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuInformation {
    #[doc = "The SKU name. Required for account creation; optional for update. Note that in older versions, SKU name was called accountType."]
    pub name: SkuName,
    #[doc = "The SKU tier. This is based on the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<Tier>,
    #[doc = "The type of the resource, usually it is 'storageAccounts'."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Indicates the type of storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<sku_information::Kind>,
    #[doc = "The set of locations that the SKU is available. This will be supported and registered Azure Geo Regions (e.g. West US, East US, Southeast Asia, etc.)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "The capability information in the specified SKU, including file encryption, network ACLs, change notification, etc."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
    #[doc = "The restrictions because of which SKU cannot be used. This is empty if there are no restrictions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<Restriction>,
}
impl SkuInformation {
    pub fn new(name: SkuName) -> Self {
        Self {
            name,
            tier: None,
            resource_type: None,
            kind: None,
            locations: Vec::new(),
            capabilities: Vec::new(),
            restrictions: Vec::new(),
        }
    }
}
pub mod sku_information {
    use super::*;
    #[doc = "Indicates the type of storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Storage,
        StorageV2,
        BlobStorage,
        FileStorage,
        BlockBlobStorage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Storage => serializer.serialize_unit_variant("Kind", 0u32, "Storage"),
                Self::StorageV2 => serializer.serialize_unit_variant("Kind", 1u32, "StorageV2"),
                Self::BlobStorage => serializer.serialize_unit_variant("Kind", 2u32, "BlobStorage"),
                Self::FileStorage => serializer.serialize_unit_variant("Kind", 3u32, "FileStorage"),
                Self::BlockBlobStorage => serializer.serialize_unit_variant("Kind", 4u32, "BlockBlobStorage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The SKU name. Required for account creation; optional for update. Note that in older versions, SKU name was called accountType."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuName")]
pub enum SkuName {
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "Standard_GRS")]
    StandardGrs,
    #[serde(rename = "Standard_RAGRS")]
    StandardRagrs,
    #[serde(rename = "Standard_ZRS")]
    StandardZrs,
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(rename = "Premium_ZRS")]
    PremiumZrs,
    #[serde(rename = "Standard_GZRS")]
    StandardGzrs,
    #[serde(rename = "Standard_RAGZRS")]
    StandardRagzrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StandardLrs => serializer.serialize_unit_variant("SkuName", 0u32, "Standard_LRS"),
            Self::StandardGrs => serializer.serialize_unit_variant("SkuName", 1u32, "Standard_GRS"),
            Self::StandardRagrs => serializer.serialize_unit_variant("SkuName", 2u32, "Standard_RAGRS"),
            Self::StandardZrs => serializer.serialize_unit_variant("SkuName", 3u32, "Standard_ZRS"),
            Self::PremiumLrs => serializer.serialize_unit_variant("SkuName", 4u32, "Premium_LRS"),
            Self::PremiumZrs => serializer.serialize_unit_variant("SkuName", 5u32, "Premium_ZRS"),
            Self::StandardGzrs => serializer.serialize_unit_variant("SkuName", 6u32, "Standard_GZRS"),
            Self::StandardRagzrs => serializer.serialize_unit_variant("SkuName", 7u32, "Standard_RAGZRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The SKU of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Gets the Kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<storage_account::Kind>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Properties of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountProperties>,
}
impl StorageAccount {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            kind: None,
            identity: None,
            properties: None,
        }
    }
}
pub mod storage_account {
    use super::*;
    #[doc = "Gets the Kind."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Storage,
        StorageV2,
        BlobStorage,
        FileStorage,
        BlockBlobStorage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Storage => serializer.serialize_unit_variant("Kind", 0u32, "Storage"),
                Self::StorageV2 => serializer.serialize_unit_variant("Kind", 1u32, "StorageV2"),
                Self::BlobStorage => serializer.serialize_unit_variant("Kind", 2u32, "BlobStorage"),
                Self::FileStorage => serializer.serialize_unit_variant("Kind", 3u32, "FileStorage"),
                Self::BlockBlobStorage => serializer.serialize_unit_variant("Kind", 4u32, "BlockBlobStorage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters used to check the availability of the storage account name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCheckNameAvailabilityParameters {
    #[doc = "The storage account name."]
    pub name: String,
    #[doc = "The type of resource, Microsoft.Storage/storageAccounts"]
    #[serde(rename = "type")]
    pub type_: storage_account_check_name_availability_parameters::Type,
}
impl StorageAccountCheckNameAvailabilityParameters {
    pub fn new(name: String, type_: storage_account_check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod storage_account_check_name_availability_parameters {
    use super::*;
    #[doc = "The type of resource, Microsoft.Storage/storageAccounts"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Storage/storageAccounts")]
        MicrosoftStorageStorageAccounts,
    }
}
#[doc = "The parameters used when creating a storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCreateParameters {
    #[doc = "The SKU of the storage account."]
    pub sku: Sku,
    #[doc = "Required. Indicates the type of storage account."]
    pub kind: storage_account_create_parameters::Kind,
    #[doc = "Required. Gets or sets the location of the resource. This will be one of the supported and registered Azure Geo Regions (e.g. West US, East US, Southeast Asia, etc.). The geo region of a resource cannot be changed once it is created, but if an identical geo region is specified on update, the request will succeed."]
    pub location: String,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used for viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key with a length no greater than 128 characters and a value with a length no greater than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "The parameters used to create the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesCreateParameters>,
}
impl StorageAccountCreateParameters {
    pub fn new(sku: Sku, kind: storage_account_create_parameters::Kind, location: String) -> Self {
        Self {
            sku,
            kind,
            location,
            tags: None,
            identity: None,
            properties: None,
        }
    }
}
pub mod storage_account_create_parameters {
    use super::*;
    #[doc = "Required. Indicates the type of storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Storage,
        StorageV2,
        BlobStorage,
        FileStorage,
        BlockBlobStorage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Storage => serializer.serialize_unit_variant("Kind", 0u32, "Storage"),
                Self::StorageV2 => serializer.serialize_unit_variant("Kind", 1u32, "StorageV2"),
                Self::BlobStorage => serializer.serialize_unit_variant("Kind", 2u32, "BlobStorage"),
                Self::FileStorage => serializer.serialize_unit_variant("Kind", 3u32, "FileStorage"),
                Self::BlockBlobStorage => serializer.serialize_unit_variant("Kind", 4u32, "BlockBlobStorage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The URIs that are used to perform a retrieval of a public blob, file, web or dfs object via a internet routing endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountInternetEndpoints {
    #[doc = "Gets the blob endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[doc = "Gets the file endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[doc = "Gets the web endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
    #[doc = "Gets the dfs endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dfs: Option<String>,
}
impl StorageAccountInternetEndpoints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An access key for the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountKey {
    #[doc = "Name of the key."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "Base 64-encoded value of the key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Permissions for the key -- read-only or full permissions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<storage_account_key::Permissions>,
}
impl StorageAccountKey {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_key {
    use super::*;
    #[doc = "Permissions for the key -- read-only or full permissions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Permissions {
        Read,
        Full,
    }
}
#[doc = "The response from the ListKeys operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountListKeysResult {
    #[doc = "Gets the list of storage account keys and their properties for the specified storage account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<StorageAccountKey>,
}
impl StorageAccountListKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List Storage Accounts operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountListResult {
    #[doc = "Gets the list of storage accounts and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageAccount>,
    #[doc = "Request URL that can be used to query next page of storage accounts. Returned when total number of requested storage accounts exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The URIs that are used to perform a retrieval of a public blob, queue, table, web or dfs object via a microsoft routing endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountMicrosoftEndpoints {
    #[doc = "Gets the blob endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[doc = "Gets the queue endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[doc = "Gets the table endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "Gets the file endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[doc = "Gets the web endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
    #[doc = "Gets the dfs endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dfs: Option<String>,
}
impl StorageAccountMicrosoftEndpoints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountProperties {
    #[doc = "Gets the status of the storage account at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_account_properties::ProvisioningState>,
    #[doc = "The URIs that are used to perform a retrieval of a public blob, queue, table, web or dfs object."]
    #[serde(rename = "primaryEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub primary_endpoints: Option<Endpoints>,
    #[doc = "Gets the location of the primary data center for the storage account."]
    #[serde(rename = "primaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub primary_location: Option<String>,
    #[doc = "Gets the status indicating whether the primary location of the storage account is available or unavailable."]
    #[serde(rename = "statusOfPrimary", default, skip_serializing_if = "Option::is_none")]
    pub status_of_primary: Option<storage_account_properties::StatusOfPrimary>,
    #[doc = "Gets the timestamp of the most recent instance of a failover to the secondary location. Only the most recent timestamp is retained. This element is not returned if there has never been a failover instance. Only available if the accountType is Standard_GRS or Standard_RAGRS."]
    #[serde(rename = "lastGeoFailoverTime", with = "azure_core::date::rfc3339::option")]
    pub last_geo_failover_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the location of the geo-replicated secondary for the storage account. Only available if the accountType is Standard_GRS or Standard_RAGRS."]
    #[serde(rename = "secondaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub secondary_location: Option<String>,
    #[doc = "Gets the status indicating whether the secondary location of the storage account is available or unavailable. Only available if the SKU name is Standard_GRS or Standard_RAGRS."]
    #[serde(rename = "statusOfSecondary", default, skip_serializing_if = "Option::is_none")]
    pub status_of_secondary: Option<storage_account_properties::StatusOfSecondary>,
    #[doc = "Gets the creation date and time of the storage account in UTC."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The custom domain assigned to this storage account. This can be set via Update."]
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[doc = "The URIs that are used to perform a retrieval of a public blob, queue, table, web or dfs object."]
    #[serde(rename = "secondaryEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub secondary_endpoints: Option<Endpoints>,
    #[doc = "The encryption settings on the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[serde(rename = "accessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties::AccessTier>,
    #[doc = "Settings for Azure Files identity based authentication."]
    #[serde(rename = "azureFilesIdentityBasedAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub azure_files_identity_based_authentication: Option<AzureFilesIdentityBasedAuthentication>,
    #[doc = "Allows https traffic only to storage service if sets to true."]
    #[serde(rename = "supportsHttpsTrafficOnly", default, skip_serializing_if = "Option::is_none")]
    pub supports_https_traffic_only: Option<bool>,
    #[doc = "Network rule set"]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[doc = "Account HierarchicalNamespace enabled if sets to true."]
    #[serde(rename = "isHnsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_hns_enabled: Option<bool>,
    #[doc = "Statistics related to replication for storage account's Blob, Table, Queue and File services. It is only available when geo-redundant replication is enabled for the storage account."]
    #[serde(rename = "geoReplicationStats", default, skip_serializing_if = "Option::is_none")]
    pub geo_replication_stats: Option<GeoReplicationStats>,
    #[doc = "If the failover is in progress, the value will be true, otherwise, it will be null."]
    #[serde(rename = "failoverInProgress", default, skip_serializing_if = "Option::is_none")]
    pub failover_in_progress: Option<bool>,
    #[doc = "Allow large file shares if sets to Enabled. It cannot be disabled once it is enabled."]
    #[serde(rename = "largeFileSharesState", default, skip_serializing_if = "Option::is_none")]
    pub large_file_shares_state: Option<storage_account_properties::LargeFileSharesState>,
    #[doc = "List of private endpoint connection associated with the specified storage account"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Routing preference defines the type of network, either microsoft or internet routing to be used to deliver the user data, the default option is microsoft routing"]
    #[serde(rename = "routingPreference", default, skip_serializing_if = "Option::is_none")]
    pub routing_preference: Option<RoutingPreference>,
    #[doc = "Blob restore status."]
    #[serde(rename = "blobRestoreStatus", default, skip_serializing_if = "Option::is_none")]
    pub blob_restore_status: Option<BlobRestoreStatus>,
    #[doc = "Allow or disallow public access to all blobs or containers in the storage account. The default interpretation is true for this property."]
    #[serde(rename = "allowBlobPublicAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_blob_public_access: Option<bool>,
    #[doc = "Set the minimum TLS version to be permitted on requests to storage. The default interpretation is TLS 1.0 for this property."]
    #[serde(rename = "minimumTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<storage_account_properties::MinimumTlsVersion>,
    #[doc = "Indicates whether the storage account permits requests to be authorized with the account access key via Shared Key. If false, then all requests, including shared access signatures, must be authorized with Azure Active Directory (Azure AD). The default value is null, which is equivalent to true."]
    #[serde(rename = "allowSharedKeyAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_shared_key_access: Option<bool>,
}
impl StorageAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_properties {
    use super::*;
    #[doc = "Gets the status of the storage account at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
        Succeeded,
    }
    #[doc = "Gets the status indicating whether the primary location of the storage account is available or unavailable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfPrimary {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    #[doc = "Gets the status indicating whether the secondary location of the storage account is available or unavailable. Only available if the SKU name is Standard_GRS or Standard_RAGRS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfSecondary {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
    #[doc = "Allow large file shares if sets to Enabled. It cannot be disabled once it is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LargeFileSharesState")]
    pub enum LargeFileSharesState {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LargeFileSharesState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LargeFileSharesState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LargeFileSharesState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("LargeFileSharesState", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("LargeFileSharesState", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Set the minimum TLS version to be permitted on requests to storage. The default interpretation is TLS 1.0 for this property."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MinimumTlsVersion")]
    pub enum MinimumTlsVersion {
        #[serde(rename = "TLS1_0")]
        Tls10,
        #[serde(rename = "TLS1_1")]
        Tls11,
        #[serde(rename = "TLS1_2")]
        Tls12,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MinimumTlsVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MinimumTlsVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MinimumTlsVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tls10 => serializer.serialize_unit_variant("MinimumTlsVersion", 0u32, "TLS1_0"),
                Self::Tls11 => serializer.serialize_unit_variant("MinimumTlsVersion", 1u32, "TLS1_1"),
                Self::Tls12 => serializer.serialize_unit_variant("MinimumTlsVersion", 2u32, "TLS1_2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters used to create the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountPropertiesCreateParameters {
    #[doc = "The custom domain assigned to this storage account. This can be set via Update."]
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[doc = "The encryption settings on the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Network rule set"]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[serde(rename = "accessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties_create_parameters::AccessTier>,
    #[doc = "Settings for Azure Files identity based authentication."]
    #[serde(rename = "azureFilesIdentityBasedAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub azure_files_identity_based_authentication: Option<AzureFilesIdentityBasedAuthentication>,
    #[doc = "Allows https traffic only to storage service if sets to true. The default value is true since API version 2019-04-01."]
    #[serde(rename = "supportsHttpsTrafficOnly", default, skip_serializing_if = "Option::is_none")]
    pub supports_https_traffic_only: Option<bool>,
    #[doc = "Account HierarchicalNamespace enabled if sets to true."]
    #[serde(rename = "isHnsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_hns_enabled: Option<bool>,
    #[doc = "Allow large file shares if sets to Enabled. It cannot be disabled once it is enabled."]
    #[serde(rename = "largeFileSharesState", default, skip_serializing_if = "Option::is_none")]
    pub large_file_shares_state: Option<storage_account_properties_create_parameters::LargeFileSharesState>,
    #[doc = "Routing preference defines the type of network, either microsoft or internet routing to be used to deliver the user data, the default option is microsoft routing"]
    #[serde(rename = "routingPreference", default, skip_serializing_if = "Option::is_none")]
    pub routing_preference: Option<RoutingPreference>,
    #[doc = "Allow or disallow public access to all blobs or containers in the storage account. The default interpretation is true for this property."]
    #[serde(rename = "allowBlobPublicAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_blob_public_access: Option<bool>,
    #[doc = "Set the minimum TLS version to be permitted on requests to storage. The default interpretation is TLS 1.0 for this property."]
    #[serde(rename = "minimumTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<storage_account_properties_create_parameters::MinimumTlsVersion>,
    #[doc = "Indicates whether the storage account permits requests to be authorized with the account access key via Shared Key. If false, then all requests, including shared access signatures, must be authorized with Azure Active Directory (Azure AD). The default value is null, which is equivalent to true."]
    #[serde(rename = "allowSharedKeyAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_shared_key_access: Option<bool>,
}
impl StorageAccountPropertiesCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_properties_create_parameters {
    use super::*;
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
    #[doc = "Allow large file shares if sets to Enabled. It cannot be disabled once it is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LargeFileSharesState")]
    pub enum LargeFileSharesState {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LargeFileSharesState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LargeFileSharesState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LargeFileSharesState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("LargeFileSharesState", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("LargeFileSharesState", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Set the minimum TLS version to be permitted on requests to storage. The default interpretation is TLS 1.0 for this property."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MinimumTlsVersion")]
    pub enum MinimumTlsVersion {
        #[serde(rename = "TLS1_0")]
        Tls10,
        #[serde(rename = "TLS1_1")]
        Tls11,
        #[serde(rename = "TLS1_2")]
        Tls12,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MinimumTlsVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MinimumTlsVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MinimumTlsVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tls10 => serializer.serialize_unit_variant("MinimumTlsVersion", 0u32, "TLS1_0"),
                Self::Tls11 => serializer.serialize_unit_variant("MinimumTlsVersion", 1u32, "TLS1_1"),
                Self::Tls12 => serializer.serialize_unit_variant("MinimumTlsVersion", 2u32, "TLS1_2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters used when updating a storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountPropertiesUpdateParameters {
    #[doc = "The custom domain assigned to this storage account. This can be set via Update."]
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[doc = "The encryption settings on the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[serde(rename = "accessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties_update_parameters::AccessTier>,
    #[doc = "Settings for Azure Files identity based authentication."]
    #[serde(rename = "azureFilesIdentityBasedAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub azure_files_identity_based_authentication: Option<AzureFilesIdentityBasedAuthentication>,
    #[doc = "Allows https traffic only to storage service if sets to true."]
    #[serde(rename = "supportsHttpsTrafficOnly", default, skip_serializing_if = "Option::is_none")]
    pub supports_https_traffic_only: Option<bool>,
    #[doc = "Network rule set"]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[doc = "Allow large file shares if sets to Enabled. It cannot be disabled once it is enabled."]
    #[serde(rename = "largeFileSharesState", default, skip_serializing_if = "Option::is_none")]
    pub large_file_shares_state: Option<storage_account_properties_update_parameters::LargeFileSharesState>,
    #[doc = "Routing preference defines the type of network, either microsoft or internet routing to be used to deliver the user data, the default option is microsoft routing"]
    #[serde(rename = "routingPreference", default, skip_serializing_if = "Option::is_none")]
    pub routing_preference: Option<RoutingPreference>,
    #[doc = "Allow or disallow public access to all blobs or containers in the storage account. The default interpretation is true for this property."]
    #[serde(rename = "allowBlobPublicAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_blob_public_access: Option<bool>,
    #[doc = "Set the minimum TLS version to be permitted on requests to storage. The default interpretation is TLS 1.0 for this property."]
    #[serde(rename = "minimumTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<storage_account_properties_update_parameters::MinimumTlsVersion>,
    #[doc = "Indicates whether the storage account permits requests to be authorized with the account access key via Shared Key. If false, then all requests, including shared access signatures, must be authorized with Azure Active Directory (Azure AD). The default value is null, which is equivalent to true."]
    #[serde(rename = "allowSharedKeyAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_shared_key_access: Option<bool>,
}
impl StorageAccountPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_properties_update_parameters {
    use super::*;
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
    #[doc = "Allow large file shares if sets to Enabled. It cannot be disabled once it is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LargeFileSharesState")]
    pub enum LargeFileSharesState {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LargeFileSharesState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LargeFileSharesState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LargeFileSharesState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("LargeFileSharesState", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("LargeFileSharesState", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Set the minimum TLS version to be permitted on requests to storage. The default interpretation is TLS 1.0 for this property."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MinimumTlsVersion")]
    pub enum MinimumTlsVersion {
        #[serde(rename = "TLS1_0")]
        Tls10,
        #[serde(rename = "TLS1_1")]
        Tls11,
        #[serde(rename = "TLS1_2")]
        Tls12,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MinimumTlsVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MinimumTlsVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MinimumTlsVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tls10 => serializer.serialize_unit_variant("MinimumTlsVersion", 0u32, "TLS1_0"),
                Self::Tls11 => serializer.serialize_unit_variant("MinimumTlsVersion", 1u32, "TLS1_1"),
                Self::Tls12 => serializer.serialize_unit_variant("MinimumTlsVersion", 2u32, "TLS1_2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters used to regenerate the storage account key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountRegenerateKeyParameters {
    #[doc = "The name of storage keys that want to be regenerated, possible values are key1, key2, kerb1, kerb2."]
    #[serde(rename = "keyName")]
    pub key_name: String,
}
impl StorageAccountRegenerateKeyParameters {
    pub fn new(key_name: String) -> Self {
        Self { key_name }
    }
}
#[doc = "The parameters that can be provided when updating the storage account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountUpdateParameters {
    #[doc = "The SKU of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater in length than 128 characters and a value no greater in length than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "The parameters used when updating a storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesUpdateParameters>,
    #[doc = "Optional. Indicates the type of storage account. Currently only StorageV2 value supported by server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<storage_account_update_parameters::Kind>,
}
impl StorageAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_update_parameters {
    use super::*;
    #[doc = "Optional. Indicates the type of storage account. Currently only StorageV2 value supported by server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Storage,
        StorageV2,
        BlobStorage,
        FileStorage,
        BlockBlobStorage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Storage => serializer.serialize_unit_variant("Kind", 0u32, "Storage"),
                Self::StorageV2 => serializer.serialize_unit_variant("Kind", 1u32, "StorageV2"),
                Self::BlobStorage => serializer.serialize_unit_variant("Kind", 2u32, "BlobStorage"),
                Self::FileStorage => serializer.serialize_unit_variant("Kind", 3u32, "FileStorage"),
                Self::BlockBlobStorage => serializer.serialize_unit_variant("Kind", 4u32, "BlockBlobStorage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response from the List Storage SKUs operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSkuListResult {
    #[doc = "Get the list result of storage SKUs and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuInformation>,
}
impl azure_core::Continuable for StorageSkuListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl StorageSkuListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Blob index tag based filtering for blob objects"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagFilter {
    #[doc = "This is the filter tag name, it can have 1 - 128 characters"]
    pub name: String,
    #[doc = "This is the comparison operator which is used for object comparison and filtering. Only == (equality operator) is currently supported"]
    pub op: String,
    #[doc = "This is the filter tag value field used for tag based filtering, it can have 0 - 256 characters"]
    pub value: String,
}
impl TagFilter {
    pub fn new(name: String, op: String, value: String) -> Self {
        Self { name, op, value }
    }
}
#[doc = "The SKU tier. This is based on the SKU name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Tier {
    Standard,
    Premium,
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
#[doc = "Describes Storage Resource Usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "Gets the unit of measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
    #[doc = "Gets the current count of the allocated resources in the subscription."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[doc = "Gets the maximum count of the resources that can be allocated in the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The usage names that can be used; currently limited to StorageAccount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[doc = "Gets the unit of measurement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountsPerSecond,
        BytesPerSecond,
    }
}
#[doc = "The response from the List Usages operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[doc = "Gets or sets the list of Storage Resource Usages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl azure_core::Continuable for UsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl UsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The usage names that can be used; currently limited to StorageAccount."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "Gets a string describing the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Gets a localized string describing the resource name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[doc = "Resource ID of a subnet, for example: /subscriptions/{subscriptionId}/resourceGroups/{groupName}/providers/Microsoft.Network/virtualNetworks/{vnetName}/subnets/{subnetName}."]
    pub id: String,
    #[doc = "The action of virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<virtual_network_rule::Action>,
    #[doc = "Gets the state of virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<virtual_network_rule::State>,
}
impl VirtualNetworkRule {
    pub fn new(id: String) -> Self {
        Self {
            id,
            action: None,
            state: None,
        }
    }
}
pub mod virtual_network_rule {
    use super::*;
    #[doc = "The action of virtual network rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
    }
    impl Default for Action {
        fn default() -> Self {
            Self::Allow
        }
    }
    #[doc = "Gets the state of virtual network rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "deprovisioning")]
        Deprovisioning,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "networkSourceDeleted")]
        NetworkSourceDeleted,
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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
