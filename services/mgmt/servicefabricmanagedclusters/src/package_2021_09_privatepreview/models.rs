#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents a scaling mechanism for adding or removing named partitions of a stateless service. Partition names are in the format '0','1'...'N-1'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddRemoveIncrementalNamedPartitionScalingMechanism {
    #[serde(flatten)]
    pub scaling_mechanism: ScalingMechanism,
    #[doc = "Minimum number of named partitions of the service."]
    #[serde(rename = "minPartitionCount")]
    pub min_partition_count: i32,
    #[doc = "Maximum number of named partitions of the service."]
    #[serde(rename = "maxPartitionCount")]
    pub max_partition_count: i32,
    #[doc = "The number of instances to add or remove during a scaling operation."]
    #[serde(rename = "scaleIncrement")]
    pub scale_increment: i32,
}
impl AddRemoveIncrementalNamedPartitionScalingMechanism {
    pub fn new(scaling_mechanism: ScalingMechanism, min_partition_count: i32, max_partition_count: i32, scale_increment: i32) -> Self {
        Self {
            scaling_mechanism,
            min_partition_count,
            max_partition_count,
            scale_increment,
        }
    }
}
#[doc = "Defines a health policy used to evaluate the health of an application or one of its children entities.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationHealthPolicy {
    #[doc = "Indicates whether warnings are treated with the same severity as errors."]
    #[serde(rename = "considerWarningAsError")]
    pub consider_warning_as_error: bool,
    #[doc = "The maximum allowed percentage of unhealthy deployed applications. Allowed values are Byte values from zero to 100.\nThe percentage represents the maximum tolerated percentage of deployed applications that can be unhealthy before the application is considered in error.\nThis is calculated by dividing the number of unhealthy deployed applications over the number of nodes where the application is currently deployed on in the cluster.\nThe computation rounds up to tolerate one failure on small numbers of nodes. Default percentage is zero.\n"]
    #[serde(rename = "maxPercentUnhealthyDeployedApplications")]
    pub max_percent_unhealthy_deployed_applications: i32,
    #[doc = "Represents the health policy used to evaluate the health of services belonging to a service type.\n"]
    #[serde(rename = "defaultServiceTypeHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub default_service_type_health_policy: Option<ServiceTypeHealthPolicy>,
    #[doc = "Defines a ServiceTypeHealthPolicy per service type name.\n\nThe entries in the map replace the default service type health policy for each specified service type.\nFor example, in an application that contains both a stateless gateway service type and a stateful engine service type, the health policies for the stateless and stateful services can be configured differently.\nWith policy per service type, there's more granular control of the health of the service.\n\nIf no policy is specified for a service type name, the DefaultServiceTypeHealthPolicy is used for evaluation.\n"]
    #[serde(rename = "serviceTypeHealthPolicyMap", default, skip_serializing_if = "Option::is_none")]
    pub service_type_health_policy_map: Option<ServiceTypeHealthPolicyMap>,
}
impl ApplicationHealthPolicy {
    pub fn new(consider_warning_as_error: bool, max_percent_unhealthy_deployed_applications: i32) -> Self {
        Self {
            consider_warning_as_error,
            max_percent_unhealthy_deployed_applications,
            default_service_type_health_policy: None,
            service_type_health_policy_map: None,
        }
    }
}
#[doc = "List of application parameters with overridden values from their default values specified in the application manifest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationParameterList {}
impl ApplicationParameterList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The application resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Describes the managed identities for an Azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
    #[doc = "The application resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationResourceProperties>,
}
impl ApplicationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of application resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationResource>,
    #[doc = "URL to get the next set of application list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApplicationResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The application resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResourceProperties {
    #[doc = "The current deployment or provisioning state, which only appears in the response"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The version of the application type as defined in the application manifest.\nThis name must be the full Arm Resource ID for the referenced application type version.\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ApplicationTypeVersion>,
    #[doc = "List of application parameters with overridden values from their default values specified in the application manifest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ApplicationParameterList>,
    #[doc = "Describes the policy for a monitored application upgrade."]
    #[serde(rename = "upgradePolicy", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_policy: Option<ApplicationUpgradePolicy>,
    #[doc = "List of user assigned identities for the application, each mapped to a friendly name."]
    #[serde(rename = "managedIdentities", default, skip_serializing_if = "Vec::is_empty")]
    pub managed_identities: Vec<ApplicationUserAssignedIdentity>,
}
impl ApplicationResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The application type name resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The application type name properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationTypeResourceProperties>,
}
impl ApplicationTypeResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of application type names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationTypeResource>,
    #[doc = "URL to get the next set of application type list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationTypeResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApplicationTypeResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The application type name properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeResourceProperties {
    #[doc = "The current deployment or provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ApplicationTypeResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application type update request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeUpdateParameters {
    #[doc = "Application type update parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ApplicationTypeUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ApplicationTypeVersion = String;
#[doc = "An application type version resource for the specified application type name resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeVersionResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the application type version resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationTypeVersionResourceProperties>,
}
impl ApplicationTypeVersionResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of application type version resources for the specified application type name resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeVersionResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationTypeVersionResource>,
    #[doc = "URL to get the next set of application type version list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationTypeVersionResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApplicationTypeVersionResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the application type version resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationTypeVersionResourceProperties {
    #[doc = "The current deployment or provisioning state, which only appears in the response"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The URL to the application package"]
    #[serde(rename = "appPackageUrl")]
    pub app_package_url: String,
}
impl ApplicationTypeVersionResourceProperties {
    pub fn new(app_package_url: String) -> Self {
        Self {
            provisioning_state: None,
            app_package_url,
        }
    }
}
#[doc = "Application type version update request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeVersionUpdateParameters {
    #[doc = "Application type version update parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ApplicationTypeVersionUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy used to clean up unused versions. When the policy is not specified explicitly, the default unused application versions to keep will be 3."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationTypeVersionsCleanupPolicy {
    #[doc = "Number of unused versions per application type to keep."]
    #[serde(rename = "maxUnusedVersionsToKeep")]
    pub max_unused_versions_to_keep: i32,
}
impl ApplicationTypeVersionsCleanupPolicy {
    pub fn new(max_unused_versions_to_keep: i32) -> Self {
        Self {
            max_unused_versions_to_keep,
        }
    }
}
#[doc = "Application update request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationUpdateParameters {
    #[doc = "Application update parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ApplicationUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the policy for a monitored application upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationUpgradePolicy {
    #[doc = "Defines a health policy used to evaluate the health of an application or one of its children entities.\n"]
    #[serde(rename = "applicationHealthPolicy", default, skip_serializing_if = "Option::is_none")]
    pub application_health_policy: Option<ApplicationHealthPolicy>,
    #[doc = "If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data)."]
    #[serde(rename = "forceRestart", default, skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<ForceRestart>,
    #[doc = "The policy used for monitoring the application upgrade"]
    #[serde(rename = "rollingUpgradeMonitoringPolicy", default, skip_serializing_if = "Option::is_none")]
    pub rolling_upgrade_monitoring_policy: Option<RollingUpgradeMonitoringPolicy>,
    #[doc = "Duration in seconds, to wait before a stateless instance is closed, to allow the active requests to drain gracefully. This would be effective when the instance is closing during the application/cluster upgrade, only for those instances which have a non-zero delay duration configured in the service description."]
    #[serde(rename = "instanceCloseDelayDuration", default, skip_serializing_if = "Option::is_none")]
    pub instance_close_delay_duration: Option<i64>,
    #[doc = "The mode used to monitor health during a rolling upgrade. The values are Monitored, and UnmonitoredAuto."]
    #[serde(rename = "upgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_mode: Option<RollingUpgradeMode>,
    #[doc = "The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer)."]
    #[serde(rename = "upgradeReplicaSetCheckTimeout", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_replica_set_check_timeout: Option<i64>,
    #[doc = "Determines whether the application should be recreated on update. If value=true, the rest of the upgrade policy parameters are not allowed."]
    #[serde(rename = "recreateApplication", default, skip_serializing_if = "Option::is_none")]
    pub recreate_application: Option<bool>,
}
impl ApplicationUpgradePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUserAssignedIdentity {
    #[doc = "The friendly name of user assigned identity."]
    pub name: String,
    #[doc = "The principal id of user assigned identity."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
}
impl ApplicationUserAssignedIdentity {
    pub fn new(name: String, principal_id: String) -> Self {
        Self { name, principal_id }
    }
}
#[doc = "Operation supported by the Service Fabric resource provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperationDisplay {
    #[doc = "The name of the provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource on which the operation is performed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The operation that can be performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Operation description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AvailableOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a scaling trigger related to an average load of a metric/resource of a partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AveragePartitionLoadScalingTrigger {
    #[serde(flatten)]
    pub scaling_trigger: ScalingTrigger,
    #[doc = "The name of the metric for which usage should be tracked."]
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[doc = "The lower limit of the load below which a scale in operation should be performed."]
    #[serde(rename = "lowerLoadThreshold")]
    pub lower_load_threshold: f64,
    #[doc = "The upper limit of the load beyond which a scale out operation should be performed."]
    #[serde(rename = "upperLoadThreshold")]
    pub upper_load_threshold: f64,
    #[doc = "The period in seconds on which a decision is made whether to scale or not. This property should come in ISO 8601 format \"hh:mm:ss\"."]
    #[serde(rename = "scaleInterval")]
    pub scale_interval: String,
}
impl AveragePartitionLoadScalingTrigger {
    pub fn new(
        scaling_trigger: ScalingTrigger,
        metric_name: String,
        lower_load_threshold: f64,
        upper_load_threshold: f64,
        scale_interval: String,
    ) -> Self {
        Self {
            scaling_trigger,
            metric_name,
            lower_load_threshold,
            upper_load_threshold,
            scale_interval,
        }
    }
}
#[doc = "Represents a scaling policy related to an average load of a metric/resource of a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AverageServiceLoadScalingTrigger {
    #[serde(flatten)]
    pub scaling_trigger: ScalingTrigger,
    #[doc = "The name of the metric for which usage should be tracked."]
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[doc = "The lower limit of the load below which a scale in operation should be performed."]
    #[serde(rename = "lowerLoadThreshold")]
    pub lower_load_threshold: f64,
    #[doc = "The upper limit of the load beyond which a scale out operation should be performed."]
    #[serde(rename = "upperLoadThreshold")]
    pub upper_load_threshold: f64,
    #[doc = "The period in seconds on which a decision is made whether to scale or not. This property should come in ISO 8601 format \"hh:mm:ss\"."]
    #[serde(rename = "scaleInterval")]
    pub scale_interval: String,
    #[doc = "Flag determines whether only the load of primary replica should be considered for scaling. If set to true, then trigger will only consider the load of primary replicas of stateful service. If set to false, trigger will consider load of all replicas. This parameter cannot be set to true for stateless service."]
    #[serde(rename = "useOnlyPrimaryLoad")]
    pub use_only_primary_load: bool,
}
impl AverageServiceLoadScalingTrigger {
    pub fn new(
        scaling_trigger: ScalingTrigger,
        metric_name: String,
        lower_load_threshold: f64,
        upper_load_threshold: f64,
        scale_interval: String,
        use_only_primary_load: bool,
    ) -> Self {
        Self {
            scaling_trigger,
            metric_name,
            lower_load_threshold,
            upper_load_threshold,
            scale_interval,
            use_only_primary_load,
        }
    }
}
#[doc = "The settings to enable AAD authentication on the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureActiveDirectory {
    #[doc = "Azure active directory tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Azure active directory cluster application id."]
    #[serde(rename = "clusterApplication", default, skip_serializing_if = "Option::is_none")]
    pub cluster_application: Option<String>,
    #[doc = "Azure active directory client application id."]
    #[serde(rename = "clientApplication", default, skip_serializing_if = "Option::is_none")]
    pub client_application: Option<String>,
}
impl AzureActiveDirectory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Client certificate definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientCertificate {
    #[doc = "Indicates if the client certificate has admin access to the cluster. Non admin clients can perform only read only operations on the cluster."]
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[doc = "Certificate thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Certificate common name."]
    #[serde(rename = "commonName", default, skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    #[doc = "Issuer thumbprint for the certificate. Only used together with CommonName."]
    #[serde(rename = "issuerThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub issuer_thumbprint: Option<String>,
}
impl ClientCertificate {
    pub fn new(is_admin: bool) -> Self {
        Self {
            is_admin,
            thumbprint: None,
            common_name: None,
            issuer_thumbprint: None,
        }
    }
}
#[doc = "The current state of the cluster.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ClusterState")]
pub enum ClusterState {
    WaitingForNodes,
    Deploying,
    BaselineUpgrade,
    Upgrading,
    UpgradeFailed,
    Ready,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ClusterState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ClusterState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ClusterState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::WaitingForNodes => serializer.serialize_unit_variant("ClusterState", 0u32, "WaitingForNodes"),
            Self::Deploying => serializer.serialize_unit_variant("ClusterState", 1u32, "Deploying"),
            Self::BaselineUpgrade => serializer.serialize_unit_variant("ClusterState", 2u32, "BaselineUpgrade"),
            Self::Upgrading => serializer.serialize_unit_variant("ClusterState", 3u32, "Upgrading"),
            Self::UpgradeFailed => serializer.serialize_unit_variant("ClusterState", 4u32, "UpgradeFailed"),
            Self::Ready => serializer.serialize_unit_variant("ClusterState", 5u32, "Ready"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates when new cluster runtime version upgrades will be applied after they are released. By default is Wave0."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ClusterUpgradeCadence")]
