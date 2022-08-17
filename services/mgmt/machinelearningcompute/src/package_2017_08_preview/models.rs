#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Information about the container service backing the cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsClusterProperties {
    #[doc = "The FQDN of the cluster. "]
    #[serde(rename = "clusterFqdn", default, skip_serializing_if = "Option::is_none")]
    pub cluster_fqdn: Option<String>,
    #[doc = "Type of orchestrator. It cannot be changed once the cluster is created."]
    #[serde(rename = "orchestratorType")]
    pub orchestrator_type: acs_cluster_properties::OrchestratorType,
    #[doc = "Kubernetes cluster specific properties"]
    #[serde(rename = "orchestratorProperties", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_properties: Option<KubernetesClusterProperties>,
    #[doc = "The system services deployed to the cluster"]
    #[serde(rename = "systemServices", default, skip_serializing_if = "Vec::is_empty")]
    pub system_services: Vec<SystemService>,
    #[doc = "The number of master nodes in the container service."]
    #[serde(rename = "masterCount", default, skip_serializing_if = "Option::is_none")]
    pub master_count: Option<i64>,
    #[doc = "The number of agent nodes in the Container Service. This can be changed to scale the cluster."]
    #[serde(rename = "agentCount", default, skip_serializing_if = "Option::is_none")]
    pub agent_count: Option<i64>,
    #[doc = "The Azure VM size of the agent VM nodes. This cannot be changed once the cluster is created. This list is non exhaustive; refer to https://docs.microsoft.com/en-us/azure/virtual-machines/windows/sizes for the possible VM sizes."]
    #[serde(rename = "agentVmSize", default, skip_serializing_if = "Option::is_none")]
    pub agent_vm_size: Option<acs_cluster_properties::AgentVmSize>,
}
impl AcsClusterProperties {
    pub fn new(orchestrator_type: acs_cluster_properties::OrchestratorType) -> Self {
        Self {
            cluster_fqdn: None,
            orchestrator_type,
            orchestrator_properties: None,
            system_services: Vec::new(),
            master_count: None,
            agent_count: None,
            agent_vm_size: None,
        }
    }
}
pub mod acs_cluster_properties {
    use super::*;
    #[doc = "Type of orchestrator. It cannot be changed once the cluster is created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OrchestratorType")]
    pub enum OrchestratorType {
        Kubernetes,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OrchestratorType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OrchestratorType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OrchestratorType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Kubernetes => serializer.serialize_unit_variant("OrchestratorType", 0u32, "Kubernetes"),
                Self::None => serializer.serialize_unit_variant("OrchestratorType", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Azure VM size of the agent VM nodes. This cannot be changed once the cluster is created. This list is non exhaustive; refer to https://docs.microsoft.com/en-us/azure/virtual-machines/windows/sizes for the possible VM sizes."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgentVmSize")]
    pub enum AgentVmSize {
        #[serde(rename = "Standard_A0")]
        StandardA0,
        #[serde(rename = "Standard_A1")]
        StandardA1,
        #[serde(rename = "Standard_A2")]
        StandardA2,
        #[serde(rename = "Standard_A3")]
        StandardA3,
        #[serde(rename = "Standard_A4")]
        StandardA4,
        #[serde(rename = "Standard_A5")]
        StandardA5,
        #[serde(rename = "Standard_A6")]
        StandardA6,
        #[serde(rename = "Standard_A7")]
        StandardA7,
        #[serde(rename = "Standard_A8")]
        StandardA8,
        #[serde(rename = "Standard_A9")]
        StandardA9,
        #[serde(rename = "Standard_A10")]
        StandardA10,
        #[serde(rename = "Standard_A11")]
        StandardA11,
        #[serde(rename = "Standard_D1")]
        StandardD1,
        #[serde(rename = "Standard_D2")]
        StandardD2,
        #[serde(rename = "Standard_D3")]
        StandardD3,
        #[serde(rename = "Standard_D4")]
        StandardD4,
        #[serde(rename = "Standard_D11")]
        StandardD11,
        #[serde(rename = "Standard_D12")]
        StandardD12,
        #[serde(rename = "Standard_D13")]
        StandardD13,
        #[serde(rename = "Standard_D14")]
        StandardD14,
        #[serde(rename = "Standard_D1_v2")]
        StandardD1V2,
        #[serde(rename = "Standard_D2_v2")]
        StandardD2V2,
        #[serde(rename = "Standard_D3_v2")]
        StandardD3V2,
        #[serde(rename = "Standard_D4_v2")]
        StandardD4V2,
        #[serde(rename = "Standard_D5_v2")]
        StandardD5V2,
        #[serde(rename = "Standard_D11_v2")]
        StandardD11V2,
        #[serde(rename = "Standard_D12_v2")]
        StandardD12V2,
        #[serde(rename = "Standard_D13_v2")]
        StandardD13V2,
        #[serde(rename = "Standard_D14_v2")]
        StandardD14V2,
        #[serde(rename = "Standard_G1")]
        StandardG1,
        #[serde(rename = "Standard_G2")]
        StandardG2,
        #[serde(rename = "Standard_G3")]
        StandardG3,
        #[serde(rename = "Standard_G4")]
        StandardG4,
        #[serde(rename = "Standard_G5")]
        StandardG5,
        #[serde(rename = "Standard_DS1")]
        StandardDs1,
        #[serde(rename = "Standard_DS2")]
        StandardDs2,
        #[serde(rename = "Standard_DS3")]
        StandardDs3,
        #[serde(rename = "Standard_DS4")]
        StandardDs4,
        #[serde(rename = "Standard_DS11")]
        StandardDs11,
        #[serde(rename = "Standard_DS12")]
        StandardDs12,
        #[serde(rename = "Standard_DS13")]
        StandardDs13,
        #[serde(rename = "Standard_DS14")]
        StandardDs14,
        #[serde(rename = "Standard_GS1")]
        StandardGs1,
        #[serde(rename = "Standard_GS2")]
        StandardGs2,
        #[serde(rename = "Standard_GS3")]
        StandardGs3,
        #[serde(rename = "Standard_GS4")]
        StandardGs4,
        #[serde(rename = "Standard_GS5")]
        StandardGs5,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AgentVmSize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AgentVmSize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AgentVmSize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardA0 => serializer.serialize_unit_variant("AgentVmSize", 0u32, "Standard_A0"),
                Self::StandardA1 => serializer.serialize_unit_variant("AgentVmSize", 1u32, "Standard_A1"),
                Self::StandardA2 => serializer.serialize_unit_variant("AgentVmSize", 2u32, "Standard_A2"),
                Self::StandardA3 => serializer.serialize_unit_variant("AgentVmSize", 3u32, "Standard_A3"),
                Self::StandardA4 => serializer.serialize_unit_variant("AgentVmSize", 4u32, "Standard_A4"),
                Self::StandardA5 => serializer.serialize_unit_variant("AgentVmSize", 5u32, "Standard_A5"),
                Self::StandardA6 => serializer.serialize_unit_variant("AgentVmSize", 6u32, "Standard_A6"),
                Self::StandardA7 => serializer.serialize_unit_variant("AgentVmSize", 7u32, "Standard_A7"),
                Self::StandardA8 => serializer.serialize_unit_variant("AgentVmSize", 8u32, "Standard_A8"),
                Self::StandardA9 => serializer.serialize_unit_variant("AgentVmSize", 9u32, "Standard_A9"),
                Self::StandardA10 => serializer.serialize_unit_variant("AgentVmSize", 10u32, "Standard_A10"),
                Self::StandardA11 => serializer.serialize_unit_variant("AgentVmSize", 11u32, "Standard_A11"),
                Self::StandardD1 => serializer.serialize_unit_variant("AgentVmSize", 12u32, "Standard_D1"),
                Self::StandardD2 => serializer.serialize_unit_variant("AgentVmSize", 13u32, "Standard_D2"),
                Self::StandardD3 => serializer.serialize_unit_variant("AgentVmSize", 14u32, "Standard_D3"),
                Self::StandardD4 => serializer.serialize_unit_variant("AgentVmSize", 15u32, "Standard_D4"),
                Self::StandardD11 => serializer.serialize_unit_variant("AgentVmSize", 16u32, "Standard_D11"),
                Self::StandardD12 => serializer.serialize_unit_variant("AgentVmSize", 17u32, "Standard_D12"),
                Self::StandardD13 => serializer.serialize_unit_variant("AgentVmSize", 18u32, "Standard_D13"),
                Self::StandardD14 => serializer.serialize_unit_variant("AgentVmSize", 19u32, "Standard_D14"),
                Self::StandardD1V2 => serializer.serialize_unit_variant("AgentVmSize", 20u32, "Standard_D1_v2"),
                Self::StandardD2V2 => serializer.serialize_unit_variant("AgentVmSize", 21u32, "Standard_D2_v2"),
                Self::StandardD3V2 => serializer.serialize_unit_variant("AgentVmSize", 22u32, "Standard_D3_v2"),
                Self::StandardD4V2 => serializer.serialize_unit_variant("AgentVmSize", 23u32, "Standard_D4_v2"),
                Self::StandardD5V2 => serializer.serialize_unit_variant("AgentVmSize", 24u32, "Standard_D5_v2"),
                Self::StandardD11V2 => serializer.serialize_unit_variant("AgentVmSize", 25u32, "Standard_D11_v2"),
                Self::StandardD12V2 => serializer.serialize_unit_variant("AgentVmSize", 26u32, "Standard_D12_v2"),
                Self::StandardD13V2 => serializer.serialize_unit_variant("AgentVmSize", 27u32, "Standard_D13_v2"),
                Self::StandardD14V2 => serializer.serialize_unit_variant("AgentVmSize", 28u32, "Standard_D14_v2"),
                Self::StandardG1 => serializer.serialize_unit_variant("AgentVmSize", 29u32, "Standard_G1"),
                Self::StandardG2 => serializer.serialize_unit_variant("AgentVmSize", 30u32, "Standard_G2"),
                Self::StandardG3 => serializer.serialize_unit_variant("AgentVmSize", 31u32, "Standard_G3"),
                Self::StandardG4 => serializer.serialize_unit_variant("AgentVmSize", 32u32, "Standard_G4"),
                Self::StandardG5 => serializer.serialize_unit_variant("AgentVmSize", 33u32, "Standard_G5"),
                Self::StandardDs1 => serializer.serialize_unit_variant("AgentVmSize", 34u32, "Standard_DS1"),
                Self::StandardDs2 => serializer.serialize_unit_variant("AgentVmSize", 35u32, "Standard_DS2"),
                Self::StandardDs3 => serializer.serialize_unit_variant("AgentVmSize", 36u32, "Standard_DS3"),
                Self::StandardDs4 => serializer.serialize_unit_variant("AgentVmSize", 37u32, "Standard_DS4"),
                Self::StandardDs11 => serializer.serialize_unit_variant("AgentVmSize", 38u32, "Standard_DS11"),
                Self::StandardDs12 => serializer.serialize_unit_variant("AgentVmSize", 39u32, "Standard_DS12"),
                Self::StandardDs13 => serializer.serialize_unit_variant("AgentVmSize", 40u32, "Standard_DS13"),
                Self::StandardDs14 => serializer.serialize_unit_variant("AgentVmSize", 41u32, "Standard_DS14"),
                Self::StandardGs1 => serializer.serialize_unit_variant("AgentVmSize", 42u32, "Standard_GS1"),
                Self::StandardGs2 => serializer.serialize_unit_variant("AgentVmSize", 43u32, "Standard_GS2"),
                Self::StandardGs3 => serializer.serialize_unit_variant("AgentVmSize", 44u32, "Standard_GS3"),
                Self::StandardGs4 => serializer.serialize_unit_variant("AgentVmSize", 45u32, "Standard_GS4"),
                Self::StandardGs5 => serializer.serialize_unit_variant("AgentVmSize", 46u32, "Standard_GS5"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AgentVmSize {
        fn default() -> Self {
            Self::StandardD3V2
        }
    }
}
#[doc = "AppInsights credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppInsightsCredentials {
    #[doc = "The AppInsights application ID."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "The AppInsights instrumentation key. This is not returned in response of GET/PUT on the resource. To see this please call listKeys API."]
    #[serde(rename = "instrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub instrumentation_key: Option<String>,
}
impl AppInsightsCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of App Insights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppInsightsProperties {
    #[doc = "ARM resource ID of the App Insights."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl AppInsightsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AutoScale configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoScaleConfiguration {
    #[doc = "If auto-scale is enabled for all services. Each service can turn it off individually."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<auto_scale_configuration::Status>,
    #[doc = "The minimum number of replicas for each service."]
    #[serde(rename = "minReplicas", default, skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i64>,
    #[doc = "The maximum number of replicas for each service."]
    #[serde(rename = "maxReplicas", default, skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i64>,
    #[doc = "The target utilization."]
    #[serde(rename = "targetUtilization", default, skip_serializing_if = "Option::is_none")]
    pub target_utilization: Option<f64>,
    #[doc = "Refresh period in seconds."]
    #[serde(rename = "refreshPeriodInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub refresh_period_in_seconds: Option<i64>,
}
impl AutoScaleConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_scale_configuration {
    use super::*;
    #[doc = "If auto-scale is enabled for all services. Each service can turn it off individually."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "Available operation list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperations {
    #[doc = "An array of available operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceOperation>,
}
impl AvailableOperations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about updates available for system services in a cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckSystemServicesUpdatesAvailableResponse {
    #[doc = "Yes if updates are available for the system services, No if not."]
    #[serde(rename = "updatesAvailable", default, skip_serializing_if = "Option::is_none")]
    pub updates_available: Option<check_system_services_updates_available_response::UpdatesAvailable>,
}
impl CheckSystemServicesUpdatesAvailableResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_system_services_updates_available_response {
    use super::*;
    #[doc = "Yes if updates are available for the system services, No if not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UpdatesAvailable")]
    pub enum UpdatesAvailable {
        Yes,
        No,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UpdatesAvailable {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UpdatesAvailable {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UpdatesAvailable {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Yes => serializer.serialize_unit_variant("UpdatesAvailable", 0u32, "Yes"),
                Self::No => serializer.serialize_unit_variant("UpdatesAvailable", 1u32, "No"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about the Azure Container Registry which contains the images deployed to the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryCredentials {
    #[doc = "The ACR login server name. User name is the first part of the FQDN."]
    #[serde(rename = "loginServer", default, skip_serializing_if = "Option::is_none")]
    pub login_server: Option<String>,
    #[doc = "The ACR primary password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The ACR secondary password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password2: Option<String>,
    #[doc = "The ACR login username."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl ContainerRegistryCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Azure Container Registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryProperties {
    #[doc = "ARM resource ID of the Azure Container Registry used to store Docker images for web services in the cluster. If not provided one will be created. This cannot be changed once the cluster is created."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl ContainerRegistryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the Azure Container Registry which contains the images deployed to the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceCredentials {
    #[doc = "The ACS kube config file."]
    #[serde(rename = "acsKubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub acs_kube_config: Option<String>,
    #[doc = "The Azure service principal used by Kubernetes for configuring load balancers"]
    #[serde(rename = "servicePrincipalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_configuration: Option<ServicePrincipalProperties>,
    #[doc = "The ACR image pull secret name which was created in Kubernetes."]
    #[serde(rename = "imagePullSecretName", default, skip_serializing_if = "Option::is_none")]
    pub image_pull_secret_name: Option<String>,
}
impl ContainerServiceCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error detail information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[doc = "Error code."]
    pub code: String,
    #[doc = "Error message."]
    pub message: String,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "Error response information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[doc = "Error code."]
    pub code: String,
    #[doc = "Error message."]
    pub message: String,
    #[doc = "An array of error detail objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
        }
    }
}
#[doc = "Wrapper for error response to follow ARM guidelines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseWrapper {
    #[doc = "Error response information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl ErrorResponseWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Global configuration for services in the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalServiceConfiguration {
    #[doc = "The configuration ETag for updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "SSL configuration. If configured data-plane calls to user services will be exposed over SSL only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl: Option<SslConfiguration>,
    #[doc = "Global service auth configuration properties. These are the data-plane authorization keys and are used if a service doesn't define it's own."]
    #[serde(rename = "serviceAuth", default, skip_serializing_if = "Option::is_none")]
    pub service_auth: Option<ServiceAuthConfiguration>,
    #[doc = "AutoScale configuration properties."]
    #[serde(rename = "autoScale", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale: Option<AutoScaleConfiguration>,
}
impl GlobalServiceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kubernetes cluster specific properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesClusterProperties {
    #[doc = "The Azure service principal used by Kubernetes for configuring load balancers"]
    #[serde(rename = "servicePrincipal", default, skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<ServicePrincipalProperties>,
}
impl KubernetesClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Instance of an Azure ML Operationalization Cluster resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationalizationCluster {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an operationalization cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationalizationClusterProperties>,
}
impl OperationalizationCluster {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "Credentials to resources in the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationalizationClusterCredentials {
    #[doc = "Access information for the storage account."]
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<StorageAccountCredentials>,
    #[doc = "Information about the Azure Container Registry which contains the images deployed to the cluster."]
    #[serde(rename = "containerRegistry", default, skip_serializing_if = "Option::is_none")]
    pub container_registry: Option<ContainerRegistryCredentials>,
    #[doc = "Information about the Azure Container Registry which contains the images deployed to the cluster."]
    #[serde(rename = "containerService", default, skip_serializing_if = "Option::is_none")]
    pub container_service: Option<ContainerServiceCredentials>,
    #[doc = "AppInsights credentials."]
    #[serde(rename = "appInsights", default, skip_serializing_if = "Option::is_none")]
    pub app_insights: Option<AppInsightsCredentials>,
    #[doc = "Global service auth configuration properties. These are the data-plane authorization keys and are used if a service doesn't define it's own."]
    #[serde(rename = "serviceAuthConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub service_auth_configuration: Option<ServiceAuthConfiguration>,
    #[doc = "SSL configuration. If configured data-plane calls to user services will be exposed over SSL only."]
    #[serde(rename = "sslConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub ssl_configuration: Option<SslConfiguration>,
}
impl OperationalizationClusterCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an operationalization cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationalizationClusterProperties {
    #[doc = "The description of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The date and time when the cluster was created."]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The date and time when the cluster was last modified."]
    #[serde(rename = "modifiedOn", default, with = "azure_core::date::rfc3339::option")]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "The provision state of the cluster. Valid values are Unknown, Updating, Provisioning, Succeeded, and Failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<operationalization_cluster_properties::ProvisioningState>,
    #[doc = "List of provisioning errors reported by the resource provider."]
    #[serde(rename = "provisioningErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub provisioning_errors: Vec<ErrorResponseWrapper>,
    #[doc = "The cluster type."]
    #[serde(rename = "clusterType")]
    pub cluster_type: operationalization_cluster_properties::ClusterType,
    #[doc = "Properties of Storage Account."]
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<StorageAccountProperties>,
    #[doc = "Properties of Azure Container Registry."]
    #[serde(rename = "containerRegistry", default, skip_serializing_if = "Option::is_none")]
    pub container_registry: Option<ContainerRegistryProperties>,
    #[doc = "Information about the container service backing the cluster"]
    #[serde(rename = "containerService", default, skip_serializing_if = "Option::is_none")]
    pub container_service: Option<AcsClusterProperties>,
    #[doc = "Properties of App Insights."]
    #[serde(rename = "appInsights", default, skip_serializing_if = "Option::is_none")]
    pub app_insights: Option<AppInsightsProperties>,
    #[doc = "Global configuration for services in the cluster."]
    #[serde(rename = "globalServiceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub global_service_configuration: Option<GlobalServiceConfiguration>,
}
impl OperationalizationClusterProperties {
    pub fn new(cluster_type: operationalization_cluster_properties::ClusterType) -> Self {
        Self {
            description: None,
            created_on: None,
            modified_on: None,
            provisioning_state: None,
            provisioning_errors: Vec::new(),
            cluster_type,
            storage_account: None,
            container_registry: None,
            container_service: None,
            app_insights: None,
            global_service_configuration: None,
        }
    }
}
pub mod operationalization_cluster_properties {
    use super::*;
    #[doc = "The provision state of the cluster. Valid values are Unknown, Updating, Provisioning, Succeeded, and Failed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Unknown,
        Updating,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
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
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The cluster type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClusterType")]
    pub enum ClusterType {
        #[serde(rename = "ACS")]
        Acs,
        Local,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClusterType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClusterType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClusterType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Acs => serializer.serialize_unit_variant("ClusterType", 0u32, "ACS"),
                Self::Local => serializer.serialize_unit_variant("ClusterType", 1u32, "Local"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters for PATCH operation on an operationalization cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationalizationClusterUpdateParameters {
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater in length than 128 characters and a value no greater in length than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl OperationalizationClusterUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paginated list of operationalization clusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedOperationalizationClustersList {
    #[doc = "An array of cluster objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationalizationCluster>,
    #[doc = "A continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedOperationalizationClustersList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PaginatedOperationalizationClustersList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the location of the resource."]
    pub location: String,
    #[doc = "Specifies the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Contains resource tags defined as key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            location,
            type_: None,
            tags: None,
        }
    }
}
#[doc = "Resource operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceOperation {
    #[doc = "Name of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_operation::Display>,
    #[doc = "The operation origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl ResourceOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_operation {
    use super::*;
    #[doc = "Display of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The resource provider name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The resource name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Global service auth configuration properties. These are the data-plane authorization keys and are used if a service doesn't define it's own."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAuthConfiguration {
    #[doc = "The primary auth key hash. This is not returned in response of GET/PUT on the resource.. To see this please call listKeys API."]
    #[serde(rename = "primaryAuthKeyHash")]
    pub primary_auth_key_hash: String,
    #[doc = "The secondary auth key hash. This is not returned in response of GET/PUT on the resource.. To see this please call listKeys API."]
    #[serde(rename = "secondaryAuthKeyHash")]
    pub secondary_auth_key_hash: String,
}
impl ServiceAuthConfiguration {
    pub fn new(primary_auth_key_hash: String, secondary_auth_key_hash: String) -> Self {
        Self {
            primary_auth_key_hash,
            secondary_auth_key_hash,
        }
    }
}
#[doc = "The Azure service principal used by Kubernetes for configuring load balancers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalProperties {
    #[doc = "The service principal client ID"]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "The service principal secret. This is not returned in response of GET/PUT on the resource. To see this please call listKeys."]
    pub secret: String,
}
impl ServicePrincipalProperties {
    pub fn new(client_id: String, secret: String) -> Self {
        Self { client_id, secret }
    }
}
#[doc = "SSL configuration. If configured data-plane calls to user services will be exposed over SSL only."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SslConfiguration {
    #[doc = "SSL status. Allowed values are Enabled and Disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ssl_configuration::Status>,
    #[doc = "The SSL cert data in PEM format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    #[doc = "The SSL key data in PEM format. This is not returned in response of GET/PUT on the resource. To see this please call listKeys API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The CName of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
}
impl SslConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ssl_configuration {
    use super::*;
    #[doc = "SSL status. Allowed values are Enabled and Disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Access information for the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountCredentials {
    #[doc = "The ARM resource ID of the storage account."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The primary key of the storage account."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary key of the storage account."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl StorageAccountCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Storage Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountProperties {
    #[doc = "ARM resource ID of the Azure Storage Account to store CLI specific files. If not provided one will be created. This cannot be changed once the cluster is created."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl StorageAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a system service deployed in the cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemService {
    #[doc = "The system service type"]
    #[serde(rename = "systemServiceType")]
    pub system_service_type: system_service::SystemServiceType,
    #[doc = "The public IP address of the system service"]
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[doc = "The state of the system service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SystemService {
    pub fn new(system_service_type: system_service::SystemServiceType) -> Self {
        Self {
            system_service_type,
            public_ip_address: None,
            version: None,
        }
    }
}
pub mod system_service {
    use super::*;
    #[doc = "The system service type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SystemServiceType")]
    pub enum SystemServiceType {
        None,
        ScoringFrontEnd,
        BatchFrontEnd,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SystemServiceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SystemServiceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SystemServiceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("SystemServiceType", 0u32, "None"),
                Self::ScoringFrontEnd => serializer.serialize_unit_variant("SystemServiceType", 1u32, "ScoringFrontEnd"),
                Self::BatchFrontEnd => serializer.serialize_unit_variant("SystemServiceType", 2u32, "BatchFrontEnd"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response of the update system services API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSystemServicesResponse {
    #[doc = "Update status"]
    #[serde(rename = "updateStatus", default, skip_serializing_if = "Option::is_none")]
    pub update_status: Option<update_system_services_response::UpdateStatus>,
    #[doc = "The date and time when the last system services update was started."]
    #[serde(rename = "updateStartedOn", default, with = "azure_core::date::rfc3339::option")]
    pub update_started_on: Option<time::OffsetDateTime>,
    #[doc = "The date and time when the last system services update completed."]
    #[serde(rename = "updateCompletedOn", default, with = "azure_core::date::rfc3339::option")]
    pub update_completed_on: Option<time::OffsetDateTime>,
}
impl UpdateSystemServicesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_system_services_response {
    use super::*;
    #[doc = "Update status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UpdateStatus")]
    pub enum UpdateStatus {
        Unknown,
        Updating,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UpdateStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UpdateStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UpdateStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("UpdateStatus", 0u32, "Unknown"),
                Self::Updating => serializer.serialize_unit_variant("UpdateStatus", 1u32, "Updating"),
                Self::Creating => serializer.serialize_unit_variant("UpdateStatus", 2u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("UpdateStatus", 3u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("UpdateStatus", 4u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("UpdateStatus", 5u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("UpdateStatus", 6u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
