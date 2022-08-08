#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The Azure active directory domain service resource details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AaddsResourceDetails {
    #[doc = "The Azure active directory domain service name."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "This indicates whether initial sync complete or not."]
    #[serde(rename = "initialSyncComplete", default, skip_serializing_if = "Option::is_none")]
    pub initial_sync_complete: Option<bool>,
    #[doc = "This indicates whether enable ldaps or not."]
    #[serde(rename = "ldapsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub ldaps_enabled: Option<bool>,
    #[doc = "The base 64 format string of public ldap certificate."]
    #[serde(rename = "ldapsPublicCertificateInBase64", default, skip_serializing_if = "Option::is_none")]
    pub ldaps_public_certificate_in_base64: Option<String>,
    #[doc = "The resource id of azure active directory domain service."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The subnet resource id."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "The tenant id of azure active directory domain service ."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl AaddsResourceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The HDInsight cluster application"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Application {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The ETag for the application"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The tags for the application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The HDInsight cluster application GET response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets the application SSH endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGetEndpoint {
    #[doc = "The location of the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The destination port to connect to."]
    #[serde(rename = "destinationPort", default, skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i32>,
    #[doc = "The public port to connect to."]
    #[serde(rename = "publicPort", default, skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    #[doc = "The private ip address of the endpoint."]
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}
impl ApplicationGetEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets the application HTTP endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGetHttpsEndpoint {
    #[doc = "The list of access modes for the application."]
    #[serde(rename = "accessModes", default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,
    #[doc = "The location of the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The destination port to connect to."]
    #[serde(rename = "destinationPort", default, skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i32>,
    #[doc = "The public port to connect to."]
    #[serde(rename = "publicPort", default, skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    #[doc = "The private ip address of the endpoint."]
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "The subdomain suffix of the application."]
    #[serde(rename = "subDomainSuffix", default, skip_serializing_if = "Option::is_none")]
    pub sub_domain_suffix: Option<String>,
    #[doc = "The value indicates whether to disable GatewayAuth."]
    #[serde(rename = "disableGatewayAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_gateway_auth: Option<bool>,
}
impl ApplicationGetHttpsEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list cluster Applications. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationListResult {
    #[doc = "The list of HDInsight applications installed on HDInsight cluster."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[doc = "The URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApplicationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The HDInsight cluster application GET response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationProperties {
    #[doc = "Describes the compute profile."]
    #[serde(rename = "computeProfile", default, skip_serializing_if = "Option::is_none")]
    pub compute_profile: Option<ComputeProfile>,
    #[doc = "The list of install script actions."]
    #[serde(rename = "installScriptActions", default, skip_serializing_if = "Vec::is_empty")]
    pub install_script_actions: Vec<RuntimeScriptAction>,
    #[doc = "The list of uninstall script actions."]
    #[serde(rename = "uninstallScriptActions", default, skip_serializing_if = "Vec::is_empty")]
    pub uninstall_script_actions: Vec<RuntimeScriptAction>,
    #[doc = "The list of application HTTPS endpoints."]
    #[serde(rename = "httpsEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub https_endpoints: Vec<ApplicationGetHttpsEndpoint>,
    #[doc = "The list of application SSH endpoints."]
    #[serde(rename = "sshEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub ssh_endpoints: Vec<ApplicationGetEndpoint>,
    #[doc = "The provisioning state of the application."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The application type."]
    #[serde(rename = "applicationType", default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[doc = "The application state."]
    #[serde(rename = "applicationState", default, skip_serializing_if = "Option::is_none")]
    pub application_state: Option<String>,
    #[doc = "The list of errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Errors>,
    #[doc = "The application create date time."]
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[doc = "The marketplace identifier."]
    #[serde(rename = "marketplaceIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_identifier: Option<String>,
    #[doc = "The private link configurations."]
    #[serde(rename = "privateLinkConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_configurations: Vec<PrivateLinkConfiguration>,
}
impl ApplicationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The azure async operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncOperationResult {
    #[doc = "The async operation state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<async_operation_result::Status>,
    #[doc = "The error message associated with the cluster creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Errors>,
}
impl AsyncOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod async_operation_result {
    use super::*;
    #[doc = "The async operation state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        InProgress,
        Succeeded,
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
                Self::InProgress => serializer.serialize_unit_variant("Status", 0u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The autoscale request parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Autoscale {
    #[doc = "The load-based autoscale request parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<AutoscaleCapacity>,
    #[doc = "Schedule-based autoscale request parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<AutoscaleRecurrence>,
}
impl Autoscale {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The load-based autoscale request parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoscaleCapacity {
    #[doc = "The minimum instance count of the cluster"]
    #[serde(rename = "minInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_count: Option<i32>,
    #[doc = "The maximum instance count of the cluster"]
    #[serde(rename = "maxInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub max_instance_count: Option<i32>,
}
impl AutoscaleCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The autoscale configuration update parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoscaleConfigurationUpdateParameter {
    #[doc = "The autoscale request parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<Autoscale>,
}
impl AutoscaleConfigurationUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schedule-based autoscale request parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoscaleRecurrence {
    #[doc = "The time zone for the autoscale schedule times"]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "Array of schedule-based autoscale rules"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedule: Vec<AutoscaleSchedule>,
}
impl AutoscaleRecurrence {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for a schedule-based autoscale rule, consisting of an array of days + a time and capacity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoscaleSchedule {
    #[doc = "Days of the week for a schedule-based autoscale rule"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub days: Vec<String>,
    #[doc = "Time and capacity request parameters"]
    #[serde(rename = "timeAndCapacity", default, skip_serializing_if = "Option::is_none")]
    pub time_and_capacity: Option<AutoscaleTimeAndCapacity>,
}
impl AutoscaleSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Time and capacity request parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoscaleTimeAndCapacity {
    #[doc = "24-hour time in the form xx:xx"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[doc = "The minimum instance count of the cluster"]
    #[serde(rename = "minInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_count: Option<i32>,
    #[doc = "The maximum instance count of the cluster"]
    #[serde(rename = "maxInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub max_instance_count: Option<i32>,
}
impl AutoscaleTimeAndCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The azure monitor parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMonitorRequest {
    #[doc = "The Log Analytics workspace ID."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The Log Analytics workspace key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The selected configurations for azure monitor."]
    #[serde(rename = "selectedConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub selected_configurations: Option<AzureMonitorSelectedConfigurations>,
}
impl AzureMonitorRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The azure monitor status response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMonitorResponse {
    #[doc = "The status of the monitor on the HDInsight cluster."]
    #[serde(rename = "clusterMonitoringEnabled", default, skip_serializing_if = "Option::is_none")]
    pub cluster_monitoring_enabled: Option<bool>,
    #[doc = "The workspace ID of the monitor on the HDInsight cluster."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The selected configurations for azure monitor."]
    #[serde(rename = "selectedConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub selected_configurations: Option<AzureMonitorSelectedConfigurations>,
}
impl AzureMonitorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The selected configurations for azure monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMonitorSelectedConfigurations {
    #[doc = "The configuration version."]
    #[serde(rename = "configurationVersion", default, skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<String>,
    #[doc = "The global configurations of selected configurations."]
    #[serde(rename = "globalConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub global_configurations: Option<serde_json::Value>,
    #[doc = "The table list."]
    #[serde(rename = "tableList", default, skip_serializing_if = "Vec::is_empty")]
    pub table_list: Vec<AzureMonitorTableConfiguration>,
}
impl AzureMonitorSelectedConfigurations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The table configuration for the Log Analytics integration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMonitorTableConfiguration {
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AzureMonitorTableConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The billing meters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingMeters {
    #[doc = "The virtual machine sizes."]
    #[serde(rename = "meterParameter", default, skip_serializing_if = "Option::is_none")]
    pub meter_parameter: Option<String>,
    #[doc = "The HDInsight meter guid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meter: Option<String>,
    #[doc = "The unit of meter, VMHours or CoreHours."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl BillingMeters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The billing resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingResources {
    #[doc = "The region or location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "The billing meter information."]
    #[serde(rename = "billingMeters", default, skip_serializing_if = "Vec::is_empty")]
    pub billing_meters: Vec<BillingMeters>,
    #[doc = "The managed disk billing information."]
    #[serde(rename = "diskBillingMeters", default, skip_serializing_if = "Vec::is_empty")]
    pub disk_billing_meters: Vec<DiskBillingMeters>,
}
impl BillingResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response for the operation to get regional billingSpecs for a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingResponseListResult {
    #[doc = "The virtual machine sizes to include or exclude."]
    #[serde(rename = "vmSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_sizes: Vec<String>,
    #[doc = "The vm sizes which enable encryption at host."]
    #[serde(rename = "vmSizesWithEncryptionAtHost", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_sizes_with_encryption_at_host: Vec<String>,
    #[doc = "The virtual machine filtering mode. Effectively this can enabling or disabling the virtual machine sizes in a particular set."]
    #[serde(rename = "vmSizeFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_size_filters: Vec<VmSizeCompatibilityFilterV2>,
    #[doc = "The vm size properties."]
    #[serde(rename = "vmSizeProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_size_properties: Vec<VmSizeProperty>,
    #[doc = "The billing and managed disk billing resources for a region."]
    #[serde(rename = "billingResources", default, skip_serializing_if = "Vec::is_empty")]
    pub billing_resources: Vec<BillingResources>,
}
impl BillingResponseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Get Capabilities operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilitiesResult {
    #[doc = "The version capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<serde_json::Value>,
    #[doc = "The virtual machine size compatibility features."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regions: Option<serde_json::Value>,
    #[doc = "The capability features."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<String>,
    #[doc = "The regional quota capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota: Option<QuotaCapability>,
}
impl CapabilitiesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The information of AAD security group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientGroupInfo {
    #[doc = "The AAD security group name."]
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[doc = "The AAD security group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}
impl ClientGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The HDInsight cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The ETag for the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The availability zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "The properties of cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterGetProperties>,
    #[doc = "Identity for the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ClusterIdentity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Cluster {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            zones: Vec::new(),
            properties: None,
            identity: None,
            system_data: None,
        }
    }
}
#[doc = "The configuration object for the specified configuration for the specified cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterConfiguration {}
impl ClusterConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration object for the specified cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterConfigurations {
    #[doc = "The configuration object for the specified configuration for the specified cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configurations: Option<serde_json::Value>,
}
impl ClusterConfigurations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The CreateCluster request parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterCreateParametersExtended {
    #[doc = "The location of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The availability zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "The cluster create parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterCreateProperties>,
    #[doc = "Identity for the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ClusterIdentity>,
}
impl ClusterCreateParametersExtended {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The cluster create parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterCreateProperties {
    #[doc = "The version of the cluster."]
    #[serde(rename = "clusterVersion", default, skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[doc = "The type of operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<cluster_create_properties::OsType>,
    #[doc = "The cluster tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<cluster_create_properties::Tier>,
    #[doc = "The cluster definition."]
    #[serde(rename = "clusterDefinition", default, skip_serializing_if = "Option::is_none")]
    pub cluster_definition: Option<ClusterDefinition>,
    #[doc = "The kafka rest proxy configuration which contains AAD security group information."]
    #[serde(rename = "kafkaRestProperties", default, skip_serializing_if = "Option::is_none")]
    pub kafka_rest_properties: Option<KafkaRestProperties>,
    #[doc = "The security profile which contains Ssh public key for the HDInsight cluster."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[doc = "Describes the compute profile."]
    #[serde(rename = "computeProfile", default, skip_serializing_if = "Option::is_none")]
    pub compute_profile: Option<ComputeProfile>,
    #[doc = "The storage profile."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "The disk encryption properties"]
    #[serde(rename = "diskEncryptionProperties", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_properties: Option<DiskEncryptionProperties>,
    #[doc = "The encryption-in-transit properties."]
    #[serde(rename = "encryptionInTransitProperties", default, skip_serializing_if = "Option::is_none")]
    pub encryption_in_transit_properties: Option<EncryptionInTransitProperties>,
    #[doc = "The minimal supported tls version."]
    #[serde(rename = "minSupportedTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub min_supported_tls_version: Option<String>,
    #[doc = "The network properties."]
    #[serde(rename = "networkProperties", default, skip_serializing_if = "Option::is_none")]
    pub network_properties: Option<NetworkProperties>,
    #[doc = "The compute isolation properties."]
    #[serde(rename = "computeIsolationProperties", default, skip_serializing_if = "Option::is_none")]
    pub compute_isolation_properties: Option<ComputeIsolationProperties>,
    #[doc = "The private link configurations."]
    #[serde(rename = "privateLinkConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_configurations: Vec<PrivateLinkConfiguration>,
}
impl ClusterCreateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_create_properties {
    use super::*;
    #[doc = "The type of operating system."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Windows,
        Linux,
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
                Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("OsType", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The cluster tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Standard,
        Premium,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Tier", 1u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Tier {
        fn default() -> Self {
            Self::Standard
        }
    }
}
#[doc = "The cluster create request specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterCreateRequestValidationParameters {
    #[serde(flatten)]
    pub cluster_create_parameters_extended: ClusterCreateParametersExtended,
    #[doc = "The cluster name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "This indicates whether fetch Aadds resource or not."]
    #[serde(rename = "fetchAaddsResource", default, skip_serializing_if = "Option::is_none")]
    pub fetch_aadds_resource: Option<bool>,
}
impl ClusterCreateRequestValidationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of cluster create request validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterCreateValidationResult {
    #[doc = "The validation errors."]
    #[serde(rename = "validationErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_errors: Vec<ValidationErrorInfo>,
    #[doc = "The validation warnings."]
    #[serde(rename = "validationWarnings", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_warnings: Vec<ValidationErrorInfo>,
    #[doc = "The estimated creation duration."]
    #[serde(rename = "estimatedCreationDuration", default, skip_serializing_if = "Option::is_none")]
    pub estimated_creation_duration: Option<String>,
    #[doc = "The Azure active directory domain service resource details."]
    #[serde(rename = "aaddsResourcesDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub aadds_resources_details: Vec<AaddsResourceDetails>,
}
impl ClusterCreateValidationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The cluster definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterDefinition {
    #[doc = "The link to the blueprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blueprint: Option<String>,
    #[doc = "The type of cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The versions of different services in the cluster."]
    #[serde(rename = "componentVersion", default, skip_serializing_if = "Option::is_none")]
    pub component_version: Option<serde_json::Value>,
    #[doc = "The cluster configurations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configurations: Option<serde_json::Value>,
}
impl ClusterDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Disk Encryption Cluster request parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterDiskEncryptionParameters {
    #[doc = "Base key vault URI where the customers key is located eg. https://myvault.vault.azure.net"]
    #[serde(rename = "vaultUri", default, skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,
    #[doc = "Key name that is used for enabling disk encryption."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "Specific key version that is used for enabling disk encryption."]
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}
impl ClusterDiskEncryptionParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterGetProperties {
    #[doc = "The version of the cluster."]
    #[serde(rename = "clusterVersion", default, skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[doc = "The hdp version of the cluster."]
    #[serde(rename = "clusterHdpVersion", default, skip_serializing_if = "Option::is_none")]
    pub cluster_hdp_version: Option<String>,
    #[doc = "The type of operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<cluster_get_properties::OsType>,
    #[doc = "The cluster tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<cluster_get_properties::Tier>,
    #[doc = "The cluster id."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The cluster definition."]
    #[serde(rename = "clusterDefinition")]
    pub cluster_definition: ClusterDefinition,
    #[doc = "The kafka rest proxy configuration which contains AAD security group information."]
    #[serde(rename = "kafkaRestProperties", default, skip_serializing_if = "Option::is_none")]
    pub kafka_rest_properties: Option<KafkaRestProperties>,
    #[doc = "The security profile which contains Ssh public key for the HDInsight cluster."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[doc = "Describes the compute profile."]
    #[serde(rename = "computeProfile", default, skip_serializing_if = "Option::is_none")]
    pub compute_profile: Option<ComputeProfile>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cluster_get_properties::ProvisioningState>,
    #[doc = "The date on which the cluster was created."]
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[doc = "The state of the cluster."]
    #[serde(rename = "clusterState", default, skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<String>,
    #[doc = "The quota properties for the cluster."]
    #[serde(rename = "quotaInfo", default, skip_serializing_if = "Option::is_none")]
    pub quota_info: Option<QuotaInfo>,
    #[doc = "The list of errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Errors>,
    #[doc = "The list of connectivity endpoints."]
    #[serde(rename = "connectivityEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub connectivity_endpoints: Vec<ConnectivityEndpoint>,
    #[doc = "The disk encryption properties"]
    #[serde(rename = "diskEncryptionProperties", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_properties: Option<DiskEncryptionProperties>,
    #[doc = "The encryption-in-transit properties."]
    #[serde(rename = "encryptionInTransitProperties", default, skip_serializing_if = "Option::is_none")]
    pub encryption_in_transit_properties: Option<EncryptionInTransitProperties>,
    #[doc = "The storage profile."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "The minimal supported tls version."]
    #[serde(rename = "minSupportedTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub min_supported_tls_version: Option<String>,
    #[doc = "The configuration that services will be excluded when creating cluster."]
    #[serde(rename = "excludedServicesConfig", default, skip_serializing_if = "Option::is_none")]
    pub excluded_services_config: Option<ExcludedServicesConfig>,
    #[doc = "The network properties."]
    #[serde(rename = "networkProperties", default, skip_serializing_if = "Option::is_none")]
    pub network_properties: Option<NetworkProperties>,
    #[doc = "The compute isolation properties."]
    #[serde(rename = "computeIsolationProperties", default, skip_serializing_if = "Option::is_none")]
    pub compute_isolation_properties: Option<ComputeIsolationProperties>,
    #[doc = "The private link configurations."]
    #[serde(rename = "privateLinkConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_configurations: Vec<PrivateLinkConfiguration>,
    #[doc = "The list of private endpoint connections."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl ClusterGetProperties {
    pub fn new(cluster_definition: ClusterDefinition) -> Self {
        Self {
            cluster_version: None,
            cluster_hdp_version: None,
            os_type: None,
            tier: None,
            cluster_id: None,
            cluster_definition,
            kafka_rest_properties: None,
            security_profile: None,
            compute_profile: None,
            provisioning_state: None,
            created_date: None,
            cluster_state: None,
            quota_info: None,
            errors: Vec::new(),
            connectivity_endpoints: Vec::new(),
            disk_encryption_properties: None,
            encryption_in_transit_properties: None,
            storage_profile: None,
            min_supported_tls_version: None,
            excluded_services_config: None,
            network_properties: None,
            compute_isolation_properties: None,
            private_link_configurations: Vec::new(),
            private_endpoint_connections: Vec::new(),
        }
    }
}
pub mod cluster_get_properties {
    use super::*;
    #[doc = "The type of operating system."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Windows,
        Linux,
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
                Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("OsType", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The cluster tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Standard,
        Premium,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Tier", 1u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state, which only appears in the response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        InProgress,
        Failed,
        Succeeded,
        Canceled,
        Deleting,
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
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 0u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Identity for the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterIdentity {
    #[doc = "The principal id of cluster identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id associated with the cluster. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the cluster. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<cluster_identity::Type>,
    #[doc = "The list of user identities associated with the cluster. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ClusterIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_identity {
    use super::*;
    #[doc = "The type of identity used for the cluster. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
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
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 1u32, "UserAssigned"),
                Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 2u32, "SystemAssigned, UserAssigned"),
                Self::None => serializer.serialize_unit_variant("Type", 3u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The ListPersistedScriptActions operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterListPersistedScriptActionsResult {
    #[doc = "The list of Persisted Script Actions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RuntimeScriptAction>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ClusterListPersistedScriptActionsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Cluster operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterListResult {
    #[doc = "The list of Clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The cluster monitor parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterMonitoringRequest {
    #[doc = "The cluster monitor workspace ID."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The cluster monitor workspace key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
}
impl ClusterMonitoringRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The cluster monitoring status response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterMonitoringResponse {
    #[doc = "The status of the monitor on the HDInsight cluster."]
    #[serde(rename = "clusterMonitoringEnabled", default, skip_serializing_if = "Option::is_none")]
    pub cluster_monitoring_enabled: Option<bool>,
    #[doc = "The workspace ID of the monitor on the HDInsight cluster."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}
impl ClusterMonitoringResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The PatchCluster request parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterPatchParameters {
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ClusterPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resize Cluster request parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterResizeParameters {
    #[doc = "The target instance count for the operation."]
    #[serde(rename = "targetInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub target_instance_count: Option<i32>,
}
impl ClusterResizeParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The compute isolation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeIsolationProperties {
    #[doc = "The flag indicates whether enable compute isolation or not."]
    #[serde(rename = "enableComputeIsolation", default, skip_serializing_if = "Option::is_none")]
    pub enable_compute_isolation: Option<bool>,
    #[doc = "The host sku."]
    #[serde(rename = "hostSku", default, skip_serializing_if = "Option::is_none")]
    pub host_sku: Option<String>,
}
impl ComputeIsolationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the compute profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeProfile {
    #[doc = "The list of roles in the cluster."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<Role>,
}
impl ComputeProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connectivity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectivityEndpoint {
    #[doc = "The name of the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The protocol of the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "The location of the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The port to connect to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The private ip address of the endpoint."]
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}
impl ConnectivityEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The data disks groups for the role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDisksGroups {
    #[doc = "The number of disks per node."]
    #[serde(rename = "disksPerNode", default, skip_serializing_if = "Option::is_none")]
    pub disks_per_node: Option<i32>,
    #[doc = "ReadOnly. The storage account type. Do not set this value."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<String>,
    #[doc = "ReadOnly. The DiskSize in GB. Do not set this value."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
}
impl DataDisksGroups {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of Dimension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[doc = "The name of the dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display name of the dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The display name of the dimension."]
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[doc = "The flag indicates whether the metric will be exported for shoebox or not."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The disk billing meters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskBillingMeters {
    #[doc = "The managed disk meter guid."]
    #[serde(rename = "diskRpMeter", default, skip_serializing_if = "Option::is_none")]
    pub disk_rp_meter: Option<String>,
    #[doc = "The managed disk billing sku, P30 or S30."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The managed disk billing tier, Standard or Premium."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<disk_billing_meters::Tier>,
}
impl DiskBillingMeters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disk_billing_meters {
    use super::*;
    #[doc = "The managed disk billing tier, Standard or Premium."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Standard,
        Premium,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Tier", 1u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The disk encryption properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionProperties {
    #[doc = "Base key vault URI where the customers key is located eg. https://myvault.vault.azure.net"]
    #[serde(rename = "vaultUri", default, skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,
    #[doc = "Key name that is used for enabling disk encryption."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "Specific key version that is used for enabling disk encryption."]
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
    #[doc = "Algorithm identifier for encryption, default RSA-OAEP."]
    #[serde(rename = "encryptionAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<disk_encryption_properties::EncryptionAlgorithm>,
    #[doc = "Resource ID of Managed Identity that is used to access the key vault."]
    #[serde(rename = "msiResourceId", default, skip_serializing_if = "Option::is_none")]
    pub msi_resource_id: Option<String>,
    #[doc = "Indicates whether or not resource disk encryption is enabled."]
    #[serde(rename = "encryptionAtHost", default, skip_serializing_if = "Option::is_none")]
    pub encryption_at_host: Option<bool>,
}
impl DiskEncryptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disk_encryption_properties {
    use super::*;
    #[doc = "Algorithm identifier for encryption, default RSA-OAEP."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EncryptionAlgorithm")]
    pub enum EncryptionAlgorithm {
        #[serde(rename = "RSA-OAEP")]
        RsaOaep,
        #[serde(rename = "RSA-OAEP-256")]
        RsaOaep256,
        #[serde(rename = "RSA1_5")]
        Rsa15,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EncryptionAlgorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EncryptionAlgorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EncryptionAlgorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RsaOaep => serializer.serialize_unit_variant("EncryptionAlgorithm", 0u32, "RSA-OAEP"),
                Self::RsaOaep256 => serializer.serialize_unit_variant("EncryptionAlgorithm", 1u32, "RSA-OAEP-256"),
                Self::Rsa15 => serializer.serialize_unit_variant("EncryptionAlgorithm", 2u32, "RSA1_5"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The encryption-in-transit properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionInTransitProperties {
    #[doc = "Indicates whether or not inter cluster node communication is encrypted in transit."]
    #[serde(rename = "isEncryptionInTransitEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_encryption_in_transit_enabled: Option<bool>,
}
impl EncryptionInTransitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
#[doc = "The error message associated with the cluster creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Errors {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Errors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration that services will be excluded when creating cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExcludedServicesConfig {
    #[doc = "The config id of excluded services."]
    #[serde(rename = "excludedServicesConfigId", default, skip_serializing_if = "Option::is_none")]
    pub excluded_services_config_id: Option<String>,
    #[doc = "The list of excluded services."]
    #[serde(rename = "excludedServicesList", default, skip_serializing_if = "Option::is_none")]
    pub excluded_services_list: Option<String>,
}
impl ExcludedServicesConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for the script actions to execute on a running cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteScriptActionParameters {
    #[doc = "The list of run time script actions."]
    #[serde(rename = "scriptActions", default, skip_serializing_if = "Vec::is_empty")]
    pub script_actions: Vec<RuntimeScriptAction>,
    #[doc = "Gets or sets if the scripts needs to be persisted."]
    #[serde(rename = "persistOnSuccess")]
    pub persist_on_success: bool,
}
impl ExecuteScriptActionParameters {
    pub fn new(persist_on_success: bool) -> Self {
        Self {
            script_actions: Vec::new(),
            persist_on_success,
        }
    }
}
#[doc = "Cluster monitoring extensions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Extension {
    #[doc = "The workspace ID for the cluster monitoring extension."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The certificate for the cluster monitoring extensions."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
}
impl Extension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gateway settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewaySettings {
    #[doc = "Indicates whether or not the gateway settings based authorization is enabled."]
    #[serde(rename = "restAuthCredential.isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rest_auth_credential_is_enabled: Option<String>,
    #[doc = "The gateway settings user name."]
    #[serde(rename = "restAuthCredential.username", default, skip_serializing_if = "Option::is_none")]
    pub rest_auth_credential_username: Option<String>,
    #[doc = "The gateway settings user password."]
    #[serde(rename = "restAuthCredential.password", default, skip_serializing_if = "Option::is_none")]
    pub rest_auth_credential_password: Option<String>,
}
impl GatewaySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The hardware profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareProfile {
    #[doc = "The size of the VM"]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
}
impl HardwareProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The cluster host information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostInfo {
    #[doc = "The host name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Fully Qualified Domain Name of host"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The effective disk encryption key URL used by the host"]
    #[serde(rename = "effectiveDiskEncryptionKeyUrl", default, skip_serializing_if = "Option::is_none")]
    pub effective_disk_encryption_key_url: Option<String>,
}
impl HostInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type HostInfoListResult = Vec<HostInfo>;
#[doc = "The ip configurations for the private link service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpConfiguration {
    #[doc = "The private link IP configuration id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of private link IP configuration."]
    pub name: String,
    #[doc = "The type of the private link IP configuration."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The private link ip configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IpConfigurationProperties>,
}
impl IpConfiguration {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            type_: None,
            properties: None,
        }
    }
}
#[doc = "The private link ip configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpConfigurationProperties {
    #[doc = "The private link configuration provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ip_configuration_properties::ProvisioningState>,
    #[doc = "Indicates whether this IP configuration is primary for the corresponding NIC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "The IP address."]
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "The method that private IP address is allocated."]
    #[serde(rename = "privateIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<ip_configuration_properties::PrivateIpAllocationMethod>,
    #[doc = "The azure resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<ResourceId>,
}
impl IpConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ip_configuration_properties {
    use super::*;
    #[doc = "The private link configuration provisioning state, which only appears in the response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        InProgress,
        Failed,
        Succeeded,
        Canceled,
        Deleting,
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
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 0u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The method that private IP address is allocated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateIpAllocationMethod")]
    pub enum PrivateIpAllocationMethod {
        #[serde(rename = "dynamic")]
        Dynamic,
        #[serde(rename = "static")]
        Static,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateIpAllocationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateIpAllocationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateIpAllocationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dynamic => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 0u32, "dynamic"),
                Self::Static => serializer.serialize_unit_variant("PrivateIpAllocationMethod", 1u32, "static"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The kafka rest proxy configuration which contains AAD security group information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaRestProperties {
    #[doc = "The information of AAD security group."]
    #[serde(rename = "clientGroupInfo", default, skip_serializing_if = "Option::is_none")]
    pub client_group_info: Option<ClientGroupInfo>,
    #[doc = "The configurations that need to be overriden."]
    #[serde(rename = "configurationOverride", default, skip_serializing_if = "Option::is_none")]
    pub configuration_override: Option<serde_json::Value>,
}
impl KafkaRestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ssh username, password, and ssh public key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxOperatingSystemProfile {
    #[doc = "The username."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The list of SSH public keys."]
    #[serde(rename = "sshProfile", default, skip_serializing_if = "Option::is_none")]
    pub ssh_profile: Option<SshProfile>,
}
impl LinuxOperatingSystemProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details about the localizable name of a type of usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalizedName {
    #[doc = "The name of the used resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The localized name of the used resource."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl LocalizedName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecifications {
    #[doc = "The name of the metric specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display name of the metric specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The display description of the metric specification."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The unit of the metric specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The aggregation type of the metric specification."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "The supported aggregation types of the metric specification."]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "The supported time grain types of the metric specification."]
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "The flag indicates whether enable regional mdm account or not."]
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<bool>,
    #[doc = "The source mdm account."]
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[doc = "The source mdm namespace."]
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[doc = "The metric filter pattern."]
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
    #[doc = "The flag indicates whether filling gap with zero."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "The category of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The override name of resource id dimension name."]
    #[serde(rename = "resourceIdDimensionNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
    #[doc = "The flag indicates whether the metric is internal or not."]
    #[serde(rename = "isInternal", default, skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    #[doc = "The override name of delegate metric."]
    #[serde(rename = "delegateMetricNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub delegate_metric_name_override: Option<String>,
    #[doc = "The dimensions of the metric specification."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
}
impl MetricSpecifications {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request spec of checking name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailabilityCheckRequestParameters {
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NameAvailabilityCheckRequestParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response spec of checking name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailabilityCheckResult {
    #[doc = "This indicates whether the name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason of the result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The related message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NameAvailabilityCheckResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProperties {
    #[doc = "The direction for the resource provider connection."]
    #[serde(rename = "resourceProviderConnection", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_connection: Option<network_properties::ResourceProviderConnection>,
    #[doc = "Indicates whether or not private link is enabled."]
    #[serde(rename = "privateLink", default, skip_serializing_if = "Option::is_none")]
    pub private_link: Option<network_properties::PrivateLink>,
}
impl NetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_properties {
    use super::*;
    #[doc = "The direction for the resource provider connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceProviderConnection")]
    pub enum ResourceProviderConnection {
        Inbound,
        Outbound,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceProviderConnection {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceProviderConnection {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceProviderConnection {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inbound => serializer.serialize_unit_variant("ResourceProviderConnection", 0u32, "Inbound"),
                Self::Outbound => serializer.serialize_unit_variant("ResourceProviderConnection", 1u32, "Outbound"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates whether or not private link is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateLink")]
    pub enum PrivateLink {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateLink {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateLink {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateLink {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("PrivateLink", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("PrivateLink", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The HDInsight REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "The details of operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "The service provider: Microsoft.HDInsight"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource on which the operation is performed: Cluster, Applications, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The operation type: read, write, delete, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Localized friendly description for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list HDInsight operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of HDInsight operations supported by the HDInsight resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URL to get the next set of operation list results if there are any."]
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
#[doc = "The details of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "The specification of the service."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Linux operation systems profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "The ssh username, password, and ssh public key."]
    #[serde(rename = "linuxOperatingSystemProfile", default, skip_serializing_if = "Option::is_none")]
    pub linux_operating_system_profile: Option<LinuxOperatingSystemProfile>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The private endpoint id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The private endpoint connection properties."]
    pub properties: PrivateEndpointConnectionProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateEndpointConnection {
    pub fn new(properties: PrivateEndpointConnectionProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The list private endpoint connections response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "The list of private endpoint connections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint connection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The private endpoint."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "The private link service connection state."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The link identifier."]
    #[serde(rename = "linkIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub link_identifier: Option<String>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection_properties::ProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            link_identifier: None,
            provisioning_state: None,
        }
    }
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[doc = "The provisioning state, which only appears in the response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        InProgress,
        Updating,
        Failed,
        Succeeded,
        Canceled,
        Deleting,
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
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 0u32, "InProgress"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The private link configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkConfiguration {
    #[doc = "The private link configuration id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of private link configuration."]
    pub name: String,
    #[doc = "The type of the private link configuration."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The private link configuration properties."]
    pub properties: PrivateLinkConfigurationProperties,
}
impl PrivateLinkConfiguration {
    pub fn new(name: String, properties: PrivateLinkConfigurationProperties) -> Self {
        Self {
            id: None,
            name,
            type_: None,
            properties,
        }
    }
}
#[doc = "The private link configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkConfigurationProperties {
    #[doc = "The HDInsight private linkable sub-resource name to apply the private link configuration to. For example, 'headnode', 'gateway', 'edgenode'."]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "The private link configuration provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_link_configuration_properties::ProvisioningState>,
    #[doc = "The IP configurations for the private link service."]
    #[serde(rename = "ipConfigurations")]
    pub ip_configurations: Vec<IpConfiguration>,
}
impl PrivateLinkConfigurationProperties {
    pub fn new(group_id: String, ip_configurations: Vec<IpConfiguration>) -> Self {
        Self {
            group_id,
            provisioning_state: None,
            ip_configurations,
        }
    }
}
pub mod private_link_configuration_properties {
    use super::*;
    #[doc = "The private link configuration provisioning state, which only appears in the response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        InProgress,
        Failed,
        Succeeded,
        Canceled,
        Deleting,
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
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 0u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourceListResult {
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
#[doc = "The private link service connection state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The concrete private link service connection."]
    pub status: private_link_service_connection_state::Status,
    #[doc = "The optional description of the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether there is further actions."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new(status: private_link_service_connection_state::Status) -> Self {
        Self {
            status,
            description: None,
            actions_required: None,
        }
    }
}
pub mod private_link_service_connection_state {
    use super::*;
    #[doc = "The concrete private link service connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Approved,
        Rejected,
        Pending,
        Removed,
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
                Self::Approved => serializer.serialize_unit_variant("Status", 0u32, "Approved"),
                Self::Rejected => serializer.serialize_unit_variant("Status", 1u32, "Rejected"),
                Self::Pending => serializer.serialize_unit_variant("Status", 2u32, "Pending"),
                Self::Removed => serializer.serialize_unit_variant("Status", 3u32, "Removed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "The regional quota capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaCapability {
    #[doc = "The number of cores used in the subscription."]
    #[serde(rename = "coresUsed", default, skip_serializing_if = "Option::is_none")]
    pub cores_used: Option<i64>,
    #[doc = "The number of cores that the subscription allowed."]
    #[serde(rename = "maxCoresAllowed", default, skip_serializing_if = "Option::is_none")]
    pub max_cores_allowed: Option<i64>,
    #[doc = "The list of region quota capabilities."]
    #[serde(rename = "regionalQuotas", default, skip_serializing_if = "Vec::is_empty")]
    pub regional_quotas: Vec<RegionalQuotaCapability>,
}
impl QuotaCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The quota properties for the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaInfo {
    #[doc = "The cores used by the cluster."]
    #[serde(rename = "coresUsed", default, skip_serializing_if = "Option::is_none")]
    pub cores_used: Option<i32>,
}
impl QuotaInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The regional quota capacity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegionalQuotaCapability {
    #[doc = "The region name."]
    #[serde(rename = "regionName", default, skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[doc = "The number of cores used in the region."]
    #[serde(rename = "coresUsed", default, skip_serializing_if = "Option::is_none")]
    pub cores_used: Option<i64>,
    #[doc = "The number of cores available in the region."]
    #[serde(rename = "coresAvailable", default, skip_serializing_if = "Option::is_none")]
    pub cores_available: Option<i64>,
}
impl RegionalQuotaCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The regions capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegionsCapability {
    #[doc = "The list of region capabilities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available: Vec<String>,
}
impl RegionsCapability {
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
#[doc = "The azure resource id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceId {
    #[doc = "The azure resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceId {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type RestartHostsParameters = Vec<String>;
#[doc = "Describes a role on the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Role {
    #[doc = "The name of the role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The minimum instance count of the cluster."]
    #[serde(rename = "minInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub min_instance_count: Option<i32>,
    #[doc = "The instance count of the cluster."]
    #[serde(rename = "targetInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub target_instance_count: Option<i32>,
    #[doc = "The name of the virtual machine group."]
    #[serde(rename = "VMGroupName", default, skip_serializing_if = "Option::is_none")]
    pub vm_group_name: Option<String>,
    #[doc = "The autoscale request parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<Autoscale>,
    #[doc = "The hardware profile."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "The Linux operation systems profile."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "The virtual network properties."]
    #[serde(rename = "virtualNetworkProfile", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_profile: Option<VirtualNetworkProfile>,
    #[doc = "The data disks groups for the role."]
    #[serde(rename = "dataDisksGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks_groups: Vec<DataDisksGroups>,
    #[doc = "The list of script actions on the role."]
    #[serde(rename = "scriptActions", default, skip_serializing_if = "Vec::is_empty")]
    pub script_actions: Vec<ScriptAction>,
    #[doc = "Indicates whether encrypt the data disks."]
    #[serde(rename = "encryptDataDisks", default, skip_serializing_if = "Option::is_none")]
    pub encrypt_data_disks: Option<bool>,
}
impl Role {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a script action on a running cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeScriptAction {
    #[doc = "The name of the script action."]
    pub name: String,
    #[doc = "The URI to the script."]
    pub uri: String,
    #[doc = "The parameters for the script"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[doc = "The list of roles where script will be executed."]
    pub roles: Vec<String>,
    #[doc = "The application name of the script action, if any."]
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
}
impl RuntimeScriptAction {
    pub fn new(name: String, uri: String, roles: Vec<String>) -> Self {
        Self {
            name,
            uri,
            parameters: None,
            roles,
            application_name: None,
        }
    }
}
#[doc = "The execution details of a script action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeScriptActionDetail {
    #[serde(flatten)]
    pub runtime_script_action: RuntimeScriptAction,
    #[doc = "The execution id of the script action."]
    #[serde(rename = "scriptExecutionId", default, skip_serializing_if = "Option::is_none")]
    pub script_execution_id: Option<i64>,
    #[doc = "The start time of script action execution."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of script action execution."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The current execution status of the script action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The reason why the script action was executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The summary of script action execution result."]
    #[serde(rename = "executionSummary", default, skip_serializing_if = "Vec::is_empty")]
    pub execution_summary: Vec<ScriptActionExecutionSummary>,
    #[doc = "The script action execution debug information."]
    #[serde(rename = "debugInformation", default, skip_serializing_if = "Option::is_none")]
    pub debug_information: Option<String>,
}
impl RuntimeScriptActionDetail {
    pub fn new(runtime_script_action: RuntimeScriptAction) -> Self {
        Self {
            runtime_script_action,
            script_execution_id: None,
            start_time: None,
            end_time: None,
            status: None,
            operation: None,
            execution_summary: Vec::new(),
            debug_information: None,
        }
    }
}
#[doc = "Describes a script action on role on the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptAction {
    #[doc = "The name of the script action."]
    pub name: String,
    #[doc = "The URI to the script."]
    pub uri: String,
    #[doc = "The parameters for the script provided."]
    pub parameters: String,
}
impl ScriptAction {
    pub fn new(name: String, uri: String, parameters: String) -> Self {
        Self { name, uri, parameters }
    }
}
#[doc = "The list script execution history response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptActionExecutionHistoryList {
    #[doc = "The list of persisted script action details for the cluster."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RuntimeScriptActionDetail>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScriptActionExecutionHistoryList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScriptActionExecutionHistoryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The execution summary of a script action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptActionExecutionSummary {
    #[doc = "The status of script action execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The instance count for a given script action execution status."]
    #[serde(rename = "instanceCount", default, skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
}
impl ScriptActionExecutionSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The persisted script action for cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptActionPersistedGetResponseSpec {
    #[doc = "The name of script action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The URI to the script."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The parameters for the script provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[doc = "The list of roles where script will be executed."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
    #[doc = "The application name for the script action."]
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
}
impl ScriptActionPersistedGetResponseSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The persisted script action for the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptActionsList {
    #[doc = "The list of persisted script action details for the cluster."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RuntimeScriptActionDetail>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScriptActionsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScriptActionsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The security profile which contains Ssh public key for the HDInsight cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityProfile {
    #[doc = "The directory type."]
    #[serde(rename = "directoryType", default, skip_serializing_if = "Option::is_none")]
    pub directory_type: Option<security_profile::DirectoryType>,
    #[doc = "The organization's active directory domain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The organizational unit within the Active Directory to place the cluster and service accounts."]
    #[serde(rename = "organizationalUnitDN", default, skip_serializing_if = "Option::is_none")]
    pub organizational_unit_dn: Option<String>,
    #[doc = "The LDAPS protocol URLs to communicate with the Active Directory."]
    #[serde(rename = "ldapsUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub ldaps_urls: Vec<String>,
    #[doc = "The domain user account that will have admin privileges on the cluster."]
    #[serde(rename = "domainUsername", default, skip_serializing_if = "Option::is_none")]
    pub domain_username: Option<String>,
    #[doc = "The domain admin password."]
    #[serde(rename = "domainUserPassword", default, skip_serializing_if = "Option::is_none")]
    pub domain_user_password: Option<String>,
    #[doc = "Optional. The Distinguished Names for cluster user groups"]
    #[serde(rename = "clusterUsersGroupDNs", default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_users_group_d_ns: Vec<String>,
    #[doc = "The resource ID of the user's Azure Active Directory Domain Service."]
    #[serde(rename = "aaddsResourceId", default, skip_serializing_if = "Option::is_none")]
    pub aadds_resource_id: Option<String>,
    #[doc = "User assigned identity that has permissions to read and create cluster-related artifacts in the user's AADDS."]
    #[serde(rename = "msiResourceId", default, skip_serializing_if = "Option::is_none")]
    pub msi_resource_id: Option<String>,
}
impl SecurityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod security_profile {
    use super::*;
    #[doc = "The directory type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DirectoryType")]
    pub enum DirectoryType {
        ActiveDirectory,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DirectoryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DirectoryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DirectoryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ActiveDirectory => serializer.serialize_unit_variant("DirectoryType", 0u32, "ActiveDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The specification of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "The metric specifications."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecifications>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of SSH public keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshProfile {
    #[doc = "The list of SSH public keys."]
    #[serde(rename = "publicKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<SshPublicKey>,
}
impl SshProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SSH public key for the cluster nodes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshPublicKey {
    #[doc = "The certificate for SSH."]
    #[serde(rename = "certificateData", default, skip_serializing_if = "Option::is_none")]
    pub certificate_data: Option<String>,
}
impl SshPublicKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storage Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccount {
    #[doc = "The name of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether or not the storage account is the default storage account."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "The container in the storage account, only to be specified for WASB storage accounts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[doc = "The filesystem, only to be specified for Azure Data Lake Storage Gen 2."]
    #[serde(rename = "fileSystem", default, skip_serializing_if = "Option::is_none")]
    pub file_system: Option<String>,
    #[doc = "The storage account access key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The resource ID of storage account, only to be specified for Azure Data Lake Storage Gen 2."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The managed identity (MSI) that is allowed to access the storage account, only to be specified for Azure Data Lake Storage Gen 2."]
    #[serde(rename = "msiResourceId", default, skip_serializing_if = "Option::is_none")]
    pub msi_resource_id: Option<String>,
    #[doc = "The shared access signature key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saskey: Option<String>,
    #[doc = "The file share name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fileshare: Option<String>,
}
impl StorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storage profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "The list of storage accounts in the cluster."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storageaccounts: Vec<StorageAccount>,
}
impl StorageProfile {
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
#[doc = "The update cluster identity certificate request parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateClusterIdentityCertificateParameters {
    #[doc = "The application id."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "The certificate in base64 encoded format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "The password of the certificate."]
    #[serde(rename = "certificatePassword", default, skip_serializing_if = "Option::is_none")]
    pub certificate_password: Option<String>,
}
impl UpdateClusterIdentityCertificateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The update gateway settings request parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateGatewaySettingsParameters {
    #[doc = "Indicates whether or not the gateway settings based authorization is enabled."]
    #[serde(rename = "restAuthCredential.isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rest_auth_credential_is_enabled: Option<bool>,
    #[doc = "The gateway settings user name."]
    #[serde(rename = "restAuthCredential.username", default, skip_serializing_if = "Option::is_none")]
    pub rest_auth_credential_username: Option<String>,
    #[doc = "The gateway settings user password."]
    #[serde(rename = "restAuthCredential.password", default, skip_serializing_if = "Option::is_none")]
    pub rest_auth_credential_password: Option<String>,
}
impl UpdateGatewaySettingsParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details about the usage of a particular limited resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "The type of measurement for usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The current usage."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The maximum allowed usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The details about the localizable name of a type of usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizedName>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response for the operation to get regional usages for a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsagesListResult {
    #[doc = "The list of usages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl UsagesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The User Assigned Identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal id of user assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client id of user assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The tenant id of user assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The validation error information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationErrorInfo {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error resource."]
    #[serde(rename = "errorResource", default, skip_serializing_if = "Option::is_none")]
    pub error_resource: Option<String>,
    #[doc = "The message arguments"]
    #[serde(rename = "messageArguments", default, skip_serializing_if = "Vec::is_empty")]
    pub message_arguments: Vec<String>,
}
impl ValidationErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The version properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionSpec {
    #[doc = "The friendly name"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Whether or not the version is the default version."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "The component version property."]
    #[serde(rename = "componentVersions", default, skip_serializing_if = "Option::is_none")]
    pub component_versions: Option<serde_json::Value>,
}
impl VersionSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The version capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionsCapability {
    #[doc = "The list of version capabilities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available: Vec<VersionSpec>,
}
impl VersionsCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The virtual network properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkProfile {
    #[doc = "The ID of the virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
}
impl VirtualNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class represent a single filter object that defines a multidimensional set. The dimensions of this set are Regions, ClusterFlavors, NodeTypes and ClusterVersions. The constraint should be defined based on the following: FilterMode (Exclude vs Include), VMSizes (the vm sizes in affect of exclusion/inclusion) and the ordering of the Filters. Later filters override previous settings if conflicted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSizeCompatibilityFilterV2 {
    #[doc = "The filtering mode. Effectively this can enabling or disabling the VM sizes in a particular set."]
    #[serde(rename = "filterMode", default, skip_serializing_if = "Option::is_none")]
    pub filter_mode: Option<vm_size_compatibility_filter_v2::FilterMode>,
    #[doc = "The list of regions under the effect of the filter."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<String>,
    #[doc = "The list of cluster flavors under the effect of the filter."]
    #[serde(rename = "clusterFlavors", default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_flavors: Vec<String>,
    #[doc = "The list of node types affected by the filter."]
    #[serde(rename = "nodeTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub node_types: Vec<String>,
    #[doc = "The list of cluster versions affected in Major.Minor format."]
    #[serde(rename = "clusterVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_versions: Vec<String>,
    #[doc = "The OSType affected, Windows or Linux."]
    #[serde(rename = "osType", default, skip_serializing_if = "Vec::is_empty")]
    pub os_type: Vec<String>,
    #[doc = "The list of virtual machine sizes to include or exclude."]
    #[serde(rename = "vmSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_sizes: Vec<String>,
    #[doc = "Whether apply for ESP cluster. 'true' means only for ESP, 'false' means only for non-ESP, null or empty string or others mean for both."]
    #[serde(rename = "espApplied", default, skip_serializing_if = "Option::is_none")]
    pub esp_applied: Option<String>,
    #[doc = "Whether support compute isolation. 'true' means only for ComputeIsolationEnabled, 'false' means only for regular cluster."]
    #[serde(rename = "computeIsolationSupported", default, skip_serializing_if = "Option::is_none")]
    pub compute_isolation_supported: Option<String>,
}
impl VmSizeCompatibilityFilterV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vm_size_compatibility_filter_v2 {
    use super::*;
    #[doc = "The filtering mode. Effectively this can enabling or disabling the VM sizes in a particular set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FilterMode")]
    pub enum FilterMode {
        Exclude,
        Include,
        Recommend,
        Default,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FilterMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FilterMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FilterMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Exclude => serializer.serialize_unit_variant("FilterMode", 0u32, "Exclude"),
                Self::Include => serializer.serialize_unit_variant("FilterMode", 1u32, "Include"),
                Self::Recommend => serializer.serialize_unit_variant("FilterMode", 2u32, "Recommend"),
                Self::Default => serializer.serialize_unit_variant("FilterMode", 3u32, "Default"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The vm size property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSizeProperty {
    #[doc = "The vm size name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The number of cores that the vm size has."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cores: Option<i32>,
    #[doc = "The data disk storage tier of the vm size."]
    #[serde(rename = "dataDiskStorageTier", default, skip_serializing_if = "Option::is_none")]
    pub data_disk_storage_tier: Option<String>,
    #[doc = "The label of the vm size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The max data disk count of the vm size."]
    #[serde(rename = "maxDataDiskCount", default, skip_serializing_if = "Option::is_none")]
    pub max_data_disk_count: Option<i64>,
    #[doc = "The memory whose unit is MB of the vm size."]
    #[serde(rename = "memoryInMb", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_mb: Option<i64>,
    #[doc = "This indicates this vm size is supported by virtual machines or not"]
    #[serde(rename = "supportedByVirtualMachines", default, skip_serializing_if = "Option::is_none")]
    pub supported_by_virtual_machines: Option<bool>,
    #[doc = "The indicates this vm size is supported by web worker roles or not"]
    #[serde(rename = "supportedByWebWorkerRoles", default, skip_serializing_if = "Option::is_none")]
    pub supported_by_web_worker_roles: Option<bool>,
    #[doc = "The virtual machine resource disk size whose unit is MB of the vm size."]
    #[serde(rename = "virtualMachineResourceDiskSizeInMb", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_resource_disk_size_in_mb: Option<i64>,
    #[doc = "The web worker resource disk size whose unit is MB of the vm size."]
    #[serde(rename = "webWorkerResourceDiskSizeInMb", default, skip_serializing_if = "Option::is_none")]
    pub web_worker_resource_disk_size_in_mb: Option<i64>,
}
impl VmSizeProperty {
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
