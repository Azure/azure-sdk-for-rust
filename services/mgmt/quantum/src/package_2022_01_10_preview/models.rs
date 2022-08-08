#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Details of check name availability request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "Name for checking availability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type of Quantum Workspace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of check name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Indicator of availability of the Quantum Workspace resource name."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason of unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The detailed info regarding the reason associated with the Namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
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
#[doc = "The response of a list Providers operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OfferingsListResult {
    #[doc = "Result of a list Providers operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProviderDescription>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of Providers."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OfferingsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OfferingsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detailed pricing information for an sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PricingDetail {
    #[doc = "Unique id for this pricing information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The unit cost of this sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl PricingDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about pricing dimension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PricingDimension {
    #[doc = "Unique id of this pricing dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name of this pricing dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PricingDimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a Provider. A Provider is an entity that offers Targets to run Azure Quantum Jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Provider {
    #[doc = "Unique id of this provider."]
    #[serde(rename = "providerId", default, skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[doc = "The sku associated with pricing information for this provider."]
    #[serde(rename = "providerSku", default, skip_serializing_if = "Option::is_none")]
    pub provider_sku: Option<String>,
    #[doc = "A Uri identifying the specific instance of this provider."]
    #[serde(rename = "instanceUri", default, skip_serializing_if = "Option::is_none")]
    pub instance_uri: Option<String>,
    #[doc = "The provider's marketplace application display name."]
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[doc = "Provisioning status field"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<provider::ProvisioningState>,
    #[doc = "Id to track resource usage for the provider."]
    #[serde(rename = "resourceUsageId", default, skip_serializing_if = "Option::is_none")]
    pub resource_usage_id: Option<String>,
}
impl Provider {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provider {
    use super::*;
    #[doc = "Provisioning status field"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Launching,
        Updating,
        Deleting,
        Deleted,
        Failed,
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
                Self::Launching => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Launching"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleted"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about an offering. A provider offering is an entity that offers Targets to run Azure Quantum Jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderDescription {
    #[doc = "Unique provider's id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Provider's display name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Provider properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProviderProperties>,
}
impl ProviderDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderProperties {
    #[doc = "A description about this provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Provider type."]
    #[serde(rename = "providerType", default, skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[doc = "Company name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[doc = "Provider's default endpoint."]
    #[serde(rename = "defaultEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub default_endpoint: Option<String>,
    #[doc = "Azure Active Directory info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aad: Option<provider_properties::Aad>,
    #[doc = "Provider's Managed-Application info"]
    #[serde(rename = "managedApplication", default, skip_serializing_if = "Option::is_none")]
    pub managed_application: Option<provider_properties::ManagedApplication>,
    #[doc = "The list of targets available from this provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<TargetDescription>,
    #[doc = "The list of skus available from this provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skus: Vec<SkuDescription>,
    #[doc = "The list of quota dimensions from the provider."]
    #[serde(rename = "quotaDimensions", default, skip_serializing_if = "Vec::is_empty")]
    pub quota_dimensions: Vec<QuotaDimension>,
    #[doc = "The list of pricing dimensions from the provider."]
    #[serde(rename = "pricingDimensions", default, skip_serializing_if = "Vec::is_empty")]
    pub pricing_dimensions: Vec<PricingDimension>,
}
impl ProviderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provider_properties {
    use super::*;
    #[doc = "Azure Active Directory info."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Aad {
        #[doc = "Provider's application id."]
        #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
        pub application_id: Option<String>,
        #[doc = "Provider's tenant id."]
        #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
        pub tenant_id: Option<String>,
    }
    impl Aad {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Provider's Managed-Application info"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ManagedApplication {
        #[doc = "Provider's publisher id."]
        #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
        pub publisher_id: Option<String>,
        #[doc = "Provider's offer id."]
        #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
        pub offer_id: Option<String>,
    }
    impl ManagedApplication {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The resource proxy definition object for quantum workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuantumWorkspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of a Workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceResourceProperties>,
    #[doc = "Managed Identity information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<quantum_workspace::Identity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl QuantumWorkspace {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
            system_data: None,
        }
    }
}
pub mod quantum_workspace {
    use super::*;
    #[doc = "Managed Identity information."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Identity {
        #[doc = "The principal ID of resource identity."]
        #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
        pub principal_id: Option<String>,
        #[doc = "The tenant ID of resource."]
        #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
        pub tenant_id: Option<String>,
        #[doc = "The identity type."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<identity::Type>,
    }
    impl Identity {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod identity {
        use super::*;
        #[doc = "The identity type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Type")]
        pub enum Type {
            SystemAssigned,
            None,
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
                    Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                    Self::None => serializer.serialize_unit_variant("Type", 1u32, "None"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Information about a specific quota dimension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaDimension {
    #[doc = "Unique id of this dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The scope of this quota dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The reset period of this quota dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[doc = "The max limit of this dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota: Option<f64>,
    #[doc = "The display name of this quota dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A description about this quota dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The standard unit of measurement used for this quota dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The standard unit of measurement used for this quota dimension in plural form."]
    #[serde(rename = "unitPlural", default, skip_serializing_if = "Option::is_none")]
    pub unit_plural: Option<String>,
}
impl QuotaDimension {
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
#[doc = "Information about a specific sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDescription {
    #[doc = "Unique sku id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of this sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of this sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Description about this sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Uri to subscribe to the restricted access sku."]
    #[serde(rename = "restrictedAccessUri", default, skip_serializing_if = "Option::is_none")]
    pub restricted_access_uri: Option<String>,
    #[doc = "Flag to indicate whether the sku should be automatically added during workspace creation."]
    #[serde(rename = "autoAdd", default, skip_serializing_if = "Option::is_none")]
    pub auto_add: Option<bool>,
    #[doc = "The list of targets available for this sku."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<String>,
    #[doc = "The list of quota dimensions for this sku."]
    #[serde(rename = "quotaDimensions", default, skip_serializing_if = "Vec::is_empty")]
    pub quota_dimensions: Vec<QuotaDimension>,
    #[doc = "The list of pricing details for the sku."]
    #[serde(rename = "pricingDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub pricing_details: Vec<PricingDetail>,
}
impl SkuDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a Target. A target is the component that can process a specific type of Job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetDescription {
    #[doc = "Unique target id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of this target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A description about this target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "List of data formats accepted by this target."]
    #[serde(rename = "acceptedDataFormats", default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_data_formats: Vec<String>,
    #[doc = "List of content encodings accepted by this target."]
    #[serde(rename = "acceptedContentEncodings", default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_content_encodings: Vec<String>,
}
impl TargetDescription {
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
#[doc = "The response of a list Workspaces operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListResult {
    #[doc = "Result of a list Workspaces operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QuantumWorkspace>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of Workspaces."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceResourceProperties {
    #[doc = "List of Providers selected for this Workspace"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub providers: Vec<Provider>,
    #[doc = "Whether the current workspace is ready to accept Jobs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usable: Option<workspace_resource_properties::Usable>,
    #[doc = "Provisioning status field"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workspace_resource_properties::ProvisioningState>,
    #[doc = "ARM Resource Id of the storage account associated with this workspace."]
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<String>,
    #[doc = "The URI of the workspace endpoint."]
    #[serde(rename = "endpointUri", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
}
impl WorkspaceResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_resource_properties {
    use super::*;
    #[doc = "Whether the current workspace is ready to accept Jobs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Usable")]
    pub enum Usable {
        Yes,
        No,
        Partial,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Usable {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Usable {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Usable {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Yes => serializer.serialize_unit_variant("Usable", 0u32, "Yes"),
                Self::No => serializer.serialize_unit_variant("Usable", 1u32, "No"),
                Self::Partial => serializer.serialize_unit_variant("Usable", 2u32, "Partial"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Provisioning status field"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        ProviderLaunching,
        ProviderUpdating,
        ProviderDeleting,
        ProviderProvisioning,
        Failed,
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
                Self::ProviderLaunching => serializer.serialize_unit_variant("ProvisioningState", 1u32, "ProviderLaunching"),
                Self::ProviderUpdating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "ProviderUpdating"),
                Self::ProviderDeleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "ProviderDeleting"),
                Self::ProviderProvisioning => serializer.serialize_unit_variant("ProvisioningState", 4u32, "ProviderProvisioning"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Operation provided by provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Properties of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Properties of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Provider name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Lists the operations available."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[doc = "Url to follow for getting next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of operations"]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationsList {
    pub fn new(value: Vec<Operation>) -> Self {
        Self { next_link: None, value }
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
