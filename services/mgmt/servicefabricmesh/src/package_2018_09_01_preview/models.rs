#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Describes the horizontal auto scaling mechanism that adds or removes replicas (containers or container groups)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddRemoveReplicaScalingMechanism {
    #[serde(flatten)]
    pub auto_scaling_mechanism: AutoScalingMechanism,
    #[doc = "Minimum number of containers (scale down won't be performed below this number)."]
    #[serde(rename = "minCount")]
    pub min_count: i64,
    #[doc = "Maximum number of containers (scale up won't be performed above this number)."]
    #[serde(rename = "maxCount")]
    pub max_count: i64,
    #[doc = "Each time auto scaling is performed, this number of containers will be added or removed."]
    #[serde(rename = "scaleIncrement")]
    pub scale_increment: i64,
}
impl AddRemoveReplicaScalingMechanism {
    pub fn new(auto_scaling_mechanism: AutoScalingMechanism, min_count: i64, max_count: i64, scale_increment: i64) -> Self {
        Self {
            auto_scaling_mechanism,
            min_count,
            max_count,
            scale_increment,
        }
    }
}
#[doc = "Describes properties of a application resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationProperties {
    #[doc = "User readable description of the application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Describes the services in the application. This property is used to create or modify services of the application. On get only the name of the service is returned. The service description can be obtained by querying for the service resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<ServiceResourceDescription>,
    #[doc = "Describes the diagnostics options available"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsDescription>,
    #[doc = "Internal - used by Visual Studio to setup the debugging session on the local development environment."]
    #[serde(rename = "debugParams", default, skip_serializing_if = "Option::is_none")]
    pub debug_params: Option<String>,
    #[doc = "Names of the services in the application."]
    #[serde(rename = "serviceNames", default, skip_serializing_if = "Vec::is_empty")]
    pub service_names: Vec<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[doc = "Gives additional information about the current status of the application."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[doc = "When the application's health state is not 'Ok', this additional details from service fabric Health Manager for the user to know why the application is marked unhealthy."]
    #[serde(rename = "unhealthyEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluation: Option<String>,
}
impl ApplicationProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Describes a volume whose lifetime is scoped to the application's lifetime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationScopedVolume {
    #[serde(flatten)]
    pub volume_reference: VolumeReference,
    #[doc = "Describes parameters for creating application-scoped volumes."]
    #[serde(rename = "creationParameters")]
    pub creation_parameters: ApplicationScopedVolumeCreationParameters,
}
impl ApplicationScopedVolume {
    pub fn new(volume_reference: VolumeReference, creation_parameters: ApplicationScopedVolumeCreationParameters) -> Self {
        Self {
            volume_reference,
            creation_parameters,
        }
    }
}
#[doc = "Describes parameters for creating application-scoped volumes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationScopedVolumeCreationParameters {
    #[doc = "Specifies the application-scoped volume kind."]
    pub kind: ApplicationScopedVolumeKind,
    #[doc = "User readable description of the volume."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ApplicationScopedVolumeCreationParameters {
    pub fn new(kind: ApplicationScopedVolumeKind) -> Self {
        Self { kind, description: None }
    }
}
#[doc = "Describes parameters for creating application-scoped volumes provided by Service Fabric Volume Disks"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk {
    #[serde(flatten)]
    pub application_scoped_volume_creation_parameters: ApplicationScopedVolumeCreationParameters,
    #[doc = "Volume size"]
    #[serde(rename = "sizeDisk")]
    pub size_disk: application_scoped_volume_creation_parameters_service_fabric_volume_disk::SizeDisk,
}
impl ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk {
    pub fn new(
        application_scoped_volume_creation_parameters: ApplicationScopedVolumeCreationParameters,
        size_disk: application_scoped_volume_creation_parameters_service_fabric_volume_disk::SizeDisk,
    ) -> Self {
        Self {
            application_scoped_volume_creation_parameters,
            size_disk,
        }
    }
}
pub mod application_scoped_volume_creation_parameters_service_fabric_volume_disk {
    use super::*;
    #[doc = "Volume size"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SizeDisk")]
    pub enum SizeDisk {
        Small,
        Medium,
        Large,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SizeDisk {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SizeDisk {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SizeDisk {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Small => serializer.serialize_unit_variant("SizeDisk", 0u32, "Small"),
                Self::Medium => serializer.serialize_unit_variant("SizeDisk", 1u32, "Medium"),
                Self::Large => serializer.serialize_unit_variant("SizeDisk", 2u32, "Large"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the application-scoped volume kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationScopedVolumeKind")]
pub enum ApplicationScopedVolumeKind {
    ServiceFabricVolumeDisk,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationScopedVolumeKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationScopedVolumeKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationScopedVolumeKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ServiceFabricVolumeDisk => {
                serializer.serialize_unit_variant("ApplicationScopedVolumeKind", 0u32, "ServiceFabricVolumeDisk")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the mechanism for performing auto scaling operation. Derived classes will describe the actual mechanism."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingMechanism {
    #[doc = "Enumerates the mechanisms for auto scaling."]
    pub kind: AutoScalingMechanismKind,
}
impl AutoScalingMechanism {
    pub fn new(kind: AutoScalingMechanismKind) -> Self {
        Self { kind }
    }
}
#[doc = "Enumerates the mechanisms for auto scaling."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoScalingMechanismKind")]
pub enum AutoScalingMechanismKind {
    AddRemoveReplica,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoScalingMechanismKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoScalingMechanismKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoScalingMechanismKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AddRemoveReplica => serializer.serialize_unit_variant("AutoScalingMechanismKind", 0u32, "AddRemoveReplica"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the metric that is used for triggering auto scaling operation. Derived classes will describe resources or metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingMetric {
    #[doc = "Enumerates the metrics that are used for triggering auto scaling."]
    pub kind: AutoScalingMetricKind,
}
impl AutoScalingMetric {
    pub fn new(kind: AutoScalingMetricKind) -> Self {
        Self { kind }
    }
}
#[doc = "Enumerates the metrics that are used for triggering auto scaling."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoScalingMetricKind")]
pub enum AutoScalingMetricKind {
    Resource,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoScalingMetricKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoScalingMetricKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoScalingMetricKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Resource => serializer.serialize_unit_variant("AutoScalingMetricKind", 0u32, "Resource"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the auto scaling policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingPolicy {
    #[doc = "The name of the auto scaling policy."]
    pub name: String,
    #[doc = "Describes the trigger for performing auto scaling operation."]
    pub trigger: AutoScalingTrigger,
    #[doc = "Describes the mechanism for performing auto scaling operation. Derived classes will describe the actual mechanism."]
    pub mechanism: AutoScalingMechanism,
}
impl AutoScalingPolicy {
    pub fn new(name: String, trigger: AutoScalingTrigger, mechanism: AutoScalingMechanism) -> Self {
        Self { name, trigger, mechanism }
    }
}
#[doc = "Describes the resource that is used for triggering auto scaling."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingResourceMetric {
    #[serde(flatten)]
    pub auto_scaling_metric: AutoScalingMetric,
    #[doc = "Enumerates the resources that are used for triggering auto scaling."]
    pub name: AutoScalingResourceMetricName,
}
impl AutoScalingResourceMetric {
    pub fn new(auto_scaling_metric: AutoScalingMetric, name: AutoScalingResourceMetricName) -> Self {
        Self { auto_scaling_metric, name }
    }
}
#[doc = "Enumerates the resources that are used for triggering auto scaling."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoScalingResourceMetricName")]
pub enum AutoScalingResourceMetricName {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "memoryInGB")]
    MemoryInGb,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoScalingResourceMetricName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoScalingResourceMetricName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoScalingResourceMetricName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Cpu => serializer.serialize_unit_variant("AutoScalingResourceMetricName", 0u32, "cpu"),
            Self::MemoryInGb => serializer.serialize_unit_variant("AutoScalingResourceMetricName", 1u32, "memoryInGB"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the trigger for performing auto scaling operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingTrigger {
    #[doc = "Enumerates the triggers for auto scaling."]
    pub kind: AutoScalingTriggerKind,
}
impl AutoScalingTrigger {
    pub fn new(kind: AutoScalingTriggerKind) -> Self {
        Self { kind }
    }
}
#[doc = "Enumerates the triggers for auto scaling."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoScalingTriggerKind")]
pub enum AutoScalingTriggerKind {
    AverageLoad,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoScalingTriggerKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoScalingTriggerKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoScalingTriggerKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AverageLoad => serializer.serialize_unit_variant("AutoScalingTriggerKind", 0u32, "AverageLoad"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Describes the average load trigger used for auto scaling."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AverageLoadScalingTrigger {
    #[serde(flatten)]
    pub auto_scaling_trigger: AutoScalingTrigger,
    #[doc = "Describes the metric that is used for triggering auto scaling operation. Derived classes will describe resources or metrics."]
    pub metric: AutoScalingMetric,
    #[doc = "Lower load threshold (if average load is below this threshold, service will scale down)."]
    #[serde(rename = "lowerLoadThreshold")]
    pub lower_load_threshold: f64,
    #[doc = "Upper load threshold (if average load is above this threshold, service will scale up)."]
    #[serde(rename = "upperLoadThreshold")]
    pub upper_load_threshold: f64,
    #[doc = "Scale interval that indicates how often will this trigger be checked."]
    #[serde(rename = "scaleIntervalInSeconds")]
    pub scale_interval_in_seconds: i64,
}
impl AverageLoadScalingTrigger {
    pub fn new(
        auto_scaling_trigger: AutoScalingTrigger,
        metric: AutoScalingMetric,
        lower_load_threshold: f64,
        upper_load_threshold: f64,
        scale_interval_in_seconds: i64,
    ) -> Self {
        Self {
            auto_scaling_trigger,
            metric,
            lower_load_threshold,
            upper_load_threshold,
            scale_interval_in_seconds,
        }
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
    #[doc = "Volumes to be attached to the container. The lifetime of these volumes is independent of the application's lifetime."]
    #[serde(rename = "volumeRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_refs: Vec<VolumeReference>,
    #[doc = "Volumes to be attached to the container. The lifetime of these volumes is scoped to the application's lifetime."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<ApplicationScopedVolume>,
    #[doc = "Reference to sinks in DiagnosticsDescription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsRef>,
    #[doc = "A list of ReliableCollection resources used by this particular code package. Please refer to ReliableCollectionsRef for more details."]
    #[serde(rename = "reliableCollectionsRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub reliable_collections_refs: Vec<ReliableCollectionsRef>,
    #[doc = "Runtime information of a container instance."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<ContainerInstanceView>,
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
            volumes: Vec::new(),
            diagnostics: None,
            reliable_collections_refs: Vec::new(),
            instance_view: None,
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
#[doc = "Container logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerLogs {
    #[doc = "Container logs."]
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
#[doc = "Describes a reference to a service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointRef {
    #[doc = "Name of the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl EndpointRef {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Describes destination endpoint for routing traffic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayDestination {
    #[doc = "Name of the service fabric Mesh application."]
    #[serde(rename = "applicationName")]
    pub application_name: String,
    #[doc = "service that contains the endpoint."]
    #[serde(rename = "serviceName")]
    pub service_name: String,
    #[doc = "name of the endpoint in the service."]
    #[serde(rename = "endpointName")]
    pub endpoint_name: String,
}
impl GatewayDestination {
    pub fn new(application_name: String, service_name: String, endpoint_name: String) -> Self {
        Self {
            application_name,
            service_name,
            endpoint_name,
        }
    }
}
#[doc = "Describes properties of a gateway resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayProperties {
    #[doc = "User readable description of the gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Describes a network reference in a service."]
    #[serde(rename = "sourceNetwork")]
    pub source_network: NetworkRef,
    #[doc = "Describes a network reference in a service."]
    #[serde(rename = "destinationNetwork")]
    pub destination_network: NetworkRef,
    #[doc = "Configuration for tcp connectivity for this gateway."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tcp: Vec<TcpConfig>,
    #[doc = "Configuration for http connectivity for this gateway."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub http: Vec<HttpConfig>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[doc = "Gives additional information about the current status of the gateway."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "IP address of the gateway. This is populated in the response and is ignored for incoming requests."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl GatewayProperties {
    pub fn new(source_network: NetworkRef, destination_network: NetworkRef) -> Self {
        Self {
            description: None,
            source_network,
            destination_network,
            tcp: Vec::new(),
            http: Vec::new(),
            status: None,
            status_details: None,
            ip_address: None,
        }
    }
}
#[doc = "This type describes a gateway resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "This type describes properties of a gateway resource."]
    pub properties: GatewayResourceProperties,
}
impl GatewayResourceDescription {
    pub fn new(tracked_resource: TrackedResource, properties: GatewayResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "A pageable list of gateway resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayResourceDescriptionList {
    #[doc = "One page of the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GatewayResourceDescription>,
    #[doc = "URI to fetch the next page of the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GatewayResourceDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GatewayResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This type describes properties of a gateway resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub gateway_properties: GatewayProperties,
}
impl GatewayResourceProperties {
    pub fn new(gateway_properties: GatewayProperties) -> Self {
        Self {
            provisioned_resource_properties: ProvisionedResourceProperties::default(),
            gateway_properties,
        }
    }
}
#[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
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
#[doc = "Describes the http configuration for external connectivity for this network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpConfig {
    #[doc = "http gateway config name."]
    pub name: String,
    #[doc = "Specifies the port at which the service endpoint below needs to be exposed."]
    pub port: i64,
    #[doc = "description for routing."]
    pub hosts: Vec<HttpHostConfig>,
}
impl HttpConfig {
    pub fn new(name: String, port: i64, hosts: Vec<HttpHostConfig>) -> Self {
        Self { name, port, hosts }
    }
}
#[doc = "Describes the hostname properties for http routing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpHostConfig {
    #[doc = "http hostname config name."]
    pub name: String,
    #[doc = "Route information to use for routing. Routes are processed in the order they are specified. Specify routes that are more specific before routes that can handle general cases."]
    pub routes: Vec<HttpRouteConfig>,
}
impl HttpHostConfig {
    pub fn new(name: String, routes: Vec<HttpRouteConfig>) -> Self {
        Self { name, routes }
    }
}
#[doc = "Describes the hostname properties for http routing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteConfig {
    #[doc = "http route name."]
    pub name: String,
    #[doc = "Describes a rule for http route matching."]
    #[serde(rename = "match")]
    pub match_: HttpRouteMatchRule,
    #[doc = "Describes destination endpoint for routing traffic."]
    pub destination: GatewayDestination,
}
impl HttpRouteConfig {
    pub fn new(name: String, match_: HttpRouteMatchRule, destination: GatewayDestination) -> Self {
        Self { name, match_, destination }
    }
}
#[doc = "Describes header information for http route matching."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteMatchHeader {
    #[doc = "Name of header to match in request."]
    pub name: String,
    #[doc = "Value of header to match in request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "how to match header value"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<http_route_match_header::Type>,
}
impl HttpRouteMatchHeader {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            type_: None,
        }
    }
}
pub mod http_route_match_header {
    use super::*;
    #[doc = "how to match header value"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "exact")]
        Exact,
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
                Self::Exact => serializer.serialize_unit_variant("Type", 0u32, "exact"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Path to match for routing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteMatchPath {
    #[doc = "Uri path to match for request."]
    pub value: String,
    #[doc = "replacement string for matched part of the Uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<String>,
    #[doc = "how to match value in the Uri"]
    #[serde(rename = "type")]
    pub type_: http_route_match_path::Type,
}
impl HttpRouteMatchPath {
    pub fn new(value: String, type_: http_route_match_path::Type) -> Self {
        Self {
            value,
            rewrite: None,
            type_,
        }
    }
}
pub mod http_route_match_path {
    use super::*;
    #[doc = "how to match value in the Uri"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "prefix")]
        Prefix,
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
                Self::Prefix => serializer.serialize_unit_variant("Type", 0u32, "prefix"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a rule for http route matching."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteMatchRule {
    #[doc = "Path to match for routing."]
    pub path: HttpRouteMatchPath,
    #[doc = "headers and their values to match in request."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<HttpRouteMatchHeader>,
}
impl HttpRouteMatchRule {
    pub fn new(path: HttpRouteMatchPath) -> Self {
        Self { path, headers: Vec::new() }
    }
}
#[doc = "Image registry credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRegistryCredential {
    #[doc = "Docker image registry server, without protocol such as `http` and `https`."]
    pub server: String,
    #[doc = "The username for the private registry."]
    pub username: String,
    #[doc = "The password for the private registry. The password is required for create or update operations, however it is not returned in the get or list operations."]
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
#[doc = "Describes the properties of a secret resource whose value is provided explicitly as plaintext. The secret resource may have multiple values, each being uniquely versioned. The secret value of each version is stored encrypted, and delivered as plaintext into the context of applications referencing it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlinedValueSecretResourceProperties {
    #[serde(flatten)]
    pub secret_resource_properties: SecretResourceProperties,
}
impl InlinedValueSecretResourceProperties {
    pub fn new(secret_resource_properties: SecretResourceProperties) -> Self {
        Self {
            secret_resource_properties,
        }
    }
}
#[doc = "Information about a Service Fabric container network local to a single Service Fabric cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalNetworkResourceProperties {
    #[serde(flatten)]
    pub network_resource_properties: NetworkResourceProperties,
    #[doc = "Address space for a container network. This is expressed in CIDR notation."]
    #[serde(rename = "networkAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub network_address_prefix: Option<NetworkAddressPrefix>,
}
impl LocalNetworkResourceProperties {
    pub fn new(network_resource_properties: NetworkResourceProperties) -> Self {
        Self {
            network_resource_properties,
            network_address_prefix: None,
        }
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
pub type NetworkAddressPrefix = String;
#[doc = "The type of a Service Fabric container network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkKind")]
pub enum NetworkKind {
    Local,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Local => serializer.serialize_unit_variant("NetworkKind", 0u32, "Local"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes a network reference in a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRef {
    #[doc = "Name of the network"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A list of endpoints that are exposed on this network."]
    #[serde(rename = "endpointRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_refs: Vec<EndpointRef>,
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
    pub network_resource_properties_base: NetworkResourcePropertiesBase,
    #[doc = "User readable description of the network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[doc = "Gives additional information about the current status of the network."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}
impl NetworkResourceProperties {
    pub fn new(network_resource_properties_base: NetworkResourcePropertiesBase) -> Self {
        Self {
            network_resource_properties_base,
            description: None,
            status: None,
            status_details: None,
        }
    }
}
#[doc = "This type describes the properties of a network resource, including its kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourcePropertiesBase {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[doc = "The type of a Service Fabric container network."]
    pub kind: NetworkKind,
}
impl NetworkResourcePropertiesBase {
    pub fn new(kind: NetworkKind) -> Self {
        Self {
            provisioned_resource_properties: ProvisionedResourceProperties::default(),
            kind,
        }
    }
}
#[doc = "The operation system required by the code in service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperatingSystemType")]
pub enum OperatingSystemType {
    Linux,
    Windows,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperatingSystemType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperatingSystemType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperatingSystemType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Linux => serializer.serialize_unit_variant("OperatingSystemType", 0u32, "Linux"),
            Self::Windows => serializer.serialize_unit_variant("OperatingSystemType", 1u32, "Windows"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Specifying this parameter adds support for reliable collections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReliableCollectionsRef {
    #[doc = "Name of ReliableCollection resource. Right now it's not used and you can use any string."]
    pub name: String,
    #[doc = "False (the default) if ReliableCollections state is persisted to disk as usual. True if you do not want to persist state, in which case replication is still enabled and you can use ReliableCollections as distributed cache."]
    #[serde(rename = "doNotPersistState", default, skip_serializing_if = "Option::is_none")]
    pub do_not_persist_state: Option<bool>,
}
impl ReliableCollectionsRef {
    pub fn new(name: String) -> Self {
        Self {
            name,
            do_not_persist_state: None,
        }
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
#[doc = "This type describes the requested resources for a given container. It describes the least amount of resources required for the container. A container can consume more than requested resources up to the specified limits before being restarted. Currently, the requested resources are treated as limits."]
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
    #[doc = "This type describes the requested resources for a given container. It describes the least amount of resources required for the container. A container can consume more than requested resources up to the specified limits before being restarted. Currently, the requested resources are treated as limits."]
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
#[doc = "Status of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceStatus")]
pub enum ResourceStatus {
    Unknown,
    Ready,
    Upgrading,
    Creating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ResourceStatus", 0u32, "Unknown"),
            Self::Ready => serializer.serialize_unit_variant("ResourceStatus", 1u32, "Ready"),
            Self::Upgrading => serializer.serialize_unit_variant("ResourceStatus", 2u32, "Upgrading"),
            Self::Creating => serializer.serialize_unit_variant("ResourceStatus", 3u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("ResourceStatus", 4u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("ResourceStatus", 5u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the kind of secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SecretKind")]
pub enum SecretKind {
    #[serde(rename = "inlinedValue")]
    InlinedValue,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SecretKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SecretKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SecretKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InlinedValue => serializer.serialize_unit_variant("SecretKind", 0u32, "inlinedValue"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "This type describes a secret resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of a secret resource."]
    pub properties: SecretResourceProperties,
}
impl SecretResourceDescription {
    pub fn new(tracked_resource: TrackedResource, properties: SecretResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "A pageable list of secret resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretResourceDescriptionList {
    #[doc = "One page of the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SecretResourceDescription>,
    #[doc = "URI to fetch the next page of the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecretResourceDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SecretResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a secret resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretResourceProperties {
    #[serde(flatten)]
    pub secret_resource_properties_base: SecretResourcePropertiesBase,
    #[doc = "User readable description of the secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[doc = "Gives additional information about the current status of the secret."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "The type of the content stored in the secret value. The value of this property is opaque to Service Fabric. Once set, the value of this property cannot be changed."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}
impl SecretResourceProperties {
    pub fn new(secret_resource_properties_base: SecretResourcePropertiesBase) -> Self {
        Self {
            secret_resource_properties_base,
            description: None,
            status: None,
            status_details: None,
            content_type: None,
        }
    }
}
#[doc = "This type describes the properties of a secret resource, including its kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretResourcePropertiesBase {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[doc = "Describes the kind of secret."]
    pub kind: SecretKind,
}
impl SecretResourcePropertiesBase {
    pub fn new(kind: SecretKind) -> Self {
        Self {
            provisioned_resource_properties: ProvisionedResourceProperties::default(),
            kind,
        }
    }
}
#[doc = "This type represents the unencrypted value of the secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretValue {
    #[doc = "The actual value of the secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SecretValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This type describes properties of secret value resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretValueProperties {
    #[doc = "The actual value of the secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SecretValueProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This type describes a value of a secret resource. The name of this resource is the version identifier corresponding to this secret value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValueResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "This type describes properties of a secret value resource."]
    pub properties: SecretValueResourceProperties,
}
impl SecretValueResourceDescription {
    pub fn new(tracked_resource: TrackedResource, properties: SecretValueResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "A pageable list of values of a secret resource. The information does not include only the name of the value and not the actual unencrypted value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretValueResourceDescriptionList {
    #[doc = "One page of the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SecretValueResourceDescription>,
    #[doc = "URI to fetch the next page of the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecretValueResourceDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SecretValueResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This type describes properties of a secret value resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretValueResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub secret_value_properties: SecretValueProperties,
}
impl SecretValueResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of a service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceProperties {
    #[doc = "User readable description of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The number of replicas of the service to create. Defaults to 1 if not specified."]
    #[serde(rename = "replicaCount", default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i64>,
    #[doc = "Auto scaling policies"]
    #[serde(rename = "autoScalingPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub auto_scaling_policies: Vec<AutoScalingPolicy>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[doc = "Gives additional information about the current status of the service."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[doc = "When the service's health state is not 'Ok', this additional details from service fabric Health Manager for the user to know why the service is marked unhealthy."]
    #[serde(rename = "unhealthyEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluation: Option<String>,
}
impl ServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a replica of a service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReplicaDescription {
    #[serde(flatten)]
    pub service_replica_properties: ServiceReplicaProperties,
    #[doc = "Name of the replica."]
    #[serde(rename = "replicaName")]
    pub replica_name: String,
}
impl ServiceReplicaDescription {
    pub fn new(service_replica_properties: ServiceReplicaProperties, replica_name: String) -> Self {
        Self {
            service_replica_properties,
            replica_name,
        }
    }
}
#[doc = "A pageable list of service replicas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceReplicaDescriptionList {
    #[doc = "One page of the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceReplicaDescription>,
    #[doc = "URI to fetch the next page of the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceReplicaDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServiceReplicaDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a service replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReplicaProperties {
    #[doc = "The operation system required by the code in service."]
    #[serde(rename = "osType")]
    pub os_type: OperatingSystemType,
    #[doc = "Describes the set of code packages that forms the service. A code package describes the container and the properties for running it. All the code packages are started together on the same host and share the same context (network, process etc.)."]
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
    pub fn new(os_type: OperatingSystemType, code_packages: Vec<ContainerCodePackageProperties>) -> Self {
        Self {
            os_type,
            code_packages,
            network_refs: Vec::new(),
            diagnostics: None,
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
#[doc = "A pageable list of service resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceResourceDescriptionList {
    #[doc = "One page of the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceResourceDescription>,
    #[doc = "URI to fetch the next page of the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceResourceDescriptionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServiceResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This type describes properties of a service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub service_replica_properties: ServiceReplicaProperties,
    #[serde(flatten)]
    pub service_properties: ServiceProperties,
}
impl ServiceResourceProperties {
    pub fn new(service_replica_properties: ServiceReplicaProperties) -> Self {
        Self {
            provisioned_resource_properties: ProvisionedResourceProperties::default(),
            service_replica_properties,
            service_properties: ServiceProperties::default(),
        }
    }
}
#[doc = "Describes a setting for the container. The setting file path can be fetched from environment variable \"Fabric_SettingPath\". The path for Windows container is \"C:\\\\secrets\". The path for Linux container is \"/var/secrets\"."]
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
#[doc = "Describes the tcp configuration for external connectivity for this network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TcpConfig {
    #[doc = "tcp gateway config name."]
    pub name: String,
    #[doc = "Specifies the port at which the service endpoint below needs to be exposed."]
    pub port: i64,
    #[doc = "Describes destination endpoint for routing traffic."]
    pub destination: GatewayDestination,
}
impl TcpConfig {
    pub fn new(name: String, port: i64, destination: GatewayDestination) -> Self {
        Self { name, port, destination }
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
#[doc = "Describes properties of a volume resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[doc = "User readable description of the volume."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[doc = "Gives additional information about the current status of the volume."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "Describes the provider of the volume resource."]
    pub provider: VolumeProvider,
    #[doc = "This type describes a volume provided by an Azure Files file share."]
    #[serde(rename = "azureFileParameters", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_parameters: Option<VolumeProviderParametersAzureFile>,
}
impl VolumeProperties {
    pub fn new(provider: VolumeProvider) -> Self {
        Self {
            description: None,
            status: None,
            status_details: None,
            provider,
            azure_file_parameters: None,
        }
    }
}
#[doc = "Describes the provider of the volume resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VolumeProvider")]
pub enum VolumeProvider {
    #[serde(rename = "SFAzureFile")]
    SfAzureFile,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VolumeProvider {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VolumeProvider {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VolumeProvider {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SfAzureFile => serializer.serialize_unit_variant("VolumeProvider", 0u32, "SFAzureFile"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Describes a reference to a volume resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeReference {
    #[doc = "Name of the volume being referenced."]
    pub name: String,
    #[doc = "The flag indicating whether the volume is read only. Default is 'false'."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "The path within the container at which the volume should be mounted. Only valid path characters are allowed."]
    #[serde(rename = "destinationPath")]
    pub destination_path: String,
}
impl VolumeReference {
    pub fn new(name: String, destination_path: String) -> Self {
        Self {
            name,
            read_only: None,
            destination_path,
        }
    }
}
#[doc = "This type describes a volume resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "This type describes properties of a volume resource."]
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
#[doc = "This type describes properties of a volume resource."]
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
