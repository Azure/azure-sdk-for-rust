#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "This type describes properties of an application resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationProperties {
    #[doc = "User readable description of the application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Internal use."]
    #[serde(rename = "debugParams", default, skip_serializing_if = "Option::is_none")]
    pub debug_params: Option<String>,
    #[doc = "describes the services in the application."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<ServiceResourceDescription>,
    #[doc = "The health state of a resource such as Application, Service, or Network."]
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[doc = "When the application's health state is not 'Ok', this additional details from service fabric Health Manager for the user to know why the application is marked unhealthy."]
    #[serde(rename = "unhealthyEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluation: Option<String>,
    #[doc = "Status of the application resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<application_properties::Status>,
    #[doc = "Gives additional information about the current status of the application deployment."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "Names of the services in the application."]
    #[serde(rename = "serviceNames", default, skip_serializing_if = "Vec::is_empty")]
    pub service_names: Vec<String>,
    #[doc = "Describes the diagnostics options available"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsDescription>,
}
impl ApplicationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod application_properties {
    use super::*;
    #[doc = "Status of the application resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Invalid,
        Ready,
        Upgrading,
        Creating,
        Deleting,
        Failed,
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
                Self::Invalid => serializer.serialize_unit_variant("Status", 0u32, "Invalid"),
                Self::Ready => serializer.serialize_unit_variant("Status", 1u32, "Ready"),
                Self::Upgrading => serializer.serialize_unit_variant("Status", 2u32, "Upgrading"),
                Self::Creating => serializer.serialize_unit_variant("Status", 3u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("Status", 4u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("Status", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This type describes an application resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "This type describes properties of an application resource."]
    pub properties: ApplicationResourceProperties,
}
impl ApplicationResourceDescription {
    pub fn new(tracked_resource: TrackedResource, properties: ApplicationResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "A pageable list of application resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResourceDescriptionList {
    #[doc = "One page of the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationResourceDescription>,
    #[doc = "URI to fetch the next page of the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationResourceDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApplicationResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This type describes properties of an application resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub application_properties: ApplicationProperties,
}
impl ApplicationResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An operation available at the listed Azure resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperationDisplay {
    #[doc = "Name of the operation provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource on which the operation is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the available operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the available operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AvailableOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostics settings for Geneva."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureInternalMonitoringPipelineSinkDescription {
    #[serde(flatten)]
    pub diagnostics_sink_properties: DiagnosticsSinkProperties,
    #[doc = "Azure Internal monitoring pipeline account."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Azure Internal monitoring pipeline account namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Azure Internal monitoring agent configuration."]
    #[serde(rename = "maConfigUrl", default, skip_serializing_if = "Option::is_none")]
    pub ma_config_url: Option<String>,
    #[doc = "Azure Internal monitoring agent fluentd configuration."]
    #[serde(rename = "fluentdConfigUrl", default, skip_serializing_if = "Option::is_none")]
    pub fluentd_config_url: Option<serde_json::Value>,
    #[doc = "Azure Internal monitoring pipeline autokey associated with the certificate."]
    #[serde(rename = "autoKeyConfigUrl", default, skip_serializing_if = "Option::is_none")]
    pub auto_key_config_url: Option<String>,
}
impl AzureInternalMonitoringPipelineSinkDescription {
    pub fn new(diagnostics_sink_properties: DiagnosticsSinkProperties) -> Self {
        Self {
            diagnostics_sink_properties,
            account_name: None,
            namespace: None,
            ma_config_url: None,
            fluentd_config_url: None,
            auto_key_config_url: None,
        }
    }
}
#[doc = "Describes a container and its runtime properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerCodePackageProperties {
    #[doc = "The name of the code package."]
    pub name: String,
    #[doc = "The Container image to use."]
    pub image: String,
    #[doc = "Image registry credential."]
    #[serde(rename = "imageRegistryCredential", default, skip_serializing_if = "Option::is_none")]
    pub image_registry_credential: Option<ImageRegistryCredential>,
    #[doc = "Override for the default entry point in the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[doc = "Command array to execute within the container in exec form."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<String>,
    #[doc = "The environment variables to set in this container"]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentVariable>,
    #[doc = "The settings to set in this container. The setting file path can be fetched from environment variable \"Fabric_SettingPath\". The path for Windows container is \"C:\\\\secrets\". The path for Linux container is \"/var/secrets\"."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub settings: Vec<Setting>,
    #[doc = "The labels to set in this container."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<ContainerLabel>,
    #[doc = "The endpoints exposed by this container."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<EndpointProperties>,
    #[doc = "This type describes the resource requirements for a container or a service."]
    pub resources: ResourceRequirements,
    #[doc = "The volumes to be attached to the container."]
    #[serde(rename = "volumeRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_refs: Vec<ContainerVolume>,
    #[doc = "Runtime information of a container instance."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<ContainerInstanceView>,
    #[doc = "Reference to sinks in DiagnosticsDescription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsRef>,
}
impl ContainerCodePackageProperties {
    pub fn new(name: String, image: String, resources: ResourceRequirements) -> Self {
        Self {
            name,
            image,
            image_registry_credential: None,
            entrypoint: None,
            commands: Vec::new(),
            environment_variables: Vec::new(),
            settings: Vec::new(),
            labels: Vec::new(),
            endpoints: Vec::new(),
            resources,
            volume_refs: Vec::new(),
            instance_view: None,
            diagnostics: None,
        }
    }
}
#[doc = "A container event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerEvent {
    #[doc = "The name of the container event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The count of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Date/time of the first event."]
    #[serde(rename = "firstTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    #[doc = "Date/time of the last event."]
    #[serde(rename = "lastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    #[doc = "The event message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The event type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ContainerEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Runtime information of a container instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerInstanceView {
    #[doc = "The number of times the container has been restarted."]
    #[serde(rename = "restartCount", default, skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i64>,
    #[doc = "The container state."]
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<ContainerState>,
    #[doc = "The container state."]
    #[serde(rename = "previousState", default, skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<ContainerState>,
    #[doc = "The events of this container instance."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<ContainerEvent>,
}
impl ContainerInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a container label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerLabel {
    #[doc = "The name of the container label."]
    pub name: String,
    #[doc = "The value of the container label."]
    pub value: String,
}
impl ContainerLabel {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "The logs of the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerLogs {
    #[doc = "content of the log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl ContainerLogs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerState {
    #[doc = "The state of this container"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Date/time when the container state started."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The container exit code."]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<String>,
    #[doc = "Date/time when the container state finished."]
    #[serde(rename = "finishTime", with = "azure_core::date::rfc3339::option")]
    pub finish_time: Option<time::OffsetDateTime>,
    #[doc = "Human-readable status of this state."]
    #[serde(rename = "detailStatus", default, skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
}
impl ContainerState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes how a volume is attached to a container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerVolume {
    #[doc = "Name of the volume."]
    pub name: String,
    #[doc = "The flag indicating whether the volume is read only. Default is 'false'."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "The path within the container at which the volume should be mounted. Only valid path characters are allowed."]
    #[serde(rename = "destinationPath")]
    pub destination_path: String,
}
impl ContainerVolume {
    pub fn new(name: String, destination_path: String) -> Self {
        Self {
            name,
            read_only: None,
            destination_path,
        }
    }
}
#[doc = "Describes the diagnostics options available"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticsDescription {
    #[doc = "List of supported sinks that can be referenced."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sinks: Vec<DiagnosticsSinkProperties>,
    #[doc = "Status of whether or not sinks are enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The sinks to be used if diagnostics is enabled. Sink choices can be overridden at the service and code package level."]
    #[serde(rename = "defaultSinkRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub default_sink_refs: Vec<String>,
}
impl DiagnosticsDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to sinks in DiagnosticsDescription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticsRef {
    #[doc = "Status of whether or not sinks are enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "List of sinks to be used if enabled. References the list of sinks in DiagnosticsDescription."]
    #[serde(rename = "sinkRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub sink_refs: Vec<String>,
}
impl DiagnosticsRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of DiagnosticsSink."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiagnosticsSinkKind")]
pub enum DiagnosticsSinkKind {
    Invalid,
    AzureInternalMonitoringPipeline,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiagnosticsSinkKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiagnosticsSinkKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiagnosticsSinkKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("DiagnosticsSinkKind", 0u32, "Invalid"),
            Self::AzureInternalMonitoringPipeline => {
                serializer.serialize_unit_variant("DiagnosticsSinkKind", 1u32, "AzureInternalMonitoringPipeline")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties of a DiagnosticsSink."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsSinkProperties {
    #[doc = "The kind of DiagnosticsSink."]
    pub kind: DiagnosticsSinkKind,
    #[doc = "Name of the sink. This value is referenced by DiagnosticsReferenceDescription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A description of the sink."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl DiagnosticsSinkProperties {
    pub fn new(kind: DiagnosticsSinkKind) -> Self {
        Self {
            kind,
            name: None,
            description: None,
        }
    }
}
#[doc = "Describes a container endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointProperties {
    #[doc = "The name of the endpoint."]
    pub name: String,
    #[doc = "Port used by the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}
impl EndpointProperties {
    pub fn new(name: String) -> Self {
        Self { name, port: None }
    }
}
#[doc = "Describes an environment variable for the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentVariable {
    #[doc = "The name of the environment variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the environment variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl EnvironmentVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error model details information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetailsModel {
    pub code: String,
    #[doc = "Error message."]
    pub message: String,
}
impl ErrorDetailsModel {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "Error model information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorErrorModel {
    pub code: String,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<String>,
    #[doc = "List of error message details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetailsModel>,
}
impl ErrorErrorModel {
    pub fn new(code: String) -> Self {
        Self {
            code,
            message: None,
            inner_error: None,
            details: Vec::new(),
        }
    }
}
#[doc = "The error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorModel {
    #[doc = "Error model information"]
    pub error: ErrorErrorModel,
}
impl azure_core::Continuable for ErrorModel {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorModel {
    pub fn new(error: ErrorErrorModel) -> Self {
        Self { error }
    }
}
#[doc = "The health state of a resource such as Application, Service, or Network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthState")]
pub enum HealthState {
    Invalid,
    Ok,
    Warning,
    Error,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("HealthState", 0u32, "Invalid"),
            Self::Ok => serializer.serialize_unit_variant("HealthState", 1u32, "Ok"),
            Self::Warning => serializer.serialize_unit_variant("HealthState", 2u32, "Warning"),
            Self::Error => serializer.serialize_unit_variant("HealthState", 3u32, "Error"),
            Self::Unknown => serializer.serialize_unit_variant("HealthState", 4u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Image registry credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRegistryCredential {
    #[doc = "Docker image registry server, without protocol such as `http` and `https`."]
    pub server: String,
    #[doc = "The username for the private registry."]
    pub username: String,
    #[doc = "The password for the private registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ImageRegistryCredential {
    pub fn new(server: String, username: String) -> Self {
        Self {
            server,
            username,
            password: None,
        }
    }
}
#[doc = "Describes public connectivity configuration for the network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngressConfig {
    #[doc = "The QoS tier for ingress."]
    #[serde(rename = "qosLevel", default, skip_serializing_if = "Option::is_none")]
    pub qos_level: Option<ingress_config::QosLevel>,
    #[doc = "Configuration for layer4 public connectivity for this network."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layer4: Vec<Layer4IngressConfig>,
    #[doc = "The public IP address for reaching this network."]
    #[serde(rename = "publicIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
}
impl IngressConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ingress_config {
    use super::*;
    #[doc = "The QoS tier for ingress."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "QosLevel")]
    pub enum QosLevel {
        Bronze,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for QosLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for QosLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for QosLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Bronze => serializer.serialize_unit_variant("QosLevel", 0u32, "Bronze"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the layer4 configuration for public connectivity for this network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Layer4IngressConfig {
    #[doc = "Layer4 ingress config name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the public port at which the service endpoint below needs to be exposed."]
    #[serde(rename = "publicPort", default, skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i64>,
    #[doc = "The application name which contains the service to be exposed."]
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[doc = "The service whose endpoint needs to be exposed at the public port."]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[doc = "The service endpoint that needs to be exposed."]
    #[serde(rename = "endpointName", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
}
impl Layer4IngressConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for Azure Resource Manager proxy resource. It will have everything other than required location and tags. This proxy resource is explicitly created or updated by including it in the parent resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedProxyResource {
    #[doc = "Fully qualified identifier for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Compute/virtualMachines or Microsoft.Storage/storageAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ManagedProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProperties {
    #[doc = "User readable description of the network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "the address prefix for this network."]
    #[serde(rename = "addressPrefix")]
    pub address_prefix: String,
    #[doc = "Describes public connectivity configuration for the network."]
    #[serde(rename = "ingressConfig", default, skip_serializing_if = "Option::is_none")]
    pub ingress_config: Option<IngressConfig>,
}
impl NetworkProperties {
    pub fn new(address_prefix: String) -> Self {
        Self {
            description: None,
            address_prefix,
            ingress_config: None,
        }
    }
}
#[doc = "Describes a network reference in a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRef {
    #[doc = "Name of the network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl NetworkRef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This type describes a network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes properties of a network resource."]
    pub properties: NetworkResourceProperties,
}
impl NetworkResourceDescription {
    pub fn new(tracked_resource: TrackedResource, properties: NetworkResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "A pageable list of network resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkResourceDescriptionList {
    #[doc = "One page of the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkResourceDescription>,
    #[doc = "URI to fetch the next page of the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkResourceDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of a network resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub network_properties: NetworkProperties,
}
impl NetworkResourceProperties {
    pub fn new(network_properties: NetworkProperties) -> Self {
        Self {
            provisioned_resource_properties: ProvisionedResourceProperties::default(),
            network_properties,
        }
    }
}
#[doc = "Describes the result of the request to list Service Fabric operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Service Fabric operations supported by the Microsoft.ServiceFabric resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResult>,
    #[doc = "URL to get the next set of operation list results if there are any."]
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
#[doc = "List of operations available at the listed Azure resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "The name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An operation available at the listed Azure resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableOperationDisplay>,
    #[doc = "Origin result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes common properties of a provisioned resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionedResourceProperties {
    #[doc = "State of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ProvisionedResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for Azure Resource Manager proxy resource. It will have everything other than required location and tags."]
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
#[doc = "The resource model definition for Azure Resource Manager resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified identifier for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Compute/virtualMachines or Microsoft.Storage/storageAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This type describes the resource limits for a given container. It describes the most amount of resources a container is allowed to use before being restarted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceLimits {
    #[doc = "The memory limit in GB."]
    #[serde(rename = "memoryInGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_gb: Option<f64>,
    #[doc = "CPU limits in cores. At present, only full cores are supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
}
impl ResourceLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This type describes the requested resources for a given container. It describes the least amount of resources required for the container. A container can consume more than requested resources up to the specified limits before being restarted. Currently, the requested resources are treated as limits.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequests {
    #[doc = "The memory request in GB for this container."]
    #[serde(rename = "memoryInGB")]
    pub memory_in_gb: f64,
    #[doc = "Requested number of CPU cores. At present, only full cores are supported."]
    pub cpu: f64,
}
impl ResourceRequests {
    pub fn new(memory_in_gb: f64, cpu: f64) -> Self {
        Self { memory_in_gb, cpu }
    }
}
#[doc = "This type describes the resource requirements for a container or a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirements {
    #[doc = "This type describes the requested resources for a given container. It describes the least amount of resources required for the container. A container can consume more than requested resources up to the specified limits before being restarted. Currently, the requested resources are treated as limits.\n"]
    pub requests: ResourceRequests,
    #[doc = "This type describes the resource limits for a given container. It describes the most amount of resources a container is allowed to use before being restarted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceLimits>,
}
impl ResourceRequirements {
    pub fn new(requests: ResourceRequests) -> Self {
        Self { requests, limits: None }
    }
}
#[doc = "A pageable list of all services in an application.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceList {
    #[doc = "One page of the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceResourceDescription>,
    #[doc = "URI to fetch the next page of the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServiceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This type describes a replica of a service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReplicaDescription {
    #[serde(flatten)]
    pub service_replica_properties: ServiceReplicaProperties,
    #[doc = "Name of the replica."]
    #[serde(rename = "replicaName", default, skip_serializing_if = "Option::is_none")]
    pub replica_name: Option<String>,
}
impl ServiceReplicaDescription {
    pub fn new(service_replica_properties: ServiceReplicaProperties) -> Self {
        Self {
            service_replica_properties,
            replica_name: None,
        }
    }
}
#[doc = "A pageable list of replicas of a service resource.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceReplicaList {
    #[doc = "One page of the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceReplicaDescription>,
    #[doc = "URI to fetch the next page of the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceReplicaList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServiceReplicaList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a service replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReplicaProperties {
    #[doc = "The Operating system type required by the code in service.\n"]
    #[serde(rename = "osType")]
    pub os_type: service_replica_properties::OsType,
    #[doc = "Describes the set of code packages that forms the service. A code package describes the container and the properties for running it. All the code packages are started together on the same host and share the same context (network, process etc.).\n"]
    #[serde(rename = "codePackages")]
    pub code_packages: Vec<ContainerCodePackageProperties>,
    #[doc = "The names of the private networks that this service needs to be part of."]
    #[serde(rename = "networkRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub network_refs: Vec<NetworkRef>,
    #[doc = "Reference to sinks in DiagnosticsDescription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsRef>,
}
impl ServiceReplicaProperties {
    pub fn new(os_type: service_replica_properties::OsType, code_packages: Vec<ContainerCodePackageProperties>) -> Self {
        Self {
            os_type,
            code_packages,
            network_refs: Vec::new(),
            diagnostics: None,
        }
    }
}
pub mod service_replica_properties {
    use super::*;
    #[doc = "The Operating system type required by the code in service.\n"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Linux,
        Windows,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OsType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OsType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OsType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Linux => serializer.serialize_unit_variant("OsType", 0u32, "Linux"),
                Self::Windows => serializer.serialize_unit_variant("OsType", 1u32, "Windows"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This type describes a service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceDescription {
    #[serde(flatten)]
    pub managed_proxy_resource: ManagedProxyResource,
    #[doc = "This type describes properties of a service resource."]
    pub properties: ServiceResourceProperties,
}
impl ServiceResourceDescription {
    pub fn new(properties: ServiceResourceProperties) -> Self {
        Self {
            managed_proxy_resource: ManagedProxyResource::default(),
            properties,
        }
    }
}
#[doc = "This type describes properties of a service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceProperties {
    #[serde(flatten)]
    pub service_replica_properties: ServiceReplicaProperties,
    #[doc = "User readable description of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The number of replicas of the service to create. Defaults to 1 if not specified."]
    #[serde(rename = "replicaCount", default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i64>,
    #[doc = "The health state of a resource such as Application, Service, or Network."]
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[doc = "Represents the status of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<service_resource_properties::Status>,
}
impl ServiceResourceProperties {
    pub fn new(service_replica_properties: ServiceReplicaProperties) -> Self {
        Self {
            service_replica_properties,
            description: None,
            replica_count: None,
            health_state: None,
            status: None,
        }
    }
}
pub mod service_resource_properties {
    use super::*;
    #[doc = "Represents the status of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        Active,
        Upgrading,
        Deleting,
        Creating,
        Failed,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::Active => serializer.serialize_unit_variant("Status", 1u32, "Active"),
                Self::Upgrading => serializer.serialize_unit_variant("Status", 2u32, "Upgrading"),
                Self::Deleting => serializer.serialize_unit_variant("Status", 3u32, "Deleting"),
                Self::Creating => serializer.serialize_unit_variant("Status", 4u32, "Creating"),
                Self::Failed => serializer.serialize_unit_variant("Status", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a setting for the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Setting {
    #[doc = "The name of the setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Setting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for Azure Resource Manager tracked top-level resource."]
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
#[doc = "This type describes properties of a volume resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[doc = "User readable description of the volume."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Provider of the volume."]
    pub provider: volume_properties::Provider,
    #[doc = "This type describes a volume provided by an Azure Files file share."]
    #[serde(rename = "azureFileParameters", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_parameters: Option<VolumeProviderParametersAzureFile>,
}
impl VolumeProperties {
    pub fn new(provider: volume_properties::Provider) -> Self {
        Self {
            description: None,
            provider,
            azure_file_parameters: None,
        }
    }
}
pub mod volume_properties {
    use super::*;
    #[doc = "Provider of the volume."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Provider")]
    pub enum Provider {
        #[serde(rename = "SFAzureFile")]
        SfAzureFile,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Provider {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Provider {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Provider {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SfAzureFile => serializer.serialize_unit_variant("Provider", 0u32, "SFAzureFile"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This type describes a volume provided by an Azure Files file share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProviderParametersAzureFile {
    #[doc = "Name of the Azure storage account for the File Share."]
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[doc = "Access key of the Azure storage account for the File Share."]
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[doc = "Name of the Azure Files file share that provides storage for the volume."]
    #[serde(rename = "shareName")]
    pub share_name: String,
}
impl VolumeProviderParametersAzureFile {
    pub fn new(account_name: String, share_name: String) -> Self {
        Self {
            account_name,
            account_key: None,
            share_name,
        }
    }
}
#[doc = "This type describes a volume resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes properties of a volume resource."]
    pub properties: VolumeResourceProperties,
}
impl VolumeResourceDescription {
    pub fn new(tracked_resource: TrackedResource, properties: VolumeResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "A pageable list of volume resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeResourceDescriptionList {
    #[doc = "One page of the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VolumeResourceDescription>,
    #[doc = "URI to fetch the next page of the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VolumeResourceDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VolumeResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of a volume resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub volume_properties: VolumeProperties,
}
impl VolumeResourceProperties {
    pub fn new(volume_properties: VolumeProperties) -> Self {
        Self {
            provisioned_resource_properties: ProvisionedResourceProperties::default(),
            volume_properties,
        }
    }
}
