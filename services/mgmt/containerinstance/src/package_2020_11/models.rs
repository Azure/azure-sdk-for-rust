#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The properties of the Azure File volume. Azure File shares are mounted as volumes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileVolume {
    #[doc = "The name of the Azure File share to be mounted as a volume."]
    #[serde(rename = "shareName")]
    pub share_name: String,
    #[doc = "The flag indicating whether the Azure File shared mounted as a volume is read-only."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "The name of the storage account that contains the Azure File share."]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "The storage account access key used to access the Azure File share."]
    #[serde(rename = "storageAccountKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_key: Option<String>,
}
impl AzureFileVolume {
    pub fn new(share_name: String, storage_account_name: String) -> Self {
        Self {
            share_name,
            read_only: None,
            storage_account_name,
            storage_account_key: None,
        }
    }
}
#[doc = "The response containing cached images."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CachedImagesListResult {
    #[doc = "The list of cached images."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CachedImages>,
    #[doc = "The URI to fetch the next page of cached images."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CachedImagesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CachedImagesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The regional capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Capabilities {
    #[doc = "The resource type that this capability describes."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The OS type that this capability describes."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The ip address type that this capability describes."]
    #[serde(rename = "ipAddressType", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[doc = "The GPU sku that this capability describes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpu: Option<String>,
    #[doc = "The supported capabilities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<capabilities::Capabilities>,
}
impl Capabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod capabilities {
    use super::*;
    #[doc = "The supported capabilities."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Capabilities {
        #[doc = "The maximum allowed memory request in GB."]
        #[serde(rename = "maxMemoryInGB", default, skip_serializing_if = "Option::is_none")]
        pub max_memory_in_gb: Option<f64>,
        #[doc = "The maximum allowed CPU request in cores."]
        #[serde(rename = "maxCpu", default, skip_serializing_if = "Option::is_none")]
        pub max_cpu: Option<f64>,
        #[doc = "The maximum allowed GPU count."]
        #[serde(rename = "maxGpuCount", default, skip_serializing_if = "Option::is_none")]
        pub max_gpu_count: Option<f64>,
    }
    impl Capabilities {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response containing list of capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilitiesListResult {
    #[doc = "The list of capabilities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Capabilities>,
    #[doc = "The URI to fetch the next page of capabilities."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CapabilitiesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CapabilitiesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Container Instance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Container Instance service."]
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
#[doc = "An error response from the Container Instance service."]
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
#[doc = "A container instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Container {
    #[doc = "The user-provided name of the container instance."]
    pub name: String,
    #[doc = "The container instance properties."]
    pub properties: ContainerProperties,
}
impl Container {
    pub fn new(name: String, properties: ContainerProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The information for the output stream from container attach."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerAttachResponse {
    #[doc = "The uri for the output stream from the attach."]
    #[serde(rename = "webSocketUri", default, skip_serializing_if = "Option::is_none")]
    pub web_socket_uri: Option<String>,
    #[doc = "The password to the output stream from the attach. Send as an Authorization header value when connecting to the websocketUri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ContainerAttachResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container execution command, for liveness or readiness probe"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerExec {
    #[doc = "The commands to execute within the container."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}
impl ContainerExec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container exec request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerExecRequest {
    #[doc = "The command to be executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[doc = "The size of the terminal."]
    #[serde(rename = "terminalSize", default, skip_serializing_if = "Option::is_none")]
    pub terminal_size: Option<container_exec_request::TerminalSize>,
}
impl ContainerExecRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_exec_request {
    use super::*;
    #[doc = "The size of the terminal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TerminalSize {
        #[doc = "The row size of the terminal"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rows: Option<i64>,
        #[doc = "The column size of the terminal"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cols: Option<i64>,
    }
    impl TerminalSize {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The information for the container exec command."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerExecResponse {
    #[doc = "The uri for the exec websocket."]
    #[serde(rename = "webSocketUri", default, skip_serializing_if = "Option::is_none")]
    pub web_socket_uri: Option<String>,
    #[doc = "The password to start the exec command."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ContainerExecResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Identity for the container group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ContainerGroupIdentity>,
    #[doc = "The container group properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<container_group::Properties>,
}
impl ContainerGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_group {
    use super::*;
    #[doc = "The container group properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "The provisioning state of the container group. This only appears in the response."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<String>,
        #[doc = "The containers within the container group."]
        pub containers: Vec<Container>,
        #[doc = "The image registry credentials by which the container group is created from."]
        #[serde(rename = "imageRegistryCredentials", default, skip_serializing_if = "Vec::is_empty")]
        pub image_registry_credentials: Vec<ImageRegistryCredential>,
        #[doc = "Restart policy for all containers within the container group. \n- `Always` Always restart\n- `OnFailure` Restart on failure\n- `Never` Never restart\n"]
        #[serde(rename = "restartPolicy", default, skip_serializing_if = "Option::is_none")]
        pub restart_policy: Option<properties::RestartPolicy>,
        #[doc = "IP address for the container group."]
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<IpAddress>,
        #[doc = "The operating system type required by the containers in the container group."]
        #[serde(rename = "osType")]
        pub os_type: properties::OsType,
        #[doc = "The list of volumes that can be mounted by containers in this container group."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub volumes: Vec<Volume>,
        #[doc = "The instance view of the container group. Only valid in response."]
        #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
        pub instance_view: Option<properties::InstanceView>,
        #[doc = "Container group diagnostic information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub diagnostics: Option<ContainerGroupDiagnostics>,
        #[doc = "Container group network profile information."]
        #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
        pub network_profile: Option<ContainerGroupNetworkProfile>,
        #[doc = "DNS configuration for the container group."]
        #[serde(rename = "dnsConfig", default, skip_serializing_if = "Option::is_none")]
        pub dns_config: Option<DnsConfiguration>,
        #[doc = "The container group SKU."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sku: Option<ContainerGroupSku>,
        #[doc = "The container group encryption properties."]
        #[serde(rename = "encryptionProperties", default, skip_serializing_if = "Option::is_none")]
        pub encryption_properties: Option<EncryptionProperties>,
        #[doc = "The init containers for a container group."]
        #[serde(rename = "initContainers", default, skip_serializing_if = "Vec::is_empty")]
        pub init_containers: Vec<InitContainerDefinition>,
    }
    impl Properties {
        pub fn new(containers: Vec<Container>, os_type: properties::OsType) -> Self {
            Self {
                provisioning_state: None,
                containers,
                image_registry_credentials: Vec::new(),
                restart_policy: None,
                ip_address: None,
                os_type,
                volumes: Vec::new(),
                instance_view: None,
                diagnostics: None,
                network_profile: None,
                dns_config: None,
                sku: None,
                encryption_properties: None,
                init_containers: Vec::new(),
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Restart policy for all containers within the container group. \n- `Always` Always restart\n- `OnFailure` Restart on failure\n- `Never` Never restart\n"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "RestartPolicy")]
        pub enum RestartPolicy {
            Always,
            OnFailure,
            Never,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for RestartPolicy {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for RestartPolicy {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for RestartPolicy {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Always => serializer.serialize_unit_variant("RestartPolicy", 0u32, "Always"),
                    Self::OnFailure => serializer.serialize_unit_variant("RestartPolicy", 1u32, "OnFailure"),
                    Self::Never => serializer.serialize_unit_variant("RestartPolicy", 2u32, "Never"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "The operating system type required by the containers in the container group."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "OsType")]
        pub enum OsType {
            Windows,
            Linux,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for OsType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for OsType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for OsType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
                    Self::Linux => serializer.serialize_unit_variant("OsType", 1u32, "Linux"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "The instance view of the container group. Only valid in response."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct InstanceView {
            #[doc = "The events of this container group."]
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub events: Vec<Event>,
            #[doc = "The state of the container group. Only valid in response."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub state: Option<String>,
        }
        impl InstanceView {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "Container group diagnostic information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerGroupDiagnostics {
    #[doc = "Container group log analytics information."]
    #[serde(rename = "logAnalytics", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics: Option<LogAnalytics>,
}
impl ContainerGroupDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the container group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerGroupIdentity {
    #[doc = "The principal id of the container group identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id associated with the container group. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the container group. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the container group."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<container_group_identity::Type>,
    #[doc = "The list of user identities associated with the container group. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ContainerGroupIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_group_identity {
    use super::*;
    #[doc = "The type of identity used for the container group. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the container group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "The container group list response that contains the container group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerGroupListResult {
    #[doc = "The list of container groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ContainerGroup>,
    #[doc = "The URI to fetch the next page of container groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ContainerGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ContainerGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container group network profile information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupNetworkProfile {
    #[doc = "The identifier for a network profile."]
    pub id: String,
}
impl ContainerGroupNetworkProfile {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The container group SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ContainerGroupSku")]
pub enum ContainerGroupSku {
    Standard,
    Dedicated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ContainerGroupSku {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ContainerGroupSku {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ContainerGroupSku {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Standard => serializer.serialize_unit_variant("ContainerGroupSku", 0u32, "Standard"),
            Self::Dedicated => serializer.serialize_unit_variant("ContainerGroupSku", 1u32, "Dedicated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The container Http Get settings, for liveness or readiness probe"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerHttpGet {
    #[doc = "The path to probe."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The port number to probe."]
    pub port: i32,
    #[doc = "The scheme."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<container_http_get::Scheme>,
    #[doc = "The HTTP headers."]
    #[serde(rename = "httpHeaders", default, skip_serializing_if = "Vec::is_empty")]
    pub http_headers: Vec<HttpHeader>,
}
impl ContainerHttpGet {
    pub fn new(port: i32) -> Self {
        Self {
            path: None,
            port,
            scheme: None,
            http_headers: Vec::new(),
        }
    }
}
pub mod container_http_get {
    use super::*;
    #[doc = "The scheme."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Scheme")]
    pub enum Scheme {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "https")]
        Https,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Scheme {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Scheme {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Scheme {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("Scheme", 0u32, "http"),
                Self::Https => serializer.serialize_unit_variant("Scheme", 1u32, "https"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The port exposed on the container instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerPort {
    #[doc = "The protocol associated with the port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<container_port::Protocol>,
    #[doc = "The port number exposed within the container group."]
    pub port: i32,
}
impl ContainerPort {
    pub fn new(port: i32) -> Self {
        Self { protocol: None, port }
    }
}
pub mod container_port {
    use super::*;
    #[doc = "The protocol associated with the port."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
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
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 0u32, "TCP"),
                Self::Udp => serializer.serialize_unit_variant("Protocol", 1u32, "UDP"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The container probe, for liveness or readiness"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerProbe {
    #[doc = "The container execution command, for liveness or readiness probe"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ContainerExec>,
    #[doc = "The container Http Get settings, for liveness or readiness probe"]
    #[serde(rename = "httpGet", default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<ContainerHttpGet>,
    #[doc = "The initial delay seconds."]
    #[serde(rename = "initialDelaySeconds", default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    #[doc = "The period seconds."]
    #[serde(rename = "periodSeconds", default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[doc = "The failure threshold."]
    #[serde(rename = "failureThreshold", default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[doc = "The success threshold."]
    #[serde(rename = "successThreshold", default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[doc = "The timeout seconds."]
    #[serde(rename = "timeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}
impl ContainerProbe {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container instance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerProperties {
    #[doc = "The name of the image used to create the container instance."]
    pub image: String,
    #[doc = "The commands to execute within the container instance in exec form."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[doc = "The exposed ports on the container instance."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ContainerPort>,
    #[doc = "The environment variables to set in the container instance."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentVariable>,
    #[doc = "The instance view of the container instance. Only valid in response."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<container_properties::InstanceView>,
    #[doc = "The resource requirements."]
    pub resources: ResourceRequirements,
    #[doc = "The volume mounts available to the container instance."]
    #[serde(rename = "volumeMounts", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,
    #[doc = "The container probe, for liveness or readiness"]
    #[serde(rename = "livenessProbe", default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<ContainerProbe>,
    #[doc = "The container probe, for liveness or readiness"]
    #[serde(rename = "readinessProbe", default, skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<ContainerProbe>,
}
impl ContainerProperties {
    pub fn new(image: String, resources: ResourceRequirements) -> Self {
        Self {
            image,
            command: Vec::new(),
            ports: Vec::new(),
            environment_variables: Vec::new(),
            instance_view: None,
            resources,
            volume_mounts: Vec::new(),
            liveness_probe: None,
            readiness_probe: None,
        }
    }
}
pub mod container_properties {
    use super::*;
    #[doc = "The instance view of the container instance. Only valid in response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InstanceView {
        #[doc = "The number of times that the container instance has been restarted."]
        #[serde(rename = "restartCount", default, skip_serializing_if = "Option::is_none")]
        pub restart_count: Option<i64>,
        #[doc = "The container instance state."]
        #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
        pub current_state: Option<ContainerState>,
        #[doc = "The container instance state."]
        #[serde(rename = "previousState", default, skip_serializing_if = "Option::is_none")]
        pub previous_state: Option<ContainerState>,
        #[doc = "The events of the container instance."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub events: Vec<Event>,
    }
    impl InstanceView {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The container instance state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerState {
    #[doc = "The state of the container instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The date-time when the container instance state started."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The container instance exit codes correspond to those from the `docker run` command."]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    #[doc = "The date-time when the container instance state finished."]
    #[serde(rename = "finishTime", with = "azure_core::date::rfc3339::option")]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "The human-readable status of the container instance state."]
    #[serde(rename = "detailStatus", default, skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
}
impl ContainerState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DNS configuration for the container group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsConfiguration {
    #[doc = "The DNS servers for the container group."]
    #[serde(rename = "nameServers")]
    pub name_servers: Vec<String>,
    #[doc = "The DNS search domains for hostname lookup in the container group."]
    #[serde(rename = "searchDomains", default, skip_serializing_if = "Option::is_none")]
    pub search_domains: Option<String>,
    #[doc = "The DNS options for the container group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}
impl DnsConfiguration {
    pub fn new(name_servers: Vec<String>) -> Self {
        Self {
            name_servers,
            search_domains: None,
            options: None,
        }
    }
}
#[doc = "The empty directory volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyDirVolume {}
impl EmptyDirVolume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container group encryption properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionProperties {
    #[doc = "The keyvault base url."]
    #[serde(rename = "vaultBaseUrl")]
    pub vault_base_url: String,
    #[doc = "The encryption key name."]
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[doc = "The encryption key version."]
    #[serde(rename = "keyVersion")]
    pub key_version: String,
}
impl EncryptionProperties {
    pub fn new(vault_base_url: String, key_name: String, key_version: String) -> Self {
        Self {
            vault_base_url,
            key_name,
            key_version,
        }
    }
}
#[doc = "The environment variable to set within the container instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    #[doc = "The name of the environment variable."]
    pub name: String,
    #[doc = "The value of the environment variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The value of the secure environment variable."]
    #[serde(rename = "secureValue", default, skip_serializing_if = "Option::is_none")]
    pub secure_value: Option<String>,
}
impl EnvironmentVariable {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            secure_value: None,
        }
    }
}
#[doc = "A container group or container instance event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    #[doc = "The count of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "The date-time of the earliest logged event."]
    #[serde(rename = "firstTimestamp", with = "azure_core::date::rfc3339::option")]
    pub first_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The date-time of the latest logged event."]
    #[serde(rename = "lastTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The event name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The event message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The event type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Event {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a volume that is populated with the contents of a git repository"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitRepoVolume {
    #[doc = "Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[doc = "Repository URL"]
    pub repository: String,
    #[doc = "Commit hash for the specified revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}
impl GitRepoVolume {
    pub fn new(repository: String) -> Self {
        Self {
            directory: None,
            repository,
            revision: None,
        }
    }
}
#[doc = "The GPU resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpuResource {
    #[doc = "The count of the GPU resource."]
    pub count: i32,
    #[doc = "The SKU of the GPU resource."]
    pub sku: gpu_resource::Sku,
}
impl GpuResource {
    pub fn new(count: i32, sku: gpu_resource::Sku) -> Self {
        Self { count, sku }
    }
}
pub mod gpu_resource {
    use super::*;
    #[doc = "The SKU of the GPU resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Sku")]
    pub enum Sku {
        K80,
        P100,
        V100,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Sku {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Sku {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Sku {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::K80 => serializer.serialize_unit_variant("Sku", 0u32, "K80"),
                Self::P100 => serializer.serialize_unit_variant("Sku", 1u32, "P100"),
                Self::V100 => serializer.serialize_unit_variant("Sku", 2u32, "V100"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The HTTP header."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpHeader {
    #[doc = "The header name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The header value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl HttpHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image registry credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRegistryCredential {
    #[doc = "The Docker image registry server without a protocol such as \"http\" and \"https\"."]
    pub server: String,
    #[doc = "The username for the private registry."]
    pub username: String,
    #[doc = "The password for the private registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ImageRegistryCredential {
    pub fn new(server: String, username: String) -> Self {
        Self {
            server,
            username,
            password: None,
        }
    }
}
#[doc = "The init container definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InitContainerDefinition {
    #[doc = "The name for the init container."]
    pub name: String,
    #[doc = "The init container definition properties."]
    pub properties: InitContainerPropertiesDefinition,
}
impl InitContainerDefinition {
    pub fn new(name: String, properties: InitContainerPropertiesDefinition) -> Self {
        Self { name, properties }
    }
}
#[doc = "The init container definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InitContainerPropertiesDefinition {
    #[doc = "The image of the init container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[doc = "The command to execute within the init container in exec form."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[doc = "The environment variables to set in the init container."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentVariable>,
    #[doc = "The instance view of the init container. Only valid in response."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<init_container_properties_definition::InstanceView>,
    #[doc = "The volume mounts available to the init container."]
    #[serde(rename = "volumeMounts", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,
}
impl InitContainerPropertiesDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod init_container_properties_definition {
    use super::*;
    #[doc = "The instance view of the init container. Only valid in response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InstanceView {
        #[doc = "The number of times that the init container has been restarted."]
        #[serde(rename = "restartCount", default, skip_serializing_if = "Option::is_none")]
        pub restart_count: Option<i64>,
        #[doc = "The container instance state."]
        #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
        pub current_state: Option<ContainerState>,
        #[doc = "The container instance state."]
        #[serde(rename = "previousState", default, skip_serializing_if = "Option::is_none")]
        pub previous_state: Option<ContainerState>,
        #[doc = "The events of the init container."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub events: Vec<Event>,
    }
    impl InstanceView {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "IP address for the container group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddress {
    #[doc = "The list of ports exposed on the container group."]
    pub ports: Vec<Port>,
    #[doc = "Specifies if the IP is exposed to the public internet or private VNET."]
    #[serde(rename = "type")]
    pub type_: ip_address::Type,
    #[doc = "The IP exposed to the public internet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The Dns name label for the IP."]
    #[serde(rename = "dnsNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub dns_name_label: Option<String>,
    #[doc = "The FQDN for the IP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}
impl IpAddress {
    pub fn new(ports: Vec<Port>, type_: ip_address::Type) -> Self {
        Self {
            ports,
            type_,
            ip: None,
            dns_name_label: None,
            fqdn: None,
        }
    }
}
pub mod ip_address {
    use super::*;
    #[doc = "Specifies if the IP is exposed to the public internet or private VNET."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Public,
        Private,
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
                Self::Public => serializer.serialize_unit_variant("Type", 0u32, "Public"),
                Self::Private => serializer.serialize_unit_variant("Type", 1u32, "Private"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Container group log analytics information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogAnalytics {
    #[doc = "The workspace id for log analytics"]
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[doc = "The workspace key for log analytics"]
    #[serde(rename = "workspaceKey")]
    pub workspace_key: String,
    #[doc = "The log type to be used."]
    #[serde(rename = "logType", default, skip_serializing_if = "Option::is_none")]
    pub log_type: Option<log_analytics::LogType>,
    #[doc = "Metadata for log analytics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The workspace resource id for log analytics"]
    #[serde(rename = "workspaceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_resource_id: Option<serde_json::Value>,
}
impl LogAnalytics {
    pub fn new(workspace_id: String, workspace_key: String) -> Self {
        Self {
            workspace_id,
            workspace_key,
            log_type: None,
            metadata: None,
            workspace_resource_id: None,
        }
    }
}
pub mod log_analytics {
    use super::*;
    #[doc = "The log type to be used."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LogType")]
    pub enum LogType {
        ContainerInsights,
        ContainerInstanceLogs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LogType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LogType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LogType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ContainerInsights => serializer.serialize_unit_variant("LogType", 0u32, "ContainerInsights"),
                Self::ContainerInstanceLogs => serializer.serialize_unit_variant("LogType", 1u32, "ContainerInstanceLogs"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Logs {
    #[doc = "The content of the log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl Logs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An operation for Azure Container Instance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[doc = "The name of the operation."]
    pub name: String,
    #[doc = "The display information of the operation."]
    pub display: operation::Display,
    #[doc = "The additional properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
}
impl Operation {
    pub fn new(name: String, display: operation::Display) -> Self {
        Self {
            name,
            display,
            properties: None,
            origin: None,
        }
    }
}
pub mod operation {
    use super::*;
    #[doc = "The display information of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The name of the provider of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The name of the resource type of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The friendly name of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        User,
        System,
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
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "User"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "System"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The operation list response that contains all operations for Azure Container Instance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URI to fetch the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The port exposed on the container group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Port {
    #[doc = "The protocol associated with the port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<port::Protocol>,
    #[doc = "The port number."]
    pub port: i32,
}
impl Port {
    pub fn new(port: i32) -> Self {
        Self { protocol: None, port }
    }
}
pub mod port {
    use super::*;
    #[doc = "The protocol associated with the port."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
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
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 0u32, "TCP"),
                Self::Udp => serializer.serialize_unit_variant("Protocol", 1u32, "UDP"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceLimits {
    #[doc = "The memory limit in GB of this container instance."]
    #[serde(rename = "memoryInGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_gb: Option<f64>,
    #[doc = "The CPU limit of this container instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    #[doc = "The GPU resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpu: Option<GpuResource>,
}
impl ResourceLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequests {
    #[doc = "The memory request in GB of this container instance."]
    #[serde(rename = "memoryInGB")]
    pub memory_in_gb: f64,
    #[doc = "The CPU request of this container instance."]
    pub cpu: f64,
    #[doc = "The GPU resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpu: Option<GpuResource>,
}
impl ResourceRequests {
    pub fn new(memory_in_gb: f64, cpu: f64) -> Self {
        Self {
            memory_in_gb,
            cpu,
            gpu: None,
        }
    }
}
#[doc = "The resource requirements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirements {
    #[doc = "The resource requests."]
    pub requests: ResourceRequests,
    #[doc = "The resource limits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceLimits>,
}
impl ResourceRequirements {
    pub fn new(requests: ResourceRequests) -> Self {
        Self { requests, limits: None }
    }
}
#[doc = "The secret volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretVolume {}
impl SecretVolume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A single usage result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "Unit of the usage result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The current usage of the resource"]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The maximum permitted usage of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The name object of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<usage::Name>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[doc = "The name object of the resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Name {
        #[doc = "The name of the resource"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[doc = "The localized name of the resource"]
        #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
        pub localized_value: Option<String>,
    }
    impl Name {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response containing the usage data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[doc = "The usage data."]
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
#[doc = "The properties of the volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    #[doc = "The name of the volume."]
    pub name: String,
    #[doc = "The properties of the Azure File volume. Azure File shares are mounted as volumes."]
    #[serde(rename = "azureFile", default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<AzureFileVolume>,
    #[doc = "The empty directory volume."]
    #[serde(rename = "emptyDir", default, skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EmptyDirVolume>,
    #[doc = "The secret volume."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretVolume>,
    #[doc = "Represents a volume that is populated with the contents of a git repository"]
    #[serde(rename = "gitRepo", default, skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<GitRepoVolume>,
}
impl Volume {
    pub fn new(name: String) -> Self {
        Self {
            name,
            azure_file: None,
            empty_dir: None,
            secret: None,
            git_repo: None,
        }
    }
}
#[doc = "The properties of the volume mount."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeMount {
    #[doc = "The name of the volume mount."]
    pub name: String,
    #[doc = "The path within the container where the volume should be mounted. Must not contain colon (:)."]
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    #[doc = "The flag indicating whether the volume mount is read-only."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
impl VolumeMount {
    pub fn new(name: String, mount_path: String) -> Self {
        Self {
            name,
            mount_path,
            read_only: None,
        }
    }
}
#[doc = "The cached image and OS type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CachedImages {
    #[doc = "The OS type of the cached image."]
    #[serde(rename = "osType")]
    pub os_type: String,
    #[doc = "The cached image name."]
    pub image: String,
}
impl CachedImages {
    pub fn new(os_type: String, image: String) -> Self {
        Self { os_type, image }
    }
}
