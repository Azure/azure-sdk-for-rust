#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicy {
    #[doc = "Start time of the access policy"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Expiry time of the access policy"]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "List of abbreviated permissions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}
impl AccessPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This defines account-level immutability policy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountImmutabilityPolicyProperties {
    #[doc = "The immutability period for the blobs in the container since the policy creation, in days."]
    #[serde(rename = "immutabilityPeriodSinceCreationInDays", default, skip_serializing_if = "Option::is_none")]
    pub immutability_period_since_creation_in_days: Option<i32>,
    #[doc = "The ImmutabilityPolicy state defines the mode of the policy. Disabled state disables the policy, Unlocked state allows increase and decrease of immutability retention time and also allows toggling allowProtectedAppendWrites property, Locked state only allows the increase of the immutability retention time. A policy can only be created in a Disabled or Unlocked state and can be toggled between the two states. Only a policy in an Unlocked state can transition to a Locked state which cannot be reverted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<account_immutability_policy_properties::State>,
    #[doc = "This property can only be changed for disabled and unlocked time-based retention policies. When enabled, new blocks can be written to an append blob while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted."]
    #[serde(rename = "allowProtectedAppendWrites", default, skip_serializing_if = "Option::is_none")]
    pub allow_protected_append_writes: Option<bool>,
}
impl AccountImmutabilityPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_immutability_policy_properties {
    use super::*;
    #[doc = "The ImmutabilityPolicy state defines the mode of the policy. Disabled state disables the policy, Unlocked state allows increase and decrease of immutability retention time and also allows toggling allowProtectedAppendWrites property, Locked state only allows the increase of the immutability retention time. A policy can only be created in a Disabled or Unlocked state and can be toggled between the two states. Only a policy in an Unlocked state can transition to a Locked state which cannot be reverted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Unlocked,
        Locked,
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
                Self::Unlocked => serializer.serialize_unit_variant("State", 0u32, "Unlocked"),
                Self::Locked => serializer.serialize_unit_variant("State", 1u32, "Locked"),
                Self::Disabled => serializer.serialize_unit_variant("State", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
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
#[doc = "The resource model definition for an Azure Resource Manager resource with an etag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureEntityResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource Etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl AzureEntityResource {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Default share permission for users using Kerberos authentication if RBAC role is not assigned."]
    #[serde(rename = "defaultSharePermission", default, skip_serializing_if = "Option::is_none")]
    pub default_share_permission: Option<azure_files_identity_based_authentication::DefaultSharePermission>,
}
impl AzureFilesIdentityBasedAuthentication {
    pub fn new(directory_service_options: azure_files_identity_based_authentication::DirectoryServiceOptions) -> Self {
        Self {
            directory_service_options,
            active_directory_properties: None,
            default_share_permission: None,
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
    #[doc = "Default share permission for users using Kerberos authentication if RBAC role is not assigned."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DefaultSharePermission")]
    pub enum DefaultSharePermission {
        None,
        StorageFileDataSmbShareReader,
        StorageFileDataSmbShareContributor,
        StorageFileDataSmbShareElevatedContributor,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DefaultSharePermission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DefaultSharePermission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DefaultSharePermission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("DefaultSharePermission", 0u32, "None"),
                Self::StorageFileDataSmbShareReader => {
                    serializer.serialize_unit_variant("DefaultSharePermission", 1u32, "StorageFileDataSmbShareReader")
                }
                Self::StorageFileDataSmbShareContributor => {
                    serializer.serialize_unit_variant("DefaultSharePermission", 2u32, "StorageFileDataSmbShareContributor")
                }
                Self::StorageFileDataSmbShareElevatedContributor => {
                    serializer.serialize_unit_variant("DefaultSharePermission", 3u32, "StorageFileDataSmbShareElevatedContributor")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the blob container, including Id, resource name, resource type, Etag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobContainer {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "The properties of a container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContainerProperties>,
}
impl BlobContainer {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "An object that defines the blob inventory rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobInventoryPolicyDefinition {
    #[doc = "An object that defines the blob inventory rule filter conditions. For 'Blob' definition.objectType all filter properties are applicable, 'blobTypes' is required and others are optional. For 'Container' definition.objectType only prefixMatch is applicable and is optional."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<BlobInventoryPolicyFilter>,
    #[doc = "This is a required field, it specifies the format for the inventory files."]
    pub format: blob_inventory_policy_definition::Format,
    #[doc = "This is a required field. This field is used to schedule an inventory formation."]
    pub schedule: blob_inventory_policy_definition::Schedule,
    #[doc = "This is a required field. This field specifies the scope of the inventory created either at the blob or container level."]
    #[serde(rename = "objectType")]
    pub object_type: blob_inventory_policy_definition::ObjectType,
    #[doc = "This is a required field. This field specifies the fields and properties of the object to be included in the inventory. The Schema field value 'Name' is always required. The valid values for this field for the 'Blob' definition.objectType include 'Name, Creation-Time, Last-Modified, Content-Length, Content-MD5, BlobType, AccessTier, AccessTierChangeTime, AccessTierInferred, Tags, Expiry-Time, hdi_isfolder, Owner, Group, Permissions, Acl, Snapshot, VersionId, IsCurrentVersion, Metadata, LastAccessTime'. The valid values for 'Container' definition.objectType include 'Name, Last-Modified, Metadata, LeaseStatus, LeaseState, LeaseDuration, PublicAccess, HasImmutabilityPolicy, HasLegalHold'. Schema field values 'Expiry-Time, hdi_isfolder, Owner, Group, Permissions, Acl' are valid only for Hns enabled accounts.'Tags' field is only valid for non Hns accounts"]
    #[serde(rename = "schemaFields")]
    pub schema_fields: Vec<String>,
}
impl BlobInventoryPolicyDefinition {
    pub fn new(
        format: blob_inventory_policy_definition::Format,
        schedule: blob_inventory_policy_definition::Schedule,
        object_type: blob_inventory_policy_definition::ObjectType,
        schema_fields: Vec<String>,
    ) -> Self {
        Self {
            filters: None,
            format,
            schedule,
            object_type,
            schema_fields,
        }
    }
}
pub mod blob_inventory_policy_definition {
    use super::*;
    #[doc = "This is a required field, it specifies the format for the inventory files."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Format")]
    pub enum Format {
        Csv,
        Parquet,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Format {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Format {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Format {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Csv => serializer.serialize_unit_variant("Format", 0u32, "Csv"),
                Self::Parquet => serializer.serialize_unit_variant("Format", 1u32, "Parquet"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "This is a required field. This field is used to schedule an inventory formation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Schedule")]
    pub enum Schedule {
        Daily,
        Weekly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Schedule {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Schedule {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Schedule {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Daily => serializer.serialize_unit_variant("Schedule", 0u32, "Daily"),
                Self::Weekly => serializer.serialize_unit_variant("Schedule", 1u32, "Weekly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "This is a required field. This field specifies the scope of the inventory created either at the blob or container level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ObjectType")]
    pub enum ObjectType {
        Blob,
        Container,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ObjectType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ObjectType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ObjectType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Blob => serializer.serialize_unit_variant("ObjectType", 0u32, "Blob"),
                Self::Container => serializer.serialize_unit_variant("ObjectType", 1u32, "Container"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An object that defines the blob inventory rule filter conditions. For 'Blob' definition.objectType all filter properties are applicable, 'blobTypes' is required and others are optional. For 'Container' definition.objectType only prefixMatch is applicable and is optional."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobInventoryPolicyFilter {
    #[doc = "An array of strings for blob prefixes to be matched."]
    #[serde(rename = "prefixMatch", default, skip_serializing_if = "Vec::is_empty")]
    pub prefix_match: Vec<String>,
    #[doc = "An array of predefined enum values. Valid values include blockBlob, appendBlob, pageBlob. Hns accounts does not support pageBlobs. This field is required when definition.objectType property is set to 'Blob'."]
    #[serde(rename = "blobTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub blob_types: Vec<String>,
    #[doc = "Includes blob versions in blob inventory when value is set to true. The definition.schemaFields values 'VersionId and IsCurrentVersion' are required if this property is set to true, else they must be excluded."]
    #[serde(rename = "includeBlobVersions", default, skip_serializing_if = "Option::is_none")]
    pub include_blob_versions: Option<bool>,
    #[doc = "Includes blob snapshots in blob inventory when value is set to true. The definition.schemaFields value 'Snapshot' is required if this property is set to true, else it must be excluded."]
    #[serde(rename = "includeSnapshots", default, skip_serializing_if = "Option::is_none")]
    pub include_snapshots: Option<bool>,
}
impl BlobInventoryPolicyFilter {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Container name where blob inventory files are stored. Must be pre-created."]
    pub destination: String,
    #[doc = "An object that defines the blob inventory rule."]
    pub definition: BlobInventoryPolicyDefinition,
}
impl BlobInventoryPolicyRule {
    pub fn new(enabled: bool, name: String, destination: String, definition: BlobInventoryPolicyDefinition) -> Self {
        Self {
            enabled,
            name,
            destination,
            definition,
        }
    }
}
#[doc = "The storage account blob inventory policy rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobInventoryPolicySchema {
    #[doc = "Policy is enabled if set to true."]
    pub enabled: bool,
    #[doc = "The valid value is Inventory"]
    #[serde(rename = "type")]
    pub type_: blob_inventory_policy_schema::Type,
    #[doc = "The storage account blob inventory policy rules. The rule is applied when it is enabled."]
    pub rules: Vec<BlobInventoryPolicyRule>,
}
impl BlobInventoryPolicySchema {
    pub fn new(enabled: bool, type_: blob_inventory_policy_schema::Type, rules: Vec<BlobInventoryPolicyRule>) -> Self {
        Self { enabled, type_, rules }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobServiceItems {
    #[doc = "List of blob services returned."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BlobServiceProperties>,
}
impl azure_core::Continuable for BlobServiceItems {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BlobServiceItems {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a storage account’s Blob service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobServiceProperties {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a storage account’s Blob service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<blob_service_properties::Properties>,
    #[doc = "The SKU of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl BlobServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod blob_service_properties {
    use super::*;
    #[doc = "The properties of a storage account’s Blob service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Sets the CORS rules. You can include up to five CorsRule elements in the request. "]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cors: Option<CorsRules>,
        #[doc = "DefaultServiceVersion indicates the default version to use for requests to the Blob service if an incoming request’s version is not specified. Possible values include version 2008-10-27 and all more recent versions."]
        #[serde(rename = "defaultServiceVersion", default, skip_serializing_if = "Option::is_none")]
        pub default_service_version: Option<String>,
        #[doc = "The service properties for soft delete."]
        #[serde(rename = "deleteRetentionPolicy", default, skip_serializing_if = "Option::is_none")]
        pub delete_retention_policy: Option<DeleteRetentionPolicy>,
        #[doc = "Versioning is enabled if set to true."]
        #[serde(rename = "isVersioningEnabled", default, skip_serializing_if = "Option::is_none")]
        pub is_versioning_enabled: Option<bool>,
        #[doc = "Deprecated in favor of isVersioningEnabled property."]
        #[serde(rename = "automaticSnapshotPolicyEnabled", default, skip_serializing_if = "Option::is_none")]
        pub automatic_snapshot_policy_enabled: Option<bool>,
        #[doc = "The blob service properties for change feed events."]
        #[serde(rename = "changeFeed", default, skip_serializing_if = "Option::is_none")]
        pub change_feed: Option<ChangeFeed>,
        #[doc = "The blob service properties for blob restore policy"]
        #[serde(rename = "restorePolicy", default, skip_serializing_if = "Option::is_none")]
        pub restore_policy: Option<RestorePolicyProperties>,
        #[doc = "The service properties for soft delete."]
        #[serde(rename = "containerDeleteRetentionPolicy", default, skip_serializing_if = "Option::is_none")]
        pub container_delete_retention_policy: Option<DeleteRetentionPolicy>,
        #[doc = "The blob service properties for Last access time based tracking policy."]
        #[serde(rename = "lastAccessTimeTrackingPolicy", default, skip_serializing_if = "Option::is_none")]
        pub last_access_time_tracking_policy: Option<LastAccessTimeTrackingPolicy>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The blob service properties for change feed events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeFeed {
    #[doc = "Indicates whether change feed event logging is enabled for the Blob service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Indicates the duration of changeFeed retention in days. Minimum value is 1 day and maximum value is 146000 days (400 years). A null value indicates an infinite retention of the change feed."]
    #[serde(rename = "retentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
}
impl ChangeFeed {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "An error response from the Storage service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Storage service."]
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
#[doc = "An error response from the Storage service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerProperties {
    #[doc = "The version of the deleted blob container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Indicates whether the blob container was deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[doc = "Blob container deletion time."]
    #[serde(rename = "deletedTime", with = "azure_core::date::rfc3339::option")]
    pub deleted_time: Option<time::OffsetDateTime>,
    #[doc = "Remaining retention days for soft deleted blob container."]
    #[serde(rename = "remainingRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub remaining_retention_days: Option<i64>,
    #[doc = "Default the container to use specified encryption scope for all writes."]
    #[serde(rename = "defaultEncryptionScope", default, skip_serializing_if = "Option::is_none")]
    pub default_encryption_scope: Option<String>,
    #[doc = "Block override of encryption scope from the container default."]
    #[serde(rename = "denyEncryptionScopeOverride", default, skip_serializing_if = "Option::is_none")]
    pub deny_encryption_scope_override: Option<bool>,
    #[doc = "Specifies whether data in the container may be accessed publicly and the level of access."]
    #[serde(rename = "publicAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_access: Option<container_properties::PublicAccess>,
    #[doc = "Returns the date and time the container was last modified."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The lease status of the container."]
    #[serde(rename = "leaseStatus", default, skip_serializing_if = "Option::is_none")]
    pub lease_status: Option<container_properties::LeaseStatus>,
    #[doc = "Lease state of the container."]
    #[serde(rename = "leaseState", default, skip_serializing_if = "Option::is_none")]
    pub lease_state: Option<container_properties::LeaseState>,
    #[doc = "Specifies whether the lease on a container is of infinite or fixed duration, only when the container is leased."]
    #[serde(rename = "leaseDuration", default, skip_serializing_if = "Option::is_none")]
    pub lease_duration: Option<container_properties::LeaseDuration>,
    #[doc = "A name-value pair to associate with the container as metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The properties of an ImmutabilityPolicy of a blob container."]
    #[serde(rename = "immutabilityPolicy", default, skip_serializing_if = "Option::is_none")]
    pub immutability_policy: Option<ImmutabilityPolicyProperties>,
    #[doc = "The LegalHold property of a blob container."]
    #[serde(rename = "legalHold", default, skip_serializing_if = "Option::is_none")]
    pub legal_hold: Option<LegalHoldProperties>,
    #[doc = "The hasLegalHold public property is set to true by SRP if there are at least one existing tag. The hasLegalHold public property is set to false by SRP if all existing legal hold tags are cleared out. There can be a maximum of 1000 blob containers with hasLegalHold=true for a given account."]
    #[serde(rename = "hasLegalHold", default, skip_serializing_if = "Option::is_none")]
    pub has_legal_hold: Option<bool>,
    #[doc = "The hasImmutabilityPolicy public property is set to true by SRP if ImmutabilityPolicy has been created for this container. The hasImmutabilityPolicy public property is set to false by SRP if ImmutabilityPolicy has not been created for this container."]
    #[serde(rename = "hasImmutabilityPolicy", default, skip_serializing_if = "Option::is_none")]
    pub has_immutability_policy: Option<bool>,
    #[doc = "Object level immutability properties of the container."]
    #[serde(rename = "immutableStorageWithVersioning", default, skip_serializing_if = "Option::is_none")]
    pub immutable_storage_with_versioning: Option<ImmutableStorageWithVersioning>,
    #[doc = "Enable NFSv3 root squash on blob container."]
    #[serde(rename = "enableNfsV3RootSquash", default, skip_serializing_if = "Option::is_none")]
    pub enable_nfs_v3_root_squash: Option<bool>,
    #[doc = "Enable NFSv3 all squash on blob container."]
    #[serde(rename = "enableNfsV3AllSquash", default, skip_serializing_if = "Option::is_none")]
    pub enable_nfs_v3_all_squash: Option<bool>,
}
impl ContainerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_properties {
    use super::*;
    #[doc = "Specifies whether data in the container may be accessed publicly and the level of access."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicAccess {
        Container,
        Blob,
        None,
    }
    #[doc = "The lease status of the container."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LeaseStatus")]
    pub enum LeaseStatus {
        Locked,
        Unlocked,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LeaseStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LeaseStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LeaseStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Locked => serializer.serialize_unit_variant("LeaseStatus", 0u32, "Locked"),
                Self::Unlocked => serializer.serialize_unit_variant("LeaseStatus", 1u32, "Unlocked"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Lease state of the container."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LeaseState")]
    pub enum LeaseState {
        Available,
        Leased,
        Expired,
        Breaking,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LeaseState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LeaseState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LeaseState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Available => serializer.serialize_unit_variant("LeaseState", 0u32, "Available"),
                Self::Leased => serializer.serialize_unit_variant("LeaseState", 1u32, "Leased"),
                Self::Expired => serializer.serialize_unit_variant("LeaseState", 2u32, "Expired"),
                Self::Breaking => serializer.serialize_unit_variant("LeaseState", 3u32, "Breaking"),
                Self::Broken => serializer.serialize_unit_variant("LeaseState", 4u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies whether the lease on a container is of infinite or fixed duration, only when the container is leased."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LeaseDuration")]
    pub enum LeaseDuration {
        Infinite,
        Fixed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LeaseDuration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LeaseDuration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LeaseDuration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Infinite => serializer.serialize_unit_variant("LeaseDuration", 0u32, "Infinite"),
                Self::Fixed => serializer.serialize_unit_variant("LeaseDuration", 1u32, "Fixed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies a CORS rule for the Blob service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsRule {
    #[doc = "Required if CorsRule element is present. A list of origin domains that will be allowed via CORS, or \"*\" to allow all domains"]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Vec<String>,
    #[doc = "Required if CorsRule element is present. A list of HTTP methods that are allowed to be executed by the origin."]
    #[serde(rename = "allowedMethods")]
    pub allowed_methods: Vec<String>,
    #[doc = "Required if CorsRule element is present. The number of seconds that the client/browser should cache a preflight response."]
    #[serde(rename = "maxAgeInSeconds")]
    pub max_age_in_seconds: i64,
    #[doc = "Required if CorsRule element is present. A list of response headers to expose to CORS clients."]
    #[serde(rename = "exposedHeaders")]
    pub exposed_headers: Vec<String>,
    #[doc = "Required if CorsRule element is present. A list of headers allowed to be part of the cross-origin request."]
    #[serde(rename = "allowedHeaders")]
    pub allowed_headers: Vec<String>,
}
impl CorsRule {
    pub fn new(
        allowed_origins: Vec<String>,
        allowed_methods: Vec<String>,
        max_age_in_seconds: i64,
        exposed_headers: Vec<String>,
        allowed_headers: Vec<String>,
    ) -> Self {
        Self {
            allowed_origins,
            allowed_methods,
            max_age_in_seconds,
            exposed_headers,
            allowed_headers,
        }
    }
}
#[doc = "Sets the CORS rules. You can include up to five CorsRule elements in the request. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CorsRules {
    #[doc = "The List of CORS rules. You can include up to five CorsRule elements in the request. "]
    #[serde(rename = "corsRules", default, skip_serializing_if = "Vec::is_empty")]
    pub cors_rules: Vec<CorsRule>,
}
impl CorsRules {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The service properties for soft delete."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteRetentionPolicy {
    #[doc = "Indicates whether DeleteRetentionPolicy is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Indicates the number of days that the deleted item should be retained. The minimum specified value can be 1 and the maximum value can be 365."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}
impl DeleteRetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deleted storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedAccount {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Attributes of a deleted storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeletedAccountProperties>,
}
impl DeletedAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List Deleted Accounts operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedAccountListResult {
    #[doc = "Gets the list of deleted accounts and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeletedAccount>,
    #[doc = "Request URL that can be used to query next page of deleted accounts. Returned when total number of requested deleted accounts exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeletedAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Attributes of a deleted storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedAccountProperties {
    #[doc = "Full resource id of the original storage account."]
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[doc = "Location of the deleted account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Can be used to attempt recovering this deleted account via PutStorageAccount API."]
    #[serde(rename = "restoreReference", default, skip_serializing_if = "Option::is_none")]
    pub restore_reference: Option<String>,
    #[doc = "Creation time of the deleted account."]
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[doc = "Deletion time of the deleted account."]
    #[serde(rename = "deletionTime", default, skip_serializing_if = "Option::is_none")]
    pub deletion_time: Option<String>,
}
impl DeletedAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deleted share to be restored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedShare {
    #[doc = "Required. Identify the name of the deleted share that will be restored."]
    #[serde(rename = "deletedShareName")]
    pub deleted_share_name: String,
    #[doc = "Required. Identify the version of the deleted share that will be restored."]
    #[serde(rename = "deletedShareVersion")]
    pub deleted_share_version: String,
}
impl DeletedShare {
    pub fn new(deleted_share_name: String, deleted_share_version: String) -> Self {
        Self {
            deleted_share_name,
            deleted_share_version,
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
    #[doc = "Encryption identity for the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionIdentity>,
}
impl Encryption {
    pub fn new(key_source: encryption::KeySource) -> Self {
        Self {
            services: None,
            key_source,
            require_infrastructure_encryption: None,
            keyvaultproperties: None,
            identity: None,
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
#[doc = "Encryption identity for the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionIdentity {
    #[doc = "Resource identifier of the UserAssigned identity to be associated with server-side encryption on the storage account."]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
impl EncryptionIdentity {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "The object identifier of the current versioned Key Vault Key in use."]
    #[serde(rename = "currentVersionedKeyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub current_versioned_key_identifier: Option<String>,
    #[doc = "Timestamp of last rotation of the Key Vault Key."]
    #[serde(rename = "lastKeyRotationTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_key_rotation_timestamp: Option<time::OffsetDateTime>,
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
    #[doc = "A boolean indicating whether or not the service applies a secondary layer of encryption with platform managed keys for data at rest."]
    #[serde(rename = "requireInfrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
    pub require_infrastructure_encryption: Option<bool>,
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
#[doc = "The complex type of the extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "The name of the extended location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of extendedLocation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ExtendedLocationType>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of extendedLocation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExtendedLocationType")]
pub enum ExtendedLocationType {
    EdgeZone,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExtendedLocationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExtendedLocationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExtendedLocationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EdgeZone => serializer.serialize_unit_variant("ExtendedLocationType", 0u32, "EdgeZone"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileServiceItems {
    #[doc = "List of file services returned."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FileServiceProperties>,
}
impl FileServiceItems {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of File services in storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileServiceProperties {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of File services in storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<file_service_properties::Properties>,
    #[doc = "The SKU of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl FileServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod file_service_properties {
    use super::*;
    #[doc = "The properties of File services in storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Sets the CORS rules. You can include up to five CorsRule elements in the request. "]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cors: Option<CorsRules>,
        #[doc = "The service properties for soft delete."]
        #[serde(rename = "shareDeleteRetentionPolicy", default, skip_serializing_if = "Option::is_none")]
        pub share_delete_retention_policy: Option<DeleteRetentionPolicy>,
        #[doc = "Protocol settings for file service"]
        #[serde(rename = "protocolSettings", default, skip_serializing_if = "Option::is_none")]
        pub protocol_settings: Option<ProtocolSettings>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Properties of the file share, including Id, resource name, resource type, Etag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileShare {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "The properties of the file share."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FileShareProperties>,
}
impl FileShare {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The file share properties be listed out."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileShareItem {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "The properties of the file share."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FileShareProperties>,
}
impl FileShareItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response schema. Contains list of shares returned, and if paging is requested or required, a URL to next page of shares."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileShareItems {
    #[doc = "List of file shares returned."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FileShareItem>,
    #[doc = "Request URL that can be used to query next page of shares. Returned when total number of requested shares exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FileShareItems {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FileShareItems {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the file share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileShareProperties {
    #[doc = "Returns the date and time the share was last modified."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "A name-value pair to associate with the share as metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The maximum size of the share, in gigabytes. Must be greater than 0, and less than or equal to 5TB (5120). For Large File Shares, the maximum size is 102400."]
    #[serde(rename = "shareQuota", default, skip_serializing_if = "Option::is_none")]
    pub share_quota: Option<i64>,
    #[doc = "The authentication protocol that is used for the file share. Can only be specified when creating a share."]
    #[serde(rename = "enabledProtocols", default, skip_serializing_if = "Option::is_none")]
    pub enabled_protocols: Option<file_share_properties::EnabledProtocols>,
    #[doc = "The property is for NFS share only. The default is NoRootSquash."]
    #[serde(rename = "rootSquash", default, skip_serializing_if = "Option::is_none")]
    pub root_squash: Option<file_share_properties::RootSquash>,
    #[doc = "The version of the share."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Indicates whether the share was deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[doc = "The deleted time if the share was deleted."]
    #[serde(rename = "deletedTime", with = "azure_core::date::rfc3339::option")]
    pub deleted_time: Option<time::OffsetDateTime>,
    #[doc = "Remaining retention days for share that was soft deleted."]
    #[serde(rename = "remainingRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub remaining_retention_days: Option<i64>,
    #[doc = "Access tier for specific share. GpV2 account can choose between TransactionOptimized (default), Hot, and Cool. FileStorage account can choose Premium."]
    #[serde(rename = "accessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<file_share_properties::AccessTier>,
    #[doc = "Indicates the last modification time for share access tier."]
    #[serde(rename = "accessTierChangeTime", with = "azure_core::date::rfc3339::option")]
    pub access_tier_change_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates if there is a pending transition for access tier."]
    #[serde(rename = "accessTierStatus", default, skip_serializing_if = "Option::is_none")]
    pub access_tier_status: Option<String>,
    #[doc = "The approximate size of the data stored on the share. Note that this value may not include all recently created or recently resized files."]
    #[serde(rename = "shareUsageBytes", default, skip_serializing_if = "Option::is_none")]
    pub share_usage_bytes: Option<i64>,
    #[doc = "The lease status of the share."]
    #[serde(rename = "leaseStatus", default, skip_serializing_if = "Option::is_none")]
    pub lease_status: Option<file_share_properties::LeaseStatus>,
    #[doc = "Lease state of the share."]
    #[serde(rename = "leaseState", default, skip_serializing_if = "Option::is_none")]
    pub lease_state: Option<file_share_properties::LeaseState>,
    #[doc = "Specifies whether the lease on a share is of infinite or fixed duration, only when the share is leased."]
    #[serde(rename = "leaseDuration", default, skip_serializing_if = "Option::is_none")]
    pub lease_duration: Option<file_share_properties::LeaseDuration>,
    #[doc = "List of stored access policies specified on the share."]
    #[serde(rename = "signedIdentifiers", default, skip_serializing_if = "Vec::is_empty")]
    pub signed_identifiers: Vec<SignedIdentifier>,
    #[doc = "Creation time of share snapshot returned in the response of list shares with expand param \"snapshots\"."]
    #[serde(rename = "snapshotTime", with = "azure_core::date::rfc3339::option")]
    pub snapshot_time: Option<time::OffsetDateTime>,
}
impl FileShareProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod file_share_properties {
    use super::*;
    #[doc = "The authentication protocol that is used for the file share. Can only be specified when creating a share."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnabledProtocols")]
    pub enum EnabledProtocols {
        #[serde(rename = "SMB")]
        Smb,
        #[serde(rename = "NFS")]
        Nfs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnabledProtocols {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnabledProtocols {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnabledProtocols {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Smb => serializer.serialize_unit_variant("EnabledProtocols", 0u32, "SMB"),
                Self::Nfs => serializer.serialize_unit_variant("EnabledProtocols", 1u32, "NFS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The property is for NFS share only. The default is NoRootSquash."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RootSquash")]
    pub enum RootSquash {
        NoRootSquash,
        RootSquash,
        AllSquash,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RootSquash {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RootSquash {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RootSquash {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NoRootSquash => serializer.serialize_unit_variant("RootSquash", 0u32, "NoRootSquash"),
                Self::RootSquash => serializer.serialize_unit_variant("RootSquash", 1u32, "RootSquash"),
                Self::AllSquash => serializer.serialize_unit_variant("RootSquash", 2u32, "AllSquash"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Access tier for specific share. GpV2 account can choose between TransactionOptimized (default), Hot, and Cool. FileStorage account can choose Premium."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessTier")]
    pub enum AccessTier {
        TransactionOptimized,
        Hot,
        Cool,
        Premium,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::TransactionOptimized => serializer.serialize_unit_variant("AccessTier", 0u32, "TransactionOptimized"),
                Self::Hot => serializer.serialize_unit_variant("AccessTier", 1u32, "Hot"),
                Self::Cool => serializer.serialize_unit_variant("AccessTier", 2u32, "Cool"),
                Self::Premium => serializer.serialize_unit_variant("AccessTier", 3u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The lease status of the share."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LeaseStatus")]
    pub enum LeaseStatus {
        Locked,
        Unlocked,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LeaseStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LeaseStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LeaseStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Locked => serializer.serialize_unit_variant("LeaseStatus", 0u32, "Locked"),
                Self::Unlocked => serializer.serialize_unit_variant("LeaseStatus", 1u32, "Unlocked"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Lease state of the share."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LeaseState")]
    pub enum LeaseState {
        Available,
        Leased,
        Expired,
        Breaking,
        Broken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LeaseState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LeaseState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LeaseState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Available => serializer.serialize_unit_variant("LeaseState", 0u32, "Available"),
                Self::Leased => serializer.serialize_unit_variant("LeaseState", 1u32, "Leased"),
                Self::Expired => serializer.serialize_unit_variant("LeaseState", 2u32, "Expired"),
                Self::Breaking => serializer.serialize_unit_variant("LeaseState", 3u32, "Breaking"),
                Self::Broken => serializer.serialize_unit_variant("LeaseState", 4u32, "Broken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies whether the lease on a share is of infinite or fixed duration, only when the share is leased."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LeaseDuration")]
    pub enum LeaseDuration {
        Infinite,
        Fixed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LeaseDuration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LeaseDuration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LeaseDuration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Infinite => serializer.serialize_unit_variant("LeaseDuration", 0u32, "Infinite"),
                Self::Fixed => serializer.serialize_unit_variant("LeaseDuration", 1u32, "Fixed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[doc = "Gets or sets a list of key value pairs that describe the set of User Assigned identities that will be used with this storage account. The key is the ARM resource identifier of the identity. Only 1 User Assigned identity is permitted here."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl Identity {
    pub fn new(type_: identity::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned,UserAssigned")]
        SystemAssignedUserAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 3u32, "SystemAssigned,UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The ImmutabilityPolicy property of a blob container, including Id, resource name, resource type, Etag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImmutabilityPolicy {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "The properties of an ImmutabilityPolicy of a blob container."]
    pub properties: ImmutabilityPolicyProperty,
}
impl ImmutabilityPolicy {
    pub fn new(properties: ImmutabilityPolicyProperty) -> Self {
        Self {
            azure_entity_resource: AzureEntityResource::default(),
            properties,
        }
    }
}
#[doc = "The properties of an ImmutabilityPolicy of a blob container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImmutabilityPolicyProperties {
    #[doc = "The properties of an ImmutabilityPolicy of a blob container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImmutabilityPolicyProperty>,
    #[doc = "ImmutabilityPolicy Etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The ImmutabilityPolicy update history of the blob container."]
    #[serde(rename = "updateHistory", default, skip_serializing_if = "Vec::is_empty")]
    pub update_history: Vec<UpdateHistoryProperty>,
}
impl ImmutabilityPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an ImmutabilityPolicy of a blob container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImmutabilityPolicyProperty {
    #[doc = "The immutability period for the blobs in the container since the policy creation, in days."]
    #[serde(rename = "immutabilityPeriodSinceCreationInDays", default, skip_serializing_if = "Option::is_none")]
    pub immutability_period_since_creation_in_days: Option<i64>,
    #[doc = "The ImmutabilityPolicy state of a blob container, possible values include: Locked and Unlocked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<immutability_policy_property::State>,
    #[doc = "This property can only be changed for unlocked time-based retention policies. When enabled, new blocks can be written to an append blob while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted. This property cannot be changed with ExtendImmutabilityPolicy API."]
    #[serde(rename = "allowProtectedAppendWrites", default, skip_serializing_if = "Option::is_none")]
    pub allow_protected_append_writes: Option<bool>,
    #[doc = "This property can only be changed for unlocked time-based retention policies. When enabled, new blocks can be written to both 'Append and Bock Blobs' while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted. This property cannot be changed with ExtendImmutabilityPolicy API. The 'allowProtectedAppendWrites' and 'allowProtectedAppendWritesAll' properties are mutually exclusive."]
    #[serde(rename = "allowProtectedAppendWritesAll", default, skip_serializing_if = "Option::is_none")]
    pub allow_protected_append_writes_all: Option<bool>,
}
impl ImmutabilityPolicyProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod immutability_policy_property {
    use super::*;
    #[doc = "The ImmutabilityPolicy state of a blob container, possible values include: Locked and Unlocked."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Locked,
        Unlocked,
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
                Self::Locked => serializer.serialize_unit_variant("State", 0u32, "Locked"),
                Self::Unlocked => serializer.serialize_unit_variant("State", 1u32, "Unlocked"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This property enables and defines account-level immutability. Enabling the feature auto-enables Blob Versioning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImmutableStorageAccount {
    #[doc = "A boolean flag which enables account-level immutability. All the containers under such an account have object-level immutability enabled by default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "This defines account-level immutability policy properties."]
    #[serde(rename = "immutabilityPolicy", default, skip_serializing_if = "Option::is_none")]
    pub immutability_policy: Option<AccountImmutabilityPolicyProperties>,
}
impl ImmutableStorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object level immutability properties of the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImmutableStorageWithVersioning {
    #[doc = "This is an immutable property, when set to true it enables object level immutability at the container level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Returns the date and time the object level immutability was enabled."]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339::option")]
    pub time_stamp: Option<time::OffsetDateTime>,
    #[doc = "This property denotes the container level immutability to object level immutability migration state."]
    #[serde(rename = "migrationState", default, skip_serializing_if = "Option::is_none")]
    pub migration_state: Option<immutable_storage_with_versioning::MigrationState>,
}
impl ImmutableStorageWithVersioning {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod immutable_storage_with_versioning {
    use super::*;
    #[doc = "This property denotes the container level immutability to object level immutability migration state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MigrationState")]
    pub enum MigrationState {
        InProgress,
        Completed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MigrationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MigrationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MigrationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::InProgress => serializer.serialize_unit_variant("MigrationState", 0u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("MigrationState", 1u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Storage account keys creation time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyCreationTime {
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub key1: Option<time::OffsetDateTime>,
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub key2: Option<time::OffsetDateTime>,
}
impl KeyCreationTime {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KeyPolicy assigned to the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyPolicy {
    #[doc = "The key expiration period in days."]
    #[serde(rename = "keyExpirationPeriodInDays")]
    pub key_expiration_period_in_days: i32,
}
impl KeyPolicy {
    pub fn new(key_expiration_period_in_days: i32) -> Self {
        Self {
            key_expiration_period_in_days,
        }
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
#[doc = "The blob service properties for Last access time based tracking policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LastAccessTimeTrackingPolicy {
    #[doc = "When set to true last access time based tracking is enabled."]
    pub enable: bool,
    #[doc = "Name of the policy. The valid value is AccessTimeTracking. This field is currently read only"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<last_access_time_tracking_policy::Name>,
    #[doc = "The field specifies blob object tracking granularity in days, typically how often the blob object should be tracked.This field is currently read only with value as 1"]
    #[serde(rename = "trackingGranularityInDays", default, skip_serializing_if = "Option::is_none")]
    pub tracking_granularity_in_days: Option<i32>,
    #[doc = "An array of predefined supported blob types. Only blockBlob is the supported value. This field is currently read only"]
    #[serde(rename = "blobType", default, skip_serializing_if = "Vec::is_empty")]
    pub blob_type: Vec<String>,
}
impl LastAccessTimeTrackingPolicy {
    pub fn new(enable: bool) -> Self {
        Self {
            enable,
            name: None,
            tracking_granularity_in_days: None,
            blob_type: Vec::new(),
        }
    }
}
pub mod last_access_time_tracking_policy {
    use super::*;
    #[doc = "Name of the policy. The valid value is AccessTimeTracking. This field is currently read only"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        AccessTimeTracking,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AccessTimeTracking => serializer.serialize_unit_variant("Name", 0u32, "AccessTimeTracking"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Lease Container request schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeaseContainerRequest {
    #[doc = "Specifies the lease action. Can be one of the available actions."]
    pub action: lease_container_request::Action,
    #[doc = "Identifies the lease. Can be specified in any valid GUID string format."]
    #[serde(rename = "leaseId", default, skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<String>,
    #[doc = "Optional. For a break action, proposed duration the lease should continue before it is broken, in seconds, between 0 and 60."]
    #[serde(rename = "breakPeriod", default, skip_serializing_if = "Option::is_none")]
    pub break_period: Option<i64>,
    #[doc = "Required for acquire. Specifies the duration of the lease, in seconds, or negative one (-1) for a lease that never expires."]
    #[serde(rename = "leaseDuration", default, skip_serializing_if = "Option::is_none")]
    pub lease_duration: Option<i64>,
    #[doc = "Optional for acquire, required for change. Proposed lease ID, in a GUID string format."]
    #[serde(rename = "proposedLeaseId", default, skip_serializing_if = "Option::is_none")]
    pub proposed_lease_id: Option<String>,
}
impl LeaseContainerRequest {
    pub fn new(action: lease_container_request::Action) -> Self {
        Self {
            action,
            lease_id: None,
            break_period: None,
            lease_duration: None,
            proposed_lease_id: None,
        }
    }
}
pub mod lease_container_request {
    use super::*;
    #[doc = "Specifies the lease action. Can be one of the available actions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Acquire,
        Renew,
        Change,
        Release,
        Break,
    }
}
#[doc = "Lease Container response schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LeaseContainerResponse {
    #[doc = "Returned unique lease ID that must be included with any request to delete the container, or to renew, change, or release the lease."]
    #[serde(rename = "leaseId", default, skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<String>,
    #[doc = "Approximate time remaining in the lease period, in seconds."]
    #[serde(rename = "leaseTimeSeconds", default, skip_serializing_if = "Option::is_none")]
    pub lease_time_seconds: Option<String>,
}
impl LeaseContainerResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lease Share request schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeaseShareRequest {
    #[doc = "Specifies the lease action. Can be one of the available actions."]
    pub action: lease_share_request::Action,
    #[doc = "Identifies the lease. Can be specified in any valid GUID string format."]
    #[serde(rename = "leaseId", default, skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<String>,
    #[doc = "Optional. For a break action, proposed duration the lease should continue before it is broken, in seconds, between 0 and 60."]
    #[serde(rename = "breakPeriod", default, skip_serializing_if = "Option::is_none")]
    pub break_period: Option<i32>,
    #[doc = "Required for acquire. Specifies the duration of the lease, in seconds, or negative one (-1) for a lease that never expires."]
    #[serde(rename = "leaseDuration", default, skip_serializing_if = "Option::is_none")]
    pub lease_duration: Option<i32>,
    #[doc = "Optional for acquire, required for change. Proposed lease ID, in a GUID string format."]
    #[serde(rename = "proposedLeaseId", default, skip_serializing_if = "Option::is_none")]
    pub proposed_lease_id: Option<String>,
}
impl LeaseShareRequest {
    pub fn new(action: lease_share_request::Action) -> Self {
        Self {
            action,
            lease_id: None,
            break_period: None,
            lease_duration: None,
            proposed_lease_id: None,
        }
    }
}
pub mod lease_share_request {
    use super::*;
    #[doc = "Specifies the lease action. Can be one of the available actions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Acquire,
        Renew,
        Change,
        Release,
        Break,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Acquire => serializer.serialize_unit_variant("Action", 0u32, "Acquire"),
                Self::Renew => serializer.serialize_unit_variant("Action", 1u32, "Renew"),
                Self::Change => serializer.serialize_unit_variant("Action", 2u32, "Change"),
                Self::Release => serializer.serialize_unit_variant("Action", 3u32, "Release"),
                Self::Break => serializer.serialize_unit_variant("Action", 4u32, "Break"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Lease Share response schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LeaseShareResponse {
    #[doc = "Returned unique lease ID that must be included with any request to delete the share, or to renew, change, or release the lease."]
    #[serde(rename = "leaseId", default, skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<String>,
    #[doc = "Approximate time remaining in the lease period, in seconds."]
    #[serde(rename = "leaseTimeSeconds", default, skip_serializing_if = "Option::is_none")]
    pub lease_time_seconds: Option<String>,
}
impl LeaseShareResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The LegalHold property of a blob container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegalHold {
    #[doc = "The hasLegalHold public property is set to true by SRP if there are at least one existing tag. The hasLegalHold public property is set to false by SRP if all existing legal hold tags are cleared out. There can be a maximum of 1000 blob containers with hasLegalHold=true for a given account."]
    #[serde(rename = "hasLegalHold", default, skip_serializing_if = "Option::is_none")]
    pub has_legal_hold: Option<bool>,
    #[doc = "Each tag should be 3 to 23 alphanumeric characters and is normalized to lower case at SRP."]
    pub tags: Vec<String>,
    #[doc = "When enabled, new blocks can be written to both 'Append and Bock Blobs' while maintaining legal hold protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted."]
    #[serde(rename = "allowProtectedAppendWritesAll", default, skip_serializing_if = "Option::is_none")]
    pub allow_protected_append_writes_all: Option<bool>,
}
impl LegalHold {
    pub fn new(tags: Vec<String>) -> Self {
        Self {
            has_legal_hold: None,
            tags,
            allow_protected_append_writes_all: None,
        }
    }
}
#[doc = "The LegalHold property of a blob container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LegalHoldProperties {
    #[doc = "The hasLegalHold public property is set to true by SRP if there are at least one existing tag. The hasLegalHold public property is set to false by SRP if all existing legal hold tags are cleared out. There can be a maximum of 1000 blob containers with hasLegalHold=true for a given account."]
    #[serde(rename = "hasLegalHold", default, skip_serializing_if = "Option::is_none")]
    pub has_legal_hold: Option<bool>,
    #[doc = "The list of LegalHold tags of a blob container."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagProperty>,
    #[doc = "Protected append writes history setting for the blob container with Legal holds."]
    #[serde(rename = "protectedAppendWritesHistory", default, skip_serializing_if = "Option::is_none")]
    pub protected_append_writes_history: Option<ProtectedAppendWritesHistory>,
}
impl LegalHoldProperties {
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
#[doc = "The blob container properties be listed out."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListContainerItem {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "The properties of a container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContainerProperties>,
}
impl ListContainerItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response schema. Contains list of blobs returned, and if paging is requested or required, a URL to next page of containers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListContainerItems {
    #[doc = "List of blobs containers returned."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ListContainerItem>,
    #[doc = "Request URL that can be used to query next page of containers. Returned when total number of requested containers exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListContainerItems {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListContainerItems {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListQueue {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ListQueueProperties>,
}
impl ListQueue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListQueueProperties {
    #[doc = "A name-value pair that represents queue metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl ListQueueProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response schema. Contains list of queues returned"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListQueueResource {
    #[doc = "List of queues returned."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ListQueue>,
    #[doc = "Request URL that can be used to list next page of queues"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListQueueResource {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListQueueResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListQueueServices {
    #[doc = "List of queue services returned."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QueueServiceProperties>,
}
impl ListQueueServices {
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
#[doc = "Response schema. Contains list of tables returned"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListTableResource {
    #[doc = "List of tables returned."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Table>,
    #[doc = "Request URL that can be used to query next page of tables"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListTableResource {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListTableResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListTableServices {
    #[doc = "List of table services returned."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TableServiceProperties>,
}
impl ListTableServices {
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
#[doc = "Multichannel setting. Applies to Premium FileStorage only."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Multichannel {
    #[doc = "Indicates whether multichannel is enabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl Multichannel {
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
    #[doc = "Sets the resource access rules"]
    #[serde(rename = "resourceAccessRules", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_access_rules: Vec<ResourceAccessRule>,
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
            resource_access_rules: Vec::new(),
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
    #[doc = "Required. Source account name. It should be full resource id if allowCrossTenantReplication set to false."]
    #[serde(rename = "sourceAccount")]
    pub source_account: String,
    #[doc = "Required. Destination account name. It should be full resource id if allowCrossTenantReplication set to false."]
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
#[doc = "Protected append writes history setting for the blob container with Legal holds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectedAppendWritesHistory {
    #[doc = "When enabled, new blocks can be written to both 'Append and Bock Blobs' while maintaining legal hold protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted."]
    #[serde(rename = "allowProtectedAppendWritesAll", default, skip_serializing_if = "Option::is_none")]
    pub allow_protected_append_writes_all: Option<bool>,
    #[doc = "Returns the date and time the tag was added."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
}
impl ProtectedAppendWritesHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protocol settings for file service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtocolSettings {
    #[doc = "Setting for SMB protocol"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smb: Option<SmbSetting>,
}
impl ProtocolSettings {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueueProperties {
    #[doc = "A name-value pair that represents queue metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "Integer indicating an approximate number of messages in the queue. This number is not lower than the actual number of messages in the queue, but could be higher."]
    #[serde(rename = "approximateMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub approximate_message_count: Option<i64>,
}
impl QueueProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a storage account’s Queue service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueueServiceProperties {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a storage account’s Queue service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<queue_service_properties::Properties>,
}
impl QueueServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod queue_service_properties {
    use super::*;
    #[doc = "The properties of a storage account’s Queue service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Sets the CORS rules. You can include up to five CorsRule elements in the request. "]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cors: Option<CorsRules>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource Access Rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceAccessRule {
    #[doc = "Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Resource Id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl ResourceAccessRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The blob service properties for blob restore policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestorePolicyProperties {
    #[doc = "Blob restore is enabled if set to true."]
    pub enabled: bool,
    #[doc = "how long this blob can be restored. It should be great than zero and less than DeleteRetentionPolicy.days."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[doc = "Deprecated in favor of minRestoreTime property."]
    #[serde(rename = "lastEnabledTime", with = "azure_core::date::rfc3339::option")]
    pub last_enabled_time: Option<time::OffsetDateTime>,
    #[doc = "Returns the minimum date and time that the restore can be started."]
    #[serde(rename = "minRestoreTime", with = "azure_core::date::rfc3339::option")]
    pub min_restore_time: Option<time::OffsetDateTime>,
}
impl RestorePolicyProperties {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            days: None,
            last_enabled_time: None,
            min_restore_time: None,
        }
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
#[doc = "SasPolicy assigned to the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasPolicy {
    #[doc = "The SAS expiration period, DD.HH:MM:SS."]
    #[serde(rename = "sasExpirationPeriod")]
    pub sas_expiration_period: String,
    #[doc = "The SAS expiration action. Can only be Log."]
    #[serde(rename = "expirationAction")]
    pub expiration_action: sas_policy::ExpirationAction,
}
impl SasPolicy {
    pub fn new(sas_expiration_period: String, expiration_action: sas_policy::ExpirationAction) -> Self {
        Self {
            sas_expiration_period,
            expiration_action,
        }
    }
}
pub mod sas_policy {
    use super::*;
    #[doc = "The SAS expiration action. Can only be Log."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExpirationAction")]
    pub enum ExpirationAction {
        Log,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExpirationAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExpirationAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExpirationAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Log => serializer.serialize_unit_variant("ExpirationAction", 0u32, "Log"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ExpirationAction {
        fn default() -> Self {
            Self::Log
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignedIdentifier {
    #[doc = "An unique identifier of the stored access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "accessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub access_policy: Option<AccessPolicy>,
}
impl SignedIdentifier {
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
#[doc = "Setting for SMB protocol"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmbSetting {
    #[doc = "Multichannel setting. Applies to Premium FileStorage only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multichannel: Option<Multichannel>,
    #[doc = "SMB protocol versions supported by server. Valid values are SMB2.1, SMB3.0, SMB3.1.1. Should be passed as a string with delimiter ';'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<String>,
    #[doc = "SMB authentication methods supported by server. Valid values are NTLMv2, Kerberos. Should be passed as a string with delimiter ';'."]
    #[serde(rename = "authenticationMethods", default, skip_serializing_if = "Option::is_none")]
    pub authentication_methods: Option<String>,
    #[doc = "Kerberos ticket encryption supported by server. Valid values are RC4-HMAC, AES-256. Should be passed as a string with delimiter ';'"]
    #[serde(rename = "kerberosTicketEncryption", default, skip_serializing_if = "Option::is_none")]
    pub kerberos_ticket_encryption: Option<String>,
    #[doc = "SMB channel encryption supported by server. Valid values are AES-128-CCM, AES-128-GCM, AES-256-GCM. Should be passed as a string with delimiter ';'."]
    #[serde(rename = "channelEncryption", default, skip_serializing_if = "Option::is_none")]
    pub channel_encryption: Option<String>,
}
impl SmbSetting {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
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
            extended_location: None,
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
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
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
            extended_location: None,
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
    #[doc = "Creation time of the key, in round trip date format."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
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
    #[doc = "SasPolicy assigned to the storage account."]
    #[serde(rename = "sasPolicy", default, skip_serializing_if = "Option::is_none")]
    pub sas_policy: Option<SasPolicy>,
    #[doc = "KeyPolicy assigned to the storage account."]
    #[serde(rename = "keyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub key_policy: Option<KeyPolicy>,
    #[doc = "Storage account keys creation time."]
    #[serde(rename = "keyCreationTime", default, skip_serializing_if = "Option::is_none")]
    pub key_creation_time: Option<KeyCreationTime>,
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
    #[doc = "NFS 3.0 protocol support enabled if set to true."]
    #[serde(rename = "isNfsV3Enabled", default, skip_serializing_if = "Option::is_none")]
    pub is_nfs_v3_enabled: Option<bool>,
    #[doc = "Allow or disallow cross AAD tenant object replication. The default interpretation is true for this property."]
    #[serde(rename = "allowCrossTenantReplication", default, skip_serializing_if = "Option::is_none")]
    pub allow_cross_tenant_replication: Option<bool>,
    #[doc = "A boolean flag which indicates whether the default authentication is OAuth or not. The default interpretation is false for this property."]
    #[serde(rename = "defaultToOAuthAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub default_to_o_auth_authentication: Option<bool>,
    #[doc = "Allow or disallow public network access to Storage Account. Value is optional but if passed in, must be 'Enabled' or 'Disabled'."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<storage_account_properties::PublicNetworkAccess>,
    #[doc = "This property enables and defines account-level immutability. Enabling the feature auto-enables Blob Versioning."]
    #[serde(rename = "immutableStorageWithVersioning", default, skip_serializing_if = "Option::is_none")]
    pub immutable_storage_with_versioning: Option<ImmutableStorageAccount>,
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
    #[doc = "Allow or disallow public network access to Storage Account. Value is optional but if passed in, must be 'Enabled' or 'Disabled'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters used to create the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountPropertiesCreateParameters {
    #[doc = "Allow or disallow public network access to Storage Account. Value is optional but if passed in, must be 'Enabled' or 'Disabled'."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<storage_account_properties_create_parameters::PublicNetworkAccess>,
    #[doc = "SasPolicy assigned to the storage account."]
    #[serde(rename = "sasPolicy", default, skip_serializing_if = "Option::is_none")]
    pub sas_policy: Option<SasPolicy>,
    #[doc = "KeyPolicy assigned to the storage account."]
    #[serde(rename = "keyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub key_policy: Option<KeyPolicy>,
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
    #[doc = "NFS 3.0 protocol support enabled if set to true."]
    #[serde(rename = "isNfsV3Enabled", default, skip_serializing_if = "Option::is_none")]
    pub is_nfs_v3_enabled: Option<bool>,
    #[doc = "Allow or disallow cross AAD tenant object replication. The default interpretation is true for this property."]
    #[serde(rename = "allowCrossTenantReplication", default, skip_serializing_if = "Option::is_none")]
    pub allow_cross_tenant_replication: Option<bool>,
    #[doc = "A boolean flag which indicates whether the default authentication is OAuth or not. The default interpretation is false for this property."]
    #[serde(rename = "defaultToOAuthAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub default_to_o_auth_authentication: Option<bool>,
    #[doc = "This property enables and defines account-level immutability. Enabling the feature auto-enables Blob Versioning."]
    #[serde(rename = "immutableStorageWithVersioning", default, skip_serializing_if = "Option::is_none")]
    pub immutable_storage_with_versioning: Option<ImmutableStorageAccount>,
}
impl StorageAccountPropertiesCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_properties_create_parameters {
    use super::*;
    #[doc = "Allow or disallow public network access to Storage Account. Value is optional but if passed in, must be 'Enabled' or 'Disabled'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "The parameters used when updating a storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountPropertiesUpdateParameters {
    #[doc = "The custom domain assigned to this storage account. This can be set via Update."]
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[doc = "The encryption settings on the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "SasPolicy assigned to the storage account."]
    #[serde(rename = "sasPolicy", default, skip_serializing_if = "Option::is_none")]
    pub sas_policy: Option<SasPolicy>,
    #[doc = "KeyPolicy assigned to the storage account."]
    #[serde(rename = "keyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub key_policy: Option<KeyPolicy>,
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
    #[doc = "Allow or disallow cross AAD tenant object replication. The default interpretation is true for this property."]
    #[serde(rename = "allowCrossTenantReplication", default, skip_serializing_if = "Option::is_none")]
    pub allow_cross_tenant_replication: Option<bool>,
    #[doc = "A boolean flag which indicates whether the default authentication is OAuth or not. The default interpretation is false for this property."]
    #[serde(rename = "defaultToOAuthAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub default_to_o_auth_authentication: Option<bool>,
    #[doc = "Allow or disallow public network access to Storage Account. Value is optional but if passed in, must be 'Enabled' or 'Disabled'."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<storage_account_properties_update_parameters::PublicNetworkAccess>,
    #[doc = "This property enables and defines account-level immutability. Enabling the feature auto-enables Blob Versioning."]
    #[serde(rename = "immutableStorageWithVersioning", default, skip_serializing_if = "Option::is_none")]
    pub immutable_storage_with_versioning: Option<ImmutableStorageAccount>,
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
    #[doc = "Allow or disallow public network access to Storage Account. Value is optional but if passed in, must be 'Enabled' or 'Disabled'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageQueue {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueueProperties>,
}
impl StorageQueue {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Properties of the table, including Id, resource name, resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Table {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TableProperties>,
}
impl Table {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableProperties {
    #[doc = "Table name under the specified account"]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}
impl TableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a storage account’s Table service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableServiceProperties {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a storage account’s Table service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<table_service_properties::Properties>,
}
impl TableServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod table_service_properties {
    use super::*;
    #[doc = "The properties of a storage account’s Table service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Sets the CORS rules. You can include up to five CorsRule elements in the request. "]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cors: Option<CorsRules>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
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
#[doc = "A tag of the LegalHold of a blob container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagProperty {
    #[doc = "The tag value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "Returns the date and time the tag was added."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Returns the Object ID of the user who added the tag."]
    #[serde(rename = "objectIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    #[doc = "Returns the Tenant ID that issued the token for the user who added the tag."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Returns the User Principal Name of the user who added the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upn: Option<String>,
}
impl TagProperty {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "An update history of the ImmutabilityPolicy of a blob container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateHistoryProperty {
    #[doc = "The ImmutabilityPolicy update type of a blob container, possible values include: put, lock and extend."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<update_history_property::Update>,
    #[doc = "The immutability period for the blobs in the container since the policy creation, in days."]
    #[serde(rename = "immutabilityPeriodSinceCreationInDays", default, skip_serializing_if = "Option::is_none")]
    pub immutability_period_since_creation_in_days: Option<i64>,
    #[doc = "Returns the date and time the ImmutabilityPolicy was updated."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Returns the Object ID of the user who updated the ImmutabilityPolicy."]
    #[serde(rename = "objectIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    #[doc = "Returns the Tenant ID that issued the token for the user who updated the ImmutabilityPolicy."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Returns the User Principal Name of the user who updated the ImmutabilityPolicy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upn: Option<String>,
    #[doc = "This property can only be changed for unlocked time-based retention policies. When enabled, new blocks can be written to an append blob while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted. This property cannot be changed with ExtendImmutabilityPolicy API."]
    #[serde(rename = "allowProtectedAppendWrites", default, skip_serializing_if = "Option::is_none")]
    pub allow_protected_append_writes: Option<bool>,
    #[doc = "This property can only be changed for unlocked time-based retention policies. When enabled, new blocks can be written to both 'Append and Bock Blobs' while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted. This property cannot be changed with ExtendImmutabilityPolicy API. The 'allowProtectedAppendWrites' and 'allowProtectedAppendWritesAll' properties are mutually exclusive."]
    #[serde(rename = "allowProtectedAppendWritesAll", default, skip_serializing_if = "Option::is_none")]
    pub allow_protected_append_writes_all: Option<bool>,
}
impl UpdateHistoryProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_history_property {
    use super::*;
    #[doc = "The ImmutabilityPolicy update type of a blob container, possible values include: put, lock and extend."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Update")]
    pub enum Update {
        #[serde(rename = "put")]
        Put,
        #[serde(rename = "lock")]
        Lock,
        #[serde(rename = "extend")]
        Extend,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Update {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Update {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Update {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Put => serializer.serialize_unit_variant("Update", 0u32, "put"),
                Self::Lock => serializer.serialize_unit_variant("Update", 1u32, "lock"),
                Self::Extend => serializer.serialize_unit_variant("Update", 2u32, "extend"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "UserAssignedIdentity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
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
    #[serde(remote = "State")]
    pub enum State {
        Provisioning,
        Deprovisioning,
        Succeeded,
        Failed,
        NetworkSourceDeleted,
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
                Self::Provisioning => serializer.serialize_unit_variant("State", 0u32, "Provisioning"),
                Self::Deprovisioning => serializer.serialize_unit_variant("State", 1u32, "Deprovisioning"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::NetworkSourceDeleted => serializer.serialize_unit_variant("State", 4u32, "NetworkSourceDeleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
