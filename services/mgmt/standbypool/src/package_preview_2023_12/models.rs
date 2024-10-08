#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Details of the ContainerGroupProfile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupProfile {
    #[doc = "Specifies container group profile id of standby container groups."]
    pub id: String,
    #[doc = "Specifies revision of container group profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl ContainerGroupProfile {
    pub fn new(id: String) -> Self {
        Self { id, revision: None }
    }
}
#[doc = "Details of the ContainerGroupProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupProperties {
    #[doc = "Details of the ContainerGroupProfile."]
    #[serde(rename = "containerGroupProfile")]
    pub container_group_profile: ContainerGroupProfile,
    #[doc = "Specifies subnet Ids for container group."]
    #[serde(
        rename = "subnetIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subnet_ids: Vec<Subnet>,
}
impl ContainerGroupProperties {
    pub fn new(container_group_profile: ContainerGroupProfile) -> Self {
        Self {
            container_group_profile,
            subnet_ids: Vec::new(),
        }
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
#[doc = "Provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
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
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
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
#[doc = "Refill policy of standby pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RefillPolicy")]
pub enum RefillPolicy {
    #[serde(rename = "always")]
    Always,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RefillPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RefillPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RefillPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Always => serializer.serialize_unit_variant("RefillPolicy", 0u32, "always"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
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
#[doc = "Specifies the elasticity profile of the standby container group pools."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyContainerGroupPoolElasticityProfile {
    #[doc = "Specifies maximum number of standby container groups in the standby pool."]
    #[serde(rename = "maxReadyCapacity")]
    pub max_ready_capacity: i64,
    #[doc = "Refill policy of standby pool"]
    #[serde(rename = "refillPolicy", default, skip_serializing_if = "Option::is_none")]
    pub refill_policy: Option<RefillPolicy>,
}
impl StandbyContainerGroupPoolElasticityProfile {
    pub fn new(max_ready_capacity: i64) -> Self {
        Self {
            max_ready_capacity,
            refill_policy: None,
        }
    }
}
#[doc = "A StandbyContainerGroupPoolResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyContainerGroupPoolResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Details of the StandbyContainerGroupPool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StandbyContainerGroupPoolResourceProperties>,
}
impl StandbyContainerGroupPoolResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a StandbyContainerGroupPoolResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyContainerGroupPoolResourceListResult {
    #[doc = "The StandbyContainerGroupPoolResource items on this page"]
    pub value: Vec<StandbyContainerGroupPoolResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StandbyContainerGroupPoolResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StandbyContainerGroupPoolResourceListResult {
    pub fn new(value: Vec<StandbyContainerGroupPoolResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Details of the StandbyContainerGroupPool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyContainerGroupPoolResourceProperties {
    #[doc = "Specifies the elasticity profile of the standby container group pools."]
    #[serde(rename = "elasticityProfile")]
    pub elasticity_profile: StandbyContainerGroupPoolElasticityProfile,
    #[doc = "Details of the ContainerGroupProperties."]
    #[serde(rename = "containerGroupProperties")]
    pub container_group_properties: ContainerGroupProperties,
    #[doc = "Provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl StandbyContainerGroupPoolResourceProperties {
    pub fn new(
        elasticity_profile: StandbyContainerGroupPoolElasticityProfile,
        container_group_properties: ContainerGroupProperties,
    ) -> Self {
        Self {
            elasticity_profile,
            container_group_properties,
            provisioning_state: None,
        }
    }
}
#[doc = "The type used for update operations of the StandbyContainerGroupPoolResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StandbyContainerGroupPoolResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the StandbyContainerGroupPoolResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StandbyContainerGroupPoolResourceUpdateProperties>,
}
impl StandbyContainerGroupPoolResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the StandbyContainerGroupPoolResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StandbyContainerGroupPoolResourceUpdateProperties {
    #[doc = "Specifies the elasticity profile of the standby container group pools."]
    #[serde(rename = "elasticityProfile", default, skip_serializing_if = "Option::is_none")]
    pub elasticity_profile: Option<StandbyContainerGroupPoolElasticityProfile>,
    #[doc = "Details of the ContainerGroupProperties."]
    #[serde(rename = "containerGroupProperties", default, skip_serializing_if = "Option::is_none")]
    pub container_group_properties: Option<ContainerGroupProperties>,
}
impl StandbyContainerGroupPoolResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the elasticity profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyVirtualMachinePoolElasticityProfile {
    #[doc = "Specifies the maximum number of virtual machines in the standby virtual machine pool."]
    #[serde(rename = "maxReadyCapacity")]
    pub max_ready_capacity: i64,
}
impl StandbyVirtualMachinePoolElasticityProfile {
    pub fn new(max_ready_capacity: i64) -> Self {
        Self { max_ready_capacity }
    }
}
#[doc = "A StandbyVirtualMachinePoolResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyVirtualMachinePoolResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Details of the StandbyVirtualMachinePool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StandbyVirtualMachinePoolResourceProperties>,
}
impl StandbyVirtualMachinePoolResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a StandbyVirtualMachinePoolResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyVirtualMachinePoolResourceListResult {
    #[doc = "The StandbyVirtualMachinePoolResource items on this page"]
    pub value: Vec<StandbyVirtualMachinePoolResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StandbyVirtualMachinePoolResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StandbyVirtualMachinePoolResourceListResult {
    pub fn new(value: Vec<StandbyVirtualMachinePoolResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Details of the StandbyVirtualMachinePool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyVirtualMachinePoolResourceProperties {
    #[doc = "Details of the elasticity profile."]
    #[serde(rename = "elasticityProfile", default, skip_serializing_if = "Option::is_none")]
    pub elasticity_profile: Option<StandbyVirtualMachinePoolElasticityProfile>,
    #[doc = "State of standby virtual machines"]
    #[serde(rename = "virtualMachineState")]
    pub virtual_machine_state: VirtualMachineState,
    #[doc = "Specifies the fully qualified resource ID of a virtual machine scale set the pool is attached to."]
    #[serde(rename = "attachedVirtualMachineScaleSetId", default, skip_serializing_if = "Option::is_none")]
    pub attached_virtual_machine_scale_set_id: Option<String>,
    #[doc = "Provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl StandbyVirtualMachinePoolResourceProperties {
    pub fn new(virtual_machine_state: VirtualMachineState) -> Self {
        Self {
            elasticity_profile: None,
            virtual_machine_state,
            attached_virtual_machine_scale_set_id: None,
            provisioning_state: None,
        }
    }
}
#[doc = "The type used for update operations of the StandbyVirtualMachinePoolResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StandbyVirtualMachinePoolResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the StandbyVirtualMachinePoolResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StandbyVirtualMachinePoolResourceUpdateProperties>,
}
impl StandbyVirtualMachinePoolResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the StandbyVirtualMachinePoolResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StandbyVirtualMachinePoolResourceUpdateProperties {
    #[doc = "Details of the elasticity profile."]
    #[serde(rename = "elasticityProfile", default, skip_serializing_if = "Option::is_none")]
    pub elasticity_profile: Option<StandbyVirtualMachinePoolElasticityProfile>,
    #[doc = "State of standby virtual machines"]
    #[serde(rename = "virtualMachineState", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_state: Option<VirtualMachineState>,
    #[doc = "Specifies the fully qualified resource ID of a virtual machine scale set the pool is attached to."]
    #[serde(rename = "attachedVirtualMachineScaleSetId", default, skip_serializing_if = "Option::is_none")]
    pub attached_virtual_machine_scale_set_id: Option<String>,
}
impl StandbyVirtualMachinePoolResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Concrete proxy resource types can be created by aliasing this type using a specific property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StandbyVirtualMachineResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Details of the StandbyVirtualMachine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StandbyVirtualMachineResourceProperties>,
}
impl StandbyVirtualMachineResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a StandbyVirtualMachineResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyVirtualMachineResourceListResult {
    #[doc = "The StandbyVirtualMachineResource items on this page"]
    pub value: Vec<StandbyVirtualMachineResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StandbyVirtualMachineResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StandbyVirtualMachineResourceListResult {
    pub fn new(value: Vec<StandbyVirtualMachineResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Details of the StandbyVirtualMachine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyVirtualMachineResourceProperties {
    #[doc = "Resource id of the virtual machine."]
    #[serde(rename = "virtualMachineResourceId")]
    pub virtual_machine_resource_id: String,
    #[doc = "Provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl StandbyVirtualMachineResourceProperties {
    pub fn new(virtual_machine_resource_id: String) -> Self {
        Self {
            virtual_machine_resource_id,
            provisioning_state: None,
        }
    }
}
#[doc = "Subnet of container group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subnet {
    #[doc = "Specifies ARM resource id of the subnet."]
    pub id: String,
}
impl Subnet {
    pub fn new(id: String) -> Self {
        Self { id }
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
#[doc = "State of standby virtual machines"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VirtualMachineState")]
pub enum VirtualMachineState {
    Running,
    Deallocated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VirtualMachineState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VirtualMachineState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VirtualMachineState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Running => serializer.serialize_unit_variant("VirtualMachineState", 0u32, "Running"),
            Self::Deallocated => serializer.serialize_unit_variant("VirtualMachineState", 1u32, "Deallocated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
