#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Container host mapping object specifying the Container host resource ID and its associated Controller resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerHostMapping {
    #[doc = "ARM ID of the Container Host resource"]
    #[serde(rename = "containerHostResourceId", default, skip_serializing_if = "Option::is_none")]
    pub container_host_resource_id: Option<String>,
    #[doc = "ARM ID of the mapped Controller resource"]
    #[serde(rename = "mappedControllerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub mapped_controller_resource_id: Option<String>,
}
impl ContainerHostMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Controller {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: ControllerProperties,
    #[doc = "Model representing SKU for Azure Dev Spaces Controller."]
    pub sku: Sku,
}
impl Controller {
    pub fn new(properties: ControllerProperties, sku: Sku) -> Self {
        Self {
            tracked_resource: TrackedResource::default(),
            properties,
            sku,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControllerConnectionDetails {
    #[doc = "Base class for types that supply values used to connect to container orchestrators"]
    #[serde(rename = "orchestratorSpecificConnectionDetails", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_specific_connection_details: Option<OrchestratorSpecificConnectionDetails>,
}
impl ControllerConnectionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControllerConnectionDetailsList {
    #[doc = "List of Azure Dev Spaces Controller connection details."]
    #[serde(rename = "connectionDetailsList", default, skip_serializing_if = "Vec::is_empty")]
    pub connection_details_list: Vec<ControllerConnectionDetails>,
}
impl ControllerConnectionDetailsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControllerList {
    #[doc = "List of Azure Dev Spaces Controllers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Controller>,
    #[doc = "The URI that can be used to request the next page for list of Azure Dev Spaces Controllers."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ControllerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ControllerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControllerProperties {
    #[doc = "Provisioning state of the Azure Dev Spaces Controller."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<controller_properties::ProvisioningState>,
    #[doc = "DNS suffix for public endpoints running in the Azure Dev Spaces Controller."]
    #[serde(rename = "hostSuffix", default, skip_serializing_if = "Option::is_none")]
    pub host_suffix: Option<String>,
    #[doc = "DNS name for accessing DataPlane services"]
    #[serde(rename = "dataPlaneFqdn", default, skip_serializing_if = "Option::is_none")]
    pub data_plane_fqdn: Option<String>,
    #[doc = "DNS of the target container host's API server"]
    #[serde(rename = "targetContainerHostApiServerFqdn", default, skip_serializing_if = "Option::is_none")]
    pub target_container_host_api_server_fqdn: Option<String>,
    #[doc = "Resource ID of the target container host"]
    #[serde(rename = "targetContainerHostResourceId")]
    pub target_container_host_resource_id: String,
    #[doc = "Credentials of the target container host (base64)."]
    #[serde(rename = "targetContainerHostCredentialsBase64")]
    pub target_container_host_credentials_base64: String,
}
impl ControllerProperties {
    pub fn new(target_container_host_resource_id: String, target_container_host_credentials_base64: String) -> Self {
        Self {
            provisioning_state: None,
            host_suffix: None,
            data_plane_fqdn: None,
            target_container_host_api_server_fqdn: None,
            target_container_host_resource_id,
            target_container_host_credentials_base64,
        }
    }
}
pub mod controller_properties {
    use super::*;
    #[doc = "Provisioning state of the Azure Dev Spaces Controller."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Updating,
        Creating,
        Deleting,
        Deleted,
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
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters for updating an Azure Dev Spaces Controller."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControllerUpdateParameters {
    #[doc = "Tags for the Azure Dev Spaces Controller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ControllerUpdateParametersProperties>,
}
impl ControllerUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControllerUpdateParametersProperties {
    #[doc = "Credentials of the target container host (base64)."]
    #[serde(rename = "targetContainerHostCredentialsBase64", default, skip_serializing_if = "Option::is_none")]
    pub target_container_host_credentials_base64: Option<String>,
}
impl ControllerUpdateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevSpacesErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl azure_core::Continuable for DevSpacesErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DevSpacesErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Status code for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message describing the error in detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information used to connect to a Kubernetes cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesConnectionDetails {
    #[serde(flatten)]
    pub orchestrator_specific_connection_details: OrchestratorSpecificConnectionDetails,
    #[doc = "Gets the kubeconfig for the cluster."]
    #[serde(rename = "kubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub kube_config: Option<String>,
}
impl KubernetesConnectionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for listing connection details of an Azure Dev Spaces Controller."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListConnectionDetailsParameters {
    #[doc = "Resource ID of the target container host mapped to the Azure Dev Spaces Controller."]
    #[serde(rename = "targetContainerHostResourceId")]
    pub target_container_host_resource_id: String,
}
impl ListConnectionDetailsParameters {
    pub fn new(target_container_host_resource_id: String) -> Self {
        Self {
            target_container_host_resource_id,
        }
    }
}
#[doc = "Base class for types that supply values used to connect to container orchestrators"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrchestratorSpecificConnectionDetails {
    #[doc = "Gets the Instance type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}
impl OrchestratorSpecificConnectionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationDefinition {
    #[doc = "Resource provider operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ResourceProviderOperationDisplay>,
}
impl ResourceProviderOperationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationDisplay {
    #[doc = "Name of the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the resource provider operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the resource provider operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ResourceProviderOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationList {
    #[doc = "Resource provider operations list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperationDefinition>,
    #[doc = "The URI that can be used to request the next page for list of Azure operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceProviderOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model representing SKU for Azure Dev Spaces Controller."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU for Azure Dev Spaces Controller."]
    pub name: sku::Name,
    #[doc = "The tier of the SKU for Azure Dev Spaces Controller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name, tier: None }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The name of the SKU for Azure Dev Spaces Controller."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        S1,
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
                Self::S1 => serializer.serialize_unit_variant("Name", 0u32, "S1"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The tier of the SKU for Azure Dev Spaces Controller."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Tags for the Azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Region where the Azure resource is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
