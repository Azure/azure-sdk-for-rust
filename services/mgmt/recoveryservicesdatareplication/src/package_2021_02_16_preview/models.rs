#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "AzStackHCI cluster properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzStackHciClusterProperties {
    #[doc = "Gets or sets the AzStackHCICluster FQDN name."]
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    #[doc = "Gets or sets the AzStackHCICluster resource name."]
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    #[doc = "Gets or sets the Storage account name."]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "Gets or sets the list of AzStackHCICluster Storage Container."]
    #[serde(rename = "storageContainers")]
    pub storage_containers: Vec<StorageContainerProperties>,
}
impl AzStackHciClusterProperties {
    pub fn new(
        cluster_name: String,
        resource_name: String,
        storage_account_name: String,
        storage_containers: Vec<StorageContainerProperties>,
    ) -> Self {
        Self {
            cluster_name,
            resource_name,
            storage_account_name,
            storage_containers,
        }
    }
}
#[doc = "AzStackHCI fabric model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzStackHciFabricModelCustomProperties {
    #[doc = "Gets or sets the ARM Id of the AzStackHCI site."]
    #[serde(rename = "azStackHciSiteId")]
    pub az_stack_hci_site_id: String,
    #[doc = "Gets or sets the Appliance name."]
    #[serde(
        rename = "applianceName",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_name: Vec<String>,
    #[doc = "AzStackHCI cluster properties."]
    pub cluster: AzStackHciClusterProperties,
    #[doc = "Gets or sets the fabric resource Id."]
    #[serde(rename = "fabricResourceId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_resource_id: Option<String>,
    #[doc = "Gets or sets the fabric container Id."]
    #[serde(rename = "fabricContainerId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_container_id: Option<String>,
    #[doc = "Gets or sets the Migration solution ARM Id."]
    #[serde(rename = "migrationSolutionId")]
    pub migration_solution_id: String,
    #[doc = "Gets or sets the migration hub Uri."]
    #[serde(rename = "migrationHubUri", default, skip_serializing_if = "Option::is_none")]
    pub migration_hub_uri: Option<String>,
}
impl AzStackHciFabricModelCustomProperties {
    pub fn new(az_stack_hci_site_id: String, cluster: AzStackHciClusterProperties, migration_solution_id: String) -> Self {
        Self {
            az_stack_hci_site_id,
            appliance_name: Vec::new(),
            cluster,
            fabric_resource_id: None,
            fabric_container_id: None,
            migration_solution_id,
            migration_hub_uri: None,
        }
    }
}
#[doc = "Check name availability model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityModel {
    #[doc = "Gets or sets the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Check name availability response model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponseModel {
    #[doc = "Gets or sets a value indicating whether resource name is available or not."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Gets or sets the reason for resource name unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Gets or sets the message for resource name unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponseModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment preflight model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentPreflightModel {
    #[doc = "Gets or sets the list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<DeploymentPreflightResource>,
}
impl DeploymentPreflightModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment preflight resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentPreflightResource {
    #[doc = "Gets or sets the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets the Api version."]
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}
impl DeploymentPreflightResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dra model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DraModel {
    #[doc = "Dra model properties."]
    pub properties: DraModelProperties,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl DraModel {
    pub fn new(properties: DraModelProperties) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Dra model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DraModelCollection {
    #[doc = "Gets or sets the list of Dras."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DraModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DraModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DraModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the instance type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "instanceType")]
pub enum DraModelCustomPropertiesUnion {
    VMware(VMwareDraModelCustomProperties),
}
#[doc = "Dra model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DraModelProperties {
    #[doc = "Gets or sets the Dra correlation Id."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Gets or sets the machine Id where Dra is running."]
    #[serde(rename = "machineId")]
    pub machine_id: String,
    #[doc = "Gets or sets the machine name where Dra is running."]
    #[serde(rename = "machineName")]
    pub machine_name: String,
    #[doc = "Identity model."]
    #[serde(rename = "authenticationIdentity")]
    pub authentication_identity: IdentityModel,
    #[doc = "Identity model."]
    #[serde(rename = "resourceAccessIdentity")]
    pub resource_access_identity: IdentityModel,
    #[doc = "Gets or sets a value indicating whether Dra is responsive."]
    #[serde(rename = "isResponsive", default, skip_serializing_if = "Option::is_none")]
    pub is_responsive: Option<bool>,
    #[doc = "Gets or sets the time when last heartbeat was sent by the Dra."]
    #[serde(rename = "lastHeartbeat", default, with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the Dra version."]
    #[serde(rename = "versionNumber", default, skip_serializing_if = "Option::is_none")]
    pub version_number: Option<String>,
    #[doc = "Gets or sets the provisioning state of the Dra."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<dra_model_properties::ProvisioningState>,
    #[doc = "Gets or sets the list of health errors."]
    #[serde(
        rename = "healthErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_errors: Vec<HealthErrorModel>,
    #[doc = "Dra model custom properties."]
    #[serde(rename = "customProperties")]
    pub custom_properties: DraModelCustomPropertiesUnion,
}
impl DraModelProperties {
    pub fn new(
        machine_id: String,
        machine_name: String,
        authentication_identity: IdentityModel,
        resource_access_identity: IdentityModel,
        custom_properties: DraModelCustomPropertiesUnion,
    ) -> Self {
        Self {
            correlation_id: None,
            machine_id,
            machine_name,
            authentication_identity,
            resource_access_identity,
            is_responsive: None,
            last_heartbeat: None,
            version_number: None,
            provisioning_state: None,
            health_errors: Vec::new(),
            custom_properties,
        }
    }
}
pub mod dra_model_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the Dra."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Canceled,
        Creating,
        Deleting,
        Deleted,
        Failed,
        Succeeded,
        Updating,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Canceled"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleted"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Email configuration model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailConfigurationModel {
    #[doc = "Email configuration model properties."]
    pub properties: EmailConfigurationModelProperties,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl EmailConfigurationModel {
    pub fn new(properties: EmailConfigurationModelProperties) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Email configuration model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailConfigurationModelCollection {
    #[doc = "Gets or sets the list of email configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EmailConfigurationModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EmailConfigurationModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EmailConfigurationModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Email configuration model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailConfigurationModelProperties {
    #[doc = "Gets or sets a value indicating whether to send email to subscription administrator."]
    #[serde(rename = "sendToOwners")]
    pub send_to_owners: bool,
    #[doc = "Gets or sets the custom email address for sending emails."]
    #[serde(
        rename = "customEmailAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_email_addresses: Vec<String>,
    #[doc = "Gets or sets the locale for the email notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}
impl EmailConfigurationModelProperties {
    pub fn new(send_to_owners: bool) -> Self {
        Self {
            send_to_owners,
            custom_email_addresses: Vec::new(),
            locale: None,
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
#[doc = "Error model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorModel {
    #[doc = "Gets or sets the error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets or sets the error type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the error severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Gets or sets the creation time of error."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets or sets the possible causes of error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub causes: Option<String>,
    #[doc = "Gets or sets the recommended action to resolve error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
}
impl ErrorModel {
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
#[doc = "Event model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventModel {
    #[doc = "Event model properties."]
    pub properties: EventModelProperties,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl EventModel {
    pub fn new(properties: EventModelProperties) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Event model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventModelCollection {
    #[doc = "Gets or sets the list of events."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EventModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EventModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the instance type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "instanceType")]
pub enum EventModelCustomPropertiesUnion {
    #[serde(rename = "HyperVToAzStackHCI")]
    HyperVToAzStackHci(HyperVToAzStackHciEventModelCustomProperties),
}
#[doc = "Event model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventModelProperties {
    #[doc = "Gets or sets the resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Gets or sets the resource name."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Gets or sets the event type."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "Gets or sets the event name."]
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[doc = "Gets or sets the time at which the event occurred at source."]
    #[serde(rename = "timeOfOccurrence", default, with = "azure_core::date::rfc3339::option")]
    pub time_of_occurrence: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the event severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Gets or sets the event description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the event correlation Id."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Gets or sets the errors associated with this event."]
    #[serde(
        rename = "healthErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_errors: Vec<HealthErrorModel>,
    #[doc = "Event model custom properties."]
    #[serde(rename = "customProperties")]
    pub custom_properties: EventModelCustomPropertiesUnion,
}
impl EventModelProperties {
    pub fn new(custom_properties: EventModelCustomPropertiesUnion) -> Self {
        Self {
            resource_type: None,
            resource_name: None,
            event_type: None,
            event_name: None,
            time_of_occurrence: None,
            severity: None,
            description: None,
            correlation_id: None,
            health_errors: Vec::new(),
            custom_properties,
        }
    }
}
#[doc = "Fabric model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricModel {
    #[doc = "Gets or sets the location of the fabric."]
    pub location: String,
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Fabric model properties."]
    pub properties: FabricModelProperties,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl FabricModel {
    pub fn new(location: String, properties: FabricModelProperties) -> Self {
        Self {
            location,
            tags: None,
            properties,
            id: None,
            name: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Fabric model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricModelCollection {
    #[doc = "Gets or sets the list of fabrics."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FabricModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FabricModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FabricModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the instance type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "instanceType")]
pub enum FabricModelCustomPropertiesUnion {
    #[serde(rename = "AzStackHCI")]
    AzStackHci(AzStackHciFabricModelCustomProperties),
    HyperVMigrate(HyperVMigrateFabricModelCustomProperties),
    VMwareMigrate(VMwareMigrateFabricModelCustomProperties),
}
#[doc = "Fabric model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricModelProperties {
    #[doc = "Gets or sets the provisioning state of the fabric."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<fabric_model_properties::ProvisioningState>,
    #[doc = "Gets or sets the service endpoint."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "Gets or sets the service resource Id."]
    #[serde(rename = "serviceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub service_resource_id: Option<String>,
    #[doc = "Gets or sets the fabric health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<fabric_model_properties::Health>,
    #[doc = "Gets or sets the list of health errors."]
    #[serde(
        rename = "healthErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_errors: Vec<HealthErrorModel>,
    #[doc = "Fabric model custom properties."]
    #[serde(rename = "customProperties")]
    pub custom_properties: FabricModelCustomPropertiesUnion,
}
impl FabricModelProperties {
    pub fn new(custom_properties: FabricModelCustomPropertiesUnion) -> Self {
        Self {
            provisioning_state: None,
            service_endpoint: None,
            service_resource_id: None,
            health: None,
            health_errors: Vec::new(),
            custom_properties,
        }
    }
}
pub mod fabric_model_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the fabric."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Canceled,
        Creating,
        Deleting,
        Deleted,
        Failed,
        Succeeded,
        Updating,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Canceled"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleted"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the fabric health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Normal => serializer.serialize_unit_variant("Health", 0u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("Health", 1u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Health", 2u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Fabric model for update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricModelUpdate {
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Fabric model properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FabricModelProperties>,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl FabricModelUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Failover properties of the protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverProtectedItemProperties {
    #[doc = "Gets or sets the protected item name."]
    #[serde(rename = "protectedItemName", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_name: Option<String>,
    #[doc = "Gets or sets the VM name."]
    #[serde(rename = "vmName", default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[doc = "Gets or sets the test VM name."]
    #[serde(rename = "testVmName", default, skip_serializing_if = "Option::is_none")]
    pub test_vm_name: Option<String>,
    #[doc = "Gets or sets the recovery point Id."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[doc = "Gets or sets the recovery point time."]
    #[serde(rename = "recoveryPointTime", default, with = "azure_core::date::rfc3339::option")]
    pub recovery_point_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the network name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Gets or sets the network subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
}
impl FailoverProtectedItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Failover workflow model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverWorkflowModelCustomProperties {
    #[serde(flatten)]
    pub workflow_model_custom_properties: WorkflowModelCustomProperties,
    #[doc = "Gets or sets the failed over protected item details."]
    #[serde(
        rename = "protectedItemDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protected_item_details: Vec<FailoverProtectedItemProperties>,
}
impl FailoverWorkflowModelCustomProperties {
    pub fn new(workflow_model_custom_properties: WorkflowModelCustomProperties) -> Self {
        Self {
            workflow_model_custom_properties,
            protected_item_details: Vec::new(),
        }
    }
}
#[doc = "Health error model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthErrorModel {
    #[doc = "Gets or sets the type of affected resource type."]
    #[serde(rename = "affectedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub affected_resource_type: Option<String>,
    #[doc = "Gets or sets the list of affected resource correlation Ids. This can be used to\r\nuniquely identify the count of items affected by a specific category and severity\r\nas well as count of item affected by an specific issue."]
    #[serde(
        rename = "affectedResourceCorrelationIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub affected_resource_correlation_ids: Vec<String>,
    #[doc = "Gets or sets a list of child health errors associated with this error."]
    #[serde(
        rename = "childErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub child_errors: Vec<InnerHealthErrorModel>,
    #[doc = "Gets or sets the error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets or sets the health category."]
    #[serde(rename = "healthCategory", default, skip_serializing_if = "Option::is_none")]
    pub health_category: Option<String>,
    #[doc = "Gets or sets the error category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Gets or sets the error severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Gets or sets the error source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Gets or sets the error creation time."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets a value indicating whether the error is customer resolvable."]
    #[serde(rename = "isCustomerResolvable", default, skip_serializing_if = "Option::is_none")]
    pub is_customer_resolvable: Option<bool>,
    #[doc = "Gets or sets the error summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Gets or sets the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets or sets possible causes of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub causes: Option<String>,
    #[doc = "Gets or sets recommended action to resolve the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
}
impl HealthErrorModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HyperV migrate fabric model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVMigrateFabricModelCustomProperties {
    #[doc = "Gets or sets the ARM Id of the HyperV site."]
    #[serde(rename = "hyperVSiteId")]
    pub hyper_v_site_id: String,
    #[doc = "Gets or sets the fabric resource Id."]
    #[serde(rename = "fabricResourceId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_resource_id: Option<String>,
    #[doc = "Gets or sets the fabric container Id."]
    #[serde(rename = "fabricContainerId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_container_id: Option<String>,
    #[doc = "Gets or sets the migration solution ARM Id."]
    #[serde(rename = "migrationSolutionId")]
    pub migration_solution_id: String,
    #[doc = "Gets or sets the migration hub Uri."]
    #[serde(rename = "migrationHubUri", default, skip_serializing_if = "Option::is_none")]
    pub migration_hub_uri: Option<String>,
}
impl HyperVMigrateFabricModelCustomProperties {
    pub fn new(hyper_v_site_id: String, migration_solution_id: String) -> Self {
        Self {
            hyper_v_site_id,
            fabric_resource_id: None,
            fabric_container_id: None,
            migration_solution_id,
            migration_hub_uri: None,
        }
    }
}
#[doc = "HyperVToAzStack disk input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVToAzStackHciDiskInput {
    #[doc = "Gets or sets the disk Id."]
    #[serde(rename = "diskId")]
    pub disk_id: String,
    #[doc = "Gets or sets the target storage account ARM Id."]
    #[serde(rename = "storageContainerId", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_id: Option<String>,
    #[doc = "Gets or sets a value indicating whether dynamic sizing is enabled on the virtual hard\r\ndisk."]
    #[serde(rename = "isDynamic", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic: Option<bool>,
    #[doc = "Gets or sets the disk size in GB."]
    #[serde(rename = "diskSizeGB")]
    pub disk_size_gb: i64,
    #[doc = "Gets or sets the type of the virtual hard disk, vhd or vhdx."]
    #[serde(rename = "diskFileFormat")]
    pub disk_file_format: String,
    #[doc = "Gets or sets a value indicating whether disk is os disk."]
    #[serde(rename = "isOsDisk")]
    pub is_os_disk: bool,
}
impl HyperVToAzStackHciDiskInput {
    pub fn new(disk_id: String, disk_size_gb: i64, disk_file_format: String, is_os_disk: bool) -> Self {
        Self {
            disk_id,
            storage_container_id: None,
            is_dynamic: None,
            disk_size_gb,
            disk_file_format,
            is_os_disk,
        }
    }
}
#[doc = "HyperV to  AzStackHCI event model custom properties. This class provides provider specific\r\ndetails for events of type DataContract.HealthEvents.HealthEventType.ProtectedItemHealth and\r\nDataContract.HealthEvents.HealthEventType.AgentHealth."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVToAzStackHciEventModelCustomProperties {
    #[doc = "Gets or sets the friendly name of the source which has raised this health event."]
    #[serde(rename = "eventSourceFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub event_source_friendly_name: Option<String>,
    #[doc = "Gets or sets the protected item friendly name."]
    #[serde(rename = "protectedItemFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_friendly_name: Option<String>,
    #[doc = "Gets or sets the source appliance name."]
    #[serde(rename = "sourceApplianceName", default, skip_serializing_if = "Option::is_none")]
    pub source_appliance_name: Option<String>,
    #[doc = "Gets or sets the source target name."]
    #[serde(rename = "targetApplianceName", default, skip_serializing_if = "Option::is_none")]
    pub target_appliance_name: Option<String>,
    #[doc = "Gets or sets the server type."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
}
impl HyperVToAzStackHciEventModelCustomProperties {
    pub fn new() -> Self {
        Self {
            event_source_friendly_name: None,
            protected_item_friendly_name: None,
            source_appliance_name: None,
            target_appliance_name: None,
            server_type: None,
        }
    }
}
#[doc = "HyperVToAzStackHCI NIC properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVToAzStackHciNicInput {
    #[doc = "Gets or sets the NIC Id."]
    #[serde(rename = "nicId")]
    pub nic_id: String,
    #[doc = "Gets or sets the network name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Gets or sets the target network Id within AzStackHCI Cluster."]
    #[serde(rename = "targetNetworkId")]
    pub target_network_id: String,
    #[doc = "Gets or sets the target test network Id within AzStackHCI Cluster."]
    #[serde(rename = "testNetworkId")]
    pub test_network_id: String,
    #[doc = "Gets or sets the selection type of the NIC."]
    #[serde(rename = "selectionTypeForFailover")]
    pub selection_type_for_failover: hyper_v_to_az_stack_hci_nic_input::SelectionTypeForFailover,
}
impl HyperVToAzStackHciNicInput {
    pub fn new(
        nic_id: String,
        target_network_id: String,
        test_network_id: String,
        selection_type_for_failover: hyper_v_to_az_stack_hci_nic_input::SelectionTypeForFailover,
    ) -> Self {
        Self {
            nic_id,
            network_name: None,
            target_network_id,
            test_network_id,
            selection_type_for_failover,
        }
    }
}
pub mod hyper_v_to_az_stack_hci_nic_input {
    use super::*;
    #[doc = "Gets or sets the selection type of the NIC."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SelectionTypeForFailover")]
    pub enum SelectionTypeForFailover {
        NotSelected,
        SelectedByUser,
        SelectedByDefault,
        SelectedByUserOverride,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SelectionTypeForFailover {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SelectionTypeForFailover {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SelectionTypeForFailover {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSelected => serializer.serialize_unit_variant("SelectionTypeForFailover", 0u32, "NotSelected"),
                Self::SelectedByUser => serializer.serialize_unit_variant("SelectionTypeForFailover", 1u32, "SelectedByUser"),
                Self::SelectedByDefault => serializer.serialize_unit_variant("SelectionTypeForFailover", 2u32, "SelectedByDefault"),
                Self::SelectedByUserOverride => {
                    serializer.serialize_unit_variant("SelectionTypeForFailover", 3u32, "SelectedByUserOverride")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "HyperV to AzStackHCI planned failover model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVToAzStackHciPlannedFailoverModelCustomProperties {
    #[doc = "Gets or sets a value indicating whether VM needs to be shut down."]
    #[serde(rename = "shutdownSourceVM")]
    pub shutdown_source_vm: bool,
}
impl HyperVToAzStackHciPlannedFailoverModelCustomProperties {
    pub fn new(shutdown_source_vm: bool) -> Self {
        Self { shutdown_source_vm }
    }
}
#[doc = "HyperV To AzStackHCI Policy model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVToAzStackHciPolicyModelCustomProperties {
    #[doc = "Gets or sets the duration in minutes until which the recovery points need to be\r\nstored."]
    #[serde(rename = "recoveryPointHistoryInMinutes")]
    pub recovery_point_history_in_minutes: i32,
    #[doc = "Gets or sets the crash consistent snapshot frequency (in minutes)."]
    #[serde(rename = "crashConsistentFrequencyInMinutes")]
    pub crash_consistent_frequency_in_minutes: i32,
    #[doc = "Gets or sets the app consistent snapshot frequency (in minutes)."]
    #[serde(rename = "appConsistentFrequencyInMinutes")]
    pub app_consistent_frequency_in_minutes: i32,
}
impl HyperVToAzStackHciPolicyModelCustomProperties {
    pub fn new(
        recovery_point_history_in_minutes: i32,
        crash_consistent_frequency_in_minutes: i32,
        app_consistent_frequency_in_minutes: i32,
    ) -> Self {
        Self {
            recovery_point_history_in_minutes,
            crash_consistent_frequency_in_minutes,
            app_consistent_frequency_in_minutes,
        }
    }
}
#[doc = "HyperVToAzStackHCI protected disk properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVToAzStackHciProtectedDiskProperties {
    #[doc = "Gets or sets the ARM Id of the storage container."]
    #[serde(rename = "storageContainerId", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_id: Option<String>,
    #[doc = "Gets or sets the local path of the storage container."]
    #[serde(rename = "storageContainerLocalPath", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_local_path: Option<String>,
    #[doc = "Gets or sets the source disk Id."]
    #[serde(rename = "sourceDiskId", default, skip_serializing_if = "Option::is_none")]
    pub source_disk_id: Option<String>,
    #[doc = "Gets or sets the source disk Name."]
    #[serde(rename = "sourceDiskName", default, skip_serializing_if = "Option::is_none")]
    pub source_disk_name: Option<String>,
    #[doc = "Gets or sets the seed disk name."]
    #[serde(rename = "seedDiskName", default, skip_serializing_if = "Option::is_none")]
    pub seed_disk_name: Option<String>,
    #[doc = "Gets or sets the test failover clone disk."]
    #[serde(rename = "testMigrateDiskName", default, skip_serializing_if = "Option::is_none")]
    pub test_migrate_disk_name: Option<String>,
    #[doc = "Gets or sets the failover clone disk."]
    #[serde(rename = "migrateDiskName", default, skip_serializing_if = "Option::is_none")]
    pub migrate_disk_name: Option<String>,
    #[doc = "Gets or sets a value indicating whether the disk is the OS disk."]
    #[serde(rename = "isOsDisk", default, skip_serializing_if = "Option::is_none")]
    pub is_os_disk: Option<bool>,
    #[doc = "Gets or sets the disk capacity in bytes."]
    #[serde(rename = "capacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub capacity_in_bytes: Option<i64>,
    #[doc = "Gets or sets a value indicating whether dynamic sizing is enabled on the virtual hard\r\ndisk."]
    #[serde(rename = "isDynamic", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic: Option<bool>,
    #[doc = "Gets or sets the disk type."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
}
impl HyperVToAzStackHciProtectedDiskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HyperV to AzStackHCI Protected item model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVToAzStackHciProtectedItemModelCustomProperties {
    #[doc = "Gets or sets the location of the protected item."]
    #[serde(rename = "activeLocation", default, skip_serializing_if = "Option::is_none")]
    pub active_location: Option<hyper_v_to_az_stack_hci_protected_item_model_custom_properties::ActiveLocation>,
    #[doc = "Gets or sets the Target HCI Cluster ARM Id."]
    #[serde(rename = "targetHciClusterId")]
    pub target_hci_cluster_id: String,
    #[doc = "Gets or sets the Target Arc Cluster Custom Location ARM Id."]
    #[serde(rename = "targetArcClusterCustomLocationId")]
    pub target_arc_cluster_custom_location_id: String,
    #[doc = "Gets or sets the Target AzStackHCI cluster name."]
    #[serde(rename = "targetAzStackHciClusterName", default, skip_serializing_if = "Option::is_none")]
    pub target_az_stack_hci_cluster_name: Option<String>,
    #[doc = "Gets or sets the ARM Id of the discovered machine."]
    #[serde(rename = "fabricDiscoveryMachineId")]
    pub fabric_discovery_machine_id: String,
    #[doc = "Gets or sets the list of disks to replicate."]
    #[serde(rename = "disksToInclude")]
    pub disks_to_include: Vec<HyperVToAzStackHciDiskInput>,
    #[doc = "Gets or sets the list of VM NIC to replicate."]
    #[serde(rename = "nicsToInclude")]
    pub nics_to_include: Vec<HyperVToAzStackHciNicInput>,
    #[doc = "Gets or sets the source VM display name."]
    #[serde(rename = "sourceVmName", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_name: Option<String>,
    #[doc = "Gets or sets the source VM CPU cores."]
    #[serde(rename = "sourceCpuCores", default, skip_serializing_if = "Option::is_none")]
    pub source_cpu_cores: Option<i32>,
    #[doc = "Gets or sets the source VM ram memory size in megabytes."]
    #[serde(rename = "sourceMemoryInMegaBytes", default, skip_serializing_if = "Option::is_none")]
    pub source_memory_in_mega_bytes: Option<f64>,
    #[doc = "Gets or sets the target VM display name."]
    #[serde(rename = "targetVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_name: Option<String>,
    #[doc = "Gets or sets the target resource group ARM Id."]
    #[serde(rename = "targetResourceGroupId")]
    pub target_resource_group_id: String,
    #[doc = "Gets or sets the target storage container ARM Id."]
    #[serde(rename = "storageContainerId")]
    pub storage_container_id: String,
    #[doc = "Gets or sets the hypervisor generation of the virtual machine."]
    #[serde(rename = "hyperVGeneration")]
    pub hyper_v_generation: String,
    #[doc = "Gets or sets the target network Id within AzStackHCI Cluster."]
    #[serde(rename = "targetNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_network_id: Option<String>,
    #[doc = "Gets or sets the target test network Id within AzStackHCI Cluster."]
    #[serde(rename = "testNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub test_network_id: Option<String>,
    #[doc = "Gets or sets the target CPU cores."]
    #[serde(rename = "targetCpuCores", default, skip_serializing_if = "Option::is_none")]
    pub target_cpu_cores: Option<i32>,
    #[doc = "Gets or sets a value indicating whether memory is dynamical."]
    #[serde(rename = "isDynamicRam", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic_ram: Option<bool>,
    #[doc = "Protected item dynamic memory config."]
    #[serde(rename = "dynamicMemoryConfig", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_config: Option<ProtectedItemDynamicMemoryConfig>,
    #[doc = "Gets or sets the target memory in mega-bytes."]
    #[serde(rename = "targetMemoryInMegaBytes", default, skip_serializing_if = "Option::is_none")]
    pub target_memory_in_mega_bytes: Option<i32>,
    #[doc = "Gets or sets the Run As account Id."]
    #[serde(rename = "runAsAccountId")]
    pub run_as_account_id: String,
    #[doc = "Gets or sets the source DRA name."]
    #[serde(rename = "sourceDraName")]
    pub source_dra_name: String,
    #[doc = "Gets or sets the target DRA name."]
    #[serde(rename = "targetDraName")]
    pub target_dra_name: String,
    #[doc = "Gets or sets the source appliance name."]
    #[serde(rename = "sourceApplianceName", default, skip_serializing_if = "Option::is_none")]
    pub source_appliance_name: Option<String>,
    #[doc = "Gets or sets the target appliance name."]
    #[serde(rename = "targetApplianceName", default, skip_serializing_if = "Option::is_none")]
    pub target_appliance_name: Option<String>,
    #[doc = "Gets or sets the type of the OS."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Gets or sets the name of the OS."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets the firmware type."]
    #[serde(rename = "firmwareType", default, skip_serializing_if = "Option::is_none")]
    pub firmware_type: Option<String>,
    #[doc = "Gets or sets the target location."]
    #[serde(rename = "targetLocation", default, skip_serializing_if = "Option::is_none")]
    pub target_location: Option<String>,
    #[doc = "Gets or sets the location of Azure Arc HCI custom location resource."]
    #[serde(rename = "customLocationRegion")]
    pub custom_location_region: String,
    #[doc = "Gets or sets the recovery point Id to which the VM was failed over."]
    #[serde(rename = "failoverRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub failover_recovery_point_id: Option<String>,
    #[doc = "Gets or sets the last recovery point received time."]
    #[serde(rename = "lastRecoveryPointReceived", default, with = "azure_core::date::rfc3339::option")]
    pub last_recovery_point_received: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last recovery point Id."]
    #[serde(rename = "lastRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub last_recovery_point_id: Option<String>,
    #[doc = "Gets or sets the initial replication progress percentage. This is calculated based on\r\ntotal bytes processed for all disks in the source VM."]
    #[serde(rename = "initialReplicationProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_progress_percentage: Option<i32>,
    #[doc = "Gets or sets the resync progress percentage. This is calculated based on total bytes\r\nprocessed for all disks in the source VM."]
    #[serde(rename = "resyncProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub resync_progress_percentage: Option<i32>,
    #[doc = "Gets or sets the list of protected disks."]
    #[serde(
        rename = "protectedDisks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protected_disks: Vec<HyperVToAzStackHciProtectedDiskProperties>,
    #[doc = "Gets or sets the VM NIC details."]
    #[serde(
        rename = "protectedNics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protected_nics: Vec<HyperVToAzStackHciProtectedNicProperties>,
    #[doc = "Gets or sets the BIOS Id of the target AzStackHCI VM."]
    #[serde(rename = "targetVmBiosId", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_bios_id: Option<String>,
    #[doc = "Gets or sets the latest timestamp that replication status is updated."]
    #[serde(rename = "lastReplicationUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_replication_update_time: Option<time::OffsetDateTime>,
}
impl HyperVToAzStackHciProtectedItemModelCustomProperties {
    pub fn new(
        target_hci_cluster_id: String,
        target_arc_cluster_custom_location_id: String,
        fabric_discovery_machine_id: String,
        disks_to_include: Vec<HyperVToAzStackHciDiskInput>,
        nics_to_include: Vec<HyperVToAzStackHciNicInput>,
        target_resource_group_id: String,
        storage_container_id: String,
        hyper_v_generation: String,
        run_as_account_id: String,
        source_dra_name: String,
        target_dra_name: String,
        custom_location_region: String,
    ) -> Self {
        Self {
            active_location: None,
            target_hci_cluster_id,
            target_arc_cluster_custom_location_id,
            target_az_stack_hci_cluster_name: None,
            fabric_discovery_machine_id,
            disks_to_include,
            nics_to_include,
            source_vm_name: None,
            source_cpu_cores: None,
            source_memory_in_mega_bytes: None,
            target_vm_name: None,
            target_resource_group_id,
            storage_container_id,
            hyper_v_generation,
            target_network_id: None,
            test_network_id: None,
            target_cpu_cores: None,
            is_dynamic_ram: None,
            dynamic_memory_config: None,
            target_memory_in_mega_bytes: None,
            run_as_account_id,
            source_dra_name,
            target_dra_name,
            source_appliance_name: None,
            target_appliance_name: None,
            os_type: None,
            os_name: None,
            firmware_type: None,
            target_location: None,
            custom_location_region,
            failover_recovery_point_id: None,
            last_recovery_point_received: None,
            last_recovery_point_id: None,
            initial_replication_progress_percentage: None,
            resync_progress_percentage: None,
            protected_disks: Vec::new(),
            protected_nics: Vec::new(),
            target_vm_bios_id: None,
            last_replication_update_time: None,
        }
    }
}
pub mod hyper_v_to_az_stack_hci_protected_item_model_custom_properties {
    use super::*;
    #[doc = "Gets or sets the location of the protected item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActiveLocation")]
    pub enum ActiveLocation {
        Primary,
        Recovery,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActiveLocation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActiveLocation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActiveLocation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("ActiveLocation", 0u32, "Primary"),
                Self::Recovery => serializer.serialize_unit_variant("ActiveLocation", 1u32, "Recovery"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "HyperVToAzStackHCI NIC properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVToAzStackHciProtectedNicProperties {
    #[doc = "Gets or sets the NIC Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "Gets or sets the NIC mac address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets or sets the network name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Gets or sets the target network Id within AzStackHCI Cluster."]
    #[serde(rename = "targetNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_network_id: Option<String>,
    #[doc = "Gets or sets the target test network Id within AzStackHCI Cluster."]
    #[serde(rename = "testNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub test_network_id: Option<String>,
    #[doc = "Gets or sets the selection type of the NIC."]
    #[serde(rename = "selectionTypeForFailover", default, skip_serializing_if = "Option::is_none")]
    pub selection_type_for_failover: Option<hyper_v_to_az_stack_hci_protected_nic_properties::SelectionTypeForFailover>,
}
impl HyperVToAzStackHciProtectedNicProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hyper_v_to_az_stack_hci_protected_nic_properties {
    use super::*;
    #[doc = "Gets or sets the selection type of the NIC."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SelectionTypeForFailover")]
    pub enum SelectionTypeForFailover {
        NotSelected,
        SelectedByUser,
        SelectedByDefault,
        SelectedByUserOverride,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SelectionTypeForFailover {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SelectionTypeForFailover {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SelectionTypeForFailover {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSelected => serializer.serialize_unit_variant("SelectionTypeForFailover", 0u32, "NotSelected"),
                Self::SelectedByUser => serializer.serialize_unit_variant("SelectionTypeForFailover", 1u32, "SelectedByUser"),
                Self::SelectedByDefault => serializer.serialize_unit_variant("SelectionTypeForFailover", 2u32, "SelectedByDefault"),
                Self::SelectedByUserOverride => {
                    serializer.serialize_unit_variant("SelectionTypeForFailover", 3u32, "SelectedByUserOverride")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "HyperV to AzStackHCI recovery point model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVToAzStackHciRecoveryPointModelCustomProperties {
    #[doc = "Gets or sets the list of the disk Ids."]
    #[serde(
        rename = "diskIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disk_ids: Vec<String>,
}
impl HyperVToAzStackHciRecoveryPointModelCustomProperties {
    pub fn new() -> Self {
        Self { disk_ids: Vec::new() }
    }
}
#[doc = "HyperV to AzStackHCI Replication extension model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVToAzStackHciReplicationExtensionModelCustomProperties {
    #[doc = "Gets or sets the ARM Id of the source HyperV fabric."]
    #[serde(rename = "hyperVFabricArmId")]
    pub hyper_v_fabric_arm_id: String,
    #[doc = "Gets or sets the ARM Id of the HyperV site."]
    #[serde(rename = "hyperVSiteId", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_site_id: Option<String>,
    #[doc = "Gets or sets the ARM Id of the target AzStackHCI fabric."]
    #[serde(rename = "azStackHciFabricArmId")]
    pub az_stack_hci_fabric_arm_id: String,
    #[doc = "Gets or sets the ARM Id of the AzStackHCI site."]
    #[serde(rename = "azStackHciSiteId", default, skip_serializing_if = "Option::is_none")]
    pub az_stack_hci_site_id: Option<String>,
    #[doc = "Gets or sets the storage account Id."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "Gets or sets the Sas Secret of storage account."]
    #[serde(rename = "storageAccountSasSecretName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_sas_secret_name: Option<String>,
    #[doc = "Gets or sets the Uri of ASR."]
    #[serde(rename = "asrServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub asr_service_uri: Option<String>,
    #[doc = "Gets or sets the Uri of Rcm."]
    #[serde(rename = "rcmServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub rcm_service_uri: Option<String>,
    #[doc = "Gets or sets the Uri of Gateway."]
    #[serde(rename = "gatewayServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub gateway_service_uri: Option<String>,
    #[doc = "Gets or sets the gateway service Id of source."]
    #[serde(rename = "sourceGatewayServiceId", default, skip_serializing_if = "Option::is_none")]
    pub source_gateway_service_id: Option<String>,
    #[doc = "Gets or sets the gateway service Id of target."]
    #[serde(rename = "targetGatewayServiceId", default, skip_serializing_if = "Option::is_none")]
    pub target_gateway_service_id: Option<String>,
    #[doc = "Gets or sets the source storage container name."]
    #[serde(rename = "sourceStorageContainerName", default, skip_serializing_if = "Option::is_none")]
    pub source_storage_container_name: Option<String>,
    #[doc = "Gets or sets the target storage container name."]
    #[serde(rename = "targetStorageContainerName", default, skip_serializing_if = "Option::is_none")]
    pub target_storage_container_name: Option<String>,
    #[doc = "Gets or sets the resource location."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Gets or sets the subscription."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Gets or sets the resource group."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl HyperVToAzStackHciReplicationExtensionModelCustomProperties {
    pub fn new(hyper_v_fabric_arm_id: String, az_stack_hci_fabric_arm_id: String) -> Self {
        Self {
            hyper_v_fabric_arm_id,
            hyper_v_site_id: None,
            az_stack_hci_fabric_arm_id,
            az_stack_hci_site_id: None,
            storage_account_id: None,
            storage_account_sas_secret_name: None,
            asr_service_uri: None,
            rcm_service_uri: None,
            gateway_service_uri: None,
            source_gateway_service_id: None,
            target_gateway_service_id: None,
            source_storage_container_name: None,
            target_storage_container_name: None,
            resource_location: None,
            subscription_id: None,
            resource_group: None,
        }
    }
}
#[doc = "Identity model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityModel {
    #[doc = "Gets or sets the tenant Id of the SPN with which Dra communicates to service."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "Gets or sets the client/application Id of the SPN with which Dra communicates to\r\nservice."]
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[doc = "Gets or sets the object Id of the SPN with which Dra communicates to service."]
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[doc = "Gets or sets the audience of the SPN with which Dra communicates to service."]
    pub audience: String,
    #[doc = "Gets or sets the authority of the SPN with which Dra communicates to service."]
    #[serde(rename = "aadAuthority")]
    pub aad_authority: String,
}
impl IdentityModel {
    pub fn new(tenant_id: String, application_id: String, object_id: String, audience: String, aad_authority: String) -> Self {
        Self {
            tenant_id,
            application_id,
            object_id,
            audience,
            aad_authority,
        }
    }
}
#[doc = "Inner health error model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerHealthErrorModel {
    #[doc = "Gets or sets the error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets or sets the health category."]
    #[serde(rename = "healthCategory", default, skip_serializing_if = "Option::is_none")]
    pub health_category: Option<String>,
    #[doc = "Gets or sets the error category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Gets or sets the error severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Gets or sets the error source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Gets or sets the error creation time."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets a value indicating whether the error is customer resolvable."]
    #[serde(rename = "isCustomerResolvable", default, skip_serializing_if = "Option::is_none")]
    pub is_customer_resolvable: Option<bool>,
    #[doc = "Gets or sets the error summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Gets or sets the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets or sets possible causes of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub causes: Option<String>,
    #[doc = "Gets or sets recommended action to resolve the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
}
impl InnerHealthErrorModel {
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
#[doc = "Operation model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationModel {
    #[doc = "Gets or sets the name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets a value indicating whether the action is specific to data plane or\r\ncontrol plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Gets or sets the executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Operation model properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationModelProperties>,
}
impl OperationModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operations of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationModelCollection {
    #[doc = "Gets or sets the list of operations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationModelProperties {
    #[doc = "Gets or sets the resource provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Gets or sets resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Gets or sets the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the operation status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "Gets or sets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the status of the operation. ARM expects the terminal status to be one of\r\nSucceeded/ Failed/ Canceled. All other values imply that the operation is still running."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Gets or sets the end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Planned failover model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannedFailoverModel {
    #[doc = "Planned failover model properties."]
    pub properties: PlannedFailoverModelProperties,
}
impl PlannedFailoverModel {
    pub fn new(properties: PlannedFailoverModelProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Gets or sets the instance type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "instanceType")]
pub enum PlannedFailoverModelCustomPropertiesUnion {
    #[serde(rename = "HyperVToAzStackHCI")]
    HyperVToAzStackHci(HyperVToAzStackHciPlannedFailoverModelCustomProperties),
    #[serde(rename = "VMwareToAzStackHCI")]
    VMwareToAzStackHci(VMwareToAzStackHciPlannedFailoverModelCustomProperties),
}
#[doc = "Planned failover model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannedFailoverModelProperties {
    #[doc = "Planned failover model custom properties."]
    #[serde(rename = "customProperties")]
    pub custom_properties: PlannedFailoverModelCustomPropertiesUnion,
}
impl PlannedFailoverModelProperties {
    pub fn new(custom_properties: PlannedFailoverModelCustomPropertiesUnion) -> Self {
        Self { custom_properties }
    }
}
#[doc = "Policy model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyModel {
    #[doc = "Policy model properties."]
    pub properties: PolicyModelProperties,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl PolicyModel {
    pub fn new(properties: PolicyModelProperties) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Policy model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyModelCollection {
    #[doc = "Gets or sets the list of policies."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PolicyModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the instance type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "instanceType")]
pub enum PolicyModelCustomPropertiesUnion {
    #[serde(rename = "HyperVToAzStackHCI")]
    HyperVToAzStackHci(HyperVToAzStackHciPolicyModelCustomProperties),
    #[serde(rename = "VMwareToAzStackHCI")]
    VMwareToAzStackHci(VMwareToAzStackHciPolicyModelCustomProperties),
}
#[doc = "Policy model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyModelProperties {
    #[doc = "Gets or sets the provisioning state of the policy."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<policy_model_properties::ProvisioningState>,
    #[doc = "Policy model custom properties."]
    #[serde(rename = "customProperties")]
    pub custom_properties: PolicyModelCustomPropertiesUnion,
}
impl PolicyModelProperties {
    pub fn new(custom_properties: PolicyModelCustomPropertiesUnion) -> Self {
        Self {
            provisioning_state: None,
            custom_properties,
        }
    }
}
pub mod policy_model_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Canceled,
        Creating,
        Deleting,
        Deleted,
        Failed,
        Succeeded,
        Updating,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Canceled"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleted"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Protected item dynamic memory config."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectedItemDynamicMemoryConfig {
    #[doc = "Gets or sets maximum memory in MB."]
    #[serde(rename = "maximumMemoryInMegaBytes")]
    pub maximum_memory_in_mega_bytes: i64,
    #[doc = "Gets or sets minimum memory in MB."]
    #[serde(rename = "minimumMemoryInMegaBytes")]
    pub minimum_memory_in_mega_bytes: i64,
    #[doc = "Gets or sets target memory buffer in %."]
    #[serde(rename = "targetMemoryBufferPercentage")]
    pub target_memory_buffer_percentage: i32,
}
impl ProtectedItemDynamicMemoryConfig {
    pub fn new(maximum_memory_in_mega_bytes: i64, minimum_memory_in_mega_bytes: i64, target_memory_buffer_percentage: i32) -> Self {
        Self {
            maximum_memory_in_mega_bytes,
            minimum_memory_in_mega_bytes,
            target_memory_buffer_percentage,
        }
    }
}
#[doc = "Protected item job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectedItemJobProperties {
    #[doc = "Gets or sets protection scenario name."]
    #[serde(rename = "scenarioName", default, skip_serializing_if = "Option::is_none")]
    pub scenario_name: Option<String>,
    #[doc = "Gets or sets workflow Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets workflow name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the workflow friendly display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets workflow state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Gets or sets start time of the workflow."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets end time of the workflow."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl ProtectedItemJobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protected item model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectedItemModel {
    #[doc = "Protected item model properties."]
    pub properties: ProtectedItemModelProperties,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl ProtectedItemModel {
    pub fn new(properties: ProtectedItemModelProperties) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Protected item model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectedItemModelCollection {
    #[doc = "Gets or sets the list of protected items."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProtectedItemModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProtectedItemModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProtectedItemModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the instance type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "instanceType")]
pub enum ProtectedItemModelCustomPropertiesUnion {
    #[serde(rename = "HyperVToAzStackHCI")]
    HyperVToAzStackHci(HyperVToAzStackHciProtectedItemModelCustomProperties),
    #[serde(rename = "VMwareToAzStackHCI")]
    VMwareToAzStackHci(VMwareToAzStackHciProtectedItemModelCustomProperties),
}
#[doc = "Protected item model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectedItemModelProperties {
    #[doc = "Gets or sets the policy name."]
    #[serde(rename = "policyName")]
    pub policy_name: String,
    #[doc = "Gets or sets the replication extension name."]
    #[serde(rename = "replicationExtensionName")]
    pub replication_extension_name: String,
    #[doc = "Gets or sets the protected item correlation Id."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Gets or sets the provisioning state of the Dra."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<protected_item_model_properties::ProvisioningState>,
    #[doc = "Gets or sets the protection state."]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<protected_item_model_properties::ProtectionState>,
    #[doc = "Gets or sets the protection state description."]
    #[serde(rename = "protectionStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub protection_state_description: Option<String>,
    #[doc = "Gets or sets the test failover state."]
    #[serde(rename = "testFailoverState", default, skip_serializing_if = "Option::is_none")]
    pub test_failover_state: Option<protected_item_model_properties::TestFailoverState>,
    #[doc = "Gets or sets the Test failover state description."]
    #[serde(rename = "testFailoverStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub test_failover_state_description: Option<String>,
    #[doc = "Gets or sets the resynchronization state."]
    #[serde(rename = "resynchronizationState", default, skip_serializing_if = "Option::is_none")]
    pub resynchronization_state: Option<protected_item_model_properties::ResynchronizationState>,
    #[doc = "Gets or sets the fabric object Id."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "Gets or sets the fabric object name."]
    #[serde(rename = "fabricObjectName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_name: Option<String>,
    #[doc = "Gets or sets the source fabric provider Id."]
    #[serde(rename = "sourceFabricProviderId", default, skip_serializing_if = "Option::is_none")]
    pub source_fabric_provider_id: Option<String>,
    #[doc = "Gets or sets the target fabric provider Id."]
    #[serde(rename = "targetFabricProviderId", default, skip_serializing_if = "Option::is_none")]
    pub target_fabric_provider_id: Option<String>,
    #[doc = "Gets or sets the fabric Id."]
    #[serde(rename = "fabricId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_id: Option<String>,
    #[doc = "Gets or sets the target fabric Id."]
    #[serde(rename = "targetFabricId", default, skip_serializing_if = "Option::is_none")]
    pub target_fabric_id: Option<String>,
    #[doc = "Gets or sets the DRA Id."]
    #[serde(rename = "draId", default, skip_serializing_if = "Option::is_none")]
    pub dra_id: Option<String>,
    #[doc = "Gets or sets the target DRA Id."]
    #[serde(rename = "targetDraId", default, skip_serializing_if = "Option::is_none")]
    pub target_dra_id: Option<String>,
    #[doc = "Gets or sets a value indicating whether resynchronization is required or not."]
    #[serde(rename = "resyncRequired", default, skip_serializing_if = "Option::is_none")]
    pub resync_required: Option<bool>,
    #[doc = "Gets or sets the Last successful planned failover time."]
    #[serde(rename = "lastSuccessfulPlannedFailoverTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_successful_planned_failover_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the Last successful unplanned failover time."]
    #[serde(
        rename = "lastSuccessfulUnplannedFailoverTime",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub last_successful_unplanned_failover_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the Last successful test failover time."]
    #[serde(rename = "lastSuccessfulTestFailoverTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_successful_test_failover_time: Option<time::OffsetDateTime>,
    #[serde(rename = "currentJob", default, skip_serializing_if = "Option::is_none")]
    pub current_job: Option<serde_json::Value>,
    #[doc = "Gets or sets the allowed scenarios on the protected item."]
    #[serde(
        rename = "allowedJobs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_jobs: Vec<String>,
    #[serde(rename = "lastFailedEnableProtectionJob", default, skip_serializing_if = "Option::is_none")]
    pub last_failed_enable_protection_job: Option<serde_json::Value>,
    #[serde(rename = "lastFailedPlannedFailoverJob", default, skip_serializing_if = "Option::is_none")]
    pub last_failed_planned_failover_job: Option<serde_json::Value>,
    #[serde(rename = "lastTestFailoverJob", default, skip_serializing_if = "Option::is_none")]
    pub last_test_failover_job: Option<serde_json::Value>,
    #[doc = "Gets or sets protected item replication health."]
    #[serde(rename = "replicationHealth", default, skip_serializing_if = "Option::is_none")]
    pub replication_health: Option<protected_item_model_properties::ReplicationHealth>,
    #[doc = "Gets or sets the list of health errors."]
    #[serde(
        rename = "healthErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_errors: Vec<HealthErrorModel>,
    #[doc = "Protected item model custom properties."]
    #[serde(rename = "customProperties")]
    pub custom_properties: ProtectedItemModelCustomPropertiesUnion,
}
impl ProtectedItemModelProperties {
    pub fn new(
        policy_name: String,
        replication_extension_name: String,
        custom_properties: ProtectedItemModelCustomPropertiesUnion,
    ) -> Self {
        Self {
            policy_name,
            replication_extension_name,
            correlation_id: None,
            provisioning_state: None,
            protection_state: None,
            protection_state_description: None,
            test_failover_state: None,
            test_failover_state_description: None,
            resynchronization_state: None,
            fabric_object_id: None,
            fabric_object_name: None,
            source_fabric_provider_id: None,
            target_fabric_provider_id: None,
            fabric_id: None,
            target_fabric_id: None,
            dra_id: None,
            target_dra_id: None,
            resync_required: None,
            last_successful_planned_failover_time: None,
            last_successful_unplanned_failover_time: None,
            last_successful_test_failover_time: None,
            current_job: None,
            allowed_jobs: Vec::new(),
            last_failed_enable_protection_job: None,
            last_failed_planned_failover_job: None,
            last_test_failover_job: None,
            replication_health: None,
            health_errors: Vec::new(),
            custom_properties,
        }
    }
}
pub mod protected_item_model_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the Dra."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Canceled,
        Creating,
        Deleting,
        Deleted,
        Failed,
        Succeeded,
        Updating,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Canceled"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleted"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the protection state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionState")]
    pub enum ProtectionState {
        UnprotectedStatesBegin,
        EnablingProtection,
        EnablingFailed,
        DisablingProtection,
        MarkedForDeletion,
        DisablingFailed,
        UnprotectedStatesEnd,
        InitialReplicationStatesBegin,
        InitialReplicationInProgress,
        InitialReplicationCompletedOnPrimary,
        InitialReplicationCompletedOnRecovery,
        InitialReplicationFailed,
        InitialReplicationStatesEnd,
        ProtectedStatesBegin,
        Protected,
        ProtectedStatesEnd,
        PlannedFailoverTransitionStatesBegin,
        PlannedFailoverInitiated,
        PlannedFailoverCompleting,
        PlannedFailoverCompleted,
        PlannedFailoverFailed,
        PlannedFailoverCompletionFailed,
        PlannedFailoverTransitionStatesEnd,
        UnplannedFailoverTransitionStatesBegin,
        UnplannedFailoverInitiated,
        UnplannedFailoverCompleting,
        UnplannedFailoverCompleted,
        UnplannedFailoverFailed,
        UnplannedFailoverCompletionFailed,
        UnplannedFailoverTransitionStatesEnd,
        CommitFailoverStatesBegin,
        CommitFailoverInProgressOnPrimary,
        CommitFailoverInProgressOnRecovery,
        CommitFailoverCompleted,
        CommitFailoverFailedOnPrimary,
        CommitFailoverFailedOnRecovery,
        CommitFailoverStatesEnd,
        CancelFailoverStatesBegin,
        CancelFailoverInProgressOnPrimary,
        CancelFailoverInProgressOnRecovery,
        CancelFailoverFailedOnPrimary,
        CancelFailoverFailedOnRecovery,
        CancelFailoverStatesEnd,
        ChangeRecoveryPointStatesBegin,
        ChangeRecoveryPointInitiated,
        ChangeRecoveryPointCompleted,
        ChangeRecoveryPointFailed,
        ChangeRecoveryPointStatesEnd,
        ReprotectStatesBegin,
        ReprotectInitiated,
        ReprotectFailed,
        ReprotectStatesEnd,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UnprotectedStatesBegin => serializer.serialize_unit_variant("ProtectionState", 0u32, "UnprotectedStatesBegin"),
                Self::EnablingProtection => serializer.serialize_unit_variant("ProtectionState", 1u32, "EnablingProtection"),
                Self::EnablingFailed => serializer.serialize_unit_variant("ProtectionState", 2u32, "EnablingFailed"),
                Self::DisablingProtection => serializer.serialize_unit_variant("ProtectionState", 3u32, "DisablingProtection"),
                Self::MarkedForDeletion => serializer.serialize_unit_variant("ProtectionState", 4u32, "MarkedForDeletion"),
                Self::DisablingFailed => serializer.serialize_unit_variant("ProtectionState", 5u32, "DisablingFailed"),
                Self::UnprotectedStatesEnd => serializer.serialize_unit_variant("ProtectionState", 6u32, "UnprotectedStatesEnd"),
                Self::InitialReplicationStatesBegin => {
                    serializer.serialize_unit_variant("ProtectionState", 7u32, "InitialReplicationStatesBegin")
                }
                Self::InitialReplicationInProgress => {
                    serializer.serialize_unit_variant("ProtectionState", 8u32, "InitialReplicationInProgress")
                }
                Self::InitialReplicationCompletedOnPrimary => {
                    serializer.serialize_unit_variant("ProtectionState", 9u32, "InitialReplicationCompletedOnPrimary")
                }
                Self::InitialReplicationCompletedOnRecovery => {
                    serializer.serialize_unit_variant("ProtectionState", 10u32, "InitialReplicationCompletedOnRecovery")
                }
                Self::InitialReplicationFailed => serializer.serialize_unit_variant("ProtectionState", 11u32, "InitialReplicationFailed"),
                Self::InitialReplicationStatesEnd => {
                    serializer.serialize_unit_variant("ProtectionState", 12u32, "InitialReplicationStatesEnd")
                }
                Self::ProtectedStatesBegin => serializer.serialize_unit_variant("ProtectionState", 13u32, "ProtectedStatesBegin"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionState", 14u32, "Protected"),
                Self::ProtectedStatesEnd => serializer.serialize_unit_variant("ProtectionState", 15u32, "ProtectedStatesEnd"),
                Self::PlannedFailoverTransitionStatesBegin => {
                    serializer.serialize_unit_variant("ProtectionState", 16u32, "PlannedFailoverTransitionStatesBegin")
                }
                Self::PlannedFailoverInitiated => serializer.serialize_unit_variant("ProtectionState", 17u32, "PlannedFailoverInitiated"),
                Self::PlannedFailoverCompleting => serializer.serialize_unit_variant("ProtectionState", 18u32, "PlannedFailoverCompleting"),
                Self::PlannedFailoverCompleted => serializer.serialize_unit_variant("ProtectionState", 19u32, "PlannedFailoverCompleted"),
                Self::PlannedFailoverFailed => serializer.serialize_unit_variant("ProtectionState", 20u32, "PlannedFailoverFailed"),
                Self::PlannedFailoverCompletionFailed => {
                    serializer.serialize_unit_variant("ProtectionState", 21u32, "PlannedFailoverCompletionFailed")
                }
                Self::PlannedFailoverTransitionStatesEnd => {
                    serializer.serialize_unit_variant("ProtectionState", 22u32, "PlannedFailoverTransitionStatesEnd")
                }
                Self::UnplannedFailoverTransitionStatesBegin => {
                    serializer.serialize_unit_variant("ProtectionState", 23u32, "UnplannedFailoverTransitionStatesBegin")
                }
                Self::UnplannedFailoverInitiated => {
                    serializer.serialize_unit_variant("ProtectionState", 24u32, "UnplannedFailoverInitiated")
                }
                Self::UnplannedFailoverCompleting => {
                    serializer.serialize_unit_variant("ProtectionState", 25u32, "UnplannedFailoverCompleting")
                }
                Self::UnplannedFailoverCompleted => {
                    serializer.serialize_unit_variant("ProtectionState", 26u32, "UnplannedFailoverCompleted")
                }
                Self::UnplannedFailoverFailed => serializer.serialize_unit_variant("ProtectionState", 27u32, "UnplannedFailoverFailed"),
                Self::UnplannedFailoverCompletionFailed => {
                    serializer.serialize_unit_variant("ProtectionState", 28u32, "UnplannedFailoverCompletionFailed")
                }
                Self::UnplannedFailoverTransitionStatesEnd => {
                    serializer.serialize_unit_variant("ProtectionState", 29u32, "UnplannedFailoverTransitionStatesEnd")
                }
                Self::CommitFailoverStatesBegin => serializer.serialize_unit_variant("ProtectionState", 30u32, "CommitFailoverStatesBegin"),
                Self::CommitFailoverInProgressOnPrimary => {
                    serializer.serialize_unit_variant("ProtectionState", 31u32, "CommitFailoverInProgressOnPrimary")
                }
                Self::CommitFailoverInProgressOnRecovery => {
                    serializer.serialize_unit_variant("ProtectionState", 32u32, "CommitFailoverInProgressOnRecovery")
                }
                Self::CommitFailoverCompleted => serializer.serialize_unit_variant("ProtectionState", 33u32, "CommitFailoverCompleted"),
                Self::CommitFailoverFailedOnPrimary => {
                    serializer.serialize_unit_variant("ProtectionState", 34u32, "CommitFailoverFailedOnPrimary")
                }
                Self::CommitFailoverFailedOnRecovery => {
                    serializer.serialize_unit_variant("ProtectionState", 35u32, "CommitFailoverFailedOnRecovery")
                }
                Self::CommitFailoverStatesEnd => serializer.serialize_unit_variant("ProtectionState", 36u32, "CommitFailoverStatesEnd"),
                Self::CancelFailoverStatesBegin => serializer.serialize_unit_variant("ProtectionState", 37u32, "CancelFailoverStatesBegin"),
                Self::CancelFailoverInProgressOnPrimary => {
                    serializer.serialize_unit_variant("ProtectionState", 38u32, "CancelFailoverInProgressOnPrimary")
                }
                Self::CancelFailoverInProgressOnRecovery => {
                    serializer.serialize_unit_variant("ProtectionState", 39u32, "CancelFailoverInProgressOnRecovery")
                }
                Self::CancelFailoverFailedOnPrimary => {
                    serializer.serialize_unit_variant("ProtectionState", 40u32, "CancelFailoverFailedOnPrimary")
                }
                Self::CancelFailoverFailedOnRecovery => {
                    serializer.serialize_unit_variant("ProtectionState", 41u32, "CancelFailoverFailedOnRecovery")
                }
                Self::CancelFailoverStatesEnd => serializer.serialize_unit_variant("ProtectionState", 42u32, "CancelFailoverStatesEnd"),
                Self::ChangeRecoveryPointStatesBegin => {
                    serializer.serialize_unit_variant("ProtectionState", 43u32, "ChangeRecoveryPointStatesBegin")
                }
                Self::ChangeRecoveryPointInitiated => {
                    serializer.serialize_unit_variant("ProtectionState", 44u32, "ChangeRecoveryPointInitiated")
                }
                Self::ChangeRecoveryPointCompleted => {
                    serializer.serialize_unit_variant("ProtectionState", 45u32, "ChangeRecoveryPointCompleted")
                }
                Self::ChangeRecoveryPointFailed => serializer.serialize_unit_variant("ProtectionState", 46u32, "ChangeRecoveryPointFailed"),
                Self::ChangeRecoveryPointStatesEnd => {
                    serializer.serialize_unit_variant("ProtectionState", 47u32, "ChangeRecoveryPointStatesEnd")
                }
                Self::ReprotectStatesBegin => serializer.serialize_unit_variant("ProtectionState", 48u32, "ReprotectStatesBegin"),
                Self::ReprotectInitiated => serializer.serialize_unit_variant("ProtectionState", 49u32, "ReprotectInitiated"),
                Self::ReprotectFailed => serializer.serialize_unit_variant("ProtectionState", 50u32, "ReprotectFailed"),
                Self::ReprotectStatesEnd => serializer.serialize_unit_variant("ProtectionState", 51u32, "ReprotectStatesEnd"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the test failover state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TestFailoverState")]
    pub enum TestFailoverState {
        None,
        TestFailoverInitiated,
        TestFailoverCompleting,
        TestFailoverCompleted,
        TestFailoverFailed,
        TestFailoverCompletionFailed,
        TestFailoverCleanupInitiated,
        TestFailoverCleanupCompleting,
        MarkedForDeletion,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TestFailoverState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TestFailoverState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TestFailoverState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("TestFailoverState", 0u32, "None"),
                Self::TestFailoverInitiated => serializer.serialize_unit_variant("TestFailoverState", 1u32, "TestFailoverInitiated"),
                Self::TestFailoverCompleting => serializer.serialize_unit_variant("TestFailoverState", 2u32, "TestFailoverCompleting"),
                Self::TestFailoverCompleted => serializer.serialize_unit_variant("TestFailoverState", 3u32, "TestFailoverCompleted"),
                Self::TestFailoverFailed => serializer.serialize_unit_variant("TestFailoverState", 4u32, "TestFailoverFailed"),
                Self::TestFailoverCompletionFailed => {
                    serializer.serialize_unit_variant("TestFailoverState", 5u32, "TestFailoverCompletionFailed")
                }
                Self::TestFailoverCleanupInitiated => {
                    serializer.serialize_unit_variant("TestFailoverState", 6u32, "TestFailoverCleanupInitiated")
                }
                Self::TestFailoverCleanupCompleting => {
                    serializer.serialize_unit_variant("TestFailoverState", 7u32, "TestFailoverCleanupCompleting")
                }
                Self::MarkedForDeletion => serializer.serialize_unit_variant("TestFailoverState", 8u32, "MarkedForDeletion"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the resynchronization state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResynchronizationState")]
    pub enum ResynchronizationState {
        None,
        ResynchronizationInitiated,
        ResynchronizationCompleted,
        ResynchronizationFailed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResynchronizationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResynchronizationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResynchronizationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ResynchronizationState", 0u32, "None"),
                Self::ResynchronizationInitiated => {
                    serializer.serialize_unit_variant("ResynchronizationState", 1u32, "ResynchronizationInitiated")
                }
                Self::ResynchronizationCompleted => {
                    serializer.serialize_unit_variant("ResynchronizationState", 2u32, "ResynchronizationCompleted")
                }
                Self::ResynchronizationFailed => {
                    serializer.serialize_unit_variant("ResynchronizationState", 3u32, "ResynchronizationFailed")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets protected item replication health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationHealth")]
    pub enum ReplicationHealth {
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationHealth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationHealth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationHealth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Normal => serializer.serialize_unit_variant("ReplicationHealth", 0u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("ReplicationHealth", 1u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("ReplicationHealth", 2u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery point model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPointModel {
    #[doc = "Recovery point model properties."]
    pub properties: RecoveryPointModelProperties,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl RecoveryPointModel {
    pub fn new(properties: RecoveryPointModelProperties) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Recovery point model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointModelCollection {
    #[doc = "Gets or sets the list of recovery points."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RecoveryPointModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecoveryPointModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RecoveryPointModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the instance type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "instanceType")]
pub enum RecoveryPointModelCustomPropertiesUnion {
    #[serde(rename = "HyperVToAzStackHCI")]
    HyperVToAzStackHci(HyperVToAzStackHciRecoveryPointModelCustomProperties),
}
#[doc = "Recovery point model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPointModelProperties {
    #[doc = "Gets or sets the recovery point time."]
    #[serde(rename = "recoveryPointTime", with = "azure_core::date::rfc3339")]
    pub recovery_point_time: time::OffsetDateTime,
    #[doc = "Gets or sets the recovery point type."]
    #[serde(rename = "recoveryPointType")]
    pub recovery_point_type: recovery_point_model_properties::RecoveryPointType,
    #[doc = "Recovery point model custom properties."]
    #[serde(rename = "customProperties")]
    pub custom_properties: RecoveryPointModelCustomPropertiesUnion,
}
impl RecoveryPointModelProperties {
    pub fn new(
        recovery_point_time: time::OffsetDateTime,
        recovery_point_type: recovery_point_model_properties::RecoveryPointType,
        custom_properties: RecoveryPointModelCustomPropertiesUnion,
    ) -> Self {
        Self {
            recovery_point_time,
            recovery_point_type,
            custom_properties,
        }
    }
}
pub mod recovery_point_model_properties {
    use super::*;
    #[doc = "Gets or sets the recovery point type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        ApplicationConsistent,
        CrashConsistent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ApplicationConsistent => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "ApplicationConsistent"),
                Self::CrashConsistent => serializer.serialize_unit_variant("RecoveryPointType", 1u32, "CrashConsistent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Replication extension model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationExtensionModel {
    #[doc = "Replication extension model properties."]
    pub properties: ReplicationExtensionModelProperties,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl ReplicationExtensionModel {
    pub fn new(properties: ReplicationExtensionModelProperties) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Replication extension model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationExtensionModelCollection {
    #[doc = "Gets or sets the list of replication extensions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReplicationExtensionModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReplicationExtensionModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReplicationExtensionModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets the instance type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "instanceType")]
pub enum ReplicationExtensionModelCustomPropertiesUnion {
    #[serde(rename = "HyperVToAzStackHCI")]
    HyperVToAzStackHci(HyperVToAzStackHciReplicationExtensionModelCustomProperties),
    #[serde(rename = "VMwareToAzStackHCI")]
    VMwareToAzStackHci(VMwareToAzStackHciReplicationExtensionModelCustomProperties),
}
#[doc = "Replication extension model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationExtensionModelProperties {
    #[doc = "Gets or sets the provisioning state of the replication extension."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<replication_extension_model_properties::ProvisioningState>,
    #[doc = "Replication extension model custom properties."]
    #[serde(rename = "customProperties")]
    pub custom_properties: ReplicationExtensionModelCustomPropertiesUnion,
}
impl ReplicationExtensionModelProperties {
    pub fn new(custom_properties: ReplicationExtensionModelCustomPropertiesUnion) -> Self {
        Self {
            provisioning_state: None,
            custom_properties,
        }
    }
}
pub mod replication_extension_model_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the replication extension."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Canceled,
        Creating,
        Deleting,
        Deleted,
        Failed,
        Succeeded,
        Updating,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Canceled"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleted"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Storage container properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageContainerProperties {
    #[doc = "Gets or sets the Name."]
    pub name: String,
    #[doc = "Gets or sets the ClusterSharedVolumePath."]
    #[serde(rename = "clusterSharedVolumePath")]
    pub cluster_shared_volume_path: String,
}
impl StorageContainerProperties {
    pub fn new(name: String, cluster_shared_volume_path: String) -> Self {
        Self {
            name,
            cluster_shared_volume_path,
        }
    }
}
#[doc = "System data required to be defined for Azure resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemDataModel {
    #[doc = "Gets or sets identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Gets or sets the type of identity that created the resource: user, application,\r\nmanagedIdentity."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<String>,
    #[doc = "Gets or sets the timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "Gets or sets the type of identity that last modified the resource: user, application,\r\nmanagedIdentity."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<String>,
    #[doc = "Gets or sets the timestamp of resource last modification (UTC)."]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemDataModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Task model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskModel {
    #[doc = "Gets or sets the task name."]
    #[serde(rename = "taskName", default, skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[doc = "Gets or sets the task state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<task_model::State>,
    #[doc = "Gets or sets the start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Task model custom properties."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<TaskModelCustomProperties>,
    #[doc = "Gets or sets the list of children workflow models."]
    #[serde(
        rename = "childrenWorkflows",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub children_workflows: Vec<WorkflowModel>,
}
impl TaskModel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod task_model {
    use super::*;
    #[doc = "Gets or sets the task state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Pending,
        Started,
        Succeeded,
        Failed,
        Cancelled,
        Skipped,
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
                Self::Pending => serializer.serialize_unit_variant("State", 0u32, "Pending"),
                Self::Started => serializer.serialize_unit_variant("State", 1u32, "Started"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("State", 4u32, "Cancelled"),
                Self::Skipped => serializer.serialize_unit_variant("State", 5u32, "Skipped"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Task model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskModelCustomProperties {
    #[doc = "Gets or sets the instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl TaskModelCustomProperties {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Test failover cleanup workflow model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestFailoverCleanupWorkflowModelCustomProperties {
    #[serde(flatten)]
    pub workflow_model_custom_properties: WorkflowModelCustomProperties,
    #[doc = "Gets or sets the test failover cleanup comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}
impl TestFailoverCleanupWorkflowModelCustomProperties {
    pub fn new(workflow_model_custom_properties: WorkflowModelCustomProperties) -> Self {
        Self {
            workflow_model_custom_properties,
            comments: None,
        }
    }
}
#[doc = "Test failover workflow model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestFailoverWorkflowModelCustomProperties {
    #[serde(flatten)]
    pub workflow_model_custom_properties: WorkflowModelCustomProperties,
    #[doc = "Gets or sets the test VM details."]
    #[serde(
        rename = "protectedItemDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protected_item_details: Vec<FailoverProtectedItemProperties>,
}
impl TestFailoverWorkflowModelCustomProperties {
    pub fn new(workflow_model_custom_properties: WorkflowModelCustomProperties) -> Self {
        Self {
            workflow_model_custom_properties,
            protected_item_details: Vec::new(),
        }
    }
}
#[doc = "VMware DRA model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareDraModelCustomProperties {
    #[doc = "Gets or sets the BIOS Id of the DRA machine."]
    #[serde(rename = "biosId")]
    pub bios_id: String,
    #[doc = "Identity model."]
    #[serde(rename = "marsAuthenticationIdentity")]
    pub mars_authentication_identity: IdentityModel,
}
impl VMwareDraModelCustomProperties {
    pub fn new(bios_id: String, mars_authentication_identity: IdentityModel) -> Self {
        Self {
            bios_id,
            mars_authentication_identity,
        }
    }
}
#[doc = "VMware migrate fabric model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareMigrateFabricModelCustomProperties {
    #[doc = "Gets or sets the ARM Id of the VMware site."]
    #[serde(rename = "vmwareSiteId")]
    pub vmware_site_id: String,
    #[doc = "Gets or sets the ARM Id of the migration solution."]
    #[serde(rename = "migrationSolutionId")]
    pub migration_solution_id: String,
}
impl VMwareMigrateFabricModelCustomProperties {
    pub fn new(vmware_site_id: String, migration_solution_id: String) -> Self {
        Self {
            vmware_site_id,
            migration_solution_id,
        }
    }
}
#[doc = "VMwareToAzStack disk input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareToAzStackHciDiskInput {
    #[doc = "Gets or sets the disk Id."]
    #[serde(rename = "diskId")]
    pub disk_id: String,
    #[doc = "Gets or sets the target storage account ARM Id."]
    #[serde(rename = "storageContainerId", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_id: Option<String>,
    #[doc = "Gets or sets a value indicating whether dynamic sizing is enabled on the virtual hard\r\ndisk."]
    #[serde(rename = "isDynamic", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic: Option<bool>,
    #[doc = "Gets or sets the disk size in GB."]
    #[serde(rename = "diskSizeGB")]
    pub disk_size_gb: i64,
    #[doc = "Gets or sets the type of the virtual hard disk, vhd or vhdx."]
    #[serde(rename = "diskFileFormat")]
    pub disk_file_format: String,
    #[doc = "Gets or sets a value indicating whether disk is os disk."]
    #[serde(rename = "isOsDisk")]
    pub is_os_disk: bool,
}
impl VMwareToAzStackHciDiskInput {
    pub fn new(disk_id: String, disk_size_gb: i64, disk_file_format: String, is_os_disk: bool) -> Self {
        Self {
            disk_id,
            storage_container_id: None,
            is_dynamic: None,
            disk_size_gb,
            disk_file_format,
            is_os_disk,
        }
    }
}
#[doc = "VMwareToAzStackHCI NIC properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareToAzStackHciNicInput {
    #[doc = "Gets or sets the NIC Id."]
    #[serde(rename = "nicId")]
    pub nic_id: String,
    #[doc = "Gets or sets the NIC label."]
    pub label: String,
    #[doc = "Gets or sets the network name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Gets or sets the target network Id within AzStackHCI Cluster."]
    #[serde(rename = "targetNetworkId")]
    pub target_network_id: String,
    #[doc = "Gets or sets the target test network Id within AzStackHCI Cluster."]
    #[serde(rename = "testNetworkId")]
    pub test_network_id: String,
    #[doc = "Gets or sets the selection type of the NIC."]
    #[serde(rename = "selectionTypeForFailover")]
    pub selection_type_for_failover: v_mware_to_az_stack_hci_nic_input::SelectionTypeForFailover,
}
impl VMwareToAzStackHciNicInput {
    pub fn new(
        nic_id: String,
        label: String,
        target_network_id: String,
        test_network_id: String,
        selection_type_for_failover: v_mware_to_az_stack_hci_nic_input::SelectionTypeForFailover,
    ) -> Self {
        Self {
            nic_id,
            label,
            network_name: None,
            target_network_id,
            test_network_id,
            selection_type_for_failover,
        }
    }
}
pub mod v_mware_to_az_stack_hci_nic_input {
    use super::*;
    #[doc = "Gets or sets the selection type of the NIC."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SelectionTypeForFailover")]
    pub enum SelectionTypeForFailover {
        NotSelected,
        SelectedByUser,
        SelectedByDefault,
        SelectedByUserOverride,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SelectionTypeForFailover {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SelectionTypeForFailover {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SelectionTypeForFailover {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSelected => serializer.serialize_unit_variant("SelectionTypeForFailover", 0u32, "NotSelected"),
                Self::SelectedByUser => serializer.serialize_unit_variant("SelectionTypeForFailover", 1u32, "SelectedByUser"),
                Self::SelectedByDefault => serializer.serialize_unit_variant("SelectionTypeForFailover", 2u32, "SelectedByDefault"),
                Self::SelectedByUserOverride => {
                    serializer.serialize_unit_variant("SelectionTypeForFailover", 3u32, "SelectedByUserOverride")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VMware to AzStackHCI planned failover model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareToAzStackHciPlannedFailoverModelCustomProperties {
    #[doc = "Gets or sets a value indicating whether VM needs to be shut down."]
    #[serde(rename = "shutdownSourceVM")]
    pub shutdown_source_vm: bool,
}
impl VMwareToAzStackHciPlannedFailoverModelCustomProperties {
    pub fn new(shutdown_source_vm: bool) -> Self {
        Self { shutdown_source_vm }
    }
}
#[doc = "VMware To AzStackHCI Policy model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareToAzStackHciPolicyModelCustomProperties {
    #[doc = "Gets or sets the duration in minutes until which the recovery points need to be\r\nstored."]
    #[serde(rename = "recoveryPointHistoryInMinutes")]
    pub recovery_point_history_in_minutes: i32,
    #[doc = "Gets or sets the crash consistent snapshot frequency (in minutes)."]
    #[serde(rename = "crashConsistentFrequencyInMinutes")]
    pub crash_consistent_frequency_in_minutes: i32,
    #[doc = "Gets or sets the app consistent snapshot frequency (in minutes)."]
    #[serde(rename = "appConsistentFrequencyInMinutes")]
    pub app_consistent_frequency_in_minutes: i32,
}
impl VMwareToAzStackHciPolicyModelCustomProperties {
    pub fn new(
        recovery_point_history_in_minutes: i32,
        crash_consistent_frequency_in_minutes: i32,
        app_consistent_frequency_in_minutes: i32,
    ) -> Self {
        Self {
            recovery_point_history_in_minutes,
            crash_consistent_frequency_in_minutes,
            app_consistent_frequency_in_minutes,
        }
    }
}
#[doc = "VMwareToAzStackHCI protected disk properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareToAzStackHciProtectedDiskProperties {
    #[doc = "Gets or sets the ARM Id of the storage container."]
    #[serde(rename = "storageContainerId", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_id: Option<String>,
    #[doc = "Gets or sets the local path of the storage container."]
    #[serde(rename = "storageContainerLocalPath", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_local_path: Option<String>,
    #[doc = "Gets or sets the source disk Id."]
    #[serde(rename = "sourceDiskId", default, skip_serializing_if = "Option::is_none")]
    pub source_disk_id: Option<String>,
    #[doc = "Gets or sets the source disk Name."]
    #[serde(rename = "sourceDiskName", default, skip_serializing_if = "Option::is_none")]
    pub source_disk_name: Option<String>,
    #[doc = "Gets or sets the seed disk name."]
    #[serde(rename = "seedDiskName", default, skip_serializing_if = "Option::is_none")]
    pub seed_disk_name: Option<String>,
    #[doc = "Gets or sets the test failover clone disk."]
    #[serde(rename = "testMigrateDiskName", default, skip_serializing_if = "Option::is_none")]
    pub test_migrate_disk_name: Option<String>,
    #[doc = "Gets or sets the failover clone disk."]
    #[serde(rename = "migrateDiskName", default, skip_serializing_if = "Option::is_none")]
    pub migrate_disk_name: Option<String>,
    #[doc = "Gets or sets a value indicating whether the disk is the OS disk."]
    #[serde(rename = "isOsDisk", default, skip_serializing_if = "Option::is_none")]
    pub is_os_disk: Option<bool>,
    #[doc = "Gets or sets the disk capacity in bytes."]
    #[serde(rename = "capacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub capacity_in_bytes: Option<i64>,
    #[doc = "Gets or sets a value indicating whether dynamic sizing is enabled on the virtual hard\r\ndisk."]
    #[serde(rename = "isDynamic", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic: Option<bool>,
    #[doc = "Gets or sets the disk type."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
}
impl VMwareToAzStackHciProtectedDiskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VMware to AzStackHCI Protected item model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareToAzStackHciProtectedItemModelCustomProperties {
    #[doc = "Gets or sets the location of the protected item."]
    #[serde(rename = "activeLocation", default, skip_serializing_if = "Option::is_none")]
    pub active_location: Option<v_mware_to_az_stack_hci_protected_item_model_custom_properties::ActiveLocation>,
    #[doc = "Gets or sets the Target HCI Cluster ARM Id."]
    #[serde(rename = "targetHciClusterId")]
    pub target_hci_cluster_id: String,
    #[doc = "Gets or sets the Target Arc Cluster Custom Location ARM Id."]
    #[serde(rename = "targetArcClusterCustomLocationId")]
    pub target_arc_cluster_custom_location_id: String,
    #[doc = "Gets or sets the Target AzStackHCI cluster name."]
    #[serde(rename = "targetAzStackHciClusterName", default, skip_serializing_if = "Option::is_none")]
    pub target_az_stack_hci_cluster_name: Option<String>,
    #[doc = "Gets or sets the target storage container ARM Id."]
    #[serde(rename = "storageContainerId")]
    pub storage_container_id: String,
    #[doc = "Gets or sets the target resource group ARM Id."]
    #[serde(rename = "targetResourceGroupId")]
    pub target_resource_group_id: String,
    #[doc = "Gets or sets the target location."]
    #[serde(rename = "targetLocation", default, skip_serializing_if = "Option::is_none")]
    pub target_location: Option<String>,
    #[doc = "Gets or sets the location of Azure Arc HCI custom location resource."]
    #[serde(rename = "customLocationRegion")]
    pub custom_location_region: String,
    #[doc = "Gets or sets the list of disks to replicate."]
    #[serde(rename = "disksToInclude")]
    pub disks_to_include: Vec<VMwareToAzStackHciDiskInput>,
    #[doc = "Gets or sets the list of VM NIC to replicate."]
    #[serde(rename = "nicsToInclude")]
    pub nics_to_include: Vec<VMwareToAzStackHciNicInput>,
    #[doc = "Gets or sets the list of protected disks."]
    #[serde(
        rename = "protectedDisks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protected_disks: Vec<VMwareToAzStackHciProtectedDiskProperties>,
    #[doc = "Gets or sets the VM NIC details."]
    #[serde(
        rename = "protectedNics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protected_nics: Vec<VMwareToAzStackHciProtectedNicProperties>,
    #[doc = "Gets or sets the BIOS Id of the target AzStackHCI VM."]
    #[serde(rename = "targetVmBiosId", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_bios_id: Option<String>,
    #[doc = "Gets or sets the target VM display name."]
    #[serde(rename = "targetVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_name: Option<String>,
    #[doc = "Gets or sets the hypervisor generation of the virtual machine possible values are 1,2."]
    #[serde(rename = "hyperVGeneration")]
    pub hyper_v_generation: String,
    #[doc = "Gets or sets the target network Id within AzStackHCI Cluster."]
    #[serde(rename = "targetNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_network_id: Option<String>,
    #[doc = "Gets or sets the target test network Id within AzStackHCI Cluster."]
    #[serde(rename = "testNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub test_network_id: Option<String>,
    #[doc = "Gets or sets the target CPU cores."]
    #[serde(rename = "targetCpuCores", default, skip_serializing_if = "Option::is_none")]
    pub target_cpu_cores: Option<i32>,
    #[doc = "Gets or sets a value indicating whether memory is dynamical."]
    #[serde(rename = "isDynamicRam", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic_ram: Option<bool>,
    #[doc = "Protected item dynamic memory config."]
    #[serde(rename = "dynamicMemoryConfig", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_config: Option<ProtectedItemDynamicMemoryConfig>,
    #[doc = "Gets or sets the target memory in mega-bytes."]
    #[serde(rename = "targetMemoryInMegaBytes", default, skip_serializing_if = "Option::is_none")]
    pub target_memory_in_mega_bytes: Option<i32>,
    #[doc = "Gets or sets the type of the OS."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Gets or sets the name of the OS."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets the firmware type."]
    #[serde(rename = "firmwareType", default, skip_serializing_if = "Option::is_none")]
    pub firmware_type: Option<String>,
    #[doc = "Gets or sets the ARM Id of the discovered machine."]
    #[serde(rename = "fabricDiscoveryMachineId")]
    pub fabric_discovery_machine_id: String,
    #[doc = "Gets or sets the source VM display name."]
    #[serde(rename = "sourceVmName", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_name: Option<String>,
    #[doc = "Gets or sets the source VM CPU cores."]
    #[serde(rename = "sourceCpuCores", default, skip_serializing_if = "Option::is_none")]
    pub source_cpu_cores: Option<i32>,
    #[doc = "Gets or sets the source VM ram memory size in megabytes."]
    #[serde(rename = "sourceMemoryInMegaBytes", default, skip_serializing_if = "Option::is_none")]
    pub source_memory_in_mega_bytes: Option<f64>,
    #[doc = "Gets or sets the run as account Id."]
    #[serde(rename = "runAsAccountId")]
    pub run_as_account_id: String,
    #[doc = "Gets or sets the source DRA name."]
    #[serde(rename = "sourceDraName")]
    pub source_dra_name: String,
    #[doc = "Gets or sets the target DRA name."]
    #[serde(rename = "targetDraName")]
    pub target_dra_name: String,
    #[doc = "Gets or sets the source appliance name."]
    #[serde(rename = "sourceApplianceName", default, skip_serializing_if = "Option::is_none")]
    pub source_appliance_name: Option<String>,
    #[doc = "Gets or sets the target appliance name."]
    #[serde(rename = "targetApplianceName", default, skip_serializing_if = "Option::is_none")]
    pub target_appliance_name: Option<String>,
    #[doc = "Gets or sets the recovery point Id to which the VM was failed over."]
    #[serde(rename = "failoverRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub failover_recovery_point_id: Option<String>,
    #[doc = "Gets or sets the last recovery point received time."]
    #[serde(rename = "lastRecoveryPointReceived", default, with = "azure_core::date::rfc3339::option")]
    pub last_recovery_point_received: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last recovery point Id."]
    #[serde(rename = "lastRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub last_recovery_point_id: Option<String>,
    #[doc = "Gets or sets the initial replication progress percentage. This is calculated based on\r\ntotal bytes processed for all disks in the source VM."]
    #[serde(rename = "initialReplicationProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_progress_percentage: Option<i32>,
    #[doc = "Gets or sets the migration progress percentage."]
    #[serde(rename = "migrationProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub migration_progress_percentage: Option<i32>,
    #[doc = "Gets or sets the resume progress percentage."]
    #[serde(rename = "resumeProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub resume_progress_percentage: Option<i32>,
    #[doc = "Gets or sets the resync progress percentage. This is calculated based on total bytes\r\nprocessed for all disks in the source VM."]
    #[serde(rename = "resyncProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub resync_progress_percentage: Option<i32>,
    #[doc = "Gets or sets the resync retry count."]
    #[serde(rename = "resyncRetryCount", default, skip_serializing_if = "Option::is_none")]
    pub resync_retry_count: Option<i64>,
    #[doc = "Gets or sets a value indicating whether resync is required."]
    #[serde(rename = "resyncRequired", default, skip_serializing_if = "Option::is_none")]
    pub resync_required: Option<bool>,
    #[doc = "Gets or sets the resync state."]
    #[serde(rename = "resyncState", default, skip_serializing_if = "Option::is_none")]
    pub resync_state: Option<v_mware_to_az_stack_hci_protected_item_model_custom_properties::ResyncState>,
    #[doc = "Gets or sets a value indicating whether auto resync is to be done."]
    #[serde(rename = "performAutoResync", default, skip_serializing_if = "Option::is_none")]
    pub perform_auto_resync: Option<bool>,
    #[doc = "Gets or sets the resume retry count."]
    #[serde(rename = "resumeRetryCount", default, skip_serializing_if = "Option::is_none")]
    pub resume_retry_count: Option<i64>,
    #[doc = "Gets or sets the latest timestamp that replication status is updated."]
    #[serde(rename = "lastReplicationUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_replication_update_time: Option<time::OffsetDateTime>,
}
impl VMwareToAzStackHciProtectedItemModelCustomProperties {
    pub fn new(
        target_hci_cluster_id: String,
        target_arc_cluster_custom_location_id: String,
        storage_container_id: String,
        target_resource_group_id: String,
        custom_location_region: String,
        disks_to_include: Vec<VMwareToAzStackHciDiskInput>,
        nics_to_include: Vec<VMwareToAzStackHciNicInput>,
        hyper_v_generation: String,
        fabric_discovery_machine_id: String,
        run_as_account_id: String,
        source_dra_name: String,
        target_dra_name: String,
    ) -> Self {
        Self {
            active_location: None,
            target_hci_cluster_id,
            target_arc_cluster_custom_location_id,
            target_az_stack_hci_cluster_name: None,
            storage_container_id,
            target_resource_group_id,
            target_location: None,
            custom_location_region,
            disks_to_include,
            nics_to_include,
            protected_disks: Vec::new(),
            protected_nics: Vec::new(),
            target_vm_bios_id: None,
            target_vm_name: None,
            hyper_v_generation,
            target_network_id: None,
            test_network_id: None,
            target_cpu_cores: None,
            is_dynamic_ram: None,
            dynamic_memory_config: None,
            target_memory_in_mega_bytes: None,
            os_type: None,
            os_name: None,
            firmware_type: None,
            fabric_discovery_machine_id,
            source_vm_name: None,
            source_cpu_cores: None,
            source_memory_in_mega_bytes: None,
            run_as_account_id,
            source_dra_name,
            target_dra_name,
            source_appliance_name: None,
            target_appliance_name: None,
            failover_recovery_point_id: None,
            last_recovery_point_received: None,
            last_recovery_point_id: None,
            initial_replication_progress_percentage: None,
            migration_progress_percentage: None,
            resume_progress_percentage: None,
            resync_progress_percentage: None,
            resync_retry_count: None,
            resync_required: None,
            resync_state: None,
            perform_auto_resync: None,
            resume_retry_count: None,
            last_replication_update_time: None,
        }
    }
}
pub mod v_mware_to_az_stack_hci_protected_item_model_custom_properties {
    use super::*;
    #[doc = "Gets or sets the location of the protected item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActiveLocation")]
    pub enum ActiveLocation {
        Primary,
        Recovery,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActiveLocation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActiveLocation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActiveLocation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("ActiveLocation", 0u32, "Primary"),
                Self::Recovery => serializer.serialize_unit_variant("ActiveLocation", 1u32, "Recovery"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the resync state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResyncState")]
    pub enum ResyncState {
        None,
        PreparedForResynchronization,
        StartedResynchronization,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResyncState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResyncState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResyncState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ResyncState", 0u32, "None"),
                Self::PreparedForResynchronization => {
                    serializer.serialize_unit_variant("ResyncState", 1u32, "PreparedForResynchronization")
                }
                Self::StartedResynchronization => serializer.serialize_unit_variant("ResyncState", 2u32, "StartedResynchronization"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VMwareToAzStackHCI NIC properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareToAzStackHciProtectedNicProperties {
    #[doc = "Gets or sets the NIC Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "Gets or sets the NIC mac address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets or sets the NIC label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Gets or sets a value indicating whether this is the primary NIC."]
    #[serde(rename = "isPrimaryNic", default, skip_serializing_if = "Option::is_none")]
    pub is_primary_nic: Option<bool>,
    #[doc = "Gets or sets the network name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Gets or sets the target network Id within AzStackHCI Cluster."]
    #[serde(rename = "targetNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_network_id: Option<String>,
    #[doc = "Gets or sets the target test network Id within AzStackHCI Cluster."]
    #[serde(rename = "testNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub test_network_id: Option<String>,
    #[doc = "Gets or sets the selection type of the NIC."]
    #[serde(rename = "selectionTypeForFailover", default, skip_serializing_if = "Option::is_none")]
    pub selection_type_for_failover: Option<v_mware_to_az_stack_hci_protected_nic_properties::SelectionTypeForFailover>,
}
impl VMwareToAzStackHciProtectedNicProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod v_mware_to_az_stack_hci_protected_nic_properties {
    use super::*;
    #[doc = "Gets or sets the selection type of the NIC."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SelectionTypeForFailover")]
    pub enum SelectionTypeForFailover {
        NotSelected,
        SelectedByUser,
        SelectedByDefault,
        SelectedByUserOverride,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SelectionTypeForFailover {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SelectionTypeForFailover {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SelectionTypeForFailover {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSelected => serializer.serialize_unit_variant("SelectionTypeForFailover", 0u32, "NotSelected"),
                Self::SelectedByUser => serializer.serialize_unit_variant("SelectionTypeForFailover", 1u32, "SelectedByUser"),
                Self::SelectedByDefault => serializer.serialize_unit_variant("SelectionTypeForFailover", 2u32, "SelectedByDefault"),
                Self::SelectedByUserOverride => {
                    serializer.serialize_unit_variant("SelectionTypeForFailover", 3u32, "SelectedByUserOverride")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VMware to AzStackHCI Replication extension model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareToAzStackHciReplicationExtensionModelCustomProperties {
    #[doc = "Gets or sets the ARM Id of the source VMware fabric."]
    #[serde(rename = "vmwareFabricArmId")]
    pub vmware_fabric_arm_id: String,
    #[doc = "Gets or sets the ARM Id of the VMware site."]
    #[serde(rename = "vmwareSiteId", default, skip_serializing_if = "Option::is_none")]
    pub vmware_site_id: Option<String>,
    #[doc = "Gets or sets the ARM Id of the target AzStackHCI fabric."]
    #[serde(rename = "azStackHciFabricArmId")]
    pub az_stack_hci_fabric_arm_id: String,
    #[doc = "Gets or sets the ARM Id of the AzStackHCI site."]
    #[serde(rename = "azStackHciSiteId", default, skip_serializing_if = "Option::is_none")]
    pub az_stack_hci_site_id: Option<String>,
    #[doc = "Gets or sets the storage account Id."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "Gets or sets the Sas Secret of storage account."]
    #[serde(rename = "storageAccountSasSecretName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_sas_secret_name: Option<String>,
    #[doc = "Gets or sets the Uri of ASR."]
    #[serde(rename = "asrServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub asr_service_uri: Option<String>,
    #[doc = "Gets or sets the Uri of Rcm."]
    #[serde(rename = "rcmServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub rcm_service_uri: Option<String>,
    #[doc = "Gets or sets the Uri of Gateway."]
    #[serde(rename = "gatewayServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub gateway_service_uri: Option<String>,
    #[doc = "Gets or sets the gateway service Id of source."]
    #[serde(rename = "sourceGatewayServiceId", default, skip_serializing_if = "Option::is_none")]
    pub source_gateway_service_id: Option<String>,
    #[doc = "Gets or sets the gateway service Id of target."]
    #[serde(rename = "targetGatewayServiceId", default, skip_serializing_if = "Option::is_none")]
    pub target_gateway_service_id: Option<String>,
    #[doc = "Gets or sets the source storage container name."]
    #[serde(rename = "sourceStorageContainerName", default, skip_serializing_if = "Option::is_none")]
    pub source_storage_container_name: Option<String>,
    #[doc = "Gets or sets the target storage container name."]
    #[serde(rename = "targetStorageContainerName", default, skip_serializing_if = "Option::is_none")]
    pub target_storage_container_name: Option<String>,
    #[doc = "Gets or sets the resource location."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Gets or sets the subscription."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Gets or sets the resource group."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl VMwareToAzStackHciReplicationExtensionModelCustomProperties {
    pub fn new(vmware_fabric_arm_id: String, az_stack_hci_fabric_arm_id: String) -> Self {
        Self {
            vmware_fabric_arm_id,
            vmware_site_id: None,
            az_stack_hci_fabric_arm_id,
            az_stack_hci_site_id: None,
            storage_account_id: None,
            storage_account_sas_secret_name: None,
            asr_service_uri: None,
            rcm_service_uri: None,
            gateway_service_uri: None,
            source_gateway_service_id: None,
            target_gateway_service_id: None,
            source_storage_container_name: None,
            target_storage_container_name: None,
            resource_location: None,
            subscription_id: None,
            resource_group: None,
        }
    }
}
#[doc = "Vault model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultModel {
    #[doc = "Gets or sets the location of the vault."]
    pub location: String,
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Vault properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultModelProperties>,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl VaultModel {
    pub fn new(location: String) -> Self {
        Self {
            location,
            tags: None,
            properties: None,
            id: None,
            name: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Vault model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultModelCollection {
    #[doc = "Gets or sets the list of vaults."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VaultModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VaultModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VaultModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vault properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultModelProperties {
    #[doc = "Gets or sets the provisioning state of the vault."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<vault_model_properties::ProvisioningState>,
    #[doc = "Gets or sets the service resource Id."]
    #[serde(rename = "serviceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub service_resource_id: Option<String>,
    #[doc = "Gets or sets the type of vault."]
    #[serde(rename = "vaultType", default, skip_serializing_if = "Option::is_none")]
    pub vault_type: Option<vault_model_properties::VaultType>,
}
impl VaultModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vault_model_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Canceled,
        Creating,
        Deleting,
        Deleted,
        Failed,
        Succeeded,
        Updating,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Canceled"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleted"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the type of vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VaultType")]
    pub enum VaultType {
        DisasterRecovery,
        Migrate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VaultType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VaultType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VaultType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DisasterRecovery => serializer.serialize_unit_variant("VaultType", 0u32, "DisasterRecovery"),
                Self::Migrate => serializer.serialize_unit_variant("VaultType", 1u32, "Migrate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Vault model for update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultModelUpdate {
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Vault properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultModelProperties>,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl VaultModelUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workflow model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowModel {
    #[doc = "Workflow model properties."]
    pub properties: WorkflowModelProperties,
    #[doc = "Gets or sets the Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl WorkflowModel {
    pub fn new(properties: WorkflowModelProperties) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Workflow model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowModelCollection {
    #[doc = "Gets or sets the list of workflows."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkflowModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkflowModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkflowModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workflow model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowModelCustomProperties {
    #[doc = "Gets or sets any custom properties of the affected object."]
    #[serde(rename = "affectedObjectDetails", default, skip_serializing_if = "Option::is_none")]
    pub affected_object_details: Option<serde_json::Value>,
}
impl WorkflowModelCustomProperties {
    pub fn new() -> Self {
        Self {
            affected_object_details: None,
        }
    }
}
#[doc = "Gets or sets the instance type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "instanceType")]
pub enum WorkflowModelCustomPropertiesUnion {
    FailoverWorkflowDetails(FailoverWorkflowModelCustomProperties),
    TestFailoverCleanupWorkflowDetails(TestFailoverCleanupWorkflowModelCustomProperties),
    TestFailoverWorkflowDetails(TestFailoverWorkflowModelCustomProperties),
}
#[doc = "Workflow model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowModelProperties {
    #[doc = "Gets or sets the friendly display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the workflow state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<workflow_model_properties::State>,
    #[doc = "Gets or sets the start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the affected object Id."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Gets or sets the affected object name."]
    #[serde(rename = "objectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "Gets or sets the affected object internal Id."]
    #[serde(rename = "objectInternalId", default, skip_serializing_if = "Option::is_none")]
    pub object_internal_id: Option<String>,
    #[doc = "Gets or sets the affected object internal name."]
    #[serde(rename = "objectInternalName", default, skip_serializing_if = "Option::is_none")]
    pub object_internal_name: Option<String>,
    #[doc = "Gets or sets the object type."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<workflow_model_properties::ObjectType>,
    #[doc = "Gets or sets the replication provider."]
    #[serde(rename = "replicationProviderId", default, skip_serializing_if = "Option::is_none")]
    pub replication_provider_id: Option<String>,
    #[doc = "Gets or sets the source fabric provider."]
    #[serde(rename = "sourceFabricProviderId", default, skip_serializing_if = "Option::is_none")]
    pub source_fabric_provider_id: Option<String>,
    #[doc = "Gets or sets the target fabric provider."]
    #[serde(rename = "targetFabricProviderId", default, skip_serializing_if = "Option::is_none")]
    pub target_fabric_provider_id: Option<String>,
    #[doc = "Gets or sets the list of allowed actions on the workflow."]
    #[serde(
        rename = "allowedActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_actions: Vec<String>,
    #[doc = "Gets or sets the workflow activity id."]
    #[serde(rename = "activityId", default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[doc = "Gets or sets the list of tasks."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<TaskModel>,
    #[doc = "Gets or sets the list of errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<ErrorModel>,
    #[doc = "Workflow model custom properties."]
    #[serde(rename = "customProperties")]
    pub custom_properties: WorkflowModelCustomPropertiesUnion,
}
impl WorkflowModelProperties {
    pub fn new(custom_properties: WorkflowModelCustomPropertiesUnion) -> Self {
        Self {
            display_name: None,
            state: None,
            start_time: None,
            end_time: None,
            object_id: None,
            object_name: None,
            object_internal_id: None,
            object_internal_name: None,
            object_type: None,
            replication_provider_id: None,
            source_fabric_provider_id: None,
            target_fabric_provider_id: None,
            allowed_actions: Vec::new(),
            activity_id: None,
            tasks: Vec::new(),
            errors: Vec::new(),
            custom_properties,
        }
    }
}
pub mod workflow_model_properties {
    use super::*;
    #[doc = "Gets or sets the workflow state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Pending,
        Started,
        Cancelling,
        Succeeded,
        Failed,
        Cancelled,
        CompletedWithInformation,
        CompletedWithWarnings,
        CompletedWithErrors,
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
                Self::Pending => serializer.serialize_unit_variant("State", 0u32, "Pending"),
                Self::Started => serializer.serialize_unit_variant("State", 1u32, "Started"),
                Self::Cancelling => serializer.serialize_unit_variant("State", 2u32, "Cancelling"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 4u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("State", 5u32, "Cancelled"),
                Self::CompletedWithInformation => serializer.serialize_unit_variant("State", 6u32, "CompletedWithInformation"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("State", 7u32, "CompletedWithWarnings"),
                Self::CompletedWithErrors => serializer.serialize_unit_variant("State", 8u32, "CompletedWithErrors"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the object type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ObjectType")]
    pub enum ObjectType {
        AvsDiskPool,
        Dra,
        Fabric,
        Policy,
        ProtectedItem,
        RecoveryPlan,
        ReplicationExtension,
        Vault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ObjectType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ObjectType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ObjectType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AvsDiskPool => serializer.serialize_unit_variant("ObjectType", 0u32, "AvsDiskPool"),
                Self::Dra => serializer.serialize_unit_variant("ObjectType", 1u32, "Dra"),
                Self::Fabric => serializer.serialize_unit_variant("ObjectType", 2u32, "Fabric"),
                Self::Policy => serializer.serialize_unit_variant("ObjectType", 3u32, "Policy"),
                Self::ProtectedItem => serializer.serialize_unit_variant("ObjectType", 4u32, "ProtectedItem"),
                Self::RecoveryPlan => serializer.serialize_unit_variant("ObjectType", 5u32, "RecoveryPlan"),
                Self::ReplicationExtension => serializer.serialize_unit_variant("ObjectType", 6u32, "ReplicationExtension"),
                Self::Vault => serializer.serialize_unit_variant("ObjectType", 7u32, "Vault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
