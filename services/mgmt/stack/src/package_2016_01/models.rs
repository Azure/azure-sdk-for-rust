#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The resource containing the Azure Stack activation key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivationKeyResult {
    #[doc = "Azure Stack activation key."]
    #[serde(rename = "activationKey", default, skip_serializing_if = "Option::is_none")]
    pub activation_key: Option<String>,
}
impl ActivationKeyResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product compatibility"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Compatibility {
    #[doc = "Tells if product is compatible with current device"]
    #[serde(rename = "isCompatible", default, skip_serializing_if = "Option::is_none")]
    pub is_compatible: Option<bool>,
    #[doc = "Short error message if any compatibility issues are found"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Full error message if any compatibility issues are found"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "List of all issues found"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub issues: Vec<CompatibilityIssue>,
}
impl Compatibility {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Compatibility issue"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CompatibilityIssue")]
pub enum CompatibilityIssue {
    HigherDeviceVersionRequired,
    LowerDeviceVersionRequired,
    CapacityBillingModelRequired,
    PayAsYouGoBillingModelRequired,
    DevelopmentBillingModelRequired,
    #[serde(rename = "AzureADIdentitySystemRequired")]
    AzureAdIdentitySystemRequired,
    #[serde(rename = "ADFSIdentitySystemRequired")]
    AdfsIdentitySystemRequired,
    ConnectionToInternetRequired,
    ConnectionToAzureRequired,
    DisconnectedEnvironmentRequired,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CompatibilityIssue {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CompatibilityIssue {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CompatibilityIssue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::HigherDeviceVersionRequired => {
                serializer.serialize_unit_variant("CompatibilityIssue", 0u32, "HigherDeviceVersionRequired")
            }
            Self::LowerDeviceVersionRequired => serializer.serialize_unit_variant("CompatibilityIssue", 1u32, "LowerDeviceVersionRequired"),
            Self::CapacityBillingModelRequired => {
                serializer.serialize_unit_variant("CompatibilityIssue", 2u32, "CapacityBillingModelRequired")
            }
            Self::PayAsYouGoBillingModelRequired => {
                serializer.serialize_unit_variant("CompatibilityIssue", 3u32, "PayAsYouGoBillingModelRequired")
            }
            Self::DevelopmentBillingModelRequired => {
                serializer.serialize_unit_variant("CompatibilityIssue", 4u32, "DevelopmentBillingModelRequired")
            }
            Self::AzureAdIdentitySystemRequired => {
                serializer.serialize_unit_variant("CompatibilityIssue", 5u32, "AzureADIdentitySystemRequired")
            }
            Self::AdfsIdentitySystemRequired => serializer.serialize_unit_variant("CompatibilityIssue", 6u32, "ADFSIdentitySystemRequired"),
            Self::ConnectionToInternetRequired => {
                serializer.serialize_unit_variant("CompatibilityIssue", 7u32, "ConnectionToInternetRequired")
            }
            Self::ConnectionToAzureRequired => serializer.serialize_unit_variant("CompatibilityIssue", 8u32, "ConnectionToAzureRequired"),
            Self::DisconnectedEnvironmentRequired => {
                serializer.serialize_unit_variant("CompatibilityIssue", 9u32, "DisconnectedEnvironmentRequired")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Compute role type (IaaS or PaaS)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ComputeRole")]
pub enum ComputeRole {
    None,
    IaaS,
    PaaS,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ComputeRole {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ComputeRole {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ComputeRole {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ComputeRole", 0u32, "None"),
            Self::IaaS => serializer.serialize_unit_variant("ComputeRole", 1u32, "IaaS"),
            Self::PaaS => serializer.serialize_unit_variant("ComputeRole", 2u32, "PaaS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Data disk image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDiskImage {
    #[doc = "The LUN."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "SAS key for source blob."]
    #[serde(rename = "sourceBlobSasUri", default, skip_serializing_if = "Option::is_none")]
    pub source_blob_sas_uri: Option<String>,
}
impl DataDiskImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Device Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceConfiguration {
    #[doc = "Version of the device."]
    #[serde(rename = "deviceVersion", default, skip_serializing_if = "Option::is_none")]
    pub device_version: Option<String>,
    #[doc = "Identity system of the device."]
    #[serde(rename = "identitySystem", default, skip_serializing_if = "Option::is_none")]
    pub identity_system: Option<device_configuration::IdentitySystem>,
}
impl DeviceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod device_configuration {
    use super::*;
    #[doc = "Identity system of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IdentitySystem")]
    pub enum IdentitySystem {
        #[serde(rename = "AzureAD")]
        AzureAd,
        #[serde(rename = "ADFS")]
        Adfs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IdentitySystem {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IdentitySystem {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IdentitySystem {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureAd => serializer.serialize_unit_variant("IdentitySystem", 0u32, "AzureAD"),
                Self::Adfs => serializer.serialize_unit_variant("IdentitySystem", 1u32, "ADFS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "Extended description about the product required for installing it into Azure Stack."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedProduct {
    #[doc = "The URI to the .azpkg file that provides information required for showing product in the gallery."]
    #[serde(rename = "galleryPackageBlobSasUri", default, skip_serializing_if = "Option::is_none")]
    pub gallery_package_blob_sas_uri: Option<String>,
    #[doc = "Specifies the kind of the product (virtualMachine or virtualMachineExtension)."]
    #[serde(rename = "productKind", default, skip_serializing_if = "Option::is_none")]
    pub product_kind: Option<String>,
    #[doc = "Product information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExtendedProductProperties>,
}
impl ExtendedProduct {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedProductProperties {
    #[serde(flatten)]
    pub virtual_machine_extension_product_properties: VirtualMachineExtensionProductProperties,
    #[serde(flatten)]
    pub virtual_machine_product_properties: VirtualMachineProductProperties,
}
impl ExtendedProductProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Links to product icons."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IconUris {
    #[doc = "URI to large icon."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large: Option<String>,
    #[doc = "URI to wide icon."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wide: Option<String>,
    #[doc = "URI to medium icon."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[doc = "URI to small icon."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small: Option<String>,
    #[doc = "URI to hero icon."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hero: Option<String>,
}
impl IconUris {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update details for product log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceProductLogUpdate {
    #[doc = "Operation to log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Operation status to log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error related to the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "Error details related to operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl MarketplaceProductLogUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operating system type (Windows or Linux)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperatingSystem")]
pub enum OperatingSystem {
    None,
    Windows,
    Linux,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperatingSystem {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperatingSystem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperatingSystem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("OperatingSystem", 0u32, "None"),
            Self::Windows => serializer.serialize_unit_variant("OperatingSystem", 1u32, "Windows"),
            Self::Linux => serializer.serialize_unit_variant("OperatingSystem", 2u32, "Linux"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "OS disk image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsDiskImage {
    #[doc = "Operating system type (Windows or Linux)."]
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<OperatingSystem>,
    #[doc = "SAS key for source blob."]
    #[serde(rename = "sourceBlobSasUri", default, skip_serializing_if = "Option::is_none")]
    pub source_blob_sas_uri: Option<String>,
}
impl OsDiskImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Product {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties portion of the product resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductNestedProperties>,
}
impl Product {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Link with additional information about a product."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductLink {
    #[doc = "The description of the link."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The URI corresponding to the link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl ProductLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pageable list of products."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductList {
    #[doc = "URI to the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of products."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Product>,
}
impl azure_core::Continuable for ProductList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProductList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product action log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductLog {
    #[doc = "Log ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Logged product ID."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "Logged subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Logged registration name."]
    #[serde(rename = "registrationName", default, skip_serializing_if = "Option::is_none")]
    pub registration_name: Option<String>,
    #[doc = "Logged resource group name."]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "Logged operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Operation start datetime."]
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "Operation end datetime."]
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Operation error data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "Operation error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl ProductLog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties portion of the product resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductNestedProperties {
    #[doc = "The display name of the product."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The user-friendly name of the product publisher."]
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[doc = "Publisher identifier."]
    #[serde(rename = "publisherIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub publisher_identifier: Option<String>,
    #[doc = "The offer representing the product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The version of the product offer."]
    #[serde(rename = "offerVersion", default, skip_serializing_if = "Option::is_none")]
    pub offer_version: Option<String>,
    #[doc = "The product SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The part number used for billing purposes."]
    #[serde(rename = "billingPartNumber", default, skip_serializing_if = "Option::is_none")]
    pub billing_part_number: Option<String>,
    #[doc = "The type of the Virtual Machine Extension."]
    #[serde(rename = "vmExtensionType", default, skip_serializing_if = "Option::is_none")]
    pub vm_extension_type: Option<String>,
    #[doc = "The identifier of the gallery item corresponding to the product."]
    #[serde(rename = "galleryItemIdentity", default, skip_serializing_if = "Option::is_none")]
    pub gallery_item_identity: Option<String>,
    #[doc = "Links to product icons."]
    #[serde(rename = "iconUris", default, skip_serializing_if = "Option::is_none")]
    pub icon_uris: Option<IconUris>,
    #[doc = "Additional links available for this product."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub links: Vec<ProductLink>,
    #[doc = "The legal terms."]
    #[serde(rename = "legalTerms", default, skip_serializing_if = "Option::is_none")]
    pub legal_terms: Option<String>,
    #[doc = "The privacy policy."]
    #[serde(rename = "privacyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<String>,
    #[doc = "The length of product content."]
    #[serde(rename = "payloadLength", default, skip_serializing_if = "Option::is_none")]
    pub payload_length: Option<i64>,
    #[doc = "The kind of the product (virtualMachine or virtualMachineExtension)"]
    #[serde(rename = "productKind", default, skip_serializing_if = "Option::is_none")]
    pub product_kind: Option<String>,
    #[doc = "Additional properties of the product"]
    #[serde(rename = "productProperties", default, skip_serializing_if = "Option::is_none")]
    pub product_properties: Option<ProductProperties>,
    #[doc = "Product compatibility"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<Compatibility>,
}
impl ProductNestedProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional properties of the product"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductProperties {
    #[doc = "The version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ProductProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Registration information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Registration {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties portion of the registration resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationProperties>,
    #[doc = "The entity tag used for optimistic concurrency when modifying the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Registration {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            etag: None,
        }
    }
}
#[doc = "Pageable list of registrations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationList {
    #[doc = "URI to the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of Registrations"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Registration>,
}
impl azure_core::Continuable for RegistrationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RegistrationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Registration resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationParameter {
    #[doc = "Properties of the Azure Stack registration resource"]
    pub properties: RegistrationParameterProperties,
    #[doc = "Location of the resource."]
    pub location: registration_parameter::Location,
}
impl RegistrationParameter {
    pub fn new(properties: RegistrationParameterProperties, location: registration_parameter::Location) -> Self {
        Self { properties, location }
    }
}
pub mod registration_parameter {
    use super::*;
    #[doc = "Location of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Location")]
    pub enum Location {
        #[serde(rename = "global")]
        Global,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Location {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Location {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Location {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Global => serializer.serialize_unit_variant("Location", 0u32, "global"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the Azure Stack registration resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationParameterProperties {
    #[doc = "The token identifying registered Azure Stack"]
    #[serde(rename = "registrationToken")]
    pub registration_token: String,
}
impl RegistrationParameterProperties {
    pub fn new(registration_token: String) -> Self {
        Self { registration_token }
    }
}
#[doc = "Properties portion of the registration resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationProperties {
    #[doc = "The object identifier associated with the Azure Stack connecting to Azure."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The identifier of the registered Azure Stack."]
    #[serde(rename = "cloudId", default, skip_serializing_if = "Option::is_none")]
    pub cloud_id: Option<String>,
    #[doc = "Specifies the billing mode for the Azure Stack registration."]
    #[serde(rename = "billingModel", default, skip_serializing_if = "Option::is_none")]
    pub billing_model: Option<String>,
}
impl RegistrationProperties {
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
#[doc = "The URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Uri {
    #[doc = "The URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl Uri {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineExtensionProductProperties {
    #[doc = "Compute role type (IaaS or PaaS)."]
    #[serde(rename = "computeRole", default, skip_serializing_if = "Option::is_none")]
    pub compute_role: Option<ComputeRole>,
    #[doc = "Specifies if product is a Virtual Machine Extension."]
    #[serde(rename = "isSystemExtension", default, skip_serializing_if = "Option::is_none")]
    pub is_system_extension: Option<bool>,
    #[doc = "The URI."]
    #[serde(rename = "sourceBlob", default, skip_serializing_if = "Option::is_none")]
    pub source_blob: Option<Uri>,
    #[doc = "Indicates if specified product supports multiple extensions."]
    #[serde(rename = "supportMultipleExtensions", default, skip_serializing_if = "Option::is_none")]
    pub support_multiple_extensions: Option<bool>,
    #[doc = "Specifies product version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Operating system type (Windows or Linux)."]
    #[serde(rename = "vmOsType", default, skip_serializing_if = "Option::is_none")]
    pub vm_os_type: Option<OperatingSystem>,
    #[doc = "Indicates if virtual machine Scale Set is enabled in the specified product."]
    #[serde(rename = "vmScaleSetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub vm_scale_set_enabled: Option<bool>,
}
impl VirtualMachineExtensionProductProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineProductProperties {
    #[doc = "Specifies product version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "OS disk image."]
    #[serde(rename = "osDiskImage", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_image: Option<OsDiskImage>,
    #[doc = "List of attached data disks."]
    #[serde(
        rename = "dataDiskImages",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_disk_images: Vec<DataDiskImage>,
}
impl VirtualMachineProductProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