pub enum ClusterUpgradeCadence {
    Wave0,
    Wave1,
    Wave2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ClusterUpgradeCadence {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ClusterUpgradeCadence {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ClusterUpgradeCadence {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Wave0 => serializer.serialize_unit_variant("ClusterUpgradeCadence", 0u32, "Wave0"),
            Self::Wave1 => serializer.serialize_unit_variant("ClusterUpgradeCadence", 1u32, "Wave1"),
            Self::Wave2 => serializer.serialize_unit_variant("ClusterUpgradeCadence", 2u32, "Wave2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The upgrade mode of the cluster when new Service Fabric runtime version is available.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ClusterUpgradeMode")]
pub enum ClusterUpgradeMode {
    Automatic,
    Manual,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ClusterUpgradeMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ClusterUpgradeMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ClusterUpgradeMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Automatic => serializer.serialize_unit_variant("ClusterUpgradeMode", 0u32, "Automatic"),
            Self::Manual => serializer.serialize_unit_variant("ClusterUpgradeMode", 1u32, "Manual"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ClusterUpgradeMode {
    fn default() -> Self {
        Self::Automatic
    }
}
pub type CorrelationSchemeList = Vec<ServiceCorrelation>;
#[doc = "Managed data disk type. IOPS and throughput are given by the disk size, to see more information go to https://docs.microsoft.com/en-us/azure/virtual-machines/disks-types.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiskType")]
pub enum DiskType {
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "StandardSSD_LRS")]
    StandardSsdLrs,
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiskType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiskType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiskType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StandardLrs => serializer.serialize_unit_variant("DiskType", 0u32, "Standard_LRS"),
            Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskType", 1u32, "StandardSSD_LRS"),
            Self::PremiumLrs => serializer.serialize_unit_variant("DiskType", 2u32, "Premium_LRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for DiskType {
    fn default() -> Self {
        Self::StandardSsdLrs
    }
}
#[doc = "Port range details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointRangeDescription {
    #[doc = "Starting port of a range of ports"]
    #[serde(rename = "startPort")]
    pub start_port: i32,
    #[doc = "End port of a range of ports"]
    #[serde(rename = "endPort")]
    pub end_port: i32,
}
impl EndpointRangeDescription {
    pub fn new(start_port: i32, end_port: i32) -> Self {
        Self { start_port, end_port }
    }
}
#[doc = "The structure of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorModel {
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorModelError>,
}
impl azure_core::Continuable for ErrorModel {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorModelError {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorModelError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ForceRestart = bool;
#[doc = "Describes the frontend configurations for the node type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendConfiguration {
    #[doc = "The IP address type.\n"]
    #[serde(rename = "ipAddressType", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<IpAddressType>,
    #[doc = "The resource Id of the Load Balancer backend address pool that the VM instances of the node type are associated with. The format of the resource Id is '/subscriptions/<subscriptionId>/resourceGroups/<resourceGroupName>/providers/Microsoft.Network/loadBalancers/<loadBalancerName>/backendAddressPools/<backendAddressPoolName>'."]
    #[serde(rename = "loadBalancerBackendAddressPoolId", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_backend_address_pool_id: Option<String>,
    #[doc = "The resource Id of the Load Balancer inbound NAT pool that the VM instances of the node type are associated with. The format of the resource Id is '/subscriptions/<subscriptionId>/resourceGroups/<resourceGroupName>/providers/Microsoft.Network/loadBalancers/<loadBalancerName>/inboundNatPools/<inboundNatPoolName>'."]
    #[serde(rename = "loadBalancerInboundNatPoolId", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_inbound_nat_pool_id: Option<String>,
}
impl FrontendConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type HealthCheckRetryTimeout = String;
pub type HealthCheckStableDuration = String;
pub type HealthCheckWaitDuration = String;
#[doc = "The IP address type.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IpAddressType")]
pub enum IpAddressType {
    IPv4,
    IPv6,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IpAddressType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IpAddressType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IpAddressType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::IPv4 => serializer.serialize_unit_variant("IpAddressType", 0u32, "IPv4"),
            Self::IPv6 => serializer.serialize_unit_variant("IpAddressType", 1u32, "IPv6"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for IpAddressType {
    fn default() -> Self {
        Self::IPv4
    }
}
#[doc = "IPTag associated with the object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpTag {
    #[doc = "The IP tag type."]
    #[serde(rename = "ipTagType")]
    pub ip_tag_type: String,
    #[doc = "The value of the IP tag."]
    pub tag: String,
}
impl IpTag {
    pub fn new(ip_tag_type: String, tag: String) -> Self {
        Self { ip_tag_type, tag }
    }
}
#[doc = "Describes a load balancing rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancingRule {
    #[doc = "The port for the external endpoint. Port numbers for each rule must be unique within the Load Balancer. Acceptable values are between 1 and 65534."]
    #[serde(rename = "frontendPort")]
    pub frontend_port: i32,
    #[doc = "The port used for internal connections on the endpoint. Acceptable values are between 1 and 65535."]
    #[serde(rename = "backendPort")]
    pub backend_port: i32,
    #[doc = "The reference to the transport protocol used by the load balancing rule."]
    pub protocol: load_balancing_rule::Protocol,
    #[doc = "The prob port used by the load balancing rule. Acceptable values are between 1 and 65535."]
    #[serde(rename = "probePort", default, skip_serializing_if = "Option::is_none")]
    pub probe_port: Option<i32>,
    #[doc = "the reference to the load balancer probe used by the load balancing rule."]
    #[serde(rename = "probeProtocol")]
    pub probe_protocol: load_balancing_rule::ProbeProtocol,
    #[doc = "The probe request path. Only supported for HTTP/HTTPS probes."]
    #[serde(rename = "probeRequestPath", default, skip_serializing_if = "Option::is_none")]
    pub probe_request_path: Option<String>,
}
impl LoadBalancingRule {
    pub fn new(
        frontend_port: i32,
        backend_port: i32,
        protocol: load_balancing_rule::Protocol,
        probe_protocol: load_balancing_rule::ProbeProtocol,
    ) -> Self {
        Self {
            frontend_port,
            backend_port,
            protocol,
            probe_port: None,
            probe_protocol,
            probe_request_path: None,
        }
    }
}
pub mod load_balancing_rule {
    use super::*;
    #[doc = "The reference to the transport protocol used by the load balancing rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(rename = "udp")]
        Udp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 0u32, "tcp"),
                Self::Udp => serializer.serialize_unit_variant("Protocol", 1u32, "udp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "the reference to the load balancer probe used by the load balancing rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProbeProtocol")]
    pub enum ProbeProtocol {
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "https")]
        Https,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProbeProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProbeProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProbeProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tcp => serializer.serialize_unit_variant("ProbeProtocol", 0u32, "tcp"),
                Self::Http => serializer.serialize_unit_variant("ProbeProtocol", 1u32, "http"),
                Self::Https => serializer.serialize_unit_variant("ProbeProtocol", 2u32, "https"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The manged cluster resource\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedCluster {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the managed cluster resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedClusterProperties>,
    #[doc = "Service Fabric managed cluster Sku definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl ManagedCluster {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            sku: None,
        }
    }
}
#[doc = "Available cluster add-on features"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagedClusterAddOnFeature {
    DnsService,
    BackupRestoreService,
    ResourceMonitorService,
}
pub type ManagedClusterCodeVersionListResult = Vec<ManagedClusterCodeVersionResult>;
#[doc = "The result of the Service Fabric runtime versions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterCodeVersionResult {
    #[doc = "The identification of the result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The result resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The detail of the Service Fabric runtime version result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedClusterVersionDetails>,
}
impl ManagedClusterCodeVersionResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed Cluster list results"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedCluster>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the managed cluster resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterProperties {
    #[doc = "The cluster dns name."]
    #[serde(rename = "dnsName")]
    pub dns_name: String,
    #[doc = "The fully qualified domain name associated with the public load balancer of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The IPv4 address associated with the public load balancer of the cluster."]
    #[serde(rename = "ipv4Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[doc = "A service generated unique identifier for the cluster resource."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The current state of the cluster.\n"]
    #[serde(rename = "clusterState", default, skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<ClusterState>,
    #[doc = "List of thumbprints of the cluster certificates."]
    #[serde(rename = "clusterCertificateThumbprints", default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_certificate_thumbprints: Vec<String>,
    #[doc = "The port used for client connections to the cluster."]
    #[serde(rename = "clientConnectionPort", default, skip_serializing_if = "Option::is_none")]
    pub client_connection_port: Option<i32>,
    #[doc = "The port used for HTTP connections to the cluster."]
    #[serde(rename = "httpGatewayConnectionPort", default, skip_serializing_if = "Option::is_none")]
    pub http_gateway_connection_port: Option<i32>,
    #[doc = "VM admin user name."]
    #[serde(rename = "adminUserName")]
    pub admin_user_name: String,
    #[doc = "VM admin user password."]
    #[serde(rename = "adminPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
    #[doc = "Load balancing rules that are applied to the public load balancer of the cluster."]
    #[serde(rename = "loadBalancingRules", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancing_rules: Vec<LoadBalancingRule>,
    #[doc = "Setting this to true enables RDP access to the VM. The default NSG rule opens RDP port to Internet which can be overridden with custom Network Security Rules. The default value for this setting is false."]
    #[serde(rename = "allowRdpAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_rdp_access: Option<bool>,
    #[doc = "Custom Network Security Rules that are applied to the Virtual Network of the cluster."]
    #[serde(rename = "networkSecurityRules", default, skip_serializing_if = "Vec::is_empty")]
    pub network_security_rules: Vec<NetworkSecurityRule>,
    #[doc = "Client certificates that are allowed to manage the cluster."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clients: Vec<ClientCertificate>,
    #[doc = "The settings to enable AAD authentication on the cluster."]
    #[serde(rename = "azureActiveDirectory", default, skip_serializing_if = "Option::is_none")]
    pub azure_active_directory: Option<AzureActiveDirectory>,
    #[doc = "The list of custom fabric settings to configure the cluster."]
    #[serde(rename = "fabricSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub fabric_settings: Vec<SettingsSectionDescription>,
    #[doc = "The provisioning state of the managed resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ManagedResourceProvisioningState>,
    #[doc = "The Service Fabric runtime version of the cluster. This property is required when **clusterUpgradeMode** is set to 'Manual'. To get list of available Service Fabric versions for new clusters use [ClusterVersion API](./ClusterVersion.md). To get the list of available version for existing clusters use **availableClusterVersions**."]
    #[serde(rename = "clusterCodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub cluster_code_version: Option<String>,
    #[doc = "The upgrade mode of the cluster when new Service Fabric runtime version is available.\n"]
    #[serde(rename = "clusterUpgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub cluster_upgrade_mode: Option<ClusterUpgradeMode>,
    #[doc = "Indicates when new cluster runtime version upgrades will be applied after they are released. By default is Wave0."]
    #[serde(rename = "clusterUpgradeCadence", default, skip_serializing_if = "Option::is_none")]
    pub cluster_upgrade_cadence: Option<ClusterUpgradeCadence>,
    #[doc = "List of add-on features to enable on the cluster."]
    #[serde(rename = "addonFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub addon_features: Vec<ManagedClusterAddOnFeature>,
    #[doc = "Setting this to true enables automatic OS upgrade for the node types that are created using any platform OS image with version 'latest'. The default value for this setting is false."]
    #[serde(rename = "enableAutoOSUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_auto_os_upgrade: Option<bool>,
    #[doc = "Indicates if the cluster has zone resiliency."]
    #[serde(rename = "zonalResiliency", default, skip_serializing_if = "Option::is_none")]
    pub zonal_resiliency: Option<bool>,
    #[doc = "The policy used to clean up unused versions. When the policy is not specified explicitly, the default unused application versions to keep will be 3."]
    #[serde(rename = "applicationTypeVersionsCleanupPolicy", default, skip_serializing_if = "Option::is_none")]
    pub application_type_versions_cleanup_policy: Option<ApplicationTypeVersionsCleanupPolicy>,
    #[doc = "Setting this to true creates IPv6 address space for the default VNet used by the cluster. This setting cannot be changed once the cluster is created. The default value for this setting is false."]
    #[serde(rename = "enableIpv6", default, skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    #[doc = "If specified, the node types for the cluster are created in this subnet instead of the default VNet. The **networkSecurityRules** specified for the cluster are also applied to this subnet. This setting cannot be changed once the cluster is created."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "The list of IP tags associated with the default public IP address of the cluster."]
    #[serde(rename = "ipTags", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_tags: Vec<IpTag>,
}
impl ManagedClusterProperties {
    pub fn new(dns_name: String, admin_user_name: String) -> Self {
        Self {
            dns_name,
            fqdn: None,
            ipv4_address: None,
            cluster_id: None,
            cluster_state: None,
            cluster_certificate_thumbprints: Vec::new(),
            client_connection_port: None,
            http_gateway_connection_port: None,
            admin_user_name,
            admin_password: None,
            load_balancing_rules: Vec::new(),
            allow_rdp_access: None,
            network_security_rules: Vec::new(),
            clients: Vec::new(),
            azure_active_directory: None,
            fabric_settings: Vec::new(),
            provisioning_state: None,
            cluster_code_version: None,
            cluster_upgrade_mode: None,
            cluster_upgrade_cadence: None,
            addon_features: Vec::new(),
            enable_auto_os_upgrade: None,
            zonal_resiliency: None,
            application_type_versions_cleanup_policy: None,
            enable_ipv6: None,
            subnet_id: None,
            ip_tags: Vec::new(),
        }
    }
}
#[doc = "Managed cluster update request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterUpdateParameters {
    #[doc = "Managed cluster update parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ManagedClusterUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The detail of the Service Fabric runtime version result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterVersionDetails {
    #[doc = "The Service Fabric runtime version of the cluster."]
    #[serde(rename = "clusterCodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub cluster_code_version: Option<String>,
    #[doc = "The date of expiry of support of the version."]
    #[serde(rename = "supportExpiryUtc", default, skip_serializing_if = "Option::is_none")]
    pub support_expiry_utc: Option<String>,
    #[doc = "Cluster operating system, the default will be Windows"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
}
impl ManagedClusterVersionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the managed identities for an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentity {
    #[doc = "The principal id of the managed identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of the managed identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of managed identity for the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagedIdentityType>,
    #[doc = "The list of user identities associated with the resource. The user identity dictionary key references will be ARM resource ids in the form:\n'/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'.\n"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentityMap>,
}
impl ManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of managed identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagedIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned, UserAssigned")]
    SystemAssignedUserAssigned,
}
#[doc = "The resource model definition for proxy-only resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedProxyResource {
    #[doc = "Azure resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ManagedProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state of the managed resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedResourceProvisioningState")]
