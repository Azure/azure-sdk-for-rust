#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents an instance of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControllerResource {
    #[doc = "An identifier that represents the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ControllerResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for updating a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControllerResourceUpdateParameters {
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ControllerResourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an instance of a DNC controller."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelegatedController {
    #[serde(flatten)]
    pub controller_resource: ControllerResource,
    #[doc = "Properties of Delegated controller resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DelegatedControllerProperties>,
}
impl DelegatedController {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Delegated controller resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelegatedControllerProperties {
    #[doc = "Resource guid."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The current state of dnc controller resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<delegated_controller_properties::ProvisioningState>,
    #[doc = "dnc application id should be used by customer to authenticate with dnc gateway."]
    #[serde(rename = "dncAppId", default, skip_serializing_if = "Option::is_none")]
    pub dnc_app_id: Option<String>,
    #[doc = "tenant id of dnc application id"]
    #[serde(rename = "dncTenantId", default, skip_serializing_if = "Option::is_none")]
    pub dnc_tenant_id: Option<String>,
    #[doc = "dnc endpoint url that customers can use to connect to"]
    #[serde(rename = "dncEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub dnc_endpoint: Option<String>,
}
impl DelegatedControllerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod delegated_controller_properties {
    use super::*;
    #[doc = "The current state of dnc controller resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Failed,
        Provisioning,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An array of Delegated controller resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DelegatedControllers {
    #[doc = "An array of Delegated controller resources."]
    pub value: Vec<DelegatedController>,
    #[doc = "The URL to get the next set of controllers."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DelegatedControllers {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DelegatedControllers {
    pub fn new(value: Vec<DelegatedController>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Represents an instance of a orchestrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelegatedSubnet {
    #[serde(flatten)]
    pub delegated_subnet_resource: DelegatedSubnetResource,
    #[doc = "Properties of delegated subnet"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DelegatedSubnetProperties>,
}
impl DelegatedSubnet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of delegated subnet"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelegatedSubnetProperties {
    #[doc = "Resource guid."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The current state of dnc delegated subnet resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<delegated_subnet_properties::ProvisioningState>,
    #[doc = "Properties of orchestrator"]
    #[serde(rename = "subnetDetails", default, skip_serializing_if = "Option::is_none")]
    pub subnet_details: Option<SubnetDetails>,
    #[doc = "controller details"]
    #[serde(rename = "controllerDetails", default, skip_serializing_if = "Option::is_none")]
    pub controller_details: Option<ControllerDetails>,
}
impl DelegatedSubnetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod delegated_subnet_properties {
    use super::*;
    #[doc = "The current state of dnc delegated subnet resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Failed,
        Provisioning,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents an instance of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelegatedSubnetResource {
    #[doc = "An identifier that represents the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DelegatedSubnetResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An array of DelegatedSubnet resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DelegatedSubnets {
    #[doc = "An array of DelegatedSubnet resources."]
    pub value: Vec<DelegatedSubnet>,
    #[doc = "The URL to get the next set of DelegatedSubnet resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DelegatedSubnets {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DelegatedSubnets {
    pub fn new(value: Vec<DelegatedSubnet>) -> Self {
        Self { value, next_link: None }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
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
#[doc = "Represents an instance of a orchestrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Orchestrator {
    #[serde(flatten)]
    pub orchestrator_resource: OrchestratorResource,
    #[doc = "Properties of orchestrator"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OrchestratorResourceProperties>,
}
impl Orchestrator {
    pub fn new(orchestrator_resource: OrchestratorResource) -> Self {
        Self {
            orchestrator_resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrchestratorIdentity {
    #[doc = "The principal id of the system assigned identity which is used by orchestrator."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of the system assigned identity which is used by orchestrator."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for orchestrator cluster. Type 'SystemAssigned' will use an implicitly created identity orchestrator clusters"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<orchestrator_identity::Type>,
}
impl OrchestratorIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod orchestrator_identity {
    use super::*;
    #[doc = "The type of identity used for orchestrator cluster. Type 'SystemAssigned' will use an implicitly created identity orchestrator clusters"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[doc = "Represents an instance of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrchestratorResource {
    #[doc = "An identifier that represents the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The kind of workbook. Choices are user and shared."]
    pub kind: orchestrator_resource::Kind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<OrchestratorIdentity>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl OrchestratorResource {
    pub fn new(kind: orchestrator_resource::Kind) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location: None,
            kind,
            identity: None,
            tags: None,
        }
    }
}
pub mod orchestrator_resource {
    use super::*;
    #[doc = "The kind of workbook. Choices are user and shared."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Kubernetes,
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
                Self::Kubernetes => serializer.serialize_unit_variant("Kind", 0u32, "Kubernetes"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of orchestrator"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrchestratorResourceProperties {
    #[doc = "Resource guid."]
    #[serde(rename = "resourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[doc = "The current state of orchestratorInstance resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<orchestrator_resource_properties::ProvisioningState>,
    #[doc = "AAD ID used with apiserver"]
    #[serde(rename = "orchestratorAppId", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_app_id: Option<String>,
    #[doc = "TenantID of server App ID"]
    #[serde(rename = "orchestratorTenantId", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_tenant_id: Option<String>,
    #[doc = "RootCA certificate of kubernetes cluster base64 encoded"]
    #[serde(rename = "clusterRootCA", default, skip_serializing_if = "Option::is_none")]
    pub cluster_root_ca: Option<String>,
    #[doc = "K8s APIServer url. Either one of apiServerEndpoint or privateLinkResourceId can be specified"]
    #[serde(rename = "apiServerEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub api_server_endpoint: Option<String>,
    #[doc = "private link arm resource id. Either one of apiServerEndpoint or privateLinkResourceId can be specified"]
    #[serde(rename = "privateLinkResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_resource_id: Option<String>,
    #[doc = "controller details"]
    #[serde(rename = "controllerDetails")]
    pub controller_details: ControllerDetails,
}
impl OrchestratorResourceProperties {
    pub fn new(controller_details: ControllerDetails) -> Self {
        Self {
            resource_guid: None,
            provisioning_state: None,
            orchestrator_app_id: None,
            orchestrator_tenant_id: None,
            cluster_root_ca: None,
            api_server_endpoint: None,
            private_link_resource_id: None,
            controller_details,
        }
    }
}
pub mod orchestrator_resource_properties {
    use super::*;
    #[doc = "The current state of orchestratorInstance resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Failed,
        Provisioning,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters for updating a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrchestratorResourceUpdateParameters {
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl OrchestratorResourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An array of OrchestratorInstance resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Orchestrators {
    #[doc = "An array of OrchestratorInstance resources."]
    pub value: Vec<Orchestrator>,
    #[doc = "The URL to get the next set of orchestrators."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for Orchestrators {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl Orchestrators {
    pub fn new(value: Vec<Orchestrator>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Parameters for updating a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUpdateParameters {
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "controller details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ControllerDetails {
    #[doc = "controller arm resource id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ControllerDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of orchestrator"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetDetails {
    #[doc = "subnet arm resource id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubnetDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
