#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Azure Active Directory metadata used for secured connection to cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadMetadata {
    #[doc = "The AAD authority url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[doc = "The AAD client application Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[doc = "The AAD cluster application Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[doc = "The AAD login url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[doc = "The client application redirect address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<String>,
    #[doc = "The AAD tenant Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}
impl AadMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Active Directory metadata object used for secured connection to cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadMetadataObject {
    #[doc = "The client authentication method."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure Active Directory metadata used for secured connection to cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AadMetadata>,
}
impl AadMetadataObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a scaling mechanism for adding or removing named partitions of a stateless service. Partition names are in the format '0','1''N-1'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddRemoveIncrementalNamedPartitionScalingMechanism {
    #[serde(flatten)]
    pub scaling_mechanism_description: ScalingMechanismDescription,
    #[doc = "Minimum number of named partitions of the service."]
    #[serde(rename = "MinPartitionCount")]
    pub min_partition_count: i64,
    #[doc = "Maximum number of named partitions of the service."]
    #[serde(rename = "MaxPartitionCount")]
    pub max_partition_count: i64,
    #[doc = "The number of instances to add or remove during a scaling operation."]
    #[serde(rename = "ScaleIncrement")]
    pub scale_increment: i64,
}
impl AddRemoveIncrementalNamedPartitionScalingMechanism {
    pub fn new(
        scaling_mechanism_description: ScalingMechanismDescription,
        min_partition_count: i64,
        max_partition_count: i64,
        scale_increment: i64,
    ) -> Self {
        Self {
            scaling_mechanism_description,
            min_partition_count,
            max_partition_count,
            scale_increment,
        }
    }
}
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
#[doc = "Metadata about an Analysis Event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisEventMetadata {
    #[doc = "The analysis delay."]
    #[serde(rename = "Delay", default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    #[doc = "The duration of analysis."]
    #[serde(rename = "Duration", default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}
impl AnalysisEventMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backup configuration information for a specific Service Fabric application specifying what backup policy is being applied and suspend description, if any."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationBackupConfigurationInfo {
    #[serde(flatten)]
    pub backup_configuration_info: BackupConfigurationInfo,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
}
impl ApplicationBackupConfigurationInfo {
    pub fn new(backup_configuration_info: BackupConfigurationInfo) -> Self {
        Self {
            backup_configuration_info,
            application_name: None,
        }
    }
}
#[doc = "Identifies the Service Fabric application which is being backed up."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationBackupEntity {
    #[serde(flatten)]
    pub backup_entity: BackupEntity,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
}
impl ApplicationBackupEntity {
    pub fn new(backup_entity: BackupEntity) -> Self {
        Self {
            backup_entity,
            application_name: None,
        }
    }
}
#[doc = "Describes capacity information for services of this application. This description can be used for describing the following.\n- Reserving the capacity for the services on the nodes\n- Limiting the total number of nodes that services of this application can run on\n- Limiting the custom capacity metrics to limit the total consumption of this metric by the services of this application"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationCapacityDescription {
    #[doc = "The minimum number of nodes where Service Fabric will reserve capacity for this application. Note that this does not mean that the services of this application will be placed on all of those nodes. If this property is set to zero, no capacity will be reserved. The value of this property cannot be more than the value of the MaximumNodes property."]
    #[serde(rename = "MinimumNodes", default, skip_serializing_if = "Option::is_none")]
    pub minimum_nodes: Option<i64>,
    #[doc = "The maximum number of nodes where Service Fabric will reserve capacity for this application. Note that this does not mean that the services of this application will be placed on all of those nodes. By default, the value of this property is zero and it means that the services can be placed on any node."]
    #[serde(rename = "MaximumNodes", default, skip_serializing_if = "Option::is_none")]
    pub maximum_nodes: Option<i64>,
    #[doc = "List of application capacity metric description."]
    #[serde(rename = "ApplicationMetrics", default, skip_serializing_if = "Option::is_none")]
    pub application_metrics: Option<ApplicationMetricDescriptionList>,
}
impl ApplicationCapacityDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container Exited event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationContainerInstanceExitedEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Name of Service."]
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[doc = "Name of Service package."]
    #[serde(rename = "ServicePackageName")]
    pub service_package_name: String,
    #[doc = "Activation Id of Service package."]
    #[serde(rename = "ServicePackageActivationId")]
    pub service_package_activation_id: String,
    #[doc = "Indicates IsExclusive flag."]
    #[serde(rename = "IsExclusive")]
    pub is_exclusive: bool,
    #[doc = "Name of Code package."]
    #[serde(rename = "CodePackageName")]
    pub code_package_name: String,
    #[doc = "Type of EntryPoint."]
    #[serde(rename = "EntryPointType")]
    pub entry_point_type: String,
    #[doc = "Name of Container image."]
    #[serde(rename = "ImageName")]
    pub image_name: String,
    #[doc = "Name of Container."]
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    #[doc = "Host Id."]
    #[serde(rename = "HostId")]
    pub host_id: String,
    #[doc = "Exit code of process."]
    #[serde(rename = "ExitCode")]
    pub exit_code: i64,
    #[doc = "Indicates if termination is unexpected."]
    #[serde(rename = "UnexpectedTermination")]
    pub unexpected_termination: bool,
    #[doc = "Start time of process."]
    #[serde(rename = "StartTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
}
impl ApplicationContainerInstanceExitedEvent {
    pub fn new(
        application_event: ApplicationEvent,
        service_name: String,
        service_package_name: String,
        service_package_activation_id: String,
        is_exclusive: bool,
        code_package_name: String,
        entry_point_type: String,
        image_name: String,
        container_name: String,
        host_id: String,
        exit_code: i64,
        unexpected_termination: bool,
        start_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            application_event,
            service_name,
            service_package_name,
            service_package_activation_id,
            is_exclusive,
            code_package_name,
            entry_point_type,
            image_name,
            container_name,
            host_id,
            exit_code,
            unexpected_termination,
            start_time,
        }
    }
}
#[doc = "Application Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCreatedEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Application type name."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: String,
    #[doc = "Application type version."]
    #[serde(rename = "ApplicationTypeVersion")]
    pub application_type_version: String,
    #[doc = "Application definition kind."]
    #[serde(rename = "ApplicationDefinitionKind")]
    pub application_definition_kind: String,
}
impl ApplicationCreatedEvent {
    pub fn new(
        application_event: ApplicationEvent,
        application_type_name: String,
        application_type_version: String,
        application_definition_kind: String,
    ) -> Self {
        Self {
            application_event,
            application_type_name,
            application_type_version,
            application_definition_kind,
        }
    }
}
#[doc = "The mechanism used to define a Service Fabric application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationDefinitionKind")]
pub enum ApplicationDefinitionKind {
    Invalid,
    ServiceFabricApplicationDescription,
    Compose,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationDefinitionKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationDefinitionKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationDefinitionKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ApplicationDefinitionKind", 0u32, "Invalid"),
            Self::ServiceFabricApplicationDescription => {
                serializer.serialize_unit_variant("ApplicationDefinitionKind", 1u32, "ServiceFabricApplicationDescription")
            }
            Self::Compose => serializer.serialize_unit_variant("ApplicationDefinitionKind", 2u32, "Compose"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Application Deleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDeletedEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Application type name."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: String,
    #[doc = "Application type version."]
    #[serde(rename = "ApplicationTypeVersion")]
    pub application_type_version: String,
}
impl ApplicationDeletedEvent {
    pub fn new(application_event: ApplicationEvent, application_type_name: String, application_type_version: String) -> Self {
        Self {
            application_event,
            application_type_name,
            application_type_version,
        }
    }
}
#[doc = "Describes a Service Fabric application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDescription {
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name")]
    pub name: ApplicationName,
    #[doc = "The application type name as defined in the application manifest."]
    #[serde(rename = "TypeName")]
    pub type_name: ApplicationTypeName,
    #[doc = "The version of the application type as defined in the application manifest."]
    #[serde(rename = "TypeVersion")]
    pub type_version: ApplicationTypeVersion,
    #[doc = "List of application parameters with overridden values from their default values specified in the application manifest."]
    #[serde(rename = "ParameterList", default, skip_serializing_if = "Option::is_none")]
    pub parameter_list: Option<ApplicationParameterList>,
    #[doc = "Describes capacity information for services of this application. This description can be used for describing the following.\n- Reserving the capacity for the services on the nodes\n- Limiting the total number of nodes that services of this application can run on\n- Limiting the custom capacity metrics to limit the total consumption of this metric by the services of this application"]
    #[serde(rename = "ApplicationCapacity", default, skip_serializing_if = "Option::is_none")]
    pub application_capacity: Option<ApplicationCapacityDescription>,
    #[doc = "Managed application identity description."]
    #[serde(rename = "ManagedApplicationIdentity", default, skip_serializing_if = "Option::is_none")]
    pub managed_application_identity: Option<ManagedApplicationIdentityDescription>,
}
impl ApplicationDescription {
    pub fn new(name: ApplicationName, type_name: ApplicationTypeName, type_version: ApplicationTypeVersion) -> Self {
        Self {
            name,
            type_name,
            type_version,
            parameter_list: None,
            application_capacity: None,
            managed_application_identity: None,
        }
    }
}
#[doc = "Represents the base for all Application Events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationEvent {
    #[serde(flatten)]
    pub fabric_event: FabricEvent,
    #[doc = "The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource.\nStarting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the application name is \"fabric:/myapp/app1\",\nthe application identity would be \"myapp\\~app1\" in 6.0+ and \"myapp/app1\" in previous versions."]
    #[serde(rename = "ApplicationId")]
    pub application_id: ApplicationId,
}
impl ApplicationEvent {
    pub fn new(fabric_event: FabricEvent, application_id: ApplicationId) -> Self {
        Self {
            fabric_event,
            application_id,
        }
    }
}
pub type ApplicationEventList = Vec<ApplicationEvent>;
#[doc = "Represents the health of the application. Contains the application aggregated health state and the service and deployed application health states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationHealth {
    #[serde(flatten)]
    pub entity_health: EntityHealth,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ApplicationName>,
    #[doc = "Service health states as found in the health store."]
    #[serde(rename = "ServiceHealthStates", default, skip_serializing_if = "Vec::is_empty")]
    pub service_health_states: Vec<ServiceHealthState>,
    #[doc = "Deployed application health states as found in the health store."]
    #[serde(rename = "DeployedApplicationHealthStates", default, skip_serializing_if = "Vec::is_empty")]
    pub deployed_application_health_states: Vec<DeployedApplicationHealthState>,
}
impl ApplicationHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents health evaluation for an application, containing information about the data and the algorithm used by the health store to evaluate health."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl ApplicationHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            application_name: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Defines the application health policy map used to evaluate the health of an application or one of its children entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationHealthPolicies {
    #[doc = "Defines a map that contains specific application health policies for different applications.\nEach entry specifies as key the application name and as value an ApplicationHealthPolicy used to evaluate the application health.\nIf an application is not specified in the map, the application health evaluation uses the ApplicationHealthPolicy found in its application manifest or the default application health policy (if no health policy is defined in the manifest).\nThe map is empty by default."]
    #[serde(rename = "ApplicationHealthPolicyMap", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy_map: Option<ApplicationHealthPolicyMap>,
}
impl ApplicationHealthPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a health policy used to evaluate the health of an application or one of its children entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationHealthPolicy {
    #[doc = "Indicates whether warnings are treated with the same severity as errors."]
    #[serde(rename = "ConsiderWarningAsError", default, skip_serializing_if = "Option::is_none")]
    pub consider_warning_as_error: Option<bool>,
    #[doc = "The maximum allowed percentage of unhealthy deployed applications. Allowed values are Byte values from zero to 100.\nThe percentage represents the maximum tolerated percentage of deployed applications that can be unhealthy before the application is considered in error.\nThis is calculated by dividing the number of unhealthy deployed applications over the number of nodes where the application is currently deployed on in the cluster.\nThe computation rounds up to tolerate one failure on small numbers of nodes. Default percentage is zero."]
    #[serde(
        rename = "MaxPercentUnhealthyDeployedApplications",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_percent_unhealthy_deployed_applications: Option<i64>,
    #[doc = "Represents the health policy used to evaluate the health of services belonging to a service type."]
    #[serde(rename = "DefaultServiceTypeHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub default_service_type_health_policy: Option<ServiceTypeHealthPolicy>,
    #[doc = "Defines a ServiceTypeHealthPolicy per service type name.\n\nThe entries in the map replace the default service type health policy for each specified service type. For example, in an application that contains both a stateless gateway service type and a stateful engine service type, the health policies for the stateless and stateful services can be configured differently. With policy per service type, there's more granular control of the health of the service.\n\nIf no policy is specified for a service type name, the DefaultServiceTypeHealthPolicy is used for evaluation."]
    #[serde(rename = "ServiceTypeHealthPolicyMap", default, skip_serializing_if = "Option::is_none")]
    pub service_type_health_policy_map: Option<ServiceTypeHealthPolicyMap>,
}
impl ApplicationHealthPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ApplicationHealthPolicyMap = Vec<ApplicationHealthPolicyMapItem>;
#[doc = "Defines an item in ApplicationHealthPolicyMap."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationHealthPolicyMapItem {
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Key")]
    pub key: ApplicationName,
    #[doc = "Defines a health policy used to evaluate the health of an application or one of its children entities."]
    #[serde(rename = "Value")]
    pub value: ApplicationHealthPolicy,
}
impl ApplicationHealthPolicyMapItem {
    pub fn new(key: ApplicationName, value: ApplicationHealthPolicy) -> Self {
        Self { key, value }
    }
}
#[doc = "Represents the map of application health policies for a ServiceFabric cluster upgrade"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationHealthPolicyMapObject {
    #[doc = "Defines a map that contains specific application health policies for different applications.\nEach entry specifies as key the application name and as value an ApplicationHealthPolicy used to evaluate the application health.\nIf an application is not specified in the map, the application health evaluation uses the ApplicationHealthPolicy found in its application manifest or the default application health policy (if no health policy is defined in the manifest).\nThe map is empty by default."]
    #[serde(rename = "ApplicationHealthPolicyMap", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy_map: Option<ApplicationHealthPolicyMap>,
}
impl ApplicationHealthPolicyMapObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application Health Report Expired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationHealthReportExpiredEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Id of Application instance."]
    #[serde(rename = "ApplicationInstanceId")]
    pub application_instance_id: i64,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl ApplicationHealthReportExpiredEvent {
    pub fn new(
        application_event: ApplicationEvent,
        application_instance_id: i64,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            application_event,
            application_instance_id,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Represents the health state of an application, which contains the application identifier and the aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationHealthState {
    #[serde(flatten)]
    pub entity_health_state: EntityHealthState,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ApplicationName>,
}
impl ApplicationHealthState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the health state chunk of a application.\nThe application health state chunk contains the application name, its aggregated health state and any children services and deployed applications that respect the filters in cluster health chunk query description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationHealthStateChunk {
    #[serde(flatten)]
    pub entity_health_state_chunk: EntityHealthStateChunk,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
    #[doc = "The application type name as defined in the application manifest."]
    #[serde(rename = "ApplicationTypeName", default, skip_serializing_if = "Option::is_none")]
    pub application_type_name: Option<ApplicationTypeName>,
    #[doc = "The list of service health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
    #[serde(rename = "ServiceHealthStateChunks", default, skip_serializing_if = "Option::is_none")]
    pub service_health_state_chunks: Option<ServiceHealthStateChunkList>,
    #[doc = "The list of deployed application health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
    #[serde(rename = "DeployedApplicationHealthStateChunks", default, skip_serializing_if = "Option::is_none")]
    pub deployed_application_health_state_chunks: Option<DeployedApplicationHealthStateChunkList>,
}
impl ApplicationHealthStateChunk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of application health state chunks in the cluster that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationHealthStateChunkList {
    #[serde(flatten)]
    pub entity_health_state_chunk_list: EntityHealthStateChunkList,
    #[doc = "The list of application health state chunks that respect the input filters in the chunk query."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ApplicationHealthStateChunk>,
}
impl ApplicationHealthStateChunkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines matching criteria to determine whether a application should be included in the cluster health chunk.\nOne filter can match zero, one or multiple applications, depending on its properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationHealthStateFilter {
    #[doc = "The name of the application that matches the filter, as a fabric uri. The filter is applied only to the specified application, if it exists.\nIf the application doesn't exist, no application is returned in the cluster health chunk based on this filter.\nIf the application exists, it is included in the cluster health chunk if it respects the other filter properties.\nIf not specified, all applications are matched against the other filter members, like health state filter."]
    #[serde(rename = "ApplicationNameFilter", default, skip_serializing_if = "Option::is_none")]
    pub application_name_filter: Option<String>,
    #[doc = "The name of the application type that matches the filter.\nIf specified, the filter is applied only to applications of the selected application type, if any exists.\nIf no applications of the specified application type exists, no application is returned in the cluster health chunk based on this filter.\nEach application of the specified application type is included in the cluster health chunk if it respects the other filter properties.\nIf not specified, all applications are matched against the other filter members, like health state filter."]
    #[serde(rename = "ApplicationTypeNameFilter", default, skip_serializing_if = "Option::is_none")]
    pub application_type_name_filter: Option<String>,
    #[doc = "The filter for the health state of the applications. It allows selecting applications if they match the desired health states.\nThe possible values are integer value of one of the following health states. Only applications that match the filter are returned. All applications are used to evaluate the cluster aggregated health state.\nIf not specified, default value is None, unless the application name or the application type name are specified. If the filter has default value and application name is specified, the matching application is returned.\nThe state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise 'OR' operator.\nFor example, if the provided value is 6, it matches applications with HealthState value of OK (2) and Warning (4).\n\n- Default - Default value. Matches any HealthState. The value is zero.\n- None - Filter that doesn't match any HealthState value. Used in order to return no results on a given collection of states. The value is 1.\n- Ok - Filter that matches input with HealthState value Ok. The value is 2.\n- Warning - Filter that matches input with HealthState value Warning. The value is 4.\n- Error - Filter that matches input with HealthState value Error. The value is 8.\n- All - Filter that matches input with any HealthState value. The value is 65535."]
    #[serde(rename = "HealthStateFilter", default, skip_serializing_if = "Option::is_none")]
    pub health_state_filter: Option<i64>,
    #[doc = "Defines a list of filters that specify which services to be included in the returned cluster health chunk as children of the application. The services are returned only if the parent application matches a filter.\nIf the list is empty, no services are returned. All the services are used to evaluate the parent application aggregated health state, regardless of the input filters.\nThe application filter may specify multiple service filters.\nFor example, it can specify a filter to return all services with health state Error and another filter to always include a service identified by its service name."]
    #[serde(rename = "ServiceFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub service_filters: Vec<ServiceHealthStateFilter>,
    #[doc = "Defines a list of filters that specify which deployed applications to be included in the returned cluster health chunk as children of the application. The deployed applications are returned only if the parent application matches a filter.\nIf the list is empty, no deployed applications are returned. All the deployed applications are used to evaluate the parent application aggregated health state, regardless of the input filters.\nThe application filter may specify multiple deployed application filters.\nFor example, it can specify a filter to return all deployed applications with health state Error and another filter to always include a deployed application on a specified node."]
    #[serde(rename = "DeployedApplicationFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub deployed_application_filters: Vec<DeployedApplicationHealthStateFilter>,
}
impl ApplicationHealthStateFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ApplicationId = String;
#[doc = "Information about a Service Fabric application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInfo {
    #[doc = "The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource.\nStarting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the application name is \"fabric:/myapp/app1\",\nthe application identity would be \"myapp\\~app1\" in 6.0+ and \"myapp/app1\" in previous versions."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ApplicationId>,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ApplicationName>,
    #[doc = "The application type name as defined in the application manifest."]
    #[serde(rename = "TypeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<ApplicationTypeName>,
    #[doc = "The version of the application type as defined in the application manifest."]
    #[serde(rename = "TypeVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_version: Option<ApplicationTypeVersion>,
    #[doc = "The status of the application."]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ApplicationStatus>,
    #[doc = "List of application parameters with overridden values from their default values specified in the application manifest."]
    #[serde(rename = "Parameters", default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ApplicationParameterList>,
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "HealthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[doc = "The mechanism used to define a Service Fabric application."]
    #[serde(rename = "ApplicationDefinitionKind", default, skip_serializing_if = "Option::is_none")]
    pub application_definition_kind: Option<ApplicationDefinitionKind>,
}
impl ApplicationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Load Information about a Service Fabric application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationLoadInfo {
    #[doc = "The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource.\nStarting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the application name is \"fabric:/myapp/app1\",\nthe application identity would be \"myapp\\~app1\" in 6.0+ and \"myapp/app1\" in previous versions."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ApplicationId>,
    #[doc = "The minimum number of nodes for this application.\nIt is the number of nodes where Service Fabric will reserve Capacity in the cluster which equals to ReservedLoad * MinimumNodes for this Application instance.\nFor applications that do not have application capacity defined this value will be zero."]
    #[serde(rename = "MinimumNodes", default, skip_serializing_if = "Option::is_none")]
    pub minimum_nodes: Option<i64>,
    #[doc = "The maximum number of nodes where this application can be instantiated.\nIt is the number of nodes this application is allowed to span.\nFor applications that do not have application capacity defined this value will be zero."]
    #[serde(rename = "MaximumNodes", default, skip_serializing_if = "Option::is_none")]
    pub maximum_nodes: Option<i64>,
    #[doc = "The number of nodes on which this application is instantiated.\nFor applications that do not have application capacity defined this value will be zero."]
    #[serde(rename = "NodeCount", default, skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i64>,
    #[doc = "List of application load metric information."]
    #[serde(rename = "ApplicationLoadMetricInformation", default, skip_serializing_if = "Option::is_none")]
    pub application_load_metric_information: Option<ApplicationLoadMetricInformationList>,
}
impl ApplicationLoadInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes load information for a custom resource balancing metric. This can be used to limit the total consumption of this metric by the services of this application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationLoadMetricInformation {
    #[doc = "The name of the metric."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "This is the capacity reserved in the cluster for the application.\nIt's the product of NodeReservationCapacity and MinimumNodes.\nIf set to zero, no capacity is reserved for this metric.\nWhen setting application capacity or when updating application capacity this value must be smaller than or equal to MaximumCapacity for each metric."]
    #[serde(rename = "ReservationCapacity", default, skip_serializing_if = "Option::is_none")]
    pub reservation_capacity: Option<i64>,
    #[doc = "Total capacity for this metric in this application instance."]
    #[serde(rename = "ApplicationCapacity", default, skip_serializing_if = "Option::is_none")]
    pub application_capacity: Option<i64>,
    #[doc = "Current load for this metric in this application instance."]
    #[serde(rename = "ApplicationLoad", default, skip_serializing_if = "Option::is_none")]
    pub application_load: Option<i64>,
}
impl ApplicationLoadMetricInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ApplicationLoadMetricInformationList = Vec<ApplicationLoadMetricInformation>;
#[doc = "Describes capacity information for a custom resource balancing metric. This can be used to limit the total consumption of this metric by the services of this application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationMetricDescription {
    #[doc = "The name of the metric."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The maximum node capacity for Service Fabric application.\nThis is the maximum Load for an instance of this application on a single node. Even if the capacity of node is greater than this value, Service Fabric will limit the total load of services within the application on each node to this value.\nIf set to zero, capacity for this metric is unlimited on each node.\nWhen creating a new application with application capacity defined, the product of MaximumNodes and this value must always be smaller than or equal to TotalApplicationCapacity.\nWhen updating existing application with application capacity, the product of MaximumNodes and this value must always be smaller than or equal to TotalApplicationCapacity."]
    #[serde(rename = "MaximumCapacity", default, skip_serializing_if = "Option::is_none")]
    pub maximum_capacity: Option<i64>,
    #[doc = "The node reservation capacity for Service Fabric application.\nThis is the amount of load which is reserved on nodes which have instances of this application.\nIf MinimumNodes is specified, then the product of these values will be the capacity reserved in the cluster for the application.\nIf set to zero, no capacity is reserved for this metric.\nWhen setting application capacity or when updating application capacity; this value must be smaller than or equal to MaximumCapacity for each metric."]
    #[serde(rename = "ReservationCapacity", default, skip_serializing_if = "Option::is_none")]
    pub reservation_capacity: Option<i64>,
    #[doc = "The total metric capacity for Service Fabric application.\nThis is the total metric capacity for this application in the cluster. Service Fabric will try to limit the sum of loads of services within the application to this value.\nWhen creating a new application with application capacity defined, the product of MaximumNodes and MaximumCapacity must always be smaller than or equal to this value."]
    #[serde(rename = "TotalApplicationCapacity", default, skip_serializing_if = "Option::is_none")]
    pub total_application_capacity: Option<i64>,
}
impl ApplicationMetricDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ApplicationMetricDescriptionList = Vec<ApplicationMetricDescription>;
pub type ApplicationName = String;
#[doc = "Information about the application name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationNameInfo {
    #[doc = "The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource.\nStarting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the application name is \"fabric:/myapp/app1\",\nthe application identity would be \"myapp\\~app1\" in 6.0+ and \"myapp/app1\" in previous versions."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ApplicationId>,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ApplicationName>,
}
impl ApplicationNameInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application Health Report Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationNewHealthReportEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Id of Application instance."]
    #[serde(rename = "ApplicationInstanceId")]
    pub application_instance_id: i64,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl ApplicationNewHealthReportEvent {
    pub fn new(
        application_event: ApplicationEvent,
        application_instance_id: i64,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            application_event,
            application_instance_id,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "The kind of action that needs to be taken for cleaning up the application package after successful provision."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationPackageCleanupPolicy")]
pub enum ApplicationPackageCleanupPolicy {
    Invalid,
    Default,
    Automatic,
    Manual,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationPackageCleanupPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationPackageCleanupPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationPackageCleanupPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ApplicationPackageCleanupPolicy", 0u32, "Invalid"),
            Self::Default => serializer.serialize_unit_variant("ApplicationPackageCleanupPolicy", 1u32, "Default"),
            Self::Automatic => serializer.serialize_unit_variant("ApplicationPackageCleanupPolicy", 2u32, "Automatic"),
            Self::Manual => serializer.serialize_unit_variant("ApplicationPackageCleanupPolicy", 3u32, "Manual"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes an application parameter override to be applied when creating or upgrading an application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationParameter {
    #[doc = "The name of the parameter."]
    #[serde(rename = "Key")]
    pub key: String,
    #[doc = "The value of the parameter."]
    #[serde(rename = "Value")]
    pub value: String,
}
impl ApplicationParameter {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
pub type ApplicationParameterList = Vec<ApplicationParameter>;
#[doc = "Process Exited event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProcessExitedEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Name of Service."]
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[doc = "Name of Service package."]
    #[serde(rename = "ServicePackageName")]
    pub service_package_name: String,
    #[doc = "Activation Id of Service package."]
    #[serde(rename = "ServicePackageActivationId")]
    pub service_package_activation_id: String,
    #[doc = "Indicates IsExclusive flag."]
    #[serde(rename = "IsExclusive")]
    pub is_exclusive: bool,
    #[doc = "Name of Code package."]
    #[serde(rename = "CodePackageName")]
    pub code_package_name: String,
    #[doc = "Type of EntryPoint."]
    #[serde(rename = "EntryPointType")]
    pub entry_point_type: String,
    #[doc = "Name of executable."]
    #[serde(rename = "ExeName")]
    pub exe_name: String,
    #[doc = "Process Id."]
    #[serde(rename = "ProcessId")]
    pub process_id: i64,
    #[doc = "Host Id."]
    #[serde(rename = "HostId")]
    pub host_id: String,
    #[doc = "Exit code of process."]
    #[serde(rename = "ExitCode")]
    pub exit_code: i64,
    #[doc = "Indicates if termination is unexpected."]
    #[serde(rename = "UnexpectedTermination")]
    pub unexpected_termination: bool,
    #[doc = "Start time of process."]
    #[serde(rename = "StartTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
}
impl ApplicationProcessExitedEvent {
    pub fn new(
        application_event: ApplicationEvent,
        service_name: String,
        service_package_name: String,
        service_package_activation_id: String,
        is_exclusive: bool,
        code_package_name: String,
        entry_point_type: String,
        exe_name: String,
        process_id: i64,
        host_id: String,
        exit_code: i64,
        unexpected_termination: bool,
        start_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            application_event,
            service_name,
            service_package_name,
            service_package_activation_id,
            is_exclusive,
            code_package_name,
            entry_point_type,
            exe_name,
            process_id,
            host_id,
            exit_code,
            unexpected_termination,
            start_time,
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
#[doc = "This type describes a application resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationResourceDescription {
    #[doc = "Name of the Application resource."]
    pub name: ApplicationResourceName,
    #[doc = "Describes properties of a application resource."]
    pub properties: ApplicationProperties,
    #[doc = "Information describing the identities associated with this application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityDescription>,
}
impl ApplicationResourceDescription {
    pub fn new(name: ApplicationResourceName, properties: ApplicationProperties) -> Self {
        Self {
            name,
            properties,
            identity: None,
        }
    }
}
pub type ApplicationResourceName = String;
#[doc = "This type describes an application resource upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResourceUpgradeProgressInfo {
    #[doc = "Name of the Application resource."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The target application version for the application upgrade."]
    #[serde(rename = "TargetApplicationTypeVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_application_type_version: Option<String>,
    #[doc = "The estimated UTC datetime when the upgrade started."]
    #[serde(rename = "StartTimestampUtc", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp_utc: Option<String>,
    #[doc = "The state of the application resource upgrade."]
    #[serde(rename = "UpgradeState", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_state: Option<ApplicationResourceUpgradeState>,
    #[doc = "The estimated percent of replicas are completed in the upgrade."]
    #[serde(rename = "PercentCompleted", default, skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<String>,
    #[doc = "List of service upgrade progresses."]
    #[serde(rename = "ServiceUpgradeProgress", default, skip_serializing_if = "Option::is_none")]
    pub service_upgrade_progress: Option<ServiceUpgradeProgressList>,
    #[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
    #[serde(rename = "RollingUpgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub rolling_upgrade_mode: Option<RollingUpgradeMode>,
    #[doc = "The estimated amount of time that the overall upgrade elapsed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "UpgradeDuration", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_duration: Option<String>,
    #[doc = "Additional detailed information about the status of the pending upgrade."]
    #[serde(rename = "ApplicationUpgradeStatusDetails", default, skip_serializing_if = "Option::is_none")]
    pub application_upgrade_status_details: Option<String>,
    #[doc = "The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer)."]
    #[serde(
        rename = "UpgradeReplicaSetCheckTimeoutInSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upgrade_replica_set_check_timeout_in_seconds: Option<i64>,
    #[doc = "The estimated UTC datetime when the upgrade failed and FailureAction was executed."]
    #[serde(rename = "FailureTimestampUtc", default, skip_serializing_if = "Option::is_none")]
    pub failure_timestamp_utc: Option<String>,
}
impl ApplicationResourceUpgradeProgressInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of the application resource upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationResourceUpgradeState")]
pub enum ApplicationResourceUpgradeState {
    Invalid,
    ProvisioningTarget,
    RollingForward,
    UnprovisioningCurrent,
    CompletedRollforward,
    RollingBack,
    UnprovisioningTarget,
    CompletedRollback,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationResourceUpgradeState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationResourceUpgradeState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationResourceUpgradeState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ApplicationResourceUpgradeState", 0u32, "Invalid"),
            Self::ProvisioningTarget => serializer.serialize_unit_variant("ApplicationResourceUpgradeState", 1u32, "ProvisioningTarget"),
            Self::RollingForward => serializer.serialize_unit_variant("ApplicationResourceUpgradeState", 2u32, "RollingForward"),
            Self::UnprovisioningCurrent => {
                serializer.serialize_unit_variant("ApplicationResourceUpgradeState", 3u32, "UnprovisioningCurrent")
            }
            Self::CompletedRollforward => {
                serializer.serialize_unit_variant("ApplicationResourceUpgradeState", 4u32, "CompletedRollforward")
            }
            Self::RollingBack => serializer.serialize_unit_variant("ApplicationResourceUpgradeState", 5u32, "RollingBack"),
            Self::UnprovisioningTarget => {
                serializer.serialize_unit_variant("ApplicationResourceUpgradeState", 6u32, "UnprovisioningTarget")
            }
            Self::CompletedRollback => serializer.serialize_unit_variant("ApplicationResourceUpgradeState", 7u32, "CompletedRollback"),
            Self::Failed => serializer.serialize_unit_variant("ApplicationResourceUpgradeState", 8u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "The status of the application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationStatus")]
pub enum ApplicationStatus {
    Invalid,
    Ready,
    Upgrading,
    Creating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ApplicationStatus", 0u32, "Invalid"),
            Self::Ready => serializer.serialize_unit_variant("ApplicationStatus", 1u32, "Ready"),
            Self::Upgrading => serializer.serialize_unit_variant("ApplicationStatus", 2u32, "Upgrading"),
            Self::Creating => serializer.serialize_unit_variant("ApplicationStatus", 3u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("ApplicationStatus", 4u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("ApplicationStatus", 5u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents health evaluation for applications of a particular application type. The application type applications evaluation can be returned when cluster health evaluation returns unhealthy aggregated health state, either Error or Warning. It contains health evaluations for each unhealthy application of the included application type that impacted current aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationTypeApplicationsHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "The application type name as defined in the application manifest."]
    #[serde(rename = "ApplicationTypeName", default, skip_serializing_if = "Option::is_none")]
    pub application_type_name: Option<ApplicationTypeName>,
    #[doc = "Maximum allowed percentage of unhealthy applications for the application type, specified as an entry in ApplicationTypeHealthPolicyMap."]
    #[serde(rename = "MaxPercentUnhealthyApplications", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_unhealthy_applications: Option<i64>,
    #[doc = "Total number of applications of the application type found in the health store."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl ApplicationTypeApplicationsHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            application_type_name: None,
            max_percent_unhealthy_applications: None,
            total_count: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "The mechanism used to define a Service Fabric application type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationTypeDefinitionKind")]
pub enum ApplicationTypeDefinitionKind {
    Invalid,
    ServiceFabricApplicationPackage,
    Compose,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationTypeDefinitionKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationTypeDefinitionKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationTypeDefinitionKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ApplicationTypeDefinitionKind", 0u32, "Invalid"),
            Self::ServiceFabricApplicationPackage => {
                serializer.serialize_unit_variant("ApplicationTypeDefinitionKind", 1u32, "ServiceFabricApplicationPackage")
            }
            Self::Compose => serializer.serialize_unit_variant("ApplicationTypeDefinitionKind", 2u32, "Compose"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type ApplicationTypeHealthPolicyMap = Vec<ApplicationTypeHealthPolicyMapItem>;
#[doc = "Defines an item in ApplicationTypeHealthPolicyMap."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationTypeHealthPolicyMapItem {
    #[doc = "The key of the application type health policy map item. This is the name of the application type."]
    #[serde(rename = "Key")]
    pub key: String,
    #[doc = "The value of the application type health policy map item.\nThe max percent unhealthy applications allowed for the application type. Must be between zero and 100."]
    #[serde(rename = "Value")]
    pub value: i64,
}
impl ApplicationTypeHealthPolicyMapItem {
    pub fn new(key: String, value: i64) -> Self {
        Self { key, value }
    }
}
#[doc = "Path description for the application package in the image store specified during the prior copy operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationTypeImageStorePath {
    #[doc = "The relative image store path to the application package."]
    #[serde(rename = "ApplicationTypeBuildPath")]
    pub application_type_build_path: String,
}
impl ApplicationTypeImageStorePath {
    pub fn new(application_type_build_path: String) -> Self {
        Self {
            application_type_build_path,
        }
    }
}
#[doc = "Information about an application type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeInfo {
    #[doc = "The application type name as defined in the application manifest."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ApplicationTypeName>,
    #[doc = "The version of the application type as defined in the application manifest."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ApplicationTypeVersion>,
    #[doc = "List of application type parameters that can be overridden when creating or updating the application."]
    #[serde(rename = "DefaultParameterList", default, skip_serializing_if = "Option::is_none")]
    pub default_parameter_list: Option<ApplicationTypeParameterList>,
    #[doc = "The status of the application type."]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ApplicationTypeStatus>,
    #[doc = "Additional detailed information about the status of the application type."]
    #[serde(rename = "StatusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "The mechanism used to define a Service Fabric application type."]
    #[serde(rename = "ApplicationTypeDefinitionKind", default, skip_serializing_if = "Option::is_none")]
    pub application_type_definition_kind: Option<ApplicationTypeDefinitionKind>,
}
impl ApplicationTypeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the manifest describing an application type registered in a Service Fabric cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeManifest {
    #[doc = "The XML manifest as a string."]
    #[serde(rename = "Manifest", default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
}
impl ApplicationTypeManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ApplicationTypeName = String;
pub type ApplicationTypeParameterList = Vec<ApplicationParameter>;
#[doc = "The status of the application type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationTypeStatus")]
pub enum ApplicationTypeStatus {
    Invalid,
    Provisioning,
    Available,
    Unprovisioning,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationTypeStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationTypeStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationTypeStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ApplicationTypeStatus", 0u32, "Invalid"),
            Self::Provisioning => serializer.serialize_unit_variant("ApplicationTypeStatus", 1u32, "Provisioning"),
            Self::Available => serializer.serialize_unit_variant("ApplicationTypeStatus", 2u32, "Available"),
            Self::Unprovisioning => serializer.serialize_unit_variant("ApplicationTypeStatus", 3u32, "Unprovisioning"),
            Self::Failed => serializer.serialize_unit_variant("ApplicationTypeStatus", 4u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type ApplicationTypeVersion = String;
pub type ApplicationUnhealthyEvaluations = Vec<HealthEvaluationWrapper>;
#[doc = "Application Upgrade Completed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUpgradeCompletedEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Application type name."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: String,
    #[doc = "Application type version."]
    #[serde(rename = "ApplicationTypeVersion")]
    pub application_type_version: String,
    #[doc = "Overall upgrade time in milli-seconds."]
    #[serde(rename = "OverallUpgradeElapsedTimeInMs")]
    pub overall_upgrade_elapsed_time_in_ms: f64,
}
impl ApplicationUpgradeCompletedEvent {
    pub fn new(
        application_event: ApplicationEvent,
        application_type_name: String,
        application_type_version: String,
        overall_upgrade_elapsed_time_in_ms: f64,
    ) -> Self {
        Self {
            application_event,
            application_type_name,
            application_type_version,
            overall_upgrade_elapsed_time_in_ms,
        }
    }
}
#[doc = "Describes the parameters for an application upgrade. Note that upgrade description replaces the existing application description. This means that if the parameters are not specified, the existing parameters on the applications will be overwritten with the empty parameters list. This would result in the application using the default value of the parameters from the application manifest. If you do not want to change any existing parameter values, please get the application parameters first using the GetApplicationInfo query and then supply those values as Parameters in this ApplicationUpgradeDescription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUpgradeDescription {
    #[doc = "The name of the target application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name")]
    pub name: TargetApplicationName,
    #[doc = "The target application type version (found in the application manifest) for the application upgrade."]
    #[serde(rename = "TargetApplicationTypeVersion")]
    pub target_application_type_version: TargetApplicationTypeVersion,
    #[doc = "List of application parameters with overridden values from their default values specified in the application manifest."]
    #[serde(rename = "Parameters", default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ApplicationParameterList>,
    #[doc = "The kind of upgrade out of the following possible values."]
    #[serde(rename = "UpgradeKind")]
    pub upgrade_kind: UpgradeKind,
    #[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
    #[serde(rename = "RollingUpgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub rolling_upgrade_mode: Option<UpgradeMode>,
    #[doc = "The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer)."]
    #[serde(
        rename = "UpgradeReplicaSetCheckTimeoutInSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upgrade_replica_set_check_timeout_in_seconds: Option<UpgradeReplicaSetCheckTimeout>,
    #[doc = "If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data)."]
    #[serde(rename = "ForceRestart", default, skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<ForceRestart>,
    #[doc = "Defines the order in which an upgrade proceeds through the cluster."]
    #[serde(rename = "SortOrder", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<UpgradeSortOrder>,
    #[doc = "Describes the parameters for monitoring an upgrade in Monitored mode."]
    #[serde(rename = "MonitoringPolicy", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_policy: Option<MonitoringPolicyDescription>,
    #[doc = "Defines a health policy used to evaluate the health of an application or one of its children entities."]
    #[serde(rename = "ApplicationHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy: Option<ApplicationHealthPolicy>,
    #[doc = "Duration in seconds, to wait before a stateless instance is closed, to allow the active requests to drain gracefully. This would be effective when the instance is closing during the application/cluster\nupgrade, only for those instances which have a non-zero delay duration configured in the service description. See InstanceCloseDelayDurationSeconds property in $ref: \"#/definitions/StatelessServiceDescription.yaml\" for details.\nNote, the default value of InstanceCloseDelayDurationInSeconds is 4294967295, which indicates that the behavior will entirely depend on the delay configured in the stateless service description."]
    #[serde(rename = "InstanceCloseDelayDurationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub instance_close_delay_duration_in_seconds: Option<InstanceCloseDelayDurationInSeconds>,
}
impl ApplicationUpgradeDescription {
    pub fn new(
        name: TargetApplicationName,
        target_application_type_version: TargetApplicationTypeVersion,
        upgrade_kind: UpgradeKind,
    ) -> Self {
        Self {
            name,
            target_application_type_version,
            parameters: None,
            upgrade_kind,
            rolling_upgrade_mode: None,
            upgrade_replica_set_check_timeout_in_seconds: None,
            force_restart: None,
            sort_order: None,
            monitoring_policy: None,
            application_health_policy: None,
            instance_close_delay_duration_in_seconds: None,
        }
    }
}
#[doc = "Application Upgrade Domain Completed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUpgradeDomainCompletedEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Application type name."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: String,
    #[doc = "Current Application type version."]
    #[serde(rename = "CurrentApplicationTypeVersion")]
    pub current_application_type_version: String,
    #[doc = "Target Application type version."]
    #[serde(rename = "ApplicationTypeVersion")]
    pub application_type_version: String,
    #[doc = "State of upgrade."]
    #[serde(rename = "UpgradeState")]
    pub upgrade_state: String,
    #[doc = "Upgrade domains."]
    #[serde(rename = "UpgradeDomains")]
    pub upgrade_domains: String,
    #[doc = "Upgrade time of domain in milli-seconds."]
    #[serde(rename = "UpgradeDomainElapsedTimeInMs")]
    pub upgrade_domain_elapsed_time_in_ms: f64,
}
impl ApplicationUpgradeDomainCompletedEvent {
    pub fn new(
        application_event: ApplicationEvent,
        application_type_name: String,
        current_application_type_version: String,
        application_type_version: String,
        upgrade_state: String,
        upgrade_domains: String,
        upgrade_domain_elapsed_time_in_ms: f64,
    ) -> Self {
        Self {
            application_event,
            application_type_name,
            current_application_type_version,
            application_type_version,
            upgrade_state,
            upgrade_domains,
            upgrade_domain_elapsed_time_in_ms,
        }
    }
}
#[doc = "Describes the parameters for an application upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationUpgradeProgressInfo {
    #[doc = "The name of the target application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<TargetApplicationName>,
    #[doc = "The application type name as defined in the application manifest."]
    #[serde(rename = "TypeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<ApplicationTypeName>,
    #[doc = "The target application type version (found in the application manifest) for the application upgrade."]
    #[serde(rename = "TargetApplicationTypeVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_application_type_version: Option<TargetApplicationTypeVersion>,
    #[doc = "List of upgrade domains and their statuses."]
    #[serde(rename = "UpgradeDomains", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domains: Option<UpgradeDomainInfoList>,
    #[doc = "The state of the upgrade domain."]
    #[serde(rename = "UpgradeState", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_state: Option<UpgradeState>,
    #[doc = "The name of the next upgrade domain to be processed."]
    #[serde(rename = "NextUpgradeDomain", default, skip_serializing_if = "Option::is_none")]
    pub next_upgrade_domain: Option<NextUpgradeDomain>,
    #[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
    #[serde(rename = "RollingUpgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub rolling_upgrade_mode: Option<UpgradeMode>,
    #[doc = "Describes the parameters for an application upgrade. Note that upgrade description replaces the existing application description. This means that if the parameters are not specified, the existing parameters on the applications will be overwritten with the empty parameters list. This would result in the application using the default value of the parameters from the application manifest. If you do not want to change any existing parameter values, please get the application parameters first using the GetApplicationInfo query and then supply those values as Parameters in this ApplicationUpgradeDescription."]
    #[serde(rename = "UpgradeDescription", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_description: Option<ApplicationUpgradeDescription>,
    #[doc = "The estimated total amount of time spent processing the overall upgrade."]
    #[serde(rename = "UpgradeDurationInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_duration_in_milliseconds: Option<String>,
    #[doc = "The estimated total amount of time spent processing the current upgrade domain."]
    #[serde(rename = "UpgradeDomainDurationInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain_duration_in_milliseconds: Option<String>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
    #[doc = "Information about the current in-progress upgrade domain."]
    #[serde(rename = "CurrentUpgradeDomainProgress", default, skip_serializing_if = "Option::is_none")]
    pub current_upgrade_domain_progress: Option<CurrentUpgradeDomainProgressInfo>,
    #[doc = "The estimated UTC datetime when the upgrade started."]
    #[serde(rename = "StartTimestampUtc", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp_utc: Option<String>,
    #[doc = "The estimated UTC datetime when the upgrade failed and FailureAction was executed."]
    #[serde(rename = "FailureTimestampUtc", default, skip_serializing_if = "Option::is_none")]
    pub failure_timestamp_utc: Option<String>,
    #[doc = "The cause of an upgrade failure that resulted in FailureAction being executed."]
    #[serde(rename = "FailureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<FailureReason>,
    #[doc = "Information about the upgrade domain progress at the time of upgrade failure."]
    #[serde(rename = "UpgradeDomainProgressAtFailure", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain_progress_at_failure: Option<FailureUpgradeDomainProgressInfo>,
    #[doc = "Additional detailed information about the status of the pending upgrade."]
    #[serde(rename = "UpgradeStatusDetails", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_status_details: Option<String>,
}
impl ApplicationUpgradeProgressInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application Upgrade Rollback Completed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUpgradeRollbackCompletedEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Application type name."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: String,
    #[doc = "Application type version."]
    #[serde(rename = "ApplicationTypeVersion")]
    pub application_type_version: String,
    #[doc = "Describes reason of failure."]
    #[serde(rename = "FailureReason")]
    pub failure_reason: String,
    #[doc = "Overall upgrade time in milli-seconds."]
    #[serde(rename = "OverallUpgradeElapsedTimeInMs")]
    pub overall_upgrade_elapsed_time_in_ms: f64,
}
impl ApplicationUpgradeRollbackCompletedEvent {
    pub fn new(
        application_event: ApplicationEvent,
        application_type_name: String,
        application_type_version: String,
        failure_reason: String,
        overall_upgrade_elapsed_time_in_ms: f64,
    ) -> Self {
        Self {
            application_event,
            application_type_name,
            application_type_version,
            failure_reason,
            overall_upgrade_elapsed_time_in_ms,
        }
    }
}
#[doc = "Application Upgrade Rollback Started event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUpgradeRollbackStartedEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Application type name."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: String,
    #[doc = "Current Application type version."]
    #[serde(rename = "CurrentApplicationTypeVersion")]
    pub current_application_type_version: String,
    #[doc = "Target Application type version."]
    #[serde(rename = "ApplicationTypeVersion")]
    pub application_type_version: String,
    #[doc = "Describes reason of failure."]
    #[serde(rename = "FailureReason")]
    pub failure_reason: String,
    #[doc = "Overall upgrade time in milli-seconds."]
    #[serde(rename = "OverallUpgradeElapsedTimeInMs")]
    pub overall_upgrade_elapsed_time_in_ms: f64,
}
impl ApplicationUpgradeRollbackStartedEvent {
    pub fn new(
        application_event: ApplicationEvent,
        application_type_name: String,
        current_application_type_version: String,
        application_type_version: String,
        failure_reason: String,
        overall_upgrade_elapsed_time_in_ms: f64,
    ) -> Self {
        Self {
            application_event,
            application_type_name,
            current_application_type_version,
            application_type_version,
            failure_reason,
            overall_upgrade_elapsed_time_in_ms,
        }
    }
}
#[doc = "Application Upgrade Started event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUpgradeStartedEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Application type name."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: String,
    #[doc = "Current Application type version."]
    #[serde(rename = "CurrentApplicationTypeVersion")]
    pub current_application_type_version: String,
    #[doc = "Target Application type version."]
    #[serde(rename = "ApplicationTypeVersion")]
    pub application_type_version: String,
    #[doc = "Type of upgrade."]
    #[serde(rename = "UpgradeType")]
    pub upgrade_type: String,
    #[doc = "Mode of upgrade."]
    #[serde(rename = "RollingUpgradeMode")]
    pub rolling_upgrade_mode: String,
    #[doc = "Action if failed."]
    #[serde(rename = "FailureAction")]
    pub failure_action: String,
}
impl ApplicationUpgradeStartedEvent {
    pub fn new(
        application_event: ApplicationEvent,
        application_type_name: String,
        current_application_type_version: String,
        application_type_version: String,
        upgrade_type: String,
        rolling_upgrade_mode: String,
        failure_action: String,
    ) -> Self {
        Self {
            application_event,
            application_type_name,
            current_application_type_version,
            application_type_version,
            upgrade_type,
            rolling_upgrade_mode,
            failure_action,
        }
    }
}
#[doc = "Describes the parameters for updating an ongoing application upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUpgradeUpdateDescription {
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name")]
    pub name: ApplicationName,
    #[doc = "The kind of upgrade out of the following possible values."]
    #[serde(rename = "UpgradeKind")]
    pub upgrade_kind: UpgradeKind,
    #[doc = "Defines a health policy used to evaluate the health of an application or one of its children entities."]
    #[serde(rename = "ApplicationHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy: Option<ApplicationHealthPolicy>,
    #[doc = "Describes the parameters for updating a rolling upgrade of application or cluster."]
    #[serde(rename = "UpdateDescription", default, skip_serializing_if = "Option::is_none")]
    pub update_description: Option<RollingUpgradeUpdateDescription>,
}
impl ApplicationUpgradeUpdateDescription {
    pub fn new(name: ApplicationName, upgrade_kind: UpgradeKind) -> Self {
        Self {
            name,
            upgrade_kind,
            application_health_policy: None,
            update_description: None,
        }
    }
}
#[doc = "Represents health evaluation for applications, containing health evaluations for each unhealthy application that impacted current aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationsHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Maximum allowed percentage of unhealthy applications from the ClusterHealthPolicy."]
    #[serde(rename = "MaxPercentUnhealthyApplications", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_unhealthy_applications: Option<i64>,
    #[doc = "Total number of applications from the health store."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl ApplicationsHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            max_percent_unhealthy_applications: None,
            total_count: None,
            unhealthy_evaluations: None,
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
#[doc = "Represents a scaling trigger related to an average load of a metric/resource of a partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AveragePartitionLoadScalingTrigger {
    #[serde(flatten)]
    pub scaling_trigger_description: ScalingTriggerDescription,
    #[doc = "The name of the metric for which usage should be tracked."]
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[doc = "The lower limit of the load below which a scale in operation should be performed."]
    #[serde(rename = "LowerLoadThreshold")]
    pub lower_load_threshold: String,
    #[doc = "The upper limit of the load beyond which a scale out operation should be performed."]
    #[serde(rename = "UpperLoadThreshold")]
    pub upper_load_threshold: String,
    #[doc = "The period in seconds on which a decision is made whether to scale or not."]
    #[serde(rename = "ScaleIntervalInSeconds")]
    pub scale_interval_in_seconds: i64,
}
impl AveragePartitionLoadScalingTrigger {
    pub fn new(
        scaling_trigger_description: ScalingTriggerDescription,
        metric_name: String,
        lower_load_threshold: String,
        upper_load_threshold: String,
        scale_interval_in_seconds: i64,
    ) -> Self {
        Self {
            scaling_trigger_description,
            metric_name,
            lower_load_threshold,
            upper_load_threshold,
            scale_interval_in_seconds,
        }
    }
}
#[doc = "Represents a scaling policy related to an average load of a metric/resource of a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AverageServiceLoadScalingTrigger {
    #[serde(flatten)]
    pub scaling_trigger_description: ScalingTriggerDescription,
    #[doc = "The name of the metric for which usage should be tracked."]
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[doc = "The lower limit of the load below which a scale in operation should be performed."]
    #[serde(rename = "LowerLoadThreshold")]
    pub lower_load_threshold: String,
    #[doc = "The upper limit of the load beyond which a scale out operation should be performed."]
    #[serde(rename = "UpperLoadThreshold")]
    pub upper_load_threshold: String,
    #[doc = "The period in seconds on which a decision is made whether to scale or not."]
    #[serde(rename = "ScaleIntervalInSeconds")]
    pub scale_interval_in_seconds: i64,
}
impl AverageServiceLoadScalingTrigger {
    pub fn new(
        scaling_trigger_description: ScalingTriggerDescription,
        metric_name: String,
        lower_load_threshold: String,
        upper_load_threshold: String,
        scale_interval_in_seconds: i64,
    ) -> Self {
        Self {
            scaling_trigger_description,
            metric_name,
            lower_load_threshold,
            upper_load_threshold,
            scale_interval_in_seconds,
        }
    }
}
#[doc = "Describes the parameters for Azure blob store used for storing and enumerating backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBlobBackupStorageDescription {
    #[serde(flatten)]
    pub backup_storage_description: BackupStorageDescription,
    #[doc = "The connection string to connect to the Azure blob store."]
    #[serde(rename = "ConnectionString")]
    pub connection_string: String,
    #[doc = "The name of the container in the blob store to store and enumerate backups from."]
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}
impl AzureBlobBackupStorageDescription {
    pub fn new(backup_storage_description: BackupStorageDescription, connection_string: String, container_name: String) -> Self {
        Self {
            backup_storage_description,
            connection_string,
            container_name,
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
    pub fluentd_config_url: Option<String>,
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
#[doc = "Describes the backup configuration information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupConfigurationInfo {
    #[doc = "The entity type of a Service Fabric entity such as Application, Service or a Partition where periodic backups can be enabled."]
    #[serde(rename = "Kind")]
    pub kind: BackupEntityKind,
    #[doc = "The name of the backup policy which is applicable to this Service Fabric application or service or partition."]
    #[serde(rename = "PolicyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[doc = "Specifies the scope at which the backup policy is applied."]
    #[serde(rename = "PolicyInheritedFrom", default, skip_serializing_if = "Option::is_none")]
    pub policy_inherited_from: Option<BackupPolicyScope>,
    #[doc = "Describes the backup suspension details."]
    #[serde(rename = "SuspensionInfo", default, skip_serializing_if = "Option::is_none")]
    pub suspension_info: Option<BackupSuspensionInfo>,
}
impl BackupConfigurationInfo {
    pub fn new(kind: BackupEntityKind) -> Self {
        Self {
            kind,
            policy_name: None,
            policy_inherited_from: None,
            suspension_info: None,
        }
    }
}
#[doc = "Describes the Service Fabric entity that is configured for backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupEntity {
    #[doc = "The entity type of a Service Fabric entity such as Application, Service or a Partition where periodic backups can be enabled."]
    #[serde(rename = "EntityKind")]
    pub entity_kind: BackupEntityKind,
}
impl BackupEntity {
    pub fn new(entity_kind: BackupEntityKind) -> Self {
        Self { entity_kind }
    }
}
#[doc = "The entity type of a Service Fabric entity such as Application, Service or a Partition where periodic backups can be enabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupEntityKind")]
pub enum BackupEntityKind {
    Invalid,
    Partition,
    Service,
    Application,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupEntityKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupEntityKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupEntityKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("BackupEntityKind", 0u32, "Invalid"),
            Self::Partition => serializer.serialize_unit_variant("BackupEntityKind", 1u32, "Partition"),
            Self::Service => serializer.serialize_unit_variant("BackupEntityKind", 2u32, "Service"),
            Self::Application => serializer.serialize_unit_variant("BackupEntityKind", 3u32, "Application"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a backup point which can be used to trigger a restore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupInfo {
    #[doc = "Unique backup ID ."]
    #[serde(rename = "BackupId", default, skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    #[doc = "Unique backup chain ID. All backups part of the same chain has the same backup chain id. A backup chain is comprised of 1 full backup and multiple incremental backups."]
    #[serde(rename = "BackupChainId", default, skip_serializing_if = "Option::is_none")]
    pub backup_chain_id: Option<String>,
    #[doc = "Name of the Service Fabric application this partition backup belongs to."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[doc = "Name of the Service Fabric service this partition backup belongs to."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[doc = "Information about the partition identity, partitioning scheme and keys supported by it."]
    #[serde(rename = "PartitionInformation", default, skip_serializing_if = "Option::is_none")]
    pub partition_information: Option<PartitionInformation>,
    #[doc = "Location of the backup, relative to the backup store."]
    #[serde(rename = "BackupLocation", default, skip_serializing_if = "Option::is_none")]
    pub backup_location: Option<String>,
    #[doc = "Describes the type of backup, whether its full or incremental."]
    #[serde(rename = "BackupType", default, skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<BackupType>,
    #[doc = "An Epoch is a configuration number for the partition as a whole. When the configuration of the replica set changes, for example when the Primary replica changes, the operations that are replicated from the new Primary replica are said to be a new Epoch from the ones which were sent by the old Primary replica."]
    #[serde(rename = "EpochOfLastBackupRecord", default, skip_serializing_if = "Option::is_none")]
    pub epoch_of_last_backup_record: Option<Epoch>,
    #[doc = "LSN of the last record in this backup."]
    #[serde(rename = "LsnOfLastBackupRecord", default, skip_serializing_if = "Option::is_none")]
    pub lsn_of_last_backup_record: Option<String>,
    #[doc = "The date time when this backup was taken."]
    #[serde(rename = "CreationTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub creation_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Manifest Version of the service this partition backup belongs to."]
    #[serde(rename = "ServiceManifestVersion", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_version: Option<String>,
    #[doc = "Error object containing error code and error message."]
    #[serde(rename = "FailureError", default, skip_serializing_if = "Option::is_none")]
    pub failure_error: Option<FabricErrorError>,
}
impl BackupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the parameters for triggering partition's backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupPartitionDescription {
    #[doc = "Describes the parameters for the backup storage."]
    #[serde(rename = "BackupStorage", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage: Option<BackupStorageDescription>,
}
impl BackupPartitionDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a backup policy for configuring periodic backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicyDescription {
    #[doc = "The unique name identifying this backup policy."]
    #[serde(rename = "Name")]
    pub name: String,
    #[doc = "Specifies whether to trigger restore automatically using the latest available backup in case the partition experiences a data loss event."]
    #[serde(rename = "AutoRestoreOnDataLoss")]
    pub auto_restore_on_data_loss: bool,
    #[doc = "Defines the maximum number of incremental backups to be taken between two full backups. This is just the upper limit. A full backup may be taken before specified number of incremental backups are completed in one of the following conditions\n- The replica has never taken a full backup since it has become primary,\n- Some of the log records since the last backup has been truncated, or\n- Replica passed the MaxAccumulatedBackupLogSizeInMB limit."]
    #[serde(rename = "MaxIncrementalBackups")]
    pub max_incremental_backups: i64,
    #[doc = "Describes the backup schedule parameters."]
    #[serde(rename = "Schedule")]
    pub schedule: BackupScheduleDescription,
    #[doc = "Describes the parameters for the backup storage."]
    #[serde(rename = "Storage")]
    pub storage: BackupStorageDescription,
    #[doc = "Describes the retention policy configured."]
    #[serde(rename = "RetentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicyDescription>,
}
impl BackupPolicyDescription {
    pub fn new(
        name: String,
        auto_restore_on_data_loss: bool,
        max_incremental_backups: i64,
        schedule: BackupScheduleDescription,
        storage: BackupStorageDescription,
    ) -> Self {
        Self {
            name,
            auto_restore_on_data_loss,
            max_incremental_backups,
            schedule,
            storage,
            retention_policy: None,
        }
    }
}
#[doc = "Specifies the scope at which the backup policy is applied."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupPolicyScope")]
pub enum BackupPolicyScope {
    Invalid,
    Partition,
    Service,
    Application,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupPolicyScope {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupPolicyScope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupPolicyScope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("BackupPolicyScope", 0u32, "Invalid"),
            Self::Partition => serializer.serialize_unit_variant("BackupPolicyScope", 1u32, "Partition"),
            Self::Service => serializer.serialize_unit_variant("BackupPolicyScope", 2u32, "Service"),
            Self::Application => serializer.serialize_unit_variant("BackupPolicyScope", 3u32, "Application"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the progress of a partition's backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupProgressInfo {
    #[doc = "Represents the current state of the partition backup operation."]
    #[serde(rename = "BackupState", default, skip_serializing_if = "Option::is_none")]
    pub backup_state: Option<BackupState>,
    #[doc = "TimeStamp in UTC when operation succeeded or failed."]
    #[serde(rename = "TimeStampUtc", with = "azure_core::date::rfc3339::option")]
    pub time_stamp_utc: Option<time::OffsetDateTime>,
    #[doc = "Unique ID of the newly created backup."]
    #[serde(rename = "BackupId", default, skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    #[doc = "Location, relative to the backup store, of the newly created backup."]
    #[serde(rename = "BackupLocation", default, skip_serializing_if = "Option::is_none")]
    pub backup_location: Option<String>,
    #[doc = "An Epoch is a configuration number for the partition as a whole. When the configuration of the replica set changes, for example when the Primary replica changes, the operations that are replicated from the new Primary replica are said to be a new Epoch from the ones which were sent by the old Primary replica."]
    #[serde(rename = "EpochOfLastBackupRecord", default, skip_serializing_if = "Option::is_none")]
    pub epoch_of_last_backup_record: Option<Epoch>,
    #[doc = "The LSN of last record included in backup."]
    #[serde(rename = "LsnOfLastBackupRecord", default, skip_serializing_if = "Option::is_none")]
    pub lsn_of_last_backup_record: Option<String>,
    #[doc = "Error object containing error code and error message."]
    #[serde(rename = "FailureError", default, skip_serializing_if = "Option::is_none")]
    pub failure_error: Option<FabricErrorError>,
}
impl BackupProgressInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the backup schedule parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupScheduleDescription {
    #[doc = "The kind of backup schedule, time based or frequency based."]
    #[serde(rename = "ScheduleKind")]
    pub schedule_kind: BackupScheduleKind,
}
impl BackupScheduleDescription {
    pub fn new(schedule_kind: BackupScheduleKind) -> Self {
        Self { schedule_kind }
    }
}
#[doc = "Describes the frequency with which to run the time based backup schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupScheduleFrequencyType")]
pub enum BackupScheduleFrequencyType {
    Invalid,
    Daily,
    Weekly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupScheduleFrequencyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupScheduleFrequencyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupScheduleFrequencyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("BackupScheduleFrequencyType", 0u32, "Invalid"),
            Self::Daily => serializer.serialize_unit_variant("BackupScheduleFrequencyType", 1u32, "Daily"),
            Self::Weekly => serializer.serialize_unit_variant("BackupScheduleFrequencyType", 2u32, "Weekly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The kind of backup schedule, time based or frequency based."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupScheduleKind")]
pub enum BackupScheduleKind {
    Invalid,
    TimeBased,
    FrequencyBased,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupScheduleKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupScheduleKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupScheduleKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("BackupScheduleKind", 0u32, "Invalid"),
            Self::TimeBased => serializer.serialize_unit_variant("BackupScheduleKind", 1u32, "TimeBased"),
            Self::FrequencyBased => serializer.serialize_unit_variant("BackupScheduleKind", 2u32, "FrequencyBased"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents the current state of the partition backup operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupState")]
pub enum BackupState {
    Invalid,
    Accepted,
    BackupInProgress,
    Success,
    Failure,
    Timeout,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("BackupState", 0u32, "Invalid"),
            Self::Accepted => serializer.serialize_unit_variant("BackupState", 1u32, "Accepted"),
            Self::BackupInProgress => serializer.serialize_unit_variant("BackupState", 2u32, "BackupInProgress"),
            Self::Success => serializer.serialize_unit_variant("BackupState", 3u32, "Success"),
            Self::Failure => serializer.serialize_unit_variant("BackupState", 4u32, "Failure"),
            Self::Timeout => serializer.serialize_unit_variant("BackupState", 5u32, "Timeout"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the parameters for the backup storage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupStorageDescription {
    #[doc = "The kind of backup storage, where backups are saved."]
    #[serde(rename = "StorageKind")]
    pub storage_kind: BackupStorageKind,
    #[doc = "Friendly name for this backup storage."]
    #[serde(rename = "FriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl BackupStorageDescription {
    pub fn new(storage_kind: BackupStorageKind) -> Self {
        Self {
            storage_kind,
            friendly_name: None,
        }
    }
}
#[doc = "The kind of backup storage, where backups are saved."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupStorageKind")]
pub enum BackupStorageKind {
    Invalid,
    FileShare,
    AzureBlobStore,
    DsmsAzureBlobStore,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupStorageKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupStorageKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupStorageKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("BackupStorageKind", 0u32, "Invalid"),
            Self::FileShare => serializer.serialize_unit_variant("BackupStorageKind", 1u32, "FileShare"),
            Self::AzureBlobStore => serializer.serialize_unit_variant("BackupStorageKind", 2u32, "AzureBlobStore"),
            Self::DsmsAzureBlobStore => serializer.serialize_unit_variant("BackupStorageKind", 3u32, "DsmsAzureBlobStore"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the backup suspension details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupSuspensionInfo {
    #[doc = "Indicates whether periodic backup is suspended at this level or not."]
    #[serde(rename = "IsSuspended", default, skip_serializing_if = "Option::is_none")]
    pub is_suspended: Option<bool>,
    #[doc = "Specifies the scope at which the backup suspension was applied."]
    #[serde(rename = "SuspensionInheritedFrom", default, skip_serializing_if = "Option::is_none")]
    pub suspension_inherited_from: Option<BackupSuspensionScope>,
}
impl BackupSuspensionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the scope at which the backup suspension was applied."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupSuspensionScope")]
pub enum BackupSuspensionScope {
    Invalid,
    Partition,
    Service,
    Application,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupSuspensionScope {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupSuspensionScope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupSuspensionScope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("BackupSuspensionScope", 0u32, "Invalid"),
            Self::Partition => serializer.serialize_unit_variant("BackupSuspensionScope", 1u32, "Partition"),
            Self::Service => serializer.serialize_unit_variant("BackupSuspensionScope", 2u32, "Service"),
            Self::Application => serializer.serialize_unit_variant("BackupSuspensionScope", 3u32, "Application"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the type of backup, whether its full or incremental."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupType")]
pub enum BackupType {
    Invalid,
    Full,
    Incremental,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("BackupType", 0u32, "Invalid"),
            Self::Full => serializer.serialize_unit_variant("BackupType", 1u32, "Full"),
            Self::Incremental => serializer.serialize_unit_variant("BackupType", 2u32, "Incremental"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes basic retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicRetentionPolicyDescription {
    #[serde(flatten)]
    pub retention_policy_description: RetentionPolicyDescription,
    #[doc = "It is the minimum duration for which a backup created, will remain stored in the storage and might get deleted after that span of time. It should be specified in ISO8601 format."]
    #[serde(rename = "RetentionDuration")]
    pub retention_duration: String,
    #[doc = "It is the minimum number of backups to be retained at any point of time. If specified with a non zero value, backups will not be deleted even if the backups have gone past retention duration and have number of backups less than or equal to it."]
    #[serde(rename = "MinimumNumberOfBackups", default, skip_serializing_if = "Option::is_none")]
    pub minimum_number_of_backups: Option<i64>,
}
impl BasicRetentionPolicyDescription {
    pub fn new(retention_policy_description: RetentionPolicyDescription, retention_duration: String) -> Self {
        Self {
            retention_policy_description,
            retention_duration,
            minimum_number_of_backups: None,
        }
    }
}
#[doc = "Describes a Service Fabric property value of type Binary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BinaryPropertyValue {
    #[serde(flatten)]
    pub property_value: PropertyValue,
    #[doc = "Array of bytes to be sent as an integer array. Each element of array is a number between 0 and 255."]
    #[serde(rename = "Data")]
    pub data: ByteArray,
}
impl BinaryPropertyValue {
    pub fn new(property_value: PropertyValue, data: ByteArray) -> Self {
        Self { property_value, data }
    }
}
pub type ByteArray = Vec<i64>;
#[doc = "Contains a description of Chaos."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Chaos {
    #[doc = "Defines all the parameters to configure a Chaos run."]
    #[serde(rename = "ChaosParameters", default, skip_serializing_if = "Option::is_none")]
    pub chaos_parameters: Option<ChaosParameters>,
    #[doc = "Current status of the Chaos run."]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ChaosStatus>,
    #[doc = "Current status of the schedule."]
    #[serde(rename = "ScheduleStatus", default, skip_serializing_if = "Option::is_none")]
    pub schedule_status: Option<ChaosScheduleStatus>,
}
impl Chaos {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Chaos Restart Code Package Fault Scheduled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChaosCodePackageRestartScheduledEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Id of fault group."]
    #[serde(rename = "FaultGroupId")]
    pub fault_group_id: String,
    #[doc = "Id of fault."]
    #[serde(rename = "FaultId")]
    pub fault_id: String,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName")]
    pub node_name: NodeName,
    #[doc = "Service manifest name."]
    #[serde(rename = "ServiceManifestName")]
    pub service_manifest_name: String,
    #[doc = "Code package name."]
    #[serde(rename = "CodePackageName")]
    pub code_package_name: String,
    #[doc = "Id of Service package activation."]
    #[serde(rename = "ServicePackageActivationId")]
    pub service_package_activation_id: String,
}
impl ChaosCodePackageRestartScheduledEvent {
    pub fn new(
        application_event: ApplicationEvent,
        fault_group_id: String,
        fault_id: String,
        node_name: NodeName,
        service_manifest_name: String,
        code_package_name: String,
        service_package_activation_id: String,
    ) -> Self {
        Self {
            application_event,
            fault_group_id,
            fault_id,
            node_name,
            service_manifest_name,
            code_package_name,
            service_package_activation_id,
        }
    }
}
#[doc = "Describes a map, which is a collection of (string, string) type key-value pairs. The map can be used to record information about\nthe Chaos run. There cannot be more than 100 such pairs and each string (key or value) can be at most 4095 characters long.\nThis map is set by the starter of the Chaos run to optionally store the context about the specific run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChaosContext {
    #[doc = "Describes a map that contains a collection of ChaosContextMapItem's."]
    #[serde(rename = "Map", default, skip_serializing_if = "Option::is_none")]
    pub map: Option<ChaosContextMap>,
}
impl ChaosContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a map that contains a collection of ChaosContextMapItem's."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChaosContextMap {}
impl ChaosContextMap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an event generated during a Chaos run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChaosEvent {
    #[doc = "The kind of Chaos event."]
    #[serde(rename = "Kind")]
    pub kind: ChaosEventKind,
    #[doc = "The UTC timestamp when this Chaos event was generated."]
    #[serde(rename = "TimeStampUtc", with = "azure_core::date::rfc3339")]
    pub time_stamp_utc: time::OffsetDateTime,
}
impl ChaosEvent {
    pub fn new(kind: ChaosEventKind, time_stamp_utc: time::OffsetDateTime) -> Self {
        Self { kind, time_stamp_utc }
    }
}
pub type ChaosEventHistory = Vec<ChaosEventWrapper>;
#[doc = "The kind of Chaos event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ChaosEventKind")]
pub enum ChaosEventKind {
    Invalid,
    Started,
    ExecutingFaults,
    Waiting,
    ValidationFailed,
    TestError,
    Stopped,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ChaosEventKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ChaosEventKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ChaosEventKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ChaosEventKind", 0u32, "Invalid"),
            Self::Started => serializer.serialize_unit_variant("ChaosEventKind", 1u32, "Started"),
            Self::ExecutingFaults => serializer.serialize_unit_variant("ChaosEventKind", 2u32, "ExecutingFaults"),
            Self::Waiting => serializer.serialize_unit_variant("ChaosEventKind", 3u32, "Waiting"),
            Self::ValidationFailed => serializer.serialize_unit_variant("ChaosEventKind", 4u32, "ValidationFailed"),
            Self::TestError => serializer.serialize_unit_variant("ChaosEventKind", 5u32, "TestError"),
            Self::Stopped => serializer.serialize_unit_variant("ChaosEventKind", 6u32, "Stopped"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Wrapper object for Chaos event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChaosEventWrapper {
    #[doc = "Represents an event generated during a Chaos run."]
    #[serde(rename = "ChaosEvent", default, skip_serializing_if = "Option::is_none")]
    pub chaos_event: Option<ChaosEvent>,
}
impl ChaosEventWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the list of Chaos events and the continuation token to get the next segment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChaosEventsSegment {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "An list of Chaos events that were generated during the time range passed into the GetChaosReport API call."]
    #[serde(rename = "History", default, skip_serializing_if = "Option::is_none")]
    pub history: Option<ChaosEventHistory>,
}
impl ChaosEventsSegment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Chaos Restart Node Fault Scheduled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChaosNodeRestartScheduledEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstanceId")]
    pub node_instance_id: i64,
    #[doc = "Id of fault group."]
    #[serde(rename = "FaultGroupId")]
    pub fault_group_id: String,
    #[doc = "Id of fault."]
    #[serde(rename = "FaultId")]
    pub fault_id: String,
}
impl ChaosNodeRestartScheduledEvent {
    pub fn new(node_event: NodeEvent, node_instance_id: i64, fault_group_id: String, fault_id: String) -> Self {
        Self {
            node_event,
            node_instance_id,
            fault_group_id,
            fault_id,
        }
    }
}
#[doc = "Defines all the parameters to configure a Chaos run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChaosParameters {
    #[doc = "Total time (in seconds) for which Chaos will run before automatically stopping. The maximum allowed value is 4,294,967,295 (System.UInt32.MaxValue)."]
    #[serde(rename = "TimeToRunInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub time_to_run_in_seconds: Option<String>,
    #[doc = "The maximum amount of time to wait for all cluster entities to become stable and healthy. Chaos executes in iterations and at the start of each iteration it validates the health of cluster entities.\nDuring validation if a cluster entity is not stable and healthy within MaxClusterStabilizationTimeoutInSeconds, Chaos generates a validation failed event."]
    #[serde(
        rename = "MaxClusterStabilizationTimeoutInSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_cluster_stabilization_timeout_in_seconds: Option<i64>,
    #[doc = "MaxConcurrentFaults is the maximum number of concurrent faults induced per iteration.\nChaos executes in iterations and two consecutive iterations are separated by a validation phase.\nThe higher the concurrency, the more aggressive the injection of faults, leading to inducing more complex series of states to uncover bugs.\nThe recommendation is to start with a value of 2 or 3 and to exercise caution while moving up."]
    #[serde(rename = "MaxConcurrentFaults", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_faults: Option<i64>,
    #[doc = "Enables or disables the move primary and move secondary faults."]
    #[serde(rename = "EnableMoveReplicaFaults", default, skip_serializing_if = "Option::is_none")]
    pub enable_move_replica_faults: Option<bool>,
    #[doc = "Wait time (in seconds) between consecutive faults within a single iteration.\nThe larger the value, the lower the overlapping between faults and the simpler the sequence of state transitions that the cluster goes through.\nThe recommendation is to start with a value between 1 and 5 and exercise caution while moving up."]
    #[serde(rename = "WaitTimeBetweenFaultsInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub wait_time_between_faults_in_seconds: Option<i64>,
    #[doc = "Time-separation (in seconds) between two consecutive iterations of Chaos.\nThe larger the value, the lower the fault injection rate."]
    #[serde(rename = "WaitTimeBetweenIterationsInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub wait_time_between_iterations_in_seconds: Option<i64>,
    #[doc = "Defines a health policy used to evaluate the health of the cluster or of a cluster node."]
    #[serde(rename = "ClusterHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cluster_health_policy: Option<ClusterHealthPolicy>,
    #[doc = "Describes a map, which is a collection of (string, string) type key-value pairs. The map can be used to record information about\nthe Chaos run. There cannot be more than 100 such pairs and each string (key or value) can be at most 4095 characters long.\nThis map is set by the starter of the Chaos run to optionally store the context about the specific run."]
    #[serde(rename = "Context", default, skip_serializing_if = "Option::is_none")]
    pub context: Option<ChaosContext>,
    #[doc = "Defines all filters for targeted Chaos faults, for example, faulting only certain node types or faulting only certain applications.\nIf ChaosTargetFilter is not used, Chaos faults all cluster entities. If ChaosTargetFilter is used, Chaos faults only the entities that meet the ChaosTargetFilter\nspecification. NodeTypeInclusionList and ApplicationInclusionList allow a union semantics only. It is not possible to specify an intersection\nof NodeTypeInclusionList and ApplicationInclusionList. For example, it is not possible to specify \"fault this application only when it is on that node type.\"\nOnce an entity is included in either NodeTypeInclusionList or ApplicationInclusionList, that entity cannot be excluded using ChaosTargetFilter. Even if\napplicationX does not appear in ApplicationInclusionList, in some Chaos iteration applicationX can be faulted because it happens to be on a node of nodeTypeY that is included\nin NodeTypeInclusionList. If both NodeTypeInclusionList and ApplicationInclusionList are null or empty, an ArgumentException is thrown."]
    #[serde(rename = "ChaosTargetFilter", default, skip_serializing_if = "Option::is_none")]
    pub chaos_target_filter: Option<ChaosTargetFilter>,
}
impl ChaosParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines an item in ChaosParametersDictionary of the Chaos Schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChaosParametersDictionaryItem {
    #[doc = "The key identifying the Chaos Parameter in the dictionary. This key is referenced by Chaos Schedule Jobs."]
    #[serde(rename = "Key")]
    pub key: String,
    #[doc = "Defines all the parameters to configure a Chaos run."]
    #[serde(rename = "Value")]
    pub value: ChaosParameters,
}
impl ChaosParametersDictionaryItem {
    pub fn new(key: String, value: ChaosParameters) -> Self {
        Self { key, value }
    }
}
#[doc = "Chaos Move Primary Fault Scheduled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChaosPartitionPrimaryMoveScheduledEvent {
    #[serde(flatten)]
    pub partition_event: PartitionEvent,
    #[doc = "Id of fault group."]
    #[serde(rename = "FaultGroupId")]
    pub fault_group_id: String,
    #[doc = "Id of fault."]
    #[serde(rename = "FaultId")]
    pub fault_id: String,
    #[doc = "Service name."]
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeTo")]
    pub node_to: NodeName,
    #[doc = "Indicates a forced move."]
    #[serde(rename = "ForcedMove")]
    pub forced_move: bool,
}
impl ChaosPartitionPrimaryMoveScheduledEvent {
    pub fn new(
        partition_event: PartitionEvent,
        fault_group_id: String,
        fault_id: String,
        service_name: String,
        node_to: NodeName,
        forced_move: bool,
    ) -> Self {
        Self {
            partition_event,
            fault_group_id,
            fault_id,
            service_name,
            node_to,
            forced_move,
        }
    }
}
#[doc = "Chaos Move Secondary Fault Scheduled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChaosPartitionSecondaryMoveScheduledEvent {
    #[serde(flatten)]
    pub partition_event: PartitionEvent,
    #[doc = "Id of fault group."]
    #[serde(rename = "FaultGroupId")]
    pub fault_group_id: String,
    #[doc = "Id of fault."]
    #[serde(rename = "FaultId")]
    pub fault_id: String,
    #[doc = "Service name."]
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "SourceNode")]
    pub source_node: NodeName,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "DestinationNode")]
    pub destination_node: NodeName,
    #[doc = "Indicates a forced move."]
    #[serde(rename = "ForcedMove")]
    pub forced_move: bool,
}
impl ChaosPartitionSecondaryMoveScheduledEvent {
    pub fn new(
        partition_event: PartitionEvent,
        fault_group_id: String,
        fault_id: String,
        service_name: String,
        source_node: NodeName,
        destination_node: NodeName,
        forced_move: bool,
    ) -> Self {
        Self {
            partition_event,
            fault_group_id,
            fault_id,
            service_name,
            source_node,
            destination_node,
            forced_move,
        }
    }
}
#[doc = "Chaos Remove Replica Fault Scheduled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChaosReplicaRemovalScheduledEvent {
    #[serde(flatten)]
    pub replica_event: ReplicaEvent,
    #[doc = "Id of fault group."]
    #[serde(rename = "FaultGroupId")]
    pub fault_group_id: String,
    #[doc = "Id of fault."]
    #[serde(rename = "FaultId")]
    pub fault_id: String,
    #[doc = "Service name."]
    #[serde(rename = "ServiceUri")]
    pub service_uri: String,
}
impl ChaosReplicaRemovalScheduledEvent {
    pub fn new(replica_event: ReplicaEvent, fault_group_id: String, fault_id: String, service_uri: String) -> Self {
        Self {
            replica_event,
            fault_group_id,
            fault_id,
            service_uri,
        }
    }
}
#[doc = "Chaos Restart Replica Fault Scheduled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChaosReplicaRestartScheduledEvent {
    #[serde(flatten)]
    pub replica_event: ReplicaEvent,
    #[doc = "Id of fault group."]
    #[serde(rename = "FaultGroupId")]
    pub fault_group_id: String,
    #[doc = "Id of fault."]
    #[serde(rename = "FaultId")]
    pub fault_id: String,
    #[doc = "Service name."]
    #[serde(rename = "ServiceUri")]
    pub service_uri: String,
}
impl ChaosReplicaRestartScheduledEvent {
    pub fn new(replica_event: ReplicaEvent, fault_group_id: String, fault_id: String, service_uri: String) -> Self {
        Self {
            replica_event,
            fault_group_id,
            fault_id,
            service_uri,
        }
    }
}
#[doc = "Defines the schedule used by Chaos."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChaosSchedule {
    #[doc = "The date and time Chaos will start using this schedule."]
    #[serde(rename = "StartDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "The date and time Chaos will continue to use this schedule until."]
    #[serde(rename = "ExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub expiry_date: Option<time::OffsetDateTime>,
    #[doc = "A mapping of string names to Chaos Parameters to be referenced by Chaos Schedule Jobs."]
    #[serde(rename = "ChaosParametersDictionary", default, skip_serializing_if = "Vec::is_empty")]
    pub chaos_parameters_dictionary: Vec<ChaosParametersDictionaryItem>,
    #[doc = "A list of all Chaos Schedule Jobs that will be automated by the schedule."]
    #[serde(rename = "Jobs", default, skip_serializing_if = "Vec::is_empty")]
    pub jobs: Vec<ChaosScheduleJob>,
}
impl ChaosSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the Chaos Schedule used by Chaos and the version of the Chaos Schedule. The version value wraps back to 0 after surpassing 2,147,483,647."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChaosScheduleDescription {
    #[doc = "The version number of the Schedule."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "Defines the schedule used by Chaos."]
    #[serde(rename = "Schedule", default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ChaosSchedule>,
}
impl ChaosScheduleDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a repetition rule and parameters of Chaos to be used with the Chaos Schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChaosScheduleJob {
    #[doc = "A reference to which Chaos Parameters of the Chaos Schedule to use."]
    #[serde(rename = "ChaosParameters", default, skip_serializing_if = "Option::is_none")]
    pub chaos_parameters: Option<String>,
    #[doc = "Defines the days of the week that a Chaos Schedule Job will run for."]
    #[serde(rename = "Days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<ChaosScheduleJobActiveDaysOfWeek>,
    #[doc = "A list of Time Ranges that specify when during active days that this job will run. The times are interpreted as UTC."]
    #[serde(rename = "Times", default, skip_serializing_if = "Vec::is_empty")]
    pub times: Vec<TimeRange>,
}
impl ChaosScheduleJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the days of the week that a Chaos Schedule Job will run for."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChaosScheduleJobActiveDaysOfWeek {
    #[doc = "Indicates if the Chaos Schedule Job will run on Sunday"]
    #[serde(rename = "Sunday", default, skip_serializing_if = "Option::is_none")]
    pub sunday: Option<bool>,
    #[doc = "Indicates if the Chaos Schedule Job will run on Monday"]
    #[serde(rename = "Monday", default, skip_serializing_if = "Option::is_none")]
    pub monday: Option<bool>,
    #[doc = "Indicates if the Chaos Schedule Job will run on Tuesday"]
    #[serde(rename = "Tuesday", default, skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<bool>,
    #[doc = "Indicates if the Chaos Schedule Job will run on Wednesday"]
    #[serde(rename = "Wednesday", default, skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<bool>,
    #[doc = "Indicates if the Chaos Schedule Job will run on Thursday"]
    #[serde(rename = "Thursday", default, skip_serializing_if = "Option::is_none")]
    pub thursday: Option<bool>,
    #[doc = "Indicates if the Chaos Schedule Job will run on Friday"]
    #[serde(rename = "Friday", default, skip_serializing_if = "Option::is_none")]
    pub friday: Option<bool>,
    #[doc = "Indicates if the Chaos Schedule Job will run on Saturday"]
    #[serde(rename = "Saturday", default, skip_serializing_if = "Option::is_none")]
    pub saturday: Option<bool>,
}
impl ChaosScheduleJobActiveDaysOfWeek {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current status of the schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ChaosScheduleStatus")]
pub enum ChaosScheduleStatus {
    Invalid,
    Stopped,
    Active,
    Expired,
    Pending,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ChaosScheduleStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ChaosScheduleStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ChaosScheduleStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ChaosScheduleStatus", 0u32, "Invalid"),
            Self::Stopped => serializer.serialize_unit_variant("ChaosScheduleStatus", 1u32, "Stopped"),
            Self::Active => serializer.serialize_unit_variant("ChaosScheduleStatus", 2u32, "Active"),
            Self::Expired => serializer.serialize_unit_variant("ChaosScheduleStatus", 3u32, "Expired"),
            Self::Pending => serializer.serialize_unit_variant("ChaosScheduleStatus", 4u32, "Pending"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Chaos Started event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChaosStartedEvent {
    #[serde(flatten)]
    pub cluster_event: ClusterEvent,
    #[doc = "Maximum number of concurrent faults."]
    #[serde(rename = "MaxConcurrentFaults")]
    pub max_concurrent_faults: i64,
    #[doc = "Time to run in seconds."]
    #[serde(rename = "TimeToRunInSeconds")]
    pub time_to_run_in_seconds: f64,
    #[doc = "Maximum timeout for cluster stabilization in seconds."]
    #[serde(rename = "MaxClusterStabilizationTimeoutInSeconds")]
    pub max_cluster_stabilization_timeout_in_seconds: f64,
    #[doc = "Wait time between iterations in seconds."]
    #[serde(rename = "WaitTimeBetweenIterationsInSeconds")]
    pub wait_time_between_iterations_in_seconds: f64,
    #[doc = "Wait time between faults in seconds."]
    #[serde(rename = "WaitTimeBetweenFaultsInSeconds")]
    pub wait_time_between_faults_in_seconds: f64,
    #[doc = "Indicates MoveReplica fault is enabled."]
    #[serde(rename = "MoveReplicaFaultEnabled")]
    pub move_replica_fault_enabled: bool,
    #[doc = "List of included Node types."]
    #[serde(rename = "IncludedNodeTypeList")]
    pub included_node_type_list: String,
    #[doc = "List of included Applications."]
    #[serde(rename = "IncludedApplicationList")]
    pub included_application_list: String,
    #[doc = "Health policy."]
    #[serde(rename = "ClusterHealthPolicy")]
    pub cluster_health_policy: String,
    #[doc = "Chaos Context."]
    #[serde(rename = "ChaosContext")]
    pub chaos_context: String,
}
impl ChaosStartedEvent {
    pub fn new(
        cluster_event: ClusterEvent,
        max_concurrent_faults: i64,
        time_to_run_in_seconds: f64,
        max_cluster_stabilization_timeout_in_seconds: f64,
        wait_time_between_iterations_in_seconds: f64,
        wait_time_between_faults_in_seconds: f64,
        move_replica_fault_enabled: bool,
        included_node_type_list: String,
        included_application_list: String,
        cluster_health_policy: String,
        chaos_context: String,
    ) -> Self {
        Self {
            cluster_event,
            max_concurrent_faults,
            time_to_run_in_seconds,
            max_cluster_stabilization_timeout_in_seconds,
            wait_time_between_iterations_in_seconds,
            wait_time_between_faults_in_seconds,
            move_replica_fault_enabled,
            included_node_type_list,
            included_application_list,
            cluster_health_policy,
            chaos_context,
        }
    }
}
#[doc = "Current status of the Chaos run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ChaosStatus")]
pub enum ChaosStatus {
    Invalid,
    Running,
    Stopped,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ChaosStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ChaosStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ChaosStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ChaosStatus", 0u32, "Invalid"),
            Self::Running => serializer.serialize_unit_variant("ChaosStatus", 1u32, "Running"),
            Self::Stopped => serializer.serialize_unit_variant("ChaosStatus", 2u32, "Stopped"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Chaos Stopped event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChaosStoppedEvent {
    #[serde(flatten)]
    pub cluster_event: ClusterEvent,
    #[doc = "Describes reason."]
    #[serde(rename = "Reason")]
    pub reason: String,
}
impl ChaosStoppedEvent {
    pub fn new(cluster_event: ClusterEvent, reason: String) -> Self {
        Self { cluster_event, reason }
    }
}
#[doc = "Defines all filters for targeted Chaos faults, for example, faulting only certain node types or faulting only certain applications.\nIf ChaosTargetFilter is not used, Chaos faults all cluster entities. If ChaosTargetFilter is used, Chaos faults only the entities that meet the ChaosTargetFilter\nspecification. NodeTypeInclusionList and ApplicationInclusionList allow a union semantics only. It is not possible to specify an intersection\nof NodeTypeInclusionList and ApplicationInclusionList. For example, it is not possible to specify \"fault this application only when it is on that node type.\"\nOnce an entity is included in either NodeTypeInclusionList or ApplicationInclusionList, that entity cannot be excluded using ChaosTargetFilter. Even if\napplicationX does not appear in ApplicationInclusionList, in some Chaos iteration applicationX can be faulted because it happens to be on a node of nodeTypeY that is included\nin NodeTypeInclusionList. If both NodeTypeInclusionList and ApplicationInclusionList are null or empty, an ArgumentException is thrown."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChaosTargetFilter {
    #[doc = "A list of node types to include in Chaos faults.\nAll types of faults (restart node, restart code package, remove replica, restart replica, move primary, and move secondary) are enabled for the nodes of these node types.\nIf a node type (say NodeTypeX) does not appear in the NodeTypeInclusionList, then node level faults (like NodeRestart) will never be enabled for the nodes of\nNodeTypeX, but code package and replica faults can still be enabled for NodeTypeX if an application in the ApplicationInclusionList.\nhappens to reside on a node of NodeTypeX.\nAt most 100 node type names can be included in this list, to increase this number, a config upgrade is required for MaxNumberOfNodeTypesInChaosEntityFilter configuration."]
    #[serde(rename = "NodeTypeInclusionList", default, skip_serializing_if = "Vec::is_empty")]
    pub node_type_inclusion_list: Vec<NodeType>,
    #[doc = "A list of application URIs to include in Chaos faults.\nAll replicas belonging to services of these applications are amenable to replica faults (restart replica, remove replica, move primary, and move secondary) by Chaos.\nChaos may restart a code package only if the code package hosts replicas of these applications only.\nIf an application does not appear in this list, it can still be faulted in some Chaos iteration if the application ends up on a node of a node type that is included in NodeTypeInclusionList.\nHowever, if applicationX is tied to nodeTypeY through placement constraints and applicationX is absent from ApplicationInclusionList and nodeTypeY is absent from NodeTypeInclusionList, then applicationX will never be faulted.\nAt most 1000 application names can be included in this list, to increase this number, a config upgrade is required for MaxNumberOfApplicationsInChaosEntityFilter configuration."]
    #[serde(rename = "ApplicationInclusionList", default, skip_serializing_if = "Vec::is_empty")]
    pub application_inclusion_list: Vec<ApplicationName>,
}
impl ChaosTargetFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a PropertyBatchOperation that compares the Boolean existence of a property with the Exists argument.\nThe PropertyBatchOperation operation fails if the property's existence is not equal to the Exists argument.\nThe CheckExistsPropertyBatchOperation is generally used as a precondition for the write operations in the batch.\nNote that if one PropertyBatchOperation in a PropertyBatch fails,\nthe entire batch fails and cannot be committed in a transactional manner."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckExistsPropertyBatchOperation {
    #[serde(flatten)]
    pub property_batch_operation: PropertyBatchOperation,
    #[doc = "Whether or not the property should exist for the operation to pass."]
    #[serde(rename = "Exists")]
    pub exists: bool,
}
impl CheckExistsPropertyBatchOperation {
    pub fn new(property_batch_operation: PropertyBatchOperation, exists: bool) -> Self {
        Self {
            property_batch_operation,
            exists,
        }
    }
}
#[doc = "Compares the Sequence Number of a property with the SequenceNumber argument.\nA property's sequence number can be thought of as that property's version.\nEvery time the property is modified, its sequence number is increased.\nThe sequence number can be found in a property's metadata.\nThe comparison fails if the sequence numbers are not equal.\nCheckSequencePropertyBatchOperation is generally used as a precondition for the write operations in the batch.\nNote that if one PropertyBatchOperation in a PropertyBatch fails,\nthe entire batch fails and cannot be committed in a transactional manner."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckSequencePropertyBatchOperation {
    #[serde(flatten)]
    pub property_batch_operation: PropertyBatchOperation,
    #[doc = "The expected sequence number."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: String,
}
impl CheckSequencePropertyBatchOperation {
    pub fn new(property_batch_operation: PropertyBatchOperation, sequence_number: String) -> Self {
        Self {
            property_batch_operation,
            sequence_number,
        }
    }
}
#[doc = "Represents a PropertyBatchOperation that compares the value of the property with the expected value.\nThe CheckValuePropertyBatchOperation is generally used as a precondition for the write operations in the batch.\nNote that if one PropertyBatchOperation in a PropertyBatch fails,\nthe entire batch fails and cannot be committed in a transactional manner."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckValuePropertyBatchOperation {
    #[serde(flatten)]
    pub property_batch_operation: PropertyBatchOperation,
    #[doc = "Describes a Service Fabric property value."]
    #[serde(rename = "Value")]
    pub value: PropertyValue,
}
impl CheckValuePropertyBatchOperation {
    pub fn new(property_batch_operation: PropertyBatchOperation, value: PropertyValue) -> Self {
        Self {
            property_batch_operation,
            value,
        }
    }
}
#[doc = "Information about the standalone cluster configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterConfiguration {
    #[doc = "The contents of the cluster configuration file."]
    #[serde(rename = "ClusterConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cluster_configuration: Option<String>,
}
impl ClusterConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the parameters for a standalone cluster configuration upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterConfigurationUpgradeDescription {
    #[doc = "The cluster configuration as a JSON string. For example, [this file](https://github.com/Azure-Samples/service-fabric-dotnet-standalone-cluster-configuration/blob/master/Samples/ClusterConfig.Unsecure.DevCluster.json) contains JSON describing the [nodes and other properties of the cluster](https://docs.microsoft.com/azure/service-fabric/service-fabric-cluster-manifest)."]
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: String,
    #[doc = "The length of time between attempts to perform health checks if the application or cluster is not healthy."]
    #[serde(rename = "HealthCheckRetryTimeout", default, skip_serializing_if = "Option::is_none")]
    pub health_check_retry_timeout: Option<String>,
    #[doc = "The length of time to wait after completing an upgrade domain before starting the health checks process."]
    #[serde(rename = "HealthCheckWaitDurationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub health_check_wait_duration_in_seconds: Option<String>,
    #[doc = "The length of time that the application or cluster must remain healthy before the upgrade proceeds to the next upgrade domain."]
    #[serde(rename = "HealthCheckStableDurationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub health_check_stable_duration_in_seconds: Option<String>,
    #[doc = "The timeout for the upgrade domain."]
    #[serde(rename = "UpgradeDomainTimeoutInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain_timeout_in_seconds: Option<String>,
    #[doc = "The upgrade timeout."]
    #[serde(rename = "UpgradeTimeoutInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_timeout_in_seconds: Option<String>,
    #[doc = "The maximum allowed percentage of unhealthy applications during the upgrade. Allowed values are integer values from zero to 100."]
    #[serde(rename = "MaxPercentUnhealthyApplications", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_unhealthy_applications: Option<i64>,
    #[doc = "The maximum allowed percentage of unhealthy nodes during the upgrade. Allowed values are integer values from zero to 100."]
    #[serde(rename = "MaxPercentUnhealthyNodes", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_unhealthy_nodes: Option<i64>,
    #[doc = "The maximum allowed percentage of delta health degradation during the upgrade. Allowed values are integer values from zero to 100."]
    #[serde(rename = "MaxPercentDeltaUnhealthyNodes", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_delta_unhealthy_nodes: Option<i64>,
    #[doc = "The maximum allowed percentage of upgrade domain delta health degradation during the upgrade. Allowed values are integer values from zero to 100."]
    #[serde(
        rename = "MaxPercentUpgradeDomainDeltaUnhealthyNodes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_percent_upgrade_domain_delta_unhealthy_nodes: Option<i64>,
    #[doc = "Defines the application health policy map used to evaluate the health of an application or one of its children entities."]
    #[serde(rename = "ApplicationHealthPolicies", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policies: Option<ApplicationHealthPolicies>,
}
impl ClusterConfigurationUpgradeDescription {
    pub fn new(cluster_config: String) -> Self {
        Self {
            cluster_config,
            health_check_retry_timeout: None,
            health_check_wait_duration_in_seconds: None,
            health_check_stable_duration_in_seconds: None,
            upgrade_domain_timeout_in_seconds: None,
            upgrade_timeout_in_seconds: None,
            max_percent_unhealthy_applications: None,
            max_percent_unhealthy_nodes: None,
            max_percent_delta_unhealthy_nodes: None,
            max_percent_upgrade_domain_delta_unhealthy_nodes: None,
            application_health_policies: None,
        }
    }
}
#[doc = "Information about a standalone cluster configuration upgrade status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterConfigurationUpgradeStatusInfo {
    #[doc = "The state of the upgrade domain."]
    #[serde(rename = "UpgradeState", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_state: Option<UpgradeState>,
    #[doc = "The cluster manifest version."]
    #[serde(rename = "ProgressStatus", default, skip_serializing_if = "Option::is_none")]
    pub progress_status: Option<i64>,
    #[doc = "The cluster configuration version."]
    #[serde(rename = "ConfigVersion", default, skip_serializing_if = "Option::is_none")]
    pub config_version: Option<String>,
    #[doc = "The cluster upgrade status details."]
    #[serde(rename = "Details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl ClusterConfigurationUpgradeStatusInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the base for all Cluster Events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterEvent {
    #[serde(flatten)]
    pub fabric_event: FabricEvent,
}
impl ClusterEvent {
    pub fn new(fabric_event: FabricEvent) -> Self {
        Self { fabric_event }
    }
}
pub type ClusterEventList = Vec<ClusterEvent>;
pub type ClusterFabricCodeVersionString = String;
pub type ClusterFabricConfigVersionString = String;
#[doc = "Represents the health of the cluster.\nContains the cluster aggregated health state, the cluster application and node health states as well as the health events and the unhealthy evaluations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterHealth {
    #[serde(flatten)]
    pub entity_health: EntityHealth,
    #[doc = "Cluster node health states as found in the health store."]
    #[serde(rename = "NodeHealthStates", default, skip_serializing_if = "Vec::is_empty")]
    pub node_health_states: Vec<NodeHealthState>,
    #[doc = "Cluster application health states as found in the health store."]
    #[serde(rename = "ApplicationHealthStates", default, skip_serializing_if = "Vec::is_empty")]
    pub application_health_states: Vec<ApplicationHealthState>,
}
impl ClusterHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the health chunk of the cluster.\nContains the cluster aggregated health state, and the cluster entities that respect the input filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterHealthChunk {
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "HealthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[doc = "The list of node health state chunks in the cluster that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
    #[serde(rename = "NodeHealthStateChunks", default, skip_serializing_if = "Option::is_none")]
    pub node_health_state_chunks: Option<NodeHealthStateChunkList>,
    #[doc = "The list of application health state chunks in the cluster that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
    #[serde(rename = "ApplicationHealthStateChunks", default, skip_serializing_if = "Option::is_none")]
    pub application_health_state_chunks: Option<ApplicationHealthStateChunkList>,
}
impl ClusterHealthChunk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The cluster health chunk query description, which can specify the health policies to evaluate cluster health and very expressive filters to select which cluster entities to include in response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterHealthChunkQueryDescription {
    #[doc = "Defines a list of filters that specify which nodes to be included in the returned cluster health chunk.\nIf no filters are specified, no nodes are returned. All the nodes are used to evaluate the cluster's aggregated health state, regardless of the input filters.\nThe cluster health chunk query may specify multiple node filters.\nFor example, it can specify a filter to return all nodes with health state Error and another filter to always include a node identified by its NodeName."]
    #[serde(rename = "NodeFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub node_filters: Vec<NodeHealthStateFilter>,
    #[doc = "Defines a list of filters that specify which applications to be included in the returned cluster health chunk.\nIf no filters are specified, no applications are returned. All the applications are used to evaluate the cluster's aggregated health state, regardless of the input filters.\nThe cluster health chunk query may specify multiple application filters.\nFor example, it can specify a filter to return all applications with health state Error and another filter to always include applications of a specified application type."]
    #[serde(rename = "ApplicationFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub application_filters: Vec<ApplicationHealthStateFilter>,
    #[doc = "Defines a health policy used to evaluate the health of the cluster or of a cluster node."]
    #[serde(rename = "ClusterHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cluster_health_policy: Option<ClusterHealthPolicy>,
    #[doc = "Defines the application health policy map used to evaluate the health of an application or one of its children entities."]
    #[serde(rename = "ApplicationHealthPolicies", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policies: Option<ApplicationHealthPolicies>,
}
impl ClusterHealthChunkQueryDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Health policies to evaluate cluster health."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterHealthPolicies {
    #[doc = "Defines a map that contains specific application health policies for different applications.\nEach entry specifies as key the application name and as value an ApplicationHealthPolicy used to evaluate the application health.\nIf an application is not specified in the map, the application health evaluation uses the ApplicationHealthPolicy found in its application manifest or the default application health policy (if no health policy is defined in the manifest).\nThe map is empty by default."]
    #[serde(rename = "ApplicationHealthPolicyMap", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy_map: Option<ApplicationHealthPolicyMap>,
    #[doc = "Defines a health policy used to evaluate the health of the cluster or of a cluster node."]
    #[serde(rename = "ClusterHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cluster_health_policy: Option<ClusterHealthPolicy>,
}
impl ClusterHealthPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a health policy used to evaluate the health of the cluster or of a cluster node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterHealthPolicy {
    #[doc = "Indicates whether warnings are treated with the same severity as errors."]
    #[serde(rename = "ConsiderWarningAsError", default, skip_serializing_if = "Option::is_none")]
    pub consider_warning_as_error: Option<bool>,
    #[doc = "The maximum allowed percentage of unhealthy nodes before reporting an error. For example, to allow 10% of nodes to be unhealthy, this value would be 10.\n\nThe percentage represents the maximum tolerated percentage of nodes that can be unhealthy before the cluster is considered in error.\nIf the percentage is respected but there is at least one unhealthy node, the health is evaluated as Warning.\nThe percentage is calculated by dividing the number of unhealthy nodes over the total number of nodes in the cluster.\nThe computation rounds up to tolerate one failure on small numbers of nodes. Default percentage is zero.\n\nIn large clusters, some nodes will always be down or out for repairs, so this percentage should be configured to tolerate that."]
    #[serde(rename = "MaxPercentUnhealthyNodes", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_unhealthy_nodes: Option<i64>,
    #[doc = "The maximum allowed percentage of unhealthy applications before reporting an error. For example, to allow 10% of applications to be unhealthy, this value would be 10.\n\nThe percentage represents the maximum tolerated percentage of applications that can be unhealthy before the cluster is considered in error.\nIf the percentage is respected but there is at least one unhealthy application, the health is evaluated as Warning.\nThis is calculated by dividing the number of unhealthy applications over the total number of application instances in the cluster, excluding applications of application types that are included in the ApplicationTypeHealthPolicyMap.\nThe computation rounds up to tolerate one failure on small numbers of applications. Default percentage is zero."]
    #[serde(rename = "MaxPercentUnhealthyApplications", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_unhealthy_applications: Option<i64>,
    #[doc = "Defines a map with max percentage unhealthy applications for specific application types.\nEach entry specifies as key the application type name and as value an integer that represents the MaxPercentUnhealthyApplications percentage used to evaluate the applications of the specified application type.\n\nThe application type health policy map can be used during cluster health evaluation to describe special application types.\nThe application types included in the map are evaluated against the percentage specified in the map, and not with the global MaxPercentUnhealthyApplications defined in the cluster health policy.\nThe applications of application types specified in the map are not counted against the global pool of applications.\nFor example, if some applications of a type are critical, the cluster administrator can add an entry to the map for that application type\nand assign it a value of 0% (that is, do not tolerate any failures).\nAll other applications can be evaluated with MaxPercentUnhealthyApplications set to 20% to tolerate some failures out of the thousands of application instances.\nThe application type health policy map is used only if the cluster manifest enables application type health evaluation using the configuration entry for HealthManager/EnableApplicationTypeHealthEvaluation."]
    #[serde(rename = "ApplicationTypeHealthPolicyMap", default, skip_serializing_if = "Option::is_none")]
    pub application_type_health_policy_map: Option<ApplicationTypeHealthPolicyMap>,
}
impl ClusterHealthPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cluster Health Report Expired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterHealthReportExpiredEvent {
    #[serde(flatten)]
    pub cluster_event: ClusterEvent,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl ClusterHealthReportExpiredEvent {
    pub fn new(
        cluster_event: ClusterEvent,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            cluster_event,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Information about load in a Service Fabric cluster. It holds a summary of all metrics and their load in a cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterLoadInfo {
    #[doc = "The starting time of last resource balancing run."]
    #[serde(rename = "LastBalancingStartTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_balancing_start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The end time of last resource balancing run."]
    #[serde(rename = "LastBalancingEndTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_balancing_end_time_utc: Option<time::OffsetDateTime>,
    #[doc = "List that contains metrics and their load information in this cluster."]
    #[serde(rename = "LoadMetricInformation", default, skip_serializing_if = "Vec::is_empty")]
    pub load_metric_information: Vec<LoadMetricInformation>,
}
impl ClusterLoadInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the cluster manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterManifest {
    #[doc = "The contents of the cluster manifest file."]
    #[serde(rename = "Manifest", default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
}
impl ClusterManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cluster Health Report Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterNewHealthReportEvent {
    #[serde(flatten)]
    pub cluster_event: ClusterEvent,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl ClusterNewHealthReportEvent {
    pub fn new(
        cluster_event: ClusterEvent,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            cluster_event,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Cluster Upgrade Completed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpgradeCompletedEvent {
    #[serde(flatten)]
    pub cluster_event: ClusterEvent,
    #[doc = "Target Cluster version."]
    #[serde(rename = "TargetClusterVersion")]
    pub target_cluster_version: String,
    #[doc = "Overall duration of upgrade in milli-seconds."]
    #[serde(rename = "OverallUpgradeElapsedTimeInMs")]
    pub overall_upgrade_elapsed_time_in_ms: f64,
}
impl ClusterUpgradeCompletedEvent {
    pub fn new(cluster_event: ClusterEvent, target_cluster_version: String, overall_upgrade_elapsed_time_in_ms: f64) -> Self {
        Self {
            cluster_event,
            target_cluster_version,
            overall_upgrade_elapsed_time_in_ms,
        }
    }
}
#[doc = "Represents a ServiceFabric cluster upgrade"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpgradeDescriptionObject {
    #[doc = "The cluster configuration version (specified in the cluster manifest)."]
    #[serde(rename = "ConfigVersion", default, skip_serializing_if = "Option::is_none")]
    pub config_version: Option<ClusterFabricConfigVersionString>,
    #[doc = "The ServiceFabric code version of the cluster."]
    #[serde(rename = "CodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub code_version: Option<ClusterFabricCodeVersionString>,
    #[doc = "The kind of upgrade out of the following possible values."]
    #[serde(rename = "UpgradeKind", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_kind: Option<UpgradeKind>,
    #[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
    #[serde(rename = "RollingUpgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub rolling_upgrade_mode: Option<UpgradeMode>,
    #[doc = "The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer)."]
    #[serde(
        rename = "UpgradeReplicaSetCheckTimeoutInSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upgrade_replica_set_check_timeout_in_seconds: Option<UpgradeReplicaSetCheckTimeout>,
    #[doc = "If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data)."]
    #[serde(rename = "ForceRestart", default, skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<ForceRestart>,
    #[doc = "Defines the order in which an upgrade proceeds through the cluster."]
    #[serde(rename = "SortOrder", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<UpgradeSortOrder>,
    #[doc = "When true, enables delta health evaluation rather than absolute health evaluation after completion of each upgrade domain."]
    #[serde(rename = "EnableDeltaHealthEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub enable_delta_health_evaluation: Option<DeltaHealthEvaluationBool>,
    #[doc = "Describes the parameters for monitoring an upgrade in Monitored mode."]
    #[serde(rename = "MonitoringPolicy", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_policy: Option<MonitoringPolicyDescription>,
    #[doc = "Defines a health policy used to evaluate the health of the cluster or of a cluster node."]
    #[serde(rename = "ClusterHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cluster_health_policy: Option<ClusterHealthPolicy>,
    #[doc = "Defines a health policy used to evaluate the health of the cluster during a cluster upgrade."]
    #[serde(rename = "ClusterUpgradeHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cluster_upgrade_health_policy: Option<ClusterUpgradeHealthPolicyObject>,
    #[doc = "Represents the map of application health policies for a ServiceFabric cluster upgrade"]
    #[serde(rename = "ApplicationHealthPolicyMap", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy_map: Option<ApplicationHealthPolicyMapObject>,
}
impl ClusterUpgradeDescriptionObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cluster Upgrade Domain Completed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpgradeDomainCompletedEvent {
    #[serde(flatten)]
    pub cluster_event: ClusterEvent,
    #[doc = "Target Cluster version."]
    #[serde(rename = "TargetClusterVersion")]
    pub target_cluster_version: String,
    #[doc = "State of upgrade."]
    #[serde(rename = "UpgradeState")]
    pub upgrade_state: String,
    #[doc = "Upgrade domains."]
    #[serde(rename = "UpgradeDomains")]
    pub upgrade_domains: String,
    #[doc = "Duration of domain upgrade in milli-seconds."]
    #[serde(rename = "UpgradeDomainElapsedTimeInMs")]
    pub upgrade_domain_elapsed_time_in_ms: f64,
}
impl ClusterUpgradeDomainCompletedEvent {
    pub fn new(
        cluster_event: ClusterEvent,
        target_cluster_version: String,
        upgrade_state: String,
        upgrade_domains: String,
        upgrade_domain_elapsed_time_in_ms: f64,
    ) -> Self {
        Self {
            cluster_event,
            target_cluster_version,
            upgrade_state,
            upgrade_domains,
            upgrade_domain_elapsed_time_in_ms,
        }
    }
}
#[doc = "Defines a health policy used to evaluate the health of the cluster during a cluster upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpgradeHealthPolicyObject {
    #[doc = "The maximum allowed percentage of nodes health degradation allowed during cluster upgrades. The delta is measured between the state of the nodes at the beginning of upgrade and the state of the nodes at the time of the health evaluation. The check is performed after every upgrade domain upgrade completion to make sure the global state of the cluster is within tolerated limits. The default value is 10%."]
    #[serde(rename = "MaxPercentDeltaUnhealthyNodes", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_delta_unhealthy_nodes: Option<i64>,
    #[doc = "The maximum allowed percentage of upgrade domain nodes health degradation allowed during cluster upgrades. The delta is measured between the state of the upgrade domain nodes at the beginning of upgrade and the state of the upgrade domain nodes at the time of the health evaluation. The check is performed after every upgrade domain upgrade completion for all completed upgrade domains to make sure the state of the upgrade domains is within tolerated limits. The default value is 15%."]
    #[serde(
        rename = "MaxPercentUpgradeDomainDeltaUnhealthyNodes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_percent_upgrade_domain_delta_unhealthy_nodes: Option<i64>,
}
impl ClusterUpgradeHealthPolicyObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a cluster upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpgradeProgressObject {
    #[doc = "The ServiceFabric code version of the cluster."]
    #[serde(rename = "CodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub code_version: Option<ClusterFabricCodeVersionString>,
    #[doc = "The cluster configuration version (specified in the cluster manifest)."]
    #[serde(rename = "ConfigVersion", default, skip_serializing_if = "Option::is_none")]
    pub config_version: Option<ClusterFabricConfigVersionString>,
    #[doc = "List of upgrade domains and their statuses."]
    #[serde(rename = "UpgradeDomains", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domains: Option<UpgradeDomainInfoList>,
    #[doc = "The state of the upgrade domain."]
    #[serde(rename = "UpgradeState", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_state: Option<UpgradeState>,
    #[doc = "The name of the next upgrade domain to be processed."]
    #[serde(rename = "NextUpgradeDomain", default, skip_serializing_if = "Option::is_none")]
    pub next_upgrade_domain: Option<NextUpgradeDomain>,
    #[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
    #[serde(rename = "RollingUpgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub rolling_upgrade_mode: Option<UpgradeMode>,
    #[doc = "Represents a ServiceFabric cluster upgrade"]
    #[serde(rename = "UpgradeDescription", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_description: Option<ClusterUpgradeDescriptionObject>,
    #[doc = "The estimated elapsed time spent processing the current overall upgrade."]
    #[serde(rename = "UpgradeDurationInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_duration_in_milliseconds: Option<UpgradeDurationString>,
    #[doc = "The estimated elapsed time spent processing the current upgrade domain."]
    #[serde(rename = "UpgradeDomainDurationInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain_duration_in_milliseconds: Option<UpgradeDomainDurationString>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
    #[doc = "Information about the current in-progress upgrade domain."]
    #[serde(rename = "CurrentUpgradeDomainProgress", default, skip_serializing_if = "Option::is_none")]
    pub current_upgrade_domain_progress: Option<CurrentUpgradeDomainProgressInfo>,
    #[doc = "The start time of the upgrade in UTC."]
    #[serde(rename = "StartTimestampUtc", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp_utc: Option<UpgradeStartTimeUtcString>,
    #[doc = "The failure time of the upgrade in UTC."]
    #[serde(rename = "FailureTimestampUtc", default, skip_serializing_if = "Option::is_none")]
    pub failure_timestamp_utc: Option<UpgradeFailureTimeUtcString>,
    #[doc = "The cause of an upgrade failure that resulted in FailureAction being executed."]
    #[serde(rename = "FailureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<FailureReason>,
    #[doc = "The detailed upgrade progress for nodes in the current upgrade domain at the point of failure."]
    #[serde(rename = "UpgradeDomainProgressAtFailure", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain_progress_at_failure: Option<FailedUpgradeDomainProgressObject>,
}
impl ClusterUpgradeProgressObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cluster Upgrade Rollback Completed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpgradeRollbackCompletedEvent {
    #[serde(flatten)]
    pub cluster_event: ClusterEvent,
    #[doc = "Target Cluster version."]
    #[serde(rename = "TargetClusterVersion")]
    pub target_cluster_version: String,
    #[doc = "Describes failure."]
    #[serde(rename = "FailureReason")]
    pub failure_reason: String,
    #[doc = "Overall duration of upgrade in milli-seconds."]
    #[serde(rename = "OverallUpgradeElapsedTimeInMs")]
    pub overall_upgrade_elapsed_time_in_ms: f64,
}
impl ClusterUpgradeRollbackCompletedEvent {
    pub fn new(
        cluster_event: ClusterEvent,
        target_cluster_version: String,
        failure_reason: String,
        overall_upgrade_elapsed_time_in_ms: f64,
    ) -> Self {
        Self {
            cluster_event,
            target_cluster_version,
            failure_reason,
            overall_upgrade_elapsed_time_in_ms,
        }
    }
}
#[doc = "Cluster Upgrade Rollback Started event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpgradeRollbackStartedEvent {
    #[serde(flatten)]
    pub cluster_event: ClusterEvent,
    #[doc = "Target Cluster version."]
    #[serde(rename = "TargetClusterVersion")]
    pub target_cluster_version: String,
    #[doc = "Describes failure."]
    #[serde(rename = "FailureReason")]
    pub failure_reason: String,
    #[doc = "Overall duration of upgrade in milli-seconds."]
    #[serde(rename = "OverallUpgradeElapsedTimeInMs")]
    pub overall_upgrade_elapsed_time_in_ms: f64,
}
impl ClusterUpgradeRollbackStartedEvent {
    pub fn new(
        cluster_event: ClusterEvent,
        target_cluster_version: String,
        failure_reason: String,
        overall_upgrade_elapsed_time_in_ms: f64,
    ) -> Self {
        Self {
            cluster_event,
            target_cluster_version,
            failure_reason,
            overall_upgrade_elapsed_time_in_ms,
        }
    }
}
#[doc = "Cluster Upgrade Started event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpgradeStartedEvent {
    #[serde(flatten)]
    pub cluster_event: ClusterEvent,
    #[doc = "Current Cluster version."]
    #[serde(rename = "CurrentClusterVersion")]
    pub current_cluster_version: String,
    #[doc = "Target Cluster version."]
    #[serde(rename = "TargetClusterVersion")]
    pub target_cluster_version: String,
    #[doc = "Type of upgrade."]
    #[serde(rename = "UpgradeType")]
    pub upgrade_type: String,
    #[doc = "Mode of upgrade."]
    #[serde(rename = "RollingUpgradeMode")]
    pub rolling_upgrade_mode: String,
    #[doc = "Action if failed."]
    #[serde(rename = "FailureAction")]
    pub failure_action: String,
}
impl ClusterUpgradeStartedEvent {
    pub fn new(
        cluster_event: ClusterEvent,
        current_cluster_version: String,
        target_cluster_version: String,
        upgrade_type: String,
        rolling_upgrade_mode: String,
        failure_action: String,
    ) -> Self {
        Self {
            cluster_event,
            current_cluster_version,
            target_cluster_version,
            upgrade_type,
            rolling_upgrade_mode,
            failure_action,
        }
    }
}
#[doc = "The cluster version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterVersion {
    #[doc = "The Service Fabric cluster runtime version."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ClusterVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about setup or main entry point of a code package deployed on a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodePackageEntryPoint {
    #[doc = "The location of entry point executable on the node."]
    #[serde(rename = "EntryPointLocation", default, skip_serializing_if = "Option::is_none")]
    pub entry_point_location: Option<String>,
    #[doc = "The process ID of the entry point."]
    #[serde(rename = "ProcessId", default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[doc = "The user name under which entry point executable is run on the node."]
    #[serde(rename = "RunAsUserName", default, skip_serializing_if = "Option::is_none")]
    pub run_as_user_name: Option<String>,
    #[doc = "Statistics about setup or main entry point  of a code package deployed on a Service Fabric node."]
    #[serde(rename = "CodePackageEntryPointStatistics", default, skip_serializing_if = "Option::is_none")]
    pub code_package_entry_point_statistics: Option<CodePackageEntryPointStatistics>,
    #[doc = "Specifies the status of the code package entry point deployed on a Service Fabric node."]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EntryPointStatus>,
    #[doc = "The time (in UTC) when the entry point executable will be run next."]
    #[serde(rename = "NextActivationTime", with = "azure_core::date::rfc3339::option")]
    pub next_activation_time: Option<time::OffsetDateTime>,
    #[doc = "The instance ID for current running entry point. For a code package setup entry point (if specified) runs first and after it finishes main entry point is started. Each time entry point executable is run, its instance id will change."]
    #[serde(rename = "InstanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<CodePackageInstanceId>,
}
impl CodePackageEntryPoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Statistics about setup or main entry point  of a code package deployed on a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodePackageEntryPointStatistics {
    #[doc = "The last exit code of the entry point."]
    #[serde(rename = "LastExitCode", default, skip_serializing_if = "Option::is_none")]
    pub last_exit_code: Option<String>,
    #[doc = "The last time (in UTC) when Service Fabric attempted to run the entry point."]
    #[serde(rename = "LastActivationTime", with = "azure_core::date::rfc3339::option")]
    pub last_activation_time: Option<time::OffsetDateTime>,
    #[doc = "The last time (in UTC) when the entry point finished running."]
    #[serde(rename = "LastExitTime", with = "azure_core::date::rfc3339::option")]
    pub last_exit_time: Option<time::OffsetDateTime>,
    #[doc = "The last time (in UTC) when the entry point ran successfully."]
    #[serde(rename = "LastSuccessfulActivationTime", with = "azure_core::date::rfc3339::option")]
    pub last_successful_activation_time: Option<time::OffsetDateTime>,
    #[doc = "The last time (in UTC) when the entry point finished running gracefully."]
    #[serde(rename = "LastSuccessfulExitTime", with = "azure_core::date::rfc3339::option")]
    pub last_successful_exit_time: Option<time::OffsetDateTime>,
    #[doc = "Number of times the entry point has run."]
    #[serde(rename = "ActivationCount", default, skip_serializing_if = "Option::is_none")]
    pub activation_count: Option<String>,
    #[doc = "Number of times the entry point failed to run."]
    #[serde(rename = "ActivationFailureCount", default, skip_serializing_if = "Option::is_none")]
    pub activation_failure_count: Option<String>,
    #[doc = "Number of times the entry point continuously failed to run."]
    #[serde(rename = "ContinuousActivationFailureCount", default, skip_serializing_if = "Option::is_none")]
    pub continuous_activation_failure_count: Option<String>,
    #[doc = "Number of times the entry point finished running."]
    #[serde(rename = "ExitCount", default, skip_serializing_if = "Option::is_none")]
    pub exit_count: Option<String>,
    #[doc = "Number of times the entry point failed to exit gracefully."]
    #[serde(rename = "ExitFailureCount", default, skip_serializing_if = "Option::is_none")]
    pub exit_failure_count: Option<String>,
    #[doc = "Number of times the entry point continuously failed to exit gracefully."]
    #[serde(rename = "ContinuousExitFailureCount", default, skip_serializing_if = "Option::is_none")]
    pub continuous_exit_failure_count: Option<String>,
}
impl CodePackageEntryPointStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type CodePackageInstanceId = String;
pub type CodePackageName = String;
#[doc = "The status of the compose deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ComposeDeploymentStatus")]
pub enum ComposeDeploymentStatus {
    Invalid,
    Provisioning,
    Creating,
    Ready,
    Unprovisioning,
    Deleting,
    Failed,
    Upgrading,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ComposeDeploymentStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ComposeDeploymentStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ComposeDeploymentStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ComposeDeploymentStatus", 0u32, "Invalid"),
            Self::Provisioning => serializer.serialize_unit_variant("ComposeDeploymentStatus", 1u32, "Provisioning"),
            Self::Creating => serializer.serialize_unit_variant("ComposeDeploymentStatus", 2u32, "Creating"),
            Self::Ready => serializer.serialize_unit_variant("ComposeDeploymentStatus", 3u32, "Ready"),
            Self::Unprovisioning => serializer.serialize_unit_variant("ComposeDeploymentStatus", 4u32, "Unprovisioning"),
            Self::Deleting => serializer.serialize_unit_variant("ComposeDeploymentStatus", 5u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("ComposeDeploymentStatus", 6u32, "Failed"),
            Self::Upgrading => serializer.serialize_unit_variant("ComposeDeploymentStatus", 7u32, "Upgrading"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information about a Service Fabric compose deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComposeDeploymentStatusInfo {
    #[doc = "The name of the deployment."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<DeploymentName>,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
    #[doc = "The status of the compose deployment."]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ComposeDeploymentStatus>,
    #[doc = "The status details of compose deployment including failure message."]
    #[serde(rename = "StatusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}
impl ComposeDeploymentStatusInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the parameters for a compose deployment upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposeDeploymentUpgradeDescription {
    #[doc = "The name of the deployment."]
    #[serde(rename = "DeploymentName")]
    pub deployment_name: DeploymentName,
    #[doc = "The content of the compose file that describes the deployment to create."]
    #[serde(rename = "ComposeFileContent")]
    pub compose_file_content: String,
    #[doc = "Credential information to connect to container registry."]
    #[serde(rename = "RegistryCredential", default, skip_serializing_if = "Option::is_none")]
    pub registry_credential: Option<RegistryCredential>,
    #[doc = "The kind of upgrade out of the following possible values."]
    #[serde(rename = "UpgradeKind")]
    pub upgrade_kind: UpgradeKind,
    #[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
    #[serde(rename = "RollingUpgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub rolling_upgrade_mode: Option<UpgradeMode>,
    #[doc = "The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer)."]
    #[serde(
        rename = "UpgradeReplicaSetCheckTimeoutInSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upgrade_replica_set_check_timeout_in_seconds: Option<UpgradeReplicaSetCheckTimeout>,
    #[doc = "If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data)."]
    #[serde(rename = "ForceRestart", default, skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<ForceRestart>,
    #[doc = "Describes the parameters for monitoring an upgrade in Monitored mode."]
    #[serde(rename = "MonitoringPolicy", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_policy: Option<MonitoringPolicyDescription>,
    #[doc = "Defines a health policy used to evaluate the health of an application or one of its children entities."]
    #[serde(rename = "ApplicationHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy: Option<ApplicationHealthPolicy>,
}
impl ComposeDeploymentUpgradeDescription {
    pub fn new(deployment_name: DeploymentName, compose_file_content: String, upgrade_kind: UpgradeKind) -> Self {
        Self {
            deployment_name,
            compose_file_content,
            registry_credential: None,
            upgrade_kind,
            rolling_upgrade_mode: None,
            upgrade_replica_set_check_timeout_in_seconds: None,
            force_restart: None,
            monitoring_policy: None,
            application_health_policy: None,
        }
    }
}
#[doc = "Describes the parameters for a compose deployment upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComposeDeploymentUpgradeProgressInfo {
    #[doc = "The name of the target deployment."]
    #[serde(rename = "DeploymentName", default, skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<TargetDeploymentName>,
    #[doc = "The name of the target application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<TargetApplicationName>,
    #[doc = "The state of the compose deployment upgrade."]
    #[serde(rename = "UpgradeState", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_state: Option<ComposeDeploymentUpgradeState>,
    #[doc = "Additional detailed information about the status of the pending upgrade."]
    #[serde(rename = "UpgradeStatusDetails", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_status_details: Option<String>,
    #[doc = "The kind of upgrade out of the following possible values."]
    #[serde(rename = "UpgradeKind", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_kind: Option<UpgradeKind>,
    #[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
    #[serde(rename = "RollingUpgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub rolling_upgrade_mode: Option<UpgradeMode>,
    #[doc = "If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data)."]
    #[serde(rename = "ForceRestart", default, skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<ForceRestart>,
    #[doc = "The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer)."]
    #[serde(
        rename = "UpgradeReplicaSetCheckTimeoutInSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upgrade_replica_set_check_timeout_in_seconds: Option<UpgradeReplicaSetCheckTimeout>,
    #[doc = "Describes the parameters for monitoring an upgrade in Monitored mode."]
    #[serde(rename = "MonitoringPolicy", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_policy: Option<MonitoringPolicyDescription>,
    #[doc = "Defines a health policy used to evaluate the health of an application or one of its children entities."]
    #[serde(rename = "ApplicationHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy: Option<ApplicationHealthPolicy>,
    #[doc = "The target application type version (found in the application manifest) for the application upgrade."]
    #[serde(rename = "TargetApplicationTypeVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_application_type_version: Option<TargetApplicationTypeVersion>,
    #[doc = "The estimated amount of time that the overall upgrade elapsed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "UpgradeDuration", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_duration: Option<UpgradeDuration>,
    #[doc = "The estimated amount of time spent processing current Upgrade Domain. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "CurrentUpgradeDomainDuration", default, skip_serializing_if = "Option::is_none")]
    pub current_upgrade_domain_duration: Option<CurrentUpgradeDomainDuration>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "ApplicationUnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub application_unhealthy_evaluations: Option<ApplicationUnhealthyEvaluations>,
    #[doc = "Information about the current in-progress upgrade domain."]
    #[serde(rename = "CurrentUpgradeDomainProgress", default, skip_serializing_if = "Option::is_none")]
    pub current_upgrade_domain_progress: Option<CurrentUpgradeDomainProgressInfo>,
    #[doc = "The estimated UTC datetime when the upgrade started."]
    #[serde(rename = "StartTimestampUtc", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp_utc: Option<String>,
    #[doc = "The estimated UTC datetime when the upgrade failed and FailureAction was executed."]
    #[serde(rename = "FailureTimestampUtc", default, skip_serializing_if = "Option::is_none")]
    pub failure_timestamp_utc: Option<String>,
    #[doc = "The cause of an upgrade failure that resulted in FailureAction being executed."]
    #[serde(rename = "FailureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<FailureReason>,
    #[doc = "Information about the upgrade domain progress at the time of upgrade failure."]
    #[serde(rename = "UpgradeDomainProgressAtFailure", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain_progress_at_failure: Option<FailureUpgradeDomainProgressInfo>,
    #[doc = "Additional details of application upgrade including failure message."]
    #[serde(rename = "ApplicationUpgradeStatusDetails", default, skip_serializing_if = "Option::is_none")]
    pub application_upgrade_status_details: Option<String>,
}
impl ComposeDeploymentUpgradeProgressInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of the compose deployment upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ComposeDeploymentUpgradeState")]
pub enum ComposeDeploymentUpgradeState {
    Invalid,
    ProvisioningTarget,
    RollingForwardInProgress,
    RollingForwardPending,
    UnprovisioningCurrent,
    RollingForwardCompleted,
    RollingBackInProgress,
    UnprovisioningTarget,
    RollingBackCompleted,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ComposeDeploymentUpgradeState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ComposeDeploymentUpgradeState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ComposeDeploymentUpgradeState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ComposeDeploymentUpgradeState", 0u32, "Invalid"),
            Self::ProvisioningTarget => serializer.serialize_unit_variant("ComposeDeploymentUpgradeState", 1u32, "ProvisioningTarget"),
            Self::RollingForwardInProgress => {
                serializer.serialize_unit_variant("ComposeDeploymentUpgradeState", 2u32, "RollingForwardInProgress")
            }
            Self::RollingForwardPending => {
                serializer.serialize_unit_variant("ComposeDeploymentUpgradeState", 3u32, "RollingForwardPending")
            }
            Self::UnprovisioningCurrent => {
                serializer.serialize_unit_variant("ComposeDeploymentUpgradeState", 4u32, "UnprovisioningCurrent")
            }
            Self::RollingForwardCompleted => {
                serializer.serialize_unit_variant("ComposeDeploymentUpgradeState", 5u32, "RollingForwardCompleted")
            }
            Self::RollingBackInProgress => {
                serializer.serialize_unit_variant("ComposeDeploymentUpgradeState", 6u32, "RollingBackInProgress")
            }
            Self::UnprovisioningTarget => serializer.serialize_unit_variant("ComposeDeploymentUpgradeState", 7u32, "UnprovisioningTarget"),
            Self::RollingBackCompleted => serializer.serialize_unit_variant("ComposeDeploymentUpgradeState", 8u32, "RollingBackCompleted"),
            Self::Failed => serializer.serialize_unit_variant("ComposeDeploymentUpgradeState", 9u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information about a configuration parameter override."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigParameterOverride {
    #[doc = "Name of the section for the parameter override."]
    #[serde(rename = "SectionName")]
    pub section_name: String,
    #[doc = "Name of the parameter that has been overridden."]
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,
    #[doc = "Value of the overridden parameter."]
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,
    #[doc = "The duration until config override is considered as valid."]
    #[serde(rename = "Timeout", default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "A value that indicates whether config override will be removed on upgrade or will still be considered as valid."]
    #[serde(rename = "PersistAcrossUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub persist_across_upgrade: Option<bool>,
}
impl ConfigParameterOverride {
    pub fn new(section_name: String, parameter_name: String, parameter_value: String) -> Self {
        Self {
            section_name,
            parameter_name,
            parameter_value,
            timeout: None,
            persist_across_upgrade: None,
        }
    }
}
pub type ConfigParameterOverrideList = Vec<ConfigParameterOverride>;
#[doc = "parameters for making container API call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerApiRequestBody {
    #[doc = "HTTP verb of container REST API, defaults to \"GET\""]
    #[serde(rename = "HttpVerb", default, skip_serializing_if = "Option::is_none")]
    pub http_verb: Option<String>,
    #[doc = "URI path of container REST API"]
    #[serde(rename = "UriPath")]
    pub uri_path: String,
    #[doc = "Content type of container REST API request, defaults to \"application/json\""]
    #[serde(rename = "Content-Type", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "HTTP request body of container REST API"]
    #[serde(rename = "Body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
impl ContainerApiRequestBody {
    pub fn new(uri_path: String) -> Self {
        Self {
            http_verb: None,
            uri_path,
            content_type: None,
            body: None,
        }
    }
}
#[doc = "Response body that wraps container API result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerApiResponse {
    #[doc = "Container API result."]
    #[serde(rename = "ContainerApiResult")]
    pub container_api_result: ContainerApiResult,
}
impl ContainerApiResponse {
    pub fn new(container_api_result: ContainerApiResult) -> Self {
        Self { container_api_result }
    }
}
#[doc = "Container API result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerApiResult {
    #[doc = "HTTP status code returned by the target container API"]
    #[serde(rename = "Status")]
    pub status: i64,
    #[doc = "HTTP content type"]
    #[serde(rename = "Content-Type", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "HTTP content encoding"]
    #[serde(rename = "Content-Encoding", default, skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
    #[doc = "container API result body"]
    #[serde(rename = "Body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
impl ContainerApiResult {
    pub fn new(status: i64) -> Self {
        Self {
            status,
            content_type: None,
            content_encoding: None,
            body: None,
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
    #[serde(rename = "entryPoint", default, skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<String>,
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
    #[doc = "An array of liveness probes for a code package. It determines when to restart a code package."]
    #[serde(rename = "livenessProbe", default, skip_serializing_if = "Vec::is_empty")]
    pub liveness_probe: Vec<Probe>,
    #[doc = "An array of readiness probes for a code package. It determines when to unpublish an endpoint."]
    #[serde(rename = "readinessProbe", default, skip_serializing_if = "Vec::is_empty")]
    pub readiness_probe: Vec<Probe>,
}
impl ContainerCodePackageProperties {
    pub fn new(name: String, image: String, resources: ResourceRequirements) -> Self {
        Self {
            name,
            image,
            image_registry_credential: None,
            entry_point: None,
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
            liveness_probe: Vec::new(),
            readiness_probe: Vec::new(),
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
#[doc = "Represents the base for all Container Events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerInstanceEvent {
    #[serde(flatten)]
    pub fabric_event: FabricEvent,
}
impl ContainerInstanceEvent {
    pub fn new(fabric_event: FabricEvent) -> Self {
        Self { fabric_event }
    }
}
pub type ContainerInstanceEventList = Vec<ContainerInstanceEvent>;
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
    #[serde(rename = "Content", default, skip_serializing_if = "Option::is_none")]
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
pub type ContinuationToken = String;
pub type CorrelationSchemeList = Vec<ServiceCorrelationDescription>;
#[doc = "Defines description for creating a Service Fabric compose deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateComposeDeploymentDescription {
    #[doc = "The name of the deployment."]
    #[serde(rename = "DeploymentName")]
    pub deployment_name: DeploymentName,
    #[doc = "The content of the compose file that describes the deployment to create."]
    #[serde(rename = "ComposeFileContent")]
    pub compose_file_content: String,
    #[doc = "Credential information to connect to container registry."]
    #[serde(rename = "RegistryCredential", default, skip_serializing_if = "Option::is_none")]
    pub registry_credential: Option<RegistryCredential>,
}
impl CreateComposeDeploymentDescription {
    pub fn new(deployment_name: DeploymentName, compose_file_content: String) -> Self {
        Self {
            deployment_name,
            compose_file_content,
            registry_credential: None,
        }
    }
}
pub type CurrentUpgradeDomainDuration = String;
#[doc = "Information about the current in-progress upgrade domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurrentUpgradeDomainProgressInfo {
    #[doc = "The name of the upgrade domain"]
    #[serde(rename = "DomainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<UpgradeDomainName>,
    #[doc = "List of upgrading nodes and their statuses"]
    #[serde(rename = "NodeUpgradeProgressList", default, skip_serializing_if = "Option::is_none")]
    pub node_upgrade_progress_list: Option<NodeUpgradeProgressInfoList>,
}
impl CurrentUpgradeDomainProgressInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the days in a week."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DayOfWeek")]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DayOfWeek {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DayOfWeek {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DayOfWeek {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sunday => serializer.serialize_unit_variant("DayOfWeek", 0u32, "Sunday"),
            Self::Monday => serializer.serialize_unit_variant("DayOfWeek", 1u32, "Monday"),
            Self::Tuesday => serializer.serialize_unit_variant("DayOfWeek", 2u32, "Tuesday"),
            Self::Wednesday => serializer.serialize_unit_variant("DayOfWeek", 3u32, "Wednesday"),
            Self::Thursday => serializer.serialize_unit_variant("DayOfWeek", 4u32, "Thursday"),
            Self::Friday => serializer.serialize_unit_variant("DayOfWeek", 5u32, "Friday"),
            Self::Saturday => serializer.serialize_unit_variant("DayOfWeek", 6u32, "Saturday"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type DayOfWeekList = Vec<DayOfWeek>;
#[doc = "Describes the intent or reason for deactivating the node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeactivationIntentDescription {
    #[doc = "Describes the intent or reason for deactivating the node. The possible values are following."]
    #[serde(rename = "DeactivationIntent", default, skip_serializing_if = "Option::is_none")]
    pub deactivation_intent: Option<deactivation_intent_description::DeactivationIntent>,
}
impl DeactivationIntentDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deactivation_intent_description {
    use super::*;
    #[doc = "Describes the intent or reason for deactivating the node. The possible values are following."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeactivationIntent")]
    pub enum DeactivationIntent {
        Pause,
        Restart,
        RemoveData,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeactivationIntent {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeactivationIntent {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeactivationIntent {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pause => serializer.serialize_unit_variant("DeactivationIntent", 0u32, "Pause"),
                Self::Restart => serializer.serialize_unit_variant("DeactivationIntent", 1u32, "Restart"),
                Self::RemoveData => serializer.serialize_unit_variant("DeactivationIntent", 2u32, "RemoveData"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The default execution policy. Always restart the service if an exit occurs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultExecutionPolicy {
    #[serde(flatten)]
    pub execution_policy: ExecutionPolicy,
}
impl DefaultExecutionPolicy {
    pub fn new(execution_policy: ExecutionPolicy) -> Self {
        Self { execution_policy }
    }
}
#[doc = "Represents a PropertyBatchOperation that deletes a specified property if it exists.\nNote that if one PropertyBatchOperation in a PropertyBatch fails,\nthe entire batch fails and cannot be committed in a transactional manner."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletePropertyBatchOperation {
    #[serde(flatten)]
    pub property_batch_operation: PropertyBatchOperation,
}
impl DeletePropertyBatchOperation {
    pub fn new(property_batch_operation: PropertyBatchOperation) -> Self {
        Self { property_batch_operation }
    }
}
pub type DeltaHealthEvaluationBool = bool;
#[doc = "Represents health evaluation for delta nodes, containing health evaluations for each unhealthy node that impacted current aggregated health state.\nCan be returned during cluster upgrade when the aggregated health state of the cluster is Warning or Error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeltaNodesCheckHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Number of nodes with aggregated heath state Error in the health store at the beginning of the cluster upgrade."]
    #[serde(rename = "BaselineErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub baseline_error_count: Option<i64>,
    #[doc = "Total number of nodes in the health store at the beginning of the cluster upgrade."]
    #[serde(rename = "BaselineTotalCount", default, skip_serializing_if = "Option::is_none")]
    pub baseline_total_count: Option<i64>,
    #[doc = "Maximum allowed percentage of delta unhealthy nodes from the ClusterUpgradeHealthPolicy."]
    #[serde(rename = "MaxPercentDeltaUnhealthyNodes", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_delta_unhealthy_nodes: Option<i64>,
    #[doc = "Total number of nodes in the health store."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl DeltaNodesCheckHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            baseline_error_count: None,
            baseline_total_count: None,
            max_percent_delta_unhealthy_nodes: None,
            total_count: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Defines description for downloading packages associated with a service manifest to image cache on a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployServicePackageToNodeDescription {
    #[doc = "The name of the service manifest."]
    #[serde(rename = "ServiceManifestName")]
    pub service_manifest_name: ServiceManifestName,
    #[doc = "The application type name as defined in the application manifest."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: ApplicationTypeName,
    #[doc = "The version of the application type as defined in the application manifest."]
    #[serde(rename = "ApplicationTypeVersion")]
    pub application_type_version: ApplicationTypeVersion,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName")]
    pub node_name: NodeName,
    #[doc = "List of package sharing policy information."]
    #[serde(rename = "PackageSharingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub package_sharing_policy: Option<PackageSharingPolicyInfoList>,
}
impl DeployServicePackageToNodeDescription {
    pub fn new(
        service_manifest_name: ServiceManifestName,
        application_type_name: ApplicationTypeName,
        application_type_version: ApplicationTypeVersion,
        node_name: NodeName,
    ) -> Self {
        Self {
            service_manifest_name,
            application_type_name,
            application_type_version,
            node_name,
            package_sharing_policy: None,
        }
    }
}
#[doc = "Information about the health of an application deployed on a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedApplicationHealth {
    #[serde(flatten)]
    pub entity_health: EntityHealth,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ApplicationName>,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
    #[doc = "List of health states for a service package deployed on a Service Fabric node."]
    #[serde(rename = "DeployedServicePackageHealthStates", default, skip_serializing_if = "Option::is_none")]
    pub deployed_service_package_health_states: Option<DeployedServicePackageHealthStateList>,
}
impl DeployedApplicationHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents health evaluation for a deployed application, containing information about the data and the algorithm used by the health store to evaluate health."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedApplicationHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl DeployedApplicationHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            node_name: None,
            application_name: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Deployed Application Health Report Expired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedApplicationHealthReportExpiredEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Id of Application instance."]
    #[serde(rename = "ApplicationInstanceId")]
    pub application_instance_id: i64,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName")]
    pub node_name: NodeName,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl DeployedApplicationHealthReportExpiredEvent {
    pub fn new(
        application_event: ApplicationEvent,
        application_instance_id: i64,
        node_name: NodeName,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            application_event,
            application_instance_id,
            node_name,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Represents the health state of a deployed application, which contains the entity identifier and the aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedApplicationHealthState {
    #[serde(flatten)]
    pub entity_health_state: EntityHealthState,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
}
impl DeployedApplicationHealthState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the health state chunk of a deployed application, which contains the node where the application is deployed, the aggregated health state and any deployed service packages that respect the chunk query description filters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedApplicationHealthStateChunk {
    #[serde(flatten)]
    pub entity_health_state_chunk: EntityHealthStateChunk,
    #[doc = "The name of node where the application is deployed."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "The list of deployed service package health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
    #[serde(
        rename = "DeployedServicePackageHealthStateChunks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployed_service_package_health_state_chunks: Option<DeployedServicePackageHealthStateChunkList>,
}
impl DeployedApplicationHealthStateChunk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of deployed application health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedApplicationHealthStateChunkList {
    #[doc = "The list of deployed application health state chunks that respect the input filters in the chunk query."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DeployedApplicationHealthStateChunk>,
}
impl DeployedApplicationHealthStateChunkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines matching criteria to determine whether a deployed application should be included as a child of an application in the cluster health chunk.\nThe deployed applications are only returned if the parent application matches a filter specified in the cluster health chunk query description.\nOne filter can match zero, one or multiple deployed applications, depending on its properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedApplicationHealthStateFilter {
    #[doc = "The name of the node where the application is deployed in order to match the filter.\nIf specified, the filter is applied only to the application deployed on the specified node.\nIf the application is not deployed on the node with the specified name, no deployed application is returned in the cluster health chunk based on this filter.\nOtherwise, the deployed application is included in the cluster health chunk if it respects the other filter properties.\nIf not specified, all deployed applications that match the parent filters (if any) are taken into consideration and matched against the other filter members, like health state filter."]
    #[serde(rename = "NodeNameFilter", default, skip_serializing_if = "Option::is_none")]
    pub node_name_filter: Option<String>,
    #[doc = "The filter for the health state of the deployed applications. It allows selecting deployed applications if they match the desired health states.\nThe possible values are integer value of one of the following health states. Only deployed applications that match the filter are returned. All deployed applications are used to evaluate the cluster aggregated health state.\nIf not specified, default value is None, unless the node name is specified. If the filter has default value and node name is specified, the matching deployed application is returned.\nThe state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise 'OR' operator.\nFor example, if the provided value is 6, it matches deployed applications with HealthState value of OK (2) and Warning (4).\n\n- Default - Default value. Matches any HealthState. The value is zero.\n- None - Filter that doesn't match any HealthState value. Used in order to return no results on a given collection of states. The value is 1.\n- Ok - Filter that matches input with HealthState value Ok. The value is 2.\n- Warning - Filter that matches input with HealthState value Warning. The value is 4.\n- Error - Filter that matches input with HealthState value Error. The value is 8.\n- All - Filter that matches input with any HealthState value. The value is 65535."]
    #[serde(rename = "HealthStateFilter", default, skip_serializing_if = "Option::is_none")]
    pub health_state_filter: Option<i64>,
    #[doc = "Defines a list of filters that specify which deployed service packages to be included in the returned cluster health chunk as children of the parent deployed application. The deployed service packages are returned only if the parent deployed application matches a filter.\nIf the list is empty, no deployed service packages are returned. All the deployed service packages are used to evaluate the parent deployed application aggregated health state, regardless of the input filters.\nThe deployed application filter may specify multiple deployed service package filters.\nFor example, it can specify a filter to return all deployed service packages with health state Error and another filter to always include a deployed service package on a node."]
    #[serde(rename = "DeployedServicePackageFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub deployed_service_package_filters: Vec<DeployedServicePackageHealthStateFilter>,
}
impl DeployedApplicationHealthStateFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about application deployed on the node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedApplicationInfo {
    #[doc = "The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource.\nStarting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the application name is \"fabric:/myapp/app1\",\nthe application identity would be \"myapp\\~app1\" in 6.0+ and \"myapp/app1\" in previous versions."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ApplicationId>,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ApplicationName>,
    #[doc = "The application type name as defined in the application manifest."]
    #[serde(rename = "TypeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<ApplicationTypeName>,
    #[doc = "The status of the application deployed on the node. Following are the possible values."]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DeployedApplicationStatus>,
    #[doc = "The work directory of the application on the node. The work directory can be used to store application data."]
    #[serde(rename = "WorkDirectory", default, skip_serializing_if = "Option::is_none")]
    pub work_directory: Option<String>,
    #[doc = "The log directory of the application on the node. The log directory can be used to store application logs."]
    #[serde(rename = "LogDirectory", default, skip_serializing_if = "Option::is_none")]
    pub log_directory: Option<String>,
    #[doc = "The temp directory of the application on the node. The code packages belonging to the application are forked with this directory set as their temporary directory."]
    #[serde(rename = "TempDirectory", default, skip_serializing_if = "Option::is_none")]
    pub temp_directory: Option<String>,
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "HealthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
}
impl DeployedApplicationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type DeployedApplicationInfoList = Vec<DeployedApplicationInfo>;
#[doc = "Deployed Application Health Report Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedApplicationNewHealthReportEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Id of Application instance."]
    #[serde(rename = "ApplicationInstanceId")]
    pub application_instance_id: i64,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName")]
    pub node_name: NodeName,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl DeployedApplicationNewHealthReportEvent {
    pub fn new(
        application_event: ApplicationEvent,
        application_instance_id: i64,
        node_name: NodeName,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            application_event,
            application_instance_id,
            node_name,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "The status of the application deployed on the node. Following are the possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeployedApplicationStatus")]
pub enum DeployedApplicationStatus {
    Invalid,
    Downloading,
    Activating,
    Active,
    Upgrading,
    Deactivating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeployedApplicationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeployedApplicationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeployedApplicationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("DeployedApplicationStatus", 0u32, "Invalid"),
            Self::Downloading => serializer.serialize_unit_variant("DeployedApplicationStatus", 1u32, "Downloading"),
            Self::Activating => serializer.serialize_unit_variant("DeployedApplicationStatus", 2u32, "Activating"),
            Self::Active => serializer.serialize_unit_variant("DeployedApplicationStatus", 3u32, "Active"),
            Self::Upgrading => serializer.serialize_unit_variant("DeployedApplicationStatus", 4u32, "Upgrading"),
            Self::Deactivating => serializer.serialize_unit_variant("DeployedApplicationStatus", 5u32, "Deactivating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents health evaluation for deployed applications, containing health evaluations for each unhealthy deployed application that impacted current aggregated health state.\nCan be returned when evaluating application health and the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedApplicationsHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Maximum allowed percentage of unhealthy deployed applications from the ApplicationHealthPolicy."]
    #[serde(
        rename = "MaxPercentUnhealthyDeployedApplications",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_percent_unhealthy_deployed_applications: Option<i64>,
    #[doc = "Total number of deployed applications of the application in the health store."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl DeployedApplicationsHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            max_percent_unhealthy_deployed_applications: None,
            total_count: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Information about code package deployed on a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedCodePackageInfo {
    #[doc = "The name of the code package defined in the service manifest."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CodePackageName>,
    #[doc = "The version of the code package specified in service manifest."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The name of the service manifest."]
    #[serde(rename = "ServiceManifestName", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_name: Option<ServiceManifestName>,
    #[doc = "The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service\nis 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId\nis always an empty string."]
    #[serde(rename = "ServicePackageActivationId", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_id: Option<ServicePackageActivationId>,
    #[doc = "Specifies the type of host for main entry point of a code package as specified in service manifest."]
    #[serde(rename = "HostType", default, skip_serializing_if = "Option::is_none")]
    pub host_type: Option<HostType>,
    #[doc = "Specifies the isolation mode of main entry point of a code package when it's host type is ContainerHost. This is specified as part of container host policies in application manifest while importing service manifest."]
    #[serde(rename = "HostIsolationMode", default, skip_serializing_if = "Option::is_none")]
    pub host_isolation_mode: Option<HostIsolationMode>,
    #[doc = "Specifies the status of a deployed application or service package on a Service Fabric node."]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DeploymentStatus>,
    #[doc = "The interval at which code package is run. This is used for periodic code package."]
    #[serde(rename = "RunFrequencyInterval", default, skip_serializing_if = "Option::is_none")]
    pub run_frequency_interval: Option<String>,
    #[doc = "Information about setup or main entry point of a code package deployed on a Service Fabric node."]
    #[serde(rename = "SetupEntryPoint", default, skip_serializing_if = "Option::is_none")]
    pub setup_entry_point: Option<CodePackageEntryPoint>,
    #[doc = "Information about setup or main entry point of a code package deployed on a Service Fabric node."]
    #[serde(rename = "MainEntryPoint", default, skip_serializing_if = "Option::is_none")]
    pub main_entry_point: Option<CodePackageEntryPoint>,
}
impl DeployedCodePackageInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type DeployedCodePackageInfoList = Vec<DeployedCodePackageInfo>;
#[doc = "Information about the health of a service package for a specific application deployed on a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedServicePackageHealth {
    #[serde(flatten)]
    pub entity_health: EntityHealth,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
    #[doc = "The name of the service manifest."]
    #[serde(rename = "ServiceManifestName", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_name: Option<ServiceManifestName>,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
}
impl DeployedServicePackageHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents health evaluation for a deployed service package, containing information about the data and the algorithm used by health store to evaluate health. The evaluation is returned only when the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedServicePackageHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
    #[doc = "The name of the service manifest."]
    #[serde(rename = "ServiceManifestName", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_name: Option<ServiceManifestName>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl DeployedServicePackageHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            node_name: None,
            application_name: None,
            service_manifest_name: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Deployed Service Health Report Expired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedServicePackageHealthReportExpiredEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Service manifest name."]
    #[serde(rename = "ServiceManifest")]
    pub service_manifest: String,
    #[doc = "Id of Service package instance."]
    #[serde(rename = "ServicePackageInstanceId")]
    pub service_package_instance_id: i64,
    #[doc = "Id of Service package activation."]
    #[serde(rename = "ServicePackageActivationId")]
    pub service_package_activation_id: String,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName")]
    pub node_name: NodeName,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl DeployedServicePackageHealthReportExpiredEvent {
    pub fn new(
        application_event: ApplicationEvent,
        service_manifest: String,
        service_package_instance_id: i64,
        service_package_activation_id: String,
        node_name: NodeName,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            application_event,
            service_manifest,
            service_package_instance_id,
            service_package_activation_id,
            node_name,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Represents the health state of a deployed service package, containing the entity identifier and the aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedServicePackageHealthState {
    #[serde(flatten)]
    pub entity_health_state: EntityHealthState,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
    #[doc = "The name of the service manifest."]
    #[serde(rename = "ServiceManifestName", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_name: Option<ServiceManifestName>,
    #[doc = "The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service\nis 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId\nis always an empty string."]
    #[serde(rename = "ServicePackageActivationId", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_id: Option<ServicePackageActivationId>,
}
impl DeployedServicePackageHealthState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the health state chunk of a deployed service package, which contains the service manifest name and the service package aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedServicePackageHealthStateChunk {
    #[serde(flatten)]
    pub entity_health_state_chunk: EntityHealthStateChunk,
    #[doc = "The name of the service manifest."]
    #[serde(rename = "ServiceManifestName", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_name: Option<ServiceManifestName>,
    #[doc = "The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service\nis 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId\nis always an empty string."]
    #[serde(rename = "ServicePackageActivationId", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_id: Option<ServicePackageActivationId>,
}
impl DeployedServicePackageHealthStateChunk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of deployed service package health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedServicePackageHealthStateChunkList {
    #[doc = "The list of deployed service package health state chunks that respect the input filters in the chunk query."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DeployedServicePackageHealthStateChunk>,
}
impl DeployedServicePackageHealthStateChunkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines matching criteria to determine whether a deployed service package should be included as a child of a deployed application in the cluster health chunk.\nThe deployed service packages are only returned if the parent entities match a filter specified in the cluster health chunk query description. The parent deployed application and its parent application must be included in the cluster health chunk.\nOne filter can match zero, one or multiple deployed service packages, depending on its properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedServicePackageHealthStateFilter {
    #[doc = "The name of the service manifest which identifies the deployed service packages that matches the filter.\nIf specified, the filter is applied only to the specified deployed service packages, if any.\nIf no deployed service packages with specified manifest name exist, nothing is returned in the cluster health chunk based on this filter.\nIf any deployed service package exists, they are included in the cluster health chunk if it respects the other filter properties.\nIf not specified, all deployed service packages that match the parent filters (if any) are taken into consideration and matched against the other filter members, like health state filter."]
    #[serde(rename = "ServiceManifestNameFilter", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_name_filter: Option<String>,
    #[doc = "The activation ID of a deployed service package that matches the filter.\nIf not specified, the filter applies to all deployed service packages that match the other parameters.\nIf specified, the filter matches only the deployed service package with the specified activation ID."]
    #[serde(rename = "ServicePackageActivationIdFilter", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_id_filter: Option<String>,
    #[doc = "The filter for the health state of the deployed service packages. It allows selecting deployed service packages if they match the desired health states.\nThe possible values are integer value of one of the following health states. Only deployed service packages that match the filter are returned. All deployed service packages are used to evaluate the parent deployed application aggregated health state.\nIf not specified, default value is None, unless the deployed service package ID is specified. If the filter has default value and deployed service package ID is specified, the matching deployed service package is returned.\nThe state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise 'OR' operator.\nFor example, if the provided value is 6, it matches deployed service packages with HealthState value of OK (2) and Warning (4).\n\n- Default - Default value. Matches any HealthState. The value is zero.\n- None - Filter that doesn't match any HealthState value. Used in order to return no results on a given collection of states. The value is 1.\n- Ok - Filter that matches input with HealthState value Ok. The value is 2.\n- Warning - Filter that matches input with HealthState value Warning. The value is 4.\n- Error - Filter that matches input with HealthState value Error. The value is 8.\n- All - Filter that matches input with any HealthState value. The value is 65535."]
    #[serde(rename = "HealthStateFilter", default, skip_serializing_if = "Option::is_none")]
    pub health_state_filter: Option<i64>,
}
impl DeployedServicePackageHealthStateFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type DeployedServicePackageHealthStateList = Vec<DeployedServicePackageHealthState>;
#[doc = "Information about service package deployed on a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedServicePackageInfo {
    #[doc = "The name of the service manifest."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ServiceManifestName>,
    #[doc = "The version of the service package specified in service manifest."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Specifies the status of a deployed application or service package on a Service Fabric node."]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DeploymentStatus>,
    #[doc = "The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service\nis 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId\nis always an empty string."]
    #[serde(rename = "ServicePackageActivationId", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_id: Option<ServicePackageActivationId>,
}
impl DeployedServicePackageInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type DeployedServicePackageInfoList = Vec<DeployedServicePackageInfo>;
#[doc = "Deployed Service Health Report Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedServicePackageNewHealthReportEvent {
    #[serde(flatten)]
    pub application_event: ApplicationEvent,
    #[doc = "Service manifest name."]
    #[serde(rename = "ServiceManifestName")]
    pub service_manifest_name: String,
    #[doc = "Id of Service package instance."]
    #[serde(rename = "ServicePackageInstanceId")]
    pub service_package_instance_id: i64,
    #[doc = "Id of Service package activation."]
    #[serde(rename = "ServicePackageActivationId")]
    pub service_package_activation_id: String,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName")]
    pub node_name: NodeName,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl DeployedServicePackageNewHealthReportEvent {
    pub fn new(
        application_event: ApplicationEvent,
        service_manifest_name: String,
        service_package_instance_id: i64,
        service_package_activation_id: String,
        node_name: NodeName,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            application_event,
            service_manifest_name,
            service_package_instance_id,
            service_package_activation_id,
            node_name,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Represents health evaluation for deployed service packages, containing health evaluations for each unhealthy deployed service package that impacted current aggregated health state. Can be returned when evaluating deployed application health and the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedServicePackagesHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Total number of deployed service packages of the deployed application in the health store."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl DeployedServicePackagesHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            total_count: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Information about a Service Fabric service replica deployed on a node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedServiceReplicaDetailInfo {
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "ServiceKind")]
    pub service_kind: ServiceKind,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
    #[doc = "Specifies the current active life-cycle operation on a stateful service replica or stateless service instance."]
    #[serde(rename = "CurrentServiceOperation", default, skip_serializing_if = "Option::is_none")]
    pub current_service_operation: Option<ServiceOperationName>,
    #[doc = "The start time of the current service operation in UTC format."]
    #[serde(rename = "CurrentServiceOperationStartTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub current_service_operation_start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "List of load reported by replica."]
    #[serde(rename = "ReportedLoad", default, skip_serializing_if = "Option::is_none")]
    pub reported_load: Option<LoadMetricReportInfoList>,
}
impl DeployedServiceReplicaDetailInfo {
    pub fn new(service_kind: ServiceKind) -> Self {
        Self {
            service_kind,
            service_name: None,
            partition_id: None,
            current_service_operation: None,
            current_service_operation_start_time_utc: None,
            reported_load: None,
        }
    }
}
#[doc = "Information about a Service Fabric service replica deployed on a node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedServiceReplicaInfo {
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "ServiceKind")]
    pub service_kind: ServiceKind,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
    #[doc = "Name of the service type as specified in the service manifest."]
    #[serde(rename = "ServiceTypeName", default, skip_serializing_if = "Option::is_none")]
    pub service_type_name: Option<ServiceTypeName>,
    #[doc = "The name of the service manifest."]
    #[serde(rename = "ServiceManifestName", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_name: Option<ServiceManifestName>,
    #[doc = "The name of the code package defined in the service manifest."]
    #[serde(rename = "CodePackageName", default, skip_serializing_if = "Option::is_none")]
    pub code_package_name: Option<CodePackageName>,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
    #[doc = "The status of a replica of a service."]
    #[serde(rename = "ReplicaStatus", default, skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<ReplicaStatus>,
    #[doc = "The last address returned by the replica in Open or ChangeRole."]
    #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service\nis 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId\nis always an empty string."]
    #[serde(rename = "ServicePackageActivationId", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_id: Option<ServicePackageActivationId>,
    #[doc = "Host process ID of the process that is hosting the replica. This will be zero if the replica is down. In hyper-v containers this host process ID will be from different kernel."]
    #[serde(rename = "HostProcessId", default, skip_serializing_if = "Option::is_none")]
    pub host_process_id: Option<String>,
}
impl DeployedServiceReplicaInfo {
    pub fn new(service_kind: ServiceKind) -> Self {
        Self {
            service_kind,
            service_name: None,
            service_type_name: None,
            service_manifest_name: None,
            code_package_name: None,
            partition_id: None,
            replica_status: None,
            address: None,
            service_package_activation_id: None,
            host_process_id: None,
        }
    }
}
pub type DeployedServiceReplicaInfoList = Vec<DeployedServiceReplicaInfo>;
#[doc = "Information about service type deployed on a node, information such as the status of the service type registration on a node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedServiceTypeInfo {
    #[doc = "Name of the service type as specified in the service manifest."]
    #[serde(rename = "ServiceTypeName", default, skip_serializing_if = "Option::is_none")]
    pub service_type_name: Option<ServiceTypeName>,
    #[doc = "The name of the service manifest."]
    #[serde(rename = "ServiceManifestName", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_name: Option<ServiceManifestName>,
    #[doc = "The name of the code package defined in the service manifest."]
    #[serde(rename = "CodePackageName", default, skip_serializing_if = "Option::is_none")]
    pub code_package_name: Option<CodePackageName>,
    #[doc = "The status of the service type registration on the node."]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ServiceTypeRegistrationStatus>,
    #[doc = "The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service\nis 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId\nis always an empty string."]
    #[serde(rename = "ServicePackageActivationId", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_id: Option<ServicePackageActivationId>,
}
impl DeployedServiceTypeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type DeployedServiceTypeInfoList = Vec<DeployedServiceTypeInfo>;
#[doc = "Information about a stateful replica running in a code package. Note DeployedServiceReplicaQueryResult will contain duplicate data like ServiceKind, ServiceName, PartitionId and replicaId."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedStatefulServiceReplicaDetailInfo {
    #[serde(flatten)]
    pub deployed_service_replica_detail_info: DeployedServiceReplicaDetailInfo,
    #[doc = "Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id."]
    #[serde(rename = "ReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<ReplicaId>,
    #[doc = "Specifies the operation currently being executed by the Replicator."]
    #[serde(rename = "CurrentReplicatorOperation", default, skip_serializing_if = "Option::is_none")]
    pub current_replicator_operation: Option<ReplicatorOperationName>,
    #[doc = "Specifies the access status of the partition."]
    #[serde(rename = "ReadStatus", default, skip_serializing_if = "Option::is_none")]
    pub read_status: Option<PartitionAccessStatus>,
    #[doc = "Specifies the access status of the partition."]
    #[serde(rename = "WriteStatus", default, skip_serializing_if = "Option::is_none")]
    pub write_status: Option<PartitionAccessStatus>,
    #[doc = "Represents a base class for primary or secondary replicator status.\nContains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc."]
    #[serde(rename = "ReplicatorStatus", default, skip_serializing_if = "Option::is_none")]
    pub replicator_status: Option<ReplicatorStatus>,
    #[doc = "Key value store related information for the replica."]
    #[serde(rename = "ReplicaStatus", default, skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<KeyValueStoreReplicaStatus>,
    #[doc = "Information about a stateful service replica deployed on a node."]
    #[serde(rename = "DeployedServiceReplicaQueryResult", default, skip_serializing_if = "Option::is_none")]
    pub deployed_service_replica_query_result: Option<DeployedStatefulServiceReplicaInfo>,
}
impl DeployedStatefulServiceReplicaDetailInfo {
    pub fn new(deployed_service_replica_detail_info: DeployedServiceReplicaDetailInfo) -> Self {
        Self {
            deployed_service_replica_detail_info,
            replica_id: None,
            current_replicator_operation: None,
            read_status: None,
            write_status: None,
            replicator_status: None,
            replica_status: None,
            deployed_service_replica_query_result: None,
        }
    }
}
#[doc = "Information about a stateful service replica deployed on a node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedStatefulServiceReplicaInfo {
    #[serde(flatten)]
    pub deployed_service_replica_info: DeployedServiceReplicaInfo,
    #[doc = "Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id."]
    #[serde(rename = "ReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<ReplicaId>,
    #[doc = "The role of a replica of a stateful service."]
    #[serde(rename = "ReplicaRole", default, skip_serializing_if = "Option::is_none")]
    pub replica_role: Option<ReplicaRole>,
    #[doc = "Information about current reconfiguration like phase, type, previous configuration role of replica and reconfiguration start date time."]
    #[serde(rename = "ReconfigurationInformation", default, skip_serializing_if = "Option::is_none")]
    pub reconfiguration_information: Option<ReconfigurationInformation>,
}
impl DeployedStatefulServiceReplicaInfo {
    pub fn new(deployed_service_replica_info: DeployedServiceReplicaInfo) -> Self {
        Self {
            deployed_service_replica_info,
            replica_id: None,
            replica_role: None,
            reconfiguration_information: None,
        }
    }
}
#[doc = "Information about a stateless instance running in a code package. Note that DeployedServiceReplicaQueryResult will contain duplicate data like ServiceKind, ServiceName, PartitionId and InstanceId."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedStatelessServiceInstanceDetailInfo {
    #[serde(flatten)]
    pub deployed_service_replica_detail_info: DeployedServiceReplicaDetailInfo,
    #[doc = "Id of a stateless service instance. InstanceId is used by Service Fabric to uniquely identify an instance of a partition of a stateless service. It is unique within a partition and does not change for the lifetime of the instance. If the instance has failed over on the same or different node, it will get a different value for the InstanceId."]
    #[serde(rename = "InstanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<InstanceId>,
    #[doc = "Information about a stateless service instance deployed on a node."]
    #[serde(rename = "DeployedServiceReplicaQueryResult", default, skip_serializing_if = "Option::is_none")]
    pub deployed_service_replica_query_result: Option<DeployedStatelessServiceInstanceInfo>,
}
impl DeployedStatelessServiceInstanceDetailInfo {
    pub fn new(deployed_service_replica_detail_info: DeployedServiceReplicaDetailInfo) -> Self {
        Self {
            deployed_service_replica_detail_info,
            instance_id: None,
            deployed_service_replica_query_result: None,
        }
    }
}
#[doc = "Information about a stateless service instance deployed on a node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployedStatelessServiceInstanceInfo {
    #[serde(flatten)]
    pub deployed_service_replica_info: DeployedServiceReplicaInfo,
    #[doc = "Id of a stateless service instance. InstanceId is used by Service Fabric to uniquely identify an instance of a partition of a stateless service. It is unique within a partition and does not change for the lifetime of the instance. If the instance has failed over on the same or different node, it will get a different value for the InstanceId."]
    #[serde(rename = "InstanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<InstanceId>,
}
impl DeployedStatelessServiceInstanceInfo {
    pub fn new(deployed_service_replica_info: DeployedServiceReplicaInfo) -> Self {
        Self {
            deployed_service_replica_info,
            instance_id: None,
        }
    }
}
pub type DeploymentName = String;
#[doc = "Specifies the status of a deployed application or service package on a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeploymentStatus")]
pub enum DeploymentStatus {
    Invalid,
    Downloading,
    Activating,
    Active,
    Upgrading,
    Deactivating,
    RanToCompletion,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeploymentStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeploymentStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeploymentStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("DeploymentStatus", 0u32, "Invalid"),
            Self::Downloading => serializer.serialize_unit_variant("DeploymentStatus", 1u32, "Downloading"),
            Self::Activating => serializer.serialize_unit_variant("DeploymentStatus", 2u32, "Activating"),
            Self::Active => serializer.serialize_unit_variant("DeploymentStatus", 3u32, "Active"),
            Self::Upgrading => serializer.serialize_unit_variant("DeploymentStatus", 4u32, "Upgrading"),
            Self::Deactivating => serializer.serialize_unit_variant("DeploymentStatus", 5u32, "Deactivating"),
            Self::RanToCompletion => serializer.serialize_unit_variant("DeploymentStatus", 6u32, "RanToCompletion"),
            Self::Failed => serializer.serialize_unit_variant("DeploymentStatus", 7u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "It describes the body parameters while disabling backup of a backup entity(Application/Service/Partition)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisableBackupDescription {
    #[doc = "Boolean flag to delete backups. It can be set to true for deleting all the backups which were created for the backup entity that is getting disabled for backup."]
    #[serde(rename = "CleanBackup")]
    pub clean_backup: bool,
}
impl DisableBackupDescription {
    pub fn new(clean_backup: bool) -> Self {
        Self { clean_backup }
    }
}
#[doc = "Information about the disk"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskInfo {
    #[doc = "the disk size in bytes"]
    #[serde(rename = "Capacity", default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    #[doc = "the available disk space in bytes"]
    #[serde(rename = "AvailableSpace", default, skip_serializing_if = "Option::is_none")]
    pub available_space: Option<String>,
}
impl DiskInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Service Fabric property value of type Double."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoublePropertyValue {
    #[serde(flatten)]
    pub property_value: PropertyValue,
    #[doc = "The data of the property value."]
    #[serde(rename = "Data")]
    pub data: f64,
}
impl DoublePropertyValue {
    pub fn new(property_value: PropertyValue, data: f64) -> Self {
        Self { property_value, data }
    }
}
#[doc = "Describes the parameters for Dsms Azure blob store used for storing and enumerating backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DsmsAzureBlobBackupStorageDescription {
    #[serde(flatten)]
    pub backup_storage_description: BackupStorageDescription,
    #[doc = "The source location of the storage credentials to connect to the Dsms Azure blob store."]
    #[serde(rename = "StorageCredentialsSourceLocation")]
    pub storage_credentials_source_location: String,
    #[doc = "The name of the container in the blob store to store and enumerate backups from."]
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}
impl DsmsAzureBlobBackupStorageDescription {
    pub fn new(
        backup_storage_description: BackupStorageDescription,
        storage_credentials_source_location: String,
        container_name: String,
    ) -> Self {
        Self {
            backup_storage_description,
            storage_credentials_source_location,
            container_name,
        }
    }
}
#[doc = "Specifies the parameters needed to enable periodic backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableBackupDescription {
    #[doc = "Name of the backup policy to be used for enabling periodic backups."]
    #[serde(rename = "BackupPolicyName")]
    pub backup_policy_name: String,
}
impl EnableBackupDescription {
    pub fn new(backup_policy_name: String) -> Self {
        Self { backup_policy_name }
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
#[doc = "Safety check that waits to ensure the availability of the partition. It waits until there are replicas available such that bringing down this replica will not cause availability loss for the partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnsureAvailabilitySafetyCheck {
    #[serde(flatten)]
    pub partition_safety_check: PartitionSafetyCheck,
}
impl EnsureAvailabilitySafetyCheck {
    pub fn new(partition_safety_check: PartitionSafetyCheck) -> Self {
        Self { partition_safety_check }
    }
}
#[doc = "Safety check that ensures that a quorum of replicas are not lost for a partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnsurePartitionQuorumSafetyCheck {
    #[serde(flatten)]
    pub partition_safety_check: PartitionSafetyCheck,
}
impl EnsurePartitionQuorumSafetyCheck {
    pub fn new(partition_safety_check: PartitionSafetyCheck) -> Self {
        Self { partition_safety_check }
    }
}
#[doc = "Health information common to all entities in the cluster. It contains the aggregated health state, health events and unhealthy evaluation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityHealth {
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "AggregatedHealthState", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_health_state: Option<HealthState>,
    #[doc = "The list of health events reported on the entity."]
    #[serde(rename = "HealthEvents", default, skip_serializing_if = "Vec::is_empty")]
    pub health_events: Vec<HealthEvent>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
    #[doc = "The health statistics of an entity, returned as part of the health query result when the query description is configured to include statistics.\nThe statistics include health state counts for all children types of the current entity.\nFor example, for cluster, the health statistics include health state counts for nodes, applications, services, partitions, replicas, deployed applications and deployed service packages.\nFor partition, the health statistics include health counts for replicas."]
    #[serde(rename = "HealthStatistics", default, skip_serializing_if = "Option::is_none")]
    pub health_statistics: Option<HealthStatistics>,
}
impl EntityHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A base type for the health state of various entities in the cluster. It contains the aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityHealthState {
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "AggregatedHealthState", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_health_state: Option<HealthState>,
}
impl EntityHealthState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A base type for the health state chunk of various entities in the cluster. It contains the aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityHealthStateChunk {
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "HealthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
}
impl EntityHealthStateChunk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A base type for the list of health state chunks found in the cluster. It contains the total number of health states that match the input filters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityHealthStateChunkList {
    #[doc = "Total number of entity health state objects that match the specified filters from the cluster health chunk query description."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
impl EntityHealthStateChunkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The entity type of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityKind")]
pub enum EntityKind {
    Invalid,
    Node,
    Partition,
    Service,
    Application,
    Replica,
    DeployedApplication,
    DeployedServicePackage,
    Cluster,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("EntityKind", 0u32, "Invalid"),
            Self::Node => serializer.serialize_unit_variant("EntityKind", 1u32, "Node"),
            Self::Partition => serializer.serialize_unit_variant("EntityKind", 2u32, "Partition"),
            Self::Service => serializer.serialize_unit_variant("EntityKind", 3u32, "Service"),
            Self::Application => serializer.serialize_unit_variant("EntityKind", 4u32, "Application"),
            Self::Replica => serializer.serialize_unit_variant("EntityKind", 5u32, "Replica"),
            Self::DeployedApplication => serializer.serialize_unit_variant("EntityKind", 6u32, "DeployedApplication"),
            Self::DeployedServicePackage => serializer.serialize_unit_variant("EntityKind", 7u32, "DeployedServicePackage"),
            Self::Cluster => serializer.serialize_unit_variant("EntityKind", 8u32, "Cluster"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents health state count for entities of the specified entity kind."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityKindHealthStateCount {
    #[doc = "The entity type of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "EntityKind", default, skip_serializing_if = "Option::is_none")]
    pub entity_kind: Option<EntityKind>,
    #[doc = "Represents information about how many health entities are in Ok, Warning and Error health state."]
    #[serde(rename = "HealthStateCount", default, skip_serializing_if = "Option::is_none")]
    pub health_state_count: Option<HealthStateCount>,
}
impl EntityKindHealthStateCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the status of the code package entry point deployed on a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntryPointStatus")]
pub enum EntryPointStatus {
    Invalid,
    Pending,
    Starting,
    Started,
    Stopping,
    Stopped,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntryPointStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntryPointStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntryPointStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("EntryPointStatus", 0u32, "Invalid"),
            Self::Pending => serializer.serialize_unit_variant("EntryPointStatus", 1u32, "Pending"),
            Self::Starting => serializer.serialize_unit_variant("EntryPointStatus", 2u32, "Starting"),
            Self::Started => serializer.serialize_unit_variant("EntryPointStatus", 3u32, "Started"),
            Self::Stopping => serializer.serialize_unit_variant("EntryPointStatus", 4u32, "Stopping"),
            Self::Stopped => serializer.serialize_unit_variant("EntryPointStatus", 5u32, "Stopped"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes an environment variable for the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentVariable {
    #[doc = "The type of the environment variable being given in value"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<EnvironmentVariableType>,
    #[doc = "The name of the environment variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the environment variable, will be processed based on the type provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl EnvironmentVariable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of the environment variable being given in value"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentVariableType")]
pub enum EnvironmentVariableType {
    ClearText,
    KeyVaultReference,
    SecretValueReference,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentVariableType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentVariableType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentVariableType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ClearText => serializer.serialize_unit_variant("EnvironmentVariableType", 0u32, "ClearText"),
            Self::KeyVaultReference => serializer.serialize_unit_variant("EnvironmentVariableType", 1u32, "KeyVaultReference"),
            Self::SecretValueReference => serializer.serialize_unit_variant("EnvironmentVariableType", 2u32, "SecretValueReference"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for EnvironmentVariableType {
    fn default() -> Self {
        Self::ClearText
    }
}
#[doc = "An Epoch is a configuration number for the partition as a whole. When the configuration of the replica set changes, for example when the Primary replica changes, the operations that are replicated from the new Primary replica are said to be a new Epoch from the ones which were sent by the old Primary replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Epoch {
    #[doc = "The current configuration number of this Epoch. The configuration number is an increasing value that is updated whenever the configuration of this replica set changes."]
    #[serde(rename = "ConfigurationVersion", default, skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<String>,
    #[doc = "The current data loss number of this Epoch. The data loss number property is an increasing value which is updated whenever data loss is suspected, as when loss of a quorum of replicas in the replica set that includes the Primary replica."]
    #[serde(rename = "DataLossVersion", default, skip_serializing_if = "Option::is_none")]
    pub data_loss_version: Option<String>,
}
impl Epoch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents health evaluation of a HealthEvent that was reported on the entity.\nThe health evaluation is returned when evaluating health of an entity results in Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Indicates whether warnings are treated with the same severity as errors. The field is specified in the health policy used to evaluate the entity."]
    #[serde(rename = "ConsiderWarningAsError", default, skip_serializing_if = "Option::is_none")]
    pub consider_warning_as_error: Option<bool>,
    #[doc = "Represents health information reported on a health entity, such as cluster, application or node, with additional metadata added by the Health Manager."]
    #[serde(rename = "UnhealthyEvent", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_event: Option<HealthEvent>,
}
impl EventHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            consider_warning_as_error: None,
            unhealthy_event: None,
        }
    }
}
pub type EventList = Vec<FabricEvent>;
#[doc = "Describes a Chaos event that gets generated when Chaos has decided on the faults for an iteration. This Chaos event contains the details of the faults as a list of strings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutingFaultsChaosEvent {
    #[serde(flatten)]
    pub chaos_event: ChaosEvent,
    #[doc = "List of string description of the faults that Chaos decided to execute in an iteration."]
    #[serde(rename = "Faults", default, skip_serializing_if = "Vec::is_empty")]
    pub faults: Vec<String>,
}
impl ExecutingFaultsChaosEvent {
    pub fn new(chaos_event: ChaosEvent) -> Self {
        Self {
            chaos_event,
            faults: Vec::new(),
        }
    }
}
#[doc = "The execution policy of the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionPolicy {
    #[doc = "Enumerates the execution policy types for services."]
    #[serde(rename = "type")]
    pub type_: ExecutionPolicyType,
}
impl ExecutionPolicy {
    pub fn new(type_: ExecutionPolicyType) -> Self {
        Self { type_ }
    }
}
#[doc = "Enumerates the execution policy types for services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExecutionPolicyType")]
pub enum ExecutionPolicyType {
    Default,
    RunToCompletion,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExecutionPolicyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExecutionPolicyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExecutionPolicyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("ExecutionPolicyType", 0u32, "Default"),
            Self::RunToCompletion => serializer.serialize_unit_variant("ExecutionPolicyType", 1u32, "RunToCompletion"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the operation to register or provision an application type using an application package from an external store instead of a package uploaded to the Service Fabric image store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalStoreProvisionApplicationTypeDescription {
    #[serde(flatten)]
    pub provision_application_type_description_base: ProvisionApplicationTypeDescriptionBase,
    #[doc = "The path to the '.sfpkg' application package from where the application package can be downloaded using HTTP or HTTPS protocols. The application package can be stored in an external store that provides GET operation to download the file. Supported protocols are HTTP and HTTPS, and the path must allow READ access."]
    #[serde(rename = "ApplicationPackageDownloadUri")]
    pub application_package_download_uri: String,
    #[doc = "The application type name represents the name of the application type found in the application manifest."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: String,
    #[doc = "The application type version represents the version of the application type found in the application manifest."]
    #[serde(rename = "ApplicationTypeVersion")]
    pub application_type_version: String,
}
impl ExternalStoreProvisionApplicationTypeDescription {
    pub fn new(
        provision_application_type_description_base: ProvisionApplicationTypeDescriptionBase,
        application_package_download_uri: String,
        application_type_name: String,
        application_type_version: String,
    ) -> Self {
        Self {
            provision_application_type_description_base,
            application_package_download_uri,
            application_type_name,
            application_type_version,
        }
    }
}
#[doc = "Information about a Service Fabric code version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricCodeVersionInfo {
    #[doc = "The product version of Service Fabric."]
    #[serde(rename = "CodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub code_version: Option<String>,
}
impl FabricCodeVersionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type FabricCodeVersionInfoList = Vec<FabricCodeVersionInfo>;
#[doc = "Information about a Service Fabric config version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricConfigVersionInfo {
    #[doc = "The config version of Service Fabric."]
    #[serde(rename = "ConfigVersion", default, skip_serializing_if = "Option::is_none")]
    pub config_version: Option<String>,
}
impl FabricConfigVersionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type FabricConfigVersionInfoList = Vec<FabricConfigVersionInfo>;
#[doc = "The REST API operations for Service Fabric return standard HTTP status codes. This type defines the additional information returned from the Service Fabric API operations that are not successful."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricError {
    #[doc = "Error object containing error code and error message."]
    #[serde(rename = "Error")]
    pub error: FabricErrorError,
}
impl FabricError {
    pub fn new(error: FabricErrorError) -> Self {
        Self { error }
    }
}
#[doc = "Defines the fabric error codes that be returned as part of the error object in response to Service Fabric API operations that are not successful. Following are the error code values that can be returned for a specific HTTP status code.\n\n  - Possible values of the error code for HTTP status code 400 (Bad Request)\n    - \"FABRIC_E_INVALID_PARTITION_KEY\"\n    - \"FABRIC_E_IMAGEBUILDER_VALIDATION_ERROR\"\n    - \"FABRIC_E_INVALID_ADDRESS\"\n    - \"FABRIC_E_APPLICATION_NOT_UPGRADING\"\n    - \"FABRIC_E_APPLICATION_UPGRADE_VALIDATION_ERROR\"\n    - \"FABRIC_E_FABRIC_NOT_UPGRADING\"\n    - \"FABRIC_E_FABRIC_UPGRADE_VALIDATION_ERROR\"\n    - \"FABRIC_E_INVALID_CONFIGURATION\"\n    - \"FABRIC_E_INVALID_NAME_URI\"\n    - \"FABRIC_E_PATH_TOO_LONG\"\n    - \"FABRIC_E_KEY_TOO_LARGE\"\n    - \"FABRIC_E_SERVICE_AFFINITY_CHAIN_NOT_SUPPORTED\"\n    - \"FABRIC_E_INVALID_ATOMIC_GROUP\"\n    - \"FABRIC_E_VALUE_EMPTY\"\n    - \"FABRIC_E_BACKUP_IS_ENABLED\"\n    - \"FABRIC_E_RESTORE_SOURCE_TARGET_PARTITION_MISMATCH\"\n    - \"FABRIC_E_INVALID_FOR_STATELESS_SERVICES\"\n    - \"FABRIC_E_INVALID_SERVICE_SCALING_POLICY\"\n    - \"E_INVALIDARG\"\n\n  - Possible values of the error code for HTTP status code 404 (Not Found)\n    - \"FABRIC_E_NODE_NOT_FOUND\"\n    - \"FABRIC_E_APPLICATION_TYPE_NOT_FOUND\"\n    - \"FABRIC_E_APPLICATION_NOT_FOUND\"\n    - \"FABRIC_E_SERVICE_TYPE_NOT_FOUND\"\n    - \"FABRIC_E_SERVICE_DOES_NOT_EXIST\"\n    - \"FABRIC_E_SERVICE_TYPE_TEMPLATE_NOT_FOUND\"\n    - \"FABRIC_E_CONFIGURATION_SECTION_NOT_FOUND\"\n    - \"FABRIC_E_PARTITION_NOT_FOUND\"\n    - \"FABRIC_E_REPLICA_DOES_NOT_EXIST\"\n    - \"FABRIC_E_SERVICE_GROUP_DOES_NOT_EXIST\"\n    - \"FABRIC_E_CONFIGURATION_PARAMETER_NOT_FOUND\"\n    - \"FABRIC_E_DIRECTORY_NOT_FOUND\"\n    - \"FABRIC_E_FABRIC_VERSION_NOT_FOUND\"\n    - \"FABRIC_E_FILE_NOT_FOUND\"\n    - \"FABRIC_E_NAME_DOES_NOT_EXIST\"\n    - \"FABRIC_E_PROPERTY_DOES_NOT_EXIST\"\n    - \"FABRIC_E_ENUMERATION_COMPLETED\"\n    - \"FABRIC_E_SERVICE_MANIFEST_NOT_FOUND\"\n    - \"FABRIC_E_KEY_NOT_FOUND\"\n    - \"FABRIC_E_HEALTH_ENTITY_NOT_FOUND\"\n    - \"FABRIC_E_BACKUP_NOT_ENABLED\"\n    - \"FABRIC_E_BACKUP_POLICY_NOT_EXISTING\"\n    - \"FABRIC_E_FAULT_ANALYSIS_SERVICE_NOT_EXISTING\"\n    - \"FABRIC_E_IMAGEBUILDER_RESERVED_DIRECTORY_ERROR\"\n\n  - Possible values of the error code for HTTP status code 409 (Conflict)\n    - \"FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS\"\n    - \"FABRIC_E_APPLICATION_ALREADY_EXISTS\"\n    - \"FABRIC_E_APPLICATION_ALREADY_IN_TARGET_VERSION\"\n    - \"FABRIC_E_APPLICATION_TYPE_PROVISION_IN_PROGRESS\"\n    - \"FABRIC_E_APPLICATION_UPGRADE_IN_PROGRESS\"\n    - \"FABRIC_E_SERVICE_ALREADY_EXISTS\"\n    - \"FABRIC_E_SERVICE_GROUP_ALREADY_EXISTS\"\n    - \"FABRIC_E_APPLICATION_TYPE_IN_USE\"\n    - \"FABRIC_E_FABRIC_ALREADY_IN_TARGET_VERSION\"\n    - \"FABRIC_E_FABRIC_VERSION_ALREADY_EXISTS\"\n    - \"FABRIC_E_FABRIC_VERSION_IN_USE\"\n    - \"FABRIC_E_FABRIC_UPGRADE_IN_PROGRESS\"\n    - \"FABRIC_E_NAME_ALREADY_EXISTS\"\n    - \"FABRIC_E_NAME_NOT_EMPTY\"\n    - \"FABRIC_E_PROPERTY_CHECK_FAILED\"\n    - \"FABRIC_E_SERVICE_METADATA_MISMATCH\"\n    - \"FABRIC_E_SERVICE_TYPE_MISMATCH\"\n    - \"FABRIC_E_HEALTH_STALE_REPORT\"\n    - \"FABRIC_E_SEQUENCE_NUMBER_CHECK_FAILED\"\n    - \"FABRIC_E_NODE_HAS_NOT_STOPPED_YET\"\n    - \"FABRIC_E_INSTANCE_ID_MISMATCH\"\n    - \"FABRIC_E_BACKUP_IN_PROGRESS\"\n    - \"FABRIC_E_RESTORE_IN_PROGRESS\"\n    - \"FABRIC_E_BACKUP_POLICY_ALREADY_EXISTING\"\n\n  - Possible values of the error code for HTTP status code 413 (Request Entity Too Large)\n    - \"FABRIC_E_VALUE_TOO_LARGE\"\n\n  - Possible values of the error code for HTTP status code 500 (Internal Server Error)\n    - \"FABRIC_E_NODE_IS_UP\"\n    - \"E_FAIL\"\n    - \"FABRIC_E_SINGLE_INSTANCE_APPLICATION_ALREADY_EXISTS\"\n    - \"FABRIC_E_SINGLE_INSTANCE_APPLICATION_NOT_FOUND\"\n    - \"FABRIC_E_VOLUME_ALREADY_EXISTS\"\n    - \"FABRIC_E_VOLUME_NOT_FOUND\"\n    - \"SerializationError\"\n\n  - Possible values of the error code for HTTP status code 503 (Service Unavailable)\n    - \"FABRIC_E_NO_WRITE_QUORUM\"\n    - \"FABRIC_E_NOT_PRIMARY\"\n    - \"FABRIC_E_NOT_READY\"\n    - \"FABRIC_E_RECONFIGURATION_PENDING\"\n    - \"FABRIC_E_SERVICE_OFFLINE\"\n    - \"E_ABORT\"\n    - \"FABRIC_E_VALUE_TOO_LARGE\"\n\n  - Possible values of the error code for HTTP status code 504 (Gateway Timeout)\n    - \"FABRIC_E_COMMUNICATION_ERROR\"\n    - \"FABRIC_E_OPERATION_NOT_COMPLETE\"\n    - \"FABRIC_E_TIMEOUT\""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FabricErrorCodes")]
pub enum FabricErrorCodes {
    #[serde(rename = "FABRIC_E_INVALID_PARTITION_KEY")]
    FabricEInvalidPartitionKey,
    #[serde(rename = "FABRIC_E_IMAGEBUILDER_VALIDATION_ERROR")]
    FabricEImagebuilderValidationError,
    #[serde(rename = "FABRIC_E_INVALID_ADDRESS")]
    FabricEInvalidAddress,
    #[serde(rename = "FABRIC_E_APPLICATION_NOT_UPGRADING")]
    FabricEApplicationNotUpgrading,
    #[serde(rename = "FABRIC_E_APPLICATION_UPGRADE_VALIDATION_ERROR")]
    FabricEApplicationUpgradeValidationError,
    #[serde(rename = "FABRIC_E_FABRIC_NOT_UPGRADING")]
    FabricEFabricNotUpgrading,
    #[serde(rename = "FABRIC_E_FABRIC_UPGRADE_VALIDATION_ERROR")]
    FabricEFabricUpgradeValidationError,
    #[serde(rename = "FABRIC_E_INVALID_CONFIGURATION")]
    FabricEInvalidConfiguration,
    #[serde(rename = "FABRIC_E_INVALID_NAME_URI")]
    FabricEInvalidNameUri,
    #[serde(rename = "FABRIC_E_PATH_TOO_LONG")]
    FabricEPathTooLong,
    #[serde(rename = "FABRIC_E_KEY_TOO_LARGE")]
    FabricEKeyTooLarge,
    #[serde(rename = "FABRIC_E_SERVICE_AFFINITY_CHAIN_NOT_SUPPORTED")]
    FabricEServiceAffinityChainNotSupported,
    #[serde(rename = "FABRIC_E_INVALID_ATOMIC_GROUP")]
    FabricEInvalidAtomicGroup,
    #[serde(rename = "FABRIC_E_VALUE_EMPTY")]
    FabricEValueEmpty,
    #[serde(rename = "FABRIC_E_NODE_NOT_FOUND")]
    FabricENodeNotFound,
    #[serde(rename = "FABRIC_E_APPLICATION_TYPE_NOT_FOUND")]
    FabricEApplicationTypeNotFound,
    #[serde(rename = "FABRIC_E_APPLICATION_NOT_FOUND")]
    FabricEApplicationNotFound,
    #[serde(rename = "FABRIC_E_SERVICE_TYPE_NOT_FOUND")]
    FabricEServiceTypeNotFound,
    #[serde(rename = "FABRIC_E_SERVICE_DOES_NOT_EXIST")]
    FabricEServiceDoesNotExist,
    #[serde(rename = "FABRIC_E_SERVICE_TYPE_TEMPLATE_NOT_FOUND")]
    FabricEServiceTypeTemplateNotFound,
    #[serde(rename = "FABRIC_E_CONFIGURATION_SECTION_NOT_FOUND")]
    FabricEConfigurationSectionNotFound,
    #[serde(rename = "FABRIC_E_PARTITION_NOT_FOUND")]
    FabricEPartitionNotFound,
    #[serde(rename = "FABRIC_E_REPLICA_DOES_NOT_EXIST")]
    FabricEReplicaDoesNotExist,
    #[serde(rename = "FABRIC_E_SERVICE_GROUP_DOES_NOT_EXIST")]
    FabricEServiceGroupDoesNotExist,
    #[serde(rename = "FABRIC_E_CONFIGURATION_PARAMETER_NOT_FOUND")]
    FabricEConfigurationParameterNotFound,
    #[serde(rename = "FABRIC_E_DIRECTORY_NOT_FOUND")]
    FabricEDirectoryNotFound,
    #[serde(rename = "FABRIC_E_FABRIC_VERSION_NOT_FOUND")]
    FabricEFabricVersionNotFound,
    #[serde(rename = "FABRIC_E_FILE_NOT_FOUND")]
    FabricEFileNotFound,
    #[serde(rename = "FABRIC_E_NAME_DOES_NOT_EXIST")]
    FabricENameDoesNotExist,
    #[serde(rename = "FABRIC_E_PROPERTY_DOES_NOT_EXIST")]
    FabricEPropertyDoesNotExist,
    #[serde(rename = "FABRIC_E_ENUMERATION_COMPLETED")]
    FabricEEnumerationCompleted,
    #[serde(rename = "FABRIC_E_SERVICE_MANIFEST_NOT_FOUND")]
    FabricEServiceManifestNotFound,
    #[serde(rename = "FABRIC_E_KEY_NOT_FOUND")]
    FabricEKeyNotFound,
    #[serde(rename = "FABRIC_E_HEALTH_ENTITY_NOT_FOUND")]
    FabricEHealthEntityNotFound,
    #[serde(rename = "FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS")]
    FabricEApplicationTypeAlreadyExists,
    #[serde(rename = "FABRIC_E_APPLICATION_ALREADY_EXISTS")]
    FabricEApplicationAlreadyExists,
    #[serde(rename = "FABRIC_E_APPLICATION_ALREADY_IN_TARGET_VERSION")]
    FabricEApplicationAlreadyInTargetVersion,
    #[serde(rename = "FABRIC_E_APPLICATION_TYPE_PROVISION_IN_PROGRESS")]
    FabricEApplicationTypeProvisionInProgress,
    #[serde(rename = "FABRIC_E_APPLICATION_UPGRADE_IN_PROGRESS")]
    FabricEApplicationUpgradeInProgress,
    #[serde(rename = "FABRIC_E_SERVICE_ALREADY_EXISTS")]
    FabricEServiceAlreadyExists,
    #[serde(rename = "FABRIC_E_SERVICE_GROUP_ALREADY_EXISTS")]
    FabricEServiceGroupAlreadyExists,
    #[serde(rename = "FABRIC_E_APPLICATION_TYPE_IN_USE")]
    FabricEApplicationTypeInUse,
    #[serde(rename = "FABRIC_E_FABRIC_ALREADY_IN_TARGET_VERSION")]
    FabricEFabricAlreadyInTargetVersion,
    #[serde(rename = "FABRIC_E_FABRIC_VERSION_ALREADY_EXISTS")]
    FabricEFabricVersionAlreadyExists,
    #[serde(rename = "FABRIC_E_FABRIC_VERSION_IN_USE")]
    FabricEFabricVersionInUse,
    #[serde(rename = "FABRIC_E_FABRIC_UPGRADE_IN_PROGRESS")]
    FabricEFabricUpgradeInProgress,
    #[serde(rename = "FABRIC_E_NAME_ALREADY_EXISTS")]
    FabricENameAlreadyExists,
    #[serde(rename = "FABRIC_E_NAME_NOT_EMPTY")]
    FabricENameNotEmpty,
    #[serde(rename = "FABRIC_E_PROPERTY_CHECK_FAILED")]
    FabricEPropertyCheckFailed,
    #[serde(rename = "FABRIC_E_SERVICE_METADATA_MISMATCH")]
    FabricEServiceMetadataMismatch,
    #[serde(rename = "FABRIC_E_SERVICE_TYPE_MISMATCH")]
    FabricEServiceTypeMismatch,
    #[serde(rename = "FABRIC_E_HEALTH_STALE_REPORT")]
    FabricEHealthStaleReport,
    #[serde(rename = "FABRIC_E_SEQUENCE_NUMBER_CHECK_FAILED")]
    FabricESequenceNumberCheckFailed,
    #[serde(rename = "FABRIC_E_NODE_HAS_NOT_STOPPED_YET")]
    FabricENodeHasNotStoppedYet,
    #[serde(rename = "FABRIC_E_INSTANCE_ID_MISMATCH")]
    FabricEInstanceIdMismatch,
    #[serde(rename = "FABRIC_E_VALUE_TOO_LARGE")]
    FabricEValueTooLarge,
    #[serde(rename = "FABRIC_E_NO_WRITE_QUORUM")]
    FabricENoWriteQuorum,
    #[serde(rename = "FABRIC_E_NOT_PRIMARY")]
    FabricENotPrimary,
    #[serde(rename = "FABRIC_E_NOT_READY")]
    FabricENotReady,
    #[serde(rename = "FABRIC_E_RECONFIGURATION_PENDING")]
    FabricEReconfigurationPending,
    #[serde(rename = "FABRIC_E_SERVICE_OFFLINE")]
    FabricEServiceOffline,
    #[serde(rename = "E_ABORT")]
    EAbort,
    #[serde(rename = "FABRIC_E_COMMUNICATION_ERROR")]
    FabricECommunicationError,
    #[serde(rename = "FABRIC_E_OPERATION_NOT_COMPLETE")]
    FabricEOperationNotComplete,
    #[serde(rename = "FABRIC_E_TIMEOUT")]
    FabricETimeout,
    #[serde(rename = "FABRIC_E_NODE_IS_UP")]
    FabricENodeIsUp,
    #[serde(rename = "E_FAIL")]
    EFail,
    #[serde(rename = "FABRIC_E_BACKUP_IS_ENABLED")]
    FabricEBackupIsEnabled,
    #[serde(rename = "FABRIC_E_RESTORE_SOURCE_TARGET_PARTITION_MISMATCH")]
    FabricERestoreSourceTargetPartitionMismatch,
    #[serde(rename = "FABRIC_E_INVALID_FOR_STATELESS_SERVICES")]
    FabricEInvalidForStatelessServices,
    #[serde(rename = "FABRIC_E_BACKUP_NOT_ENABLED")]
    FabricEBackupNotEnabled,
    #[serde(rename = "FABRIC_E_BACKUP_POLICY_NOT_EXISTING")]
    FabricEBackupPolicyNotExisting,
    #[serde(rename = "FABRIC_E_FAULT_ANALYSIS_SERVICE_NOT_EXISTING")]
    FabricEFaultAnalysisServiceNotExisting,
    #[serde(rename = "FABRIC_E_BACKUP_IN_PROGRESS")]
    FabricEBackupInProgress,
    #[serde(rename = "FABRIC_E_RESTORE_IN_PROGRESS")]
    FabricERestoreInProgress,
    #[serde(rename = "FABRIC_E_BACKUP_POLICY_ALREADY_EXISTING")]
    FabricEBackupPolicyAlreadyExisting,
    #[serde(rename = "FABRIC_E_INVALID_SERVICE_SCALING_POLICY")]
    FabricEInvalidServiceScalingPolicy,
    #[serde(rename = "E_INVALIDARG")]
    EInvalidarg,
    #[serde(rename = "FABRIC_E_SINGLE_INSTANCE_APPLICATION_ALREADY_EXISTS")]
    FabricESingleInstanceApplicationAlreadyExists,
    #[serde(rename = "FABRIC_E_SINGLE_INSTANCE_APPLICATION_NOT_FOUND")]
    FabricESingleInstanceApplicationNotFound,
    #[serde(rename = "FABRIC_E_VOLUME_ALREADY_EXISTS")]
    FabricEVolumeAlreadyExists,
    #[serde(rename = "FABRIC_E_VOLUME_NOT_FOUND")]
    FabricEVolumeNotFound,
    SerializationError,
    #[serde(rename = "FABRIC_E_IMAGEBUILDER_RESERVED_DIRECTORY_ERROR")]
    FabricEImagebuilderReservedDirectoryError,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FabricErrorCodes {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FabricErrorCodes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FabricErrorCodes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::FabricEInvalidPartitionKey => {
                serializer.serialize_unit_variant("FabricErrorCodes", 0u32, "FABRIC_E_INVALID_PARTITION_KEY")
            }
            Self::FabricEImagebuilderValidationError => {
                serializer.serialize_unit_variant("FabricErrorCodes", 1u32, "FABRIC_E_IMAGEBUILDER_VALIDATION_ERROR")
            }
            Self::FabricEInvalidAddress => serializer.serialize_unit_variant("FabricErrorCodes", 2u32, "FABRIC_E_INVALID_ADDRESS"),
            Self::FabricEApplicationNotUpgrading => {
                serializer.serialize_unit_variant("FabricErrorCodes", 3u32, "FABRIC_E_APPLICATION_NOT_UPGRADING")
            }
            Self::FabricEApplicationUpgradeValidationError => {
                serializer.serialize_unit_variant("FabricErrorCodes", 4u32, "FABRIC_E_APPLICATION_UPGRADE_VALIDATION_ERROR")
            }
            Self::FabricEFabricNotUpgrading => serializer.serialize_unit_variant("FabricErrorCodes", 5u32, "FABRIC_E_FABRIC_NOT_UPGRADING"),
            Self::FabricEFabricUpgradeValidationError => {
                serializer.serialize_unit_variant("FabricErrorCodes", 6u32, "FABRIC_E_FABRIC_UPGRADE_VALIDATION_ERROR")
            }
            Self::FabricEInvalidConfiguration => {
                serializer.serialize_unit_variant("FabricErrorCodes", 7u32, "FABRIC_E_INVALID_CONFIGURATION")
            }
            Self::FabricEInvalidNameUri => serializer.serialize_unit_variant("FabricErrorCodes", 8u32, "FABRIC_E_INVALID_NAME_URI"),
            Self::FabricEPathTooLong => serializer.serialize_unit_variant("FabricErrorCodes", 9u32, "FABRIC_E_PATH_TOO_LONG"),
            Self::FabricEKeyTooLarge => serializer.serialize_unit_variant("FabricErrorCodes", 10u32, "FABRIC_E_KEY_TOO_LARGE"),
            Self::FabricEServiceAffinityChainNotSupported => {
                serializer.serialize_unit_variant("FabricErrorCodes", 11u32, "FABRIC_E_SERVICE_AFFINITY_CHAIN_NOT_SUPPORTED")
            }
            Self::FabricEInvalidAtomicGroup => {
                serializer.serialize_unit_variant("FabricErrorCodes", 12u32, "FABRIC_E_INVALID_ATOMIC_GROUP")
            }
            Self::FabricEValueEmpty => serializer.serialize_unit_variant("FabricErrorCodes", 13u32, "FABRIC_E_VALUE_EMPTY"),
            Self::FabricENodeNotFound => serializer.serialize_unit_variant("FabricErrorCodes", 14u32, "FABRIC_E_NODE_NOT_FOUND"),
            Self::FabricEApplicationTypeNotFound => {
                serializer.serialize_unit_variant("FabricErrorCodes", 15u32, "FABRIC_E_APPLICATION_TYPE_NOT_FOUND")
            }
            Self::FabricEApplicationNotFound => {
                serializer.serialize_unit_variant("FabricErrorCodes", 16u32, "FABRIC_E_APPLICATION_NOT_FOUND")
            }
            Self::FabricEServiceTypeNotFound => {
                serializer.serialize_unit_variant("FabricErrorCodes", 17u32, "FABRIC_E_SERVICE_TYPE_NOT_FOUND")
            }
            Self::FabricEServiceDoesNotExist => {
                serializer.serialize_unit_variant("FabricErrorCodes", 18u32, "FABRIC_E_SERVICE_DOES_NOT_EXIST")
            }
            Self::FabricEServiceTypeTemplateNotFound => {
                serializer.serialize_unit_variant("FabricErrorCodes", 19u32, "FABRIC_E_SERVICE_TYPE_TEMPLATE_NOT_FOUND")
            }
            Self::FabricEConfigurationSectionNotFound => {
                serializer.serialize_unit_variant("FabricErrorCodes", 20u32, "FABRIC_E_CONFIGURATION_SECTION_NOT_FOUND")
            }
            Self::FabricEPartitionNotFound => serializer.serialize_unit_variant("FabricErrorCodes", 21u32, "FABRIC_E_PARTITION_NOT_FOUND"),
            Self::FabricEReplicaDoesNotExist => {
                serializer.serialize_unit_variant("FabricErrorCodes", 22u32, "FABRIC_E_REPLICA_DOES_NOT_EXIST")
            }
            Self::FabricEServiceGroupDoesNotExist => {
                serializer.serialize_unit_variant("FabricErrorCodes", 23u32, "FABRIC_E_SERVICE_GROUP_DOES_NOT_EXIST")
            }
            Self::FabricEConfigurationParameterNotFound => {
                serializer.serialize_unit_variant("FabricErrorCodes", 24u32, "FABRIC_E_CONFIGURATION_PARAMETER_NOT_FOUND")
            }
            Self::FabricEDirectoryNotFound => serializer.serialize_unit_variant("FabricErrorCodes", 25u32, "FABRIC_E_DIRECTORY_NOT_FOUND"),
            Self::FabricEFabricVersionNotFound => {
                serializer.serialize_unit_variant("FabricErrorCodes", 26u32, "FABRIC_E_FABRIC_VERSION_NOT_FOUND")
            }
            Self::FabricEFileNotFound => serializer.serialize_unit_variant("FabricErrorCodes", 27u32, "FABRIC_E_FILE_NOT_FOUND"),
            Self::FabricENameDoesNotExist => serializer.serialize_unit_variant("FabricErrorCodes", 28u32, "FABRIC_E_NAME_DOES_NOT_EXIST"),
            Self::FabricEPropertyDoesNotExist => {
                serializer.serialize_unit_variant("FabricErrorCodes", 29u32, "FABRIC_E_PROPERTY_DOES_NOT_EXIST")
            }
            Self::FabricEEnumerationCompleted => {
                serializer.serialize_unit_variant("FabricErrorCodes", 30u32, "FABRIC_E_ENUMERATION_COMPLETED")
            }
            Self::FabricEServiceManifestNotFound => {
                serializer.serialize_unit_variant("FabricErrorCodes", 31u32, "FABRIC_E_SERVICE_MANIFEST_NOT_FOUND")
            }
            Self::FabricEKeyNotFound => serializer.serialize_unit_variant("FabricErrorCodes", 32u32, "FABRIC_E_KEY_NOT_FOUND"),
            Self::FabricEHealthEntityNotFound => {
                serializer.serialize_unit_variant("FabricErrorCodes", 33u32, "FABRIC_E_HEALTH_ENTITY_NOT_FOUND")
            }
            Self::FabricEApplicationTypeAlreadyExists => {
                serializer.serialize_unit_variant("FabricErrorCodes", 34u32, "FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS")
            }
            Self::FabricEApplicationAlreadyExists => {
                serializer.serialize_unit_variant("FabricErrorCodes", 35u32, "FABRIC_E_APPLICATION_ALREADY_EXISTS")
            }
            Self::FabricEApplicationAlreadyInTargetVersion => {
                serializer.serialize_unit_variant("FabricErrorCodes", 36u32, "FABRIC_E_APPLICATION_ALREADY_IN_TARGET_VERSION")
            }
            Self::FabricEApplicationTypeProvisionInProgress => {
                serializer.serialize_unit_variant("FabricErrorCodes", 37u32, "FABRIC_E_APPLICATION_TYPE_PROVISION_IN_PROGRESS")
            }
            Self::FabricEApplicationUpgradeInProgress => {
                serializer.serialize_unit_variant("FabricErrorCodes", 38u32, "FABRIC_E_APPLICATION_UPGRADE_IN_PROGRESS")
            }
            Self::FabricEServiceAlreadyExists => {
                serializer.serialize_unit_variant("FabricErrorCodes", 39u32, "FABRIC_E_SERVICE_ALREADY_EXISTS")
            }
            Self::FabricEServiceGroupAlreadyExists => {
                serializer.serialize_unit_variant("FabricErrorCodes", 40u32, "FABRIC_E_SERVICE_GROUP_ALREADY_EXISTS")
            }
            Self::FabricEApplicationTypeInUse => {
                serializer.serialize_unit_variant("FabricErrorCodes", 41u32, "FABRIC_E_APPLICATION_TYPE_IN_USE")
            }
            Self::FabricEFabricAlreadyInTargetVersion => {
                serializer.serialize_unit_variant("FabricErrorCodes", 42u32, "FABRIC_E_FABRIC_ALREADY_IN_TARGET_VERSION")
            }
            Self::FabricEFabricVersionAlreadyExists => {
                serializer.serialize_unit_variant("FabricErrorCodes", 43u32, "FABRIC_E_FABRIC_VERSION_ALREADY_EXISTS")
            }
            Self::FabricEFabricVersionInUse => {
                serializer.serialize_unit_variant("FabricErrorCodes", 44u32, "FABRIC_E_FABRIC_VERSION_IN_USE")
            }
            Self::FabricEFabricUpgradeInProgress => {
                serializer.serialize_unit_variant("FabricErrorCodes", 45u32, "FABRIC_E_FABRIC_UPGRADE_IN_PROGRESS")
            }
            Self::FabricENameAlreadyExists => serializer.serialize_unit_variant("FabricErrorCodes", 46u32, "FABRIC_E_NAME_ALREADY_EXISTS"),
            Self::FabricENameNotEmpty => serializer.serialize_unit_variant("FabricErrorCodes", 47u32, "FABRIC_E_NAME_NOT_EMPTY"),
            Self::FabricEPropertyCheckFailed => {
                serializer.serialize_unit_variant("FabricErrorCodes", 48u32, "FABRIC_E_PROPERTY_CHECK_FAILED")
            }
            Self::FabricEServiceMetadataMismatch => {
                serializer.serialize_unit_variant("FabricErrorCodes", 49u32, "FABRIC_E_SERVICE_METADATA_MISMATCH")
            }
            Self::FabricEServiceTypeMismatch => {
                serializer.serialize_unit_variant("FabricErrorCodes", 50u32, "FABRIC_E_SERVICE_TYPE_MISMATCH")
            }
            Self::FabricEHealthStaleReport => serializer.serialize_unit_variant("FabricErrorCodes", 51u32, "FABRIC_E_HEALTH_STALE_REPORT"),
            Self::FabricESequenceNumberCheckFailed => {
                serializer.serialize_unit_variant("FabricErrorCodes", 52u32, "FABRIC_E_SEQUENCE_NUMBER_CHECK_FAILED")
            }
            Self::FabricENodeHasNotStoppedYet => {
                serializer.serialize_unit_variant("FabricErrorCodes", 53u32, "FABRIC_E_NODE_HAS_NOT_STOPPED_YET")
            }
            Self::FabricEInstanceIdMismatch => {
                serializer.serialize_unit_variant("FabricErrorCodes", 54u32, "FABRIC_E_INSTANCE_ID_MISMATCH")
            }
            Self::FabricEValueTooLarge => serializer.serialize_unit_variant("FabricErrorCodes", 55u32, "FABRIC_E_VALUE_TOO_LARGE"),
            Self::FabricENoWriteQuorum => serializer.serialize_unit_variant("FabricErrorCodes", 56u32, "FABRIC_E_NO_WRITE_QUORUM"),
            Self::FabricENotPrimary => serializer.serialize_unit_variant("FabricErrorCodes", 57u32, "FABRIC_E_NOT_PRIMARY"),
            Self::FabricENotReady => serializer.serialize_unit_variant("FabricErrorCodes", 58u32, "FABRIC_E_NOT_READY"),
            Self::FabricEReconfigurationPending => {
                serializer.serialize_unit_variant("FabricErrorCodes", 59u32, "FABRIC_E_RECONFIGURATION_PENDING")
            }
            Self::FabricEServiceOffline => serializer.serialize_unit_variant("FabricErrorCodes", 60u32, "FABRIC_E_SERVICE_OFFLINE"),
            Self::EAbort => serializer.serialize_unit_variant("FabricErrorCodes", 61u32, "E_ABORT"),
            Self::FabricECommunicationError => serializer.serialize_unit_variant("FabricErrorCodes", 62u32, "FABRIC_E_COMMUNICATION_ERROR"),
            Self::FabricEOperationNotComplete => {
                serializer.serialize_unit_variant("FabricErrorCodes", 63u32, "FABRIC_E_OPERATION_NOT_COMPLETE")
            }
            Self::FabricETimeout => serializer.serialize_unit_variant("FabricErrorCodes", 64u32, "FABRIC_E_TIMEOUT"),
            Self::FabricENodeIsUp => serializer.serialize_unit_variant("FabricErrorCodes", 65u32, "FABRIC_E_NODE_IS_UP"),
            Self::EFail => serializer.serialize_unit_variant("FabricErrorCodes", 66u32, "E_FAIL"),
            Self::FabricEBackupIsEnabled => serializer.serialize_unit_variant("FabricErrorCodes", 67u32, "FABRIC_E_BACKUP_IS_ENABLED"),
            Self::FabricERestoreSourceTargetPartitionMismatch => {
                serializer.serialize_unit_variant("FabricErrorCodes", 68u32, "FABRIC_E_RESTORE_SOURCE_TARGET_PARTITION_MISMATCH")
            }
            Self::FabricEInvalidForStatelessServices => {
                serializer.serialize_unit_variant("FabricErrorCodes", 69u32, "FABRIC_E_INVALID_FOR_STATELESS_SERVICES")
            }
            Self::FabricEBackupNotEnabled => serializer.serialize_unit_variant("FabricErrorCodes", 70u32, "FABRIC_E_BACKUP_NOT_ENABLED"),
            Self::FabricEBackupPolicyNotExisting => {
                serializer.serialize_unit_variant("FabricErrorCodes", 71u32, "FABRIC_E_BACKUP_POLICY_NOT_EXISTING")
            }
            Self::FabricEFaultAnalysisServiceNotExisting => {
                serializer.serialize_unit_variant("FabricErrorCodes", 72u32, "FABRIC_E_FAULT_ANALYSIS_SERVICE_NOT_EXISTING")
            }
            Self::FabricEBackupInProgress => serializer.serialize_unit_variant("FabricErrorCodes", 73u32, "FABRIC_E_BACKUP_IN_PROGRESS"),
            Self::FabricERestoreInProgress => serializer.serialize_unit_variant("FabricErrorCodes", 74u32, "FABRIC_E_RESTORE_IN_PROGRESS"),
            Self::FabricEBackupPolicyAlreadyExisting => {
                serializer.serialize_unit_variant("FabricErrorCodes", 75u32, "FABRIC_E_BACKUP_POLICY_ALREADY_EXISTING")
            }
            Self::FabricEInvalidServiceScalingPolicy => {
                serializer.serialize_unit_variant("FabricErrorCodes", 76u32, "FABRIC_E_INVALID_SERVICE_SCALING_POLICY")
            }
            Self::EInvalidarg => serializer.serialize_unit_variant("FabricErrorCodes", 77u32, "E_INVALIDARG"),
            Self::FabricESingleInstanceApplicationAlreadyExists => {
                serializer.serialize_unit_variant("FabricErrorCodes", 78u32, "FABRIC_E_SINGLE_INSTANCE_APPLICATION_ALREADY_EXISTS")
            }
            Self::FabricESingleInstanceApplicationNotFound => {
                serializer.serialize_unit_variant("FabricErrorCodes", 79u32, "FABRIC_E_SINGLE_INSTANCE_APPLICATION_NOT_FOUND")
            }
            Self::FabricEVolumeAlreadyExists => {
                serializer.serialize_unit_variant("FabricErrorCodes", 80u32, "FABRIC_E_VOLUME_ALREADY_EXISTS")
            }
            Self::FabricEVolumeNotFound => serializer.serialize_unit_variant("FabricErrorCodes", 81u32, "FABRIC_E_VOLUME_NOT_FOUND"),
            Self::SerializationError => serializer.serialize_unit_variant("FabricErrorCodes", 82u32, "SerializationError"),
            Self::FabricEImagebuilderReservedDirectoryError => {
                serializer.serialize_unit_variant("FabricErrorCodes", 83u32, "FABRIC_E_IMAGEBUILDER_RESERVED_DIRECTORY_ERROR")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Error object containing error code and error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricErrorError {
    #[doc = "Defines the fabric error codes that be returned as part of the error object in response to Service Fabric API operations that are not successful. Following are the error code values that can be returned for a specific HTTP status code.\n\n  - Possible values of the error code for HTTP status code 400 (Bad Request)\n    - \"FABRIC_E_INVALID_PARTITION_KEY\"\n    - \"FABRIC_E_IMAGEBUILDER_VALIDATION_ERROR\"\n    - \"FABRIC_E_INVALID_ADDRESS\"\n    - \"FABRIC_E_APPLICATION_NOT_UPGRADING\"\n    - \"FABRIC_E_APPLICATION_UPGRADE_VALIDATION_ERROR\"\n    - \"FABRIC_E_FABRIC_NOT_UPGRADING\"\n    - \"FABRIC_E_FABRIC_UPGRADE_VALIDATION_ERROR\"\n    - \"FABRIC_E_INVALID_CONFIGURATION\"\n    - \"FABRIC_E_INVALID_NAME_URI\"\n    - \"FABRIC_E_PATH_TOO_LONG\"\n    - \"FABRIC_E_KEY_TOO_LARGE\"\n    - \"FABRIC_E_SERVICE_AFFINITY_CHAIN_NOT_SUPPORTED\"\n    - \"FABRIC_E_INVALID_ATOMIC_GROUP\"\n    - \"FABRIC_E_VALUE_EMPTY\"\n    - \"FABRIC_E_BACKUP_IS_ENABLED\"\n    - \"FABRIC_E_RESTORE_SOURCE_TARGET_PARTITION_MISMATCH\"\n    - \"FABRIC_E_INVALID_FOR_STATELESS_SERVICES\"\n    - \"FABRIC_E_INVALID_SERVICE_SCALING_POLICY\"\n    - \"E_INVALIDARG\"\n\n  - Possible values of the error code for HTTP status code 404 (Not Found)\n    - \"FABRIC_E_NODE_NOT_FOUND\"\n    - \"FABRIC_E_APPLICATION_TYPE_NOT_FOUND\"\n    - \"FABRIC_E_APPLICATION_NOT_FOUND\"\n    - \"FABRIC_E_SERVICE_TYPE_NOT_FOUND\"\n    - \"FABRIC_E_SERVICE_DOES_NOT_EXIST\"\n    - \"FABRIC_E_SERVICE_TYPE_TEMPLATE_NOT_FOUND\"\n    - \"FABRIC_E_CONFIGURATION_SECTION_NOT_FOUND\"\n    - \"FABRIC_E_PARTITION_NOT_FOUND\"\n    - \"FABRIC_E_REPLICA_DOES_NOT_EXIST\"\n    - \"FABRIC_E_SERVICE_GROUP_DOES_NOT_EXIST\"\n    - \"FABRIC_E_CONFIGURATION_PARAMETER_NOT_FOUND\"\n    - \"FABRIC_E_DIRECTORY_NOT_FOUND\"\n    - \"FABRIC_E_FABRIC_VERSION_NOT_FOUND\"\n    - \"FABRIC_E_FILE_NOT_FOUND\"\n    - \"FABRIC_E_NAME_DOES_NOT_EXIST\"\n    - \"FABRIC_E_PROPERTY_DOES_NOT_EXIST\"\n    - \"FABRIC_E_ENUMERATION_COMPLETED\"\n    - \"FABRIC_E_SERVICE_MANIFEST_NOT_FOUND\"\n    - \"FABRIC_E_KEY_NOT_FOUND\"\n    - \"FABRIC_E_HEALTH_ENTITY_NOT_FOUND\"\n    - \"FABRIC_E_BACKUP_NOT_ENABLED\"\n    - \"FABRIC_E_BACKUP_POLICY_NOT_EXISTING\"\n    - \"FABRIC_E_FAULT_ANALYSIS_SERVICE_NOT_EXISTING\"\n    - \"FABRIC_E_IMAGEBUILDER_RESERVED_DIRECTORY_ERROR\"\n\n  - Possible values of the error code for HTTP status code 409 (Conflict)\n    - \"FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS\"\n    - \"FABRIC_E_APPLICATION_ALREADY_EXISTS\"\n    - \"FABRIC_E_APPLICATION_ALREADY_IN_TARGET_VERSION\"\n    - \"FABRIC_E_APPLICATION_TYPE_PROVISION_IN_PROGRESS\"\n    - \"FABRIC_E_APPLICATION_UPGRADE_IN_PROGRESS\"\n    - \"FABRIC_E_SERVICE_ALREADY_EXISTS\"\n    - \"FABRIC_E_SERVICE_GROUP_ALREADY_EXISTS\"\n    - \"FABRIC_E_APPLICATION_TYPE_IN_USE\"\n    - \"FABRIC_E_FABRIC_ALREADY_IN_TARGET_VERSION\"\n    - \"FABRIC_E_FABRIC_VERSION_ALREADY_EXISTS\"\n    - \"FABRIC_E_FABRIC_VERSION_IN_USE\"\n    - \"FABRIC_E_FABRIC_UPGRADE_IN_PROGRESS\"\n    - \"FABRIC_E_NAME_ALREADY_EXISTS\"\n    - \"FABRIC_E_NAME_NOT_EMPTY\"\n    - \"FABRIC_E_PROPERTY_CHECK_FAILED\"\n    - \"FABRIC_E_SERVICE_METADATA_MISMATCH\"\n    - \"FABRIC_E_SERVICE_TYPE_MISMATCH\"\n    - \"FABRIC_E_HEALTH_STALE_REPORT\"\n    - \"FABRIC_E_SEQUENCE_NUMBER_CHECK_FAILED\"\n    - \"FABRIC_E_NODE_HAS_NOT_STOPPED_YET\"\n    - \"FABRIC_E_INSTANCE_ID_MISMATCH\"\n    - \"FABRIC_E_BACKUP_IN_PROGRESS\"\n    - \"FABRIC_E_RESTORE_IN_PROGRESS\"\n    - \"FABRIC_E_BACKUP_POLICY_ALREADY_EXISTING\"\n\n  - Possible values of the error code for HTTP status code 413 (Request Entity Too Large)\n    - \"FABRIC_E_VALUE_TOO_LARGE\"\n\n  - Possible values of the error code for HTTP status code 500 (Internal Server Error)\n    - \"FABRIC_E_NODE_IS_UP\"\n    - \"E_FAIL\"\n    - \"FABRIC_E_SINGLE_INSTANCE_APPLICATION_ALREADY_EXISTS\"\n    - \"FABRIC_E_SINGLE_INSTANCE_APPLICATION_NOT_FOUND\"\n    - \"FABRIC_E_VOLUME_ALREADY_EXISTS\"\n    - \"FABRIC_E_VOLUME_NOT_FOUND\"\n    - \"SerializationError\"\n\n  - Possible values of the error code for HTTP status code 503 (Service Unavailable)\n    - \"FABRIC_E_NO_WRITE_QUORUM\"\n    - \"FABRIC_E_NOT_PRIMARY\"\n    - \"FABRIC_E_NOT_READY\"\n    - \"FABRIC_E_RECONFIGURATION_PENDING\"\n    - \"FABRIC_E_SERVICE_OFFLINE\"\n    - \"E_ABORT\"\n    - \"FABRIC_E_VALUE_TOO_LARGE\"\n\n  - Possible values of the error code for HTTP status code 504 (Gateway Timeout)\n    - \"FABRIC_E_COMMUNICATION_ERROR\"\n    - \"FABRIC_E_OPERATION_NOT_COMPLETE\"\n    - \"FABRIC_E_TIMEOUT\""]
    #[serde(rename = "Code")]
    pub code: FabricErrorCodes,
    #[doc = "Error message."]
    #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl FabricErrorError {
    pub fn new(code: FabricErrorCodes) -> Self {
        Self { code, message: None }
    }
}
#[doc = "Represents the base for all Fabric Events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricEvent {
    #[doc = "The kind of FabricEvent."]
    #[serde(rename = "Kind")]
    pub kind: FabricEventKind,
    #[doc = "The identifier for the FabricEvent instance."]
    #[serde(rename = "EventInstanceId")]
    pub event_instance_id: String,
    #[doc = "The category of event."]
    #[serde(rename = "Category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The time event was logged."]
    #[serde(rename = "TimeStamp", with = "azure_core::date::rfc3339")]
    pub time_stamp: time::OffsetDateTime,
    #[doc = "Shows there is existing related events available."]
    #[serde(rename = "HasCorrelatedEvents", default, skip_serializing_if = "Option::is_none")]
    pub has_correlated_events: Option<bool>,
}
impl FabricEvent {
    pub fn new(kind: FabricEventKind, event_instance_id: String, time_stamp: time::OffsetDateTime) -> Self {
        Self {
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
        }
    }
}
#[doc = "The kind of FabricEvent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FabricEventKind")]
pub enum FabricEventKind {
    ClusterEvent,
    ContainerInstanceEvent,
    NodeEvent,
    ApplicationEvent,
    ServiceEvent,
    PartitionEvent,
    ReplicaEvent,
    PartitionAnalysisEvent,
    ApplicationCreated,
    ApplicationDeleted,
    ApplicationNewHealthReport,
    ApplicationHealthReportExpired,
    ApplicationUpgradeCompleted,
    ApplicationUpgradeDomainCompleted,
    ApplicationUpgradeRollbackCompleted,
    ApplicationUpgradeRollbackStarted,
    ApplicationUpgradeStarted,
    DeployedApplicationNewHealthReport,
    DeployedApplicationHealthReportExpired,
    ApplicationProcessExited,
    ApplicationContainerInstanceExited,
    NodeAborted,
    NodeAddedToCluster,
    NodeClosed,
    NodeDeactivateCompleted,
    NodeDeactivateStarted,
    NodeDown,
    NodeNewHealthReport,
    NodeHealthReportExpired,
    NodeOpenSucceeded,
    NodeOpenFailed,
    NodeRemovedFromCluster,
    NodeUp,
    PartitionNewHealthReport,
    PartitionHealthReportExpired,
    PartitionReconfigured,
    PartitionPrimaryMoveAnalysis,
    ServiceCreated,
    ServiceDeleted,
    ServiceNewHealthReport,
    ServiceHealthReportExpired,
    DeployedServicePackageNewHealthReport,
    DeployedServicePackageHealthReportExpired,
    StatefulReplicaNewHealthReport,
    StatefulReplicaHealthReportExpired,
    StatelessReplicaNewHealthReport,
    StatelessReplicaHealthReportExpired,
    ClusterNewHealthReport,
    ClusterHealthReportExpired,
    ClusterUpgradeCompleted,
    ClusterUpgradeDomainCompleted,
    ClusterUpgradeRollbackCompleted,
    ClusterUpgradeRollbackStarted,
    ClusterUpgradeStarted,
    ChaosStopped,
    ChaosStarted,
    ChaosCodePackageRestartScheduled,
    ChaosReplicaRemovalScheduled,
    ChaosPartitionSecondaryMoveScheduled,
    ChaosPartitionPrimaryMoveScheduled,
    ChaosReplicaRestartScheduled,
    ChaosNodeRestartScheduled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FabricEventKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FabricEventKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FabricEventKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ClusterEvent => serializer.serialize_unit_variant("FabricEventKind", 0u32, "ClusterEvent"),
            Self::ContainerInstanceEvent => serializer.serialize_unit_variant("FabricEventKind", 1u32, "ContainerInstanceEvent"),
            Self::NodeEvent => serializer.serialize_unit_variant("FabricEventKind", 2u32, "NodeEvent"),
            Self::ApplicationEvent => serializer.serialize_unit_variant("FabricEventKind", 3u32, "ApplicationEvent"),
            Self::ServiceEvent => serializer.serialize_unit_variant("FabricEventKind", 4u32, "ServiceEvent"),
            Self::PartitionEvent => serializer.serialize_unit_variant("FabricEventKind", 5u32, "PartitionEvent"),
            Self::ReplicaEvent => serializer.serialize_unit_variant("FabricEventKind", 6u32, "ReplicaEvent"),
            Self::PartitionAnalysisEvent => serializer.serialize_unit_variant("FabricEventKind", 7u32, "PartitionAnalysisEvent"),
            Self::ApplicationCreated => serializer.serialize_unit_variant("FabricEventKind", 8u32, "ApplicationCreated"),
            Self::ApplicationDeleted => serializer.serialize_unit_variant("FabricEventKind", 9u32, "ApplicationDeleted"),
            Self::ApplicationNewHealthReport => serializer.serialize_unit_variant("FabricEventKind", 10u32, "ApplicationNewHealthReport"),
            Self::ApplicationHealthReportExpired => {
                serializer.serialize_unit_variant("FabricEventKind", 11u32, "ApplicationHealthReportExpired")
            }
            Self::ApplicationUpgradeCompleted => serializer.serialize_unit_variant("FabricEventKind", 12u32, "ApplicationUpgradeCompleted"),
            Self::ApplicationUpgradeDomainCompleted => {
                serializer.serialize_unit_variant("FabricEventKind", 13u32, "ApplicationUpgradeDomainCompleted")
            }
            Self::ApplicationUpgradeRollbackCompleted => {
                serializer.serialize_unit_variant("FabricEventKind", 14u32, "ApplicationUpgradeRollbackCompleted")
            }
            Self::ApplicationUpgradeRollbackStarted => {
                serializer.serialize_unit_variant("FabricEventKind", 15u32, "ApplicationUpgradeRollbackStarted")
            }
            Self::ApplicationUpgradeStarted => serializer.serialize_unit_variant("FabricEventKind", 16u32, "ApplicationUpgradeStarted"),
            Self::DeployedApplicationNewHealthReport => {
                serializer.serialize_unit_variant("FabricEventKind", 17u32, "DeployedApplicationNewHealthReport")
            }
            Self::DeployedApplicationHealthReportExpired => {
                serializer.serialize_unit_variant("FabricEventKind", 18u32, "DeployedApplicationHealthReportExpired")
            }
            Self::ApplicationProcessExited => serializer.serialize_unit_variant("FabricEventKind", 19u32, "ApplicationProcessExited"),
            Self::ApplicationContainerInstanceExited => {
                serializer.serialize_unit_variant("FabricEventKind", 20u32, "ApplicationContainerInstanceExited")
            }
            Self::NodeAborted => serializer.serialize_unit_variant("FabricEventKind", 21u32, "NodeAborted"),
            Self::NodeAddedToCluster => serializer.serialize_unit_variant("FabricEventKind", 22u32, "NodeAddedToCluster"),
            Self::NodeClosed => serializer.serialize_unit_variant("FabricEventKind", 23u32, "NodeClosed"),
            Self::NodeDeactivateCompleted => serializer.serialize_unit_variant("FabricEventKind", 24u32, "NodeDeactivateCompleted"),
            Self::NodeDeactivateStarted => serializer.serialize_unit_variant("FabricEventKind", 25u32, "NodeDeactivateStarted"),
            Self::NodeDown => serializer.serialize_unit_variant("FabricEventKind", 26u32, "NodeDown"),
            Self::NodeNewHealthReport => serializer.serialize_unit_variant("FabricEventKind", 27u32, "NodeNewHealthReport"),
            Self::NodeHealthReportExpired => serializer.serialize_unit_variant("FabricEventKind", 28u32, "NodeHealthReportExpired"),
            Self::NodeOpenSucceeded => serializer.serialize_unit_variant("FabricEventKind", 29u32, "NodeOpenSucceeded"),
            Self::NodeOpenFailed => serializer.serialize_unit_variant("FabricEventKind", 30u32, "NodeOpenFailed"),
            Self::NodeRemovedFromCluster => serializer.serialize_unit_variant("FabricEventKind", 31u32, "NodeRemovedFromCluster"),
            Self::NodeUp => serializer.serialize_unit_variant("FabricEventKind", 32u32, "NodeUp"),
            Self::PartitionNewHealthReport => serializer.serialize_unit_variant("FabricEventKind", 33u32, "PartitionNewHealthReport"),
            Self::PartitionHealthReportExpired => {
                serializer.serialize_unit_variant("FabricEventKind", 34u32, "PartitionHealthReportExpired")
            }
            Self::PartitionReconfigured => serializer.serialize_unit_variant("FabricEventKind", 35u32, "PartitionReconfigured"),
            Self::PartitionPrimaryMoveAnalysis => {
                serializer.serialize_unit_variant("FabricEventKind", 36u32, "PartitionPrimaryMoveAnalysis")
            }
            Self::ServiceCreated => serializer.serialize_unit_variant("FabricEventKind", 37u32, "ServiceCreated"),
            Self::ServiceDeleted => serializer.serialize_unit_variant("FabricEventKind", 38u32, "ServiceDeleted"),
            Self::ServiceNewHealthReport => serializer.serialize_unit_variant("FabricEventKind", 39u32, "ServiceNewHealthReport"),
            Self::ServiceHealthReportExpired => serializer.serialize_unit_variant("FabricEventKind", 40u32, "ServiceHealthReportExpired"),
            Self::DeployedServicePackageNewHealthReport => {
                serializer.serialize_unit_variant("FabricEventKind", 41u32, "DeployedServicePackageNewHealthReport")
            }
            Self::DeployedServicePackageHealthReportExpired => {
                serializer.serialize_unit_variant("FabricEventKind", 42u32, "DeployedServicePackageHealthReportExpired")
            }
            Self::StatefulReplicaNewHealthReport => {
                serializer.serialize_unit_variant("FabricEventKind", 43u32, "StatefulReplicaNewHealthReport")
            }
            Self::StatefulReplicaHealthReportExpired => {
                serializer.serialize_unit_variant("FabricEventKind", 44u32, "StatefulReplicaHealthReportExpired")
            }
            Self::StatelessReplicaNewHealthReport => {
                serializer.serialize_unit_variant("FabricEventKind", 45u32, "StatelessReplicaNewHealthReport")
            }
            Self::StatelessReplicaHealthReportExpired => {
                serializer.serialize_unit_variant("FabricEventKind", 46u32, "StatelessReplicaHealthReportExpired")
            }
            Self::ClusterNewHealthReport => serializer.serialize_unit_variant("FabricEventKind", 47u32, "ClusterNewHealthReport"),
            Self::ClusterHealthReportExpired => serializer.serialize_unit_variant("FabricEventKind", 48u32, "ClusterHealthReportExpired"),
            Self::ClusterUpgradeCompleted => serializer.serialize_unit_variant("FabricEventKind", 49u32, "ClusterUpgradeCompleted"),
            Self::ClusterUpgradeDomainCompleted => {
                serializer.serialize_unit_variant("FabricEventKind", 50u32, "ClusterUpgradeDomainCompleted")
            }
            Self::ClusterUpgradeRollbackCompleted => {
                serializer.serialize_unit_variant("FabricEventKind", 51u32, "ClusterUpgradeRollbackCompleted")
            }
            Self::ClusterUpgradeRollbackStarted => {
                serializer.serialize_unit_variant("FabricEventKind", 52u32, "ClusterUpgradeRollbackStarted")
            }
            Self::ClusterUpgradeStarted => serializer.serialize_unit_variant("FabricEventKind", 53u32, "ClusterUpgradeStarted"),
            Self::ChaosStopped => serializer.serialize_unit_variant("FabricEventKind", 54u32, "ChaosStopped"),
            Self::ChaosStarted => serializer.serialize_unit_variant("FabricEventKind", 55u32, "ChaosStarted"),
            Self::ChaosCodePackageRestartScheduled => {
                serializer.serialize_unit_variant("FabricEventKind", 56u32, "ChaosCodePackageRestartScheduled")
            }
            Self::ChaosReplicaRemovalScheduled => {
                serializer.serialize_unit_variant("FabricEventKind", 57u32, "ChaosReplicaRemovalScheduled")
            }
            Self::ChaosPartitionSecondaryMoveScheduled => {
                serializer.serialize_unit_variant("FabricEventKind", 58u32, "ChaosPartitionSecondaryMoveScheduled")
            }
            Self::ChaosPartitionPrimaryMoveScheduled => {
                serializer.serialize_unit_variant("FabricEventKind", 59u32, "ChaosPartitionPrimaryMoveScheduled")
            }
            Self::ChaosReplicaRestartScheduled => {
                serializer.serialize_unit_variant("FabricEventKind", 60u32, "ChaosReplicaRestartScheduled")
            }
            Self::ChaosNodeRestartScheduled => serializer.serialize_unit_variant("FabricEventKind", 61u32, "ChaosNodeRestartScheduled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type FabricName = String;
#[doc = "Specifies the status of the replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FabricReplicaStatus")]
pub enum FabricReplicaStatus {
    Invalid,
    Down,
    Up,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FabricReplicaStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FabricReplicaStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FabricReplicaStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("FabricReplicaStatus", 0u32, "Invalid"),
            Self::Down => serializer.serialize_unit_variant("FabricReplicaStatus", 1u32, "Down"),
            Self::Up => serializer.serialize_unit_variant("FabricReplicaStatus", 2u32, "Up"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Derived from PropertyBatchInfo. Represents the property batch failing. Contains information about the specific batch failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedPropertyBatchInfo {
    #[serde(flatten)]
    pub property_batch_info: PropertyBatchInfo,
    #[doc = "The error message of the failed operation. Describes the exception thrown due to the first unsuccessful operation in the property batch."]
    #[serde(rename = "ErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The index of the unsuccessful operation in the property batch."]
    #[serde(rename = "OperationIndex", default, skip_serializing_if = "Option::is_none")]
    pub operation_index: Option<i64>,
}
impl FailedPropertyBatchInfo {
    pub fn new(property_batch_info: PropertyBatchInfo) -> Self {
        Self {
            property_batch_info,
            error_message: None,
            operation_index: None,
        }
    }
}
#[doc = "The detailed upgrade progress for nodes in the current upgrade domain at the point of failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailedUpgradeDomainProgressObject {
    #[doc = "The name of the upgrade domain"]
    #[serde(rename = "DomainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<UpgradeDomainName>,
    #[doc = "List of upgrading nodes and their statuses"]
    #[serde(rename = "NodeUpgradeProgressList", default, skip_serializing_if = "Option::is_none")]
    pub node_upgrade_progress_list: Option<NodeUpgradeProgressInfoList>,
}
impl FailedUpgradeDomainProgressObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The compensating action to perform when a Monitored upgrade encounters monitoring policy or health policy violations.\nInvalid indicates the failure action is invalid. Rollback specifies that the upgrade will start rolling back automatically.\nManual indicates that the upgrade will switch to UnmonitoredManual upgrade mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FailureAction")]
pub enum FailureAction {
    Invalid,
    Rollback,
    Manual,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FailureAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FailureAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FailureAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("FailureAction", 0u32, "Invalid"),
            Self::Rollback => serializer.serialize_unit_variant("FailureAction", 1u32, "Rollback"),
            Self::Manual => serializer.serialize_unit_variant("FailureAction", 2u32, "Manual"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The cause of an upgrade failure that resulted in FailureAction being executed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FailureReason")]
pub enum FailureReason {
    None,
    Interrupted,
    HealthCheck,
    UpgradeDomainTimeout,
    OverallUpgradeTimeout,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FailureReason {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FailureReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FailureReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("FailureReason", 0u32, "None"),
            Self::Interrupted => serializer.serialize_unit_variant("FailureReason", 1u32, "Interrupted"),
            Self::HealthCheck => serializer.serialize_unit_variant("FailureReason", 2u32, "HealthCheck"),
            Self::UpgradeDomainTimeout => serializer.serialize_unit_variant("FailureReason", 3u32, "UpgradeDomainTimeout"),
            Self::OverallUpgradeTimeout => serializer.serialize_unit_variant("FailureReason", 4u32, "OverallUpgradeTimeout"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information about the upgrade domain progress at the time of upgrade failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailureUpgradeDomainProgressInfo {
    #[doc = "The name of the upgrade domain"]
    #[serde(rename = "DomainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<UpgradeDomainName>,
    #[doc = "List of upgrading nodes and their statuses"]
    #[serde(rename = "NodeUpgradeProgressList", default, skip_serializing_if = "Option::is_none")]
    pub node_upgrade_progress_list: Option<NodeUpgradeProgressInfoList>,
}
impl FailureUpgradeDomainProgressInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a image store file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileInfo {
    #[doc = "The size of file in bytes."]
    #[serde(rename = "FileSize", default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    #[doc = "Information about the version of image store file."]
    #[serde(rename = "FileVersion", default, skip_serializing_if = "Option::is_none")]
    pub file_version: Option<FileVersion>,
    #[doc = "The date and time when the image store file was last modified."]
    #[serde(rename = "ModifiedDate", with = "azure_core::date::rfc3339::option")]
    pub modified_date: Option<time::OffsetDateTime>,
    #[doc = "The file path relative to the image store root path."]
    #[serde(rename = "StoreRelativePath", default, skip_serializing_if = "Option::is_none")]
    pub store_relative_path: Option<String>,
}
impl FileInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the parameters for file share storage used for storing or enumerating backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileShareBackupStorageDescription {
    #[serde(flatten)]
    pub backup_storage_description: BackupStorageDescription,
    #[doc = "UNC path of the file share where to store or enumerate backups from."]
    #[serde(rename = "Path")]
    pub path: String,
    #[doc = "Primary user name to access the file share."]
    #[serde(rename = "PrimaryUserName", default, skip_serializing_if = "Option::is_none")]
    pub primary_user_name: Option<String>,
    #[doc = "Primary password to access the share location."]
    #[serde(rename = "PrimaryPassword", default, skip_serializing_if = "Option::is_none")]
    pub primary_password: Option<String>,
    #[doc = "Secondary user name to access the file share."]
    #[serde(rename = "SecondaryUserName", default, skip_serializing_if = "Option::is_none")]
    pub secondary_user_name: Option<String>,
    #[doc = "Secondary password to access the share location"]
    #[serde(rename = "SecondaryPassword", default, skip_serializing_if = "Option::is_none")]
    pub secondary_password: Option<String>,
}
impl FileShareBackupStorageDescription {
    pub fn new(backup_storage_description: BackupStorageDescription, path: String) -> Self {
        Self {
            backup_storage_description,
            path,
            primary_user_name: None,
            primary_password: None,
            secondary_user_name: None,
            secondary_password: None,
        }
    }
}
#[doc = "Information about the version of image store file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileVersion {
    #[doc = "The current image store version number for the file is used in image store for checking whether it need to be updated."]
    #[serde(rename = "VersionNumber", default, skip_serializing_if = "Option::is_none")]
    pub version_number: Option<String>,
    #[doc = "The epoch data loss number of image store replica when this file entry was updated or created."]
    #[serde(rename = "EpochDataLossNumber", default, skip_serializing_if = "Option::is_none")]
    pub epoch_data_loss_number: Option<String>,
    #[doc = "The epoch configuration version number of the image store replica when this file entry was created or updated."]
    #[serde(rename = "EpochConfigurationNumber", default, skip_serializing_if = "Option::is_none")]
    pub epoch_configuration_number: Option<String>,
}
impl FileVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a image store folder. It includes how many files this folder contains and its image store relative path."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FolderInfo {
    #[doc = "The remote location within image store. This path is relative to the image store root."]
    #[serde(rename = "StoreRelativePath", default, skip_serializing_if = "Option::is_none")]
    pub store_relative_path: Option<ImageStoreRelativePath>,
    #[doc = "The number of files from within the image store folder."]
    #[serde(rename = "FileCount", default, skip_serializing_if = "Option::is_none")]
    pub file_count: Option<String>,
}
impl FolderInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information of a image store folder size"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FolderSizeInfo {
    #[doc = "The remote location within image store. This path is relative to the image store root."]
    #[serde(rename = "StoreRelativePath", default, skip_serializing_if = "Option::is_none")]
    pub store_relative_path: Option<ImageStoreRelativePath>,
    #[doc = "The size of folder in bytes."]
    #[serde(rename = "FolderSize", default, skip_serializing_if = "Option::is_none")]
    pub folder_size: Option<String>,
}
impl FolderSizeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ForceRestart = bool;
#[doc = "Describes the frequency based backup schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrequencyBasedBackupScheduleDescription {
    #[serde(flatten)]
    pub backup_schedule_description: BackupScheduleDescription,
    #[doc = "Defines the interval with which backups are periodically taken. It should be specified in ISO8601 format. Timespan in seconds is not supported and will be ignored while creating the policy."]
    #[serde(rename = "Interval")]
    pub interval: String,
}
impl FrequencyBasedBackupScheduleDescription {
    pub fn new(backup_schedule_description: BackupScheduleDescription, interval: String) -> Self {
        Self {
            backup_schedule_description,
            interval,
        }
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
    #[doc = "Name of the Gateway resource."]
    pub name: GatewayResourceName,
    #[doc = "Describes properties of a gateway resource."]
    pub properties: GatewayProperties,
}
impl GatewayResourceDescription {
    pub fn new(name: GatewayResourceName, properties: GatewayProperties) -> Self {
        Self { name, properties }
    }
}
pub type GatewayResourceName = String;
#[doc = "Describes additional filters to be applied, while listing backups, and backup storage details from where to fetch the backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBackupByStorageQueryDescription {
    #[doc = "Specifies the start date time in ISO8601 from which to enumerate backups. If not specified, backups are enumerated from the beginning."]
    #[serde(rename = "StartDateTimeFilter", with = "azure_core::date::rfc3339::option")]
    pub start_date_time_filter: Option<time::OffsetDateTime>,
    #[doc = "Specifies the end date time in ISO8601 till which to enumerate backups. If not specified, backups are enumerated till the end."]
    #[serde(rename = "EndDateTimeFilter", with = "azure_core::date::rfc3339::option")]
    pub end_date_time_filter: Option<time::OffsetDateTime>,
    #[doc = "If specified as true, gets the most recent backup (within the specified time range) for every partition under the specified backup entity."]
    #[serde(rename = "Latest", default, skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
    #[doc = "Describes the parameters for the backup storage."]
    #[serde(rename = "Storage")]
    pub storage: BackupStorageDescription,
    #[doc = "Describes the Service Fabric entity that is configured for backup."]
    #[serde(rename = "BackupEntity")]
    pub backup_entity: BackupEntity,
}
impl GetBackupByStorageQueryDescription {
    pub fn new(storage: BackupStorageDescription, backup_entity: BackupEntity) -> Self {
        Self {
            start_date_time_filter: None,
            end_date_time_filter: None,
            latest: None,
            storage,
            backup_entity,
        }
    }
}
#[doc = "Represents a PropertyBatchOperation that gets the specified property if it exists.\nNote that if one PropertyBatchOperation in a PropertyBatch fails,\nthe entire batch fails and cannot be committed in a transactional manner."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPropertyBatchOperation {
    #[serde(flatten)]
    pub property_batch_operation: PropertyBatchOperation,
    #[doc = "Whether or not to return the property value with the metadata.\nTrue if values should be returned with the metadata; False to return only property metadata."]
    #[serde(rename = "IncludeValue", default, skip_serializing_if = "Option::is_none")]
    pub include_value: Option<bool>,
}
impl GetPropertyBatchOperation {
    pub fn new(property_batch_operation: PropertyBatchOperation) -> Self {
        Self {
            property_batch_operation,
            include_value: None,
        }
    }
}
#[doc = "Describes a Service Fabric property value of type Guid."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuidPropertyValue {
    #[serde(flatten)]
    pub property_value: PropertyValue,
    #[doc = "The data of the property value."]
    #[serde(rename = "Data")]
    pub data: String,
}
impl GuidPropertyValue {
    pub fn new(property_value: PropertyValue, data: String) -> Self {
        Self { property_value, data }
    }
}
pub type HealthCheckRetryTimeout = String;
pub type HealthCheckStableDuration = String;
pub type HealthCheckWaitDuration = String;
#[doc = "Represents a health evaluation which describes the data and the algorithm used by health manager to evaluate the health of an entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthEvaluation {
    #[doc = "The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values."]
    #[serde(rename = "Kind")]
    pub kind: HealthEvaluationKind,
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "AggregatedHealthState", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_health_state: Option<HealthState>,
    #[doc = "Description of the health evaluation, which represents a summary of the evaluation process."]
    #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl HealthEvaluation {
    pub fn new(kind: HealthEvaluationKind) -> Self {
        Self {
            kind,
            aggregated_health_state: None,
            description: None,
        }
    }
}
#[doc = "The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthEvaluationKind")]
pub enum HealthEvaluationKind {
    Invalid,
    Event,
    Replicas,
    Partitions,
    DeployedServicePackages,
    DeployedApplications,
    Services,
    Nodes,
    Applications,
    SystemApplication,
    UpgradeDomainDeployedApplications,
    UpgradeDomainNodes,
    Replica,
    Partition,
    DeployedServicePackage,
    DeployedApplication,
    Service,
    Node,
    Application,
    DeltaNodesCheck,
    UpgradeDomainDeltaNodesCheck,
    ApplicationTypeApplications,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthEvaluationKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthEvaluationKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthEvaluationKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("HealthEvaluationKind", 0u32, "Invalid"),
            Self::Event => serializer.serialize_unit_variant("HealthEvaluationKind", 1u32, "Event"),
            Self::Replicas => serializer.serialize_unit_variant("HealthEvaluationKind", 2u32, "Replicas"),
            Self::Partitions => serializer.serialize_unit_variant("HealthEvaluationKind", 3u32, "Partitions"),
            Self::DeployedServicePackages => serializer.serialize_unit_variant("HealthEvaluationKind", 4u32, "DeployedServicePackages"),
            Self::DeployedApplications => serializer.serialize_unit_variant("HealthEvaluationKind", 5u32, "DeployedApplications"),
            Self::Services => serializer.serialize_unit_variant("HealthEvaluationKind", 6u32, "Services"),
            Self::Nodes => serializer.serialize_unit_variant("HealthEvaluationKind", 7u32, "Nodes"),
            Self::Applications => serializer.serialize_unit_variant("HealthEvaluationKind", 8u32, "Applications"),
            Self::SystemApplication => serializer.serialize_unit_variant("HealthEvaluationKind", 9u32, "SystemApplication"),
            Self::UpgradeDomainDeployedApplications => {
                serializer.serialize_unit_variant("HealthEvaluationKind", 10u32, "UpgradeDomainDeployedApplications")
            }
            Self::UpgradeDomainNodes => serializer.serialize_unit_variant("HealthEvaluationKind", 11u32, "UpgradeDomainNodes"),
            Self::Replica => serializer.serialize_unit_variant("HealthEvaluationKind", 12u32, "Replica"),
            Self::Partition => serializer.serialize_unit_variant("HealthEvaluationKind", 13u32, "Partition"),
            Self::DeployedServicePackage => serializer.serialize_unit_variant("HealthEvaluationKind", 14u32, "DeployedServicePackage"),
            Self::DeployedApplication => serializer.serialize_unit_variant("HealthEvaluationKind", 15u32, "DeployedApplication"),
            Self::Service => serializer.serialize_unit_variant("HealthEvaluationKind", 16u32, "Service"),
            Self::Node => serializer.serialize_unit_variant("HealthEvaluationKind", 17u32, "Node"),
            Self::Application => serializer.serialize_unit_variant("HealthEvaluationKind", 18u32, "Application"),
            Self::DeltaNodesCheck => serializer.serialize_unit_variant("HealthEvaluationKind", 19u32, "DeltaNodesCheck"),
            Self::UpgradeDomainDeltaNodesCheck => {
                serializer.serialize_unit_variant("HealthEvaluationKind", 20u32, "UpgradeDomainDeltaNodesCheck")
            }
            Self::ApplicationTypeApplications => {
                serializer.serialize_unit_variant("HealthEvaluationKind", 21u32, "ApplicationTypeApplications")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Wrapper object for health evaluation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthEvaluationWrapper {
    #[doc = "Represents a health evaluation which describes the data and the algorithm used by health manager to evaluate the health of an entity."]
    #[serde(rename = "HealthEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub health_evaluation: Option<HealthEvaluation>,
}
impl HealthEvaluationWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents health information reported on a health entity, such as cluster, application or node, with additional metadata added by the Health Manager."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthEvent {
    #[serde(flatten)]
    pub health_information: HealthInformation,
    #[doc = "Returns true if the health event is expired, otherwise false."]
    #[serde(rename = "IsExpired", default, skip_serializing_if = "Option::is_none")]
    pub is_expired: Option<bool>,
    #[doc = "The date and time when the health report was sent by the source."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub source_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The date and time when the health report was last modified by the health store."]
    #[serde(rename = "LastModifiedUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "If the current health state is 'Ok', this property returns the time at which the health report was first reported with 'Ok'.\nFor periodic reporting, many reports with the same state may have been generated.\nThis property returns the date and time when the first 'Ok' health report was received.\n\nIf the current health state is 'Error' or 'Warning', returns the date and time at which the health state was last in 'Ok', before transitioning to a different state.\n\nIf the health state was never 'Ok', the value will be zero date-time."]
    #[serde(rename = "LastOkTransitionAt", with = "azure_core::date::rfc3339::option")]
    pub last_ok_transition_at: Option<time::OffsetDateTime>,
    #[doc = "If the current health state is 'Warning', this property returns the time at which the health report was first reported with 'Warning'. For periodic reporting, many reports with the same state may have been generated however, this property returns only the date and time at the first 'Warning' health report was received.\n\nIf the current health state is 'Ok' or 'Error', returns the date and time at which the health state was last in 'Warning', before transitioning to a different state.\n\nIf the health state was never 'Warning', the value will be zero date-time."]
    #[serde(rename = "LastWarningTransitionAt", with = "azure_core::date::rfc3339::option")]
    pub last_warning_transition_at: Option<time::OffsetDateTime>,
    #[doc = "If the current health state is 'Error', this property returns the time at which the health report was first reported with 'Error'. For periodic reporting, many reports with the same state may have been generated however, this property returns only the date and time at the first 'Error' health report was received.\n\nIf the current health state is 'Ok' or 'Warning', returns the date and time at which the health state was last in 'Error', before transitioning to a different state.\n\nIf the health state was never 'Error', the value will be zero date-time."]
    #[serde(rename = "LastErrorTransitionAt", with = "azure_core::date::rfc3339::option")]
    pub last_error_transition_at: Option<time::OffsetDateTime>,
}
impl HealthEvent {
    pub fn new(health_information: HealthInformation) -> Self {
        Self {
            health_information,
            is_expired: None,
            source_utc_timestamp: None,
            last_modified_utc_timestamp: None,
            last_ok_transition_at: None,
            last_warning_transition_at: None,
            last_error_transition_at: None,
        }
    }
}
#[doc = "Represents common health report information. It is included in all health reports sent to health store and in all health events returned by health queries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthInformation {
    #[doc = "The source name that identifies the client/watchdog/system component that generated the health information."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "The property of the health information. An entity can have health reports for different properties.\nThe property is a string and not a fixed enumeration to allow the reporter flexibility to categorize the state condition that triggers the report.\nFor example, a reporter with SourceId \"LocalWatchdog\" can monitor the state of the available disk on a node,\nso it can report \"AvailableDisk\" property on that node.\nThe same reporter can monitor the node connectivity, so it can report a property \"Connectivity\" on the same node.\nIn the health store, these reports are treated as separate health events for the specified node.\n\nTogether with the SourceId, the property uniquely identifies the health information."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "HealthState")]
    pub health_state: HealthState,
    #[doc = "The duration for which this health report is valid. This field uses ISO8601 format for specifying the duration.\nWhen clients report periodically, they should send reports with higher frequency than time to live.\nIf clients report on transition, they can set the time to live to infinite.\nWhen time to live expires, the health event that contains the health information\nis either removed from health store, if RemoveWhenExpired is true, or evaluated at error, if RemoveWhenExpired false.\n\nIf not specified, time to live defaults to infinite value."]
    #[serde(rename = "TimeToLiveInMilliSeconds", default, skip_serializing_if = "Option::is_none")]
    pub time_to_live_in_milli_seconds: Option<String>,
    #[doc = "The description of the health information. It represents free text used to add human readable information about the report.\nThe maximum string length for the description is 4096 characters.\nIf the provided string is longer, it will be automatically truncated.\nWhen truncated, the last characters of the description contain a marker \"[Truncated]\", and total string size is 4096 characters.\nThe presence of the marker indicates to users that truncation occurred.\nNote that when truncated, the description has less than 4096 characters from the original string."]
    #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The sequence number for this health report as a numeric string.\nThe report sequence number is used by the health store to detect stale reports.\nIf not specified, a sequence number is auto-generated by the health client when a report is added."]
    #[serde(rename = "SequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    #[doc = "Value that indicates whether the report is removed from health store when it expires.\nIf set to true, the report is removed from the health store after it expires.\nIf set to false, the report is treated as an error when expired. The value of this property is false by default.\nWhen clients report periodically, they should set RemoveWhenExpired false (default).\nThis way, if the reporter has issues (e.g. deadlock) and can't report, the entity is evaluated at error when the health report expires.\nThis flags the entity as being in Error health state."]
    #[serde(rename = "RemoveWhenExpired", default, skip_serializing_if = "Option::is_none")]
    pub remove_when_expired: Option<bool>,
    #[doc = "A health report ID which identifies the health report and can be used to find more detailed information about a specific health event at\naka.ms/sfhealthid"]
    #[serde(rename = "HealthReportId", default, skip_serializing_if = "Option::is_none")]
    pub health_report_id: Option<String>,
}
impl HealthInformation {
    pub fn new(source_id: String, property: String, health_state: HealthState) -> Self {
        Self {
            source_id,
            property,
            health_state,
            time_to_live_in_milli_seconds: None,
            description: None,
            sequence_number: None,
            remove_when_expired: None,
            health_report_id: None,
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
#[doc = "Represents information about how many health entities are in Ok, Warning and Error health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthStateCount {
    #[doc = "The number of health entities with aggregated health state Ok."]
    #[serde(rename = "OkCount", default, skip_serializing_if = "Option::is_none")]
    pub ok_count: Option<i64>,
    #[doc = "The number of health entities with aggregated health state Warning."]
    #[serde(rename = "WarningCount", default, skip_serializing_if = "Option::is_none")]
    pub warning_count: Option<i64>,
    #[doc = "The number of health entities with aggregated health state Error."]
    #[serde(rename = "ErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
}
impl HealthStateCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The health statistics of an entity, returned as part of the health query result when the query description is configured to include statistics.\nThe statistics include health state counts for all children types of the current entity.\nFor example, for cluster, the health statistics include health state counts for nodes, applications, services, partitions, replicas, deployed applications and deployed service packages.\nFor partition, the health statistics include health counts for replicas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthStatistics {
    #[doc = "List of health state counts per entity kind, which keeps track of how many children of the queried entity are in Ok, Warning and Error state."]
    #[serde(rename = "HealthStateCountList", default, skip_serializing_if = "Vec::is_empty")]
    pub health_state_count_list: Vec<EntityKindHealthStateCount>,
}
impl HealthStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the isolation mode of main entry point of a code package when it's host type is ContainerHost. This is specified as part of container host policies in application manifest while importing service manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HostIsolationMode")]
pub enum HostIsolationMode {
    None,
    Process,
    HyperV,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HostIsolationMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HostIsolationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HostIsolationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("HostIsolationMode", 0u32, "None"),
            Self::Process => serializer.serialize_unit_variant("HostIsolationMode", 1u32, "Process"),
            Self::HyperV => serializer.serialize_unit_variant("HostIsolationMode", 2u32, "HyperV"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the type of host for main entry point of a code package as specified in service manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HostType")]
pub enum HostType {
    Invalid,
    ExeHost,
    ContainerHost,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HostType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HostType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HostType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("HostType", 0u32, "Invalid"),
            Self::ExeHost => serializer.serialize_unit_variant("HostType", 1u32, "ExeHost"),
            Self::ContainerHost => serializer.serialize_unit_variant("HostType", 2u32, "ContainerHost"),
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
#[doc = "Information describing the identities associated with this application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityDescription {
    #[doc = "the endpoint for the token service managing this identity"]
    #[serde(rename = "tokenServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub token_service_endpoint: Option<String>,
    #[doc = "the types of identities associated with this resource; currently restricted to 'SystemAssigned and UserAssigned'"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "the identifier of the tenant containing the application's identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "the object identifier of the Service Principal of the identity associated with this resource."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Defines a map that contains user assigned identities."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentityMap>,
}
impl IdentityDescription {
    pub fn new(type_: String) -> Self {
        Self {
            token_service_endpoint: None,
            type_,
            tenant_id: None,
            principal_id: None,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Describes a single user-assigned identity associated with the application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityItemDescription {
    #[doc = "the object identifier of the Service Principal which this identity represents."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "the client identifier of the Service Principal which this identity represents."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl IdentityItemDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image registry credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRegistryCredential {
    #[doc = "Docker image registry server, without protocol such as `http` and `https`."]
    pub server: String,
    #[doc = "The username for the private registry."]
    pub username: String,
    #[doc = "The type of the image registry password being given in password"]
    #[serde(rename = "passwordType", default, skip_serializing_if = "Option::is_none")]
    pub password_type: Option<ImageRegistryPasswordType>,
    #[doc = "The password for the private registry. The password is required for create or update operations, however it is not returned in the get or list operations. Will be processed based on the type provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ImageRegistryCredential {
    pub fn new(server: String, username: String) -> Self {
        Self {
            server,
            username,
            password_type: None,
            password: None,
        }
    }
}
#[doc = "The type of the image registry password being given in password"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImageRegistryPasswordType")]
pub enum ImageRegistryPasswordType {
    ClearText,
    KeyVaultReference,
    SecretValueReference,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImageRegistryPasswordType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImageRegistryPasswordType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImageRegistryPasswordType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ClearText => serializer.serialize_unit_variant("ImageRegistryPasswordType", 0u32, "ClearText"),
            Self::KeyVaultReference => serializer.serialize_unit_variant("ImageRegistryPasswordType", 1u32, "KeyVaultReference"),
            Self::SecretValueReference => serializer.serialize_unit_variant("ImageRegistryPasswordType", 2u32, "SecretValueReference"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ImageRegistryPasswordType {
    fn default() -> Self {
        Self::ClearText
    }
}
#[doc = "Information about the image store content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageStoreContent {
    #[doc = "The list of image store file info objects represents files found under the given image store relative path."]
    #[serde(rename = "StoreFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub store_files: Vec<FileInfo>,
    #[doc = "The list of image store folder info objects represents subfolders found under the given image store relative path."]
    #[serde(rename = "StoreFolders", default, skip_serializing_if = "Vec::is_empty")]
    pub store_folders: Vec<FolderInfo>,
}
impl ImageStoreContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about how to copy image store content from one image store relative path to another image store relative path."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageStoreCopyDescription {
    #[doc = "The relative path of source image store content to be copied from."]
    #[serde(rename = "RemoteSource")]
    pub remote_source: String,
    #[doc = "The relative path of destination image store content to be copied to."]
    #[serde(rename = "RemoteDestination")]
    pub remote_destination: String,
    #[doc = "The list of the file names to be skipped for copying."]
    #[serde(rename = "SkipFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub skip_files: Vec<String>,
    #[doc = "Indicates whether to check mark file during copying. The property is true if checking mark file is required, false otherwise. The mark file is used to check whether the folder is well constructed. If the property is true and mark file does not exist, the copy is skipped."]
    #[serde(rename = "CheckMarkFile", default, skip_serializing_if = "Option::is_none")]
    pub check_mark_file: Option<bool>,
}
impl ImageStoreCopyDescription {
    pub fn new(remote_source: String, remote_destination: String) -> Self {
        Self {
            remote_source,
            remote_destination,
            skip_files: Vec::new(),
            check_mark_file: None,
        }
    }
}
#[doc = "Information about the ImageStore's resource usage"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageStoreInfo {
    #[doc = "Information about the disk"]
    #[serde(rename = "DiskInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_info: Option<DiskInfo>,
    #[doc = "Information about how much space and how many files in the file system the ImageStore is using in this category"]
    #[serde(rename = "UsedByMetadata", default, skip_serializing_if = "Option::is_none")]
    pub used_by_metadata: Option<UsageInfo>,
    #[doc = "Information about how much space and how many files in the file system the ImageStore is using in this category"]
    #[serde(rename = "UsedByStaging", default, skip_serializing_if = "Option::is_none")]
    pub used_by_staging: Option<UsageInfo>,
    #[doc = "Information about how much space and how many files in the file system the ImageStore is using in this category"]
    #[serde(rename = "UsedByCopy", default, skip_serializing_if = "Option::is_none")]
    pub used_by_copy: Option<UsageInfo>,
    #[doc = "Information about how much space and how many files in the file system the ImageStore is using in this category"]
    #[serde(rename = "UsedByRegister", default, skip_serializing_if = "Option::is_none")]
    pub used_by_register: Option<UsageInfo>,
}
impl ImageStoreInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ImageStoreRelativePath = String;
pub type InfrastructureServiceResponse = String;
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
pub type InstanceCloseDelayDurationInSeconds = i64;
pub type InstanceId = String;
#[doc = "Describes a Service Fabric property value of type Int64."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Int64PropertyValue {
    #[serde(flatten)]
    pub property_value: PropertyValue,
    #[doc = "The data of the property value."]
    #[serde(rename = "Data")]
    pub data: String,
}
impl Int64PropertyValue {
    pub fn new(property_value: PropertyValue, data: String) -> Self {
        Self { property_value, data }
    }
}
#[doc = "Describes the partition information for the integer range that is based on partition schemes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Int64RangePartitionInformation {
    #[serde(flatten)]
    pub partition_information: PartitionInformation,
    #[doc = "Specifies the minimum key value handled by this partition."]
    #[serde(rename = "LowKey", default, skip_serializing_if = "Option::is_none")]
    pub low_key: Option<String>,
    #[doc = "Specifies the maximum key value handled by this partition."]
    #[serde(rename = "HighKey", default, skip_serializing_if = "Option::is_none")]
    pub high_key: Option<String>,
}
impl Int64RangePartitionInformation {
    pub fn new(partition_information: PartitionInformation) -> Self {
        Self {
            partition_information,
            low_key: None,
            high_key: None,
        }
    }
}
#[doc = "Represents information about an operation in a terminal state (Completed or Faulted)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvokeDataLossResult {
    #[doc = "If OperationState is Completed, this is 0.  If OperationState is Faulted, this is an error code indicating the reason."]
    #[serde(rename = "ErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "This class returns information about the partition that the user-induced operation acted upon."]
    #[serde(rename = "SelectedPartition", default, skip_serializing_if = "Option::is_none")]
    pub selected_partition: Option<SelectedPartition>,
}
impl InvokeDataLossResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents information about an operation in a terminal state (Completed or Faulted)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvokeQuorumLossResult {
    #[doc = "If OperationState is Completed, this is 0.  If OperationState is Faulted, this is an error code indicating the reason."]
    #[serde(rename = "ErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "This class returns information about the partition that the user-induced operation acted upon."]
    #[serde(rename = "SelectedPartition", default, skip_serializing_if = "Option::is_none")]
    pub selected_partition: Option<SelectedPartition>,
}
impl InvokeQuorumLossResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Key value store related information for the replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyValueStoreReplicaStatus {
    #[serde(flatten)]
    pub replica_status_base: ReplicaStatusBase,
    #[doc = "Value indicating the estimated number of rows in the underlying database."]
    #[serde(rename = "DatabaseRowCountEstimate", default, skip_serializing_if = "Option::is_none")]
    pub database_row_count_estimate: Option<String>,
    #[doc = "Value indicating the estimated size of the underlying database."]
    #[serde(rename = "DatabaseLogicalSizeEstimate", default, skip_serializing_if = "Option::is_none")]
    pub database_logical_size_estimate: Option<String>,
    #[doc = "Value indicating the latest key-prefix filter applied to enumeration during the callback. Null if there is no pending callback."]
    #[serde(rename = "CopyNotificationCurrentKeyFilter", default, skip_serializing_if = "Option::is_none")]
    pub copy_notification_current_key_filter: Option<String>,
    #[doc = "Value indicating the latest number of keys enumerated during the callback. 0 if there is no pending callback."]
    #[serde(rename = "CopyNotificationCurrentProgress", default, skip_serializing_if = "Option::is_none")]
    pub copy_notification_current_progress: Option<String>,
    #[doc = "Value indicating the current status details of the replica."]
    #[serde(rename = "StatusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}
impl KeyValueStoreReplicaStatus {
    pub fn new(replica_status_base: ReplicaStatusBase) -> Self {
        Self {
            replica_status_base,
            database_row_count_estimate: None,
            database_logical_size_estimate: None,
            copy_notification_current_key_filter: None,
            copy_notification_current_progress: None,
            status_details: None,
        }
    }
}
#[doc = "Represents data structure that contains load information for a certain metric in a cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadMetricInformation {
    #[doc = "Name of the metric for which this load information is provided."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value that indicates whether the metrics is balanced or not before resource balancer run"]
    #[serde(rename = "IsBalancedBefore", default, skip_serializing_if = "Option::is_none")]
    pub is_balanced_before: Option<bool>,
    #[doc = "Value that indicates whether the metrics is balanced or not after resource balancer run."]
    #[serde(rename = "IsBalancedAfter", default, skip_serializing_if = "Option::is_none")]
    pub is_balanced_after: Option<bool>,
    #[doc = "The standard average deviation of the metrics before resource balancer run."]
    #[serde(rename = "DeviationBefore", default, skip_serializing_if = "Option::is_none")]
    pub deviation_before: Option<String>,
    #[doc = "The standard average deviation of the metrics after resource balancer run."]
    #[serde(rename = "DeviationAfter", default, skip_serializing_if = "Option::is_none")]
    pub deviation_after: Option<String>,
    #[doc = "The balancing threshold for a certain metric."]
    #[serde(rename = "BalancingThreshold", default, skip_serializing_if = "Option::is_none")]
    pub balancing_threshold: Option<String>,
    #[doc = "The current action being taken with regard to this metric"]
    #[serde(rename = "Action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The Activity Threshold specified for this metric in the system Cluster Manifest."]
    #[serde(rename = "ActivityThreshold", default, skip_serializing_if = "Option::is_none")]
    pub activity_threshold: Option<String>,
    #[doc = "The total cluster capacity for a given metric"]
    #[serde(rename = "ClusterCapacity", default, skip_serializing_if = "Option::is_none")]
    pub cluster_capacity: Option<String>,
    #[doc = "The total cluster load. In future releases of Service Fabric this parameter will be deprecated in favor of CurrentClusterLoad."]
    #[serde(rename = "ClusterLoad", default, skip_serializing_if = "Option::is_none")]
    pub cluster_load: Option<String>,
    #[doc = "The total cluster load."]
    #[serde(rename = "CurrentClusterLoad", default, skip_serializing_if = "Option::is_none")]
    pub current_cluster_load: Option<String>,
    #[doc = "The remaining capacity for the metric in the cluster. In future releases of Service Fabric this parameter will be deprecated in favor of ClusterCapacityRemaining."]
    #[serde(rename = "ClusterRemainingCapacity", default, skip_serializing_if = "Option::is_none")]
    pub cluster_remaining_capacity: Option<String>,
    #[doc = "The remaining capacity for the metric in the cluster."]
    #[serde(rename = "ClusterCapacityRemaining", default, skip_serializing_if = "Option::is_none")]
    pub cluster_capacity_remaining: Option<String>,
    #[doc = "Indicates that the metric is currently over capacity in the cluster."]
    #[serde(rename = "IsClusterCapacityViolation", default, skip_serializing_if = "Option::is_none")]
    pub is_cluster_capacity_violation: Option<bool>,
    #[doc = "The reserved percentage of total node capacity for this metric."]
    #[serde(rename = "NodeBufferPercentage", default, skip_serializing_if = "Option::is_none")]
    pub node_buffer_percentage: Option<String>,
    #[doc = "Remaining capacity in the cluster excluding the reserved space. In future releases of Service Fabric this parameter will be deprecated in favor of BufferedClusterCapacityRemaining."]
    #[serde(rename = "ClusterBufferedCapacity", default, skip_serializing_if = "Option::is_none")]
    pub cluster_buffered_capacity: Option<String>,
    #[doc = "Remaining capacity in the cluster excluding the reserved space."]
    #[serde(rename = "BufferedClusterCapacityRemaining", default, skip_serializing_if = "Option::is_none")]
    pub buffered_cluster_capacity_remaining: Option<String>,
    #[doc = "The remaining percentage of cluster total capacity for this metric."]
    #[serde(rename = "ClusterRemainingBufferedCapacity", default, skip_serializing_if = "Option::is_none")]
    pub cluster_remaining_buffered_capacity: Option<String>,
    #[doc = "The minimum load on any node for this metric. In future releases of Service Fabric this parameter will be deprecated in favor of MinimumNodeLoad."]
    #[serde(rename = "MinNodeLoadValue", default, skip_serializing_if = "Option::is_none")]
    pub min_node_load_value: Option<String>,
    #[doc = "The minimum load on any node for this metric."]
    #[serde(rename = "MinimumNodeLoad", default, skip_serializing_if = "Option::is_none")]
    pub minimum_node_load: Option<String>,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a node. Node Id is deterministically generated from node name."]
    #[serde(rename = "MinNodeLoadNodeId", default, skip_serializing_if = "Option::is_none")]
    pub min_node_load_node_id: Option<NodeId>,
    #[doc = "The maximum load on any node for this metric. In future releases of Service Fabric this parameter will be deprecated in favor of MaximumNodeLoad."]
    #[serde(rename = "MaxNodeLoadValue", default, skip_serializing_if = "Option::is_none")]
    pub max_node_load_value: Option<String>,
    #[doc = "The maximum load on any node for this metric."]
    #[serde(rename = "MaximumNodeLoad", default, skip_serializing_if = "Option::is_none")]
    pub maximum_node_load: Option<String>,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a node. Node Id is deterministically generated from node name."]
    #[serde(rename = "MaxNodeLoadNodeId", default, skip_serializing_if = "Option::is_none")]
    pub max_node_load_node_id: Option<NodeId>,
    #[doc = "This value represents the load of the replicas that are planned to be removed in the future within the cluster.\nThis kind of load is reported for replicas that are currently being moving to other nodes and for replicas that are currently being dropped but still use the load on the source node."]
    #[serde(rename = "PlannedLoadRemoval", default, skip_serializing_if = "Option::is_none")]
    pub planned_load_removal: Option<String>,
}
impl LoadMetricInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the load metric report which contains the time metric was reported, its name and value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadMetricReport {
    #[doc = "Gets the UTC time when the load was reported."]
    #[serde(rename = "LastReportedUtc", with = "azure_core::date::rfc3339::option")]
    pub last_reported_utc: Option<time::OffsetDateTime>,
    #[doc = "The name of the load metric."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the load metric. In future releases of Service Fabric this parameter will be deprecated in favor of CurrentValue."]
    #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The value of the load metric."]
    #[serde(rename = "CurrentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<String>,
}
impl LoadMetricReport {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about load reported by replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadMetricReportInfo {
    #[doc = "The name of the metric."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the load for the metric. In future releases of Service Fabric this parameter will be deprecated in favor of CurrentValue."]
    #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
    #[doc = "The double value of the load for the metric."]
    #[serde(rename = "CurrentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<String>,
    #[doc = "The UTC time when the load is reported."]
    #[serde(rename = "LastReportedUtc", with = "azure_core::date::rfc3339::option")]
    pub last_reported_utc: Option<time::OffsetDateTime>,
}
impl LoadMetricReportInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type LoadMetricReportInfoList = Vec<LoadMetricReportInfo>;
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
#[doc = "Describes a managed application identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedApplicationIdentity {
    #[doc = "The name of the identity."]
    #[serde(rename = "Name")]
    pub name: String,
    #[doc = "The identity's PrincipalId."]
    #[serde(rename = "PrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}
impl ManagedApplicationIdentity {
    pub fn new(name: String) -> Self {
        Self { name, principal_id: None }
    }
}
#[doc = "Managed application identity description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedApplicationIdentityDescription {
    #[doc = "Token service endpoint."]
    #[serde(rename = "TokenServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub token_service_endpoint: Option<String>,
    #[doc = "A list of managed application identity objects."]
    #[serde(rename = "ManagedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub managed_identities: Option<ManagedApplicationIdentityList>,
}
impl ManagedApplicationIdentityDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ManagedApplicationIdentityList = Vec<ManagedApplicationIdentity>;
#[doc = "Specifies metric load information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricLoadDescription {
    #[doc = "The name of the reported metric."]
    #[serde(rename = "MetricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[doc = "The current value of the metric load."]
    #[serde(rename = "CurrentLoad", default, skip_serializing_if = "Option::is_none")]
    pub current_load: Option<i64>,
    #[doc = "The predicted value of the metric load."]
    #[serde(rename = "PredictedLoad", default, skip_serializing_if = "Option::is_none")]
    pub predicted_load: Option<i64>,
}
impl MetricLoadDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type MinInstanceCount = i64;
pub type MinInstancePercentage = i64;
#[doc = "Describes the parameters for monitoring an upgrade in Monitored mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringPolicyDescription {
    #[doc = "The compensating action to perform when a Monitored upgrade encounters monitoring policy or health policy violations.\nInvalid indicates the failure action is invalid. Rollback specifies that the upgrade will start rolling back automatically.\nManual indicates that the upgrade will switch to UnmonitoredManual upgrade mode."]
    #[serde(rename = "FailureAction", default, skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<FailureAction>,
    #[doc = "The amount of time to wait after completing an upgrade domain before applying health policies. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "HealthCheckWaitDurationInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub health_check_wait_duration_in_milliseconds: Option<HealthCheckWaitDuration>,
    #[doc = "The amount of time that the application or cluster must remain healthy before the upgrade proceeds to the next upgrade domain. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(
        rename = "HealthCheckStableDurationInMilliseconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub health_check_stable_duration_in_milliseconds: Option<HealthCheckStableDuration>,
    #[doc = "The amount of time to retry health evaluation when the application or cluster is unhealthy before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "HealthCheckRetryTimeoutInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub health_check_retry_timeout_in_milliseconds: Option<HealthCheckRetryTimeout>,
    #[doc = "The amount of time the overall upgrade has to complete before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "UpgradeTimeoutInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_timeout_in_milliseconds: Option<UpgradeTimeout>,
    #[doc = "The amount of time each upgrade domain has to complete before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "UpgradeDomainTimeoutInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain_timeout_in_milliseconds: Option<UpgradeDomainTimeout>,
}
impl MonitoringPolicyDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the move cost for the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MoveCost")]
pub enum MoveCost {
    Zero,
    Low,
    Medium,
    High,
    VeryHigh,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MoveCost {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MoveCost {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MoveCost {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Zero => serializer.serialize_unit_variant("MoveCost", 0u32, "Zero"),
            Self::Low => serializer.serialize_unit_variant("MoveCost", 1u32, "Low"),
            Self::Medium => serializer.serialize_unit_variant("MoveCost", 2u32, "Medium"),
            Self::High => serializer.serialize_unit_variant("MoveCost", 3u32, "High"),
            Self::VeryHigh => serializer.serialize_unit_variant("MoveCost", 4u32, "VeryHigh"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes a Service Fabric name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameDescription {
    #[doc = "The Service Fabric name, including the 'fabric:' URI scheme."]
    #[serde(rename = "Name")]
    pub name: FabricName,
}
impl NameDescription {
    pub fn new(name: FabricName) -> Self {
        Self { name }
    }
}
#[doc = "Describes the partition information for the name as a string that is based on partition schemes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedPartitionInformation {
    #[serde(flatten)]
    pub partition_information: PartitionInformation,
    #[doc = "Name of the partition."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl NamedPartitionInformation {
    pub fn new(partition_information: PartitionInformation) -> Self {
        Self {
            partition_information,
            name: None,
        }
    }
}
#[doc = "Describes the named partition scheme of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedPartitionSchemeDescription {
    #[serde(flatten)]
    pub partition_scheme_description: PartitionSchemeDescription,
    #[doc = "The number of partitions."]
    #[serde(rename = "Count")]
    pub count: i64,
    #[doc = "Array of size specified by the ‘Count’ parameter, for the names of the partitions."]
    #[serde(rename = "Names")]
    pub names: Vec<String>,
}
impl NamedPartitionSchemeDescription {
    pub fn new(partition_scheme_description: PartitionSchemeDescription, count: i64, names: Vec<String>) -> Self {
        Self {
            partition_scheme_description,
            count,
            names,
        }
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
    #[doc = "Name of the Network resource."]
    pub name: NetworkResourceName,
    #[doc = "Describes properties of a network resource."]
    pub properties: NetworkResourceProperties,
}
impl NetworkResourceDescription {
    pub fn new(name: NetworkResourceName, properties: NetworkResourceProperties) -> Self {
        Self { name, properties }
    }
}
pub type NetworkResourceName = String;
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
    #[doc = "The type of a Service Fabric container network."]
    pub kind: NetworkKind,
}
impl NetworkResourcePropertiesBase {
    pub fn new(kind: NetworkKind) -> Self {
        Self { kind }
    }
}
pub type NextUpgradeDomain = String;
#[doc = "Node Aborted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeAbortedEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstance")]
    pub node_instance: i64,
    #[doc = "Id of Node."]
    #[serde(rename = "NodeId")]
    pub node_id: String,
    #[doc = "Upgrade domain of Node."]
    #[serde(rename = "UpgradeDomain")]
    pub upgrade_domain: String,
    #[doc = "Fault domain of Node."]
    #[serde(rename = "FaultDomain")]
    pub fault_domain: String,
    #[doc = "IP address or FQDN."]
    #[serde(rename = "IpAddressOrFQDN")]
    pub ip_address_or_fqdn: String,
    #[doc = "Name of Host."]
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[doc = "Indicates if it is seed node."]
    #[serde(rename = "IsSeedNode")]
    pub is_seed_node: bool,
    #[doc = "Version of Node."]
    #[serde(rename = "NodeVersion")]
    pub node_version: String,
}
impl NodeAbortedEvent {
    pub fn new(
        node_event: NodeEvent,
        node_instance: i64,
        node_id: String,
        upgrade_domain: String,
        fault_domain: String,
        ip_address_or_fqdn: String,
        hostname: String,
        is_seed_node: bool,
        node_version: String,
    ) -> Self {
        Self {
            node_event,
            node_instance,
            node_id,
            upgrade_domain,
            fault_domain,
            ip_address_or_fqdn,
            hostname,
            is_seed_node,
            node_version,
        }
    }
}
#[doc = "Node Added event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeAddedToClusterEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node."]
    #[serde(rename = "NodeId")]
    pub node_id: String,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstance")]
    pub node_instance: i64,
    #[doc = "Type of Node."]
    #[serde(rename = "NodeType")]
    pub node_type: String,
    #[doc = "Fabric version."]
    #[serde(rename = "FabricVersion")]
    pub fabric_version: String,
    #[doc = "IP address or FQDN."]
    #[serde(rename = "IpAddressOrFQDN")]
    pub ip_address_or_fqdn: String,
    #[doc = "Capacities."]
    #[serde(rename = "NodeCapacities")]
    pub node_capacities: String,
}
impl NodeAddedToClusterEvent {
    pub fn new(
        node_event: NodeEvent,
        node_id: String,
        node_instance: i64,
        node_type: String,
        fabric_version: String,
        ip_address_or_fqdn: String,
        node_capacities: String,
    ) -> Self {
        Self {
            node_event,
            node_id,
            node_instance,
            node_type,
            fabric_version,
            ip_address_or_fqdn,
            node_capacities,
        }
    }
}
#[doc = "Node Closed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeClosedEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node."]
    #[serde(rename = "NodeId")]
    pub node_id: String,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstance")]
    pub node_instance: i64,
    #[doc = "Describes error."]
    #[serde(rename = "Error")]
    pub error: String,
}
impl NodeClosedEvent {
    pub fn new(node_event: NodeEvent, node_id: String, node_instance: i64, error: String) -> Self {
        Self {
            node_event,
            node_id,
            node_instance,
            error,
        }
    }
}
#[doc = "Node Deactivate Completed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeDeactivateCompletedEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstance")]
    pub node_instance: i64,
    #[doc = "Describes deactivate intent."]
    #[serde(rename = "EffectiveDeactivateIntent")]
    pub effective_deactivate_intent: String,
    #[doc = "Batch Ids."]
    #[serde(rename = "BatchIdsWithDeactivateIntent")]
    pub batch_ids_with_deactivate_intent: String,
    #[doc = "Start time."]
    #[serde(rename = "StartTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
}
impl NodeDeactivateCompletedEvent {
    pub fn new(
        node_event: NodeEvent,
        node_instance: i64,
        effective_deactivate_intent: String,
        batch_ids_with_deactivate_intent: String,
        start_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            node_event,
            node_instance,
            effective_deactivate_intent,
            batch_ids_with_deactivate_intent,
            start_time,
        }
    }
}
#[doc = "Node Deactivate Started event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeDeactivateStartedEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstance")]
    pub node_instance: i64,
    #[doc = "Batch Id."]
    #[serde(rename = "BatchId")]
    pub batch_id: String,
    #[doc = "Describes deactivate intent."]
    #[serde(rename = "DeactivateIntent")]
    pub deactivate_intent: String,
}
impl NodeDeactivateStartedEvent {
    pub fn new(node_event: NodeEvent, node_instance: i64, batch_id: String, deactivate_intent: String) -> Self {
        Self {
            node_event,
            node_instance,
            batch_id,
            deactivate_intent,
        }
    }
}
#[doc = "Information about the node deactivation. This information is valid for a node that is undergoing deactivation or has already been deactivated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeDeactivationInfo {
    #[doc = "The intent or the reason for deactivating the node. Following are the possible values for it."]
    #[serde(rename = "NodeDeactivationIntent", default, skip_serializing_if = "Option::is_none")]
    pub node_deactivation_intent: Option<NodeDeactivationIntent>,
    #[doc = "The status of node deactivation operation. Following are the possible values."]
    #[serde(rename = "NodeDeactivationStatus", default, skip_serializing_if = "Option::is_none")]
    pub node_deactivation_status: Option<NodeDeactivationStatus>,
    #[doc = "List of tasks representing the deactivation operation on the node."]
    #[serde(rename = "NodeDeactivationTask", default, skip_serializing_if = "Option::is_none")]
    pub node_deactivation_task: Option<NodeDeactivationTaskList>,
    #[doc = "List of pending safety checks"]
    #[serde(rename = "PendingSafetyChecks", default, skip_serializing_if = "Option::is_none")]
    pub pending_safety_checks: Option<SafetyCheckInfoList>,
}
impl NodeDeactivationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The intent or the reason for deactivating the node. Following are the possible values for it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NodeDeactivationIntent")]
pub enum NodeDeactivationIntent {
    Invalid,
    Pause,
    Restart,
    RemoveData,
    RemoveNode,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NodeDeactivationIntent {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NodeDeactivationIntent {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NodeDeactivationIntent {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("NodeDeactivationIntent", 0u32, "Invalid"),
            Self::Pause => serializer.serialize_unit_variant("NodeDeactivationIntent", 1u32, "Pause"),
            Self::Restart => serializer.serialize_unit_variant("NodeDeactivationIntent", 2u32, "Restart"),
            Self::RemoveData => serializer.serialize_unit_variant("NodeDeactivationIntent", 3u32, "RemoveData"),
            Self::RemoveNode => serializer.serialize_unit_variant("NodeDeactivationIntent", 4u32, "RemoveNode"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of node deactivation operation. Following are the possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NodeDeactivationStatus")]
pub enum NodeDeactivationStatus {
    None,
    SafetyCheckInProgress,
    SafetyCheckComplete,
    Completed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NodeDeactivationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NodeDeactivationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NodeDeactivationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("NodeDeactivationStatus", 0u32, "None"),
            Self::SafetyCheckInProgress => serializer.serialize_unit_variant("NodeDeactivationStatus", 1u32, "SafetyCheckInProgress"),
            Self::SafetyCheckComplete => serializer.serialize_unit_variant("NodeDeactivationStatus", 2u32, "SafetyCheckComplete"),
            Self::Completed => serializer.serialize_unit_variant("NodeDeactivationStatus", 3u32, "Completed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The task representing the deactivation operation on the node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeDeactivationTask {
    #[doc = "Identity of the task related to deactivation operation on the node."]
    #[serde(rename = "NodeDeactivationTaskId", default, skip_serializing_if = "Option::is_none")]
    pub node_deactivation_task_id: Option<NodeDeactivationTaskId>,
    #[doc = "The intent or the reason for deactivating the node. Following are the possible values for it."]
    #[serde(rename = "NodeDeactivationIntent", default, skip_serializing_if = "Option::is_none")]
    pub node_deactivation_intent: Option<NodeDeactivationIntent>,
}
impl NodeDeactivationTask {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity of the task related to deactivation operation on the node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeDeactivationTaskId {
    #[doc = "Value of the task id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the task that performed the node deactivation. Following are the possible values."]
    #[serde(rename = "NodeDeactivationTaskType", default, skip_serializing_if = "Option::is_none")]
    pub node_deactivation_task_type: Option<NodeDeactivationTaskType>,
}
impl NodeDeactivationTaskId {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type NodeDeactivationTaskList = Vec<NodeDeactivationTask>;
#[doc = "The type of the task that performed the node deactivation. Following are the possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NodeDeactivationTaskType")]
pub enum NodeDeactivationTaskType {
    Invalid,
    Infrastructure,
    Repair,
    Client,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NodeDeactivationTaskType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NodeDeactivationTaskType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NodeDeactivationTaskType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("NodeDeactivationTaskType", 0u32, "Invalid"),
            Self::Infrastructure => serializer.serialize_unit_variant("NodeDeactivationTaskType", 1u32, "Infrastructure"),
            Self::Repair => serializer.serialize_unit_variant("NodeDeactivationTaskType", 2u32, "Repair"),
            Self::Client => serializer.serialize_unit_variant("NodeDeactivationTaskType", 3u32, "Client"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Node Down event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeDownEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstance")]
    pub node_instance: i64,
    #[doc = "Time when Node was last up."]
    #[serde(rename = "LastNodeUpAt", with = "azure_core::date::rfc3339")]
    pub last_node_up_at: time::OffsetDateTime,
}
impl NodeDownEvent {
    pub fn new(node_event: NodeEvent, node_instance: i64, last_node_up_at: time::OffsetDateTime) -> Self {
        Self {
            node_event,
            node_instance,
            last_node_up_at,
        }
    }
}
#[doc = "Represents the base for all Node Events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeEvent {
    #[serde(flatten)]
    pub fabric_event: FabricEvent,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName")]
    pub node_name: NodeName,
}
impl NodeEvent {
    pub fn new(fabric_event: FabricEvent, node_name: NodeName) -> Self {
        Self { fabric_event, node_name }
    }
}
pub type NodeEventList = Vec<NodeEvent>;
#[doc = "Information about the health of a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeHealth {
    #[serde(flatten)]
    pub entity_health: EntityHealth,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<NodeName>,
}
impl NodeHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents health evaluation for a node, containing information about the data and the algorithm used by health store to evaluate health. The evaluation is returned only when the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl NodeHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            node_name: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Node Health Report Expired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeHealthReportExpiredEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstanceId")]
    pub node_instance_id: i64,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl NodeHealthReportExpiredEvent {
    pub fn new(
        node_event: NodeEvent,
        node_instance_id: i64,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            node_event,
            node_instance_id,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Represents the health state of a node, which contains the node identifier and its aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeHealthState {
    #[serde(flatten)]
    pub entity_health_state: EntityHealthState,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<NodeName>,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a node. Node Id is deterministically generated from node name."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<NodeId>,
}
impl NodeHealthState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the health state chunk of a node, which contains the node name and its aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeHealthStateChunk {
    #[serde(flatten)]
    pub entity_health_state_chunk: EntityHealthStateChunk,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
}
impl NodeHealthStateChunk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of node health state chunks in the cluster that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeHealthStateChunkList {
    #[serde(flatten)]
    pub entity_health_state_chunk_list: EntityHealthStateChunkList,
    #[doc = "The list of node health state chunks that respect the input filters in the chunk query."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<NodeHealthStateChunk>,
}
impl NodeHealthStateChunkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines matching criteria to determine whether a node should be included in the returned cluster health chunk.\nOne filter can match zero, one or multiple nodes, depending on its properties.\nCan be specified in the cluster health chunk query description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeHealthStateFilter {
    #[doc = "Name of the node that matches the filter. The filter is applied only to the specified node, if it exists.\nIf the node doesn't exist, no node is returned in the cluster health chunk based on this filter.\nIf the node exists, it is included in the cluster health chunk if the health state matches the other filter properties.\nIf not specified, all nodes that match the parent filters (if any) are taken into consideration and matched against the other filter members, like health state filter."]
    #[serde(rename = "NodeNameFilter", default, skip_serializing_if = "Option::is_none")]
    pub node_name_filter: Option<String>,
    #[doc = "The filter for the health state of the nodes. It allows selecting nodes if they match the desired health states.\nThe possible values are integer value of one of the following health states. Only nodes that match the filter are returned. All nodes are used to evaluate the cluster aggregated health state.\nIf not specified, default value is None, unless the node name is specified. If the filter has default value and node name is specified, the matching node is returned.\nThe state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise 'OR' operator.\nFor example, if the provided value is 6, it matches nodes with HealthState value of OK (2) and Warning (4).\n\n- Default - Default value. Matches any HealthState. The value is zero.\n- None - Filter that doesn't match any HealthState value. Used in order to return no results on a given collection of states. The value is 1.\n- Ok - Filter that matches input with HealthState value Ok. The value is 2.\n- Warning - Filter that matches input with HealthState value Warning. The value is 4.\n- Error - Filter that matches input with HealthState value Error. The value is 8.\n- All - Filter that matches input with any HealthState value. The value is 65535."]
    #[serde(rename = "HealthStateFilter", default, skip_serializing_if = "Option::is_none")]
    pub health_state_filter: Option<i64>,
}
impl NodeHealthStateFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An internal ID used by Service Fabric to uniquely identify a node. Node Id is deterministically generated from node name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeId {
    #[doc = "Value of the node Id. This is a 128 bit integer."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl NodeId {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type NodeIdList = Vec<NodeId>;
#[doc = "Describes the expected impact of a repair to a particular node.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeImpact {
    #[doc = "The name of the impacted node."]
    #[serde(rename = "NodeName")]
    pub node_name: String,
    #[doc = "The level of impact expected."]
    #[serde(rename = "ImpactLevel", default, skip_serializing_if = "Option::is_none")]
    pub impact_level: Option<node_impact::ImpactLevel>,
}
impl NodeImpact {
    pub fn new(node_name: String) -> Self {
        Self {
            node_name,
            impact_level: None,
        }
    }
}
pub mod node_impact {
    use super::*;
    #[doc = "The level of impact expected."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ImpactLevel")]
    pub enum ImpactLevel {
        Invalid,
        None,
        Restart,
        RemoveData,
        RemoveNode,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ImpactLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ImpactLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ImpactLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ImpactLevel", 0u32, "Invalid"),
                Self::None => serializer.serialize_unit_variant("ImpactLevel", 1u32, "None"),
                Self::Restart => serializer.serialize_unit_variant("ImpactLevel", 2u32, "Restart"),
                Self::RemoveData => serializer.serialize_unit_variant("ImpactLevel", 3u32, "RemoveData"),
                Self::RemoveNode => serializer.serialize_unit_variant("ImpactLevel", 4u32, "RemoveNode"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about a node in Service Fabric cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeInfo {
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<NodeName>,
    #[doc = "The IP address or fully qualified domain name of the node."]
    #[serde(rename = "IpAddressOrFQDN", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_or_fqdn: Option<String>,
    #[doc = "The type of the node."]
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The version of Service Fabric binaries that the node is running."]
    #[serde(rename = "CodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub code_version: Option<String>,
    #[doc = "The version of Service Fabric cluster manifest that the node is using."]
    #[serde(rename = "ConfigVersion", default, skip_serializing_if = "Option::is_none")]
    pub config_version: Option<String>,
    #[doc = "The status of the node."]
    #[serde(rename = "NodeStatus", default, skip_serializing_if = "Option::is_none")]
    pub node_status: Option<NodeStatus>,
    #[doc = "Time in seconds since the node has been in NodeStatus Up. Value zero indicates that the node is not Up."]
    #[serde(rename = "NodeUpTimeInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub node_up_time_in_seconds: Option<String>,
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "HealthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[doc = "Indicates if the node is a seed node or not. Returns true if the node is a seed node, otherwise false. A quorum of seed nodes are required for proper operation of Service Fabric cluster."]
    #[serde(rename = "IsSeedNode", default, skip_serializing_if = "Option::is_none")]
    pub is_seed_node: Option<bool>,
    #[doc = "The upgrade domain of the node."]
    #[serde(rename = "UpgradeDomain", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain: Option<String>,
    #[doc = "The fault domain of the node."]
    #[serde(rename = "FaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub fault_domain: Option<String>,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a node. Node Id is deterministically generated from node name."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<NodeId>,
    #[doc = "The ID representing the node instance. While the ID of the node is deterministically generated from the node name and remains same across restarts, the InstanceId changes every time node restarts."]
    #[serde(rename = "InstanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "Information about the node deactivation. This information is valid for a node that is undergoing deactivation or has already been deactivated."]
    #[serde(rename = "NodeDeactivationInfo", default, skip_serializing_if = "Option::is_none")]
    pub node_deactivation_info: Option<NodeDeactivationInfo>,
    #[doc = "Indicates if the node is stopped by calling stop node API or not. Returns true if the node is stopped, otherwise false."]
    #[serde(rename = "IsStopped", default, skip_serializing_if = "Option::is_none")]
    pub is_stopped: Option<bool>,
    #[doc = "Time in seconds since the node has been in NodeStatus Down. Value zero indicates node is not NodeStatus Down."]
    #[serde(rename = "NodeDownTimeInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub node_down_time_in_seconds: Option<String>,
    #[doc = "Date time in UTC when the node came up. If the node has never been up then this value will be zero date time."]
    #[serde(rename = "NodeUpAt", with = "azure_core::date::rfc3339::option")]
    pub node_up_at: Option<time::OffsetDateTime>,
    #[doc = "Date time in UTC when the node went down. If node has never been down then this value will be zero date time."]
    #[serde(rename = "NodeDownAt", with = "azure_core::date::rfc3339::option")]
    pub node_down_at: Option<time::OffsetDateTime>,
}
impl NodeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about load on a Service Fabric node. It holds a summary of all metrics and their load on a node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeLoadInfo {
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
    #[doc = "List that contains metrics and their load information on this node."]
    #[serde(rename = "NodeLoadMetricInformation", default, skip_serializing_if = "Vec::is_empty")]
    pub node_load_metric_information: Vec<NodeLoadMetricInformation>,
}
impl NodeLoadInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents data structure that contains load information for a certain metric on a node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeLoadMetricInformation {
    #[doc = "Name of the metric for which this load information is provided."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Total capacity on the node for this metric."]
    #[serde(rename = "NodeCapacity", default, skip_serializing_if = "Option::is_none")]
    pub node_capacity: Option<String>,
    #[doc = "Current load on the node for this metric. In future releases of Service Fabric this parameter will be deprecated in favor of CurrentNodeLoad."]
    #[serde(rename = "NodeLoad", default, skip_serializing_if = "Option::is_none")]
    pub node_load: Option<String>,
    #[doc = "The remaining capacity on the node for this metric. In future releases of Service Fabric this parameter will be deprecated in favor of NodeCapacityRemaining."]
    #[serde(rename = "NodeRemainingCapacity", default, skip_serializing_if = "Option::is_none")]
    pub node_remaining_capacity: Option<String>,
    #[doc = "Indicates if there is a capacity violation for this metric on the node."]
    #[serde(rename = "IsCapacityViolation", default, skip_serializing_if = "Option::is_none")]
    pub is_capacity_violation: Option<bool>,
    #[doc = "The value that indicates the reserved capacity for this metric on the node."]
    #[serde(rename = "NodeBufferedCapacity", default, skip_serializing_if = "Option::is_none")]
    pub node_buffered_capacity: Option<String>,
    #[doc = "The remaining reserved capacity for this metric on the node. In future releases of Service Fabric this parameter will be deprecated in favor of BufferedNodeCapacityRemaining."]
    #[serde(rename = "NodeRemainingBufferedCapacity", default, skip_serializing_if = "Option::is_none")]
    pub node_remaining_buffered_capacity: Option<String>,
    #[doc = "Current load on the node for this metric."]
    #[serde(rename = "CurrentNodeLoad", default, skip_serializing_if = "Option::is_none")]
    pub current_node_load: Option<String>,
    #[doc = "The remaining capacity on the node for the metric."]
    #[serde(rename = "NodeCapacityRemaining", default, skip_serializing_if = "Option::is_none")]
    pub node_capacity_remaining: Option<String>,
    #[doc = "The remaining capacity which is not reserved by NodeBufferPercentage for this metric on the node."]
    #[serde(rename = "BufferedNodeCapacityRemaining", default, skip_serializing_if = "Option::is_none")]
    pub buffered_node_capacity_remaining: Option<String>,
    #[doc = "This value represents the load of the replicas that are planned to be removed in the future.\nThis kind of load is reported for replicas that are currently being moving to other nodes and for replicas that are currently being dropped but still use the load on the source node."]
    #[serde(rename = "PlannedNodeLoadRemoval", default, skip_serializing_if = "Option::is_none")]
    pub planned_node_load_removal: Option<String>,
}
impl NodeLoadMetricInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type NodeName = String;
#[doc = "Node Health Report Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeNewHealthReportEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstanceId")]
    pub node_instance_id: i64,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl NodeNewHealthReportEvent {
    pub fn new(
        node_event: NodeEvent,
        node_instance_id: i64,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            node_event,
            node_instance_id,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Node Open Failed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeOpenFailedEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstance")]
    pub node_instance: i64,
    #[doc = "Id of Node."]
    #[serde(rename = "NodeId")]
    pub node_id: String,
    #[doc = "Upgrade domain of Node."]
    #[serde(rename = "UpgradeDomain")]
    pub upgrade_domain: String,
    #[doc = "Fault domain of Node."]
    #[serde(rename = "FaultDomain")]
    pub fault_domain: String,
    #[doc = "IP address or FQDN."]
    #[serde(rename = "IpAddressOrFQDN")]
    pub ip_address_or_fqdn: String,
    #[doc = "Name of Host."]
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[doc = "Indicates if it is seed node."]
    #[serde(rename = "IsSeedNode")]
    pub is_seed_node: bool,
    #[doc = "Version of Node."]
    #[serde(rename = "NodeVersion")]
    pub node_version: String,
    #[doc = "Describes the error."]
    #[serde(rename = "Error")]
    pub error: String,
}
impl NodeOpenFailedEvent {
    pub fn new(
        node_event: NodeEvent,
        node_instance: i64,
        node_id: String,
        upgrade_domain: String,
        fault_domain: String,
        ip_address_or_fqdn: String,
        hostname: String,
        is_seed_node: bool,
        node_version: String,
        error: String,
    ) -> Self {
        Self {
            node_event,
            node_instance,
            node_id,
            upgrade_domain,
            fault_domain,
            ip_address_or_fqdn,
            hostname,
            is_seed_node,
            node_version,
            error,
        }
    }
}
#[doc = "Node Opened Succeeded event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeOpenSucceededEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstance")]
    pub node_instance: i64,
    #[doc = "Id of Node."]
    #[serde(rename = "NodeId")]
    pub node_id: String,
    #[doc = "Upgrade domain of Node."]
    #[serde(rename = "UpgradeDomain")]
    pub upgrade_domain: String,
    #[doc = "Fault domain of Node."]
    #[serde(rename = "FaultDomain")]
    pub fault_domain: String,
    #[doc = "IP address or FQDN."]
    #[serde(rename = "IpAddressOrFQDN")]
    pub ip_address_or_fqdn: String,
    #[doc = "Name of Host."]
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[doc = "Indicates if it is seed node."]
    #[serde(rename = "IsSeedNode")]
    pub is_seed_node: bool,
    #[doc = "Version of Node."]
    #[serde(rename = "NodeVersion")]
    pub node_version: String,
}
impl NodeOpenSucceededEvent {
    pub fn new(
        node_event: NodeEvent,
        node_instance: i64,
        node_id: String,
        upgrade_domain: String,
        fault_domain: String,
        ip_address_or_fqdn: String,
        hostname: String,
        is_seed_node: bool,
        node_version: String,
    ) -> Self {
        Self {
            node_event,
            node_instance,
            node_id,
            upgrade_domain,
            fault_domain,
            ip_address_or_fqdn,
            hostname,
            is_seed_node,
            node_version,
        }
    }
}
#[doc = "Node Removed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeRemovedFromClusterEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node."]
    #[serde(rename = "NodeId")]
    pub node_id: String,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstance")]
    pub node_instance: i64,
    #[doc = "Type of Node."]
    #[serde(rename = "NodeType")]
    pub node_type: String,
    #[doc = "Fabric version."]
    #[serde(rename = "FabricVersion")]
    pub fabric_version: String,
    #[doc = "IP address or FQDN."]
    #[serde(rename = "IpAddressOrFQDN")]
    pub ip_address_or_fqdn: String,
    #[doc = "Capacities."]
    #[serde(rename = "NodeCapacities")]
    pub node_capacities: String,
}
impl NodeRemovedFromClusterEvent {
    pub fn new(
        node_event: NodeEvent,
        node_id: String,
        node_instance: i64,
        node_type: String,
        fabric_version: String,
        ip_address_or_fqdn: String,
        node_capacities: String,
    ) -> Self {
        Self {
            node_event,
            node_id,
            node_instance,
            node_type,
            fabric_version,
            ip_address_or_fqdn,
            node_capacities,
        }
    }
}
#[doc = "Describes the expected impact of a repair on a set of nodes.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeRepairImpactDescription {
    #[serde(flatten)]
    pub repair_impact_description_base: RepairImpactDescriptionBase,
    #[doc = "The list of nodes impacted by a repair action and their respective expected impact."]
    #[serde(rename = "NodeImpactList", default, skip_serializing_if = "Vec::is_empty")]
    pub node_impact_list: Vec<NodeImpact>,
}
impl NodeRepairImpactDescription {
    pub fn new(repair_impact_description_base: RepairImpactDescriptionBase) -> Self {
        Self {
            repair_impact_description_base,
            node_impact_list: Vec::new(),
        }
    }
}
#[doc = "Describes the list of nodes targeted by a repair action.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeRepairTargetDescription {
    #[serde(flatten)]
    pub repair_target_description_base: RepairTargetDescriptionBase,
    #[doc = "The list of nodes targeted by a repair action."]
    #[serde(rename = "NodeNames", default, skip_serializing_if = "Vec::is_empty")]
    pub node_names: Vec<String>,
}
impl NodeRepairTargetDescription {
    pub fn new(repair_target_description_base: RepairTargetDescriptionBase) -> Self {
        Self {
            repair_target_description_base,
            node_names: Vec::new(),
        }
    }
}
#[doc = "Contains information about a node that was targeted by a user-induced operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeResult {
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
    #[doc = "The node instance id."]
    #[serde(rename = "NodeInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub node_instance_id: Option<String>,
}
impl NodeResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NodeStatus")]
pub enum NodeStatus {
    Invalid,
    Up,
    Down,
    Enabling,
    Disabling,
    Disabled,
    Unknown,
    Removed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NodeStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NodeStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NodeStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("NodeStatus", 0u32, "Invalid"),
            Self::Up => serializer.serialize_unit_variant("NodeStatus", 1u32, "Up"),
            Self::Down => serializer.serialize_unit_variant("NodeStatus", 2u32, "Down"),
            Self::Enabling => serializer.serialize_unit_variant("NodeStatus", 3u32, "Enabling"),
            Self::Disabling => serializer.serialize_unit_variant("NodeStatus", 4u32, "Disabling"),
            Self::Disabled => serializer.serialize_unit_variant("NodeStatus", 5u32, "Disabled"),
            Self::Unknown => serializer.serialize_unit_variant("NodeStatus", 6u32, "Unknown"),
            Self::Removed => serializer.serialize_unit_variant("NodeStatus", 7u32, "Removed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information about an NodeTransition operation.  This class contains an OperationState and a NodeTransitionResult.  The NodeTransitionResult is not valid until OperationState\nis Completed or Faulted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTransitionProgress {
    #[doc = "The state of the operation."]
    #[serde(rename = "State", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OperationState>,
    #[doc = "Represents information about an operation in a terminal state (Completed or Faulted)."]
    #[serde(rename = "NodeTransitionResult", default, skip_serializing_if = "Option::is_none")]
    pub node_transition_result: Option<NodeTransitionResult>,
}
impl NodeTransitionProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents information about an operation in a terminal state (Completed or Faulted)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTransitionResult {
    #[doc = "If OperationState is Completed, this is 0.  If OperationState is Faulted, this is an error code indicating the reason."]
    #[serde(rename = "ErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "Contains information about a node that was targeted by a user-induced operation."]
    #[serde(rename = "NodeResult", default, skip_serializing_if = "Option::is_none")]
    pub node_result: Option<NodeResult>,
}
impl NodeTransitionResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type NodeType = String;
#[doc = "Node Up event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeUpEvent {
    #[serde(flatten)]
    pub node_event: NodeEvent,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstance")]
    pub node_instance: i64,
    #[doc = "Time when Node was last down."]
    #[serde(rename = "LastNodeDownAt", with = "azure_core::date::rfc3339")]
    pub last_node_down_at: time::OffsetDateTime,
}
impl NodeUpEvent {
    pub fn new(node_event: NodeEvent, node_instance: i64, last_node_down_at: time::OffsetDateTime) -> Self {
        Self {
            node_event,
            node_instance,
            last_node_down_at,
        }
    }
}
#[doc = "The state of the upgrading node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NodeUpgradePhase")]
pub enum NodeUpgradePhase {
    Invalid,
    PreUpgradeSafetyCheck,
    Upgrading,
    PostUpgradeSafetyCheck,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NodeUpgradePhase {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NodeUpgradePhase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NodeUpgradePhase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("NodeUpgradePhase", 0u32, "Invalid"),
            Self::PreUpgradeSafetyCheck => serializer.serialize_unit_variant("NodeUpgradePhase", 1u32, "PreUpgradeSafetyCheck"),
            Self::Upgrading => serializer.serialize_unit_variant("NodeUpgradePhase", 2u32, "Upgrading"),
            Self::PostUpgradeSafetyCheck => serializer.serialize_unit_variant("NodeUpgradePhase", 3u32, "PostUpgradeSafetyCheck"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information about the upgrading node and its status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeUpgradeProgressInfo {
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
    #[doc = "The state of the upgrading node."]
    #[serde(rename = "UpgradePhase", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_phase: Option<NodeUpgradePhase>,
    #[doc = "List of pending safety checks"]
    #[serde(rename = "PendingSafetyChecks", default, skip_serializing_if = "Option::is_none")]
    pub pending_safety_checks: Option<SafetyCheckInfoList>,
}
impl NodeUpgradeProgressInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type NodeUpgradeProgressInfoList = Vec<NodeUpgradeProgressInfo>;
#[doc = "Represents health evaluation for nodes, containing health evaluations for each unhealthy node that impacted current aggregated health state. Can be returned when evaluating cluster health and the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodesHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Maximum allowed percentage of unhealthy nodes from the ClusterHealthPolicy."]
    #[serde(rename = "MaxPercentUnhealthyNodes", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_unhealthy_nodes: Option<i64>,
    #[doc = "Total number of nodes found in the health store."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl NodesHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            max_percent_unhealthy_nodes: None,
            total_count: None,
            unhealthy_evaluations: None,
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
pub type OperationId = String;
#[doc = "The state of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationState")]
pub enum OperationState {
    Invalid,
    Running,
    RollingBack,
    Completed,
    Faulted,
    Cancelled,
    ForceCancelled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("OperationState", 0u32, "Invalid"),
            Self::Running => serializer.serialize_unit_variant("OperationState", 1u32, "Running"),
            Self::RollingBack => serializer.serialize_unit_variant("OperationState", 2u32, "RollingBack"),
            Self::Completed => serializer.serialize_unit_variant("OperationState", 3u32, "Completed"),
            Self::Faulted => serializer.serialize_unit_variant("OperationState", 4u32, "Faulted"),
            Self::Cancelled => serializer.serialize_unit_variant("OperationState", 5u32, "Cancelled"),
            Self::ForceCancelled => serializer.serialize_unit_variant("OperationState", 6u32, "ForceCancelled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Contains the OperationId, OperationState, and OperationType for user-induced operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "A GUID that identifies a call to this API.  This is also passed into the corresponding GetProgress API."]
    #[serde(rename = "OperationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<OperationId>,
    #[doc = "The state of the operation."]
    #[serde(rename = "State", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OperationState>,
    #[doc = "The type of the operation."]
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<OperationType>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type OperationStatusList = Vec<OperationStatus>;
#[doc = "The type of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationType")]
pub enum OperationType {
    Invalid,
    PartitionDataLoss,
    PartitionQuorumLoss,
    PartitionRestart,
    NodeTransition,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("OperationType", 0u32, "Invalid"),
            Self::PartitionDataLoss => serializer.serialize_unit_variant("OperationType", 1u32, "PartitionDataLoss"),
            Self::PartitionQuorumLoss => serializer.serialize_unit_variant("OperationType", 2u32, "PartitionQuorumLoss"),
            Self::PartitionRestart => serializer.serialize_unit_variant("OperationType", 3u32, "PartitionRestart"),
            Self::NodeTransition => serializer.serialize_unit_variant("OperationType", 4u32, "NodeTransition"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a policy for the package sharing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageSharingPolicyInfo {
    #[doc = "The name of code, configuration or data package that should be shared."]
    #[serde(rename = "SharedPackageName", default, skip_serializing_if = "Option::is_none")]
    pub shared_package_name: Option<String>,
    #[doc = "Represents the scope for PackageSharingPolicy. This is specified during DeployServicePackageToNode operation."]
    #[serde(rename = "PackageSharingScope", default, skip_serializing_if = "Option::is_none")]
    pub package_sharing_scope: Option<PackageSharingPolicyScope>,
}
impl PackageSharingPolicyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type PackageSharingPolicyInfoList = Vec<PackageSharingPolicyInfo>;
#[doc = "Represents the scope for PackageSharingPolicy. This is specified during DeployServicePackageToNode operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PackageSharingPolicyScope")]
pub enum PackageSharingPolicyScope {
    None,
    All,
    Code,
    Config,
    Data,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PackageSharingPolicyScope {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PackageSharingPolicyScope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PackageSharingPolicyScope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("PackageSharingPolicyScope", 0u32, "None"),
            Self::All => serializer.serialize_unit_variant("PackageSharingPolicyScope", 1u32, "All"),
            Self::Code => serializer.serialize_unit_variant("PackageSharingPolicyScope", 2u32, "Code"),
            Self::Config => serializer.serialize_unit_variant("PackageSharingPolicyScope", 3u32, "Config"),
            Self::Data => serializer.serialize_unit_variant("PackageSharingPolicyScope", 4u32, "Data"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The list of applications in the cluster. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedApplicationInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of application information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ApplicationInfo>,
}
impl PagedApplicationInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of application resources. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedApplicationResourceDescriptionList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "One page of the list."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ApplicationResourceDescription>,
}
impl PagedApplicationResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of application types that are provisioned or being provisioned in the cluster. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedApplicationTypeInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of application type information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ApplicationTypeInfo>,
}
impl PagedApplicationTypeInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of backup configuration information. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedBackupConfigurationInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of backup configuration information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BackupConfigurationInfo>,
}
impl PagedBackupConfigurationInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of backup entities that are being periodically backed. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedBackupEntityList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of backup entity information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BackupEntity>,
}
impl PagedBackupEntityList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of backups. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedBackupInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of backup information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BackupInfo>,
}
impl PagedBackupInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of backup policies configured in the cluster. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedBackupPolicyDescriptionList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "The list of backup policies information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BackupPolicyDescription>,
}
impl PagedBackupPolicyDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of compose deployments in the cluster. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedComposeDeploymentStatusInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of compose deployment status information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ComposeDeploymentStatusInfo>,
}
impl PagedComposeDeploymentStatusInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of deployed applications in activating, downloading, or active states on a node.\nThe list is paged when all of the results cannot fit in a single message.\nThe next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedDeployedApplicationInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of deployed application information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DeployedApplicationInfo>,
}
impl PagedDeployedApplicationInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of gateway resources. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedGatewayResourceDescriptionList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "One page of the list."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<GatewayResourceDescription>,
}
impl PagedGatewayResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of network resources. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedNetworkResourceDescriptionList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "One page of the list."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<NetworkResourceDescription>,
}
impl PagedNetworkResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of nodes in the cluster. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedNodeInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of node information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<NodeInfo>,
}
impl PagedNodeInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paged list of Service Fabric properties under a given name. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedPropertyInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "Indicates whether any property under the given name has been modified during the enumeration. If there was a modification, this property value is false."]
    #[serde(rename = "IsConsistent", default, skip_serializing_if = "Option::is_none")]
    pub is_consistent: Option<bool>,
    #[doc = "List of property information."]
    #[serde(rename = "Properties", default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<PropertyInfo>,
}
impl PagedPropertyInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of replicas in the cluster for a given partition. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedReplicaInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of replica information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ReplicaInfo>,
}
impl PagedReplicaInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of secret resources. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedSecretResourceDescriptionList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "One page of the list."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SecretResourceDescription>,
}
impl PagedSecretResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of values of a secret resource, paged if the number of results exceeds the limits of a single message. The next set of results can be obtained by executing the same query with the continuation token provided in the previous page."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedSecretValueResourceDescriptionList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "One page of the list."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SecretValueResourceDescription>,
}
impl PagedSecretValueResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of services in the cluster for an application. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedServiceInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of service information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceInfo>,
}
impl PagedServiceInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of partition in the cluster for a service. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedServicePartitionInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of service partition information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServicePartitionInfo>,
}
impl PagedServicePartitionInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of service resource replicas in the cluster. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedServiceReplicaDescriptionList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of service resource replica description."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceReplicaDescription>,
}
impl PagedServiceReplicaDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of service resources. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedServiceResourceDescriptionList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "One page of the list."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceResourceDescription>,
}
impl PagedServiceResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paged list of Service Fabric names. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedSubNameInfoList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "Indicates whether any name under the given name has been modified during the enumeration. If there was a modification, this property value is false."]
    #[serde(rename = "IsConsistent", default, skip_serializing_if = "Option::is_none")]
    pub is_consistent: Option<bool>,
    #[doc = "List of the child names."]
    #[serde(rename = "SubNames", default, skip_serializing_if = "Vec::is_empty")]
    pub sub_names: Vec<FabricName>,
}
impl PagedSubNameInfoList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of results of the call UpdatePartitionLoad. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedUpdatePartitionLoadResultList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "List of partition load update information."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<UpdatePartitionLoadResult>,
}
impl PagedUpdatePartitionLoadResultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of volume resources. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedVolumeResourceDescriptionList {
    #[doc = "The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response."]
    #[serde(rename = "ContinuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<ContinuationToken>,
    #[doc = "One page of the list."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<VolumeResourceDescription>,
}
impl PagedVolumeResourceDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the access status of the partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PartitionAccessStatus")]
pub enum PartitionAccessStatus {
    Invalid,
    Granted,
    ReconfigurationPending,
    NotPrimary,
    NoWriteQuorum,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PartitionAccessStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PartitionAccessStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PartitionAccessStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("PartitionAccessStatus", 0u32, "Invalid"),
            Self::Granted => serializer.serialize_unit_variant("PartitionAccessStatus", 1u32, "Granted"),
            Self::ReconfigurationPending => serializer.serialize_unit_variant("PartitionAccessStatus", 2u32, "ReconfigurationPending"),
            Self::NotPrimary => serializer.serialize_unit_variant("PartitionAccessStatus", 3u32, "NotPrimary"),
            Self::NoWriteQuorum => serializer.serialize_unit_variant("PartitionAccessStatus", 4u32, "NoWriteQuorum"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents the base for all Partition Analysis Events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionAnalysisEvent {
    #[serde(flatten)]
    pub partition_event: PartitionEvent,
    #[doc = "Metadata about an Analysis Event."]
    #[serde(rename = "Metadata")]
    pub metadata: AnalysisEventMetadata,
}
impl PartitionAnalysisEvent {
    pub fn new(partition_event: PartitionEvent, metadata: AnalysisEventMetadata) -> Self {
        Self { partition_event, metadata }
    }
}
#[doc = "Backup configuration information, for a specific partition, specifying what backup policy is being applied and suspend description, if any."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionBackupConfigurationInfo {
    #[serde(flatten)]
    pub backup_configuration_info: BackupConfigurationInfo,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
}
impl PartitionBackupConfigurationInfo {
    pub fn new(backup_configuration_info: BackupConfigurationInfo) -> Self {
        Self {
            backup_configuration_info,
            service_name: None,
            partition_id: None,
        }
    }
}
#[doc = "Identifies the Service Fabric stateful partition which is being backed up."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionBackupEntity {
    #[serde(flatten)]
    pub backup_entity: BackupEntity,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
}
impl PartitionBackupEntity {
    pub fn new(backup_entity: BackupEntity) -> Self {
        Self {
            backup_entity,
            service_name: None,
            partition_id: None,
        }
    }
}
#[doc = "Information about a partition data loss user-induced operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionDataLossProgress {
    #[doc = "The state of the operation."]
    #[serde(rename = "State", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OperationState>,
    #[doc = "Represents information about an operation in a terminal state (Completed or Faulted)."]
    #[serde(rename = "InvokeDataLossResult", default, skip_serializing_if = "Option::is_none")]
    pub invoke_data_loss_result: Option<InvokeDataLossResult>,
}
impl PartitionDataLossProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the base for all Partition Events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionEvent {
    #[serde(flatten)]
    pub fabric_event: FabricEvent,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId")]
    pub partition_id: PartitionId,
}
impl PartitionEvent {
    pub fn new(fabric_event: FabricEvent, partition_id: PartitionId) -> Self {
        Self {
            fabric_event,
            partition_id,
        }
    }
}
pub type PartitionEventList = Vec<PartitionEvent>;
#[doc = "Information about the health of a Service Fabric partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionHealth {
    #[serde(flatten)]
    pub entity_health: EntityHealth,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
    #[doc = "The list of replica health states associated with the partition."]
    #[serde(rename = "ReplicaHealthStates", default, skip_serializing_if = "Vec::is_empty")]
    pub replica_health_states: Vec<ReplicaHealthState>,
}
impl PartitionHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents health evaluation for a partition, containing information about the data and the algorithm used by health store to evaluate health. The evaluation is returned only when the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl PartitionHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            partition_id: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Partition Health Report Expired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionHealthReportExpiredEvent {
    #[serde(flatten)]
    pub partition_event: PartitionEvent,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl PartitionHealthReportExpiredEvent {
    pub fn new(
        partition_event: PartitionEvent,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            partition_event,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Represents the health state of a partition, which contains the partition identifier and its aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionHealthState {
    #[serde(flatten)]
    pub entity_health_state: EntityHealthState,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
}
impl PartitionHealthState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the health state chunk of a partition, which contains the partition ID, its aggregated health state and any replicas that respect the filters in the cluster health chunk query description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionHealthStateChunk {
    #[serde(flatten)]
    pub entity_health_state_chunk: EntityHealthStateChunk,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
    #[doc = "The list of replica health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
    #[serde(rename = "ReplicaHealthStateChunks", default, skip_serializing_if = "Option::is_none")]
    pub replica_health_state_chunks: Option<ReplicaHealthStateChunkList>,
}
impl PartitionHealthStateChunk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of partition health state chunks that respect the input filters in the chunk query description.\nReturned by get cluster health state chunks query as part of the parent application hierarchy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionHealthStateChunkList {
    #[doc = "The list of partition health state chunks that respect the input filters in the chunk query."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PartitionHealthStateChunk>,
}
impl PartitionHealthStateChunkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines matching criteria to determine whether a partition should be included as a child of a service in the cluster health chunk.\nThe partitions are only returned if the parent entities match a filter specified in the cluster health chunk query description. The parent service and application must be included in the cluster health chunk.\nOne filter can match zero, one or multiple partitions, depending on its properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionHealthStateFilter {
    #[doc = "ID of the partition that matches the filter. The filter is applied only to the specified partition, if it exists.\nIf the partition doesn't exist, no partition is returned in the cluster health chunk based on this filter.\nIf the partition exists, it is included in the cluster health chunk if it respects the other filter properties.\nIf not specified, all partitions that match the parent filters (if any) are taken into consideration and matched against the other filter members, like health state filter."]
    #[serde(rename = "PartitionIdFilter", default, skip_serializing_if = "Option::is_none")]
    pub partition_id_filter: Option<String>,
    #[doc = "The filter for the health state of the partitions. It allows selecting partitions if they match the desired health states.\nThe possible values are integer value of one of the following health states. Only partitions that match the filter are returned. All partitions are used to evaluate the cluster aggregated health state.\nIf not specified, default value is None, unless the partition ID is specified. If the filter has default value and partition ID is specified, the matching partition is returned.\nThe state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise 'OR' operator.\nFor example, if the provided value is 6, it matches partitions with HealthState value of OK (2) and Warning (4).\n\n- Default - Default value. Matches any HealthState. The value is zero.\n- None - Filter that doesn't match any HealthState value. Used in order to return no results on a given collection of states. The value is 1.\n- Ok - Filter that matches input with HealthState value Ok. The value is 2.\n- Warning - Filter that matches input with HealthState value Warning. The value is 4.\n- Error - Filter that matches input with HealthState value Error. The value is 8.\n- All - Filter that matches input with any HealthState value. The value is 65535."]
    #[serde(rename = "HealthStateFilter", default, skip_serializing_if = "Option::is_none")]
    pub health_state_filter: Option<i64>,
    #[doc = "Defines a list of filters that specify which replicas to be included in the returned cluster health chunk as children of the parent partition. The replicas are returned only if the parent partition matches a filter.\nIf the list is empty, no replicas are returned. All the replicas are used to evaluate the parent partition aggregated health state, regardless of the input filters.\nThe partition filter may specify multiple replica filters.\nFor example, it can specify a filter to return all replicas with health state Error and another filter to always include a replica identified by its replica id."]
    #[serde(rename = "ReplicaFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub replica_filters: Vec<ReplicaHealthStateFilter>,
}
impl PartitionHealthStateFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type PartitionId = String;
#[doc = "Information about the partition identity, partitioning scheme and keys supported by it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionInformation {
    #[doc = "The kind of partitioning scheme used to partition the service."]
    #[serde(rename = "ServicePartitionKind")]
    pub service_partition_kind: ServicePartitionKind,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PartitionId>,
}
impl PartitionInformation {
    pub fn new(service_partition_kind: ServicePartitionKind) -> Self {
        Self {
            service_partition_kind,
            id: None,
        }
    }
}
#[doc = "Represents a scaling mechanism for adding or removing instances of stateless service partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionInstanceCountScaleMechanism {
    #[serde(flatten)]
    pub scaling_mechanism_description: ScalingMechanismDescription,
    #[doc = "Minimum number of instances of the partition."]
    #[serde(rename = "MinInstanceCount")]
    pub min_instance_count: i64,
    #[doc = "Maximum number of instances of the partition."]
    #[serde(rename = "MaxInstanceCount")]
    pub max_instance_count: i64,
    #[doc = "The number of instances to add or remove during a scaling operation."]
    #[serde(rename = "ScaleIncrement")]
    pub scale_increment: i64,
}
impl PartitionInstanceCountScaleMechanism {
    pub fn new(
        scaling_mechanism_description: ScalingMechanismDescription,
        min_instance_count: i64,
        max_instance_count: i64,
        scale_increment: i64,
    ) -> Self {
        Self {
            scaling_mechanism_description,
            min_instance_count,
            max_instance_count,
            scale_increment,
        }
    }
}
#[doc = "Represents load information for a partition, which contains the primary and secondary reported load metrics.\nIn case there is no load reported, PartitionLoadInformation will contain the default load for the service of the partition.\nFor default loads, LoadMetricReport's LastReportedUtc is set to 0."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionLoadInformation {
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
    #[doc = "Array of load reports from the primary replica for this partition."]
    #[serde(rename = "PrimaryLoadMetricReports", default, skip_serializing_if = "Vec::is_empty")]
    pub primary_load_metric_reports: Vec<LoadMetricReport>,
    #[doc = "Array of aggregated load reports from all secondary replicas for this partition.\nArray only contains the latest reported load for each metric."]
    #[serde(rename = "SecondaryLoadMetricReports", default, skip_serializing_if = "Vec::is_empty")]
    pub secondary_load_metric_reports: Vec<LoadMetricReport>,
}
impl PartitionLoadInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents load information for a partition, which contains the metrics load information about primary, all secondary replicas/instances or a specific secondary replica/instance located on a specific node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionMetricLoadDescription {
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
    #[doc = "Partition's load information for primary replica, in case partition is from a stateful service."]
    #[serde(rename = "PrimaryReplicaLoadEntries", default, skip_serializing_if = "Vec::is_empty")]
    pub primary_replica_load_entries: Vec<MetricLoadDescription>,
    #[doc = "Partition's load information for all secondary replicas or instances."]
    #[serde(rename = "SecondaryReplicasOrInstancesLoadEntries", default, skip_serializing_if = "Vec::is_empty")]
    pub secondary_replicas_or_instances_load_entries: Vec<MetricLoadDescription>,
    #[doc = "Partition's load information for a specific secondary replica or instance located on a specific node."]
    #[serde(
        rename = "SecondaryReplicaOrInstanceLoadEntriesPerNode",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub secondary_replica_or_instance_load_entries_per_node: Vec<ReplicaMetricLoadDescription>,
}
impl PartitionMetricLoadDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type PartitionMetricLoadDescriptionList = Vec<PartitionMetricLoadDescription>;
#[doc = "Partition Health Report Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionNewHealthReportEvent {
    #[serde(flatten)]
    pub partition_event: PartitionEvent,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl PartitionNewHealthReportEvent {
    pub fn new(
        partition_event: PartitionEvent,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            partition_event,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Partition Primary Move Analysis event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionPrimaryMoveAnalysisEvent {
    #[serde(flatten)]
    pub partition_analysis_event: PartitionAnalysisEvent,
    #[doc = "Time when the move was completed."]
    #[serde(rename = "WhenMoveCompleted", with = "azure_core::date::rfc3339")]
    pub when_move_completed: time::OffsetDateTime,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "PreviousNode")]
    pub previous_node: NodeName,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "CurrentNode")]
    pub current_node: NodeName,
    #[doc = "Move reason."]
    #[serde(rename = "MoveReason")]
    pub move_reason: String,
    #[doc = "Relevant traces."]
    #[serde(rename = "RelevantTraces")]
    pub relevant_traces: String,
}
impl PartitionPrimaryMoveAnalysisEvent {
    pub fn new(
        partition_analysis_event: PartitionAnalysisEvent,
        when_move_completed: time::OffsetDateTime,
        previous_node: NodeName,
        current_node: NodeName,
        move_reason: String,
        relevant_traces: String,
    ) -> Self {
        Self {
            partition_analysis_event,
            when_move_completed,
            previous_node,
            current_node,
            move_reason,
            relevant_traces,
        }
    }
}
#[doc = "Information about a partition quorum loss user-induced operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionQuorumLossProgress {
    #[doc = "The state of the operation."]
    #[serde(rename = "State", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OperationState>,
    #[doc = "Represents information about an operation in a terminal state (Completed or Faulted)."]
    #[serde(rename = "InvokeQuorumLossResult", default, skip_serializing_if = "Option::is_none")]
    pub invoke_quorum_loss_result: Option<InvokeQuorumLossResult>,
}
impl PartitionQuorumLossProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Partition Reconfiguration event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionReconfiguredEvent {
    #[serde(flatten)]
    pub partition_event: PartitionEvent,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName")]
    pub node_name: NodeName,
    #[doc = "Id of Node instance."]
    #[serde(rename = "NodeInstanceId")]
    pub node_instance_id: String,
    #[doc = "Type of Service."]
    #[serde(rename = "ServiceType")]
    pub service_type: String,
    #[doc = "CcEpochDataLoss version."]
    #[serde(rename = "CcEpochDataLossVersion")]
    pub cc_epoch_data_loss_version: i64,
    #[doc = "CcEpochConfig version."]
    #[serde(rename = "CcEpochConfigVersion")]
    pub cc_epoch_config_version: i64,
    #[doc = "Type of reconfiguration."]
    #[serde(rename = "ReconfigType")]
    pub reconfig_type: String,
    #[doc = "Describes reconfiguration result."]
    #[serde(rename = "Result")]
    pub result: String,
    #[doc = "Duration of Phase0 in milli-seconds."]
    #[serde(rename = "Phase0DurationMs")]
    pub phase0_duration_ms: f64,
    #[doc = "Duration of Phase1 in milli-seconds."]
    #[serde(rename = "Phase1DurationMs")]
    pub phase1_duration_ms: f64,
    #[doc = "Duration of Phase2 in milli-seconds."]
    #[serde(rename = "Phase2DurationMs")]
    pub phase2_duration_ms: f64,
    #[doc = "Duration of Phase3 in milli-seconds."]
    #[serde(rename = "Phase3DurationMs")]
    pub phase3_duration_ms: f64,
    #[doc = "Duration of Phase4 in milli-seconds."]
    #[serde(rename = "Phase4DurationMs")]
    pub phase4_duration_ms: f64,
    #[doc = "Total duration in milli-seconds."]
    #[serde(rename = "TotalDurationMs")]
    pub total_duration_ms: f64,
}
impl PartitionReconfiguredEvent {
    pub fn new(
        partition_event: PartitionEvent,
        node_name: NodeName,
        node_instance_id: String,
        service_type: String,
        cc_epoch_data_loss_version: i64,
        cc_epoch_config_version: i64,
        reconfig_type: String,
        result: String,
        phase0_duration_ms: f64,
        phase1_duration_ms: f64,
        phase2_duration_ms: f64,
        phase3_duration_ms: f64,
        phase4_duration_ms: f64,
        total_duration_ms: f64,
    ) -> Self {
        Self {
            partition_event,
            node_name,
            node_instance_id,
            service_type,
            cc_epoch_data_loss_version,
            cc_epoch_config_version,
            reconfig_type,
            result,
            phase0_duration_ms,
            phase1_duration_ms,
            phase2_duration_ms,
            phase3_duration_ms,
            phase4_duration_ms,
            total_duration_ms,
        }
    }
}
#[doc = "Information about a partition restart user-induced operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionRestartProgress {
    #[doc = "The state of the operation."]
    #[serde(rename = "State", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OperationState>,
    #[doc = "Represents information about an operation in a terminal state (Completed or Faulted)."]
    #[serde(rename = "RestartPartitionResult", default, skip_serializing_if = "Option::is_none")]
    pub restart_partition_result: Option<RestartPartitionResult>,
}
impl PartitionRestartProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a safety check for the service partition being performed by service fabric before continuing with operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionSafetyCheck {
    #[serde(flatten)]
    pub safety_check: SafetyCheck,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
}
impl PartitionSafetyCheck {
    pub fn new(safety_check: SafetyCheck) -> Self {
        Self {
            safety_check,
            partition_id: None,
        }
    }
}
#[doc = "Enumerates the ways that a service can be partitioned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PartitionScheme")]
pub enum PartitionScheme {
    Invalid,
    Singleton,
    UniformInt64Range,
    Named,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PartitionScheme {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PartitionScheme {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PartitionScheme {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("PartitionScheme", 0u32, "Invalid"),
            Self::Singleton => serializer.serialize_unit_variant("PartitionScheme", 1u32, "Singleton"),
            Self::UniformInt64Range => serializer.serialize_unit_variant("PartitionScheme", 2u32, "UniformInt64Range"),
            Self::Named => serializer.serialize_unit_variant("PartitionScheme", 3u32, "Named"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes how the service is partitioned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionSchemeDescription {
    #[doc = "Enumerates the ways that a service can be partitioned."]
    #[serde(rename = "PartitionScheme")]
    pub partition_scheme: PartitionScheme,
}
impl PartitionSchemeDescription {
    pub fn new(partition_scheme: PartitionScheme) -> Self {
        Self { partition_scheme }
    }
}
#[doc = "Represents health evaluation for the partitions of a service, containing health evaluations for each unhealthy partition that impacts current aggregated health state. Can be returned when evaluating service health and the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionsHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Maximum allowed percentage of unhealthy partitions per service from the ServiceTypeHealthPolicy."]
    #[serde(
        rename = "MaxPercentUnhealthyPartitionsPerService",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_percent_unhealthy_partitions_per_service: Option<i64>,
    #[doc = "Total number of partitions of the service from the health store."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl PartitionsHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            max_percent_unhealthy_partitions_per_service: None,
            total_count: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Provides statistics about the Service Fabric Replicator, when it is functioning in a Primary role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrimaryReplicatorStatus {
    #[serde(flatten)]
    pub replicator_status: ReplicatorStatus,
    #[doc = "Provides various statistics of the queue used in the service fabric replicator.\nContains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc.\nDepending on the role of the replicator, the properties in this type imply different meanings."]
    #[serde(rename = "ReplicationQueueStatus", default, skip_serializing_if = "Option::is_none")]
    pub replication_queue_status: Option<ReplicatorQueueStatus>,
    #[doc = "List of remote replicator status"]
    #[serde(rename = "RemoteReplicators", default, skip_serializing_if = "Option::is_none")]
    pub remote_replicators: Option<RemoteReplicatorStatusList>,
}
impl PrimaryReplicatorStatus {
    pub fn new(replicator_status: ReplicatorStatus) -> Self {
        Self {
            replicator_status,
            replication_queue_status: None,
            remote_replicators: None,
        }
    }
}
#[doc = "Probes have a number of fields that you can use to control their behavior."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Probe {
    #[doc = "The initial delay in seconds to start executing probe once codepackage has started."]
    #[serde(rename = "initialDelaySeconds", default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i64>,
    #[doc = "Periodic seconds to execute probe."]
    #[serde(rename = "periodSeconds", default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i64>,
    #[doc = "Period after which probe is considered as failed if it hasn't completed successfully."]
    #[serde(rename = "timeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
    #[doc = "The count of successful probe executions after which probe is considered success."]
    #[serde(rename = "successThreshold", default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i64>,
    #[doc = "The count of failures after which probe is considered failed."]
    #[serde(rename = "failureThreshold", default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i64>,
    #[doc = "Exec command to run inside the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ProbeExec>,
    #[doc = "Http probe for the container."]
    #[serde(rename = "httpGet", default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<ProbeHttpGet>,
    #[doc = "Tcp port to probe inside the container."]
    #[serde(rename = "tcpSocket", default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<ProbeTcpSocket>,
}
impl Probe {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Exec command to run inside the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProbeExec {
    #[doc = "Comma separated command to run inside the container for example \"sh, -c, echo hello world\"."]
    pub command: String,
}
impl ProbeExec {
    pub fn new(command: String) -> Self {
        Self { command }
    }
}
#[doc = "Http probe for the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProbeHttpGet {
    #[doc = "Port to access for probe."]
    pub port: i64,
    #[doc = "Path to access on the HTTP request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Host IP to connect to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "Headers to set in the request."]
    #[serde(rename = "httpHeaders", default, skip_serializing_if = "Vec::is_empty")]
    pub http_headers: Vec<ProbeHttpGetHeaders>,
    #[doc = "Scheme for the http probe. Can be Http or Https."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<probe_http_get::Scheme>,
}
impl ProbeHttpGet {
    pub fn new(port: i64) -> Self {
        Self {
            port,
            path: None,
            host: None,
            http_headers: Vec::new(),
            scheme: None,
        }
    }
}
pub mod probe_http_get {
    use super::*;
    #[doc = "Scheme for the http probe. Can be Http or Https."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Scheme")]
    pub enum Scheme {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "https")]
        Https,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Scheme {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Scheme {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Scheme {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("Scheme", 0u32, "http"),
                Self::Https => serializer.serialize_unit_variant("Scheme", 1u32, "https"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Http headers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProbeHttpGetHeaders {
    #[doc = "The name of the header."]
    pub name: String,
    #[doc = "The value of the header."]
    pub value: String,
}
impl ProbeHttpGetHeaders {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "Tcp port to probe inside the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProbeTcpSocket {
    #[doc = "Port to access for probe."]
    pub port: i64,
}
impl ProbeTcpSocket {
    pub fn new(port: i64) -> Self {
        Self { port }
    }
}
#[doc = "Describes a list of property batch operations to be executed. Either all or none of the operations will be committed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PropertyBatchDescriptionList {
    #[doc = "A list of the property batch operations to be executed."]
    #[serde(rename = "Operations", default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<PropertyBatchOperation>,
}
impl PropertyBatchDescriptionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the results of a property batch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyBatchInfo {
    #[doc = "The kind of property batch info, determined by the results of a property batch. The following are the possible values."]
    #[serde(rename = "Kind")]
    pub kind: PropertyBatchInfoKind,
}
impl PropertyBatchInfo {
    pub fn new(kind: PropertyBatchInfoKind) -> Self {
        Self { kind }
    }
}
#[doc = "The kind of property batch info, determined by the results of a property batch. The following are the possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PropertyBatchInfoKind")]
pub enum PropertyBatchInfoKind {
    Invalid,
    Successful,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PropertyBatchInfoKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PropertyBatchInfoKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PropertyBatchInfoKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("PropertyBatchInfoKind", 0u32, "Invalid"),
            Self::Successful => serializer.serialize_unit_variant("PropertyBatchInfoKind", 1u32, "Successful"),
            Self::Failed => serializer.serialize_unit_variant("PropertyBatchInfoKind", 2u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents the base type for property operations that can be put into a batch and submitted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyBatchOperation {
    #[doc = "The kind of property batch operation, determined by the operation to be performed. The following are the possible values."]
    #[serde(rename = "Kind")]
    pub kind: PropertyBatchOperationKind,
    #[doc = "The name of the Service Fabric property."]
    #[serde(rename = "PropertyName")]
    pub property_name: PropertyName,
}
impl PropertyBatchOperation {
    pub fn new(kind: PropertyBatchOperationKind, property_name: PropertyName) -> Self {
        Self { kind, property_name }
    }
}
#[doc = "The kind of property batch operation, determined by the operation to be performed. The following are the possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PropertyBatchOperationKind")]
pub enum PropertyBatchOperationKind {
    Invalid,
    Put,
    Get,
    CheckExists,
    CheckSequence,
    Delete,
    CheckValue,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PropertyBatchOperationKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PropertyBatchOperationKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PropertyBatchOperationKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("PropertyBatchOperationKind", 0u32, "Invalid"),
            Self::Put => serializer.serialize_unit_variant("PropertyBatchOperationKind", 1u32, "Put"),
            Self::Get => serializer.serialize_unit_variant("PropertyBatchOperationKind", 2u32, "Get"),
            Self::CheckExists => serializer.serialize_unit_variant("PropertyBatchOperationKind", 3u32, "CheckExists"),
            Self::CheckSequence => serializer.serialize_unit_variant("PropertyBatchOperationKind", 4u32, "CheckSequence"),
            Self::Delete => serializer.serialize_unit_variant("PropertyBatchOperationKind", 5u32, "Delete"),
            Self::CheckValue => serializer.serialize_unit_variant("PropertyBatchOperationKind", 6u32, "CheckValue"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type PropertyCustomTypeId = String;
#[doc = "Description of a Service Fabric property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyDescription {
    #[doc = "The name of the Service Fabric property."]
    #[serde(rename = "PropertyName")]
    pub property_name: PropertyName,
    #[doc = "The property's custom type ID. Using this property, the user is able to tag the type of the value of the property."]
    #[serde(rename = "CustomTypeId", default, skip_serializing_if = "Option::is_none")]
    pub custom_type_id: Option<PropertyCustomTypeId>,
    #[doc = "Describes a Service Fabric property value."]
    #[serde(rename = "Value")]
    pub value: PropertyValue,
}
impl PropertyDescription {
    pub fn new(property_name: PropertyName, value: PropertyValue) -> Self {
        Self {
            property_name,
            custom_type_id: None,
            value,
        }
    }
}
#[doc = "Information about a Service Fabric property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyInfo {
    #[doc = "The name of the Service Fabric property."]
    #[serde(rename = "Name")]
    pub name: PropertyName,
    #[doc = "Describes a Service Fabric property value."]
    #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<PropertyValue>,
    #[doc = "The metadata associated with a property, including the property's name."]
    #[serde(rename = "Metadata")]
    pub metadata: PropertyMetadata,
}
impl PropertyInfo {
    pub fn new(name: PropertyName, metadata: PropertyMetadata) -> Self {
        Self {
            name,
            value: None,
            metadata,
        }
    }
}
#[doc = "The metadata associated with a property, including the property's name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PropertyMetadata {
    #[doc = "The kind of property, determined by the type of data. Following are the possible values."]
    #[serde(rename = "TypeId", default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<PropertyValueKind>,
    #[doc = "The property's custom type ID. Using this property, the user is able to tag the type of the value of the property."]
    #[serde(rename = "CustomTypeId", default, skip_serializing_if = "Option::is_none")]
    pub custom_type_id: Option<PropertyCustomTypeId>,
    #[doc = "The Service Fabric name, including the 'fabric:' URI scheme."]
    #[serde(rename = "Parent", default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<FabricName>,
    #[doc = "The length of the serialized property value."]
    #[serde(rename = "SizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[doc = "Represents when the Property was last modified. Only write operations will cause this field to be updated."]
    #[serde(rename = "LastModifiedUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The version of the property. Every time a property is modified, its sequence number is increased."]
    #[serde(rename = "SequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}
impl PropertyMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type PropertyName = String;
#[doc = "Describes a Service Fabric property value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyValue {
    #[doc = "The kind of property, determined by the type of data. Following are the possible values."]
    #[serde(rename = "Kind")]
    pub kind: PropertyValueKind,
}
impl PropertyValue {
    pub fn new(kind: PropertyValueKind) -> Self {
        Self { kind }
    }
}
#[doc = "The kind of property, determined by the type of data. Following are the possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PropertyValueKind")]
pub enum PropertyValueKind {
    Invalid,
    Binary,
    Int64,
    Double,
    String,
    Guid,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PropertyValueKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PropertyValueKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PropertyValueKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("PropertyValueKind", 0u32, "Invalid"),
            Self::Binary => serializer.serialize_unit_variant("PropertyValueKind", 1u32, "Binary"),
            Self::Int64 => serializer.serialize_unit_variant("PropertyValueKind", 2u32, "Int64"),
            Self::Double => serializer.serialize_unit_variant("PropertyValueKind", 3u32, "Double"),
            Self::String => serializer.serialize_unit_variant("PropertyValueKind", 4u32, "String"),
            Self::Guid => serializer.serialize_unit_variant("PropertyValueKind", 5u32, "Guid"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the operation to register or provision an application type using an application package uploaded to the Service Fabric image store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisionApplicationTypeDescription {
    #[serde(flatten)]
    pub provision_application_type_description_base: ProvisionApplicationTypeDescriptionBase,
    #[doc = "The relative path for the application package in the image store specified during the prior upload operation."]
    #[serde(rename = "ApplicationTypeBuildPath")]
    pub application_type_build_path: String,
    #[doc = "The kind of action that needs to be taken for cleaning up the application package after successful provision."]
    #[serde(rename = "ApplicationPackageCleanupPolicy", default, skip_serializing_if = "Option::is_none")]
    pub application_package_cleanup_policy: Option<ApplicationPackageCleanupPolicy>,
}
impl ProvisionApplicationTypeDescription {
    pub fn new(
        provision_application_type_description_base: ProvisionApplicationTypeDescriptionBase,
        application_type_build_path: String,
    ) -> Self {
        Self {
            provision_application_type_description_base,
            application_type_build_path,
            application_package_cleanup_policy: None,
        }
    }
}
#[doc = "Represents the type of registration or provision requested, and if the operation needs to be asynchronous or not. Supported types of provision operations are from either image store or external store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisionApplicationTypeDescriptionBase {
    #[doc = "The kind of application type registration or provision requested. The application package can be registered or provisioned either from the image store or from an external store. Following are the kinds of the application type provision."]
    #[serde(rename = "Kind")]
    pub kind: ProvisionApplicationTypeKind,
    #[doc = "Indicates whether or not provisioning should occur asynchronously. When set to true, the provision operation returns when the request is accepted by the system, and the provision operation continues without any timeout limit. The default value is false. For large application packages, we recommend setting the value to true."]
    #[serde(rename = "Async")]
    pub async_: bool,
}
impl ProvisionApplicationTypeDescriptionBase {
    pub fn new(kind: ProvisionApplicationTypeKind, async_: bool) -> Self {
        Self { kind, async_ }
    }
}
#[doc = "The kind of application type registration or provision requested. The application package can be registered or provisioned either from the image store or from an external store. Following are the kinds of the application type provision."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisionApplicationTypeKind")]
pub enum ProvisionApplicationTypeKind {
    Invalid,
    ImageStorePath,
    ExternalStore,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProvisionApplicationTypeKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProvisionApplicationTypeKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProvisionApplicationTypeKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ProvisionApplicationTypeKind", 0u32, "Invalid"),
            Self::ImageStorePath => serializer.serialize_unit_variant("ProvisionApplicationTypeKind", 1u32, "ImageStorePath"),
            Self::ExternalStore => serializer.serialize_unit_variant("ProvisionApplicationTypeKind", 2u32, "ExternalStore"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the parameters for provisioning a cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisionFabricDescription {
    #[doc = "The cluster code package file path."]
    #[serde(rename = "CodeFilePath", default, skip_serializing_if = "Option::is_none")]
    pub code_file_path: Option<String>,
    #[doc = "The cluster manifest file path."]
    #[serde(rename = "ClusterManifestFilePath", default, skip_serializing_if = "Option::is_none")]
    pub cluster_manifest_file_path: Option<String>,
}
impl ProvisionFabricDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Puts the specified property under the specified name.\nNote that if one PropertyBatchOperation in a PropertyBatch fails,\nthe entire batch fails and cannot be committed in a transactional manner."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutPropertyBatchOperation {
    #[serde(flatten)]
    pub property_batch_operation: PropertyBatchOperation,
    #[doc = "Describes a Service Fabric property value."]
    #[serde(rename = "Value")]
    pub value: PropertyValue,
    #[doc = "The property's custom type ID. Using this property, the user is able to tag the type of the value of the property."]
    #[serde(rename = "CustomTypeId", default, skip_serializing_if = "Option::is_none")]
    pub custom_type_id: Option<PropertyCustomTypeId>,
}
impl PutPropertyBatchOperation {
    pub fn new(property_batch_operation: PropertyBatchOperation, value: PropertyValue) -> Self {
        Self {
            property_batch_operation,
            value,
            custom_type_id: None,
        }
    }
}
#[doc = "Information about current reconfiguration like phase, type, previous configuration role of replica and reconfiguration start date time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReconfigurationInformation {
    #[doc = "The role of a replica of a stateful service."]
    #[serde(rename = "PreviousConfigurationRole", default, skip_serializing_if = "Option::is_none")]
    pub previous_configuration_role: Option<ReplicaRole>,
    #[doc = "The reconfiguration phase of a replica of a stateful service."]
    #[serde(rename = "ReconfigurationPhase", default, skip_serializing_if = "Option::is_none")]
    pub reconfiguration_phase: Option<ReconfigurationPhase>,
    #[doc = "The type of reconfiguration for replica of a stateful service."]
    #[serde(rename = "ReconfigurationType", default, skip_serializing_if = "Option::is_none")]
    pub reconfiguration_type: Option<ReconfigurationType>,
    #[doc = "Start time (in UTC) of the ongoing reconfiguration. If no reconfiguration is taking place then this value will be zero date-time."]
    #[serde(rename = "ReconfigurationStartTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub reconfiguration_start_time_utc: Option<time::OffsetDateTime>,
}
impl ReconfigurationInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The reconfiguration phase of a replica of a stateful service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReconfigurationPhase")]
pub enum ReconfigurationPhase {
    Unknown,
    None,
    Phase0,
    Phase1,
    Phase2,
    Phase3,
    Phase4,
    AbortPhaseZero,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReconfigurationPhase {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReconfigurationPhase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReconfigurationPhase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ReconfigurationPhase", 0u32, "Unknown"),
            Self::None => serializer.serialize_unit_variant("ReconfigurationPhase", 1u32, "None"),
            Self::Phase0 => serializer.serialize_unit_variant("ReconfigurationPhase", 2u32, "Phase0"),
            Self::Phase1 => serializer.serialize_unit_variant("ReconfigurationPhase", 3u32, "Phase1"),
            Self::Phase2 => serializer.serialize_unit_variant("ReconfigurationPhase", 4u32, "Phase2"),
            Self::Phase3 => serializer.serialize_unit_variant("ReconfigurationPhase", 5u32, "Phase3"),
            Self::Phase4 => serializer.serialize_unit_variant("ReconfigurationPhase", 6u32, "Phase4"),
            Self::AbortPhaseZero => serializer.serialize_unit_variant("ReconfigurationPhase", 7u32, "AbortPhaseZero"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of reconfiguration for replica of a stateful service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReconfigurationType")]
pub enum ReconfigurationType {
    Unknown,
    SwapPrimary,
    Failover,
    Other,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReconfigurationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReconfigurationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReconfigurationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ReconfigurationType", 0u32, "Unknown"),
            Self::SwapPrimary => serializer.serialize_unit_variant("ReconfigurationType", 1u32, "SwapPrimary"),
            Self::Failover => serializer.serialize_unit_variant("ReconfigurationType", 2u32, "Failover"),
            Self::Other => serializer.serialize_unit_variant("ReconfigurationType", 3u32, "Other"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Credential information to connect to container registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryCredential {
    #[doc = "The user name to connect to container registry."]
    #[serde(rename = "RegistryUserName", default, skip_serializing_if = "Option::is_none")]
    pub registry_user_name: Option<String>,
    #[doc = "The password for supplied username to connect to container registry."]
    #[serde(rename = "RegistryPassword", default, skip_serializing_if = "Option::is_none")]
    pub registry_password: Option<String>,
    #[doc = "Indicates that supplied container registry password is encrypted."]
    #[serde(rename = "PasswordEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub password_encrypted: Option<bool>,
}
impl RegistryCredential {
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
#[doc = "Provides various statistics of the acknowledgements that are being received from the remote replicator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoteReplicatorAcknowledgementDetail {
    #[doc = "Represents the average duration it takes for the remote replicator to receive an operation."]
    #[serde(rename = "AverageReceiveDuration", default, skip_serializing_if = "Option::is_none")]
    pub average_receive_duration: Option<String>,
    #[doc = "Represents the average duration it takes for the remote replicator to apply an operation. This usually entails writing the operation to disk."]
    #[serde(rename = "AverageApplyDuration", default, skip_serializing_if = "Option::is_none")]
    pub average_apply_duration: Option<String>,
    #[doc = "Represents the number of operations not yet received by a remote replicator."]
    #[serde(rename = "NotReceivedCount", default, skip_serializing_if = "Option::is_none")]
    pub not_received_count: Option<String>,
    #[doc = "Represents the number of operations received and not yet applied by a remote replicator."]
    #[serde(rename = "ReceivedAndNotAppliedCount", default, skip_serializing_if = "Option::is_none")]
    pub received_and_not_applied_count: Option<String>,
}
impl RemoteReplicatorAcknowledgementDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides details about the remote replicators from the primary replicator's point of view."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoteReplicatorAcknowledgementStatus {
    #[doc = "Provides various statistics of the acknowledgements that are being received from the remote replicator."]
    #[serde(
        rename = "ReplicationStreamAcknowledgementDetail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replication_stream_acknowledgement_detail: Option<RemoteReplicatorAcknowledgementDetail>,
    #[doc = "Provides various statistics of the acknowledgements that are being received from the remote replicator."]
    #[serde(rename = "CopyStreamAcknowledgementDetail", default, skip_serializing_if = "Option::is_none")]
    pub copy_stream_acknowledgement_detail: Option<RemoteReplicatorAcknowledgementDetail>,
}
impl RemoteReplicatorAcknowledgementStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the state of the secondary replicator from the primary replicator’s point of view."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoteReplicatorStatus {
    #[doc = "Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id."]
    #[serde(rename = "ReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<ReplicaId>,
    #[doc = "The last timestamp (in UTC) when an acknowledgement from the secondary replicator was processed on the primary.\nUTC 0 represents an invalid value, indicating that no acknowledgement messages were ever processed."]
    #[serde(rename = "LastAcknowledgementProcessedTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_acknowledgement_processed_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The highest replication operation sequence number that the secondary has received from the primary."]
    #[serde(rename = "LastReceivedReplicationSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub last_received_replication_sequence_number: Option<String>,
    #[doc = "The highest replication operation sequence number that the secondary has applied to its state."]
    #[serde(rename = "LastAppliedReplicationSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub last_applied_replication_sequence_number: Option<String>,
    #[doc = "A value that indicates whether the secondary replica is in the process of being built."]
    #[serde(rename = "IsInBuild", default, skip_serializing_if = "Option::is_none")]
    pub is_in_build: Option<bool>,
    #[doc = "The highest copy operation sequence number that the secondary has received from the primary.\nA value of -1 implies that the secondary has received all copy operations."]
    #[serde(rename = "LastReceivedCopySequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub last_received_copy_sequence_number: Option<String>,
    #[doc = "The highest copy operation sequence number that the secondary has applied to its state.\nA value of -1 implies that the secondary has applied all copy operations and the copy process is complete."]
    #[serde(rename = "LastAppliedCopySequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub last_applied_copy_sequence_number: Option<String>,
    #[doc = "Provides details about the remote replicators from the primary replicator's point of view."]
    #[serde(rename = "RemoteReplicatorAcknowledgementStatus", default, skip_serializing_if = "Option::is_none")]
    pub remote_replicator_acknowledgement_status: Option<RemoteReplicatorAcknowledgementStatus>,
}
impl RemoteReplicatorStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type RemoteReplicatorStatusList = Vec<RemoteReplicatorStatus>;
#[doc = "Describes the expected impact of executing a repair task.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepairImpactDescriptionBase {
    #[doc = "Specifies the kind of the impact. This type supports the Service Fabric platform; it is not meant to be used directly from your code.'"]
    #[serde(rename = "Kind")]
    pub kind: RepairImpactKind,
}
impl RepairImpactDescriptionBase {
    pub fn new(kind: RepairImpactKind) -> Self {
        Self { kind }
    }
}
#[doc = "Specifies the kind of the impact. This type supports the Service Fabric platform; it is not meant to be used directly from your code.'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RepairImpactKind")]
pub enum RepairImpactKind {
    Invalid,
    Node,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RepairImpactKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RepairImpactKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RepairImpactKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("RepairImpactKind", 0u32, "Invalid"),
            Self::Node => serializer.serialize_unit_variant("RepairImpactKind", 1u32, "Node"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the entities targeted by a repair action.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepairTargetDescriptionBase {
    #[doc = "Specifies the kind of the repair target. This type supports the Service Fabric platform; it is not meant to be used directly from your code.'"]
    #[serde(rename = "Kind")]
    pub kind: RepairTargetKind,
}
impl RepairTargetDescriptionBase {
    pub fn new(kind: RepairTargetKind) -> Self {
        Self { kind }
    }
}
#[doc = "Specifies the kind of the repair target. This type supports the Service Fabric platform; it is not meant to be used directly from your code.'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RepairTargetKind")]
pub enum RepairTargetKind {
    Invalid,
    Node,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RepairTargetKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RepairTargetKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RepairTargetKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("RepairTargetKind", 0u32, "Invalid"),
            Self::Node => serializer.serialize_unit_variant("RepairTargetKind", 1u32, "Node"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a repair task, which includes information about what kind of repair was requested, what its progress is, and what its final result was.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepairTask {
    #[doc = "The ID of the repair task."]
    #[serde(rename = "TaskId")]
    pub task_id: String,
    #[doc = "The version of the repair task.\nWhen creating a new repair task, the version must be set to zero.  When updating a repair task,\nthe version is used for optimistic concurrency checks.  If the version is\nset to zero, the update will not check for write conflicts.  If the version is set to a non-zero value, then the\nupdate will only succeed if the actual current version of the repair task matches this value."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "A description of the purpose of the repair task, or other informational details.\nMay be set when the repair task is created, and is immutable once set."]
    #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The workflow state of the repair task. Valid initial states are Created, Claimed, and Preparing."]
    #[serde(rename = "State")]
    pub state: repair_task::State,
    #[doc = "A bitwise-OR of the following values, which gives additional details about the status of the repair task.\n- 1 - Cancellation of the repair has been requested\n- 2 - Abort of the repair has been requested\n- 4 - Approval of the repair was forced via client request"]
    #[serde(rename = "Flags", default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i64>,
    #[doc = "The requested repair action. Must be specified when the repair task is created, and is immutable once set."]
    #[serde(rename = "Action")]
    pub action: String,
    #[doc = "Describes the entities targeted by a repair action.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
    #[serde(rename = "Target", default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RepairTargetDescriptionBase>,
    #[doc = "The name of the repair executor. Must be specified in Claimed and later states, and is immutable once set."]
    #[serde(rename = "Executor", default, skip_serializing_if = "Option::is_none")]
    pub executor: Option<String>,
    #[doc = "A data string that the repair executor can use to store its internal state."]
    #[serde(rename = "ExecutorData", default, skip_serializing_if = "Option::is_none")]
    pub executor_data: Option<String>,
    #[doc = "Describes the expected impact of executing a repair task.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
    #[serde(rename = "Impact", default, skip_serializing_if = "Option::is_none")]
    pub impact: Option<RepairImpactDescriptionBase>,
    #[doc = "A value describing the overall result of the repair task execution. Must be specified in the Restoring and later states, and is immutable once set."]
    #[serde(rename = "ResultStatus", default, skip_serializing_if = "Option::is_none")]
    pub result_status: Option<repair_task::ResultStatus>,
    #[doc = "A numeric value providing additional details about the result of the repair task execution.\nMay be specified in the Restoring and later states, and is immutable once set."]
    #[serde(rename = "ResultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<i64>,
    #[doc = "A string providing additional details about the result of the repair task execution.\nMay be specified in the Restoring and later states, and is immutable once set."]
    #[serde(rename = "ResultDetails", default, skip_serializing_if = "Option::is_none")]
    pub result_details: Option<String>,
    #[doc = "A record of the times when the repair task entered each state.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
    #[serde(rename = "History", default, skip_serializing_if = "Option::is_none")]
    pub history: Option<RepairTaskHistory>,
    #[doc = "Specifies the workflow state of a repair task's health check. This type supports the Service Fabric platform; it is not meant to be used directly from your code."]
    #[serde(rename = "PreparingHealthCheckState", default, skip_serializing_if = "Option::is_none")]
    pub preparing_health_check_state: Option<RepairTaskHealthCheckState>,
    #[doc = "Specifies the workflow state of a repair task's health check. This type supports the Service Fabric platform; it is not meant to be used directly from your code."]
    #[serde(rename = "RestoringHealthCheckState", default, skip_serializing_if = "Option::is_none")]
    pub restoring_health_check_state: Option<RepairTaskHealthCheckState>,
    #[doc = "A value to determine if health checks will be performed when the repair task enters the Preparing state."]
    #[serde(rename = "PerformPreparingHealthCheck", default, skip_serializing_if = "Option::is_none")]
    pub perform_preparing_health_check: Option<bool>,
    #[doc = "A value to determine if health checks will be performed when the repair task enters the Restoring state."]
    #[serde(rename = "PerformRestoringHealthCheck", default, skip_serializing_if = "Option::is_none")]
    pub perform_restoring_health_check: Option<bool>,
}
impl RepairTask {
    pub fn new(task_id: String, state: repair_task::State, action: String) -> Self {
        Self {
            task_id,
            version: None,
            description: None,
            state,
            flags: None,
            action,
            target: None,
            executor: None,
            executor_data: None,
            impact: None,
            result_status: None,
            result_code: None,
            result_details: None,
            history: None,
            preparing_health_check_state: None,
            restoring_health_check_state: None,
            perform_preparing_health_check: None,
            perform_restoring_health_check: None,
        }
    }
}
pub mod repair_task {
    use super::*;
    #[doc = "The workflow state of the repair task. Valid initial states are Created, Claimed, and Preparing."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Invalid,
        Created,
        Claimed,
        Preparing,
        Approved,
        Executing,
        Restoring,
        Completed,
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
                Self::Invalid => serializer.serialize_unit_variant("State", 0u32, "Invalid"),
                Self::Created => serializer.serialize_unit_variant("State", 1u32, "Created"),
                Self::Claimed => serializer.serialize_unit_variant("State", 2u32, "Claimed"),
                Self::Preparing => serializer.serialize_unit_variant("State", 3u32, "Preparing"),
                Self::Approved => serializer.serialize_unit_variant("State", 4u32, "Approved"),
                Self::Executing => serializer.serialize_unit_variant("State", 5u32, "Executing"),
                Self::Restoring => serializer.serialize_unit_variant("State", 6u32, "Restoring"),
                Self::Completed => serializer.serialize_unit_variant("State", 7u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value describing the overall result of the repair task execution. Must be specified in the Restoring and later states, and is immutable once set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResultStatus")]
    pub enum ResultStatus {
        Invalid,
        Succeeded,
        Cancelled,
        Interrupted,
        Failed,
        Pending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResultStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResultStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResultStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ResultStatus", 0u32, "Invalid"),
                Self::Succeeded => serializer.serialize_unit_variant("ResultStatus", 1u32, "Succeeded"),
                Self::Cancelled => serializer.serialize_unit_variant("ResultStatus", 2u32, "Cancelled"),
                Self::Interrupted => serializer.serialize_unit_variant("ResultStatus", 3u32, "Interrupted"),
                Self::Failed => serializer.serialize_unit_variant("ResultStatus", 4u32, "Failed"),
                Self::Pending => serializer.serialize_unit_variant("ResultStatus", 5u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a request for forced approval of a repair task.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepairTaskApproveDescription {
    #[doc = "The ID of the repair task."]
    #[serde(rename = "TaskId")]
    pub task_id: String,
    #[doc = "The current version number of the repair task. If non-zero, then the request will only succeed if this value matches the actual current version of the repair task. If zero, then no version check is performed."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl RepairTaskApproveDescription {
    pub fn new(task_id: String) -> Self {
        Self { task_id, version: None }
    }
}
#[doc = "Describes a request to cancel a repair task.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepairTaskCancelDescription {
    #[doc = "The ID of the repair task."]
    #[serde(rename = "TaskId")]
    pub task_id: String,
    #[doc = "The current version number of the repair task. If non-zero, then the request will only succeed if this value matches the actual current version of the repair task. If zero, then no version check is performed."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "_True_ if the repair should be stopped as soon as possible even if it has already started executing. _False_ if the repair should be cancelled only if execution has not yet started."]
    #[serde(rename = "RequestAbort", default, skip_serializing_if = "Option::is_none")]
    pub request_abort: Option<bool>,
}
impl RepairTaskCancelDescription {
    pub fn new(task_id: String) -> Self {
        Self {
            task_id,
            version: None,
            request_abort: None,
        }
    }
}
#[doc = "Describes a request to delete a completed repair task.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepairTaskDeleteDescription {
    #[doc = "The ID of the completed repair task to be deleted."]
    #[serde(rename = "TaskId")]
    pub task_id: String,
    #[doc = "The current version number of the repair task. If non-zero, then the request will only succeed if this value matches the actual current version of the repair task. If zero, then no version check is performed."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl RepairTaskDeleteDescription {
    pub fn new(task_id: String) -> Self {
        Self { task_id, version: None }
    }
}
#[doc = "Specifies the workflow state of a repair task's health check. This type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RepairTaskHealthCheckState")]
pub enum RepairTaskHealthCheckState {
    NotStarted,
    InProgress,
    Succeeded,
    Skipped,
    TimedOut,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RepairTaskHealthCheckState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RepairTaskHealthCheckState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RepairTaskHealthCheckState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("RepairTaskHealthCheckState", 0u32, "NotStarted"),
            Self::InProgress => serializer.serialize_unit_variant("RepairTaskHealthCheckState", 1u32, "InProgress"),
            Self::Succeeded => serializer.serialize_unit_variant("RepairTaskHealthCheckState", 2u32, "Succeeded"),
            Self::Skipped => serializer.serialize_unit_variant("RepairTaskHealthCheckState", 3u32, "Skipped"),
            Self::TimedOut => serializer.serialize_unit_variant("RepairTaskHealthCheckState", 4u32, "TimedOut"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A record of the times when the repair task entered each state.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RepairTaskHistory {
    #[doc = "The time when the repair task entered the Created state."]
    #[serde(rename = "CreatedUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub created_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The time when the repair task entered the Claimed state."]
    #[serde(rename = "ClaimedUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub claimed_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The time when the repair task entered the Preparing state."]
    #[serde(rename = "PreparingUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub preparing_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The time when the repair task entered the Approved state"]
    #[serde(rename = "ApprovedUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub approved_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The time when the repair task entered the Executing state"]
    #[serde(rename = "ExecutingUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub executing_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The time when the repair task entered the Restoring state"]
    #[serde(rename = "RestoringUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub restoring_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The time when the repair task entered the Completed state"]
    #[serde(rename = "CompletedUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub completed_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The time when the repair task started the health check in the Preparing state."]
    #[serde(rename = "PreparingHealthCheckStartUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub preparing_health_check_start_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The time when the repair task completed the health check in the Preparing state."]
    #[serde(rename = "PreparingHealthCheckEndUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub preparing_health_check_end_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The time when the repair task started the health check in the Restoring state."]
    #[serde(rename = "RestoringHealthCheckStartUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub restoring_health_check_start_utc_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The time when the repair task completed the health check in the Restoring state."]
    #[serde(rename = "RestoringHealthCheckEndUtcTimestamp", with = "azure_core::date::rfc3339::option")]
    pub restoring_health_check_end_utc_timestamp: Option<time::OffsetDateTime>,
}
impl RepairTaskHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type RepairTaskList = Vec<RepairTask>;
#[doc = "Describes a request to update the health policy of a repair task.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepairTaskUpdateHealthPolicyDescription {
    #[doc = "The ID of the repair task to be updated."]
    #[serde(rename = "TaskId")]
    pub task_id: String,
    #[doc = "The current version number of the repair task. If non-zero, then the request will only succeed if this value matches the actual current value of the repair task. If zero, then no version check is performed."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "A boolean indicating if health check is to be performed in the Preparing stage of the repair task. If not specified the existing value should not be altered. Otherwise, specify the desired new value."]
    #[serde(rename = "PerformPreparingHealthCheck", default, skip_serializing_if = "Option::is_none")]
    pub perform_preparing_health_check: Option<bool>,
    #[doc = "A boolean indicating if health check is to be performed in the Restoring stage of the repair task. If not specified the existing value should not be altered. Otherwise, specify the desired new value."]
    #[serde(rename = "PerformRestoringHealthCheck", default, skip_serializing_if = "Option::is_none")]
    pub perform_restoring_health_check: Option<bool>,
}
impl RepairTaskUpdateHealthPolicyDescription {
    pub fn new(task_id: String) -> Self {
        Self {
            task_id,
            version: None,
            perform_preparing_health_check: None,
            perform_restoring_health_check: None,
        }
    }
}
#[doc = "Describes the result of an operation that created or updated a repair task.\n\nThis type supports the Service Fabric platform; it is not meant to be used directly from your code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepairTaskUpdateInfo {
    #[doc = "The new version of the repair task."]
    #[serde(rename = "Version")]
    pub version: String,
}
impl RepairTaskUpdateInfo {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}
#[doc = "Represents the base for all Replica Events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicaEvent {
    #[serde(flatten)]
    pub fabric_event: FabricEvent,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId")]
    pub partition_id: PartitionId,
    #[doc = "Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id."]
    #[serde(rename = "ReplicaId")]
    pub replica_id: ReplicaIdInteger,
}
impl ReplicaEvent {
    pub fn new(fabric_event: FabricEvent, partition_id: PartitionId, replica_id: ReplicaIdInteger) -> Self {
        Self {
            fabric_event,
            partition_id,
            replica_id,
        }
    }
}
pub type ReplicaEventList = Vec<ReplicaEvent>;
#[doc = "Represents a base class for stateful service replica or stateless service instance health.\nContains the replica aggregated health state, the health events and the unhealthy evaluations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicaHealth {
    #[serde(flatten)]
    pub entity_health: EntityHealth,
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "ServiceKind")]
    pub service_kind: ServiceKind,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
}
impl ReplicaHealth {
    pub fn new(service_kind: ServiceKind) -> Self {
        Self {
            entity_health: EntityHealth::default(),
            service_kind,
            partition_id: None,
        }
    }
}
#[doc = "Represents health evaluation for a replica, containing information about the data and the algorithm used by health store to evaluate health. The evaluation is returned only when the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicaHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
    #[doc = "Id of a stateful service replica or a stateless service instance. This ID is used in the queries that apply to both stateful and stateless services. It is used by Service Fabric to uniquely identify a replica of a partition of a stateful service or an instance of a stateless service partition. It is unique within a partition and does not change for the lifetime of the replica or the instance. If a stateful replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the ID. If a stateless instance is failed over on the same or different node it will get a different value for the ID."]
    #[serde(rename = "ReplicaOrInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub replica_or_instance_id: Option<ReplicaOrInstanceId>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl ReplicaHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            partition_id: None,
            replica_or_instance_id: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Represents a base class for stateful service replica or stateless service instance health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicaHealthState {
    #[serde(flatten)]
    pub entity_health_state: EntityHealthState,
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "ServiceKind")]
    pub service_kind: ServiceKind,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
}
impl ReplicaHealthState {
    pub fn new(service_kind: ServiceKind) -> Self {
        Self {
            entity_health_state: EntityHealthState::default(),
            service_kind,
            partition_id: None,
        }
    }
}
#[doc = "Represents the health state chunk of a stateful service replica or a stateless service instance.\nThe replica health state contains the replica ID and its aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicaHealthStateChunk {
    #[serde(flatten)]
    pub entity_health_state_chunk: EntityHealthStateChunk,
    #[doc = "Id of a stateful service replica or a stateless service instance. This ID is used in the queries that apply to both stateful and stateless services. It is used by Service Fabric to uniquely identify a replica of a partition of a stateful service or an instance of a stateless service partition. It is unique within a partition and does not change for the lifetime of the replica or the instance. If a stateful replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the ID. If a stateless instance is failed over on the same or different node it will get a different value for the ID."]
    #[serde(rename = "ReplicaOrInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub replica_or_instance_id: Option<ReplicaOrInstanceId>,
}
impl ReplicaHealthStateChunk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of replica health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicaHealthStateChunkList {
    #[doc = "The list of replica health state chunks that respect the input filters in the chunk query."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ReplicaHealthStateChunk>,
}
impl ReplicaHealthStateChunkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines matching criteria to determine whether a replica should be included as a child of a partition in the cluster health chunk.\nThe replicas are only returned if the parent entities match a filter specified in the cluster health chunk query description. The parent partition, service and application must be included in the cluster health chunk.\nOne filter can match zero, one or multiple replicas, depending on its properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicaHealthStateFilter {
    #[doc = "Id of the stateful service replica or stateless service instance that matches the filter. The filter is applied only to the specified replica, if it exists.\nIf the replica doesn't exist, no replica is returned in the cluster health chunk based on this filter.\nIf the replica exists, it is included in the cluster health chunk if it respects the other filter properties.\nIf not specified, all replicas that match the parent filters (if any) are taken into consideration and matched against the other filter members, like health state filter."]
    #[serde(rename = "ReplicaOrInstanceIdFilter", default, skip_serializing_if = "Option::is_none")]
    pub replica_or_instance_id_filter: Option<String>,
    #[doc = "The filter for the health state of the replicas. It allows selecting replicas if they match the desired health states.\nThe possible values are integer value of one of the following health states. Only replicas that match the filter are returned. All replicas are used to evaluate the parent partition aggregated health state.\nIf not specified, default value is None, unless the replica ID is specified. If the filter has default value and replica ID is specified, the matching replica is returned.\nThe state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise 'OR' operator.\nFor example, if the provided value is 6, it matches replicas with HealthState value of OK (2) and Warning (4).\n\n- Default - Default value. Matches any HealthState. The value is zero.\n- None - Filter that doesn't match any HealthState value. Used in order to return no results on a given collection of states. The value is 1.\n- Ok - Filter that matches input with HealthState value Ok. The value is 2.\n- Warning - Filter that matches input with HealthState value Warning. The value is 4.\n- Error - Filter that matches input with HealthState value Error. The value is 8.\n- All - Filter that matches input with any HealthState value. The value is 65535."]
    #[serde(rename = "HealthStateFilter", default, skip_serializing_if = "Option::is_none")]
    pub health_state_filter: Option<i64>,
}
impl ReplicaHealthStateFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ReplicaId = String;
pub type ReplicaIdInteger = i64;
#[doc = "Information about the identity, status, health, node name, uptime, and other details about the replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicaInfo {
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "ServiceKind")]
    pub service_kind: ServiceKind,
    #[doc = "The status of a replica of a service."]
    #[serde(rename = "ReplicaStatus", default, skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<ReplicaStatus>,
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "HealthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[doc = "The name of a Service Fabric node."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<NodeName>,
    #[doc = "The address the replica is listening on."]
    #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "The last in build duration of the replica in seconds."]
    #[serde(rename = "LastInBuildDurationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub last_in_build_duration_in_seconds: Option<String>,
}
impl ReplicaInfo {
    pub fn new(service_kind: ServiceKind) -> Self {
        Self {
            service_kind,
            replica_status: None,
            health_state: None,
            node_name: None,
            address: None,
            last_in_build_duration_in_seconds: None,
        }
    }
}
#[doc = "The role of a replica of a stateful service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReplicaKind")]
pub enum ReplicaKind {
    Invalid,
    KeyValueStore,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReplicaKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReplicaKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReplicaKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ReplicaKind", 0u32, "Invalid"),
            Self::KeyValueStore => serializer.serialize_unit_variant("ReplicaKind", 1u32, "KeyValueStore"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies metric loads of a partition's specific secondary replica or instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicaMetricLoadDescription {
    #[doc = "Node name of a specific secondary replica or instance."]
    #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "Loads of a different metrics for a partition's secondary replica or instance."]
    #[serde(rename = "ReplicaOrInstanceLoadEntries", default, skip_serializing_if = "Vec::is_empty")]
    pub replica_or_instance_load_entries: Vec<MetricLoadDescription>,
}
impl ReplicaMetricLoadDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ReplicaOrInstanceId = String;
#[doc = "The role of a replica of a stateful service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReplicaRole")]
pub enum ReplicaRole {
    Unknown,
    None,
    Primary,
    IdleSecondary,
    ActiveSecondary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReplicaRole {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReplicaRole {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReplicaRole {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ReplicaRole", 0u32, "Unknown"),
            Self::None => serializer.serialize_unit_variant("ReplicaRole", 1u32, "None"),
            Self::Primary => serializer.serialize_unit_variant("ReplicaRole", 2u32, "Primary"),
            Self::IdleSecondary => serializer.serialize_unit_variant("ReplicaRole", 3u32, "IdleSecondary"),
            Self::ActiveSecondary => serializer.serialize_unit_variant("ReplicaRole", 4u32, "ActiveSecondary"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of a replica of a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReplicaStatus")]
pub enum ReplicaStatus {
    Invalid,
    InBuild,
    Standby,
    Ready,
    Down,
    Dropped,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReplicaStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReplicaStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReplicaStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ReplicaStatus", 0u32, "Invalid"),
            Self::InBuild => serializer.serialize_unit_variant("ReplicaStatus", 1u32, "InBuild"),
            Self::Standby => serializer.serialize_unit_variant("ReplicaStatus", 2u32, "Standby"),
            Self::Ready => serializer.serialize_unit_variant("ReplicaStatus", 3u32, "Ready"),
            Self::Down => serializer.serialize_unit_variant("ReplicaStatus", 4u32, "Down"),
            Self::Dropped => serializer.serialize_unit_variant("ReplicaStatus", 5u32, "Dropped"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information about the replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicaStatusBase {
    #[doc = "The role of a replica of a stateful service."]
    #[serde(rename = "Kind")]
    pub kind: ReplicaKind,
}
impl ReplicaStatusBase {
    pub fn new(kind: ReplicaKind) -> Self {
        Self { kind }
    }
}
#[doc = "Represents health evaluation for replicas, containing health evaluations for each unhealthy replica that impacted current aggregated health state. Can be returned when evaluating partition health and the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicasHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Maximum allowed percentage of unhealthy replicas per partition from the ApplicationHealthPolicy."]
    #[serde(
        rename = "MaxPercentUnhealthyReplicasPerPartition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_percent_unhealthy_replicas_per_partition: Option<i64>,
    #[doc = "Total number of replicas in the partition from the health store."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl ReplicasHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            max_percent_unhealthy_replicas_per_partition: None,
            total_count: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Specifies the operation currently being executed by the Replicator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReplicatorOperationName")]
pub enum ReplicatorOperationName {
    Invalid,
    None,
    Open,
    ChangeRole,
    UpdateEpoch,
    Close,
    Abort,
    OnDataLoss,
    WaitForCatchup,
    Build,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReplicatorOperationName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReplicatorOperationName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReplicatorOperationName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ReplicatorOperationName", 0u32, "Invalid"),
            Self::None => serializer.serialize_unit_variant("ReplicatorOperationName", 1u32, "None"),
            Self::Open => serializer.serialize_unit_variant("ReplicatorOperationName", 2u32, "Open"),
            Self::ChangeRole => serializer.serialize_unit_variant("ReplicatorOperationName", 3u32, "ChangeRole"),
            Self::UpdateEpoch => serializer.serialize_unit_variant("ReplicatorOperationName", 4u32, "UpdateEpoch"),
            Self::Close => serializer.serialize_unit_variant("ReplicatorOperationName", 5u32, "Close"),
            Self::Abort => serializer.serialize_unit_variant("ReplicatorOperationName", 6u32, "Abort"),
            Self::OnDataLoss => serializer.serialize_unit_variant("ReplicatorOperationName", 7u32, "OnDataLoss"),
            Self::WaitForCatchup => serializer.serialize_unit_variant("ReplicatorOperationName", 8u32, "WaitForCatchup"),
            Self::Build => serializer.serialize_unit_variant("ReplicatorOperationName", 9u32, "Build"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provides various statistics of the queue used in the service fabric replicator.\nContains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc.\nDepending on the role of the replicator, the properties in this type imply different meanings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicatorQueueStatus {
    #[doc = "Represents the utilization of the queue. A value of 0 indicates that the queue is empty and a value of 100 indicates the queue is full."]
    #[serde(rename = "QueueUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub queue_utilization_percentage: Option<i32>,
    #[doc = "Represents the virtual memory consumed by the queue in bytes."]
    #[serde(rename = "QueueMemorySize", default, skip_serializing_if = "Option::is_none")]
    pub queue_memory_size: Option<String>,
    #[doc = "On a primary replicator, this is semantically the sequence number of the operation for which all the secondary replicas have sent an acknowledgement.\nOn a secondary replicator, this is the smallest sequence number of the operation that is present in the queue."]
    #[serde(rename = "FirstSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub first_sequence_number: Option<String>,
    #[doc = "On a primary replicator, this is semantically the highest sequence number of the operation for which all the secondary replicas have sent an acknowledgement.\nOn a secondary replicator, this is semantically the highest sequence number that has been applied to the persistent state."]
    #[serde(rename = "CompletedSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub completed_sequence_number: Option<String>,
    #[doc = "On a primary replicator, this is semantically the highest sequence number of the operation for which a write quorum of the secondary replicas have sent an acknowledgement.\nOn a secondary replicator, this is semantically the highest sequence number of the in-order operation received from the primary."]
    #[serde(rename = "CommittedSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub committed_sequence_number: Option<String>,
    #[doc = "Represents the latest sequence number of the operation that is available in the queue."]
    #[serde(rename = "LastSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub last_sequence_number: Option<String>,
}
impl ReplicatorQueueStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a base class for primary or secondary replicator status.\nContains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicatorStatus {
    #[doc = "The role of a replica of a stateful service."]
    #[serde(rename = "Kind")]
    pub kind: ReplicaRole,
}
impl ReplicatorStatus {
    pub fn new(kind: ReplicaRole) -> Self {
        Self { kind }
    }
}
#[doc = "Endpoint of a resolved service partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolvedServiceEndpoint {
    #[doc = "The role of the replica where the endpoint is reported."]
    #[serde(rename = "Kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ServiceEndpointRole>,
    #[doc = "The address of the endpoint. If the endpoint has multiple listeners the address is a JSON object with one property per listener with the value as the address of that listener."]
    #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}
impl ResolvedServiceEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ResolvedServiceEndpointList = Vec<ResolvedServiceEndpoint>;
#[doc = "Information about a service partition and its associated endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedServicePartition {
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "Name")]
    pub name: ServiceName,
    #[doc = "Information about the partition identity, partitioning scheme and keys supported by it."]
    #[serde(rename = "PartitionInformation")]
    pub partition_information: PartitionInformation,
    #[doc = "List of resolved service endpoints of a service partition."]
    #[serde(rename = "Endpoints")]
    pub endpoints: ResolvedServiceEndpointList,
    #[doc = "The version of this resolved service partition result. This version should be passed in the next time the ResolveService call is made via the PreviousRspVersion query parameter."]
    #[serde(rename = "Version")]
    pub version: String,
}
impl ResolvedServicePartition {
    pub fn new(
        name: ServiceName,
        partition_information: PartitionInformation,
        endpoints: ResolvedServiceEndpointList,
        version: String,
    ) -> Self {
        Self {
            name,
            partition_information,
            endpoints,
            version,
        }
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
#[doc = "Defines description for restarting a deployed code package on Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestartDeployedCodePackageDescription {
    #[doc = "The name of the service manifest."]
    #[serde(rename = "ServiceManifestName")]
    pub service_manifest_name: ServiceManifestName,
    #[doc = "The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service\nis 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId\nis always an empty string."]
    #[serde(rename = "ServicePackageActivationId", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_id: Option<ServicePackageActivationId>,
    #[doc = "The name of the code package defined in the service manifest."]
    #[serde(rename = "CodePackageName")]
    pub code_package_name: CodePackageName,
    #[doc = "The instance ID for current running entry point. For a code package setup entry point (if specified) runs first and after it finishes main entry point is started. Each time entry point executable is run, its instance id will change."]
    #[serde(rename = "CodePackageInstanceId")]
    pub code_package_instance_id: CodePackageInstanceId,
}
impl RestartDeployedCodePackageDescription {
    pub fn new(
        service_manifest_name: ServiceManifestName,
        code_package_name: CodePackageName,
        code_package_instance_id: CodePackageInstanceId,
    ) -> Self {
        Self {
            service_manifest_name,
            service_package_activation_id: None,
            code_package_name,
            code_package_instance_id,
        }
    }
}
#[doc = "Describes the parameters to restart a Service Fabric node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestartNodeDescription {
    #[doc = "The instance ID of the target node. If instance ID is specified the node is restarted only if it matches with the current instance of the node. A default value of \"0\" would match any instance ID. The instance ID can be obtained using get node query."]
    #[serde(rename = "NodeInstanceId")]
    pub node_instance_id: String,
    #[doc = "Specify True to create a dump of the fabric node process. This is case-sensitive."]
    #[serde(rename = "CreateFabricDump", default, skip_serializing_if = "Option::is_none")]
    pub create_fabric_dump: Option<restart_node_description::CreateFabricDump>,
}
impl RestartNodeDescription {
    pub fn new(node_instance_id: String) -> Self {
        Self {
            node_instance_id,
            create_fabric_dump: None,
        }
    }
}
pub mod restart_node_description {
    use super::*;
    #[doc = "Specify True to create a dump of the fabric node process. This is case-sensitive."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateFabricDump")]
    pub enum CreateFabricDump {
        False,
        True,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreateFabricDump {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreateFabricDump {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreateFabricDump {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::False => serializer.serialize_unit_variant("CreateFabricDump", 0u32, "False"),
                Self::True => serializer.serialize_unit_variant("CreateFabricDump", 1u32, "True"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for CreateFabricDump {
        fn default() -> Self {
            Self::False
        }
    }
}
#[doc = "Represents information about an operation in a terminal state (Completed or Faulted)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestartPartitionResult {
    #[doc = "If OperationState is Completed, this is 0.  If OperationState is Faulted, this is an error code indicating the reason."]
    #[serde(rename = "ErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "This class returns information about the partition that the user-induced operation acted upon."]
    #[serde(rename = "SelectedPartition", default, skip_serializing_if = "Option::is_none")]
    pub selected_partition: Option<SelectedPartition>,
}
impl RestartPartitionResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enumerates the restart policy for RunToCompletionExecutionPolicy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RestartPolicy")]
pub enum RestartPolicy {
    OnFailure,
    Never,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RestartPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RestartPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RestartPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OnFailure => serializer.serialize_unit_variant("RestartPolicy", 0u32, "OnFailure"),
            Self::Never => serializer.serialize_unit_variant("RestartPolicy", 1u32, "Never"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the parameters needed to trigger a restore of a specific partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestorePartitionDescription {
    #[doc = "Unique backup ID."]
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    #[doc = "Location of the backup relative to the backup storage specified/ configured."]
    #[serde(rename = "BackupLocation")]
    pub backup_location: String,
    #[doc = "Describes the parameters for the backup storage."]
    #[serde(rename = "BackupStorage", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage: Option<BackupStorageDescription>,
}
impl RestorePartitionDescription {
    pub fn new(backup_id: String, backup_location: String) -> Self {
        Self {
            backup_id,
            backup_location,
            backup_storage: None,
        }
    }
}
#[doc = "Describes the progress of a restore operation on a partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreProgressInfo {
    #[doc = "Represents the current state of the partition restore operation."]
    #[serde(rename = "RestoreState", default, skip_serializing_if = "Option::is_none")]
    pub restore_state: Option<RestoreState>,
    #[doc = "Timestamp when operation succeeded or failed."]
    #[serde(rename = "TimeStampUtc", with = "azure_core::date::rfc3339::option")]
    pub time_stamp_utc: Option<time::OffsetDateTime>,
    #[doc = "An Epoch is a configuration number for the partition as a whole. When the configuration of the replica set changes, for example when the Primary replica changes, the operations that are replicated from the new Primary replica are said to be a new Epoch from the ones which were sent by the old Primary replica."]
    #[serde(rename = "RestoredEpoch", default, skip_serializing_if = "Option::is_none")]
    pub restored_epoch: Option<Epoch>,
    #[doc = "Restored LSN."]
    #[serde(rename = "RestoredLsn", default, skip_serializing_if = "Option::is_none")]
    pub restored_lsn: Option<String>,
    #[doc = "Error object containing error code and error message."]
    #[serde(rename = "FailureError", default, skip_serializing_if = "Option::is_none")]
    pub failure_error: Option<FabricErrorError>,
}
impl RestoreProgressInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the current state of the partition restore operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RestoreState")]
pub enum RestoreState {
    Invalid,
    Accepted,
    RestoreInProgress,
    Success,
    Failure,
    Timeout,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RestoreState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RestoreState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RestoreState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("RestoreState", 0u32, "Invalid"),
            Self::Accepted => serializer.serialize_unit_variant("RestoreState", 1u32, "Accepted"),
            Self::RestoreInProgress => serializer.serialize_unit_variant("RestoreState", 2u32, "RestoreInProgress"),
            Self::Success => serializer.serialize_unit_variant("RestoreState", 3u32, "Success"),
            Self::Failure => serializer.serialize_unit_variant("RestoreState", 4u32, "Failure"),
            Self::Timeout => serializer.serialize_unit_variant("RestoreState", 5u32, "Timeout"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the parameters for resuming an unmonitored manual Service Fabric application upgrade"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResumeApplicationUpgradeDescription {
    #[doc = "The name of the upgrade domain in which to resume the upgrade."]
    #[serde(rename = "UpgradeDomainName")]
    pub upgrade_domain_name: String,
}
impl ResumeApplicationUpgradeDescription {
    pub fn new(upgrade_domain_name: String) -> Self {
        Self { upgrade_domain_name }
    }
}
#[doc = "Describes the parameters for resuming a cluster upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResumeClusterUpgradeDescription {
    #[doc = "The next upgrade domain for this cluster upgrade."]
    #[serde(rename = "UpgradeDomain")]
    pub upgrade_domain: String,
}
impl ResumeClusterUpgradeDescription {
    pub fn new(upgrade_domain: String) -> Self {
        Self { upgrade_domain }
    }
}
#[doc = "Describes the retention policy configured."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicyDescription {
    #[doc = "The type of retention policy. Currently only \"Basic\" retention policy is supported."]
    #[serde(rename = "RetentionPolicyType")]
    pub retention_policy_type: RetentionPolicyType,
}
impl RetentionPolicyDescription {
    pub fn new(retention_policy_type: RetentionPolicyType) -> Self {
        Self { retention_policy_type }
    }
}
#[doc = "The type of retention policy. Currently only \"Basic\" retention policy is supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RetentionPolicyType")]
pub enum RetentionPolicyType {
    Basic,
    Invalid,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RetentionPolicyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RetentionPolicyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RetentionPolicyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Basic => serializer.serialize_unit_variant("RetentionPolicyType", 0u32, "Basic"),
            Self::Invalid => serializer.serialize_unit_variant("RetentionPolicyType", 1u32, "Invalid"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RollingUpgradeMode")]
pub enum RollingUpgradeMode {
    Invalid,
    UnmonitoredAuto,
    UnmonitoredManual,
    Monitored,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RollingUpgradeMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RollingUpgradeMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RollingUpgradeMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("RollingUpgradeMode", 0u32, "Invalid"),
            Self::UnmonitoredAuto => serializer.serialize_unit_variant("RollingUpgradeMode", 1u32, "UnmonitoredAuto"),
            Self::UnmonitoredManual => serializer.serialize_unit_variant("RollingUpgradeMode", 2u32, "UnmonitoredManual"),
            Self::Monitored => serializer.serialize_unit_variant("RollingUpgradeMode", 3u32, "Monitored"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for RollingUpgradeMode {
    fn default() -> Self {
        Self::Monitored
    }
}
#[doc = "Describes the parameters for updating a rolling upgrade of application or cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RollingUpgradeUpdateDescription {
    #[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
    #[serde(rename = "RollingUpgradeMode")]
    pub rolling_upgrade_mode: UpgradeMode,
    #[doc = "If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data)."]
    #[serde(rename = "ForceRestart", default, skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<ForceRestart>,
    #[doc = "The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer)."]
    #[serde(rename = "ReplicaSetCheckTimeoutInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub replica_set_check_timeout_in_milliseconds: Option<UpgradeReplicaSetCheckTimeout>,
    #[doc = "The compensating action to perform when a Monitored upgrade encounters monitoring policy or health policy violations.\nInvalid indicates the failure action is invalid. Rollback specifies that the upgrade will start rolling back automatically.\nManual indicates that the upgrade will switch to UnmonitoredManual upgrade mode."]
    #[serde(rename = "FailureAction", default, skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<FailureAction>,
    #[doc = "The amount of time to wait after completing an upgrade domain before applying health policies. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "HealthCheckWaitDurationInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub health_check_wait_duration_in_milliseconds: Option<HealthCheckWaitDuration>,
    #[doc = "The amount of time that the application or cluster must remain healthy before the upgrade proceeds to the next upgrade domain. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(
        rename = "HealthCheckStableDurationInMilliseconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub health_check_stable_duration_in_milliseconds: Option<HealthCheckStableDuration>,
    #[doc = "The amount of time to retry health evaluation when the application or cluster is unhealthy before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "HealthCheckRetryTimeoutInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub health_check_retry_timeout_in_milliseconds: Option<HealthCheckRetryTimeout>,
    #[doc = "The amount of time the overall upgrade has to complete before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "UpgradeTimeoutInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_timeout_in_milliseconds: Option<UpgradeTimeout>,
    #[doc = "The amount of time each upgrade domain has to complete before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds."]
    #[serde(rename = "UpgradeDomainTimeoutInMilliseconds", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain_timeout_in_milliseconds: Option<UpgradeDomainTimeout>,
    #[doc = "Duration in seconds, to wait before a stateless instance is closed, to allow the active requests to drain gracefully. This would be effective when the instance is closing during the application/cluster\nupgrade, only for those instances which have a non-zero delay duration configured in the service description. See InstanceCloseDelayDurationSeconds property in $ref: \"#/definitions/StatelessServiceDescription.yaml\" for details.\nNote, the default value of InstanceCloseDelayDurationInSeconds is 4294967295, which indicates that the behavior will entirely depend on the delay configured in the stateless service description."]
    #[serde(rename = "InstanceCloseDelayDurationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub instance_close_delay_duration_in_seconds: Option<InstanceCloseDelayDurationInSeconds>,
}
impl RollingUpgradeUpdateDescription {
    pub fn new(rolling_upgrade_mode: UpgradeMode) -> Self {
        Self {
            rolling_upgrade_mode,
            force_restart: None,
            replica_set_check_timeout_in_milliseconds: None,
            failure_action: None,
            health_check_wait_duration_in_milliseconds: None,
            health_check_stable_duration_in_milliseconds: None,
            health_check_retry_timeout_in_milliseconds: None,
            upgrade_timeout_in_milliseconds: None,
            upgrade_domain_timeout_in_milliseconds: None,
            instance_close_delay_duration_in_seconds: None,
        }
    }
}
#[doc = "The run to completion execution policy, the service will perform its desired operation and complete successfully. If the service encounters failure, it will restarted based on restart policy specified. If the service completes its operation successfully, it will not be restarted again."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunToCompletionExecutionPolicy {
    #[serde(flatten)]
    pub execution_policy: ExecutionPolicy,
    #[doc = "Enumerates the restart policy for RunToCompletionExecutionPolicy"]
    pub restart: RestartPolicy,
}
impl RunToCompletionExecutionPolicy {
    pub fn new(execution_policy: ExecutionPolicy, restart: RestartPolicy) -> Self {
        Self { execution_policy, restart }
    }
}
#[doc = "Represents a safety check performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SafetyCheck {
    #[doc = "The kind of safety check performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state. Following are the kinds of safety checks."]
    #[serde(rename = "Kind")]
    pub kind: SafetyCheckKind,
}
impl SafetyCheck {
    pub fn new(kind: SafetyCheckKind) -> Self {
        Self { kind }
    }
}
pub type SafetyCheckInfoList = Vec<SafetyCheckWrapper>;
#[doc = "The kind of safety check performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state. Following are the kinds of safety checks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SafetyCheckKind")]
pub enum SafetyCheckKind {
    Invalid,
    EnsureSeedNodeQuorum,
    EnsurePartitionQuorum,
    WaitForPrimaryPlacement,
    WaitForPrimarySwap,
    WaitForReconfiguration,
    WaitForInbuildReplica,
    EnsureAvailability,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SafetyCheckKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SafetyCheckKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SafetyCheckKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("SafetyCheckKind", 0u32, "Invalid"),
            Self::EnsureSeedNodeQuorum => serializer.serialize_unit_variant("SafetyCheckKind", 1u32, "EnsureSeedNodeQuorum"),
            Self::EnsurePartitionQuorum => serializer.serialize_unit_variant("SafetyCheckKind", 2u32, "EnsurePartitionQuorum"),
            Self::WaitForPrimaryPlacement => serializer.serialize_unit_variant("SafetyCheckKind", 3u32, "WaitForPrimaryPlacement"),
            Self::WaitForPrimarySwap => serializer.serialize_unit_variant("SafetyCheckKind", 4u32, "WaitForPrimarySwap"),
            Self::WaitForReconfiguration => serializer.serialize_unit_variant("SafetyCheckKind", 5u32, "WaitForReconfiguration"),
            Self::WaitForInbuildReplica => serializer.serialize_unit_variant("SafetyCheckKind", 6u32, "WaitForInbuildReplica"),
            Self::EnsureAvailability => serializer.serialize_unit_variant("SafetyCheckKind", 7u32, "EnsureAvailability"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A wrapper for the safety check object. Safety checks are performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SafetyCheckWrapper {
    #[doc = "Represents a safety check performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state."]
    #[serde(rename = "SafetyCheck", default, skip_serializing_if = "Option::is_none")]
    pub safety_check: Option<SafetyCheck>,
}
impl SafetyCheckWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the mechanism for performing a scaling operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalingMechanismDescription {
    #[doc = "Enumerates the ways that a service can be scaled."]
    #[serde(rename = "Kind")]
    pub kind: ScalingMechanismKind,
}
impl ScalingMechanismDescription {
    pub fn new(kind: ScalingMechanismKind) -> Self {
        Self { kind }
    }
}
#[doc = "Enumerates the ways that a service can be scaled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScalingMechanismKind")]
pub enum ScalingMechanismKind {
    Invalid,
    PartitionInstanceCount,
    AddRemoveIncrementalNamedPartition,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScalingMechanismKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScalingMechanismKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScalingMechanismKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ScalingMechanismKind", 0u32, "Invalid"),
            Self::PartitionInstanceCount => serializer.serialize_unit_variant("ScalingMechanismKind", 1u32, "PartitionInstanceCount"),
            Self::AddRemoveIncrementalNamedPartition => {
                serializer.serialize_unit_variant("ScalingMechanismKind", 2u32, "AddRemoveIncrementalNamedPartition")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes how the scaling should be performed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalingPolicyDescription {
    #[doc = "Describes the trigger for performing a scaling operation."]
    #[serde(rename = "ScalingTrigger")]
    pub scaling_trigger: ScalingTriggerDescription,
    #[doc = "Describes the mechanism for performing a scaling operation."]
    #[serde(rename = "ScalingMechanism")]
    pub scaling_mechanism: ScalingMechanismDescription,
}
impl ScalingPolicyDescription {
    pub fn new(scaling_trigger: ScalingTriggerDescription, scaling_mechanism: ScalingMechanismDescription) -> Self {
        Self {
            scaling_trigger,
            scaling_mechanism,
        }
    }
}
pub type ScalingPolicyDescriptionList = Vec<ScalingPolicyDescription>;
#[doc = "Describes the trigger for performing a scaling operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalingTriggerDescription {
    #[doc = "Enumerates the ways that a service can be scaled."]
    #[serde(rename = "Kind")]
    pub kind: ScalingTriggerKind,
}
impl ScalingTriggerDescription {
    pub fn new(kind: ScalingTriggerKind) -> Self {
        Self { kind }
    }
}
#[doc = "Enumerates the ways that a service can be scaled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScalingTriggerKind")]
pub enum ScalingTriggerKind {
    Invalid,
    AveragePartitionLoad,
    AverageServiceLoad,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScalingTriggerKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScalingTriggerKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScalingTriggerKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ScalingTriggerKind", 0u32, "Invalid"),
            Self::AveragePartitionLoad => serializer.serialize_unit_variant("ScalingTriggerKind", 1u32, "AveragePartitionLoad"),
            Self::AverageServiceLoad => serializer.serialize_unit_variant("ScalingTriggerKind", 2u32, "AverageServiceLoad"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Status of the secondary replicator when it is in active mode and is part of the replica set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecondaryActiveReplicatorStatus {
    #[serde(flatten)]
    pub secondary_replicator_status: SecondaryReplicatorStatus,
}
impl SecondaryActiveReplicatorStatus {
    pub fn new(secondary_replicator_status: SecondaryReplicatorStatus) -> Self {
        Self {
            secondary_replicator_status,
        }
    }
}
#[doc = "Status of the secondary replicator when it is in idle mode and is being built by the primary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecondaryIdleReplicatorStatus {
    #[serde(flatten)]
    pub secondary_replicator_status: SecondaryReplicatorStatus,
}
impl SecondaryIdleReplicatorStatus {
    pub fn new(secondary_replicator_status: SecondaryReplicatorStatus) -> Self {
        Self {
            secondary_replicator_status,
        }
    }
}
#[doc = "Provides statistics about the Service Fabric Replicator, when it is functioning in a ActiveSecondary role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecondaryReplicatorStatus {
    #[serde(flatten)]
    pub replicator_status: ReplicatorStatus,
    #[doc = "Provides various statistics of the queue used in the service fabric replicator.\nContains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc.\nDepending on the role of the replicator, the properties in this type imply different meanings."]
    #[serde(rename = "ReplicationQueueStatus", default, skip_serializing_if = "Option::is_none")]
    pub replication_queue_status: Option<ReplicatorQueueStatus>,
    #[doc = "The last time-stamp (UTC) at which a replication operation was received from the primary.\nUTC 0 represents an invalid value, indicating that a replication operation message was never received."]
    #[serde(rename = "LastReplicationOperationReceivedTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_replication_operation_received_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Value that indicates whether the replica is currently being built."]
    #[serde(rename = "IsInBuild", default, skip_serializing_if = "Option::is_none")]
    pub is_in_build: Option<bool>,
    #[doc = "Provides various statistics of the queue used in the service fabric replicator.\nContains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc.\nDepending on the role of the replicator, the properties in this type imply different meanings."]
    #[serde(rename = "CopyQueueStatus", default, skip_serializing_if = "Option::is_none")]
    pub copy_queue_status: Option<ReplicatorQueueStatus>,
    #[doc = "The last time-stamp (UTC) at which a copy operation was received from the primary.\nUTC 0 represents an invalid value, indicating that a copy operation message was never received."]
    #[serde(rename = "LastCopyOperationReceivedTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_copy_operation_received_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The last time-stamp (UTC) at which an acknowledgment was sent to the primary replicator.\nUTC 0 represents an invalid value, indicating that an acknowledgment message was never sent."]
    #[serde(rename = "LastAcknowledgementSentTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub last_acknowledgement_sent_time_utc: Option<time::OffsetDateTime>,
}
impl SecondaryReplicatorStatus {
    pub fn new(replicator_status: ReplicatorStatus) -> Self {
        Self {
            replicator_status,
            replication_queue_status: None,
            last_replication_operation_received_time_utc: None,
            is_in_build: None,
            copy_queue_status: None,
            last_copy_operation_received_time_utc: None,
            last_acknowledgement_sent_time_utc: None,
        }
    }
}
#[doc = "Describes the kind of secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SecretKind")]
pub enum SecretKind {
    #[serde(rename = "inlinedValue")]
    InlinedValue,
    #[serde(rename = "keyVaultVersionedReference")]
    KeyVaultVersionedReference,
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
            Self::KeyVaultVersionedReference => serializer.serialize_unit_variant("SecretKind", 1u32, "keyVaultVersionedReference"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "This type describes a secret resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretResourceDescription {
    #[doc = "Describes the properties of a secret resource."]
    pub properties: SecretResourceProperties,
    #[doc = "Name of the Secret resource."]
    pub name: SecretResourceName,
}
impl SecretResourceDescription {
    pub fn new(properties: SecretResourceProperties, name: SecretResourceName) -> Self {
        Self { properties, name }
    }
}
pub type SecretResourceName = String;
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
    #[doc = "Describes the kind of secret."]
    pub kind: SecretKind,
}
impl SecretResourcePropertiesBase {
    pub fn new(kind: SecretKind) -> Self {
        Self { kind }
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
    #[doc = "Version identifier of the secret value."]
    pub name: String,
    #[doc = "This type describes properties of a secret value resource."]
    pub properties: SecretValueResourceProperties,
}
impl SecretValueResourceDescription {
    pub fn new(name: String, properties: SecretValueResourceProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "This type describes properties of a secret value resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretValueResourceProperties {
    #[serde(flatten)]
    pub secret_value_properties: SecretValueProperties,
}
impl SecretValueResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a safety check for the seed nodes being performed by service fabric before continuing with node level operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeedNodeSafetyCheck {
    #[serde(flatten)]
    pub safety_check: SafetyCheck,
}
impl SeedNodeSafetyCheck {
    pub fn new(safety_check: SafetyCheck) -> Self {
        Self { safety_check }
    }
}
#[doc = "This class returns information about the partition that the user-induced operation acted upon."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelectedPartition {
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
}
impl SelectedPartition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backup configuration information for a specific Service Fabric service specifying what backup policy is being applied and suspend description, if any."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBackupConfigurationInfo {
    #[serde(flatten)]
    pub backup_configuration_info: BackupConfigurationInfo,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
}
impl ServiceBackupConfigurationInfo {
    pub fn new(backup_configuration_info: BackupConfigurationInfo) -> Self {
        Self {
            backup_configuration_info,
            service_name: None,
        }
    }
}
#[doc = "Identifies the Service Fabric stateful service which is being backed up."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBackupEntity {
    #[serde(flatten)]
    pub backup_entity: BackupEntity,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
}
impl ServiceBackupEntity {
    pub fn new(backup_entity: BackupEntity) -> Self {
        Self {
            backup_entity,
            service_name: None,
        }
    }
}
#[doc = "Creates a particular correlation between services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCorrelationDescription {
    #[doc = "The service correlation scheme."]
    #[serde(rename = "Scheme")]
    pub scheme: ServiceCorrelationScheme,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName")]
    pub service_name: ServiceName,
}
impl ServiceCorrelationDescription {
    pub fn new(scheme: ServiceCorrelationScheme, service_name: ServiceName) -> Self {
        Self { scheme, service_name }
    }
}
#[doc = "The service correlation scheme."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceCorrelationScheme")]
pub enum ServiceCorrelationScheme {
    Invalid,
    Affinity,
    AlignedAffinity,
    NonAlignedAffinity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceCorrelationScheme {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceCorrelationScheme {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceCorrelationScheme {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ServiceCorrelationScheme", 0u32, "Invalid"),
            Self::Affinity => serializer.serialize_unit_variant("ServiceCorrelationScheme", 1u32, "Affinity"),
            Self::AlignedAffinity => serializer.serialize_unit_variant("ServiceCorrelationScheme", 2u32, "AlignedAffinity"),
            Self::NonAlignedAffinity => serializer.serialize_unit_variant("ServiceCorrelationScheme", 3u32, "NonAlignedAffinity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Service Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCreatedEvent {
    #[serde(flatten)]
    pub service_event: ServiceEvent,
    #[doc = "Service type name."]
    #[serde(rename = "ServiceTypeName")]
    pub service_type_name: String,
    #[doc = "Application name."]
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    #[doc = "Application type name."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: String,
    #[doc = "Id of Service instance."]
    #[serde(rename = "ServiceInstance")]
    pub service_instance: i64,
    #[doc = "Indicates if Service is stateful."]
    #[serde(rename = "IsStateful")]
    pub is_stateful: bool,
    #[doc = "Number of partitions."]
    #[serde(rename = "PartitionCount")]
    pub partition_count: i32,
    #[doc = "Size of target replicas set."]
    #[serde(rename = "TargetReplicaSetSize")]
    pub target_replica_set_size: i32,
    #[doc = "Minimum size of replicas set."]
    #[serde(rename = "MinReplicaSetSize")]
    pub min_replica_set_size: i32,
    #[doc = "Version of Service package."]
    #[serde(rename = "ServicePackageVersion")]
    pub service_package_version: String,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId")]
    pub partition_id: PartitionId,
}
impl ServiceCreatedEvent {
    pub fn new(
        service_event: ServiceEvent,
        service_type_name: String,
        application_name: String,
        application_type_name: String,
        service_instance: i64,
        is_stateful: bool,
        partition_count: i32,
        target_replica_set_size: i32,
        min_replica_set_size: i32,
        service_package_version: String,
        partition_id: PartitionId,
    ) -> Self {
        Self {
            service_event,
            service_type_name,
            application_name,
            application_type_name,
            service_instance,
            is_stateful,
            partition_count,
            target_replica_set_size,
            min_replica_set_size,
            service_package_version,
            partition_id,
        }
    }
}
#[doc = "Service Deleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDeletedEvent {
    #[serde(flatten)]
    pub service_event: ServiceEvent,
    #[doc = "Service type name."]
    #[serde(rename = "ServiceTypeName")]
    pub service_type_name: String,
    #[doc = "Application name."]
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    #[doc = "Application type name."]
    #[serde(rename = "ApplicationTypeName")]
    pub application_type_name: String,
    #[doc = "Id of Service instance."]
    #[serde(rename = "ServiceInstance")]
    pub service_instance: i64,
    #[doc = "Indicates if Service is stateful."]
    #[serde(rename = "IsStateful")]
    pub is_stateful: bool,
    #[doc = "Number of partitions."]
    #[serde(rename = "PartitionCount")]
    pub partition_count: i32,
    #[doc = "Size of target replicas set."]
    #[serde(rename = "TargetReplicaSetSize")]
    pub target_replica_set_size: i32,
    #[doc = "Minimum size of replicas set."]
    #[serde(rename = "MinReplicaSetSize")]
    pub min_replica_set_size: i32,
    #[doc = "Version of Service package."]
    #[serde(rename = "ServicePackageVersion")]
    pub service_package_version: String,
}
impl ServiceDeletedEvent {
    pub fn new(
        service_event: ServiceEvent,
        service_type_name: String,
        application_name: String,
        application_type_name: String,
        service_instance: i64,
        is_stateful: bool,
        partition_count: i32,
        target_replica_set_size: i32,
        min_replica_set_size: i32,
        service_package_version: String,
    ) -> Self {
        Self {
            service_event,
            service_type_name,
            application_name,
            application_type_name,
            service_instance,
            is_stateful,
            partition_count,
            target_replica_set_size,
            min_replica_set_size,
            service_package_version,
        }
    }
}
#[doc = "A ServiceDescription contains all of the information necessary to create a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDescription {
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "ServiceKind")]
    pub service_kind: ServiceKind,
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<ApplicationName>,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName")]
    pub service_name: ServiceName,
    #[doc = "Name of the service type as specified in the service manifest."]
    #[serde(rename = "ServiceTypeName")]
    pub service_type_name: ServiceTypeName,
    #[doc = "Array of bytes to be sent as an integer array. Each element of array is a number between 0 and 255."]
    #[serde(rename = "InitializationData", default, skip_serializing_if = "Option::is_none")]
    pub initialization_data: Option<ByteArray>,
    #[doc = "Describes how the service is partitioned."]
    #[serde(rename = "PartitionDescription")]
    pub partition_description: PartitionSchemeDescription,
    #[doc = "The placement constraints as a string. Placement constraints are boolean expressions on node properties and allow for restricting a service to particular nodes based on the service requirements. For example, to place a service on nodes where NodeType is blue specify the following: \"NodeColor == blue)\"."]
    #[serde(rename = "PlacementConstraints", default, skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<String>,
    #[doc = "A list that describes the correlation of the service with other services."]
    #[serde(rename = "CorrelationScheme", default, skip_serializing_if = "Option::is_none")]
    pub correlation_scheme: Option<CorrelationSchemeList>,
    #[doc = "The service load metrics is given as an array of ServiceLoadMetricDescription objects."]
    #[serde(rename = "ServiceLoadMetrics", default, skip_serializing_if = "Option::is_none")]
    pub service_load_metrics: Option<ServiceLoadMetricsList>,
    #[doc = "A list that describes the correlation of the service with other services."]
    #[serde(rename = "ServicePlacementPolicies", default, skip_serializing_if = "Option::is_none")]
    pub service_placement_policies: Option<ServicePlacementPoliciesList>,
    #[doc = "Specifies the move cost for the service."]
    #[serde(rename = "DefaultMoveCost", default, skip_serializing_if = "Option::is_none")]
    pub default_move_cost: Option<MoveCost>,
    #[doc = "Indicates if the DefaultMoveCost property is specified."]
    #[serde(rename = "IsDefaultMoveCostSpecified", default, skip_serializing_if = "Option::is_none")]
    pub is_default_move_cost_specified: Option<bool>,
    #[doc = "The activation mode of service package to be used for a Service Fabric service. This is specified at the time of creating the Service."]
    #[serde(rename = "ServicePackageActivationMode", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_mode: Option<ServicePackageActivationMode>,
    #[doc = "The DNS name of the service. It requires the DNS system service to be enabled in Service Fabric cluster."]
    #[serde(rename = "ServiceDnsName", default, skip_serializing_if = "Option::is_none")]
    pub service_dns_name: Option<String>,
    #[doc = "A list that describes the scaling policies."]
    #[serde(rename = "ScalingPolicies", default, skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<ScalingPolicyDescriptionList>,
}
impl ServiceDescription {
    pub fn new(
        service_kind: ServiceKind,
        service_name: ServiceName,
        service_type_name: ServiceTypeName,
        partition_description: PartitionSchemeDescription,
    ) -> Self {
        Self {
            service_kind,
            application_name: None,
            service_name,
            service_type_name,
            initialization_data: None,
            partition_description,
            placement_constraints: None,
            correlation_scheme: None,
            service_load_metrics: None,
            service_placement_policies: None,
            default_move_cost: None,
            is_default_move_cost_specified: None,
            service_package_activation_mode: None,
            service_dns_name: None,
            scaling_policies: None,
        }
    }
}
#[doc = "The role of the replica where the endpoint is reported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceEndpointRole")]
pub enum ServiceEndpointRole {
    Invalid,
    Stateless,
    StatefulPrimary,
    StatefulSecondary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceEndpointRole {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceEndpointRole {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceEndpointRole {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ServiceEndpointRole", 0u32, "Invalid"),
            Self::Stateless => serializer.serialize_unit_variant("ServiceEndpointRole", 1u32, "Stateless"),
            Self::StatefulPrimary => serializer.serialize_unit_variant("ServiceEndpointRole", 2u32, "StatefulPrimary"),
            Self::StatefulSecondary => serializer.serialize_unit_variant("ServiceEndpointRole", 3u32, "StatefulSecondary"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents the base for all Service Events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceEvent {
    #[serde(flatten)]
    pub fabric_event: FabricEvent,
    #[doc = "The identity of the service. This ID is an encoded representation of the service name. This is used in the REST APIs to identify the service resource.\nStarting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the service name is \"fabric:/myapp/app1/svc1\",\nthe service identity would be \"myapp~app1\\~svc1\" in 6.0+ and \"myapp/app1/svc1\" in previous versions."]
    #[serde(rename = "ServiceId")]
    pub service_id: ServiceId,
}
impl ServiceEvent {
    pub fn new(fabric_event: FabricEvent, service_id: ServiceId) -> Self {
        Self { fabric_event, service_id }
    }
}
pub type ServiceEventList = Vec<ServiceEvent>;
#[doc = "Defines description for creating a Service Fabric service from a template defined in the application manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceFromTemplateDescription {
    #[doc = "The name of the application, including the 'fabric:' URI scheme."]
    #[serde(rename = "ApplicationName")]
    pub application_name: ApplicationName,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName")]
    pub service_name: ServiceName,
    #[doc = "Name of the service type as specified in the service manifest."]
    #[serde(rename = "ServiceTypeName")]
    pub service_type_name: ServiceTypeName,
    #[doc = "Array of bytes to be sent as an integer array. Each element of array is a number between 0 and 255."]
    #[serde(rename = "InitializationData", default, skip_serializing_if = "Option::is_none")]
    pub initialization_data: Option<ByteArray>,
    #[doc = "The activation mode of service package to be used for a Service Fabric service. This is specified at the time of creating the Service."]
    #[serde(rename = "ServicePackageActivationMode", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_mode: Option<ServicePackageActivationMode>,
    #[doc = "The DNS name of the service. It requires the DNS system service to be enabled in Service Fabric cluster."]
    #[serde(rename = "ServiceDnsName", default, skip_serializing_if = "Option::is_none")]
    pub service_dns_name: Option<String>,
}
impl ServiceFromTemplateDescription {
    pub fn new(application_name: ApplicationName, service_name: ServiceName, service_type_name: ServiceTypeName) -> Self {
        Self {
            application_name,
            service_name,
            service_type_name,
            initialization_data: None,
            service_package_activation_mode: None,
            service_dns_name: None,
        }
    }
}
#[doc = "Information about the health of a Service Fabric service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceHealth {
    #[serde(flatten)]
    pub entity_health: EntityHealth,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ServiceName>,
    #[doc = "The list of partition health states associated with the service."]
    #[serde(rename = "PartitionHealthStates", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_health_states: Vec<PartitionHealthState>,
}
impl ServiceHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents health evaluation for a service, containing information about the data and the algorithm used by health store to evaluate health. The evaluation is returned only when the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl ServiceHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            service_name: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Service Health Report Expired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceHealthReportExpiredEvent {
    #[serde(flatten)]
    pub service_event: ServiceEvent,
    #[doc = "Id of Service instance."]
    #[serde(rename = "InstanceId")]
    pub instance_id: i64,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl ServiceHealthReportExpiredEvent {
    pub fn new(
        service_event: ServiceEvent,
        instance_id: i64,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            service_event,
            instance_id,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Represents the health state of a service, which contains the service identifier and its aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceHealthState {
    #[serde(flatten)]
    pub entity_health_state: EntityHealthState,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
}
impl ServiceHealthState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the health state chunk of a service, which contains the service name, its aggregated health state and any partitions that respect the filters in the cluster health chunk query description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceHealthStateChunk {
    #[serde(flatten)]
    pub entity_health_state_chunk: EntityHealthStateChunk,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
    #[doc = "The list of partition health state chunks that respect the input filters in the chunk query description.\nReturned by get cluster health state chunks query as part of the parent application hierarchy."]
    #[serde(rename = "PartitionHealthStateChunks", default, skip_serializing_if = "Option::is_none")]
    pub partition_health_state_chunks: Option<PartitionHealthStateChunkList>,
}
impl ServiceHealthStateChunk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of service health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceHealthStateChunkList {
    #[doc = "The list of service health state chunks that respect the input filters in the chunk query."]
    #[serde(rename = "Items", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceHealthStateChunk>,
}
impl ServiceHealthStateChunkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines matching criteria to determine whether a service should be included as a child of an application in the cluster health chunk.\nThe services are only returned if the parent application matches a filter specified in the cluster health chunk query description.\nOne filter can match zero, one or multiple services, depending on its properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceHealthStateFilter {
    #[doc = "The name of the service that matches the filter. The filter is applied only to the specified service, if it exists.\nIf the service doesn't exist, no service is returned in the cluster health chunk based on this filter.\nIf the service exists, it is included as the application's child if the health state matches the other filter properties.\nIf not specified, all services that match the parent filters (if any) are taken into consideration and matched against the other filter members, like health state filter."]
    #[serde(rename = "ServiceNameFilter", default, skip_serializing_if = "Option::is_none")]
    pub service_name_filter: Option<String>,
    #[doc = "The filter for the health state of the services. It allows selecting services if they match the desired health states.\nThe possible values are integer value of one of the following health states. Only services that match the filter are returned. All services are used to evaluate the cluster aggregated health state.\nIf not specified, default value is None, unless the service name is specified. If the filter has default value and service name is specified, the matching service is returned.\nThe state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise 'OR' operator.\nFor example, if the provided value is 6, it matches services with HealthState value of OK (2) and Warning (4).\n\n- Default - Default value. Matches any HealthState. The value is zero.\n- None - Filter that doesn't match any HealthState value. Used in order to return no results on a given collection of states. The value is 1.\n- Ok - Filter that matches input with HealthState value Ok. The value is 2.\n- Warning - Filter that matches input with HealthState value Warning. The value is 4.\n- Error - Filter that matches input with HealthState value Error. The value is 8.\n- All - Filter that matches input with any HealthState value. The value is 65535."]
    #[serde(rename = "HealthStateFilter", default, skip_serializing_if = "Option::is_none")]
    pub health_state_filter: Option<i64>,
    #[doc = "Defines a list of filters that specify which partitions to be included in the returned cluster health chunk as children of the service. The partitions are returned only if the parent service matches a filter.\nIf the list is empty, no partitions are returned. All the partitions are used to evaluate the parent service aggregated health state, regardless of the input filters.\nThe service filter may specify multiple partition filters.\nFor example, it can specify a filter to return all partitions with health state Error and another filter to always include a partition identified by its partition ID."]
    #[serde(rename = "PartitionFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_filters: Vec<PartitionHealthStateFilter>,
}
impl ServiceHealthStateFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceId = String;
#[doc = "Map service identity friendly name to an application identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceIdentity {
    #[doc = "The identity friendly name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The application identity name."]
    #[serde(rename = "identityRef", default, skip_serializing_if = "Option::is_none")]
    pub identity_ref: Option<String>,
}
impl ServiceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a Service Fabric service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceInfo {
    #[doc = "The identity of the service. This ID is an encoded representation of the service name. This is used in the REST APIs to identify the service resource.\nStarting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the service name is \"fabric:/myapp/app1/svc1\",\nthe service identity would be \"myapp~app1\\~svc1\" in 6.0+ and \"myapp/app1/svc1\" in previous versions."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ServiceId>,
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "ServiceKind")]
    pub service_kind: ServiceKind,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ServiceName>,
    #[doc = "Name of the service type as specified in the service manifest."]
    #[serde(rename = "TypeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<ServiceTypeName>,
    #[doc = "The version of the service manifest."]
    #[serde(rename = "ManifestVersion", default, skip_serializing_if = "Option::is_none")]
    pub manifest_version: Option<String>,
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "HealthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[doc = "The status of the application."]
    #[serde(rename = "ServiceStatus", default, skip_serializing_if = "Option::is_none")]
    pub service_status: Option<ServiceStatus>,
    #[doc = "Whether the service is in a service group."]
    #[serde(rename = "IsServiceGroup", default, skip_serializing_if = "Option::is_none")]
    pub is_service_group: Option<bool>,
}
impl ServiceInfo {
    pub fn new(service_kind: ServiceKind) -> Self {
        Self {
            id: None,
            service_kind,
            name: None,
            type_name: None,
            manifest_version: None,
            health_state: None,
            service_status: None,
            is_service_group: None,
        }
    }
}
#[doc = "The kind of service (Stateless or Stateful)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceKind")]
pub enum ServiceKind {
    Invalid,
    Stateless,
    Stateful,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ServiceKind", 0u32, "Invalid"),
            Self::Stateless => serializer.serialize_unit_variant("ServiceKind", 1u32, "Stateless"),
            Self::Stateful => serializer.serialize_unit_variant("ServiceKind", 2u32, "Stateful"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies a metric to load balance a service during runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceLoadMetricDescription {
    #[doc = "The name of the metric. If the service chooses to report load during runtime, the load metric name should match the name that is specified in Name exactly. Note that metric names are case-sensitive."]
    #[serde(rename = "Name")]
    pub name: String,
    #[doc = "Determines the metric weight relative to the other metrics that are configured for this service. During runtime, if two metrics end up in conflict, the Cluster Resource Manager prefers the metric with the higher weight."]
    #[serde(rename = "Weight", default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<ServiceLoadMetricWeight>,
    #[doc = "Used only for Stateful services. The default amount of load, as a number, that this service creates for this metric when it is a Primary replica."]
    #[serde(rename = "PrimaryDefaultLoad", default, skip_serializing_if = "Option::is_none")]
    pub primary_default_load: Option<i64>,
    #[doc = "Used only for Stateful services. The default amount of load, as a number, that this service creates for this metric when it is a Secondary replica."]
    #[serde(rename = "SecondaryDefaultLoad", default, skip_serializing_if = "Option::is_none")]
    pub secondary_default_load: Option<i64>,
    #[doc = "Used only for Stateless services. The default amount of load, as a number, that this service creates for this metric."]
    #[serde(rename = "DefaultLoad", default, skip_serializing_if = "Option::is_none")]
    pub default_load: Option<i64>,
}
impl ServiceLoadMetricDescription {
    pub fn new(name: String) -> Self {
        Self {
            name,
            weight: None,
            primary_default_load: None,
            secondary_default_load: None,
            default_load: None,
        }
    }
}
#[doc = "Determines the metric weight relative to the other metrics that are configured for this service. During runtime, if two metrics end up in conflict, the Cluster Resource Manager prefers the metric with the higher weight."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceLoadMetricWeight")]
pub enum ServiceLoadMetricWeight {
    Zero,
    Low,
    Medium,
    High,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceLoadMetricWeight {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceLoadMetricWeight {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceLoadMetricWeight {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Zero => serializer.serialize_unit_variant("ServiceLoadMetricWeight", 0u32, "Zero"),
            Self::Low => serializer.serialize_unit_variant("ServiceLoadMetricWeight", 1u32, "Low"),
            Self::Medium => serializer.serialize_unit_variant("ServiceLoadMetricWeight", 2u32, "Medium"),
            Self::High => serializer.serialize_unit_variant("ServiceLoadMetricWeight", 3u32, "High"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type ServiceLoadMetricsList = Vec<ServiceLoadMetricDescription>;
pub type ServiceManifestName = String;
pub type ServiceName = String;
#[doc = "Information about the service name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceNameInfo {
    #[doc = "The identity of the service. This ID is an encoded representation of the service name. This is used in the REST APIs to identify the service resource.\nStarting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the service name is \"fabric:/myapp/app1/svc1\",\nthe service identity would be \"myapp~app1\\~svc1\" in 6.0+ and \"myapp/app1/svc1\" in previous versions."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ServiceId>,
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ServiceName>,
}
impl ServiceNameInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service Health Report Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceNewHealthReportEvent {
    #[serde(flatten)]
    pub service_event: ServiceEvent,
    #[doc = "Id of Service instance."]
    #[serde(rename = "InstanceId")]
    pub instance_id: i64,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl ServiceNewHealthReportEvent {
    pub fn new(
        service_event: ServiceEvent,
        instance_id: i64,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            service_event,
            instance_id,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Specifies the current active life-cycle operation on a stateful service replica or stateless service instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceOperationName")]
pub enum ServiceOperationName {
    Unknown,
    None,
    Open,
    ChangeRole,
    Close,
    Abort,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceOperationName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceOperationName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceOperationName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ServiceOperationName", 0u32, "Unknown"),
            Self::None => serializer.serialize_unit_variant("ServiceOperationName", 1u32, "None"),
            Self::Open => serializer.serialize_unit_variant("ServiceOperationName", 2u32, "Open"),
            Self::ChangeRole => serializer.serialize_unit_variant("ServiceOperationName", 3u32, "ChangeRole"),
            Self::Close => serializer.serialize_unit_variant("ServiceOperationName", 4u32, "Close"),
            Self::Abort => serializer.serialize_unit_variant("ServiceOperationName", 5u32, "Abort"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type ServicePackageActivationId = String;
#[doc = "The activation mode of service package to be used for a Service Fabric service. This is specified at the time of creating the Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServicePackageActivationMode")]
pub enum ServicePackageActivationMode {
    SharedProcess,
    ExclusiveProcess,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServicePackageActivationMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServicePackageActivationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServicePackageActivationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SharedProcess => serializer.serialize_unit_variant("ServicePackageActivationMode", 0u32, "SharedProcess"),
            Self::ExclusiveProcess => serializer.serialize_unit_variant("ServicePackageActivationMode", 1u32, "ExclusiveProcess"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information about a partition of a Service Fabric service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePartitionInfo {
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "ServiceKind")]
    pub service_kind: ServiceKind,
    #[doc = "The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc."]
    #[serde(rename = "HealthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[doc = "The status of the service fabric service partition."]
    #[serde(rename = "PartitionStatus", default, skip_serializing_if = "Option::is_none")]
    pub partition_status: Option<ServicePartitionStatus>,
    #[doc = "Information about the partition identity, partitioning scheme and keys supported by it."]
    #[serde(rename = "PartitionInformation", default, skip_serializing_if = "Option::is_none")]
    pub partition_information: Option<PartitionInformation>,
}
impl ServicePartitionInfo {
    pub fn new(service_kind: ServiceKind) -> Self {
        Self {
            service_kind,
            health_state: None,
            partition_status: None,
            partition_information: None,
        }
    }
}
#[doc = "The kind of partitioning scheme used to partition the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServicePartitionKind")]
pub enum ServicePartitionKind {
    Invalid,
    Singleton,
    Int64Range,
    Named,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServicePartitionKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServicePartitionKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServicePartitionKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ServicePartitionKind", 0u32, "Invalid"),
            Self::Singleton => serializer.serialize_unit_variant("ServicePartitionKind", 1u32, "Singleton"),
            Self::Int64Range => serializer.serialize_unit_variant("ServicePartitionKind", 2u32, "Int64Range"),
            Self::Named => serializer.serialize_unit_variant("ServicePartitionKind", 3u32, "Named"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of the service fabric service partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServicePartitionStatus")]
pub enum ServicePartitionStatus {
    Invalid,
    Ready,
    NotReady,
    InQuorumLoss,
    Reconfiguring,
    Deleting,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServicePartitionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServicePartitionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServicePartitionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ServicePartitionStatus", 0u32, "Invalid"),
            Self::Ready => serializer.serialize_unit_variant("ServicePartitionStatus", 1u32, "Ready"),
            Self::NotReady => serializer.serialize_unit_variant("ServicePartitionStatus", 2u32, "NotReady"),
            Self::InQuorumLoss => serializer.serialize_unit_variant("ServicePartitionStatus", 3u32, "InQuorumLoss"),
            Self::Reconfiguring => serializer.serialize_unit_variant("ServicePartitionStatus", 4u32, "Reconfiguring"),
            Self::Deleting => serializer.serialize_unit_variant("ServicePartitionStatus", 5u32, "Deleting"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the policy to be used for placement of a Service Fabric service where a particular fault or upgrade domain should not be used for placement of the instances or replicas of that service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementInvalidDomainPolicyDescription {
    #[serde(flatten)]
    pub service_placement_policy_description: ServicePlacementPolicyDescription,
    #[doc = "The name of the domain that should not be used for placement."]
    #[serde(rename = "DomainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}
impl ServicePlacementInvalidDomainPolicyDescription {
    pub fn new(service_placement_policy_description: ServicePlacementPolicyDescription) -> Self {
        Self {
            service_placement_policy_description,
            domain_name: None,
        }
    }
}
#[doc = "Describes the policy to be used for placement of a Service Fabric service where all replicas must be able to be placed in order for any replicas to be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementNonPartiallyPlaceServicePolicyDescription {
    #[serde(flatten)]
    pub service_placement_policy_description: ServicePlacementPolicyDescription,
}
impl ServicePlacementNonPartiallyPlaceServicePolicyDescription {
    pub fn new(service_placement_policy_description: ServicePlacementPolicyDescription) -> Self {
        Self {
            service_placement_policy_description,
        }
    }
}
pub type ServicePlacementPoliciesList = Vec<ServicePlacementPolicyDescription>;
#[doc = "Describes the policy to be used for placement of a Service Fabric service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementPolicyDescription {
    #[doc = "The type of placement policy for a service fabric service. Following are the possible values."]
    #[serde(rename = "Type")]
    pub type_: ServicePlacementPolicyType,
}
impl ServicePlacementPolicyDescription {
    pub fn new(type_: ServicePlacementPolicyType) -> Self {
        Self { type_ }
    }
}
pub type ServicePlacementPolicyDescriptionList = Vec<ServicePlacementPolicyDescription>;
#[doc = "The type of placement policy for a service fabric service. Following are the possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServicePlacementPolicyType")]
pub enum ServicePlacementPolicyType {
    Invalid,
    InvalidDomain,
    RequireDomain,
    PreferPrimaryDomain,
    RequireDomainDistribution,
    NonPartiallyPlaceService,
    AllowMultipleStatelessInstancesOnNode,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServicePlacementPolicyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServicePlacementPolicyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServicePlacementPolicyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ServicePlacementPolicyType", 0u32, "Invalid"),
            Self::InvalidDomain => serializer.serialize_unit_variant("ServicePlacementPolicyType", 1u32, "InvalidDomain"),
            Self::RequireDomain => serializer.serialize_unit_variant("ServicePlacementPolicyType", 2u32, "RequireDomain"),
            Self::PreferPrimaryDomain => serializer.serialize_unit_variant("ServicePlacementPolicyType", 3u32, "PreferPrimaryDomain"),
            Self::RequireDomainDistribution => {
                serializer.serialize_unit_variant("ServicePlacementPolicyType", 4u32, "RequireDomainDistribution")
            }
            Self::NonPartiallyPlaceService => {
                serializer.serialize_unit_variant("ServicePlacementPolicyType", 5u32, "NonPartiallyPlaceService")
            }
            Self::AllowMultipleStatelessInstancesOnNode => {
                serializer.serialize_unit_variant("ServicePlacementPolicyType", 6u32, "AllowMultipleStatelessInstancesOnNode")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the policy to be used for placement of a Service Fabric service where the service's Primary replicas should optimally be placed in a particular domain.\n\nThis placement policy is usually used with fault domains in scenarios where the Service Fabric cluster is geographically distributed in order to indicate that a service's primary replica should be located in a particular fault domain, which in geo-distributed scenarios usually aligns with regional or datacenter boundaries. Note that since this is an optimization it is possible that the Primary replica may not end up located in this domain due to failures, capacity limits, or other constraints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementPreferPrimaryDomainPolicyDescription {
    #[serde(flatten)]
    pub service_placement_policy_description: ServicePlacementPolicyDescription,
    #[doc = "The name of the domain that should used for placement as per this policy."]
    #[serde(rename = "DomainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}
impl ServicePlacementPreferPrimaryDomainPolicyDescription {
    pub fn new(service_placement_policy_description: ServicePlacementPolicyDescription) -> Self {
        Self {
            service_placement_policy_description,
            domain_name: None,
        }
    }
}
#[doc = "Describes the policy to be used for placement of a Service Fabric service where two replicas from the same partition should never be placed in the same fault or upgrade domain.\n\nWhile this is not common it can expose the service to an increased risk of concurrent failures due to unplanned outages or other cases of subsequent/concurrent failures. As an example, consider a case where replicas are deployed across different data center, with one replica per location. In the event that one of the datacenters goes offline, normally the replica that was placed in that datacenter will be packed into one of the remaining datacenters. If this is not desirable then this policy should be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementRequireDomainDistributionPolicyDescription {
    #[serde(flatten)]
    pub service_placement_policy_description: ServicePlacementPolicyDescription,
    #[doc = "The name of the domain that should used for placement as per this policy."]
    #[serde(rename = "DomainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}
impl ServicePlacementRequireDomainDistributionPolicyDescription {
    pub fn new(service_placement_policy_description: ServicePlacementPolicyDescription) -> Self {
        Self {
            service_placement_policy_description,
            domain_name: None,
        }
    }
}
#[doc = "Describes the policy to be used for placement of a Service Fabric service where the instances or replicas of that service must be placed in a particular domain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementRequiredDomainPolicyDescription {
    #[serde(flatten)]
    pub service_placement_policy_description: ServicePlacementPolicyDescription,
    #[doc = "The name of the domain that should used for placement as per this policy."]
    #[serde(rename = "DomainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}
impl ServicePlacementRequiredDomainPolicyDescription {
    pub fn new(service_placement_policy_description: ServicePlacementPolicyDescription) -> Self {
        Self {
            service_placement_policy_description,
            domain_name: None,
        }
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
    #[doc = "The execution policy of the service"]
    #[serde(rename = "executionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub execution_policy: Option<ExecutionPolicy>,
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
    #[doc = "The service identity list."]
    #[serde(rename = "identityRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub identity_refs: Vec<ServiceIdentity>,
    #[doc = "Dns name of the service."]
    #[serde(rename = "dnsName", default, skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
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
    #[doc = "Name of the Service resource."]
    pub name: ServiceResourceName,
    #[doc = "This type describes properties of a service resource."]
    pub properties: ServiceResourceProperties,
}
impl ServiceResourceDescription {
    pub fn new(name: ServiceResourceName, properties: ServiceResourceProperties) -> Self {
        Self { name, properties }
    }
}
pub type ServiceResourceName = String;
#[doc = "This type describes properties of a service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceProperties {
    #[serde(flatten)]
    pub service_replica_properties: ServiceReplicaProperties,
    #[serde(flatten)]
    pub service_properties: ServiceProperties,
}
impl ServiceResourceProperties {
    pub fn new(service_replica_properties: ServiceReplicaProperties) -> Self {
        Self {
            service_replica_properties,
            service_properties: ServiceProperties::default(),
        }
    }
}
#[doc = "The status of the application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceStatus")]
pub enum ServiceStatus {
    Unknown,
    Active,
    Upgrading,
    Deleting,
    Creating,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ServiceStatus", 0u32, "Unknown"),
            Self::Active => serializer.serialize_unit_variant("ServiceStatus", 1u32, "Active"),
            Self::Upgrading => serializer.serialize_unit_variant("ServiceStatus", 2u32, "Upgrading"),
            Self::Deleting => serializer.serialize_unit_variant("ServiceStatus", 3u32, "Deleting"),
            Self::Creating => serializer.serialize_unit_variant("ServiceStatus", 4u32, "Creating"),
            Self::Failed => serializer.serialize_unit_variant("ServiceStatus", 5u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes a service type defined in the service manifest of a provisioned application type. The properties the ones defined in the service manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceTypeDescription {
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "Kind")]
    pub kind: ServiceKind,
    #[doc = "Indicates whether the service type is a stateful service type or a stateless service type. This property is true if the service type is a stateful service type, false otherwise."]
    #[serde(rename = "IsStateful", default, skip_serializing_if = "Option::is_none")]
    pub is_stateful: Option<bool>,
    #[doc = "Name of the service type as specified in the service manifest."]
    #[serde(rename = "ServiceTypeName", default, skip_serializing_if = "Option::is_none")]
    pub service_type_name: Option<ServiceTypeName>,
    #[doc = "The placement constraint to be used when instantiating this service in a Service Fabric cluster."]
    #[serde(rename = "PlacementConstraints", default, skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<String>,
    #[doc = "The service load metrics is given as an array of ServiceLoadMetricDescription objects."]
    #[serde(rename = "LoadMetrics", default, skip_serializing_if = "Option::is_none")]
    pub load_metrics: Option<ServiceLoadMetricsList>,
    #[doc = "List of service placement policy descriptions."]
    #[serde(rename = "ServicePlacementPolicies", default, skip_serializing_if = "Option::is_none")]
    pub service_placement_policies: Option<ServicePlacementPolicyDescriptionList>,
    #[doc = "List of service type extensions."]
    #[serde(rename = "Extensions", default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ServiceTypeExtensionDescriptionList>,
}
impl ServiceTypeDescription {
    pub fn new(kind: ServiceKind) -> Self {
        Self {
            kind,
            is_stateful: None,
            service_type_name: None,
            placement_constraints: None,
            load_metrics: None,
            service_placement_policies: None,
            extensions: None,
        }
    }
}
#[doc = "Describes extension of a service type defined in the service manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTypeExtensionDescription {
    #[doc = "The name of the extension."]
    #[serde(rename = "Key", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The extension value."]
    #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ServiceTypeExtensionDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceTypeExtensionDescriptionList = Vec<ServiceTypeExtensionDescription>;
#[doc = "Represents the health policy used to evaluate the health of services belonging to a service type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTypeHealthPolicy {
    #[doc = "The maximum allowed percentage of unhealthy partitions per service. Allowed values are Byte values from zero to 100\n\nThe percentage represents the maximum tolerated percentage of partitions that can be unhealthy before the service is considered in error.\nIf the percentage is respected but there is at least one unhealthy partition, the health is evaluated as Warning.\nThe percentage is calculated by dividing the number of unhealthy partitions over the total number of partitions in the service.\nThe computation rounds up to tolerate one failure on small numbers of partitions. Default percentage is zero."]
    #[serde(
        rename = "MaxPercentUnhealthyPartitionsPerService",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_percent_unhealthy_partitions_per_service: Option<i64>,
    #[doc = "The maximum allowed percentage of unhealthy replicas per partition. Allowed values are Byte values from zero to 100.\n\nThe percentage represents the maximum tolerated percentage of replicas that can be unhealthy before the partition is considered in error.\nIf the percentage is respected but there is at least one unhealthy replica, the health is evaluated as Warning.\nThe percentage is calculated by dividing the number of unhealthy replicas over the total number of replicas in the partition.\nThe computation rounds up to tolerate one failure on small numbers of replicas. Default percentage is zero."]
    #[serde(
        rename = "MaxPercentUnhealthyReplicasPerPartition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_percent_unhealthy_replicas_per_partition: Option<i64>,
    #[doc = "The maximum allowed percentage of unhealthy services. Allowed values are Byte values from zero to 100.\n\nThe percentage represents the maximum tolerated percentage of services that can be unhealthy before the application is considered in error.\nIf the percentage is respected but there is at least one unhealthy service, the health is evaluated as Warning.\nThis is calculated by dividing the number of unhealthy services of the specific service type over the total number of services of the specific service type.\nThe computation rounds up to tolerate one failure on small numbers of services. Default percentage is zero."]
    #[serde(rename = "MaxPercentUnhealthyServices", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_unhealthy_services: Option<i64>,
}
impl ServiceTypeHealthPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceTypeHealthPolicyMap = Vec<ServiceTypeHealthPolicyMapItem>;
#[doc = "Defines an item in ServiceTypeHealthPolicyMap."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceTypeHealthPolicyMapItem {
    #[doc = "The key of the service type health policy map item. This is the name of the service type."]
    #[serde(rename = "Key")]
    pub key: String,
    #[doc = "Represents the health policy used to evaluate the health of services belonging to a service type."]
    #[serde(rename = "Value")]
    pub value: ServiceTypeHealthPolicy,
}
impl ServiceTypeHealthPolicyMapItem {
    pub fn new(key: String, value: ServiceTypeHealthPolicy) -> Self {
        Self { key, value }
    }
}
#[doc = "Information about a service type that is defined in a service manifest of a provisioned application type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTypeInfo {
    #[doc = "Describes a service type defined in the service manifest of a provisioned application type. The properties the ones defined in the service manifest."]
    #[serde(rename = "ServiceTypeDescription", default, skip_serializing_if = "Option::is_none")]
    pub service_type_description: Option<ServiceTypeDescription>,
    #[doc = "The name of the service manifest."]
    #[serde(rename = "ServiceManifestName", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_name: Option<ServiceManifestName>,
    #[doc = "The version of the service manifest in which this service type is defined."]
    #[serde(rename = "ServiceManifestVersion", default, skip_serializing_if = "Option::is_none")]
    pub service_manifest_version: Option<String>,
    #[doc = "Indicates whether the service is a service group. If it is, the property value is true otherwise false."]
    #[serde(rename = "IsServiceGroup", default, skip_serializing_if = "Option::is_none")]
    pub is_service_group: Option<bool>,
}
impl ServiceTypeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceTypeInfoList = Vec<ServiceTypeInfo>;
#[doc = "Contains the manifest describing a service type registered as part of an application in a Service Fabric cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTypeManifest {
    #[doc = "The XML manifest as a string."]
    #[serde(rename = "Manifest", default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
}
impl ServiceTypeManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceTypeName = String;
#[doc = "The status of the service type registration on the node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceTypeRegistrationStatus")]
pub enum ServiceTypeRegistrationStatus {
    Invalid,
    Disabled,
    Enabled,
    Registered,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceTypeRegistrationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceTypeRegistrationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceTypeRegistrationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("ServiceTypeRegistrationStatus", 0u32, "Invalid"),
            Self::Disabled => serializer.serialize_unit_variant("ServiceTypeRegistrationStatus", 1u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("ServiceTypeRegistrationStatus", 2u32, "Enabled"),
            Self::Registered => serializer.serialize_unit_variant("ServiceTypeRegistrationStatus", 3u32, "Registered"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A ServiceUpdateDescription contains all of the information necessary to update a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceUpdateDescription {
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "ServiceKind")]
    pub service_kind: ServiceKind,
    #[doc = "Flags indicating whether other properties are set. Each of the associated properties corresponds to a flag, specified below, which, if set, indicate that the property is specified.\nThis property can be a combination of those flags obtained using bitwise 'OR' operator.\nFor example, if the provided value is 6 then the flags for ReplicaRestartWaitDuration (2) and QuorumLossWaitDuration (4) are set.\n\n- None - Does not indicate any other properties are set. The value is zero.\n- TargetReplicaSetSize/InstanceCount - Indicates whether the TargetReplicaSetSize property (for Stateful services) or the InstanceCount property (for Stateless services) is set. The value is 1.\n- ReplicaRestartWaitDuration - Indicates the ReplicaRestartWaitDuration property is set. The value is  2.\n- QuorumLossWaitDuration - Indicates the QuorumLossWaitDuration property is set. The value is 4.\n- StandByReplicaKeepDuration - Indicates the StandByReplicaKeepDuration property is set. The value is 8.\n- MinReplicaSetSize - Indicates the MinReplicaSetSize property is set. The value is 16.\n- PlacementConstraints - Indicates the PlacementConstraints property is set. The value is 32.\n- PlacementPolicyList - Indicates the ServicePlacementPolicies property is set. The value is 64.\n- Correlation - Indicates the CorrelationScheme property is set. The value is 128.\n- Metrics - Indicates the ServiceLoadMetrics property is set. The value is 256.\n- DefaultMoveCost - Indicates the DefaultMoveCost property is set. The value is 512.\n- ScalingPolicy - Indicates the ScalingPolicies property is set. The value is 1024.\n- ServicePlacementTimeLimit - Indicates the ServicePlacementTimeLimit property is set. The value is 2048.\n- MinInstanceCount - Indicates the MinInstanceCount property is set. The value is 4096.\n- MinInstancePercentage - Indicates the MinInstancePercentage property is set. The value is 8192.\n- InstanceCloseDelayDuration - Indicates the InstanceCloseDelayDuration property is set. The value is 16384.\n- DropSourceReplicaOnMove - Indicates the DropSourceReplicaOnMove property is set. The value is 32768."]
    #[serde(rename = "Flags", default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[doc = "The placement constraints as a string. Placement constraints are boolean expressions on node properties and allow for restricting a service to particular nodes based on the service requirements. For example, to place a service on nodes where NodeType is blue specify the following: \"NodeColor == blue)\"."]
    #[serde(rename = "PlacementConstraints", default, skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<String>,
    #[doc = "A list that describes the correlation of the service with other services."]
    #[serde(rename = "CorrelationScheme", default, skip_serializing_if = "Option::is_none")]
    pub correlation_scheme: Option<CorrelationSchemeList>,
    #[doc = "The service load metrics is given as an array of ServiceLoadMetricDescription objects."]
    #[serde(rename = "LoadMetrics", default, skip_serializing_if = "Option::is_none")]
    pub load_metrics: Option<ServiceLoadMetricsList>,
    #[doc = "A list that describes the correlation of the service with other services."]
    #[serde(rename = "ServicePlacementPolicies", default, skip_serializing_if = "Option::is_none")]
    pub service_placement_policies: Option<ServicePlacementPoliciesList>,
    #[doc = "Specifies the move cost for the service."]
    #[serde(rename = "DefaultMoveCost", default, skip_serializing_if = "Option::is_none")]
    pub default_move_cost: Option<MoveCost>,
    #[doc = "A list that describes the scaling policies."]
    #[serde(rename = "ScalingPolicies", default, skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<ScalingPolicyDescriptionList>,
}
impl ServiceUpdateDescription {
    pub fn new(service_kind: ServiceKind) -> Self {
        Self {
            service_kind,
            flags: None,
            placement_constraints: None,
            correlation_scheme: None,
            load_metrics: None,
            service_placement_policies: None,
            default_move_cost: None,
            scaling_policies: None,
        }
    }
}
#[doc = "Information about how many replicas are completed or pending for a specific service during upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceUpgradeProgress {
    #[doc = "Name of the Service resource."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[doc = "The number of replicas that completes the upgrade in the service."]
    #[serde(rename = "CompletedReplicaCount", default, skip_serializing_if = "Option::is_none")]
    pub completed_replica_count: Option<String>,
    #[doc = "The number of replicas that are waiting to be upgraded in the service."]
    #[serde(rename = "PendingReplicaCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_replica_count: Option<String>,
}
impl ServiceUpgradeProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceUpgradeProgressList = Vec<ServiceUpgradeProgress>;
#[doc = "Represents health evaluation for services of a certain service type belonging to an application, containing health evaluations for each unhealthy service that impacted current aggregated health state. Can be returned when evaluating application health and the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Name of the service type of the services."]
    #[serde(rename = "ServiceTypeName", default, skip_serializing_if = "Option::is_none")]
    pub service_type_name: Option<String>,
    #[doc = "Maximum allowed percentage of unhealthy services from the ServiceTypeHealthPolicy."]
    #[serde(rename = "MaxPercentUnhealthyServices", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_unhealthy_services: Option<i64>,
    #[doc = "Total number of services of the current service type in the application from the health store."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl ServicesHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            service_type_name: None,
            max_percent_unhealthy_services: None,
            total_count: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "Describes a setting for the container. The setting file path can be fetched from environment variable \"Fabric_SettingPath\". The path for Windows container is \"C:\\\\secrets\". The path for Linux container is \"/var/secrets\"."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Setting {
    #[doc = "The type of the setting being given in value"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<SettingType>,
    #[doc = "The name of the setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the setting, will be processed based on the type provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Setting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of the setting being given in value"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SettingType")]
pub enum SettingType {
    ClearText,
    KeyVaultReference,
    SecretValueReference,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SettingType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SettingType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SettingType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ClearText => serializer.serialize_unit_variant("SettingType", 0u32, "ClearText"),
            Self::KeyVaultReference => serializer.serialize_unit_variant("SettingType", 1u32, "KeyVaultReference"),
            Self::SecretValueReference => serializer.serialize_unit_variant("SettingType", 2u32, "SecretValueReference"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for SettingType {
    fn default() -> Self {
        Self::ClearText
    }
}
#[doc = "Information about a partition that is singleton. The services with singleton partitioning scheme are effectively non-partitioned. They only have one partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingletonPartitionInformation {
    #[serde(flatten)]
    pub partition_information: PartitionInformation,
}
impl SingletonPartitionInformation {
    pub fn new(partition_information: PartitionInformation) -> Self {
        Self { partition_information }
    }
}
#[doc = "Describes the partition scheme of a singleton-partitioned, or non-partitioned service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingletonPartitionSchemeDescription {
    #[serde(flatten)]
    pub partition_scheme_description: PartitionSchemeDescription,
}
impl SingletonPartitionSchemeDescription {
    pub fn new(partition_scheme_description: PartitionSchemeDescription) -> Self {
        Self {
            partition_scheme_description,
        }
    }
}
#[doc = "Describes the parameters for starting a cluster upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartClusterUpgradeDescription {
    #[doc = "The cluster code version."]
    #[serde(rename = "CodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub code_version: Option<String>,
    #[doc = "The cluster configuration version."]
    #[serde(rename = "ConfigVersion", default, skip_serializing_if = "Option::is_none")]
    pub config_version: Option<String>,
    #[doc = "The kind of upgrade out of the following possible values."]
    #[serde(rename = "UpgradeKind", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_kind: Option<UpgradeKind>,
    #[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
    #[serde(rename = "RollingUpgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub rolling_upgrade_mode: Option<UpgradeMode>,
    #[doc = "The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer)."]
    #[serde(
        rename = "UpgradeReplicaSetCheckTimeoutInSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upgrade_replica_set_check_timeout_in_seconds: Option<UpgradeReplicaSetCheckTimeout>,
    #[doc = "If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data)."]
    #[serde(rename = "ForceRestart", default, skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<ForceRestart>,
    #[doc = "Defines the order in which an upgrade proceeds through the cluster."]
    #[serde(rename = "SortOrder", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<UpgradeSortOrder>,
    #[doc = "Describes the parameters for monitoring an upgrade in Monitored mode."]
    #[serde(rename = "MonitoringPolicy", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_policy: Option<MonitoringPolicyDescription>,
    #[doc = "Defines a health policy used to evaluate the health of the cluster or of a cluster node."]
    #[serde(rename = "ClusterHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cluster_health_policy: Option<ClusterHealthPolicy>,
    #[doc = "When true, enables delta health evaluation rather than absolute health evaluation after completion of each upgrade domain."]
    #[serde(rename = "EnableDeltaHealthEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub enable_delta_health_evaluation: Option<bool>,
    #[doc = "Defines a health policy used to evaluate the health of the cluster during a cluster upgrade."]
    #[serde(rename = "ClusterUpgradeHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cluster_upgrade_health_policy: Option<ClusterUpgradeHealthPolicyObject>,
    #[doc = "Defines the application health policy map used to evaluate the health of an application or one of its children entities."]
    #[serde(rename = "ApplicationHealthPolicyMap", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy_map: Option<ApplicationHealthPolicies>,
    #[doc = "Duration in seconds, to wait before a stateless instance is closed, to allow the active requests to drain gracefully. This would be effective when the instance is closing during the application/cluster\nupgrade, only for those instances which have a non-zero delay duration configured in the service description. See InstanceCloseDelayDurationSeconds property in $ref: \"#/definitions/StatelessServiceDescription.yaml\" for details.\nNote, the default value of InstanceCloseDelayDurationInSeconds is 4294967295, which indicates that the behavior will entirely depend on the delay configured in the stateless service description."]
    #[serde(rename = "InstanceCloseDelayDurationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub instance_close_delay_duration_in_seconds: Option<InstanceCloseDelayDurationInSeconds>,
}
impl StartClusterUpgradeDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Chaos event that gets generated when Chaos is started."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartedChaosEvent {
    #[serde(flatten)]
    pub chaos_event: ChaosEvent,
    #[doc = "Defines all the parameters to configure a Chaos run."]
    #[serde(rename = "ChaosParameters", default, skip_serializing_if = "Option::is_none")]
    pub chaos_parameters: Option<ChaosParameters>,
}
impl StartedChaosEvent {
    pub fn new(chaos_event: ChaosEvent) -> Self {
        Self {
            chaos_event,
            chaos_parameters: None,
        }
    }
}
#[doc = "Stateful Replica Health Report Expired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulReplicaHealthReportExpiredEvent {
    #[serde(flatten)]
    pub replica_event: ReplicaEvent,
    #[doc = "Id of Replica instance."]
    #[serde(rename = "ReplicaInstanceId")]
    pub replica_instance_id: i64,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl StatefulReplicaHealthReportExpiredEvent {
    pub fn new(
        replica_event: ReplicaEvent,
        replica_instance_id: i64,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            replica_event,
            replica_instance_id,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Stateful Replica Health Report Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulReplicaNewHealthReportEvent {
    #[serde(flatten)]
    pub replica_event: ReplicaEvent,
    #[doc = "Id of Replica instance."]
    #[serde(rename = "ReplicaInstanceId")]
    pub replica_instance_id: i64,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl StatefulReplicaNewHealthReportEvent {
    pub fn new(
        replica_event: ReplicaEvent,
        replica_instance_id: i64,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            replica_event,
            replica_instance_id,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Describes a stateful service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulServiceDescription {
    #[serde(flatten)]
    pub service_description: ServiceDescription,
    #[doc = "The target replica set size as a number."]
    #[serde(rename = "TargetReplicaSetSize")]
    pub target_replica_set_size: i64,
    #[doc = "The minimum replica set size as a number."]
    #[serde(rename = "MinReplicaSetSize")]
    pub min_replica_set_size: i64,
    #[doc = "A flag indicating whether this is a persistent service which stores states on the local disk. If it is then the value of this property is true, if not it is false."]
    #[serde(rename = "HasPersistedState")]
    pub has_persisted_state: bool,
    #[doc = "Flags indicating whether other properties are set. Each of the associated properties corresponds to a flag, specified below, which, if set, indicate that the property is specified.\nThis property can be a combination of those flags obtained using bitwise 'OR' operator.\nFor example, if the provided value is 6 then the flags for QuorumLossWaitDuration (2) and StandByReplicaKeepDuration(4) are set.\n\n- None - Does not indicate any other properties are set. The value is zero.\n- ReplicaRestartWaitDuration - Indicates the ReplicaRestartWaitDuration property is set. The value is 1.\n- QuorumLossWaitDuration - Indicates the QuorumLossWaitDuration property is set. The value is 2.\n- StandByReplicaKeepDuration - Indicates the StandByReplicaKeepDuration property is set. The value is 4.\n- ServicePlacementTimeLimit - Indicates the ServicePlacementTimeLimit property is set. The value is 8.\n- DropSourceReplicaOnMove - Indicates the DropSourceReplicaOnMove property is set. The value is 16."]
    #[serde(rename = "Flags", default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i64>,
    #[doc = "The duration, in seconds, between when a replica goes down and when a new replica is created."]
    #[serde(rename = "ReplicaRestartWaitDurationSeconds", default, skip_serializing_if = "Option::is_none")]
    pub replica_restart_wait_duration_seconds: Option<i64>,
    #[doc = "The maximum duration, in seconds, for which a partition is allowed to be in a state of quorum loss."]
    #[serde(rename = "QuorumLossWaitDurationSeconds", default, skip_serializing_if = "Option::is_none")]
    pub quorum_loss_wait_duration_seconds: Option<i64>,
    #[doc = "The definition on how long StandBy replicas should be maintained before being removed."]
    #[serde(rename = "StandByReplicaKeepDurationSeconds", default, skip_serializing_if = "Option::is_none")]
    pub stand_by_replica_keep_duration_seconds: Option<i64>,
    #[doc = "The duration for which replicas can stay InBuild before reporting that build is stuck."]
    #[serde(rename = "ServicePlacementTimeLimitSeconds", default, skip_serializing_if = "Option::is_none")]
    pub service_placement_time_limit_seconds: Option<i64>,
    #[doc = "Indicates whether to drop source Secondary replica even if the target replica has not finished build. If desired behavior is to drop it as soon as possible the value of this property is true, if not it is false."]
    #[serde(rename = "DropSourceReplicaOnMove", default, skip_serializing_if = "Option::is_none")]
    pub drop_source_replica_on_move: Option<bool>,
}
impl StatefulServiceDescription {
    pub fn new(
        service_description: ServiceDescription,
        target_replica_set_size: i64,
        min_replica_set_size: i64,
        has_persisted_state: bool,
    ) -> Self {
        Self {
            service_description,
            target_replica_set_size,
            min_replica_set_size,
            has_persisted_state,
            flags: None,
            replica_restart_wait_duration_seconds: None,
            quorum_loss_wait_duration_seconds: None,
            stand_by_replica_keep_duration_seconds: None,
            service_placement_time_limit_seconds: None,
            drop_source_replica_on_move: None,
        }
    }
}
#[doc = "Information about a stateful Service Fabric service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulServiceInfo {
    #[serde(flatten)]
    pub service_info: ServiceInfo,
    #[doc = "Whether the service has persisted state."]
    #[serde(rename = "HasPersistedState", default, skip_serializing_if = "Option::is_none")]
    pub has_persisted_state: Option<bool>,
}
impl StatefulServiceInfo {
    pub fn new(service_info: ServiceInfo) -> Self {
        Self {
            service_info,
            has_persisted_state: None,
        }
    }
}
#[doc = "Information about a partition of a stateful Service Fabric service.."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulServicePartitionInfo {
    #[serde(flatten)]
    pub service_partition_info: ServicePartitionInfo,
    #[doc = "The target replica set size as a number."]
    #[serde(rename = "TargetReplicaSetSize", default, skip_serializing_if = "Option::is_none")]
    pub target_replica_set_size: Option<i64>,
    #[doc = "The minimum replica set size as a number."]
    #[serde(rename = "MinReplicaSetSize", default, skip_serializing_if = "Option::is_none")]
    pub min_replica_set_size: Option<i64>,
    #[doc = "The duration for which this partition was in quorum loss. If the partition is currently in quorum loss, it returns the duration since it has been in that state. This field is using ISO8601 format for specifying the duration."]
    #[serde(rename = "LastQuorumLossDuration", default, skip_serializing_if = "Option::is_none")]
    pub last_quorum_loss_duration: Option<String>,
    #[doc = "An Epoch is a configuration number for the partition as a whole. When the configuration of the replica set changes, for example when the Primary replica changes, the operations that are replicated from the new Primary replica are said to be a new Epoch from the ones which were sent by the old Primary replica."]
    #[serde(rename = "PrimaryEpoch", default, skip_serializing_if = "Option::is_none")]
    pub primary_epoch: Option<Epoch>,
}
impl StatefulServicePartitionInfo {
    pub fn new(service_partition_info: ServicePartitionInfo) -> Self {
        Self {
            service_partition_info,
            target_replica_set_size: None,
            min_replica_set_size: None,
            last_quorum_loss_duration: None,
            primary_epoch: None,
        }
    }
}
#[doc = "Represents the health of the stateful service replica.\nContains the replica aggregated health state, the health events and the unhealthy evaluations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulServiceReplicaHealth {
    #[serde(flatten)]
    pub replica_health: ReplicaHealth,
    #[doc = "Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id."]
    #[serde(rename = "ReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<ReplicaId>,
}
impl StatefulServiceReplicaHealth {
    pub fn new(replica_health: ReplicaHealth) -> Self {
        Self {
            replica_health,
            replica_id: None,
        }
    }
}
#[doc = "Represents the health state of the stateful service replica, which contains the replica ID and the aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulServiceReplicaHealthState {
    #[serde(flatten)]
    pub replica_health_state: ReplicaHealthState,
    #[doc = "Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id."]
    #[serde(rename = "ReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<ReplicaId>,
}
impl StatefulServiceReplicaHealthState {
    pub fn new(replica_health_state: ReplicaHealthState) -> Self {
        Self {
            replica_health_state,
            replica_id: None,
        }
    }
}
#[doc = "Represents a stateful service replica. This includes information about the identity, role, status, health, node name, uptime, and other details about the replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulServiceReplicaInfo {
    #[serde(flatten)]
    pub replica_info: ReplicaInfo,
    #[doc = "The role of a replica of a stateful service."]
    #[serde(rename = "ReplicaRole", default, skip_serializing_if = "Option::is_none")]
    pub replica_role: Option<ReplicaRole>,
    #[doc = "Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id."]
    #[serde(rename = "ReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<ReplicaId>,
}
impl StatefulServiceReplicaInfo {
    pub fn new(replica_info: ReplicaInfo) -> Self {
        Self {
            replica_info,
            replica_role: None,
            replica_id: None,
        }
    }
}
#[doc = "Describes a stateful service type defined in the service manifest of a provisioned application type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulServiceTypeDescription {
    #[serde(flatten)]
    pub service_type_description: ServiceTypeDescription,
    #[doc = "A flag indicating whether this is a persistent service which stores states on the local disk. If it is then the value of this property is true, if not it is false."]
    #[serde(rename = "HasPersistedState", default, skip_serializing_if = "Option::is_none")]
    pub has_persisted_state: Option<bool>,
}
impl StatefulServiceTypeDescription {
    pub fn new(service_type_description: ServiceTypeDescription) -> Self {
        Self {
            service_type_description,
            has_persisted_state: None,
        }
    }
}
#[doc = "Describes an update for a stateful service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulServiceUpdateDescription {
    #[serde(flatten)]
    pub service_update_description: ServiceUpdateDescription,
    #[doc = "The target replica set size as a number."]
    #[serde(rename = "TargetReplicaSetSize", default, skip_serializing_if = "Option::is_none")]
    pub target_replica_set_size: Option<i64>,
    #[doc = "The minimum replica set size as a number."]
    #[serde(rename = "MinReplicaSetSize", default, skip_serializing_if = "Option::is_none")]
    pub min_replica_set_size: Option<i64>,
    #[doc = "The duration, in seconds, between when a replica goes down and when a new replica is created."]
    #[serde(rename = "ReplicaRestartWaitDurationSeconds", default, skip_serializing_if = "Option::is_none")]
    pub replica_restart_wait_duration_seconds: Option<String>,
    #[doc = "The maximum duration, in seconds, for which a partition is allowed to be in a state of quorum loss."]
    #[serde(rename = "QuorumLossWaitDurationSeconds", default, skip_serializing_if = "Option::is_none")]
    pub quorum_loss_wait_duration_seconds: Option<String>,
    #[doc = "The definition on how long StandBy replicas should be maintained before being removed."]
    #[serde(rename = "StandByReplicaKeepDurationSeconds", default, skip_serializing_if = "Option::is_none")]
    pub stand_by_replica_keep_duration_seconds: Option<String>,
    #[doc = "The duration for which replicas can stay InBuild before reporting that build is stuck."]
    #[serde(rename = "ServicePlacementTimeLimitSeconds", default, skip_serializing_if = "Option::is_none")]
    pub service_placement_time_limit_seconds: Option<String>,
    #[doc = "Indicates whether to drop source Secondary replica even if the target replica has not finished build. If desired behavior is to drop it as soon as possible the value of this property is true, if not it is false."]
    #[serde(rename = "DropSourceReplicaOnMove", default, skip_serializing_if = "Option::is_none")]
    pub drop_source_replica_on_move: Option<bool>,
}
impl StatefulServiceUpdateDescription {
    pub fn new(service_update_description: ServiceUpdateDescription) -> Self {
        Self {
            service_update_description,
            target_replica_set_size: None,
            min_replica_set_size: None,
            replica_restart_wait_duration_seconds: None,
            quorum_loss_wait_duration_seconds: None,
            stand_by_replica_keep_duration_seconds: None,
            service_placement_time_limit_seconds: None,
            drop_source_replica_on_move: None,
        }
    }
}
#[doc = "Stateless Replica Health Report Expired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessReplicaHealthReportExpiredEvent {
    #[serde(flatten)]
    pub replica_event: ReplicaEvent,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl StatelessReplicaHealthReportExpiredEvent {
    pub fn new(
        replica_event: ReplicaEvent,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            replica_event,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Stateless Replica Health Report Created event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessReplicaNewHealthReportEvent {
    #[serde(flatten)]
    pub replica_event: ReplicaEvent,
    #[doc = "Id of report source."]
    #[serde(rename = "SourceId")]
    pub source_id: String,
    #[doc = "Describes the property."]
    #[serde(rename = "Property")]
    pub property: String,
    #[doc = "Describes the property health state."]
    #[serde(rename = "HealthState")]
    pub health_state: String,
    #[doc = "Time to live in milli-seconds."]
    #[serde(rename = "TimeToLiveMs")]
    pub time_to_live_ms: i64,
    #[doc = "Sequence number of report."]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: i64,
    #[doc = "Description of report."]
    #[serde(rename = "Description")]
    pub description: String,
    #[doc = "Indicates the removal when it expires."]
    #[serde(rename = "RemoveWhenExpired")]
    pub remove_when_expired: bool,
    #[doc = "Source time."]
    #[serde(rename = "SourceUtcTimestamp", with = "azure_core::date::rfc3339")]
    pub source_utc_timestamp: time::OffsetDateTime,
}
impl StatelessReplicaNewHealthReportEvent {
    pub fn new(
        replica_event: ReplicaEvent,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: time::OffsetDateTime,
    ) -> Self {
        Self {
            replica_event,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }
}
#[doc = "Describes a stateless service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessServiceDescription {
    #[serde(flatten)]
    pub service_description: ServiceDescription,
    #[doc = "The instance count."]
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,
    #[doc = "MinInstanceCount is the minimum number of instances that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node.\nThe actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ).\nNote, if InstanceCount is set to -1, during MinInstanceCount computation -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service."]
    #[serde(rename = "MinInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_count: Option<MinInstanceCount>,
    #[doc = "MinInstancePercentage is the minimum percentage of InstanceCount that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node.\nThe actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ).\nNote, if InstanceCount is set to -1, during MinInstancePercentage computation, -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service."]
    #[serde(rename = "MinInstancePercentage", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_percentage: Option<MinInstancePercentage>,
    #[doc = "Flags indicating whether other properties are set. Each of the associated properties corresponds to a flag, specified below, which, if set, indicate that the property is specified.\nThis property can be a combination of those flags obtained using bitwise 'OR' operator.\nFor example, if the provided value is 1 then the flags for InstanceCloseDelayDuration is set.\n\n- None - Does not indicate any other properties are set. The value is zero.\n- InstanceCloseDelayDuration - Indicates the InstanceCloseDelayDuration property is set. The value is 1."]
    #[serde(rename = "Flags", default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i64>,
    #[doc = "Duration in seconds, to wait before a stateless instance is closed, to allow the active requests to drain gracefully. This would be effective when the instance is closing during the application/cluster upgrade and disabling node.\nThe endpoint exposed on this instance is removed prior to starting the delay, which prevents new connections to this instance.\nIn addition, clients that have subscribed to service endpoint change events(https://docs.microsoft.com/dotnet/api/system.fabric.fabricclient.servicemanagementclient.registerservicenotificationfilterasync), can do\nthe following upon receiving the endpoint removal notification:\n    - Stop sending new requests to this instance.\n    - Close existing connections after in-flight requests have completed.\n    - Connect to a different instance of the service partition for future requests.\nNote, the default value of InstanceCloseDelayDuration is 0, which indicates that there won't be any delay or removal of the endpoint prior to closing the instance."]
    #[serde(rename = "InstanceCloseDelayDurationSeconds", default, skip_serializing_if = "Option::is_none")]
    pub instance_close_delay_duration_seconds: Option<i64>,
}
impl StatelessServiceDescription {
    pub fn new(service_description: ServiceDescription, instance_count: i64) -> Self {
        Self {
            service_description,
            instance_count,
            min_instance_count: None,
            min_instance_percentage: None,
            flags: None,
            instance_close_delay_duration_seconds: None,
        }
    }
}
#[doc = "Information about a stateless Service Fabric service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessServiceInfo {
    #[serde(flatten)]
    pub service_info: ServiceInfo,
}
impl StatelessServiceInfo {
    pub fn new(service_info: ServiceInfo) -> Self {
        Self { service_info }
    }
}
#[doc = "Represents the health of the stateless service instance.\nContains the instance aggregated health state, the health events and the unhealthy evaluations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessServiceInstanceHealth {
    #[serde(flatten)]
    pub replica_health: ReplicaHealth,
    #[doc = "Id of a stateless service instance. InstanceId is used by Service Fabric to uniquely identify an instance of a partition of a stateless service. It is unique within a partition and does not change for the lifetime of the instance. If the instance has failed over on the same or different node, it will get a different value for the InstanceId."]
    #[serde(rename = "InstanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<InstanceId>,
}
impl StatelessServiceInstanceHealth {
    pub fn new(replica_health: ReplicaHealth) -> Self {
        Self {
            replica_health,
            instance_id: None,
        }
    }
}
#[doc = "Represents the health state of the stateless service instance, which contains the instance ID and the aggregated health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessServiceInstanceHealthState {
    #[serde(flatten)]
    pub replica_health_state: ReplicaHealthState,
    #[doc = "Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id."]
    #[serde(rename = "ReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<ReplicaId>,
}
impl StatelessServiceInstanceHealthState {
    pub fn new(replica_health_state: ReplicaHealthState) -> Self {
        Self {
            replica_health_state,
            replica_id: None,
        }
    }
}
#[doc = "Represents a stateless service instance. This includes information about the identity, status, health, node name, uptime, and other details about the instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessServiceInstanceInfo {
    #[serde(flatten)]
    pub replica_info: ReplicaInfo,
    #[doc = "Id of a stateless service instance. InstanceId is used by Service Fabric to uniquely identify an instance of a partition of a stateless service. It is unique within a partition and does not change for the lifetime of the instance. If the instance has failed over on the same or different node, it will get a different value for the InstanceId."]
    #[serde(rename = "InstanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<InstanceId>,
}
impl StatelessServiceInstanceInfo {
    pub fn new(replica_info: ReplicaInfo) -> Self {
        Self {
            replica_info,
            instance_id: None,
        }
    }
}
#[doc = "Information about a partition of a stateless Service Fabric service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessServicePartitionInfo {
    #[serde(flatten)]
    pub service_partition_info: ServicePartitionInfo,
    #[doc = "Number of instances of this partition."]
    #[serde(rename = "InstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    #[doc = "MinInstanceCount is the minimum number of instances that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node.\nThe actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ).\nNote, if InstanceCount is set to -1, during MinInstanceCount computation -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service."]
    #[serde(rename = "MinInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_count: Option<MinInstanceCount>,
    #[doc = "MinInstancePercentage is the minimum percentage of InstanceCount that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node.\nThe actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ).\nNote, if InstanceCount is set to -1, during MinInstancePercentage computation, -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service."]
    #[serde(rename = "MinInstancePercentage", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_percentage: Option<MinInstancePercentage>,
}
impl StatelessServicePartitionInfo {
    pub fn new(service_partition_info: ServicePartitionInfo) -> Self {
        Self {
            service_partition_info,
            instance_count: None,
            min_instance_count: None,
            min_instance_percentage: None,
        }
    }
}
#[doc = "Describes a stateless service type defined in the service manifest of a provisioned application type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessServiceTypeDescription {
    #[serde(flatten)]
    pub service_type_description: ServiceTypeDescription,
    #[doc = "A flag indicating if this type is not implemented and hosted by a user service process, but is implicitly hosted by a system created process. This value is true for services using the guest executable services, false otherwise."]
    #[serde(rename = "UseImplicitHost", default, skip_serializing_if = "Option::is_none")]
    pub use_implicit_host: Option<bool>,
}
impl StatelessServiceTypeDescription {
    pub fn new(service_type_description: ServiceTypeDescription) -> Self {
        Self {
            service_type_description,
            use_implicit_host: None,
        }
    }
}
#[doc = "Describes an update for a stateless service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessServiceUpdateDescription {
    #[serde(flatten)]
    pub service_update_description: ServiceUpdateDescription,
    #[doc = "The instance count."]
    #[serde(rename = "InstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    #[doc = "MinInstanceCount is the minimum number of instances that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node.\nThe actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ).\nNote, if InstanceCount is set to -1, during MinInstanceCount computation -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service."]
    #[serde(rename = "MinInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_count: Option<MinInstanceCount>,
    #[doc = "MinInstancePercentage is the minimum percentage of InstanceCount that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node.\nThe actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ).\nNote, if InstanceCount is set to -1, during MinInstancePercentage computation, -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service."]
    #[serde(rename = "MinInstancePercentage", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_percentage: Option<MinInstancePercentage>,
    #[doc = "Duration in seconds, to wait before a stateless instance is closed, to allow the active requests to drain gracefully. This would be effective when the instance is closing during the application/cluster upgrade and disabling node.\nThe endpoint exposed on this instance is removed prior to starting the delay, which prevents new connections to this instance.\nIn addition, clients that have subscribed to service endpoint change events(https://docs.microsoft.com/dotnet/api/system.fabric.fabricclient.servicemanagementclient.registerservicenotificationfilterasync), can do\nthe following upon receiving the endpoint removal notification:\n    - Stop sending new requests to this instance.\n    - Close existing connections after in-flight requests have completed.\n    - Connect to a different instance of the service partition for future requests."]
    #[serde(rename = "InstanceCloseDelayDurationSeconds", default, skip_serializing_if = "Option::is_none")]
    pub instance_close_delay_duration_seconds: Option<String>,
}
impl StatelessServiceUpdateDescription {
    pub fn new(service_update_description: ServiceUpdateDescription) -> Self {
        Self {
            service_update_description,
            instance_count: None,
            min_instance_count: None,
            min_instance_percentage: None,
            instance_close_delay_duration_seconds: None,
        }
    }
}
#[doc = "Describes a Chaos event that gets generated when Chaos stops because either the user issued a stop or the time to run was up."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoppedChaosEvent {
    #[serde(flatten)]
    pub chaos_event: ChaosEvent,
    #[doc = "Describes why Chaos stopped. Chaos can stop because of StopChaos API call or the timeToRun provided in ChaosParameters is over."]
    #[serde(rename = "Reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl StoppedChaosEvent {
    pub fn new(chaos_event: ChaosEvent) -> Self {
        Self { chaos_event, reason: None }
    }
}
#[doc = "Describes a Service Fabric property value of type String."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringPropertyValue {
    #[serde(flatten)]
    pub property_value: PropertyValue,
    #[doc = "The data of the property value."]
    #[serde(rename = "Data")]
    pub data: String,
}
impl StringPropertyValue {
    pub fn new(property_value: PropertyValue, data: String) -> Self {
        Self { property_value, data }
    }
}
#[doc = "Derived from PropertyBatchInfo. Represents the property batch succeeding. Contains the results of any \"Get\" operations in the batch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuccessfulPropertyBatchInfo {
    #[serde(flatten)]
    pub property_batch_info: PropertyBatchInfo,
    #[doc = "A map containing the properties that were requested through any \"Get\" property batch operations. The key represents the index of the \"Get\" operation in the original request, in string form. The value is the property. If a property is not found, it will not be in the map."]
    #[serde(rename = "Properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl SuccessfulPropertyBatchInfo {
    pub fn new(property_batch_info: PropertyBatchInfo) -> Self {
        Self {
            property_batch_info,
            properties: None,
        }
    }
}
#[doc = "Represents health evaluation for the fabric:/System application, containing information about the data and the algorithm used by health store to evaluate health. The evaluation is returned only when the aggregated health state of the cluster is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemApplicationHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl SystemApplicationHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            unhealthy_evaluations: None,
        }
    }
}
pub type TargetApplicationName = String;
pub type TargetApplicationTypeVersion = String;
pub type TargetDeploymentName = String;
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
#[doc = "Describes a Chaos event that gets generated when an unexpected event occurs in the Chaos engine.\nFor example, due to the cluster snapshot being inconsistent, while faulting an entity, Chaos found that the entity was already faulted -- which would be an unexpected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestErrorChaosEvent {
    #[serde(flatten)]
    pub chaos_event: ChaosEvent,
    #[doc = "Describes why TestErrorChaosEvent was generated. For example, Chaos tries to fault a partition but finds that the partition is no longer fault tolerant, then a TestErrorEvent gets generated with the reason stating that the partition is not fault tolerant."]
    #[serde(rename = "Reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl TestErrorChaosEvent {
    pub fn new(chaos_event: ChaosEvent) -> Self {
        Self { chaos_event, reason: None }
    }
}
#[doc = "Describes the time based backup schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeBasedBackupScheduleDescription {
    #[serde(flatten)]
    pub backup_schedule_description: BackupScheduleDescription,
    #[doc = "Describes the frequency with which to run the time based backup schedule."]
    #[serde(rename = "ScheduleFrequencyType")]
    pub schedule_frequency_type: BackupScheduleFrequencyType,
    #[doc = "List of days of a week when to trigger the periodic backup. This is valid only when the backup schedule frequency type is weekly."]
    #[serde(rename = "RunDays", default, skip_serializing_if = "Option::is_none")]
    pub run_days: Option<DayOfWeekList>,
    #[doc = "Represents the list of exact time during the day in ISO8601 format. Like '19:00:00' will represent '7PM' during the day. Date specified along with time will be ignored."]
    #[serde(rename = "RunTimes")]
    pub run_times: TimeList,
}
impl TimeBasedBackupScheduleDescription {
    pub fn new(
        backup_schedule_description: BackupScheduleDescription,
        schedule_frequency_type: BackupScheduleFrequencyType,
        run_times: TimeList,
    ) -> Self {
        Self {
            backup_schedule_description,
            schedule_frequency_type,
            run_days: None,
            run_times,
        }
    }
}
pub type TimeList = Vec<time::OffsetDateTime>;
#[doc = "Defines an hour and minute of the day specified in 24 hour time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeOfDay {
    #[doc = "Represents the hour of the day. Value must be between 0 and 23 inclusive."]
    #[serde(rename = "Hour", default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[doc = "Represents the minute of the hour. Value must be between 0 to 59 inclusive."]
    #[serde(rename = "Minute", default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
}
impl TimeOfDay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a time range in a 24 hour day specified by a start and end time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeRange {
    #[doc = "Defines an hour and minute of the day specified in 24 hour time."]
    #[serde(rename = "StartTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeOfDay>,
    #[doc = "Defines an hour and minute of the day specified in 24 hour time."]
    #[serde(rename = "EndTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeOfDay>,
}
impl TimeRange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type UnhealthyEvaluations = Vec<HealthEvaluationWrapper>;
#[doc = "Describes a partitioning scheme where an integer range is allocated evenly across a number of partitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UniformInt64RangePartitionSchemeDescription {
    #[serde(flatten)]
    pub partition_scheme_description: PartitionSchemeDescription,
    #[doc = "The number of partitions."]
    #[serde(rename = "Count")]
    pub count: i64,
    #[doc = "String indicating the lower bound of the partition key range that\nshould be split between the partitions."]
    #[serde(rename = "LowKey")]
    pub low_key: String,
    #[doc = "String indicating the upper bound of the partition key range that\nshould be split between the partitions."]
    #[serde(rename = "HighKey")]
    pub high_key: String,
}
impl UniformInt64RangePartitionSchemeDescription {
    pub fn new(partition_scheme_description: PartitionSchemeDescription, count: i64, low_key: String, high_key: String) -> Self {
        Self {
            partition_scheme_description,
            count,
            low_key,
            high_key,
        }
    }
}
#[doc = "Contains information for an unplaced replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnplacedReplicaInformation {
    #[doc = "The full name of the service with 'fabric:' URI scheme."]
    #[serde(rename = "ServiceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<ServiceName>,
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
    #[doc = "List of reasons due to which a replica cannot be placed."]
    #[serde(rename = "UnplacedReplicaDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub unplaced_replica_details: Vec<String>,
}
impl UnplacedReplicaInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the operation to unregister or unprovision an application type and its version that was registered with the Service Fabric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnprovisionApplicationTypeDescriptionInfo {
    #[doc = "The version of the application type as defined in the application manifest."]
    #[serde(rename = "ApplicationTypeVersion")]
    pub application_type_version: ApplicationTypeVersion,
    #[doc = "The flag indicating whether or not unprovision should occur asynchronously. When set to true, the unprovision operation returns when the request is accepted by the system, and the unprovision operation continues without any timeout limit. The default value is false. However, we recommend setting it to true for large application packages that were provisioned."]
    #[serde(rename = "Async", default, skip_serializing_if = "Option::is_none")]
    pub async_: Option<bool>,
}
impl UnprovisionApplicationTypeDescriptionInfo {
    pub fn new(application_type_version: ApplicationTypeVersion) -> Self {
        Self {
            application_type_version,
            async_: None,
        }
    }
}
#[doc = "Describes the parameters for unprovisioning a cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnprovisionFabricDescription {
    #[doc = "The cluster code package version."]
    #[serde(rename = "CodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub code_version: Option<String>,
    #[doc = "The cluster manifest version."]
    #[serde(rename = "ConfigVersion", default, skip_serializing_if = "Option::is_none")]
    pub config_version: Option<String>,
}
impl UnprovisionFabricDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for updating a cluster upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateClusterUpgradeDescription {
    #[doc = "The type of upgrade out of the following possible values."]
    #[serde(rename = "UpgradeKind", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_kind: Option<UpgradeType>,
    #[doc = "Describes the parameters for updating a rolling upgrade of application or cluster."]
    #[serde(rename = "UpdateDescription", default, skip_serializing_if = "Option::is_none")]
    pub update_description: Option<RollingUpgradeUpdateDescription>,
    #[doc = "Defines a health policy used to evaluate the health of the cluster or of a cluster node."]
    #[serde(rename = "ClusterHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cluster_health_policy: Option<ClusterHealthPolicy>,
    #[doc = "When true, enables delta health evaluation rather than absolute health evaluation after completion of each upgrade domain."]
    #[serde(rename = "EnableDeltaHealthEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub enable_delta_health_evaluation: Option<DeltaHealthEvaluationBool>,
    #[doc = "Defines a health policy used to evaluate the health of the cluster during a cluster upgrade."]
    #[serde(rename = "ClusterUpgradeHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cluster_upgrade_health_policy: Option<ClusterUpgradeHealthPolicyObject>,
    #[doc = "Defines the application health policy map used to evaluate the health of an application or one of its children entities."]
    #[serde(rename = "ApplicationHealthPolicyMap", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy_map: Option<ApplicationHealthPolicies>,
}
impl UpdateClusterUpgradeDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies result of updating load for specified partitions. The output will be ordered based on the partition ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdatePartitionLoadResult {
    #[doc = "An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different."]
    #[serde(rename = "PartitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<PartitionId>,
    #[doc = "If OperationState is Completed - this is 0.  If OperationState is Faulted - this is an error code indicating the reason."]
    #[serde(rename = "PartitionErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub partition_error_code: Option<i64>,
}
impl UpdatePartitionLoadResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents health evaluation for delta unhealthy cluster nodes in an upgrade domain, containing health evaluations for each unhealthy node that impacted current aggregated health state.\nCan be returned during cluster upgrade when cluster aggregated health state is Warning or Error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpgradeDomainDeltaNodesCheckHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Name of the upgrade domain where nodes health is currently evaluated."]
    #[serde(rename = "UpgradeDomainName", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain_name: Option<String>,
    #[doc = "Number of upgrade domain nodes with aggregated heath state Error in the health store at the beginning of the cluster upgrade."]
    #[serde(rename = "BaselineErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub baseline_error_count: Option<i64>,
    #[doc = "Total number of upgrade domain nodes in the health store at the beginning of the cluster upgrade."]
    #[serde(rename = "BaselineTotalCount", default, skip_serializing_if = "Option::is_none")]
    pub baseline_total_count: Option<i64>,
    #[doc = "Maximum allowed percentage of upgrade domain delta unhealthy nodes from the ClusterUpgradeHealthPolicy."]
    #[serde(rename = "MaxPercentDeltaUnhealthyNodes", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_delta_unhealthy_nodes: Option<i64>,
    #[doc = "Total number of upgrade domain nodes in the health store."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl UpgradeDomainDeltaNodesCheckHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            upgrade_domain_name: None,
            baseline_error_count: None,
            baseline_total_count: None,
            max_percent_delta_unhealthy_nodes: None,
            total_count: None,
            unhealthy_evaluations: None,
        }
    }
}
pub type UpgradeDomainDurationString = String;
#[doc = "Information about an upgrade domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeDomainInfo {
    #[doc = "The name of the upgrade domain"]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UpgradeDomainName>,
    #[doc = "The state of the upgrade domain."]
    #[serde(rename = "State", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<UpgradeDomainState>,
}
impl UpgradeDomainInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type UpgradeDomainInfoList = Vec<UpgradeDomainInfo>;
pub type UpgradeDomainName = String;
#[doc = "Represents health evaluation for cluster nodes in an upgrade domain, containing health evaluations for each unhealthy node that impacted current aggregated health state. Can be returned when evaluating cluster health during cluster upgrade and the aggregated health state is either Error or Warning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpgradeDomainNodesHealthEvaluation {
    #[serde(flatten)]
    pub health_evaluation: HealthEvaluation,
    #[doc = "Name of the upgrade domain where nodes health is currently evaluated."]
    #[serde(rename = "UpgradeDomainName", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_domain_name: Option<String>,
    #[doc = "Maximum allowed percentage of unhealthy nodes from the ClusterHealthPolicy."]
    #[serde(rename = "MaxPercentUnhealthyNodes", default, skip_serializing_if = "Option::is_none")]
    pub max_percent_unhealthy_nodes: Option<i64>,
    #[doc = "Total number of nodes in the current upgrade domain."]
    #[serde(rename = "TotalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "List of health evaluations that resulted in the current aggregated health state."]
    #[serde(rename = "UnhealthyEvaluations", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluations: Option<UnhealthyEvaluations>,
}
impl UpgradeDomainNodesHealthEvaluation {
    pub fn new(health_evaluation: HealthEvaluation) -> Self {
        Self {
            health_evaluation,
            upgrade_domain_name: None,
            max_percent_unhealthy_nodes: None,
            total_count: None,
            unhealthy_evaluations: None,
        }
    }
}
#[doc = "The state of the upgrade domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpgradeDomainState")]
pub enum UpgradeDomainState {
    Invalid,
    Pending,
    InProgress,
    Completed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpgradeDomainState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpgradeDomainState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpgradeDomainState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("UpgradeDomainState", 0u32, "Invalid"),
            Self::Pending => serializer.serialize_unit_variant("UpgradeDomainState", 1u32, "Pending"),
            Self::InProgress => serializer.serialize_unit_variant("UpgradeDomainState", 2u32, "InProgress"),
            Self::Completed => serializer.serialize_unit_variant("UpgradeDomainState", 3u32, "Completed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type UpgradeDomainTimeout = String;
pub type UpgradeDuration = String;
pub type UpgradeDurationString = String;
pub type UpgradeFailureTimeUtcString = String;
#[doc = "The kind of upgrade out of the following possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpgradeKind")]
pub enum UpgradeKind {
    Invalid,
    Rolling,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpgradeKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpgradeKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpgradeKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("UpgradeKind", 0u32, "Invalid"),
            Self::Rolling => serializer.serialize_unit_variant("UpgradeKind", 1u32, "Rolling"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for UpgradeKind {
    fn default() -> Self {
        Self::Rolling
    }
}
#[doc = "The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpgradeMode")]
pub enum UpgradeMode {
    Invalid,
    UnmonitoredAuto,
    UnmonitoredManual,
    Monitored,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpgradeMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpgradeMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpgradeMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("UpgradeMode", 0u32, "Invalid"),
            Self::UnmonitoredAuto => serializer.serialize_unit_variant("UpgradeMode", 1u32, "UnmonitoredAuto"),
            Self::UnmonitoredManual => serializer.serialize_unit_variant("UpgradeMode", 2u32, "UnmonitoredManual"),
            Self::Monitored => serializer.serialize_unit_variant("UpgradeMode", 3u32, "Monitored"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for UpgradeMode {
    fn default() -> Self {
        Self::UnmonitoredAuto
    }
}
#[doc = "Service state of Service Fabric Upgrade Orchestration Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeOrchestrationServiceState {
    #[doc = "The state of Service Fabric Upgrade Orchestration Service."]
    #[serde(rename = "ServiceState", default, skip_serializing_if = "Option::is_none")]
    pub service_state: Option<String>,
}
impl UpgradeOrchestrationServiceState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service state summary of Service Fabric Upgrade Orchestration Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeOrchestrationServiceStateSummary {
    #[doc = "The current code version of the cluster."]
    #[serde(rename = "CurrentCodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_code_version: Option<String>,
    #[doc = "The current manifest version of the cluster."]
    #[serde(rename = "CurrentManifestVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_manifest_version: Option<String>,
    #[doc = "The target code version of  the cluster."]
    #[serde(rename = "TargetCodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_code_version: Option<String>,
    #[doc = "The target manifest version of the cluster."]
    #[serde(rename = "TargetManifestVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_manifest_version: Option<String>,
    #[doc = "The type of the pending upgrade of the cluster."]
    #[serde(rename = "PendingUpgradeType", default, skip_serializing_if = "Option::is_none")]
    pub pending_upgrade_type: Option<String>,
}
impl UpgradeOrchestrationServiceStateSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type UpgradeReplicaSetCheckTimeout = i64;
#[doc = "Defines the order in which an upgrade proceeds through the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpgradeSortOrder")]
pub enum UpgradeSortOrder {
    Invalid,
    Default,
    Numeric,
    Lexicographical,
    ReverseNumeric,
    ReverseLexicographical,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpgradeSortOrder {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpgradeSortOrder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpgradeSortOrder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("UpgradeSortOrder", 0u32, "Invalid"),
            Self::Default => serializer.serialize_unit_variant("UpgradeSortOrder", 1u32, "Default"),
            Self::Numeric => serializer.serialize_unit_variant("UpgradeSortOrder", 2u32, "Numeric"),
            Self::Lexicographical => serializer.serialize_unit_variant("UpgradeSortOrder", 3u32, "Lexicographical"),
            Self::ReverseNumeric => serializer.serialize_unit_variant("UpgradeSortOrder", 4u32, "ReverseNumeric"),
            Self::ReverseLexicographical => serializer.serialize_unit_variant("UpgradeSortOrder", 5u32, "ReverseLexicographical"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for UpgradeSortOrder {
    fn default() -> Self {
        Self::Default
    }
}
pub type UpgradeStartTimeUtcString = String;
#[doc = "The state of the upgrade domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpgradeState")]
pub enum UpgradeState {
    Invalid,
    RollingBackInProgress,
    RollingBackCompleted,
    RollingForwardPending,
    RollingForwardInProgress,
    RollingForwardCompleted,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpgradeState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpgradeState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpgradeState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("UpgradeState", 0u32, "Invalid"),
            Self::RollingBackInProgress => serializer.serialize_unit_variant("UpgradeState", 1u32, "RollingBackInProgress"),
            Self::RollingBackCompleted => serializer.serialize_unit_variant("UpgradeState", 2u32, "RollingBackCompleted"),
            Self::RollingForwardPending => serializer.serialize_unit_variant("UpgradeState", 3u32, "RollingForwardPending"),
            Self::RollingForwardInProgress => serializer.serialize_unit_variant("UpgradeState", 4u32, "RollingForwardInProgress"),
            Self::RollingForwardCompleted => serializer.serialize_unit_variant("UpgradeState", 5u32, "RollingForwardCompleted"),
            Self::Failed => serializer.serialize_unit_variant("UpgradeState", 6u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type UpgradeTimeout = String;
#[doc = "The type of upgrade out of the following possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpgradeType")]
pub enum UpgradeType {
    Invalid,
    Rolling,
    #[serde(rename = "Rolling_ForceRestart")]
    RollingForceRestart,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpgradeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpgradeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpgradeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("UpgradeType", 0u32, "Invalid"),
            Self::Rolling => serializer.serialize_unit_variant("UpgradeType", 1u32, "Rolling"),
            Self::RollingForceRestart => serializer.serialize_unit_variant("UpgradeType", 2u32, "Rolling_ForceRestart"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for UpgradeType {
    fn default() -> Self {
        Self::Rolling
    }
}
#[doc = "Information about which portion of the file to upload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadChunkRange {
    #[doc = "The start position of the portion of the file. It's represented by the number of bytes."]
    #[serde(rename = "StartPosition", default, skip_serializing_if = "Option::is_none")]
    pub start_position: Option<String>,
    #[doc = "The end position of the portion of the file. It's represented by the number of bytes."]
    #[serde(rename = "EndPosition", default, skip_serializing_if = "Option::is_none")]
    pub end_position: Option<String>,
}
impl UploadChunkRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a image store upload session"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadSession {
    #[doc = "When querying upload session by upload session ID, the result contains only one upload session. When querying upload session by image store relative path, the result might contain multiple upload sessions."]
    #[serde(rename = "UploadSessions", default, skip_serializing_if = "Vec::is_empty")]
    pub upload_sessions: Vec<UploadSessionInfo>,
}
impl UploadSession {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about an image store upload session. A session is associated with a relative path in the image store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadSessionInfo {
    #[doc = "The remote location within image store. This path is relative to the image store root."]
    #[serde(rename = "StoreRelativePath", default, skip_serializing_if = "Option::is_none")]
    pub store_relative_path: Option<String>,
    #[doc = "A unique ID of the upload session. A session ID can be reused only if the session was committed or removed."]
    #[serde(rename = "SessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "The date and time when the upload session was last modified."]
    #[serde(rename = "ModifiedDate", with = "azure_core::date::rfc3339::option")]
    pub modified_date: Option<time::OffsetDateTime>,
    #[doc = "The size in bytes of the uploading file."]
    #[serde(rename = "FileSize", default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    #[doc = "List of chunk ranges that image store has not received yet."]
    #[serde(rename = "ExpectedRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub expected_ranges: Vec<UploadChunkRange>,
}
impl UploadSessionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about how much space and how many files in the file system the ImageStore is using in this category"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageInfo {
    #[doc = "the size of all files in this category"]
    #[serde(rename = "UsedSpace", default, skip_serializing_if = "Option::is_none")]
    pub used_space: Option<String>,
    #[doc = "the number of all files in this category"]
    #[serde(rename = "FileCount", default, skip_serializing_if = "Option::is_none")]
    pub file_count: Option<String>,
}
impl UsageInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a map that contains user assigned identities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentityMap {}
impl UserAssignedIdentityMap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Chaos event corresponding to a failure during validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationFailedChaosEvent {
    #[serde(flatten)]
    pub chaos_event: ChaosEvent,
    #[doc = "Describes why the ValidationFailedChaosEvent was generated. This may happen because more than MaxPercentUnhealthyNodes are unhealthy for more than MaxClusterStabilizationTimeout. This reason will be in the Reason property of the ValidationFailedChaosEvent as a string."]
    #[serde(rename = "Reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ValidationFailedChaosEvent {
    pub fn new(chaos_event: ChaosEvent) -> Self {
        Self { chaos_event, reason: None }
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
    #[doc = "Name of the Volume resource."]
    pub name: VolumeResourceName,
    #[doc = "Describes properties of a volume resource."]
    pub properties: VolumeProperties,
}
impl VolumeResourceDescription {
    pub fn new(name: VolumeResourceName, properties: VolumeProperties) -> Self {
        Self { name, properties }
    }
}
pub type VolumeResourceName = String;
#[doc = "Safety check that waits for the replica build operation to finish. This indicates that there is a replica that is going through the copy or is providing data for building another replica. Bring the node down will abort this copy operation which are typically expensive involving data movements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitForInbuildReplicaSafetyCheck {
    #[serde(flatten)]
    pub partition_safety_check: PartitionSafetyCheck,
}
impl WaitForInbuildReplicaSafetyCheck {
    pub fn new(partition_safety_check: PartitionSafetyCheck) -> Self {
        Self { partition_safety_check }
    }
}
#[doc = "Safety check that waits for the primary replica that was moved out of the node due to upgrade to be placed back again on that node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitForPrimaryPlacementSafetyCheck {
    #[serde(flatten)]
    pub partition_safety_check: PartitionSafetyCheck,
}
impl WaitForPrimaryPlacementSafetyCheck {
    pub fn new(partition_safety_check: PartitionSafetyCheck) -> Self {
        Self { partition_safety_check }
    }
}
#[doc = "Safety check that waits for the primary replica to be moved out of the node before starting an upgrade to ensure the availability of the primary replica for the partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitForPrimarySwapSafetyCheck {
    #[serde(flatten)]
    pub partition_safety_check: PartitionSafetyCheck,
}
impl WaitForPrimarySwapSafetyCheck {
    pub fn new(partition_safety_check: PartitionSafetyCheck) -> Self {
        Self { partition_safety_check }
    }
}
#[doc = "Safety check that waits for the current reconfiguration of the partition to be completed before starting an upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitForReconfigurationSafetyCheck {
    #[serde(flatten)]
    pub partition_safety_check: PartitionSafetyCheck,
}
impl WaitForReconfigurationSafetyCheck {
    pub fn new(partition_safety_check: PartitionSafetyCheck) -> Self {
        Self { partition_safety_check }
    }
}
#[doc = "Describes a Chaos event that gets generated when Chaos is waiting for the cluster to become ready for faulting, for example, Chaos may be waiting for the on-going upgrade to finish."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitingChaosEvent {
    #[serde(flatten)]
    pub chaos_event: ChaosEvent,
    #[doc = "Describes why the WaitingChaosEvent was generated, for example, due to a cluster upgrade."]
    #[serde(rename = "Reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl WaitingChaosEvent {
    pub fn new(chaos_event: ChaosEvent) -> Self {
        Self { chaos_event, reason: None }
    }
}