pub enum ManagedResourceProvisioningState {
    None,
    Creating,
    Created,
    Updating,
    Succeeded,
    Failed,
    Canceled,
    Deleting,
    Deleted,
    Other,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedResourceProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedResourceProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedResourceProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedResourceProvisioningState", 0u32, "None"),
            Self::Creating => serializer.serialize_unit_variant("ManagedResourceProvisioningState", 1u32, "Creating"),
            Self::Created => serializer.serialize_unit_variant("ManagedResourceProvisioningState", 2u32, "Created"),
            Self::Updating => serializer.serialize_unit_variant("ManagedResourceProvisioningState", 3u32, "Updating"),
            Self::Succeeded => serializer.serialize_unit_variant("ManagedResourceProvisioningState", 4u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ManagedResourceProvisioningState", 5u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ManagedResourceProvisioningState", 6u32, "Canceled"),
            Self::Deleting => serializer.serialize_unit_variant("ManagedResourceProvisioningState", 7u32, "Deleting"),
            Self::Deleted => serializer.serialize_unit_variant("ManagedResourceProvisioningState", 8u32, "Deleted"),
            Self::Other => serializer.serialize_unit_variant("ManagedResourceProvisioningState", 9u32, "Other"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes a VM Sizes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedVmSize {
    #[doc = "VM Sizes properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VmSize>,
    #[doc = "VM Size id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "VM Size name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "VM Size type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ManagedVmSize {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the result of the request to list Managed VM Sizes for Service Fabric Managed Clusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedVmSizesResult {
    #[doc = "List of Managed VM Sizes for Service Fabric Managed Clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedVmSize>,
    #[doc = "URL to get the next set of Managed VM Sizes if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedVmSizesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedVmSizesResult {
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the named partition scheme of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedPartitionScheme {
    #[serde(flatten)]
    pub partition: Partition,
    #[doc = "Array for the names of the partitions."]
    pub names: Vec<String>,
}
impl NamedPartitionScheme {
    pub fn new(partition: Partition, names: Vec<String>) -> Self {
        Self { partition, names }
    }
}
#[doc = "Describes a network security rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSecurityRule {
    #[doc = "Network security rule name."]
    pub name: String,
    #[doc = "Network security rule description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Network protocol this rule applies to."]
    pub protocol: network_security_rule::Protocol,
    #[doc = "The CIDR or source IP ranges."]
    #[serde(rename = "sourceAddressPrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub source_address_prefixes: Vec<String>,
    #[doc = "The destination address prefixes. CIDR or destination IP ranges."]
    #[serde(rename = "destinationAddressPrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub destination_address_prefixes: Vec<String>,
    #[doc = "The source port ranges."]
    #[serde(rename = "sourcePortRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub source_port_ranges: Vec<String>,
    #[doc = "The destination port ranges."]
    #[serde(rename = "destinationPortRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub destination_port_ranges: Vec<String>,
    #[doc = "The network traffic is allowed or denied."]
    pub access: network_security_rule::Access,
    #[doc = "The priority of the rule. The value can be in the range 1000 to 3000. Values outside this range are reserved for Service Fabric ManagerCluster Resource Provider. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule."]
    pub priority: i32,
    #[doc = "Network security rule direction."]
    pub direction: network_security_rule::Direction,
}
impl NetworkSecurityRule {
    pub fn new(
        name: String,
        protocol: network_security_rule::Protocol,
        access: network_security_rule::Access,
        priority: i32,
        direction: network_security_rule::Direction,
    ) -> Self {
        Self {
            name,
            description: None,
            protocol,
            source_address_prefixes: Vec::new(),
            destination_address_prefixes: Vec::new(),
            source_port_ranges: Vec::new(),
            destination_port_ranges: Vec::new(),
            access,
            priority,
            direction,
        }
    }
}
pub mod network_security_rule {
    use super::*;
    #[doc = "Network protocol this rule applies to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "https")]
        Https,
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(rename = "udp")]
        Udp,
        #[serde(rename = "icmp")]
        Icmp,
        #[serde(rename = "ah")]
        Ah,
        #[serde(rename = "esp")]
        Esp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("Protocol", 0u32, "http"),
                Self::Https => serializer.serialize_unit_variant("Protocol", 1u32, "https"),
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 2u32, "tcp"),
                Self::Udp => serializer.serialize_unit_variant("Protocol", 3u32, "udp"),
                Self::Icmp => serializer.serialize_unit_variant("Protocol", 4u32, "icmp"),
                Self::Ah => serializer.serialize_unit_variant("Protocol", 5u32, "ah"),
                Self::Esp => serializer.serialize_unit_variant("Protocol", 6u32, "esp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The network traffic is allowed or denied."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Access")]
    pub enum Access {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "deny")]
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Access {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Access {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Access {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Access", 0u32, "allow"),
                Self::Deny => serializer.serialize_unit_variant("Access", 1u32, "deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Network security rule direction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        #[serde(rename = "inbound")]
        Inbound,
        #[serde(rename = "outbound")]
        Outbound,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Direction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Direction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inbound => serializer.serialize_unit_variant("Direction", 0u32, "inbound"),
                Self::Outbound => serializer.serialize_unit_variant("Direction", 1u32, "outbound"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a node type in the cluster, each node type represents sub set of nodes in the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeType {
    #[serde(flatten)]
    pub managed_proxy_resource: ManagedProxyResource,
    #[doc = "Describes a node type in the cluster, each node type represents sub set of nodes in the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NodeTypeProperties>,
    #[doc = "Describes a node type sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<NodeTypeSku>,
}
impl NodeType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for Node type action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeTypeActionParameters {
    #[doc = "List of node names from the node type."]
    pub nodes: Vec<String>,
    #[doc = "Force the action to go through."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}
impl NodeTypeActionParameters {
    pub fn new(nodes: Vec<String>) -> Self {
        Self { nodes, force: None }
    }
}
#[doc = "Defines the type of sku available for a node type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTypeAvailableSku {
    #[doc = "The type of resource the sku applies to.  <br /><br />Value: Microsoft.ServiceFabric/managedClusters/nodeTypes."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Describes a node type supported sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<NodeTypeSupportedSku>,
    #[doc = "Provides information about how node type can be scaled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<NodeTypeSkuCapacity>,
}
impl NodeTypeAvailableSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Node type list results"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTypeListResult {
    #[doc = "The list of node types."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NodeType>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NodeTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NodeTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Node type available sku list results"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTypeListSkuResult {
    #[doc = "The list of available node type SKUs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NodeTypeAvailableSku>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NodeTypeListSkuResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NodeTypeListSkuResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a node type in the cluster, each node type represents sub set of nodes in the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeTypeProperties {
    #[doc = "Indicates the Service Fabric system services for the cluster will run on this node type. This setting cannot be changed once the node type is created."]
    #[serde(rename = "isPrimary")]
    pub is_primary: bool,
    #[doc = "The number of nodes in the node type. <br /><br />**Values:** <br />-1 - Use when auto scale rules are configured or sku.capacity is defined <br /> 0 - Not supported <br /> >0 - Use for manual scale."]
    #[serde(rename = "vmInstanceCount")]
    pub vm_instance_count: i32,
    #[doc = "Disk size for each vm in the node type in GBs."]
    #[serde(rename = "dataDiskSizeGB")]
    pub data_disk_size_gb: i32,
    #[doc = "Managed data disk type. IOPS and throughput are given by the disk size, to see more information go to https://docs.microsoft.com/en-us/azure/virtual-machines/disks-types.\n"]
    #[serde(rename = "dataDiskType", default, skip_serializing_if = "Option::is_none")]
    pub data_disk_type: Option<DiskType>,
    #[doc = "The placement tags applied to nodes in the node type, which can be used to indicate where certain services (workload) should run."]
    #[serde(rename = "placementProperties", default, skip_serializing_if = "Option::is_none")]
    pub placement_properties: Option<serde_json::Value>,
    #[doc = "The capacity tags applied to the nodes in the node type, the cluster resource manager uses these tags to understand how much resource a node has."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacities: Option<serde_json::Value>,
    #[doc = "Port range details"]
    #[serde(rename = "applicationPorts", default, skip_serializing_if = "Option::is_none")]
    pub application_ports: Option<EndpointRangeDescription>,
    #[doc = "Port range details"]
    #[serde(rename = "ephemeralPorts", default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_ports: Option<EndpointRangeDescription>,
    #[doc = "The size of virtual machines in the pool. All virtual machines in a pool are the same size. For example, Standard_D3."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "The publisher of the Azure Virtual Machines Marketplace image. For example, Canonical or MicrosoftWindowsServer."]
    #[serde(rename = "vmImagePublisher", default, skip_serializing_if = "Option::is_none")]
    pub vm_image_publisher: Option<String>,
    #[doc = "The offer type of the Azure Virtual Machines Marketplace image. For example, UbuntuServer or WindowsServer."]
    #[serde(rename = "vmImageOffer", default, skip_serializing_if = "Option::is_none")]
    pub vm_image_offer: Option<String>,
    #[doc = "The SKU of the Azure Virtual Machines Marketplace image. For example, 14.04.0-LTS or 2012-R2-Datacenter."]
    #[serde(rename = "vmImageSku", default, skip_serializing_if = "Option::is_none")]
    pub vm_image_sku: Option<String>,
    #[doc = "The version of the Azure Virtual Machines Marketplace image. A value of 'latest' can be specified to select the latest version of an image. If omitted, the default is 'latest'."]
    #[serde(rename = "vmImageVersion", default, skip_serializing_if = "Option::is_none")]
    pub vm_image_version: Option<String>,
    #[doc = "The secrets to install in the virtual machines."]
    #[serde(rename = "vmSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_secrets: Vec<VaultSecretGroup>,
    #[doc = "Set of extensions that should be installed onto the virtual machines."]
    #[serde(rename = "vmExtensions", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_extensions: Vec<VmssExtension>,
    #[doc = "Identities for the virtual machine scale set under the node type."]
    #[serde(rename = "vmManagedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub vm_managed_identity: Option<VmManagedIdentity>,
    #[doc = "Indicates if the node type can only host Stateless workloads."]
    #[serde(rename = "isStateless", default, skip_serializing_if = "Option::is_none")]
    pub is_stateless: Option<bool>,
    #[doc = "Indicates if scale set associated with the node type can be composed of multiple placement groups."]
    #[serde(rename = "multiplePlacementGroups", default, skip_serializing_if = "Option::is_none")]
    pub multiple_placement_groups: Option<bool>,
    #[doc = "Indicates the node type uses its own frontend configurations instead of the default one for the cluster. This setting can only be specified for non-primary node types and can not be added or removed after the node type is created."]
    #[serde(rename = "frontendConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub frontend_configurations: Vec<FrontendConfiguration>,
    #[doc = "The Network Security Rules for this node type. This setting can only be specified for node types that are configured with frontend configurations."]
    #[serde(rename = "networkSecurityRules", default, skip_serializing_if = "Vec::is_empty")]
    pub network_security_rules: Vec<NetworkSecurityRule>,
    #[doc = "The provisioning state of the managed resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ManagedResourceProvisioningState>,
}
impl NodeTypeProperties {
    pub fn new(is_primary: bool, vm_instance_count: i32, data_disk_size_gb: i32) -> Self {
        Self {
            is_primary,
            vm_instance_count,
            data_disk_size_gb,
            data_disk_type: None,
            placement_properties: None,
            capacities: None,
            application_ports: None,
            ephemeral_ports: None,
            vm_size: None,
            vm_image_publisher: None,
            vm_image_offer: None,
            vm_image_sku: None,
            vm_image_version: None,
            vm_secrets: Vec::new(),
            vm_extensions: Vec::new(),
            vm_managed_identity: None,
            is_stateless: None,
            multiple_placement_groups: None,
            frontend_configurations: Vec::new(),
            network_security_rules: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[doc = "Describes a node type sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeTypeSku {
    #[doc = "The sku name. <br /><br />Name is internally generated and is used in auto-scale scenarios.<br /> Property does not allow to be changed to other values than generated.<br /> To avoid deployment errors please omit the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the tier of the node type. <br /><br /> Possible Values:<br /> **Standard**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The number of nodes in the node type.<br /><br />If present in request it will override properties.vmInstanceCount."]
    pub capacity: i32,
}
impl NodeTypeSku {
    pub fn new(capacity: i32) -> Self {
        Self {
            name: None,
            tier: None,
            capacity,
        }
    }
}
#[doc = "Provides information about how node type can be scaled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTypeSkuCapacity {
    #[doc = "Lowest permitted node count in a node type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[doc = "Highest permitted node count in a node type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[doc = "Default node count in a node type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "Node type capacity scale type.\n"]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<NodeTypeSkuScaleType>,
}
impl NodeTypeSkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Node type capacity scale type.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NodeTypeSkuScaleType")]
pub enum NodeTypeSkuScaleType {
    None,
    Manual,
    Automatic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NodeTypeSkuScaleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NodeTypeSkuScaleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NodeTypeSkuScaleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("NodeTypeSkuScaleType", 0u32, "None"),
            Self::Manual => serializer.serialize_unit_variant("NodeTypeSkuScaleType", 1u32, "Manual"),
            Self::Automatic => serializer.serialize_unit_variant("NodeTypeSkuScaleType", 2u32, "Automatic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for NodeTypeSkuScaleType {
    fn default() -> Self {
        Self::None
    }
}
#[doc = "Describes a node type supported sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTypeSupportedSku {
    #[doc = "The sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the tier of the node type. <br /><br /> Possible Values:<br /> **Standard**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl NodeTypeSupportedSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Node type update request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTypeUpdateParameters {
    #[doc = "Node type update parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Describes a node type sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<NodeTypeSku>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<String>,
}
impl NodeTypeUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the result of the request to list Service Fabric resource provider operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Service Fabric resource provider."]
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
#[doc = "Available operation list result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "The name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Operation supported by the Service Fabric resource provider"]
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
#[doc = "Cluster operating system, the default will be Windows"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OsType {
    Windows,
}
#[doc = "Describes how the service is partitioned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Partition {
    #[doc = "Enumerates the ways that a service can be partitioned."]
    #[serde(rename = "partitionScheme")]
    pub partition_scheme: PartitionScheme,
}
impl Partition {
    pub fn new(partition_scheme: PartitionScheme) -> Self {
        Self { partition_scheme }
    }
}
#[doc = "Represents a scaling mechanism for adding or removing instances of stateless service partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionInstanceCountScaleMechanism {
    #[serde(flatten)]
    pub scaling_mechanism: ScalingMechanism,
    #[doc = "Minimum number of instances of the partition."]
    #[serde(rename = "minInstanceCount")]
    pub min_instance_count: i32,
    #[doc = "Maximum number of instances of the partition."]
    #[serde(rename = "maxInstanceCount")]
    pub max_instance_count: i32,
    #[doc = "The number of instances to add or remove during a scaling operation."]
    #[serde(rename = "scaleIncrement")]
    pub scale_increment: i32,
}
impl PartitionInstanceCountScaleMechanism {
    pub fn new(scaling_mechanism: ScalingMechanism, min_instance_count: i32, max_instance_count: i32, scale_increment: i32) -> Self {
        Self {
            scaling_mechanism,
            min_instance_count,
            max_instance_count,
            scale_increment,
        }
    }
}
#[doc = "Enumerates the ways that a service can be partitioned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PartitionScheme")]
pub enum PartitionScheme {
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
            Self::Singleton => serializer.serialize_unit_variant("PartitionScheme", 0u32, "Singleton"),
            Self::UniformInt64Range => serializer.serialize_unit_variant("PartitionScheme", 1u32, "UniformInt64Range"),
            Self::Named => serializer.serialize_unit_variant("PartitionScheme", 2u32, "Named"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for proxy-only resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Azure resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location depends on the parent resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Azure resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "Azure resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure resource location."]
    pub location: String,
    #[doc = "Azure resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Azure resource etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            etag: None,
            system_data: None,
        }
    }
}
#[doc = "The mode used to monitor health during a rolling upgrade. The values are Monitored, and UnmonitoredAuto."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RollingUpgradeMode")]
pub enum RollingUpgradeMode {
    Monitored,
    UnmonitoredAuto,
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
            Self::Monitored => serializer.serialize_unit_variant("RollingUpgradeMode", 0u32, "Monitored"),
            Self::UnmonitoredAuto => serializer.serialize_unit_variant("RollingUpgradeMode", 1u32, "UnmonitoredAuto"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The policy used for monitoring the application upgrade"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RollingUpgradeMonitoringPolicy {
    #[doc = "The compensating action to perform when a Monitored upgrade encounters monitoring policy or health policy violations. Invalid indicates the failure action is invalid. Rollback specifies that the upgrade will start rolling back automatically. Manual indicates that the upgrade will switch to UnmonitoredManual upgrade mode."]
    #[serde(rename = "failureAction")]
    pub failure_action: rolling_upgrade_monitoring_policy::FailureAction,
    #[doc = "The amount of time to wait after completing an upgrade domain before applying health policies. It is interpreted as a string representing an ISO 8601 duration with following format \"hh:mm:ss.fff\"."]
    #[serde(rename = "healthCheckWaitDuration")]
    pub health_check_wait_duration: HealthCheckWaitDuration,
    #[doc = "The amount of time that the application or cluster must remain healthy before the upgrade proceeds to the next upgrade domain. It is interpreted as a string representing an ISO 8601 duration with following format \"hh:mm:ss.fff\"."]
    #[serde(rename = "healthCheckStableDuration")]
    pub health_check_stable_duration: HealthCheckStableDuration,
    #[doc = "The amount of time to retry health evaluation when the application or cluster is unhealthy before FailureAction is executed. It is interpreted as a string representing an ISO 8601 duration with following format \"hh:mm:ss.fff\"."]
    #[serde(rename = "healthCheckRetryTimeout")]
    pub health_check_retry_timeout: HealthCheckRetryTimeout,
    #[doc = "The amount of time the overall upgrade has to complete before FailureAction is executed. Cannot be larger than 12 hours. It is interpreted as a string representing an ISO 8601 duration with following format \"hh:mm:ss.fff\"."]
    #[serde(rename = "upgradeTimeout")]
    pub upgrade_timeout: UpgradeTimeout,
    #[doc = "The amount of time each upgrade domain has to complete before FailureAction is executed. Cannot be larger than 12 hours. It is interpreted as a string representing an ISO 8601 duration with following format \"hh:mm:ss.fff\"."]
    #[serde(rename = "upgradeDomainTimeout")]
    pub upgrade_domain_timeout: UpgradeDomainTimeout,
}
impl RollingUpgradeMonitoringPolicy {
    pub fn new(
        failure_action: rolling_upgrade_monitoring_policy::FailureAction,
        health_check_wait_duration: HealthCheckWaitDuration,
        health_check_stable_duration: HealthCheckStableDuration,
        health_check_retry_timeout: HealthCheckRetryTimeout,
        upgrade_timeout: UpgradeTimeout,
        upgrade_domain_timeout: UpgradeDomainTimeout,
    ) -> Self {
        Self {
            failure_action,
            health_check_wait_duration,
            health_check_stable_duration,
            health_check_retry_timeout,
            upgrade_timeout,
            upgrade_domain_timeout,
        }
    }
}
pub mod rolling_upgrade_monitoring_policy {
    use super::*;
    #[doc = "The compensating action to perform when a Monitored upgrade encounters monitoring policy or health policy violations. Invalid indicates the failure action is invalid. Rollback specifies that the upgrade will start rolling back automatically. Manual indicates that the upgrade will switch to UnmonitoredManual upgrade mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailureAction")]
    pub enum FailureAction {
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
                Self::Rollback => serializer.serialize_unit_variant("FailureAction", 0u32, "Rollback"),
                Self::Manual => serializer.serialize_unit_variant("FailureAction", 1u32, "Manual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the mechanism for performing a scaling operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalingMechanism {
    #[doc = "Enumerates the ways that a service can be partitioned."]
    pub kind: ServiceScalingMechanismKind,
}
impl ScalingMechanism {
    pub fn new(kind: ServiceScalingMechanismKind) -> Self {
        Self { kind }
    }
}
#[doc = "Specifies a metric to load balance a service during runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalingPolicy {
    #[doc = "Describes the mechanism for performing a scaling operation."]
    #[serde(rename = "scalingMechanism")]
    pub scaling_mechanism: ScalingMechanism,
    #[doc = "Describes the trigger for performing a scaling operation."]
    #[serde(rename = "scalingTrigger")]
    pub scaling_trigger: ScalingTrigger,
}
impl ScalingPolicy {
    pub fn new(scaling_mechanism: ScalingMechanism, scaling_trigger: ScalingTrigger) -> Self {
        Self {
            scaling_mechanism,
            scaling_trigger,
        }
    }
}
pub type ScalingPolicyList = Vec<ScalingPolicy>;
#[doc = "Describes the trigger for performing a scaling operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalingTrigger {
    #[doc = "Enumerates the ways that a service can be partitioned."]
    pub kind: ServiceScalingTriggerKind,
}
impl ScalingTrigger {
    pub fn new(kind: ServiceScalingTriggerKind) -> Self {
        Self { kind }
    }
}
#[doc = "Creates a particular correlation between services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCorrelation {
    #[doc = "The service correlation scheme."]
    pub scheme: ServiceCorrelationScheme,
    #[doc = "The full ARM Resource ID describing the service resource"]
    #[serde(rename = "serviceName")]
    pub service_name: ServiceName,
}
impl ServiceCorrelation {
    pub fn new(scheme: ServiceCorrelationScheme, service_name: ServiceName) -> Self {
        Self { scheme, service_name }
    }
}
#[doc = "The service correlation scheme."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceCorrelationScheme")]
pub enum ServiceCorrelationScheme {
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
            Self::AlignedAffinity => serializer.serialize_unit_variant("ServiceCorrelationScheme", 0u32, "AlignedAffinity"),
            Self::NonAlignedAffinity => serializer.serialize_unit_variant("ServiceCorrelationScheme", 1u32, "NonAlignedAffinity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The kind of service (Stateless or Stateful)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceKind")]
pub enum ServiceKind {
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
            Self::Stateless => serializer.serialize_unit_variant("ServiceKind", 0u32, "Stateless"),
            Self::Stateful => serializer.serialize_unit_variant("ServiceKind", 1u32, "Stateful"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies a metric to load balance a service during runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceLoadMetric {
    #[doc = "The name of the metric. If the service chooses to report load during runtime, the load metric name should match the name that is specified in Name exactly. Note that metric names are case sensitive."]
    pub name: String,
    #[doc = "Determines the metric weight relative to the other metrics that are configured for this service. During runtime, if two metrics end up in conflict, the Cluster Resource Manager prefers the metric with the higher weight."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<ServiceLoadMetricWeight>,
    #[doc = "Used only for Stateful services. The default amount of load, as a number, that this service creates for this metric when it is a Primary replica."]
    #[serde(rename = "primaryDefaultLoad", default, skip_serializing_if = "Option::is_none")]
    pub primary_default_load: Option<i32>,
    #[doc = "Used only for Stateful services. The default amount of load, as a number, that this service creates for this metric when it is a Secondary replica."]
    #[serde(rename = "secondaryDefaultLoad", default, skip_serializing_if = "Option::is_none")]
    pub secondary_default_load: Option<i32>,
    #[doc = "Used only for Stateless services. The default amount of load, as a number, that this service creates for this metric."]
    #[serde(rename = "defaultLoad", default, skip_serializing_if = "Option::is_none")]
    pub default_load: Option<i32>,
}
impl ServiceLoadMetric {
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
pub type ServiceLoadMetricsList = Vec<ServiceLoadMetric>;
pub type ServiceName = String;
#[doc = "Describes the policy to be used for placement of a Service Fabric service where a particular fault or upgrade domain should not be used for placement of the instances or replicas of that service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementInvalidDomainPolicy {
    #[serde(flatten)]
    pub service_placement_policy: ServicePlacementPolicy,
    #[doc = "The name of the domain that should not be used for placement."]
    #[serde(rename = "domainName")]
    pub domain_name: String,
}
impl ServicePlacementInvalidDomainPolicy {
    pub fn new(service_placement_policy: ServicePlacementPolicy, domain_name: String) -> Self {
        Self {
            service_placement_policy,
            domain_name,
        }
    }
}
#[doc = "The name of the domain that should used for placement as per this policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementNonPartiallyPlaceServicePolicy {
    #[serde(flatten)]
    pub service_placement_policy: ServicePlacementPolicy,
}
impl ServicePlacementNonPartiallyPlaceServicePolicy {
    pub fn new(service_placement_policy: ServicePlacementPolicy) -> Self {
        Self { service_placement_policy }
    }
}
pub type ServicePlacementPoliciesList = Vec<ServicePlacementPolicy>;
#[doc = "Describes the policy to be used for placement of a Service Fabric service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementPolicy {
    #[doc = "The type of placement policy for a service fabric service. Following are the possible values."]
    #[serde(rename = "type")]
    pub type_: ServicePlacementPolicyType,
}
impl ServicePlacementPolicy {
    pub fn new(type_: ServicePlacementPolicyType) -> Self {
        Self { type_ }
    }
}
#[doc = "The type of placement policy for a service fabric service. Following are the possible values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServicePlacementPolicyType")]
pub enum ServicePlacementPolicyType {
    InvalidDomain,
    RequiredDomain,
    PreferredPrimaryDomain,
    RequiredDomainDistribution,
    NonPartiallyPlaceService,
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
            Self::InvalidDomain => serializer.serialize_unit_variant("ServicePlacementPolicyType", 0u32, "InvalidDomain"),
            Self::RequiredDomain => serializer.serialize_unit_variant("ServicePlacementPolicyType", 1u32, "RequiredDomain"),
            Self::PreferredPrimaryDomain => serializer.serialize_unit_variant("ServicePlacementPolicyType", 2u32, "PreferredPrimaryDomain"),
            Self::RequiredDomainDistribution => {
                serializer.serialize_unit_variant("ServicePlacementPolicyType", 3u32, "RequiredDomainDistribution")
            }
            Self::NonPartiallyPlaceService => {
                serializer.serialize_unit_variant("ServicePlacementPolicyType", 4u32, "NonPartiallyPlaceService")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the policy to be used for placement of a Service Fabric service where the service's \nPrimary replicas should optimally be placed in a particular domain.\n\nThis placement policy is usually used with fault domains in scenarios where the Service Fabric\ncluster is geographically distributed in order to indicate that a service's primary replica should\nbe located in a particular fault domain, which in geo-distributed scenarios usually aligns with regional\nor datacenter boundaries. Note that since this is an optimization it is possible that the Primary replica\nmay not end up located in this domain due to failures, capacity limits, or other constraints.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementPreferPrimaryDomainPolicy {
    #[serde(flatten)]
    pub service_placement_policy: ServicePlacementPolicy,
    #[doc = "The name of the domain that should used for placement as per this policy."]
    #[serde(rename = "domainName")]
    pub domain_name: String,
}
impl ServicePlacementPreferPrimaryDomainPolicy {
    pub fn new(service_placement_policy: ServicePlacementPolicy, domain_name: String) -> Self {
        Self {
            service_placement_policy,
            domain_name,
        }
    }
}
#[doc = "Describes the policy to be used for placement of a Service Fabric service where two replicas\nfrom the same partition should never be placed in the same fault or upgrade domain.\n\nWhile this is not common it can expose the service to an increased risk of concurrent failures\ndue to unplanned outages or other cases of subsequent/concurrent failures. As an example, consider\na case where replicas are deployed across different data center, with one replica per location.\nIn the event that one of the datacenters goes offline, normally the replica that was placed in that\ndatacenter will be packed into one of the remaining datacenters. If this is not desirable then this\npolicy should be set.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementRequireDomainDistributionPolicy {
    #[serde(flatten)]
    pub service_placement_policy: ServicePlacementPolicy,
    #[doc = "The name of the domain that should used for placement as per this policy."]
    #[serde(rename = "domainName")]
    pub domain_name: String,
}
impl ServicePlacementRequireDomainDistributionPolicy {
    pub fn new(service_placement_policy: ServicePlacementPolicy, domain_name: String) -> Self {
        Self {
            service_placement_policy,
            domain_name,
        }
    }
}
#[doc = "Describes the policy to be used for placement of a Service Fabric service where the instances or replicas of that service must be placed in a particular domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlacementRequiredDomainPolicy {
    #[serde(flatten)]
    pub service_placement_policy: ServicePlacementPolicy,
    #[doc = "The name of the domain that should used for placement as per this policy."]
    #[serde(rename = "domainName")]
    pub domain_name: String,
}
impl ServicePlacementRequiredDomainPolicy {
    pub fn new(service_placement_policy: ServicePlacementPolicy, domain_name: String) -> Self {
        Self {
            service_placement_policy,
            domain_name,
        }
    }
}
#[doc = "The service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The service resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceResourceProperties>,
}
impl ServiceResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of service resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceResource>,
    #[doc = "URL to get the next set of service list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServiceResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The service resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceProperties {
    #[serde(flatten)]
    pub service_resource_properties_base: ServiceResourcePropertiesBase,
    #[doc = "The current deployment or provisioning state, which only appears in the response"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The kind of service (Stateless or Stateful)."]
    #[serde(rename = "serviceKind")]
    pub service_kind: ServiceKind,
    #[doc = "The name of the service type"]
    #[serde(rename = "serviceTypeName")]
    pub service_type_name: String,
    #[doc = "Describes how the service is partitioned."]
    #[serde(rename = "partitionDescription")]
    pub partition_description: Partition,
    #[doc = "The activation Mode of the service package"]
    #[serde(rename = "servicePackageActivationMode", default, skip_serializing_if = "Option::is_none")]
    pub service_package_activation_mode: Option<service_resource_properties::ServicePackageActivationMode>,
}
impl ServiceResourceProperties {
    pub fn new(service_kind: ServiceKind, service_type_name: String, partition_description: Partition) -> Self {
        Self {
            service_resource_properties_base: ServiceResourcePropertiesBase::default(),
            provisioning_state: None,
            service_kind,
            service_type_name,
            partition_description,
            service_package_activation_mode: None,
        }
    }
}
pub mod service_resource_properties {
    use super::*;
    #[doc = "The activation Mode of the service package"]
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
}
#[doc = "The common service resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceResourcePropertiesBase {
    #[doc = "The placement constraints as a string. Placement constraints are boolean expressions on node properties and allow for restricting a service to particular nodes based on the service requirements. For example, to place a service on nodes where NodeType is blue specify the following: \"NodeColor == blue)\"."]
    #[serde(rename = "placementConstraints", default, skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<String>,
    #[doc = "A list that describes the correlation of the service with other services."]
    #[serde(rename = "correlationScheme", default, skip_serializing_if = "Option::is_none")]
    pub correlation_scheme: Option<CorrelationSchemeList>,
    #[doc = "The service load metrics is given as an array of ServiceLoadMetric objects."]
    #[serde(rename = "serviceLoadMetrics", default, skip_serializing_if = "Option::is_none")]
    pub service_load_metrics: Option<ServiceLoadMetricsList>,
    #[doc = "A list that describes the correlation of the service with other services."]
    #[serde(rename = "servicePlacementPolicies", default, skip_serializing_if = "Option::is_none")]
    pub service_placement_policies: Option<ServicePlacementPoliciesList>,
    #[doc = "Specifies the move cost for the service."]
    #[serde(rename = "defaultMoveCost", default, skip_serializing_if = "Option::is_none")]
    pub default_move_cost: Option<MoveCost>,
    #[doc = "Scaling policies for this service."]
    #[serde(rename = "scalingPolicies", default, skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<ScalingPolicyList>,
}
impl ServiceResourcePropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enumerates the ways that a service can be partitioned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceScalingMechanismKind")]
pub enum ServiceScalingMechanismKind {
    ScalePartitionInstanceCount,
    AddRemoveIncrementalNamedPartition,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceScalingMechanismKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceScalingMechanismKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceScalingMechanismKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ScalePartitionInstanceCount => {
                serializer.serialize_unit_variant("ServiceScalingMechanismKind", 0u32, "ScalePartitionInstanceCount")
            }
            Self::AddRemoveIncrementalNamedPartition => {
                serializer.serialize_unit_variant("ServiceScalingMechanismKind", 1u32, "AddRemoveIncrementalNamedPartition")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enumerates the ways that a service can be partitioned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceScalingTriggerKind")]
pub enum ServiceScalingTriggerKind {
    AveragePartitionLoad,
    AverageServiceLoad,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceScalingTriggerKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceScalingTriggerKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceScalingTriggerKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AveragePartitionLoad => serializer.serialize_unit_variant("ServiceScalingTriggerKind", 0u32, "AveragePartitionLoad"),
            Self::AverageServiceLoad => serializer.serialize_unit_variant("ServiceScalingTriggerKind", 1u32, "AverageServiceLoad"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents the health policy used to evaluate the health of services belonging to a service type.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceTypeHealthPolicy {
    #[doc = "The maximum allowed percentage of unhealthy services.\n\nThe percentage represents the maximum tolerated percentage of services that can be unhealthy before the application is considered in error.\nIf the percentage is respected but there is at least one unhealthy service, the health is evaluated as Warning.\nThis is calculated by dividing the number of unhealthy services of the specific service type over the total number of services of the specific service type.\nThe computation rounds up to tolerate one failure on small numbers of services.\n"]
    #[serde(rename = "maxPercentUnhealthyServices")]
    pub max_percent_unhealthy_services: i32,
    #[doc = "The maximum allowed percentage of unhealthy partitions per service.\n\nThe percentage represents the maximum tolerated percentage of partitions that can be unhealthy before the service is considered in error.\nIf the percentage is respected but there is at least one unhealthy partition, the health is evaluated as Warning.\nThe percentage is calculated by dividing the number of unhealthy partitions over the total number of partitions in the service.\nThe computation rounds up to tolerate one failure on small numbers of partitions.\n"]
    #[serde(rename = "maxPercentUnhealthyPartitionsPerService")]
    pub max_percent_unhealthy_partitions_per_service: i32,
    #[doc = "The maximum allowed percentage of unhealthy replicas per partition.\n\nThe percentage represents the maximum tolerated percentage of replicas that can be unhealthy before the partition is considered in error.\nIf the percentage is respected but there is at least one unhealthy replica, the health is evaluated as Warning.\nThe percentage is calculated by dividing the number of unhealthy replicas over the total number of replicas in the partition.\nThe computation rounds up to tolerate one failure on small numbers of replicas.\n"]
    #[serde(rename = "maxPercentUnhealthyReplicasPerPartition")]
    pub max_percent_unhealthy_replicas_per_partition: i32,
}
impl ServiceTypeHealthPolicy {
    pub fn new(
        max_percent_unhealthy_services: i32,
        max_percent_unhealthy_partitions_per_service: i32,
        max_percent_unhealthy_replicas_per_partition: i32,
    ) -> Self {
        Self {
            max_percent_unhealthy_services,
            max_percent_unhealthy_partitions_per_service,
            max_percent_unhealthy_replicas_per_partition,
        }
    }
}
#[doc = "Defines a ServiceTypeHealthPolicy per service type name.\n\nThe entries in the map replace the default service type health policy for each specified service type.\nFor example, in an application that contains both a stateless gateway service type and a stateful engine service type, the health policies for the stateless and stateful services can be configured differently.\nWith policy per service type, there's more granular control of the health of the service.\n\nIf no policy is specified for a service type name, the DefaultServiceTypeHealthPolicy is used for evaluation.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTypeHealthPolicyMap {}
impl ServiceTypeHealthPolicyMap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service update request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceUpdateParameters {
    #[doc = "Service update parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ServiceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a parameter in fabric settings of the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingsParameterDescription {
    #[doc = "The parameter name of fabric setting."]
    pub name: String,
    #[doc = "The parameter value of fabric setting."]
    pub value: String,
}
impl SettingsParameterDescription {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "Describes a section in the fabric settings of the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingsSectionDescription {
    #[doc = "The section name of the fabric settings."]
    pub name: String,
    #[doc = "The collection of parameters in the section."]
    pub parameters: Vec<SettingsParameterDescription>,
}
impl SettingsSectionDescription {
    pub fn new(name: String, parameters: Vec<SettingsParameterDescription>) -> Self {
        Self { name, parameters }
    }
}
#[doc = "Describes the partition scheme of a singleton-partitioned, or non-partitioned service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingletonPartitionScheme {
    #[serde(flatten)]
    pub partition: Partition,
}
impl SingletonPartitionScheme {
    pub fn new(partition: Partition) -> Self {
        Self { partition }
    }
}
#[doc = "Service Fabric managed cluster Sku definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Sku Name."]
    pub name: SkuName,
}
impl Sku {
    pub fn new(name: SkuName) -> Self {
        Self { name }
    }
}
#[doc = "Sku Name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuName")]
pub enum SkuName {
    Basic,
    Standard,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Basic => serializer.serialize_unit_variant("SkuName", 0u32, "Basic"),
            Self::Standard => serializer.serialize_unit_variant("SkuName", 1u32, "Standard"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of a stateful service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatefulServiceProperties {
    #[serde(flatten)]
    pub service_resource_properties: ServiceResourceProperties,
    #[doc = "A flag indicating whether this is a persistent service which stores states on the local disk. If it is then the value of this property is true, if not it is false."]
    #[serde(rename = "hasPersistedState", default, skip_serializing_if = "Option::is_none")]
    pub has_persisted_state: Option<bool>,
    #[doc = "The target replica set size as a number."]
    #[serde(rename = "targetReplicaSetSize", default, skip_serializing_if = "Option::is_none")]
    pub target_replica_set_size: Option<i32>,
    #[doc = "The minimum replica set size as a number."]
    #[serde(rename = "minReplicaSetSize", default, skip_serializing_if = "Option::is_none")]
    pub min_replica_set_size: Option<i32>,
    #[doc = "The duration between when a replica goes down and when a new replica is created, represented in ISO 8601 format \"hh:mm:ss\"."]
    #[serde(rename = "replicaRestartWaitDuration", default, skip_serializing_if = "Option::is_none")]
    pub replica_restart_wait_duration: Option<String>,
    #[doc = "The maximum duration for which a partition is allowed to be in a state of quorum loss, represented in ISO 8601 format \"hh:mm:ss\"."]
    #[serde(rename = "quorumLossWaitDuration", default, skip_serializing_if = "Option::is_none")]
    pub quorum_loss_wait_duration: Option<String>,
    #[doc = "The definition on how long StandBy replicas should be maintained before being removed, represented in ISO 8601 format \"hh:mm:ss\"."]
    #[serde(rename = "standByReplicaKeepDuration", default, skip_serializing_if = "Option::is_none")]
    pub stand_by_replica_keep_duration: Option<String>,
    #[doc = "The duration for which replicas can stay InBuild before reporting that build is stuck, represented in ISO 8601 format \"hh:mm:ss\"."]
    #[serde(rename = "servicePlacementTimeLimit", default, skip_serializing_if = "Option::is_none")]
    pub service_placement_time_limit: Option<String>,
}
impl StatefulServiceProperties {
    pub fn new(service_resource_properties: ServiceResourceProperties) -> Self {
        Self {
            service_resource_properties,
            has_persisted_state: None,
            target_replica_set_size: None,
            min_replica_set_size: None,
            replica_restart_wait_duration: None,
            quorum_loss_wait_duration: None,
            stand_by_replica_keep_duration: None,
            service_placement_time_limit: None,
        }
    }
}
#[doc = "The properties of a stateless service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatelessServiceProperties {
    #[serde(flatten)]
    pub service_resource_properties: ServiceResourceProperties,
    #[doc = "The instance count."]
    #[serde(rename = "instanceCount")]
    pub instance_count: i32,
    #[doc = "MinInstanceCount is the minimum number of instances that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node. The actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ). Note, if InstanceCount is set to -1, during MinInstanceCount computation -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service."]
    #[serde(rename = "minInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_count: Option<i32>,
    #[doc = "MinInstancePercentage is the minimum percentage of InstanceCount that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node. The actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ). Note, if InstanceCount is set to -1, during MinInstancePercentage computation, -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service."]
    #[serde(rename = "minInstancePercentage", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_percentage: Option<i32>,
}
impl StatelessServiceProperties {
    pub fn new(service_resource_properties: ServiceResourceProperties, instance_count: i32) -> Self {
        Self {
            service_resource_properties,
            instance_count,
            min_instance_count: None,
            min_instance_percentage: None,
        }
    }
}
#[doc = "Azure resource identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Azure resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
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
    pub created_by_type: Option<String>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<String>,
    #[doc = "The timestamp of resource last modification (UTC)."]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a partitioning scheme where an integer range is allocated evenly across a number of partitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UniformInt64RangePartitionScheme {
    #[serde(flatten)]
    pub partition: Partition,
    #[doc = "The number of partitions."]
    pub count: i32,
    #[doc = "The lower bound of the partition key range that\nshould be split between the partition ‘Count’\n"]
    #[serde(rename = "lowKey")]
    pub low_key: i64,
    #[doc = "The upper bound of the partition key range that\nshould be split between the partition ‘Count’\n"]
    #[serde(rename = "highKey")]
    pub high_key: i64,
}
impl UniformInt64RangePartitionScheme {
    pub fn new(partition: Partition, count: i32, low_key: i64, high_key: i64) -> Self {
        Self {
            partition,
            count,
            low_key,
            high_key,
        }
    }
}
pub type UpgradeDomainTimeout = String;
pub type UpgradeTimeout = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal id of user assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client id of user assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of user identities associated with the resource. The user identity dictionary key references will be ARM resource ids in the form:\n'/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'.\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentityMap {}
impl UserAssignedIdentityMap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies set of extensions that should be installed onto the virtual machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmssExtension {
    #[doc = "The name of the extension."]
    pub name: String,
    #[doc = "Describes the properties of a Virtual Machine Scale Set Extension."]
    pub properties: VmssExtensionProperties,
}
impl VmssExtension {
    pub fn new(name: String, properties: VmssExtensionProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "Describes the properties of a Virtual Machine Scale Set Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmssExtensionProperties {
    #[doc = "The name of the extension handler publisher."]
    pub publisher: String,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion")]
    pub type_handler_version: String,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
    #[doc = "If a value is provided and is different from the previous value, the extension handler will be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "Collection of extension names after which this extension needs to be provisioned."]
    #[serde(rename = "provisionAfterExtensions", default, skip_serializing_if = "Vec::is_empty")]
    pub provision_after_extensions: Vec<String>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VmssExtensionProperties {
    pub fn new(publisher: String, type_: String, type_handler_version: String) -> Self {
        Self {
            publisher,
            type_,
            type_handler_version,
            auto_upgrade_minor_version: None,
            settings: None,
            protected_settings: None,
            force_update_tag: None,
            provision_after_extensions: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[doc = "VM Sizes properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSize {
    #[doc = "VM Size name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}
impl VmSize {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a single certificate reference in a Key Vault, and where the certificate should reside on the VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCertificate {
    #[doc = "This is the URL of a certificate that has been uploaded to Key Vault as a secret. For adding a secret to the Key Vault, see [Add a key or secret to the key vault](https://docs.microsoft.com/azure/key-vault/key-vault-get-started/#add). In this case, your certificate needs to be It is the Base64 encoding of the following JSON Object which is encoded in UTF-8: <br><br> {<br>  \"data\":\"<Base64-encoded-certificate>\",<br>  \"dataType\":\"pfx\",<br>  \"password\":\"<pfx-file-password>\"<br>}"]
    #[serde(rename = "certificateUrl")]
    pub certificate_url: String,
    #[doc = "For Windows VMs, specifies the certificate store on the Virtual Machine to which the certificate should be added. The specified certificate store is implicitly in the LocalMachine account. <br><br>For Linux VMs, the certificate file is placed under the /var/lib/waagent directory, with the file name <UppercaseThumbprint>.crt for the X509 certificate file and <UppercaseThumbprint>.prv for private key. Both of these files are .pem formatted."]
    #[serde(rename = "certificateStore")]
    pub certificate_store: String,
}
impl VaultCertificate {
    pub fn new(certificate_url: String, certificate_store: String) -> Self {
        Self {
            certificate_url,
            certificate_store,
        }
    }
}
#[doc = "Specifies set of certificates that should be installed onto the virtual machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultSecretGroup {
    #[doc = "Azure resource identifier."]
    #[serde(rename = "sourceVault")]
    pub source_vault: SubResource,
    #[doc = "The list of key vault references in SourceVault which contain certificates."]
    #[serde(rename = "vaultCertificates")]
    pub vault_certificates: Vec<VaultCertificate>,
}
impl VaultSecretGroup {
    pub fn new(source_vault: SubResource, vault_certificates: Vec<VaultCertificate>) -> Self {
        Self {
            source_vault,
            vault_certificates,
        }
    }
}
#[doc = "Identities for the virtual machine scale set under the node type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmManagedIdentity {
    #[doc = "The list of user identities associated with the virtual machine scale set under the node type. Each entry will be an ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Vec::is_empty")]
    pub user_assigned_identities: Vec<String>,
}
impl VmManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
