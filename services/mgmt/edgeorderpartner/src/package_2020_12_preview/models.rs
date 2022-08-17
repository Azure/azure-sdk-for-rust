#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalErrorInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl AdditionalErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains additional data about inventory in dictionary format"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalInventoryDetails {
    #[doc = "Additional Data"]
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
}
impl AdditionalInventoryDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains additional order item details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalOrderItemDetails {
    #[doc = "Resource stage details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StageDetails>,
    #[doc = "Contains subscription details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionDetails>,
}
impl AdditionalOrderItemDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains billing details for the inventory"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingDetails {
    #[doc = "Billing type for the inventory"]
    #[serde(rename = "billingType", default, skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<String>,
    #[doc = "Billing status for the inventory"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl BillingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudError>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<AdditionalErrorInfo>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information about inventory configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationData {
    #[doc = "Family identifier of inventory"]
    #[serde(rename = "familyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub family_identifier: Option<String>,
    #[doc = "Product Line identifier of inventory"]
    #[serde(rename = "productLineIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub product_line_identifier: Option<String>,
    #[doc = "Product identifier of inventory"]
    #[serde(rename = "productIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub product_identifier: Option<String>,
    #[doc = "Configuration identifier of inventory"]
    #[serde(rename = "configurationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub configuration_identifier: Option<String>,
    #[doc = "Configuration identifier on device - this is used in case of any mismatch\r\nbetween actual configuration on inventory and configuration stored in service"]
    #[serde(rename = "configurationIdentifierOnDevice", default, skip_serializing_if = "Option::is_none")]
    pub configuration_identifier_on_device: Option<String>,
}
impl ConfigurationData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains additional configuration details about inventory"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationDetails {
    #[doc = "Collection of specification details about the inventory"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specifications: Vec<SpecificationDetails>,
}
impl ConfigurationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration parameters for ManageInventoryMetadata call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationOnDevice {
    #[doc = "Configuration identifier on device"]
    #[serde(rename = "configurationIdentifier")]
    pub configuration_identifier: String,
}
impl ConfigurationOnDevice {
    pub fn new(configuration_identifier: String) -> Self {
        Self { configuration_identifier }
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
#[doc = "Represents additional details about the partner inventory"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InventoryAdditionalDetails {
    #[doc = "Contains additional order item details"]
    #[serde(rename = "orderItem", default, skip_serializing_if = "Option::is_none")]
    pub order_item: Option<AdditionalOrderItemDetails>,
    #[doc = "Contains inventory metadata"]
    #[serde(rename = "inventoryMetadata", default, skip_serializing_if = "Option::is_none")]
    pub inventory_metadata: Option<String>,
    #[doc = "Contains additional configuration details about inventory"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationDetails>,
    #[doc = "Contains additional data about inventory in dictionary format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory: Option<AdditionalInventoryDetails>,
    #[doc = "Contains billing details for the inventory"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingDetails>,
    #[doc = "Represents secrets on the inventory"]
    #[serde(rename = "inventorySecrets", default, skip_serializing_if = "Option::is_none")]
    pub inventory_secrets: Option<serde_json::Value>,
}
impl InventoryAdditionalDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains basic information about inventory"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InventoryData {
    #[doc = "Inventory status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Inventory location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Boolean flag to indicate if registration is allowed"]
    #[serde(rename = "registrationAllowed", default, skip_serializing_if = "Option::is_none")]
    pub registration_allowed: Option<bool>,
}
impl InventoryData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents inventory properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InventoryProperties {
    #[doc = "Serial number of the device."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Contains information about the order item to which inventory belongs"]
    #[serde(rename = "orderItem", default, skip_serializing_if = "Option::is_none")]
    pub order_item: Option<OrderItemData>,
    #[doc = "Contains information about inventory configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationData>,
    #[doc = "Contains information about management resource"]
    #[serde(rename = "managementResource", default, skip_serializing_if = "Option::is_none")]
    pub management_resource: Option<ManagementResourceData>,
    #[doc = "Contains basic information about inventory"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory: Option<InventoryData>,
    #[doc = "Location of inventory"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Represents additional details about the partner inventory"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<InventoryAdditionalDetails>,
}
impl InventoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for ManageInventoryMetadata call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManageInventoryMetadataRequest {
    #[doc = "Inventory metadata to be updated"]
    #[serde(rename = "inventoryMetadata")]
    pub inventory_metadata: String,
    #[doc = "Configuration parameters for ManageInventoryMetadata call"]
    #[serde(rename = "configurationOnDevice", default, skip_serializing_if = "Option::is_none")]
    pub configuration_on_device: Option<ConfigurationOnDevice>,
}
impl ManageInventoryMetadataRequest {
    pub fn new(inventory_metadata: String) -> Self {
        Self {
            inventory_metadata,
            configuration_on_device: None,
        }
    }
}
#[doc = "Request body for ManageLink call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManageLinkRequest {
    #[doc = "Arm Id of the management resource to which inventory is to be linked\r\nFor unlink operation, enter empty string"]
    #[serde(rename = "managementResourceArmId")]
    pub management_resource_arm_id: String,
    #[doc = "Operation to be performed - Link, Unlink, Relink"]
    pub operation: manage_link_request::Operation,
    #[doc = "Tenant ID of management resource associated with inventory"]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
