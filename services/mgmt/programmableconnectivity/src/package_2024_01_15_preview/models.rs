#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The Account Type of the Operator API Connections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AccountType")]
pub enum AccountType {
    AzureManaged,
    UserManaged,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AccountType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AccountType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AccountType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureManaged => serializer.serialize_unit_variant("AccountType", 0u32, "AzureManaged"),
            Self::UserManaged => serializer.serialize_unit_variant("AccountType", 1u32, "UserManaged"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details about the Application that would use the Operator's Network APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationProperties {
    #[doc = "Name of the application. Example: Contoso App."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the application."]
    #[serde(rename = "applicationDescription", default, skip_serializing_if = "Option::is_none")]
    pub application_description: Option<String>,
    #[doc = "The category that describes the application."]
    #[serde(rename = "applicationType", default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[doc = "Legal name of the organization owning the application."]
    #[serde(rename = "legalName", default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[doc = "A description of the organization owning the application."]
    #[serde(rename = "organizationDescription", default, skip_serializing_if = "Option::is_none")]
    pub organization_description: Option<String>,
    #[doc = "Unique Tax Number for the user's organization in the country/region the APC Gateway is being purchased."]
    #[serde(rename = "taxNumber", default, skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,
    #[doc = "Email address of the Privacy contact or Data Protection officer of the organization."]
    #[serde(rename = "privacyContactEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub privacy_contact_email_address: Option<String>,
}
impl ApplicationProperties {
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
#[doc = "A Programmable Connectivity Gateway resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gateway {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Gateway resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GatewayProperties>,
}
impl Gateway {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a Gateway list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayListResult {
    #[doc = "The Gateway items on this page"]
    pub value: Vec<Gateway>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GatewayListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GatewayListResult {
    pub fn new(value: Vec<Gateway>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Gateway resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayProperties {
    #[doc = "List of Operator API Connections selected by the user"]
    #[serde(
        rename = "operatorApiConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operator_api_connections: Vec<String>,
    #[doc = "Base URL of the Gateway resource. This is the URL that the users would use to make Open API Gateway requests to the Operators via Azure."]
    #[serde(rename = "gatewayBaseUrl", default, skip_serializing_if = "Option::is_none")]
    pub gateway_base_url: Option<String>,
    #[doc = "The provisioning state of a resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl GatewayProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for updating tags in Gateway resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayTagsUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl GatewayTagsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure marketplace properties for a plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceProperties {
    #[doc = "Azure marketplace Offer ID for this plan."]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Azure marketplace Legacy Offer ID for this plan. This is used to fetch the details of the plan from the Azure marketplace."]
    #[serde(rename = "legacyOfferId", default, skip_serializing_if = "Option::is_none")]
    pub legacy_offer_id: Option<String>,
    #[doc = "Azure marketplace Publisher ID for this plan."]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Azure marketplace Plan ID for this plan."]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Azure marketplace Term ID for this plan."]
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
}
impl MarketplaceProperties {
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
#[doc = "A Programmable Connectivity Operator API Connection resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperatorApiConnection {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Operator API Connection resource properties that cannot be updated once a resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperatorApiConnectionProperties>,
}
impl OperatorApiConnection {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a OperatorApiConnection list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperatorApiConnectionListResult {
    #[doc = "The OperatorApiConnection items on this page"]
    pub value: Vec<OperatorApiConnection>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperatorApiConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperatorApiConnectionListResult {
    pub fn new(value: Vec<OperatorApiConnection>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Operator API Connection resource properties that cannot be updated once a resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperatorApiConnectionProperties {
    #[doc = "Reference to the Operator API Plan Resource ID."]
    #[serde(rename = "operatorApiPlanId")]
    pub operator_api_plan_id: String,
    #[doc = "Details about the SaaS offer purchased from the marketplace."]
    #[serde(rename = "saasProperties", default, skip_serializing_if = "Option::is_none")]
    pub saas_properties: Option<SaasProperties>,
    #[doc = "Details about the Application that would use the Operator's Network APIs."]
    #[serde(rename = "configuredApplication", default, skip_serializing_if = "Option::is_none")]
    pub configured_application: Option<ApplicationProperties>,
    #[doc = "Application ID of the App Developer that is registered with the Operator in a specific country/region."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "Reference to the APC Gateway resource ID."]
    #[serde(rename = "gatewayId")]
    pub gateway_id: String,
    #[doc = "The Account Type of the Operator API Connections."]
    #[serde(rename = "accountType")]
    pub account_type: AccountType,
    #[doc = "Application secret linked to the 'appId'. This should be stored securely and is not returned back when the resource information is read."]
    #[serde(rename = "appSecret", default, skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
    #[doc = "Name of the Operator in the linked Operator API Plan belongs to."]
    #[serde(rename = "operatorName", default, skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<String>,
    #[doc = "The Network API for the current operator in the country/region provided in the linked Operator API Plan."]
    #[serde(rename = "camaraApiName", default, skip_serializing_if = "Option::is_none")]
    pub camara_api_name: Option<String>,
    #[doc = "The provisioning state of a resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Description of the current status of the OperatorApiConnection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}
impl OperatorApiConnectionProperties {
    pub fn new(operator_api_plan_id: String, gateway_id: String, account_type: AccountType) -> Self {
        Self {
            operator_api_plan_id,
            saas_properties: None,
            configured_application: None,
            app_id: None,
            gateway_id,
            account_type,
            app_secret: None,
            operator_name: None,
            camara_api_name: None,
            provisioning_state: None,
            status: None,
        }
    }
}
#[doc = "The type used for update operations of the OperatorApiConnection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatorApiConnectionUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the OperatorApiConnection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperatorApiConnectionUpdateProperties>,
}
impl OperatorApiConnectionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the OperatorApiConnection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatorApiConnectionUpdateProperties {
    #[doc = "Reference to the Operator API Plan Resource ID."]
    #[serde(rename = "operatorApiPlanId", default, skip_serializing_if = "Option::is_none")]
    pub operator_api_plan_id: Option<String>,
    #[doc = "Details about the SaaS offer purchased from the marketplace."]
    #[serde(rename = "saasProperties", default, skip_serializing_if = "Option::is_none")]
    pub saas_properties: Option<SaasProperties>,
    #[doc = "Details about the Application that would use the Operator's Network APIs."]
    #[serde(rename = "configuredApplication", default, skip_serializing_if = "Option::is_none")]
    pub configured_application: Option<ApplicationProperties>,
    #[doc = "Application ID of the App Developer that is registered with the Operator in a specific country/region."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "Application secret linked to the 'appId'. This should be stored securely and is not returned back when the resource information is read."]
    #[serde(rename = "appSecret", default, skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
}
impl OperatorApiConnectionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Programmable Connectivity Operator API Plans resource. This is a readonly resource that indicates which Operator Network APIs are available in the user's subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatorApiPlan {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Operator API Plan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperatorApiPlanProperties>,
}
impl OperatorApiPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a OperatorApiPlan list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperatorApiPlanListResult {
    #[doc = "The OperatorApiPlan items on this page"]
    pub value: Vec<OperatorApiPlan>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperatorApiPlanListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperatorApiPlanListResult {
    pub fn new(value: Vec<OperatorApiPlan>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Operator API Plan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatorApiPlanProperties {
    #[doc = "Name of the Operator this plan belongs to."]
    #[serde(rename = "operatorName", default, skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<String>,
    #[doc = "Standardized Network API name defined by CAMARA specifications."]
    #[serde(rename = "camaraApiName", default, skip_serializing_if = "Option::is_none")]
    pub camara_api_name: Option<String>,
    #[doc = "List of Azure regions where this offer is supported."]
    #[serde(
        rename = "supportedLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_locations: Vec<String>,
    #[doc = "List of country/region names where this plan is being supported by the Operator."]
    #[serde(
        rename = "operatorRegions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operator_regions: Vec<String>,
    #[doc = "List of country/region names where this plan is being supported by Azure Marketplace."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub markets: Vec<String>,
    #[doc = "The limits, if any, will be imposed by the operator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<String>,
    #[doc = "Azure marketplace properties for a plan."]
    #[serde(rename = "marketplaceProperties", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_properties: Option<MarketplaceProperties>,
    #[doc = "The provisioning state of a resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl OperatorApiPlanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    Updating,
    Deleting,
    Accepted,
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
            Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
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
#[doc = "Details about the SaaS offer purchased from the marketplace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasProperties {
    #[doc = "Subscription ID of the SaaS offer purchased from the marketplace."]
    #[serde(rename = "saasSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub saas_subscription_id: Option<String>,
    #[doc = "Resource ID of the SaaS offer purchased from the marketplace."]
    #[serde(rename = "saasResourceId", default, skip_serializing_if = "Option::is_none")]
    pub saas_resource_id: Option<String>,
}
impl SaasProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of the current status of the OperatorApiConnection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Status {
    #[doc = "Current state of the OperatorApiConnection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Explanation of the current state of the OperatorApiConnection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl Status {
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
