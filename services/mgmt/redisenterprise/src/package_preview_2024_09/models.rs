#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The secret access keys used for authenticating connections to redis"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessKeys {
    #[doc = "The current primary key that clients can use to authenticate"]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The current secondary key that clients can use to authenticate"]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl AccessKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the access policy assignment of Redis Enterprise database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of Redis Enterprise database access policy assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessPolicyAssignmentProperties>,
}
impl AccessPolicyAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list-all operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicyAssignmentList {
    #[doc = "List of access policy assignments."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessPolicyAssignment>,
    #[doc = "The URI to fetch the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessPolicyAssignmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AccessPolicyAssignmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Redis Enterprise database access policy assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyAssignmentProperties {
    #[doc = "Current provisioning status"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Name of access policy under specific access policy assignment. Only \"default\" policy is supported for now."]
    #[serde(rename = "accessPolicyName")]
    pub access_policy_name: String,
    #[doc = "The user associated with the access policy."]
    pub user: access_policy_assignment_properties::User,
}
impl AccessPolicyAssignmentProperties {
    pub fn new(access_policy_name: String, user: access_policy_assignment_properties::User) -> Self {
        Self {
            provisioning_state: None,
            access_policy_name,
            user,
        }
    }
}
pub mod access_policy_assignment_properties {
    use super::*;
    #[doc = "The user associated with the access policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct User {
        #[doc = "The object ID of the user."]
        #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
        pub object_id: Option<String>,
    }
    impl User {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Describes the Redis Enterprise cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SKU parameters supplied to the create Redis Enterprise cluster operation."]
    pub sku: Sku,
    #[doc = "The Availability Zones where this cluster will be deployed."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Properties of Redis Enterprise clusters, as opposed to general resource properties like location, tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
impl Cluster {
    pub fn new(tracked_resource: TrackedResource, sku: Sku) -> Self {
        Self {
            tracked_resource,
            sku,
            zones: Vec::new(),
            identity: None,
            properties: None,
        }
    }
}
#[doc = "The response of a list-all operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterList {
    #[doc = "List of clusters."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Cluster>,
    #[doc = "The URI to fetch the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClusterList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ClusterList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Redis Enterprise clusters, as opposed to general resource properties like location, tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProperties {
    #[doc = "Enabled by default. If highAvailability is disabled, the data set is not replicated. This affects the availability SLA, and increases the risk of data loss."]
    #[serde(rename = "highAvailability", default, skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<cluster_properties::HighAvailability>,
    #[doc = "The minimum TLS version for the cluster to support, e.g. '1.2'. Newer versions can be added in the future. Note that TLS 1.0 and TLS 1.1 are now completely obsolete -- you cannot use them. They are mentioned only for the sake of consistency with old API versions."]
    #[serde(rename = "minimumTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<cluster_properties::MinimumTlsVersion>,
    #[doc = "Encryption-at-rest configuration for the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<cluster_properties::Encryption>,
    #[doc = "DNS name of the cluster endpoint"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Current provisioning status"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Explains the current redundancy strategy of the cluster, which affects the expected SLA."]
    #[serde(rename = "redundancyMode", default, skip_serializing_if = "Option::is_none")]
    pub redundancy_mode: Option<cluster_properties::RedundancyMode>,
    #[doc = "Current resource status"]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
    #[doc = "Version of redis the cluster supports, e.g. '6'"]
    #[serde(rename = "redisVersion", default, skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,
    #[doc = "List of private endpoint connections associated with the specified Redis Enterprise cluster"]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl ClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_properties {
    use super::*;
    #[doc = "Enabled by default. If highAvailability is disabled, the data set is not replicated. This affects the availability SLA, and increases the risk of data loss."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HighAvailability")]
    pub enum HighAvailability {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HighAvailability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HighAvailability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HighAvailability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("HighAvailability", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("HighAvailability", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The minimum TLS version for the cluster to support, e.g. '1.2'. Newer versions can be added in the future. Note that TLS 1.0 and TLS 1.1 are now completely obsolete -- you cannot use them. They are mentioned only for the sake of consistency with old API versions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MinimumTlsVersion")]
    pub enum MinimumTlsVersion {
        #[serde(rename = "1.0")]
        N1_0,
        #[serde(rename = "1.1")]
        N1_1,
        #[serde(rename = "1.2")]
        N1_2,
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
                Self::N1_0 => serializer.serialize_unit_variant("MinimumTlsVersion", 0u32, "1.0"),
                Self::N1_1 => serializer.serialize_unit_variant("MinimumTlsVersion", 1u32, "1.1"),
                Self::N1_2 => serializer.serialize_unit_variant("MinimumTlsVersion", 2u32, "1.2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Encryption-at-rest configuration for the cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Encryption {
        #[doc = "All Customer-managed key encryption properties for the resource. Set this to an empty object to use Microsoft-managed key encryption."]
        #[serde(rename = "customerManagedKeyEncryption", default, skip_serializing_if = "Option::is_none")]
        pub customer_managed_key_encryption: Option<encryption::CustomerManagedKeyEncryption>,
    }
    impl Encryption {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod encryption {
        use super::*;
        #[doc = "All Customer-managed key encryption properties for the resource. Set this to an empty object to use Microsoft-managed key encryption."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct CustomerManagedKeyEncryption {
            #[doc = "All identity configuration for Customer-managed key settings defining which identity should be used to auth to Key Vault."]
            #[serde(rename = "keyEncryptionKeyIdentity", default, skip_serializing_if = "Option::is_none")]
            pub key_encryption_key_identity: Option<customer_managed_key_encryption::KeyEncryptionKeyIdentity>,
            #[doc = "Key encryption key Url, versioned only. Ex: https://contosovault.vault.azure.net/keys/contosokek/562a4bb76b524a1493a6afe8e536ee78"]
            #[serde(rename = "keyEncryptionKeyUrl", default, skip_serializing_if = "Option::is_none")]
            pub key_encryption_key_url: Option<String>,
        }
        impl CustomerManagedKeyEncryption {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod customer_managed_key_encryption {
            use super::*;
            #[doc = "All identity configuration for Customer-managed key settings defining which identity should be used to auth to Key Vault."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
            pub struct KeyEncryptionKeyIdentity {
                #[doc = "User assigned identity to use for accessing key encryption key Url. Ex: /subscriptions/<sub uuid>/resourceGroups/<resource group>/providers/Microsoft.ManagedIdentity/userAssignedIdentities/myId."]
                #[serde(rename = "userAssignedIdentityResourceId", default, skip_serializing_if = "Option::is_none")]
                pub user_assigned_identity_resource_id: Option<String>,
                #[doc = "Only userAssignedIdentity is supported in this API version; other types may be supported in the future"]
                #[serde(rename = "identityType", default, skip_serializing_if = "Option::is_none")]
                pub identity_type: Option<key_encryption_key_identity::IdentityType>,
            }
            impl KeyEncryptionKeyIdentity {
                pub fn new() -> Self {
                    Self::default()
                }
            }
            pub mod key_encryption_key_identity {
                use super::*;
                #[doc = "Only userAssignedIdentity is supported in this API version; other types may be supported in the future"]
                #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
                #[serde(remote = "IdentityType")]
                pub enum IdentityType {
                    #[serde(rename = "systemAssignedIdentity")]
                    SystemAssignedIdentity,
                    #[serde(rename = "userAssignedIdentity")]
                    UserAssignedIdentity,
                    #[serde(skip_deserializing)]
                    UnknownValue(String),
                }
                impl FromStr for IdentityType {
                    type Err = value::Error;
                    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                        Self::deserialize(s.into_deserializer())
                    }
                }
                impl<'de> Deserialize<'de> for IdentityType {
                    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        let s = String::deserialize(deserializer)?;
                        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                        Ok(deserialized)
                    }
                }
                impl Serialize for IdentityType {
                    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        match self {
                            Self::SystemAssignedIdentity => {
                                serializer.serialize_unit_variant("IdentityType", 0u32, "systemAssignedIdentity")
                            }
                            Self::UserAssignedIdentity => serializer.serialize_unit_variant("IdentityType", 1u32, "userAssignedIdentity"),
                            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                        }
                    }
                }
            }
        }
    }
    #[doc = "Explains the current redundancy strategy of the cluster, which affects the expected SLA."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RedundancyMode")]
    pub enum RedundancyMode {
        None,
        #[serde(rename = "LR")]
        Lr,
        #[serde(rename = "ZR")]
        Zr,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RedundancyMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RedundancyMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RedundancyMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("RedundancyMode", 0u32, "None"),
                Self::Lr => serializer.serialize_unit_variant("RedundancyMode", 1u32, "LR"),
                Self::Zr => serializer.serialize_unit_variant("RedundancyMode", 2u32, "ZR"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A partial update to the Redis Enterprise cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdate {
    #[doc = "SKU parameters supplied to the create Redis Enterprise cluster operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of Redis Enterprise clusters, as opposed to general resource properties like location, tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ClusterUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a database on the Redis Enterprise cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of Redis Enterprise databases, as opposed to general resource properties like location, tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
impl Database {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list-all operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseList {
    #[doc = "List of databases"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Database>,
    #[doc = "The URI to fetch the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Redis Enterprise databases, as opposed to general resource properties like location, tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseProperties {
    #[doc = "Specifies whether redis clients can connect using TLS-encrypted or plaintext redis protocols. Default is TLS-encrypted."]
    #[serde(rename = "clientProtocol", default, skip_serializing_if = "Option::is_none")]
    pub client_protocol: Option<database_properties::ClientProtocol>,
    #[doc = "TCP port of the database endpoint. Specified at create time. Defaults to an available port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Current provisioning status"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Current resource status"]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
    #[doc = "Clustering policy - default is OSSCluster. This property must be chosen at create time, and cannot be changed without deleting the database."]
    #[serde(rename = "clusteringPolicy", default, skip_serializing_if = "Option::is_none")]
    pub clustering_policy: Option<database_properties::ClusteringPolicy>,
    #[doc = "Redis eviction policy - default is VolatileLRU"]
    #[serde(rename = "evictionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<database_properties::EvictionPolicy>,
    #[doc = "Persistence-related configuration for the Redis Enterprise database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistence: Option<Persistence>,
    #[doc = "Optional set of redis modules to enable in this database - modules can only be added at creation time."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub modules: Vec<Module>,
    #[doc = "Optional set of properties to configure geo replication for this database."]
    #[serde(rename = "geoReplication", default, skip_serializing_if = "Option::is_none")]
    pub geo_replication: Option<database_properties::GeoReplication>,
    #[doc = "Version of Redis the database is running on, e.g. '6.0'"]
    #[serde(rename = "redisVersion", default, skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,
    #[doc = "Option to defer upgrade when newest version is released - default is NotDeferred. Learn more: https://aka.ms/redisversionupgrade"]
    #[serde(rename = "deferUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub defer_upgrade: Option<database_properties::DeferUpgrade>,
    #[doc = "This property can be Enabled/Disabled to allow or deny access with the current access keys. Can be updated even after database is created."]
    #[serde(rename = "accessKeysAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub access_keys_authentication: Option<database_properties::AccessKeysAuthentication>,
}
impl DatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_properties {
    use super::*;
    #[doc = "Specifies whether redis clients can connect using TLS-encrypted or plaintext redis protocols. Default is TLS-encrypted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClientProtocol")]
    pub enum ClientProtocol {
        Encrypted,
        Plaintext,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClientProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClientProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClientProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Encrypted => serializer.serialize_unit_variant("ClientProtocol", 0u32, "Encrypted"),
                Self::Plaintext => serializer.serialize_unit_variant("ClientProtocol", 1u32, "Plaintext"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Clustering policy - default is OSSCluster. This property must be chosen at create time, and cannot be changed without deleting the database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClusteringPolicy")]
    pub enum ClusteringPolicy {
        EnterpriseCluster,
        #[serde(rename = "OSSCluster")]
        OssCluster,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClusteringPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClusteringPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClusteringPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EnterpriseCluster => serializer.serialize_unit_variant("ClusteringPolicy", 0u32, "EnterpriseCluster"),
                Self::OssCluster => serializer.serialize_unit_variant("ClusteringPolicy", 1u32, "OSSCluster"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Redis eviction policy - default is VolatileLRU"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EvictionPolicy")]
    pub enum EvictionPolicy {
        #[serde(rename = "AllKeysLFU")]
        AllKeysLfu,
        #[serde(rename = "AllKeysLRU")]
        AllKeysLru,
        AllKeysRandom,
        #[serde(rename = "VolatileLRU")]
        VolatileLru,
        #[serde(rename = "VolatileLFU")]
        VolatileLfu,
        #[serde(rename = "VolatileTTL")]
        VolatileTtl,
        VolatileRandom,
        NoEviction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EvictionPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EvictionPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EvictionPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllKeysLfu => serializer.serialize_unit_variant("EvictionPolicy", 0u32, "AllKeysLFU"),
                Self::AllKeysLru => serializer.serialize_unit_variant("EvictionPolicy", 1u32, "AllKeysLRU"),
                Self::AllKeysRandom => serializer.serialize_unit_variant("EvictionPolicy", 2u32, "AllKeysRandom"),
                Self::VolatileLru => serializer.serialize_unit_variant("EvictionPolicy", 3u32, "VolatileLRU"),
                Self::VolatileLfu => serializer.serialize_unit_variant("EvictionPolicy", 4u32, "VolatileLFU"),
                Self::VolatileTtl => serializer.serialize_unit_variant("EvictionPolicy", 5u32, "VolatileTTL"),
                Self::VolatileRandom => serializer.serialize_unit_variant("EvictionPolicy", 6u32, "VolatileRandom"),
                Self::NoEviction => serializer.serialize_unit_variant("EvictionPolicy", 7u32, "NoEviction"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Optional set of properties to configure geo replication for this database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct GeoReplication {
        #[doc = "Name for the group of linked database resources"]
        #[serde(rename = "groupNickname", default, skip_serializing_if = "Option::is_none")]
        pub group_nickname: Option<String>,
        #[doc = "List of database resources to link with this database"]
        #[serde(
            rename = "linkedDatabases",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub linked_databases: Vec<LinkedDatabase>,
    }
    impl GeoReplication {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Option to defer upgrade when newest version is released - default is NotDeferred. Learn more: https://aka.ms/redisversionupgrade"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeferUpgrade")]
    pub enum DeferUpgrade {
        Deferred,
        NotDeferred,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeferUpgrade {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeferUpgrade {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeferUpgrade {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Deferred => serializer.serialize_unit_variant("DeferUpgrade", 0u32, "Deferred"),
                Self::NotDeferred => serializer.serialize_unit_variant("DeferUpgrade", 1u32, "NotDeferred"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "This property can be Enabled/Disabled to allow or deny access with the current access keys. Can be updated even after database is created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessKeysAuthentication")]
    pub enum AccessKeysAuthentication {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessKeysAuthentication {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessKeysAuthentication {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessKeysAuthentication {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("AccessKeysAuthentication", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("AccessKeysAuthentication", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A partial update to the Redis Enterprise database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUpdate {
    #[doc = "Properties of Redis Enterprise databases, as opposed to general resource properties like location, tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
impl DatabaseUpdate {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Parameters for a Redis Enterprise export operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportClusterParameters {
    #[doc = "SAS URI for the target directory to export to"]
    #[serde(rename = "sasUri")]
    pub sas_uri: String,
}
impl ExportClusterParameters {
    pub fn new(sas_uri: String) -> Self {
        Self { sas_uri }
    }
}
#[doc = "Parameters for a Redis Enterprise active geo-replication flush operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlushParameters {
    #[doc = "The identifiers of all the other database resources in the georeplication group to be flushed."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ids: Vec<String>,
}
impl FlushParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for reconfiguring active geo-replication, of an existing database that was previously unlinked from a replication group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForceLinkParameters {
    #[doc = "The name of the group of linked database resources. This should match the existing replication group name."]
    #[serde(rename = "groupNickname")]
    pub group_nickname: String,
    #[doc = "The resource IDs of the databases that are expected to be linked and included in the replication group. This parameter is used to validate that the linking is to the expected (unlinked) part of the replication group, if it is splintered."]
    #[serde(rename = "linkedDatabases")]
    pub linked_databases: Vec<LinkedDatabase>,
}
impl ForceLinkParameters {
    pub fn new(group_nickname: String, linked_databases: Vec<LinkedDatabase>) -> Self {
        Self {
            group_nickname,
            linked_databases,
        }
    }
}
#[doc = "Parameters for a redis enterprise active geo-replication force unlink operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForceUnlinkParameters {
    #[doc = "The resource IDs of the database resources to be unlinked."]
    pub ids: Vec<String>,
}
impl ForceUnlinkParameters {
    pub fn new(ids: Vec<String>) -> Self {
        Self { ids }
    }
}
#[doc = "Parameters for a Redis Enterprise import operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportClusterParameters {
    #[doc = "SAS URIs for the target blobs to import from"]
    #[serde(rename = "sasUris")]
    pub sas_uris: Vec<String>,
}
impl ImportClusterParameters {
    pub fn new(sas_uris: Vec<String>) -> Self {
        Self { sas_uris }
    }
}
#[doc = "Specifies details of a linked database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedDatabase {
    #[doc = "Resource ID of a database resource to link with this database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "State of the link between the database resources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<linked_database::State>,
}
impl LinkedDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod linked_database {
    use super::*;
    #[doc = "State of the link between the database resources."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Linked,
        Linking,
        Unlinking,
        LinkFailed,
        UnlinkFailed,
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
                Self::Linked => serializer.serialize_unit_variant("State", 0u32, "Linked"),
                Self::Linking => serializer.serialize_unit_variant("State", 1u32, "Linking"),
                Self::Unlinking => serializer.serialize_unit_variant("State", 2u32, "Unlinking"),
                Self::LinkFailed => serializer.serialize_unit_variant("State", 3u32, "LinkFailed"),
                Self::UnlinkFailed => serializer.serialize_unit_variant("State", 4u32, "UnlinkFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned, UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned, UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies configuration of a redis module"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Module {
    #[doc = "The name of the module, e.g. 'RedisBloom', 'RediSearch', 'RedisTimeSeries'"]
    pub name: String,
    #[doc = "Configuration options for the module, e.g. 'ERROR_RATE 0.01 INITIAL_SIZE 400'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
    #[doc = "The version of the module, e.g. '1.0'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: None,
            version: None,
        }
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
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
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of a long-running operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "The operation's unique id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The operation's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The current status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Persistence-related configuration for the Redis Enterprise database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Persistence {
    #[doc = "Sets whether AOF is enabled. Note that at most one of AOF or RDB persistence may be enabled."]
    #[serde(rename = "aofEnabled", default, skip_serializing_if = "Option::is_none")]
    pub aof_enabled: Option<bool>,
    #[doc = "Sets whether RDB is enabled. Note that at most one of AOF or RDB persistence may be enabled."]
    #[serde(rename = "rdbEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rdb_enabled: Option<bool>,
    #[doc = "Sets the frequency at which data is written to disk. Defaults to '1s', meaning 'every second'. Note that the 'always' setting is deprecated, because of its performance impact."]
    #[serde(rename = "aofFrequency", default, skip_serializing_if = "Option::is_none")]
    pub aof_frequency: Option<persistence::AofFrequency>,
    #[doc = "Sets the frequency at which a snapshot of the database is created."]
    #[serde(rename = "rdbFrequency", default, skip_serializing_if = "Option::is_none")]
    pub rdb_frequency: Option<persistence::RdbFrequency>,
}
impl Persistence {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod persistence {
    use super::*;
    #[doc = "Sets the frequency at which data is written to disk. Defaults to '1s', meaning 'every second'. Note that the 'always' setting is deprecated, because of its performance impact."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AofFrequency")]
    pub enum AofFrequency {
        #[serde(rename = "1s")]
        N1s,
        #[serde(rename = "always")]
        Always,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AofFrequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AofFrequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AofFrequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N1s => serializer.serialize_unit_variant("AofFrequency", 0u32, "1s"),
                Self::Always => serializer.serialize_unit_variant("AofFrequency", 1u32, "always"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Sets the frequency at which a snapshot of the database is created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RdbFrequency")]
    pub enum RdbFrequency {
        #[serde(rename = "1h")]
        N1h,
        #[serde(rename = "6h")]
        N6h,
        #[serde(rename = "12h")]
        N12h,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RdbFrequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RdbFrequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RdbFrequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N1h => serializer.serialize_unit_variant("RdbFrequency", 0u32, "1h"),
                Self::N6h => serializer.serialize_unit_variant("RdbFrequency", 1u32, "6h"),
                Self::N12h => serializer.serialize_unit_variant("RdbFrequency", 2u32, "12h"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateLinkResource>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
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
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current provisioning status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Creating,
    Updating,
    Deleting,
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
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
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
#[doc = "Specifies which access keys to reset to a new random value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateKeyParameters {
    #[doc = "Which access key to regenerate."]
    #[serde(rename = "keyType")]
    pub key_type: regenerate_key_parameters::KeyType,
}
impl RegenerateKeyParameters {
    pub fn new(key_type: regenerate_key_parameters::KeyType) -> Self {
        Self { key_type }
    }
}
pub mod regenerate_key_parameters {
    use super::*;
    #[doc = "Which access key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        Primary,
        Secondary,
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
#[doc = "Current resource status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceState")]
pub enum ResourceState {
    Running,
    Creating,
    CreateFailed,
    Updating,
    UpdateFailed,
    Deleting,
    DeleteFailed,
    Enabling,
    EnableFailed,
    Disabling,
    DisableFailed,
    Disabled,
    Scaling,
    ScalingFailed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Running => serializer.serialize_unit_variant("ResourceState", 0u32, "Running"),
            Self::Creating => serializer.serialize_unit_variant("ResourceState", 1u32, "Creating"),
            Self::CreateFailed => serializer.serialize_unit_variant("ResourceState", 2u32, "CreateFailed"),
            Self::Updating => serializer.serialize_unit_variant("ResourceState", 3u32, "Updating"),
            Self::UpdateFailed => serializer.serialize_unit_variant("ResourceState", 4u32, "UpdateFailed"),
            Self::Deleting => serializer.serialize_unit_variant("ResourceState", 5u32, "Deleting"),
            Self::DeleteFailed => serializer.serialize_unit_variant("ResourceState", 6u32, "DeleteFailed"),
            Self::Enabling => serializer.serialize_unit_variant("ResourceState", 7u32, "Enabling"),
            Self::EnableFailed => serializer.serialize_unit_variant("ResourceState", 8u32, "EnableFailed"),
            Self::Disabling => serializer.serialize_unit_variant("ResourceState", 9u32, "Disabling"),
            Self::DisableFailed => serializer.serialize_unit_variant("ResourceState", 10u32, "DisableFailed"),
            Self::Disabled => serializer.serialize_unit_variant("ResourceState", 11u32, "Disabled"),
            Self::Scaling => serializer.serialize_unit_variant("ResourceState", 12u32, "Scaling"),
            Self::ScalingFailed => serializer.serialize_unit_variant("ResourceState", 13u32, "ScalingFailed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SKU parameters supplied to the create Redis Enterprise cluster operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The level of Redis Enterprise cluster to deploy. Possible values: ('Balanced_B5', 'MemoryOptimized_M10', 'ComputeOptimized_X5', etc.). For more information on SKUs see the latest pricing documentation. Note that additional SKUs may become supported in the future."]
    pub name: sku::Name,
    #[doc = "This property is only used with Enterprise and EnterpriseFlash SKUs. Determines the size of the cluster. Valid values are (2, 4, 6, ...) for Enterprise SKUs and (3, 9, 15, ...) for EnterpriseFlash SKUs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name, capacity: None }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The level of Redis Enterprise cluster to deploy. Possible values: ('Balanced_B5', 'MemoryOptimized_M10', 'ComputeOptimized_X5', etc.). For more information on SKUs see the latest pricing documentation. Note that additional SKUs may become supported in the future."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "Enterprise_E1")]
        EnterpriseE1,
        #[serde(rename = "Enterprise_E5")]
        EnterpriseE5,
        #[serde(rename = "Enterprise_E10")]
        EnterpriseE10,
        #[serde(rename = "Enterprise_E20")]
        EnterpriseE20,
        #[serde(rename = "Enterprise_E50")]
        EnterpriseE50,
        #[serde(rename = "Enterprise_E100")]
        EnterpriseE100,
        #[serde(rename = "Enterprise_E200")]
        EnterpriseE200,
        #[serde(rename = "Enterprise_E400")]
        EnterpriseE400,
        #[serde(rename = "EnterpriseFlash_F300")]
        EnterpriseFlashF300,
        #[serde(rename = "EnterpriseFlash_F700")]
        EnterpriseFlashF700,
        #[serde(rename = "EnterpriseFlash_F1500")]
        EnterpriseFlashF1500,
        #[serde(rename = "Balanced_B0")]
        BalancedB0,
        #[serde(rename = "Balanced_B1")]
        BalancedB1,
        #[serde(rename = "Balanced_B3")]
        BalancedB3,
        #[serde(rename = "Balanced_B5")]
        BalancedB5,
        #[serde(rename = "Balanced_B10")]
        BalancedB10,
        #[serde(rename = "Balanced_B20")]
        BalancedB20,
        #[serde(rename = "Balanced_B50")]
        BalancedB50,
        #[serde(rename = "Balanced_B100")]
        BalancedB100,
        #[serde(rename = "Balanced_B150")]
        BalancedB150,
        #[serde(rename = "Balanced_B250")]
        BalancedB250,
        #[serde(rename = "Balanced_B350")]
        BalancedB350,
        #[serde(rename = "Balanced_B500")]
        BalancedB500,
        #[serde(rename = "Balanced_B700")]
        BalancedB700,
        #[serde(rename = "Balanced_B1000")]
        BalancedB1000,
        #[serde(rename = "MemoryOptimized_M10")]
        MemoryOptimizedM10,
        #[serde(rename = "MemoryOptimized_M20")]
        MemoryOptimizedM20,
        #[serde(rename = "MemoryOptimized_M50")]
        MemoryOptimizedM50,
        #[serde(rename = "MemoryOptimized_M100")]
        MemoryOptimizedM100,
        #[serde(rename = "MemoryOptimized_M150")]
        MemoryOptimizedM150,
        #[serde(rename = "MemoryOptimized_M250")]
        MemoryOptimizedM250,
        #[serde(rename = "MemoryOptimized_M350")]
        MemoryOptimizedM350,
        #[serde(rename = "MemoryOptimized_M500")]
        MemoryOptimizedM500,
        #[serde(rename = "MemoryOptimized_M700")]
        MemoryOptimizedM700,
        #[serde(rename = "MemoryOptimized_M1000")]
        MemoryOptimizedM1000,
        #[serde(rename = "MemoryOptimized_M1500")]
        MemoryOptimizedM1500,
        #[serde(rename = "MemoryOptimized_M2000")]
        MemoryOptimizedM2000,
        #[serde(rename = "ComputeOptimized_X3")]
        ComputeOptimizedX3,
        #[serde(rename = "ComputeOptimized_X5")]
        ComputeOptimizedX5,
        #[serde(rename = "ComputeOptimized_X10")]
        ComputeOptimizedX10,
        #[serde(rename = "ComputeOptimized_X20")]
        ComputeOptimizedX20,
        #[serde(rename = "ComputeOptimized_X50")]
        ComputeOptimizedX50,
        #[serde(rename = "ComputeOptimized_X100")]
        ComputeOptimizedX100,
        #[serde(rename = "ComputeOptimized_X150")]
        ComputeOptimizedX150,
        #[serde(rename = "ComputeOptimized_X250")]
        ComputeOptimizedX250,
        #[serde(rename = "ComputeOptimized_X350")]
        ComputeOptimizedX350,
        #[serde(rename = "ComputeOptimized_X500")]
        ComputeOptimizedX500,
        #[serde(rename = "ComputeOptimized_X700")]
        ComputeOptimizedX700,
        #[serde(rename = "FlashOptimized_A250")]
        FlashOptimizedA250,
        #[serde(rename = "FlashOptimized_A500")]
        FlashOptimizedA500,
        #[serde(rename = "FlashOptimized_A700")]
        FlashOptimizedA700,
        #[serde(rename = "FlashOptimized_A1000")]
        FlashOptimizedA1000,
        #[serde(rename = "FlashOptimized_A1500")]
        FlashOptimizedA1500,
        #[serde(rename = "FlashOptimized_A2000")]
        FlashOptimizedA2000,
        #[serde(rename = "FlashOptimized_A4500")]
        FlashOptimizedA4500,
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
                Self::EnterpriseE1 => serializer.serialize_unit_variant("Name", 0u32, "Enterprise_E1"),
                Self::EnterpriseE5 => serializer.serialize_unit_variant("Name", 1u32, "Enterprise_E5"),
                Self::EnterpriseE10 => serializer.serialize_unit_variant("Name", 2u32, "Enterprise_E10"),
                Self::EnterpriseE20 => serializer.serialize_unit_variant("Name", 3u32, "Enterprise_E20"),
                Self::EnterpriseE50 => serializer.serialize_unit_variant("Name", 4u32, "Enterprise_E50"),
                Self::EnterpriseE100 => serializer.serialize_unit_variant("Name", 5u32, "Enterprise_E100"),
                Self::EnterpriseE200 => serializer.serialize_unit_variant("Name", 6u32, "Enterprise_E200"),
                Self::EnterpriseE400 => serializer.serialize_unit_variant("Name", 7u32, "Enterprise_E400"),
                Self::EnterpriseFlashF300 => serializer.serialize_unit_variant("Name", 8u32, "EnterpriseFlash_F300"),
                Self::EnterpriseFlashF700 => serializer.serialize_unit_variant("Name", 9u32, "EnterpriseFlash_F700"),
                Self::EnterpriseFlashF1500 => serializer.serialize_unit_variant("Name", 10u32, "EnterpriseFlash_F1500"),
                Self::BalancedB0 => serializer.serialize_unit_variant("Name", 11u32, "Balanced_B0"),
                Self::BalancedB1 => serializer.serialize_unit_variant("Name", 12u32, "Balanced_B1"),
                Self::BalancedB3 => serializer.serialize_unit_variant("Name", 13u32, "Balanced_B3"),
                Self::BalancedB5 => serializer.serialize_unit_variant("Name", 14u32, "Balanced_B5"),
                Self::BalancedB10 => serializer.serialize_unit_variant("Name", 15u32, "Balanced_B10"),
                Self::BalancedB20 => serializer.serialize_unit_variant("Name", 16u32, "Balanced_B20"),
                Self::BalancedB50 => serializer.serialize_unit_variant("Name", 17u32, "Balanced_B50"),
                Self::BalancedB100 => serializer.serialize_unit_variant("Name", 18u32, "Balanced_B100"),
                Self::BalancedB150 => serializer.serialize_unit_variant("Name", 19u32, "Balanced_B150"),
                Self::BalancedB250 => serializer.serialize_unit_variant("Name", 20u32, "Balanced_B250"),
                Self::BalancedB350 => serializer.serialize_unit_variant("Name", 21u32, "Balanced_B350"),
                Self::BalancedB500 => serializer.serialize_unit_variant("Name", 22u32, "Balanced_B500"),
                Self::BalancedB700 => serializer.serialize_unit_variant("Name", 23u32, "Balanced_B700"),
                Self::BalancedB1000 => serializer.serialize_unit_variant("Name", 24u32, "Balanced_B1000"),
                Self::MemoryOptimizedM10 => serializer.serialize_unit_variant("Name", 25u32, "MemoryOptimized_M10"),
                Self::MemoryOptimizedM20 => serializer.serialize_unit_variant("Name", 26u32, "MemoryOptimized_M20"),
                Self::MemoryOptimizedM50 => serializer.serialize_unit_variant("Name", 27u32, "MemoryOptimized_M50"),
                Self::MemoryOptimizedM100 => serializer.serialize_unit_variant("Name", 28u32, "MemoryOptimized_M100"),
                Self::MemoryOptimizedM150 => serializer.serialize_unit_variant("Name", 29u32, "MemoryOptimized_M150"),
                Self::MemoryOptimizedM250 => serializer.serialize_unit_variant("Name", 30u32, "MemoryOptimized_M250"),
                Self::MemoryOptimizedM350 => serializer.serialize_unit_variant("Name", 31u32, "MemoryOptimized_M350"),
                Self::MemoryOptimizedM500 => serializer.serialize_unit_variant("Name", 32u32, "MemoryOptimized_M500"),
                Self::MemoryOptimizedM700 => serializer.serialize_unit_variant("Name", 33u32, "MemoryOptimized_M700"),
                Self::MemoryOptimizedM1000 => serializer.serialize_unit_variant("Name", 34u32, "MemoryOptimized_M1000"),
                Self::MemoryOptimizedM1500 => serializer.serialize_unit_variant("Name", 35u32, "MemoryOptimized_M1500"),
                Self::MemoryOptimizedM2000 => serializer.serialize_unit_variant("Name", 36u32, "MemoryOptimized_M2000"),
                Self::ComputeOptimizedX3 => serializer.serialize_unit_variant("Name", 37u32, "ComputeOptimized_X3"),
                Self::ComputeOptimizedX5 => serializer.serialize_unit_variant("Name", 38u32, "ComputeOptimized_X5"),
                Self::ComputeOptimizedX10 => serializer.serialize_unit_variant("Name", 39u32, "ComputeOptimized_X10"),
                Self::ComputeOptimizedX20 => serializer.serialize_unit_variant("Name", 40u32, "ComputeOptimized_X20"),
                Self::ComputeOptimizedX50 => serializer.serialize_unit_variant("Name", 41u32, "ComputeOptimized_X50"),
                Self::ComputeOptimizedX100 => serializer.serialize_unit_variant("Name", 42u32, "ComputeOptimized_X100"),
                Self::ComputeOptimizedX150 => serializer.serialize_unit_variant("Name", 43u32, "ComputeOptimized_X150"),
                Self::ComputeOptimizedX250 => serializer.serialize_unit_variant("Name", 44u32, "ComputeOptimized_X250"),
                Self::ComputeOptimizedX350 => serializer.serialize_unit_variant("Name", 45u32, "ComputeOptimized_X350"),
                Self::ComputeOptimizedX500 => serializer.serialize_unit_variant("Name", 46u32, "ComputeOptimized_X500"),
                Self::ComputeOptimizedX700 => serializer.serialize_unit_variant("Name", 47u32, "ComputeOptimized_X700"),
                Self::FlashOptimizedA250 => serializer.serialize_unit_variant("Name", 48u32, "FlashOptimized_A250"),
                Self::FlashOptimizedA500 => serializer.serialize_unit_variant("Name", 49u32, "FlashOptimized_A500"),
                Self::FlashOptimizedA700 => serializer.serialize_unit_variant("Name", 50u32, "FlashOptimized_A700"),
                Self::FlashOptimizedA1000 => serializer.serialize_unit_variant("Name", 51u32, "FlashOptimized_A1000"),
                Self::FlashOptimizedA1500 => serializer.serialize_unit_variant("Name", 52u32, "FlashOptimized_A1500"),
                Self::FlashOptimizedA2000 => serializer.serialize_unit_variant("Name", 53u32, "FlashOptimized_A2000"),
                Self::FlashOptimizedA4500 => serializer.serialize_unit_variant("Name", 54u32, "FlashOptimized_A4500"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
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
