#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Input values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "The name of the service instance to check."]
    pub name: String,
    #[doc = "The fully qualified resource type which includes provider namespace."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl CheckNameAvailabilityParameters {
    pub fn new(name: String, type_: String) -> Self {
        Self { name, type_ }
    }
}
pub type CorsConfigurationHeaderEntry = String;
pub type CorsConfigurationMethodEntry = String;
pub type CorsConfigurationOriginEntry = String;
pub type DicomAudience = String;
#[doc = "The description of Dicom Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DicomService {
    #[serde(flatten)]
    pub tagged_resource: TaggedResource,
    #[serde(flatten)]
    pub service_managed_identity: ServiceManagedIdentity,
    #[doc = "Dicom Service properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DicomServiceProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DicomService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authentication configuration information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DicomServiceAuthenticationConfiguration {
    #[doc = "The authority url for the service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[doc = "The audiences for the service"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audiences: Vec<DicomAudience>,
}
impl DicomServiceAuthenticationConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The collection of Dicom Services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DicomServiceCollection {
    #[doc = "The link used to get the next page of Dicom Services."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of Dicom Services."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DicomService>,
}
impl azure_core::Continuable for DicomServiceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DicomServiceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dicom Service patch properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DicomServicePatchResource {
    #[serde(flatten)]
    pub resource_tags: ResourceTags,
    #[serde(flatten)]
    pub service_managed_identity: ServiceManagedIdentity,
}
impl DicomServicePatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dicom Service properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DicomServiceProperties {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Authentication configuration information"]
    #[serde(rename = "authenticationConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<DicomServiceAuthenticationConfiguration>,
    #[doc = "The url of the Dicom Services."]
    #[serde(rename = "serviceUrl", default, skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    #[doc = "The list of private endpoint connections that are set up for this resource."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<ResourcePublicNetworkAccess>,
}
impl DicomServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetailsInternal>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetailsInternal>,
}
impl azure_core::Continuable for ErrorDetails {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetailsInternal {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorDetailsInternal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Controls how resources are versioned on the FHIR service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FhirResourceVersionPolicy")]
pub enum FhirResourceVersionPolicy {
    #[serde(rename = "no-version")]
    NoVersion,
    #[serde(rename = "versioned")]
    Versioned,
    #[serde(rename = "versioned-update")]
    VersionedUpdate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FhirResourceVersionPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FhirResourceVersionPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FhirResourceVersionPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NoVersion => serializer.serialize_unit_variant("FhirResourceVersionPolicy", 0u32, "no-version"),
            Self::Versioned => serializer.serialize_unit_variant("FhirResourceVersionPolicy", 1u32, "versioned"),
            Self::VersionedUpdate => serializer.serialize_unit_variant("FhirResourceVersionPolicy", 2u32, "versioned-update"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The description of Fhir Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FhirService {
    #[serde(flatten)]
    pub tagged_resource: TaggedResource,
    #[serde(flatten)]
    pub service_managed_identity: ServiceManagedIdentity,
    #[doc = "The kind of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<fhir_service::Kind>,
    #[doc = "Fhir Service properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FhirServiceProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl FhirService {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod fhir_service {
    use super::*;
    #[doc = "The kind of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "fhir-Stu3")]
        FhirStu3,
        #[serde(rename = "fhir-R4")]
        FhirR4,
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
                Self::FhirStu3 => serializer.serialize_unit_variant("Kind", 0u32, "fhir-Stu3"),
                Self::FhirR4 => serializer.serialize_unit_variant("Kind", 1u32, "fhir-R4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type FhirServiceAccessPolicies = Vec<FhirServiceAccessPolicyEntry>;
#[doc = "An access policy entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FhirServiceAccessPolicyEntry {
    #[doc = "An Azure AD object ID (User or Apps) that is allowed access to the FHIR service."]
    #[serde(rename = "objectId")]
    pub object_id: String,
}
impl FhirServiceAccessPolicyEntry {
    pub fn new(object_id: String) -> Self {
        Self { object_id }
    }
}
#[doc = "Azure container registry configuration information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FhirServiceAcrConfiguration {
    #[doc = "The list of the Azure container registry login servers."]
    #[serde(rename = "loginServers", default, skip_serializing_if = "Vec::is_empty")]
    pub login_servers: Vec<String>,
    #[doc = "The list of Open Container Initiative (OCI) artifacts."]
    #[serde(rename = "ociArtifacts", default, skip_serializing_if = "Vec::is_empty")]
    pub oci_artifacts: Vec<ServiceOciArtifactEntry>,
}
impl FhirServiceAcrConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authentication configuration information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FhirServiceAuthenticationConfiguration {
    #[doc = "The authority url for the service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[doc = "The audience url for the service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "If the SMART on FHIR proxy is enabled"]
    #[serde(rename = "smartProxyEnabled", default, skip_serializing_if = "Option::is_none")]
    pub smart_proxy_enabled: Option<bool>,
}
impl FhirServiceAuthenticationConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of Fhir services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FhirServiceCollection {
    #[doc = "The link used to get the next page of Fhir Services."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of Fhir Services."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FhirService>,
}
impl azure_core::Continuable for FhirServiceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FhirServiceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The settings for the CORS configuration of the service instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FhirServiceCorsConfiguration {
    #[doc = "The origins to be allowed via CORS."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<CorsConfigurationOriginEntry>,
    #[doc = "The headers to be allowed via CORS."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<CorsConfigurationHeaderEntry>,
    #[doc = "The methods to be allowed via CORS."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<CorsConfigurationMethodEntry>,
    #[doc = "The max age to be allowed via CORS."]
    #[serde(rename = "maxAge", default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
    #[doc = "If credentials are allowed via CORS."]
    #[serde(rename = "allowCredentials", default, skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
}
impl FhirServiceCorsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Export operation configuration information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FhirServiceExportConfiguration {
    #[doc = "The name of the default export storage account."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
}
impl FhirServiceExportConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "FhirService patch properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FhirServicePatchResource {
    #[serde(flatten)]
    pub resource_tags: ResourceTags,
    #[serde(flatten)]
    pub service_managed_identity: ServiceManagedIdentity,
}
impl FhirServicePatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Fhir Service properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FhirServiceProperties {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The access policies of the service instance."]
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<FhirServiceAccessPolicies>,
    #[doc = "Azure container registry configuration information"]
    #[serde(rename = "acrConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub acr_configuration: Option<FhirServiceAcrConfiguration>,
    #[doc = "Authentication configuration information"]
    #[serde(rename = "authenticationConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<FhirServiceAuthenticationConfiguration>,
    #[doc = "The settings for the CORS configuration of the service instance."]
    #[serde(rename = "corsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<FhirServiceCorsConfiguration>,
    #[doc = "Export operation configuration information"]
    #[serde(rename = "exportConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub export_configuration: Option<FhirServiceExportConfiguration>,
    #[doc = "The list of private endpoint connections that are set up for this resource."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<ResourcePublicNetworkAccess>,
    #[doc = "Indicates the current status of event support for the resource."]
    #[serde(rename = "eventState", default, skip_serializing_if = "Option::is_none")]
    pub event_state: Option<ResourceEventState>,
    #[doc = "The settings for history tracking for FHIR resources."]
    #[serde(rename = "resourceVersionPolicyConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub resource_version_policy_configuration: Option<ResourceVersionPolicyConfiguration>,
}
impl FhirServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IoT Connector definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotConnector {
    #[serde(flatten)]
    pub tagged_resource: TaggedResource,
    #[serde(flatten)]
    pub service_managed_identity: ServiceManagedIdentity,
    #[doc = "IoT Connector properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IotConnectorProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl IotConnector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of IoT Connectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotConnectorCollection {
    #[doc = "The link used to get the next page of IoT Connectors."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of IoT Connectors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotConnector>,
}
impl azure_core::Continuable for IotConnectorCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IotConnectorCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Iot Connector patch properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotConnectorPatchResource {
    #[serde(flatten)]
    pub resource_tags: ResourceTags,
    #[serde(flatten)]
    pub service_managed_identity: ServiceManagedIdentity,
}
impl IotConnectorPatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IoT Connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotConnectorProperties {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Event Hub ingestion endpoint configuration"]
    #[serde(rename = "ingestionEndpointConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_endpoint_configuration: Option<IotEventHubIngestionEndpointConfiguration>,
    #[doc = "The mapping content."]
    #[serde(rename = "deviceMapping", default, skip_serializing_if = "Option::is_none")]
    pub device_mapping: Option<IotMappingProperties>,
}
impl IotConnectorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common IoT Connector destination properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotDestinationProperties {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl IotDestinationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event Hub ingestion endpoint configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotEventHubIngestionEndpointConfiguration {
    #[doc = "Event Hub name to connect to."]
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
    #[doc = "Consumer group of the event hub to connected to."]
    #[serde(rename = "consumerGroup", default, skip_serializing_if = "Option::is_none")]
    pub consumer_group: Option<String>,
    #[doc = "Fully qualified namespace of the Event Hub to connect to."]
    #[serde(rename = "fullyQualifiedEventHubNamespace", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_event_hub_namespace: Option<String>,
}
impl IotEventHubIngestionEndpointConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IoT Connector FHIR destination definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotFhirDestination {
    #[serde(flatten)]
    pub location_based_resource: LocationBasedResource,
    #[doc = "IoT Connector destination properties for an Azure FHIR service."]
    pub properties: IotFhirDestinationProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl IotFhirDestination {
    pub fn new(properties: IotFhirDestinationProperties) -> Self {
        Self {
            location_based_resource: LocationBasedResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "A collection of IoT Connector FHIR destinations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotFhirDestinationCollection {
    #[doc = "The link used to get the next page of IoT FHIR destinations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of IoT Connector FHIR destinations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotFhirDestination>,
}
impl azure_core::Continuable for IotFhirDestinationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IotFhirDestinationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IoT Connector destination properties for an Azure FHIR service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotFhirDestinationProperties {
    #[serde(flatten)]
    pub iot_destination_properties: IotDestinationProperties,
    #[doc = "The type of IoT identity resolution to use with the destination."]
    #[serde(rename = "resourceIdentityResolutionType")]
    pub resource_identity_resolution_type: IotIdentityResolutionType,
    #[doc = "Fully qualified resource id of the FHIR service to connect to."]
    #[serde(rename = "fhirServiceResourceId")]
    pub fhir_service_resource_id: String,
    #[doc = "The mapping content."]
    #[serde(rename = "fhirMapping")]
    pub fhir_mapping: IotMappingProperties,
}
impl IotFhirDestinationProperties {
    pub fn new(
        resource_identity_resolution_type: IotIdentityResolutionType,
        fhir_service_resource_id: String,
        fhir_mapping: IotMappingProperties,
    ) -> Self {
        Self {
            iot_destination_properties: IotDestinationProperties::default(),
            resource_identity_resolution_type,
            fhir_service_resource_id,
            fhir_mapping,
        }
    }
}
#[doc = "The type of IoT identity resolution to use with the destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IotIdentityResolutionType")]
pub enum IotIdentityResolutionType {
    Create,
    Lookup,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IotIdentityResolutionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IotIdentityResolutionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IotIdentityResolutionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Create => serializer.serialize_unit_variant("IotIdentityResolutionType", 0u32, "Create"),
            Self::Lookup => serializer.serialize_unit_variant("IotIdentityResolutionType", 1u32, "Lookup"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The mapping content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotMappingProperties {
    #[doc = "The mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}
impl IotMappingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operations of the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListOperations {
    #[doc = "Collection of available operation details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationDetail>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListOperations {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListOperations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The common properties for any location based resource, tracked or proxy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationBasedResource {
    #[serde(flatten)]
    pub resource_core: ResourceCore,
    #[doc = "The resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl LocationBasedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifications of the Log for Azure Monitoring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of the log"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the log"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blob duration of the log"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifications of the Dimension of metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDimension {
    #[doc = "Name of the dimension"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the dimension"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Whether this dimension should be included for the Shoebox export scenario"]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl MetricDimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifications of the Metrics for Azure Monitoring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Name of the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the metric"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Localized friendly description of the metric"]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Unit that makes sense for the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Name of the metric category that the metric belongs to. A metric can only belong to a single category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Only provide one value for this field. Valid values: Average, Minimum, Maximum, Total, Count."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Supported aggregation types"]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "Supported time grain types"]
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "Optional. If set to true, then zero will be returned for time duration where no metric is emitted/published."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "Dimensions of the metric"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
    #[doc = "Name of the MDM namespace. Optional."]
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDetail {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Default value is 'user,system'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation_detail::ActionType>,
    #[doc = "Extra Operation properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl OperationDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_detail {
    use super::*;
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
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider: Microsoft.HealthcareApis"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource Type: Services"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Friendly description for the operation,"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extra Operation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "Service specification payload"]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties indicating the operation result of an operation on a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultsDescription {
    #[doc = "The ID of the operation returned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the operation result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the operation being performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_results_description::Status>,
    #[doc = "The time that the operation was started."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The time that the operation finished."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Additional properties of the operation result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationResultsDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_results_description {
    use super::*;
    #[doc = "The status of the operation being performed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Canceled,
        Succeeded,
        Failed,
        Requested,
        Running,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Canceled => serializer.serialize_unit_variant("Status", 0u32, "Canceled"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Requested => serializer.serialize_unit_variant("Status", 3u32, "Requested"),
                Self::Running => serializer.serialize_unit_variant("Status", 4u32, "Running"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionDescription {
    #[serde(flatten)]
    pub private_endpoint_connection: PrivateEndpointConnection,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateEndpointConnectionDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connection associated with the specified storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connection associated with the specified storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResultDescription {
    #[doc = "Array of private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnectionDescription>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResultDescription {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateEndpointConnectionListResultDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceDescription {
    #[serde(flatten)]
    pub private_link_resource: PrivateLinkResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateLinkResourceDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResultDescription {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResourceDescription>,
}
impl azure_core::Continuable for PrivateLinkResourceListResultDescription {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateLinkResourceListResultDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Deleting,
    Succeeded,
    Creating,
    Accepted,
    Verifying,
    Updating,
    Failed,
    Canceled,
    Deprovisioned,
    Moving,
    Suspended,
    Warned,
    SystemMaintenance,
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
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Creating"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Accepted"),
            Self::Verifying => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Verifying"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Canceled"),
            Self::Deprovisioned => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Deprovisioned"),
            Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Moving"),
            Self::Suspended => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Suspended"),
            Self::Warned => serializer.serialize_unit_variant("ProvisioningState", 11u32, "Warned"),
            Self::SystemMaintenance => serializer.serialize_unit_variant("ProvisioningState", 12u32, "SystemMaintenance"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "The common properties for any resource, tracked or proxy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceCore {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "An etag associated with the resource, used for optimistic concurrency when editing it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ResourceCore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates the current status of event support for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceEventState")]
pub enum ResourceEventState {
    Disabled,
    Enabled,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceEventState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceEventState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceEventState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("ResourceEventState", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("ResourceEventState", 1u32, "Enabled"),
            Self::Updating => serializer.serialize_unit_variant("ResourceEventState", 2u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourcePublicNetworkAccess")]
pub enum ResourcePublicNetworkAccess {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourcePublicNetworkAccess {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourcePublicNetworkAccess {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourcePublicNetworkAccess {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("ResourcePublicNetworkAccess", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("ResourcePublicNetworkAccess", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of key value pairs that describe the resource. This will overwrite the existing tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTags {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The settings for history tracking for FHIR resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceVersionPolicyConfiguration {
    #[doc = "Controls how resources are versioned on the FHIR service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<FhirResourceVersionPolicy>,
    #[doc = "A list of FHIR Resources and their version policy overrides."]
    #[serde(rename = "resourceTypeOverrides", default, skip_serializing_if = "Option::is_none")]
    pub resource_type_overrides: Option<serde_json::Value>,
}
impl ResourceVersionPolicyConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceAccessPoliciesInfo = Vec<ServiceAccessPolicyEntry>;
#[doc = "An access policy entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccessPolicyEntry {
    #[doc = "An Azure AD object ID (User or Apps) that is allowed access to the FHIR service."]
    #[serde(rename = "objectId")]
    pub object_id: String,
}
impl ServiceAccessPolicyEntry {
    pub fn new(object_id: String) -> Self {
        Self { object_id }
    }
}
#[doc = "Azure container registry configuration information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceAcrConfigurationInfo {
    #[doc = "The list of the ACR login servers."]
    #[serde(rename = "loginServers", default, skip_serializing_if = "Vec::is_empty")]
    pub login_servers: Vec<String>,
    #[doc = "The list of Open Container Initiative (OCI) artifacts."]
    #[serde(rename = "ociArtifacts", default, skip_serializing_if = "Vec::is_empty")]
    pub oci_artifacts: Vec<ServiceOciArtifactEntry>,
}
impl ServiceAcrConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authentication configuration information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceAuthenticationConfigurationInfo {
    #[doc = "The authority url for the service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[doc = "The audience url for the service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "If the SMART on FHIR proxy is enabled"]
    #[serde(rename = "smartProxyEnabled", default, skip_serializing_if = "Option::is_none")]
    pub smart_proxy_enabled: Option<bool>,
}
impl ServiceAuthenticationConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceCorsConfigurationHeaderEntry = String;
#[doc = "The settings for the CORS configuration of the service instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceCorsConfigurationInfo {
    #[doc = "The origins to be allowed via CORS."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<ServiceCorsConfigurationOriginEntry>,
    #[doc = "The headers to be allowed via CORS."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<ServiceCorsConfigurationHeaderEntry>,
    #[doc = "The methods to be allowed via CORS."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<ServiceCorsConfigurationMethodEntry>,
    #[doc = "The max age to be allowed via CORS."]
    #[serde(rename = "maxAge", default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
    #[doc = "If credentials are allowed via CORS."]
    #[serde(rename = "allowCredentials", default, skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
}
impl ServiceCorsConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceCorsConfigurationMethodEntry = String;
pub type ServiceCorsConfigurationOriginEntry = String;
#[doc = "The settings for the Cosmos DB database backing the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceCosmosDbConfigurationInfo {
    #[doc = "The provisioned throughput for the backing database."]
    #[serde(rename = "offerThroughput", default, skip_serializing_if = "Option::is_none")]
    pub offer_throughput: Option<i64>,
    #[doc = "The URI of the customer-managed key for the backing database."]
    #[serde(rename = "keyVaultKeyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_key_uri: Option<String>,
}
impl ServiceCosmosDbConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Export operation configuration information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceExportConfigurationInfo {
    #[doc = "The name of the default export storage account."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
}
impl ServiceExportConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceManagedIdentity {
    #[doc = "Setting indicating whether the service has a managed identity associated with it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<service_managed_identity::Identity>,
}
impl ServiceManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_managed_identity {
    use super::*;
    #[doc = "Setting indicating whether the service has a managed identity associated with it."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Identity {
        #[doc = "Type of identity being specified, currently SystemAssigned and None are allowed."]
        #[serde(rename = "type")]
        pub type_: identity::Type,
        #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
        #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
        pub principal_id: Option<String>,
        #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
        #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
        pub tenant_id: Option<String>,
        #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
        #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
        pub user_assigned_identities: Option<UserAssignedIdentities>,
    }
    impl Identity {
        pub fn new(type_: identity::Type) -> Self {
            Self {
                type_,
                principal_id: None,
                tenant_id: None,
                user_assigned_identities: None,
            }
        }
    }
    pub mod identity {
        use super::*;
        #[doc = "Type of identity being specified, currently SystemAssigned and None are allowed."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Type")]
        pub enum Type {
            None,
            SystemAssigned,
            UserAssigned,
            #[serde(rename = "SystemAssigned,UserAssigned")]
            SystemAssignedUserAssigned,
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
                    Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                    Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                    Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                    Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 3u32, "SystemAssigned,UserAssigned"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "An Open Container Initiative (OCI) artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceOciArtifactEntry {
    #[doc = "The Azure Container Registry login server."]
    #[serde(rename = "loginServer", default, skip_serializing_if = "Option::is_none")]
    pub login_server: Option<String>,
    #[doc = "The artifact name."]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "The artifact digest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}
impl ServiceOciArtifactEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service specification payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Specifications of the Log for Azure Monitoring"]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
    #[doc = "Specifications of the Metrics for Azure Monitoring"]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The description of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesDescription {
    #[serde(flatten)]
    pub services_resource: ServicesResource,
    #[doc = "The properties of a service instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ServicesDescription {
    pub fn new(services_resource: ServicesResource) -> Self {
        Self {
            services_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "A list of service description objects with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesDescriptionListResult {
    #[doc = "The link used to get the next page of service description objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of service description objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServicesDescription>,
}
impl azure_core::Continuable for ServicesDescriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServicesDescriptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties indicating whether a given service name is available."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesNameAvailabilityInfo {
    #[doc = "The value which indicates whether the provided name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason for unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<services_name_availability_info::Reason>,
    #[doc = "The detailed reason message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ServicesNameAvailabilityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod services_name_availability_info {
    use super::*;
    #[doc = "The reason for unavailability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[doc = "The description of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesPatchDescription {
    #[doc = "Instance tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The properties for updating a service instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesPropertiesUpdateParameters>,
}
impl ServicesPatchDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a service instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesProperties {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The access policies of the service instance."]
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<ServiceAccessPoliciesInfo>,
    #[doc = "The settings for the Cosmos DB database backing the service."]
    #[serde(rename = "cosmosDbConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cosmos_db_configuration: Option<ServiceCosmosDbConfigurationInfo>,
    #[doc = "Authentication configuration information"]
    #[serde(rename = "authenticationConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<ServiceAuthenticationConfigurationInfo>,
    #[doc = "The settings for the CORS configuration of the service instance."]
    #[serde(rename = "corsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<ServiceCorsConfigurationInfo>,
    #[doc = "Export operation configuration information"]
    #[serde(rename = "exportConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub export_configuration: Option<ServiceExportConfigurationInfo>,
    #[doc = "The list of private endpoint connections that are set up for this resource."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<services_properties::PublicNetworkAccess>,
    #[doc = "Azure container registry configuration information"]
    #[serde(rename = "acrConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub acr_configuration: Option<ServiceAcrConfigurationInfo>,
}
impl ServicesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod services_properties {
    use super::*;
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties for updating a service instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesPropertiesUpdateParameters {
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<services_properties_update_parameters::PublicNetworkAccess>,
}
impl ServicesPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod services_properties_update_parameters {
    use super::*;
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The common properties of a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesResource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The kind of the service."]
    pub kind: services_resource::Kind,
    #[doc = "The resource location."]
    pub location: String,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An etag associated with the resource, used for optimistic concurrency when editing it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Setting indicating whether the service has a managed identity associated with it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<services_resource::Identity>,
}
impl ServicesResource {
    pub fn new(kind: services_resource::Kind, location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            kind,
            location,
            tags: None,
            etag: None,
            identity: None,
        }
    }
}
pub mod services_resource {
    use super::*;
    #[doc = "The kind of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "fhir")]
        Fhir,
        #[serde(rename = "fhir-Stu3")]
        FhirStu3,
        #[serde(rename = "fhir-R4")]
        FhirR4,
    }
    #[doc = "Setting indicating whether the service has a managed identity associated with it."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Identity {
        #[doc = "The principal ID of the resource identity."]
        #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
        pub principal_id: Option<String>,
        #[doc = "The tenant ID of the resource."]
        #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
        pub tenant_id: Option<String>,
        #[doc = "Type of identity being specified, currently SystemAssigned and None are allowed."]
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
        #[doc = "Type of identity being specified, currently SystemAssigned and None are allowed."]
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
#[doc = "The common properties of tracked resources in the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaggedResource {
    #[serde(flatten)]
    pub resource_tags: ResourceTags,
    #[serde(flatten)]
    pub location_based_resource: LocationBasedResource,
}
impl TaggedResource {
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
#[doc = "Workspace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workspace {
    #[serde(flatten)]
    pub tagged_resource: TaggedResource,
    #[doc = "Workspaces resource specific properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<workspace::Properties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace {
    use super::*;
    #[doc = "Workspaces resource specific properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The provisioning state."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<ProvisioningState>,
        #[doc = "The list of private endpoint connections that are set up for this resource."]
        #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
        pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
        #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
        #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
        pub public_network_access: Option<ResourcePublicNetworkAccess>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of workspace object with a next link"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceList {
    #[doc = "The link used to get the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Collection of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
}
impl azure_core::Continuable for WorkspaceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkspaceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace patch properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePatchResource {
    #[serde(flatten)]
    pub resource_tags: ResourceTags,
}
impl WorkspacePatchResource {
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
