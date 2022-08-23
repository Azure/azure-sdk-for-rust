#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents an attached NetworkConnection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachedNetworkConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an attached NetworkConnection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AttachedNetworkConnectionProperties>,
}
impl AttachedNetworkConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an attached NetworkConnection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachedNetworkConnectionProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The resource ID of the NetworkConnection you want to attach."]
    #[serde(rename = "networkConnectionId")]
    pub network_connection_id: String,
    #[doc = "The geo-location where the NetworkConnection resource specified in 'networkConnectionResourceId' property lives."]
    #[serde(rename = "networkConnectionLocation", default, skip_serializing_if = "Option::is_none")]
    pub network_connection_location: Option<String>,
    #[doc = "Health check status values"]
    #[serde(rename = "healthCheckStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_check_status: Option<HealthCheckStatus>,
    #[doc = "Active Directory join type"]
    #[serde(rename = "domainJoinType", default, skip_serializing_if = "Option::is_none")]
    pub domain_join_type: Option<DomainJoinType>,
}
impl AttachedNetworkConnectionProperties {
    pub fn new(network_connection_id: String) -> Self {
        Self {
            provisioning_state: None,
            network_connection_id,
            network_connection_location: None,
            health_check_status: None,
            domain_join_type: None,
        }
    }
}
#[doc = "Results of the Attached Networks list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachedNetworkListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AttachedNetworkConnection>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AttachedNetworkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AttachedNetworkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A name/value pair to describe a capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Capability {
    #[doc = "Name of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Capability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Catalog {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a catalog."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CatalogProperties>,
}
impl Catalog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the catalog list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Catalog>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CatalogListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CatalogListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogProperties {
    #[serde(flatten)]
    pub catalog_update_properties: CatalogUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "When the catalog was last synced."]
    #[serde(rename = "lastSyncTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_sync_time: Option<time::OffsetDateTime>,
}
impl CatalogProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The catalog's properties for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogUpdate {
    #[doc = "Properties of a catalog. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CatalogUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CatalogUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a catalog. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogUpdateProperties {
    #[doc = "Properties for a Git repository catalog."]
    #[serde(rename = "gitHub", default, skip_serializing_if = "Option::is_none")]
    pub git_hub: Option<GitCatalog>,
    #[doc = "Properties for a Git repository catalog."]
    #[serde(rename = "adoGit", default, skip_serializing_if = "Option::is_none")]
    pub ado_git: Option<GitCatalog>,
}
impl CatalogUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the DevCenter service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[doc = "An error response from the DevCenter service."]
    pub error: CloudErrorBody,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new(error: CloudErrorBody) -> Self {
        Self { error }
    }
}
#[doc = "An error response from the DevCenter service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    pub code: String,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    pub message: String,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[doc = "Represents a definition for a Developer Machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxDefinition {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of a Dev Box definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevBoxDefinitionProperties>,
}
impl DevBoxDefinition {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Results of the Dev Box definition list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevBoxDefinitionListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DevBoxDefinition>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DevBoxDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DevBoxDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Dev Box definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxDefinitionProperties {
    #[serde(flatten)]
    pub dev_box_definition_update_properties: DevBoxDefinitionUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Image validation status"]
    #[serde(rename = "imageValidationStatus", default, skip_serializing_if = "Option::is_none")]
    pub image_validation_status: Option<ImageValidationStatus>,
    #[doc = "Image validation error details"]
    #[serde(rename = "imageValidationErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub image_validation_error_details: Option<ImageValidationErrorDetails>,
    #[doc = "Image reference information"]
    #[serde(rename = "activeImageReference", default, skip_serializing_if = "Option::is_none")]
    pub active_image_reference: Option<ImageReference>,
}
impl DevBoxDefinitionProperties {
    pub fn new() -> Self {
        Self {
            dev_box_definition_update_properties: DevBoxDefinitionUpdateProperties::default(),
            provisioning_state: None,
            image_validation_status: None,
            image_validation_error_details: None,
            active_image_reference: None,
        }
    }
}
#[doc = "Partial update of a Dev Box definition resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevBoxDefinitionUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Properties of a Dev Box definition. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevBoxDefinitionUpdateProperties>,
}
impl DevBoxDefinitionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Dev Box definition. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevBoxDefinitionUpdateProperties {
    #[doc = "Image reference information"]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The storage type used for the Operating System disk of Dev Boxes created using this definition."]
    #[serde(rename = "osStorageType", default, skip_serializing_if = "Option::is_none")]
    pub os_storage_type: Option<String>,
}
impl DevBoxDefinitionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a devcenter resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevCenter {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the devcenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevCenterProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl DevCenter {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "Result of the list devcenters operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DevCenter>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DevCenterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DevCenterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the devcenter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DevCenterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition representing SKU for DevCenter resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevCenterSku {
    #[serde(flatten)]
    pub sku: Sku,
    #[doc = "The name of the resource type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "SKU supported locations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Collection of name/value pairs to describe the SKU capabilities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
}
impl DevCenterSku {
    pub fn new(sku: Sku) -> Self {
        Self {
            sku,
            resource_type: None,
            locations: Vec::new(),
            capabilities: Vec::new(),
        }
    }
}
#[doc = "The devcenter resource for partial updates. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl DevCenterUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory join type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DomainJoinType")]
pub enum DomainJoinType {
    #[serde(rename = "HybridAzureADJoin")]
    HybridAzureAdJoin,
    #[serde(rename = "AzureADJoin")]
    AzureAdJoin,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DomainJoinType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DomainJoinType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DomainJoinType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::HybridAzureAdJoin => serializer.serialize_unit_variant("DomainJoinType", 0u32, "HybridAzureADJoin"),
            Self::AzureAdJoin => serializer.serialize_unit_variant("DomainJoinType", 1u32, "AzureADJoin"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enable or disable status. Indicates whether the property applied to is either enabled or disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnableStatus")]
pub enum EnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("EnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("EnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A role that can be assigned to a user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentRole {
    #[doc = "The common name of the Role Assignment. This is a descriptive name such as 'AcrPush'."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "This is a description of the Role Assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl EnvironmentRole {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentType {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an environment type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentTypeProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl EnvironmentType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the environment type list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentTypeListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnvironmentType>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EnvironmentTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EnvironmentTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentTypeProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl EnvironmentTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The environment type for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentTypeUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl EnvironmentTypeUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Gallery {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a gallery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryProperties>,
}
impl Gallery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the gallery list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Gallery>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GalleryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GalleryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The resource ID of the backing Azure Compute Gallery."]
    #[serde(rename = "galleryResourceId")]
    pub gallery_resource_id: String,
}
impl GalleryProperties {
    pub fn new(gallery_resource_id: String) -> Self {
        Self {
            provisioning_state: None,
            gallery_resource_id,
        }
    }
}
#[doc = "Properties for a Git repository catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitCatalog {
    #[doc = "Git URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Git branch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "A reference to the Key Vault secret containing a security token to authenticate to a Git repository."]
    #[serde(rename = "secretIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub secret_identifier: Option<String>,
    #[doc = "The folder where the catalog items can be found inside the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl GitCatalog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An individual health check item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthCheck {
    #[doc = "Health check status values"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<HealthCheckStatus>,
    #[doc = "The display name of this health check item."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Start time of health check item."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the health check item."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The type of error that occurred during this health check."]
    #[serde(rename = "errorType", default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[doc = "The recommended action to fix the corresponding error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Additional details about the health check or the recommended action."]
    #[serde(rename = "additionalDetails", default, skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String>,
}
impl HealthCheck {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Health check status values"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthCheckStatus")]
pub enum HealthCheckStatus {
    Pending,
    Running,
    Passed,
    Failed,
    Warning,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthCheckStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthCheckStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthCheckStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("HealthCheckStatus", 0u32, "Pending"),
            Self::Running => serializer.serialize_unit_variant("HealthCheckStatus", 1u32, "Running"),
            Self::Passed => serializer.serialize_unit_variant("HealthCheckStatus", 2u32, "Passed"),
            Self::Failed => serializer.serialize_unit_variant("HealthCheckStatus", 3u32, "Failed"),
            Self::Warning => serializer.serialize_unit_variant("HealthCheckStatus", 4u32, "Warning"),
            Self::Unknown => serializer.serialize_unit_variant("HealthCheckStatus", 5u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Health Check details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthCheckStatusDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Health Check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthCheckStatusDetailsProperties>,
}
impl HealthCheckStatusDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the network health check list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthCheckStatusDetailsListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HealthCheckStatusDetails>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HealthCheckStatusDetailsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HealthCheckStatusDetailsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Health Check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthCheckStatusDetailsProperties {
    #[doc = "Start time of last execution of the health checks."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End time of last execution of the health checks."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Details for each health check item."]
    #[serde(rename = "healthChecks", default, skip_serializing_if = "Vec::is_empty")]
    pub health_checks: Vec<HealthCheck>,
}
impl HealthCheckStatusDetailsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Image {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageProperties>,
}
impl Image {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the image list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Image>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ImageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageProperties {
    #[doc = "The description of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The publisher of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The name of the image offer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The SKU name for the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Properties for a recommended machine configuration."]
    #[serde(rename = "recommendedMachineConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub recommended_machine_configuration: Option<RecommendedMachineConfiguration>,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image reference information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[doc = "Image ID, or Image version ID. When Image ID is provided, its latest version will be used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The actual version of the image after use. When id references a gallery image latest version, this will indicate the actual version in use."]
    #[serde(rename = "exactVersion", default, skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
    #[doc = "The image publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The image offer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The image sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image validation error details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageValidationErrorDetails {
    #[doc = "An identifier for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ImageValidationErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image validation status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImageValidationStatus")]
pub enum ImageValidationStatus {
    Unknown,
    Pending,
    Succeeded,
    Failed,
    TimedOut,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImageValidationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImageValidationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImageValidationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ImageValidationStatus", 0u32, "Unknown"),
            Self::Pending => serializer.serialize_unit_variant("ImageValidationStatus", 1u32, "Pending"),
            Self::Succeeded => serializer.serialize_unit_variant("ImageValidationStatus", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ImageValidationStatus", 3u32, "Failed"),
            Self::TimedOut => serializer.serialize_unit_variant("ImageValidationStatus", 4u32, "TimedOut"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents an image version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an image version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageVersionProperties>,
}
impl ImageVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the image version list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageVersionListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ImageVersion>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ImageVersionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an image version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageVersionProperties {
    #[doc = "The semantic version string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The datetime that the backing image version was published."]
    #[serde(rename = "publishedDate", default, with = "azure_core::date::rfc3339::option")]
    pub published_date: Option<time::OffsetDateTime>,
    #[doc = "If the version should be excluded from being treated as the latest version."]
    #[serde(rename = "excludeFromLatest", default, skip_serializing_if = "Option::is_none")]
    pub exclude_from_latest: Option<bool>,
    #[doc = "The size of the OS disk image, in GB."]
    #[serde(rename = "osDiskImageSizeInGb", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_image_size_in_gb: Option<i32>,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ImageVersionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "License Types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LicenseType")]
pub enum LicenseType {
    #[serde(rename = "Windows_Client")]
    WindowsClient,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LicenseType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LicenseType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LicenseType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::WindowsClient => serializer.serialize_unit_variant("LicenseType", 0u32, "Windows_Client"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of Core Usages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListUsagesResult {
    #[doc = "The array page of Usages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
    #[doc = "The link to get the next page of Usage result."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListUsagesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListUsagesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LocalAdminStatus")]
pub enum LocalAdminStatus {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LocalAdminStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LocalAdminStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LocalAdminStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("LocalAdminStatus", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("LocalAdminStatus", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Network related settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConnection {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Network properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkProperties>,
}
impl NetworkConnection {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Result of the network connection list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConnectionListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkConnection>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network connection properties for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConnectionUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Properties of network connection. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkConnectionUpdateProperties>,
}
impl NetworkConnectionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of network connection. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConnectionUpdateProperties {
    #[doc = "The subnet to attach Virtual Machines to"]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Active Directory domain name"]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "Active Directory domain Organization Unit (OU)"]
    #[serde(rename = "organizationUnit", default, skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    #[doc = "The username of an Active Directory account (user or service account) that has permissions to create computer objects in Active Directory. Required format: admin@contoso.com."]
    #[serde(rename = "domainUsername", default, skip_serializing_if = "Option::is_none")]
    pub domain_username: Option<String>,
    #[doc = "The password for the account used to join domain"]
    #[serde(rename = "domainPassword", default, skip_serializing_if = "Option::is_none")]
    pub domain_password: Option<String>,
}
impl NetworkConnectionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProperties {
    #[serde(flatten)]
    pub network_connection_update_properties: NetworkConnectionUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Health check status values"]
    #[serde(rename = "healthCheckStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_check_status: Option<HealthCheckStatus>,
    #[doc = "The name for resource group where NICs will be placed."]
    #[serde(rename = "networkingResourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub networking_resource_group_name: Option<String>,
    #[doc = "Active Directory join type"]
    #[serde(rename = "domainJoinType")]
    pub domain_join_type: DomainJoinType,
}
impl NetworkProperties {
    pub fn new(domain_join_type: DomainJoinType) -> Self {
        Self {
            network_connection_update_properties: NetworkConnectionUpdateProperties::default(),
            provisioning_state: None,
            health_check_status: None,
            networking_resource_group_name: None,
            domain_join_type,
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
#[doc = "The current status of an async operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "Fully qualified ID for the operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The operation id name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Provisioning state of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The start time of the operation"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the operation"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Percent of the operation that is complete"]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "Custom operation properties, populated only for a successful operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Operation Error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<operation_status::Error>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_status {
    use super::*;
    #[doc = "Operation Error message"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "The error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "The error message."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A pool of Virtual Machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of a Pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolProperties>,
}
impl Pool {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Results of the machine pool list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Pool>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolProperties {
    #[serde(flatten)]
    pub pool_update_properties: PoolUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl PoolProperties {
    pub fn new() -> Self {
        Self {
            pool_update_properties: PoolUpdateProperties::default(),
            provisioning_state: None,
        }
    }
}
#[doc = "The pool properties for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Properties of a Pool. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolUpdateProperties>,
}
impl PoolUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Pool. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolUpdateProperties {
    #[doc = "Name of a Dev Box definition in parent Project of this Pool"]
    #[serde(rename = "devBoxDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub dev_box_definition_name: Option<String>,
    #[doc = "Name of a Network Connection in parent Project of this Pool"]
    #[serde(rename = "networkConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub network_connection_name: Option<String>,
    #[doc = "License Types"]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<LicenseType>,
    #[serde(rename = "localAdministrator", default, skip_serializing_if = "Option::is_none")]
    pub local_administrator: Option<LocalAdminStatus>,
}
impl PoolUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a project resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of a project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectProperties>,
}
impl Project {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Represents an environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEnvironmentType {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a project environment type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectEnvironmentTypeProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "The geo-location for the environment type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ProjectEnvironmentType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the project environment type list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEnvironmentTypeListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProjectEnvironmentType>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProjectEnvironmentTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProjectEnvironmentTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a project environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEnvironmentTypeProperties {
    #[serde(flatten)]
    pub project_environment_type_update_properties: ProjectEnvironmentTypeUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ProjectEnvironmentTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The project environment type for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEnvironmentTypeUpdate {
    #[doc = "Properties of a project environment type. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectEnvironmentTypeUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl ProjectEnvironmentTypeUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a project environment type. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEnvironmentTypeUpdateProperties {
    #[doc = "Id of a subscription that the environment type will be mapped to. The environment's resources will be deployed into this subscription."]
    #[serde(rename = "deploymentTargetId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_target_id: Option<String>,
    #[doc = "Enable or disable status. Indicates whether the property applied to is either enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EnableStatus>,
    #[doc = "The role definition assigned to the environment creator on backing resources."]
    #[serde(rename = "creatorRoleAssignment", default, skip_serializing_if = "Option::is_none")]
    pub creator_role_assignment: Option<project_environment_type_update_properties::CreatorRoleAssignment>,
    #[doc = "Role Assignments created on environment backing resources. This is a mapping from a user object ID to an object of role definition IDs."]
    #[serde(rename = "userRoleAssignments", default, skip_serializing_if = "Option::is_none")]
    pub user_role_assignments: Option<serde_json::Value>,
}
impl ProjectEnvironmentTypeUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod project_environment_type_update_properties {
    use super::*;
    #[doc = "The role definition assigned to the environment creator on backing resources."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct CreatorRoleAssignment {
        #[doc = "A map of roles to assign to the environment creator."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub roles: Option<serde_json::Value>,
    }
    impl CreatorRoleAssignment {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Results of the project list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Project>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProjectListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProjectListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectProperties {
    #[serde(flatten)]
    pub project_update_properties: ProjectUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The project properties for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Properties of a project. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectUpdateProperties>,
}
impl ProjectUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a project. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectUpdateProperties {
    #[doc = "Resource Id of an associated DevCenter"]
    #[serde(rename = "devCenterId", default, skip_serializing_if = "Option::is_none")]
    pub dev_center_id: Option<String>,
    #[doc = "Description of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ProjectUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ProvisioningState = String;
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
#[doc = "Properties for a recommended machine configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedMachineConfiguration {
    #[doc = "Properties for a range of values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<ResourceRange>,
    #[doc = "Properties for a range of values."]
    #[serde(rename = "vCPUs", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us: Option<ResourceRange>,
}
impl RecommendedMachineConfiguration {
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a range of values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRange {
    #[doc = "Minimum value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[doc = "Maximum value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
}
impl ResourceRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Schedule to execute a task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The Schedule properties defining when and what to execute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleProperties>,
}
impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the schedule list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Schedule>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Schedule properties defining when and what to execute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleProperties {
    #[serde(flatten)]
    pub schedule_update_properties: ScheduleUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ScheduleProperties {
    pub fn new() -> Self {
        Self {
            schedule_update_properties: ScheduleUpdateProperties::default(),
            provisioning_state: None,
        }
    }
}
#[doc = "The schedule properties for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Updatable properties of a Schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleUpdateProperties>,
}
impl ScheduleUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Updatable properties of a Schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleUpdateProperties {
    #[doc = "The supported types for a scheduled task."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ScheduledType>,
    #[doc = "The frequency of task execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<ScheduledFrequency>,
    #[doc = "The target time to trigger the action. The format is HH:MM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[doc = "The IANA timezone id at which the schedule should execute."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "Enable or disable status. Indicates whether the property applied to is either enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<EnableStatus>,
}
impl ScheduleUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The frequency of task execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledFrequency")]
pub enum ScheduledFrequency {
    Daily,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledFrequency {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledFrequency {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledFrequency {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Daily => serializer.serialize_unit_variant("ScheduledFrequency", 0u32, "Daily"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The supported types for a scheduled task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledType")]
pub enum ScheduledType {
    StopDevBox,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StopDevBox => serializer.serialize_unit_variant("ScheduledType", 0u32, "StopDevBox"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition representing SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU. Ex - P3. It is typically a letter+number code"]
    pub name: String,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SkuTier>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "Results of the Microsoft.DevCenter SKU list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuListResult {
    #[doc = "Current page of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DevCenterSku>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SkuListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SkuListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SkuTier {
    Free,
    Basic,
    Standard,
    Premium,
}
#[doc = "Resource tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
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
#[doc = "Base tracked resource type for PATCH updates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TrackedResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The core usage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "The current usage."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The limit integer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The unit details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
    #[doc = "The Usage Names."]
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
    #[doc = "The unit details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Usage Names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "The localized name of the resource."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Mapping of user object ID to role assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserRoleAssignment {
    #[doc = "A map of roles to assign to the parent user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<serde_json::Value>,
}
impl UserRoleAssignment {
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
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
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