impl ManageLinkRequest {
    pub fn new(management_resource_arm_id: String, operation: manage_link_request::Operation, tenant_id: String) -> Self {
        Self {
            management_resource_arm_id,
            operation,
            tenant_id,
        }
    }
}
pub mod manage_link_request {
    use super::*;
    #[doc = "Operation to be performed - Link, Unlink, Relink"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        Link,
        Unlink,
        Relink,
    }
}
#[doc = "Contains information about management resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementResourceData {
    #[doc = "Arm ID of management resource associated with inventory"]
    #[serde(rename = "armId", default, skip_serializing_if = "Option::is_none")]
    pub arm_id: Option<String>,
    #[doc = "Tenant ID of management resource associated with inventory"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagementResourceData {
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
#[doc = "Contains information about the order item to which inventory belongs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderItemData {
    #[doc = "Arm ID of order item"]
    #[serde(rename = "armId", default, skip_serializing_if = "Option::is_none")]
    pub arm_id: Option<String>,
    #[doc = "Order item type - purchase or rental"]
    #[serde(rename = "orderItemType", default, skip_serializing_if = "Option::is_none")]
    pub order_item_type: Option<order_item_data::OrderItemType>,
}
impl OrderItemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod order_item_data {
    use super::*;
    #[doc = "Order item type - purchase or rental"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OrderItemType {
        Purchase,
        Rental,
    }
}
#[doc = "Represents partner inventory contract"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerInventory {
    #[doc = "Represents inventory properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InventoryProperties>,
}
impl PartnerInventory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the list of partner inventories"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerInventoryList {
    #[doc = "List of partner inventories"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PartnerInventory>,
    #[doc = "Link for the next set of partner inventories."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PartnerInventoryList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PartnerInventoryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for SearchInventories call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchInventoriesRequest {
    #[doc = "Serial number of the inventory"]
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[doc = "Family identifier for inventory"]
    #[serde(rename = "familyIdentifier")]
    pub family_identifier: String,
}
impl SearchInventoriesRequest {
    pub fn new(serial_number: String, family_identifier: String) -> Self {
        Self {
            serial_number,
            family_identifier,
        }
    }
}
#[doc = "Specification details for the inventory"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpecificationDetails {
    #[doc = "Name of the specification property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value of the specification property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SpecificationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource stage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageDetails {
    #[doc = "Stage status."]
    #[serde(rename = "stageStatus", default, skip_serializing_if = "Option::is_none")]
    pub stage_status: Option<stage_details::StageStatus>,
    #[doc = "Stage name"]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<stage_details::StageName>,
    #[doc = "Display name of the resource stage."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Stage start time"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl StageDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod stage_details {
    use super::*;
    #[doc = "Stage status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageStatus {
        None,
        InProgress,
        Succeeded,
        Failed,
        Cancelled,
        Cancelling,
    }
    #[doc = "Stage name"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageName {
        DeviceOrdered,
        DevicePrepared,
        PickedUp,
        #[serde(rename = "AtAzureDC")]
        AtAzureDc,
        DataCopy,
        Completed,
        CompletedWithErrors,
        Cancelled,
        Aborted,
        CompletedWithWarnings,
        #[serde(rename = "ReadyToDispatchFromAzureDC")]
        ReadyToDispatchFromAzureDc,
        #[serde(rename = "ReadyToReceiveAtAzureDC")]
        ReadyToReceiveAtAzureDc,
        Placed,
        InReview,
        Confirmed,
        ReadyForDispatch,
        Shipped,
        Delivered,
        InUse,
    }
}
#[doc = "Contains subscription details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionDetails {
    #[doc = "Subscription Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Subscription State"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Subscription QuotaId"]
    #[serde(rename = "quotaId", default, skip_serializing_if = "Option::is_none")]
    pub quota_id: Option<String>,
}
impl SubscriptionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
