#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Class for ACR Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcrProperties {
    #[doc = "Gets or sets the azure container registry name."]
    #[serde(rename = "registryName", default, skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[doc = "Gets or sets the tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Gets or sets the subscription id of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Gets or sets the resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl AcrProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for AKSDeployment Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AksDeploymentProperties {
    #[doc = "Gets or sets the AKS cluster name."]
    #[serde(rename = "aksClusterName", default, skip_serializing_if = "Option::is_none")]
    pub aks_cluster_name: Option<String>,
    #[doc = "Gets or sets the tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Gets or sets the subscription id of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Gets or sets the resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl AksDeploymentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AKS Deployment Specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AksDeploymentSpecification {
    #[doc = "Gets or sets the Merged Deployment and service Yaml."]
    #[serde(rename = "kubernetesObjectsYaml", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_objects_yaml: Option<String>,
    #[doc = "Gets or sets the replica count to be created in AKS."]
    #[serde(rename = "replicaCount", default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<String>,
    #[doc = "Gets or sets the load balancer type."]
    #[serde(rename = "loadBalancerType", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_type: Option<aks_deployment_specification::LoadBalancerType>,
}
impl AksDeploymentSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aks_deployment_specification {
    use super::*;
    #[doc = "Gets or sets the load balancer type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LoadBalancerType")]
    pub enum LoadBalancerType {
        Private,
        Public,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LoadBalancerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LoadBalancerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LoadBalancerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Private => serializer.serialize_unit_variant("LoadBalancerType", 0u32, "Private"),
                Self::Public => serializer.serialize_unit_variant("LoadBalancerType", 1u32, "Public"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "ApacheTomcat web application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApacheTomcatAksWorkloadDeployment {
    #[doc = "Class for AKSDeployment Properties."]
    #[serde(rename = "clusterProperties", default, skip_serializing_if = "Option::is_none")]
    pub cluster_properties: Option<AksDeploymentProperties>,
    #[doc = "AKS Deployment Specification."]
    #[serde(rename = "deploymentSpec", default, skip_serializing_if = "Option::is_none")]
    pub deployment_spec: Option<AksDeploymentSpecification>,
    #[doc = "Gets or sets the deployment history."]
    #[serde(
        rename = "deploymentHistory",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deployment_history: Vec<DeployedResourcesProperties>,
    #[doc = "Gets or sets the deployment name prefix."]
    #[serde(rename = "deploymentNamePrefix", default, skip_serializing_if = "Option::is_none")]
    pub deployment_name_prefix: Option<String>,
    #[doc = "Class for automation artifact."]
    #[serde(rename = "automationArtifactProperties", default, skip_serializing_if = "Option::is_none")]
    pub automation_artifact_properties: Option<AutomationArtifact>,
    #[doc = "Gets or sets application directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<WebApplicationDirectory>,
    #[doc = "Gets or sets application configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configurations: Vec<WebApplicationConfiguration>,
    #[doc = "Class for container image properties."]
    #[serde(rename = "containerImageProperties", default, skip_serializing_if = "Option::is_none")]
    pub container_image_properties: Option<ContainerImageProperties>,
    #[doc = "Gets or sets the build container images."]
    #[serde(
        rename = "buildContainerImages",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub build_container_images: Vec<ContainerImageProperties>,
    #[doc = "Gets or sets the bindings for the application."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bindings: Vec<Binding>,
    #[doc = "Resource Requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<ResourceRequirements>,
    #[doc = "Resource Requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceRequirements>,
    #[doc = "Gets or sets the target platform managed identity."]
    #[serde(rename = "targetPlatformIdentity", default, skip_serializing_if = "Option::is_none")]
    pub target_platform_identity: Option<String>,
    #[doc = "Class for app insight monitoring properties."]
    #[serde(rename = "monitoringProperties", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_properties: Option<AppInsightMonitoringProperties>,
}
impl ApacheTomcatAksWorkloadDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ApacheTomcat workload instance model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApacheTomcatAksWorkloadDeploymentModelCustomProperties {
    #[serde(flatten)]
    pub workload_deployment_model_custom_properties: WorkloadDeploymentModelCustomProperties,
    #[doc = "ApacheTomcat web application."]
    #[serde(
        rename = "apacheTomcatAksWorkloadDeploymentProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub apache_tomcat_aks_workload_deployment_properties: Option<ApacheTomcatAksWorkloadDeployment>,
}
impl ApacheTomcatAksWorkloadDeploymentModelCustomProperties {
    pub fn new(workload_deployment_model_custom_properties: WorkloadDeploymentModelCustomProperties) -> Self {
        Self {
            workload_deployment_model_custom_properties,
            apache_tomcat_aks_workload_deployment_properties: None,
        }
    }
}
#[doc = "ApacheTomcat web application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApacheTomcatWebApplication {
    #[doc = "Gets or sets the web application id."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Gets or sets the web application name."]
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[doc = "Gets or sets application scratch path."]
    #[serde(rename = "applicationScratchPath", default, skip_serializing_if = "Option::is_none")]
    pub application_scratch_path: Option<String>,
    #[doc = "Gets or sets the web server id."]
    #[serde(rename = "webServerId", default, skip_serializing_if = "Option::is_none")]
    pub web_server_id: Option<String>,
    #[doc = "Gets or sets the web server name."]
    #[serde(rename = "webServerName", default, skip_serializing_if = "Option::is_none")]
    pub web_server_name: Option<String>,
    #[doc = "Gets or sets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets application directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<WebApplicationDirectory>,
    #[doc = "Gets or sets application configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configurations: Vec<WebApplicationConfiguration>,
    #[doc = "Second level entity for virtual directories."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<DirectoryPath>,
    #[doc = "Gets or sets the bindings for the application."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bindings: Vec<Binding>,
    #[doc = "Framework specific data for a web application."]
    #[serde(rename = "primaryFramework", default, skip_serializing_if = "Option::is_none")]
    pub primary_framework: Option<WebApplicationFramework>,
    #[doc = "Gets or sets the discovered frameworks of application."]
    #[serde(
        rename = "discoveredFrameworks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub discovered_frameworks: Vec<WebApplicationFramework>,
    #[doc = "Resource Requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<ResourceRequirements>,
    #[doc = "Resource Requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceRequirements>,
}
impl ApacheTomcatWebApplication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ApacheTomcat workload instance model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApacheTomcatWorkloadInstanceModelCustomProperties {
    #[serde(flatten)]
    pub workload_instance_model_custom_properties: WorkloadInstanceModelCustomProperties,
    #[doc = "ApacheTomcat web application."]
    #[serde(rename = "apacheTomcatWebApplication", default, skip_serializing_if = "Option::is_none")]
    pub apache_tomcat_web_application: Option<ApacheTomcatWebApplication>,
}
impl ApacheTomcatWorkloadInstanceModelCustomProperties {
    pub fn new(workload_instance_model_custom_properties: WorkloadInstanceModelCustomProperties) -> Self {
        Self {
            workload_instance_model_custom_properties,
            apache_tomcat_web_application: None,
        }
    }
}
#[doc = "Class for app insight monitoring properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppInsightMonitoringProperties {
    #[doc = "Gets or sets the subscription id of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Gets or sets the resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Gets or sets the app insights name."]
    #[serde(rename = "appInsightsName", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_name: Option<String>,
    #[doc = "Gets or sets the region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Gets or sets a value indicating whether monitoring is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "secretStoreDetails", default, skip_serializing_if = "Option::is_none")]
    pub secret_store_details: Option<SecretStoreDetails>,
}
impl AppInsightMonitoringProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServiceSettingSecretStoreProperties {
    #[serde(flatten)]
    pub secret_store_properties: SecretStoreProperties,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "appServiceName", default, skip_serializing_if = "Option::is_none")]
    pub app_service_name: Option<String>,
}
impl AppServiceSettingSecretStoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmError {
    #[doc = "Arm error information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ArmErrorInfo>,
}
impl azure_core::Continuable for ArmError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ArmError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Arm error information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmErrorInfo {
    #[doc = "Gets or sets the error code returned by the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets or sets error Message returned by the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ArmErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for automation artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationArtifact {
    #[doc = "Gets or sets the status of automation artifacts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<automation_artifact::Status>,
    #[doc = "Azure file share profile for hydration of application folders not mounted on\r\nthe container file system."]
    #[serde(rename = "azureFileShareProfile", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_profile: Option<AzureFileShareHydrationProfile>,
    #[doc = "Gets or sets the artifacts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<serde_json::Value>,
}
impl AutomationArtifact {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod automation_artifact {
    use super::*;
    #[doc = "Gets or sets the status of automation artifacts."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NotGenerated,
        Generated,
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
                Self::NotGenerated => serializer.serialize_unit_variant("Status", 0u32, "NotGenerated"),
                Self::Generated => serializer.serialize_unit_variant("Status", 1u32, "Generated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure file share profile for hydration of application folders not mounted on\r\nthe container file system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFileShareHydrationProfile {
    #[doc = "Gets or sets the name of the azure file share."]
    #[serde(rename = "azureFileShareName", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_name: Option<String>,
    #[doc = "Gets or sets the subscription id of the azure file share."]
    #[serde(rename = "azureFileShareSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_subscription_id: Option<String>,
    #[doc = "Gets or sets the name of the azure file share resource group."]
    #[serde(rename = "azureFileShareResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_resource_group: Option<String>,
    #[doc = "Gets or sets the name of the azure file share storage account."]
    #[serde(rename = "azureFileShareStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_storage_account: Option<String>,
    #[doc = "Gets or sets the cloud directory path of the directory on azure file share."]
    #[serde(rename = "azureFileShareDirPath", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_dir_path: Option<String>,
}
impl AzureFileShareHydrationProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Binding for a web application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Binding {
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the application port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "WebApplication port mapping."]
    #[serde(rename = "portMapping", default, skip_serializing_if = "Option::is_none")]
    pub port_mapping: Option<PortMapping>,
    #[doc = "WebApplication certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<Cert>,
    #[doc = "Gets or sets the binding host name."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Gets or sets the protocol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "Gets or sets the IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl Binding {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BuildContainerImage model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildContainerImageModel {
    #[doc = "Class for container image properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContainerImageProperties>,
}
impl BuildContainerImageModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Build container image workflow model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildContainerImageWorkflowModelCustomProperties {
    #[serde(flatten)]
    pub workflow_model_custom_properties: WorkflowModelCustomProperties,
    #[serde(rename = "containerImageProperties", default, skip_serializing_if = "Option::is_none")]
    pub container_image_properties: Option<serde_json::Value>,
}
impl BuildContainerImageWorkflowModelCustomProperties {
    pub fn new(workflow_model_custom_properties: WorkflowModelCustomProperties) -> Self {
        Self {
            workflow_model_custom_properties,
            container_image_properties: None,
        }
    }
}
#[doc = "WebApplication certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cert {
    #[doc = "Gets or sets a value indicating whether certificate is needed or not."]
    #[serde(rename = "certNeeded", default, skip_serializing_if = "Option::is_none")]
    pub cert_needed: Option<bool>,
    #[doc = "Gets or sets a value indicating whether certificate is provided or not."]
    #[serde(rename = "certProvided", default, skip_serializing_if = "Option::is_none")]
    pub cert_provided: Option<bool>,
    #[doc = "Gets or sets the Certificate data."]
    #[serde(rename = "certData", default, skip_serializing_if = "Option::is_none")]
    pub cert_data: Option<String>,
    #[doc = "Gets or sets the type of secret store for the certificate."]
    #[serde(rename = "secretStore", default, skip_serializing_if = "Option::is_none")]
    pub secret_store: Option<cert::SecretStore>,
}
impl Cert {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cert {
    use super::*;
    #[doc = "Gets or sets the type of secret store for the certificate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecretStore")]
    pub enum SecretStore {
        None,
        KubeSecret,
        KeyVaultSecret,
        AppServiceAppSettings,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecretStore {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecretStore {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecretStore {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("SecretStore", 0u32, "None"),
                Self::KubeSecret => serializer.serialize_unit_variant("SecretStore", 1u32, "KubeSecret"),
                Self::KeyVaultSecret => serializer.serialize_unit_variant("SecretStore", 2u32, "KeyVaultSecret"),
                Self::AppServiceAppSettings => serializer.serialize_unit_variant("SecretStore", 3u32, "AppServiceAppSettings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Class for container image properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerImageProperties {
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the dockerfile for the container image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<String>,
    #[doc = "Class for ACR Properties."]
    #[serde(rename = "registryProperties", default, skip_serializing_if = "Option::is_none")]
    pub registry_properties: Option<AcrProperties>,
    #[doc = "Gets or sets the container image tag."]
    #[serde(rename = "imageTag", default, skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    #[doc = "Gets or sets the container image name."]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Gets or sets the RunId."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "Gets or sets the RunStatus."]
    #[serde(rename = "runStatus", default, skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
}
impl ContainerImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DeployedResource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedResourceModel {
    #[doc = "DeployedResource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeployedResourceModelProperties>,
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
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DeployedResourceModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DeployedResource model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedResourceModelCollection {
    #[doc = "Gets or sets the list of deployedResources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeployedResourceModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeployedResourceModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeployedResourceModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DeployedResource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedResourceModelProperties {
    #[doc = "Gets or sets the list of resources deployed."]
    #[serde(rename = "resourcesDeployed", default, skip_serializing_if = "Option::is_none")]
    pub resources_deployed: Option<serde_json::Value>,
    #[doc = "Gets or sets the workload deployment id."]
    #[serde(rename = "workloadDeploymentId", default, skip_serializing_if = "Option::is_none")]
    pub workload_deployment_id: Option<String>,
    #[doc = "Gets or sets the name of deployed resources."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the deployment timestamp."]
    #[serde(rename = "deploymentTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub deployment_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the deployment target service."]
    #[serde(rename = "targetAzureService", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_service: Option<deployed_resource_model_properties::TargetAzureService>,
    #[doc = "Gets or sets the container registry ARM Id."]
    #[serde(rename = "containerRegistryId", default, skip_serializing_if = "Option::is_none")]
    pub container_registry_id: Option<String>,
    #[doc = "Gets or sets the image name."]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Gets or sets the image tag."]
    #[serde(rename = "imageTag", default, skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    #[doc = "Gets or sets the app ip address."]
    #[serde(rename = "appIp", default, skip_serializing_if = "Option::is_none")]
    pub app_ip: Option<String>,
    #[doc = "Gets or sets the secret store ARM Id."]
    #[serde(rename = "secretStoreId", default, skip_serializing_if = "Option::is_none")]
    pub secret_store_id: Option<String>,
    #[doc = "Gets or sets the custom deployed resource properties."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,
}
impl DeployedResourceModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployed_resource_model_properties {
    use super::*;
    #[doc = "Gets or sets the deployment target service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetAzureService")]
    pub enum TargetAzureService {
        AzureKubernetesService,
        AzureAppServiceContainer,
        AzureAppServiceNative,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetAzureService {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetAzureService {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetAzureService {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureKubernetesService => serializer.serialize_unit_variant("TargetAzureService", 0u32, "AzureKubernetesService"),
                Self::AzureAppServiceContainer => serializer.serialize_unit_variant("TargetAzureService", 1u32, "AzureAppServiceContainer"),
                Self::AzureAppServiceNative => serializer.serialize_unit_variant("TargetAzureService", 2u32, "AzureAppServiceNative"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Class for deployed resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployedResourcesProperties {
    #[doc = "Gets or sets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the deployed resource id."]
    #[serde(rename = "deployedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub deployed_resource_id: Option<String>,
    #[doc = "Gets or sets the name of deployed resources."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the context of deployed resources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Gets or sets the status of deployed resources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the type of deployed resources."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the deployment timestamp."]
    #[serde(rename = "deploymentTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub deployment_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets a value indicating whether resources are cleaned up from target."]
    #[serde(rename = "isCleanUpDone", default, skip_serializing_if = "Option::is_none")]
    pub is_clean_up_done: Option<bool>,
    #[doc = "Gets or sets a value indicating whether scenario is test migration."]
    #[serde(rename = "isTestMigration", default, skip_serializing_if = "Option::is_none")]
    pub is_test_migration: Option<bool>,
}
impl DeployedResourcesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level entity for virtual directories."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectoryPath {
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the virtual path for the directory."]
    #[serde(rename = "virtual", default, skip_serializing_if = "Option::is_none")]
    pub virtual_: Option<String>,
    #[doc = "Gets or sets the physical path of the directory on the web server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical: Option<String>,
}
impl DirectoryPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enable replication workflow model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableReplicationWorkflowModelCustomProperties {
    #[serde(flatten)]
    pub workflow_model_custom_properties: WorkflowModelCustomProperties,
    #[serde(rename = "workloadInstanceProperties", default, skip_serializing_if = "Option::is_none")]
    pub workload_instance_properties: Option<serde_json::Value>,
}
impl EnableReplicationWorkflowModelCustomProperties {
    pub fn new(workflow_model_custom_properties: WorkflowModelCustomProperties) -> Self {
        Self {
            workflow_model_custom_properties,
            workload_instance_properties: None,
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
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
#[doc = "Class for GMSA authentication details to configure Active Directory connectivity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GmsaAuthenticationProperties {
    #[doc = "Gets or sets the current state of GMSA configuration."]
    #[serde(rename = "configurationState", default, skip_serializing_if = "Option::is_none")]
    pub configuration_state: Option<gmsa_authentication_properties::ConfigurationState>,
    #[doc = "Gets or sets the name to be used for GMSA."]
    #[serde(rename = "gmsaAccountName", default, skip_serializing_if = "Option::is_none")]
    pub gmsa_account_name: Option<String>,
    #[doc = "Gets or sets username of the user having authorization to access GMSA on Active Directory."]
    #[serde(rename = "gmsaUsername", default, skip_serializing_if = "Option::is_none")]
    pub gmsa_username: Option<String>,
    #[doc = "Gets or sets the password of the user specified by RestApi.Controllers.V2022_05_01_preview.Models.WorkloadDeployment.Gmsa.GmsaAuthenticationProperties.GmsaUsername."]
    #[serde(rename = "gmsaUserPassword", default, skip_serializing_if = "Option::is_none")]
    pub gmsa_user_password: Option<String>,
    #[doc = "Gets or sets the list of dns server that can resolve the Active Directory Domain Name/Address."]
    #[serde(rename = "adDomainControllerDns", default, skip_serializing_if = "Option::is_none")]
    pub ad_domain_controller_dns: Option<String>,
    #[doc = "Gets or sets the FQDN of the Active Directory Domain. For e.g. 'contoso.local', 'fareast.corp.microsoft.com' etc."]
    #[serde(rename = "adDomainFqdn", default, skip_serializing_if = "Option::is_none")]
    pub ad_domain_fqdn: Option<String>,
    #[doc = "Gets or sets the address of the Active Directory Domain Controller running Domain Services."]
    #[serde(rename = "domainControllerAddress", default, skip_serializing_if = "Option::is_none")]
    pub domain_controller_address: Option<String>,
    #[doc = "Gets or sets the name of the user having admin rights on the Active Directory Domain Controller."]
    #[serde(rename = "domainAdminUsername", default, skip_serializing_if = "Option::is_none")]
    pub domain_admin_username: Option<String>,
    #[doc = "Gets or sets the password of the user specified by RestApi.Controllers.V2022_05_01_preview.Models.WorkloadDeployment.Gmsa.GmsaAuthenticationProperties.DomainAdminUsername."]
    #[serde(rename = "domainAdminPassword", default, skip_serializing_if = "Option::is_none")]
    pub domain_admin_password: Option<String>,
    #[serde(rename = "akvProperties", default, skip_serializing_if = "Option::is_none")]
    pub akv_properties: Option<KeyVaultSecretStoreProperties>,
    #[doc = "Gets Cred Spec Name to be used."]
    #[serde(rename = "gmsaCredSpecName", default, skip_serializing_if = "Option::is_none")]
    pub gmsa_cred_spec_name: Option<String>,
    #[doc = "Gets name of the secret where GMSA secret is stored in the KeyVault."]
    #[serde(rename = "gmsaSecretName", default, skip_serializing_if = "Option::is_none")]
    pub gmsa_secret_name: Option<String>,
}
impl GmsaAuthenticationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod gmsa_authentication_properties {
    use super::*;
    #[doc = "Gets or sets the current state of GMSA configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConfigurationState")]
    pub enum ConfigurationState {
        NotApplicable,
        Pending,
        InProgress,
        Completed,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConfigurationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConfigurationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConfigurationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotApplicable => serializer.serialize_unit_variant("ConfigurationState", 0u32, "NotApplicable"),
                Self::Pending => serializer.serialize_unit_variant("ConfigurationState", 1u32, "Pending"),
                Self::InProgress => serializer.serialize_unit_variant("ConfigurationState", 2u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("ConfigurationState", 3u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("ConfigurationState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
#[doc = "IIS AKS workload deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisaksWorkloadDeployment {
    #[doc = "Class for AKSDeployment Properties."]
    #[serde(rename = "clusterProperties", default, skip_serializing_if = "Option::is_none")]
    pub cluster_properties: Option<AksDeploymentProperties>,
    #[doc = "AKS Deployment Specification."]
    #[serde(rename = "deploymentSpec", default, skip_serializing_if = "Option::is_none")]
    pub deployment_spec: Option<AksDeploymentSpecification>,
    #[doc = "Gets or sets the deployment history."]
    #[serde(
        rename = "deploymentHistory",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deployment_history: Vec<DeployedResourcesProperties>,
    #[doc = "Class for GMSA authentication details to configure Active Directory connectivity."]
    #[serde(rename = "authenticationProperties", default, skip_serializing_if = "Option::is_none")]
    pub authentication_properties: Option<GmsaAuthenticationProperties>,
    #[doc = "Gets or sets the deployment name prefix."]
    #[serde(rename = "deploymentNamePrefix", default, skip_serializing_if = "Option::is_none")]
    pub deployment_name_prefix: Option<String>,
    #[doc = "Class for automation artifact."]
    #[serde(rename = "automationArtifactProperties", default, skip_serializing_if = "Option::is_none")]
    pub automation_artifact_properties: Option<AutomationArtifact>,
    #[doc = "Gets or sets application directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<WebApplicationDirectory>,
    #[doc = "Gets or sets application configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configurations: Vec<WebApplicationConfiguration>,
    #[doc = "Class for container image properties."]
    #[serde(rename = "containerImageProperties", default, skip_serializing_if = "Option::is_none")]
    pub container_image_properties: Option<ContainerImageProperties>,
    #[doc = "Gets or sets the build container images."]
    #[serde(
        rename = "buildContainerImages",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub build_container_images: Vec<ContainerImageProperties>,
    #[doc = "Gets or sets the bindings for the application."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bindings: Vec<Binding>,
    #[doc = "Resource Requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<ResourceRequirements>,
    #[doc = "Resource Requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceRequirements>,
    #[doc = "Gets or sets the target platform managed identity."]
    #[serde(rename = "targetPlatformIdentity", default, skip_serializing_if = "Option::is_none")]
    pub target_platform_identity: Option<String>,
    #[doc = "Class for app insight monitoring properties."]
    #[serde(rename = "monitoringProperties", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_properties: Option<AppInsightMonitoringProperties>,
}
impl IisaksWorkloadDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IIS workload instance model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IisaksWorkloadDeploymentModelCustomProperties {
    #[serde(flatten)]
    pub workload_deployment_model_custom_properties: WorkloadDeploymentModelCustomProperties,
    #[doc = "IIS AKS workload deployment."]
    #[serde(rename = "iisAksWorkloadDeploymentProperties", default, skip_serializing_if = "Option::is_none")]
    pub iis_aks_workload_deployment_properties: Option<IisaksWorkloadDeployment>,
}
impl IisaksWorkloadDeploymentModelCustomProperties {
    pub fn new(workload_deployment_model_custom_properties: WorkloadDeploymentModelCustomProperties) -> Self {
        Self {
            workload_deployment_model_custom_properties,
            iis_aks_workload_deployment_properties: None,
        }
    }
}
#[doc = "IISApplication details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisApplicationDetails {
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the application pool name."]
    #[serde(rename = "applicationPoolName", default, skip_serializing_if = "Option::is_none")]
    pub application_pool_name: Option<String>,
    #[doc = "Gets or sets the managed pipeline mode."]
    #[serde(rename = "managedPipelineMode", default, skip_serializing_if = "Option::is_none")]
    pub managed_pipeline_mode: Option<String>,
    #[doc = "Gets or sets the runtime version."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[doc = "Gets or sets a value indicating whether 32 bit applications are allowed to run on 64 bit."]
    #[serde(rename = "enable32BitApiOnWin64", default, skip_serializing_if = "Option::is_none")]
    pub enable32_bit_api_on_win64: Option<bool>,
    #[doc = "Second level entity for virtual directories."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<DirectoryPath>,
    #[doc = "Gets or sets the list of directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<DirectoryPath>,
}
impl IisApplicationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IIS virtual application details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisVirtualApplicationDetails {
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets a value indicating whether the application corresponds to a directory."]
    #[serde(rename = "isVirtualDirectory", default, skip_serializing_if = "Option::is_none")]
    pub is_virtual_directory: Option<bool>,
    #[doc = "Second level entity for virtual directories."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<DirectoryPath>,
    #[doc = "Gets or sets the list of directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<DirectoryPath>,
}
impl IisVirtualApplicationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IISWeb application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisWebApplication {
    #[doc = "Gets or sets the list of applications for the IIS web site."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub applications: Vec<IisApplicationDetails>,
    #[doc = "Gets or sets the list of application units for the web site."]
    #[serde(
        rename = "virtualApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_applications: Vec<IisVirtualApplicationDetails>,
    #[doc = "IISWeb server."]
    #[serde(rename = "iisWebServer", default, skip_serializing_if = "Option::is_none")]
    pub iis_web_server: Option<IisWebServer>,
    #[doc = "Gets or sets the web application id."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Gets or sets the web application name."]
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[doc = "Gets or sets application scratch path."]
    #[serde(rename = "applicationScratchPath", default, skip_serializing_if = "Option::is_none")]
    pub application_scratch_path: Option<String>,
    #[doc = "Gets or sets the web server id."]
    #[serde(rename = "webServerId", default, skip_serializing_if = "Option::is_none")]
    pub web_server_id: Option<String>,
    #[doc = "Gets or sets the web server name."]
    #[serde(rename = "webServerName", default, skip_serializing_if = "Option::is_none")]
    pub web_server_name: Option<String>,
    #[doc = "Gets or sets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets application directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<WebApplicationDirectory>,
    #[doc = "Gets or sets application configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configurations: Vec<WebApplicationConfiguration>,
    #[doc = "Second level entity for virtual directories."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<DirectoryPath>,
    #[doc = "Gets or sets the bindings for the application."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bindings: Vec<Binding>,
    #[doc = "Framework specific data for a web application."]
    #[serde(rename = "primaryFramework", default, skip_serializing_if = "Option::is_none")]
    pub primary_framework: Option<WebApplicationFramework>,
    #[doc = "Gets or sets the discovered frameworks of application."]
    #[serde(
        rename = "discoveredFrameworks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub discovered_frameworks: Vec<WebApplicationFramework>,
    #[doc = "Resource Requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<ResourceRequirements>,
    #[doc = "Resource Requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceRequirements>,
}
impl IisWebApplication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IISWeb server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisWebServer {
    #[doc = "Gets or sets the web server id."]
    #[serde(rename = "serverId", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[doc = "Gets or sets the web server name."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Gets or sets the server root configuration location."]
    #[serde(rename = "rootConfigurationLocation", default, skip_serializing_if = "Option::is_none")]
    pub root_configuration_location: Option<String>,
    #[doc = "Gets or sets the server version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets or sets the list of machines."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machines: Vec<String>,
    #[doc = "Gets or sets the list of web applications."]
    #[serde(
        rename = "webApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub web_applications: Vec<String>,
    #[doc = "Gets or sets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the server FQDN."]
    #[serde(rename = "serverFqdn", default, skip_serializing_if = "Option::is_none")]
    pub server_fqdn: Option<String>,
    #[doc = "Gets or sets list of ip addresses."]
    #[serde(
        rename = "ipAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_addresses: Vec<String>,
    #[doc = "Gets or sets the run as account id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<OperatingSystemDetails>,
}
impl IisWebServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IIS workload instance model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IisWorkloadInstanceModelCustomProperties {
    #[serde(flatten)]
    pub workload_instance_model_custom_properties: WorkloadInstanceModelCustomProperties,
    #[doc = "IISWeb application."]
    #[serde(rename = "iisWebApplication", default, skip_serializing_if = "Option::is_none")]
    pub iis_web_application: Option<IisWebApplication>,
    #[doc = "Gets or sets the container Id."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "Gets or sets the fileshare name."]
    #[serde(rename = "fileshareName", default, skip_serializing_if = "Option::is_none")]
    pub fileshare_name: Option<String>,
}
impl IisWorkloadInstanceModelCustomProperties {
    pub fn new(workload_instance_model_custom_properties: WorkloadInstanceModelCustomProperties) -> Self {
        Self {
            workload_instance_model_custom_properties,
            iis_web_application: None,
            container_name: None,
            fileshare_name: None,
        }
    }
}
#[doc = "Identity model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityModel {
    #[doc = "Gets or sets the tenant Id of the SPN with which MigrateAgent communicates to service."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Gets or sets the client/application Id of the SPN with which MigrateAgent communicates to\r\nservice."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Gets or sets the object Id of the SPN with which MigrateAgent communicates to service."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Gets or sets the audience of the SPN with which MigrateAgent communicates to service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "Gets or sets the authority of the SPN with which MigrateAgent communicates to service."]
    #[serde(rename = "aadAuthority", default, skip_serializing_if = "Option::is_none")]
    pub aad_authority: Option<String>,
}
impl IdentityModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Inner health error model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerHealthErrorModel {
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretStoreProperties {
    #[serde(flatten)]
    pub secret_store_properties: SecretStoreProperties,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "keyvaultName", default, skip_serializing_if = "Option::is_none")]
    pub keyvault_name: Option<String>,
    #[serde(rename = "managedIdentityProperties", default, skip_serializing_if = "Option::is_none")]
    pub managed_identity_properties: Option<ManagedIdentityProperties>,
}
impl KeyVaultSecretStoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubeSecretStoreProperties {
    #[serde(flatten)]
    pub secret_store_properties: SecretStoreProperties,
}
impl KubeSecretStoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentityProperties {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "managedIdentityName", default, skip_serializing_if = "Option::is_none")]
    pub managed_identity_name: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}
impl ManagedIdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MigrateAgent model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateAgentModel {
    #[doc = "MigrateAgent model properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MigrateAgentModelProperties>,
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
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl MigrateAgentModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MigrateAgent model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateAgentModelCollection {
    #[doc = "Gets or sets the list of agents."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<MigrateAgentModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MigrateAgentModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MigrateAgent model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateAgentModelCustomProperties {
    #[doc = "Gets or sets the instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl MigrateAgentModelCustomProperties {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "MigrateAgent model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateAgentModelProperties {
    #[doc = "Gets or sets the MigrateAgent correlation Id."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Gets or sets the machine Id where MigrateAgent is running."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "Gets or sets the machine name where MigrateAgent is running."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "Identity model."]
    #[serde(rename = "authenticationIdentity", default, skip_serializing_if = "Option::is_none")]
    pub authentication_identity: Option<IdentityModel>,
    #[doc = "Gets or sets a value indicating whether MigrateAgent is responsive."]
    #[serde(rename = "isResponsive", default, skip_serializing_if = "Option::is_none")]
    pub is_responsive: Option<bool>,
    #[doc = "Gets or sets the time when last heartbeat was sent by the MigrateAgent."]
    #[serde(rename = "lastHeartbeat", default, with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the MigrateAgent version."]
    #[serde(rename = "versionNumber", default, skip_serializing_if = "Option::is_none")]
    pub version_number: Option<String>,
    #[doc = "Gets or sets the provisioning state of the MigrateAgent."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<migrate_agent_model_properties::ProvisioningState>,
    #[doc = "Gets or sets the list of health errors."]
    #[serde(
        rename = "healthErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_errors: Vec<HealthErrorModel>,
    #[doc = "MigrateAgent model custom properties."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<MigrateAgentModelCustomProperties>,
}
impl MigrateAgentModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migrate_agent_model_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the MigrateAgent."]
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
#[doc = "Migrate workflow model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateWorkflowModelCustomProperties {
    #[serde(flatten)]
    pub workflow_model_custom_properties: WorkflowModelCustomProperties,
    #[serde(rename = "deployedResourcesProperties", default, skip_serializing_if = "Option::is_none")]
    pub deployed_resources_properties: Option<serde_json::Value>,
}
impl MigrateWorkflowModelCustomProperties {
    pub fn new(workflow_model_custom_properties: WorkflowModelCustomProperties) -> Self {
        Self {
            workflow_model_custom_properties,
            deployed_resources_properties: None,
        }
    }
}
#[doc = "MigrationConfiguration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationConfiguration {
    #[doc = "Gets or sets the storage account resource Id."]
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[doc = "Gets or sets the key vault resource Id."]
    #[serde(rename = "keyVaultResourceId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_resource_id: Option<String>,
    #[doc = "Gets or sets the migration solution resource Id."]
    #[serde(rename = "migrationSolutionResourceId", default, skip_serializing_if = "Option::is_none")]
    pub migration_solution_resource_id: Option<String>,
}
impl MigrationConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ModernizeProject model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernizeProjectModel {
    #[doc = "Gets or sets the location of the modernizeProject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "ModernizeProject properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ModernizeProjectModelProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
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
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ModernizeProjectModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ModernizeProject model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernizeProjectModelCollection {
    #[doc = "Gets or sets the list of modernizeProjects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ModernizeProjectModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ModernizeProjectModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ModernizeProjectModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ModernizeProject properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernizeProjectModelProperties {
    #[doc = "Gets or sets the provisioning state of the ModernizeProject."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<modernize_project_model_properties::ProvisioningState>,
    #[doc = "Gets or sets the service resource Id."]
    #[serde(rename = "serviceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub service_resource_id: Option<String>,
    #[doc = "Gets or sets the service endpoint."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "MigrationConfiguration properties."]
    #[serde(rename = "migrationConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub migration_configuration: Option<MigrationConfiguration>,
}
impl ModernizeProjectModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod modernize_project_model_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the ModernizeProject."]
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
#[doc = "ModernizeProject statistics model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernizeProjectStatisticsModel {
    #[doc = "ModernizeProject statistics properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ModernizeProjectStatisticsModelProperties>,
}
impl ModernizeProjectStatisticsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ModernizeProject statistics properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernizeProjectStatisticsModelProperties {
    #[serde(rename = "workloadDeploymentStatistics", default, skip_serializing_if = "Option::is_none")]
    pub workload_deployment_statistics: Option<serde_json::Value>,
    #[serde(rename = "jobStatistics", default, skip_serializing_if = "Option::is_none")]
    pub job_statistics: Option<serde_json::Value>,
    #[serde(rename = "workloadInstanceStatistics", default, skip_serializing_if = "Option::is_none")]
    pub workload_instance_statistics: Option<serde_json::Value>,
    #[doc = "Gets or sets the list of modernizeProject health errors."]
    #[serde(
        rename = "modernizeProjectErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub modernize_project_errors: Vec<HealthErrorModel>,
}
impl ModernizeProjectStatisticsModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatingSystemDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<operating_system_details::Os>,
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "osArchitecture", default, skip_serializing_if = "Option::is_none")]
    pub os_architecture: Option<String>,
}
impl OperatingSystemDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operating_system_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Os")]
    pub enum Os {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Os {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Os {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Os {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("Os", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("Os", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the operation status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "Gets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the status of the operation. ARM expects the terminal status to be one of\r\nSucceeded/ Failed/ Canceled. All other values imply that the operation is still running."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets the start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Gets the end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "WebApplication port mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortMapping {
    #[doc = "Gets or sets the Internal Port."]
    #[serde(rename = "internalPort", default, skip_serializing_if = "Option::is_none")]
    pub internal_port: Option<i32>,
    #[doc = "Gets or sets the External Port."]
    #[serde(rename = "externalPort", default, skip_serializing_if = "Option::is_none")]
    pub external_port: Option<i32>,
}
impl PortMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_identity::Type>,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Resource Requirements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRequirements {
    #[doc = "Gets or sets the Cpu requirement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[doc = "Gets or sets the Memory requirement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}
impl ResourceRequirements {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretStoreDetails {
    #[serde(rename = "secretStore", default, skip_serializing_if = "Option::is_none")]
    pub secret_store: Option<secret_store_details::SecretStore>,
    #[serde(rename = "secretStoreProperties", default, skip_serializing_if = "Option::is_none")]
    pub secret_store_properties: Option<SecretStoreProperties>,
}
impl SecretStoreDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod secret_store_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecretStore")]
    pub enum SecretStore {
        None,
        KubeSecret,
        KeyVaultSecret,
        AppServiceAppSettings,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecretStore {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecretStore {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecretStore {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("SecretStore", 0u32, "None"),
                Self::KubeSecret => serializer.serialize_unit_variant("SecretStore", 1u32, "KubeSecret"),
                Self::KeyVaultSecret => serializer.serialize_unit_variant("SecretStore", 2u32, "KeyVaultSecret"),
                Self::AppServiceAppSettings => serializer.serialize_unit_variant("SecretStore", 3u32, "AppServiceAppSettings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretStoreProperties {
    #[serde(rename = "secretStoreId", default, skip_serializing_if = "Option::is_none")]
    pub secret_store_id: Option<String>,
    #[serde(rename = "inputType", default, skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
}
impl SecretStoreProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Storage profile for the directory on the target container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetStorageProfile {
    #[doc = "Gets or sets the target storage access type."]
    #[serde(rename = "storageAccessType", default, skip_serializing_if = "Option::is_none")]
    pub storage_access_type: Option<target_storage_profile::StorageAccessType>,
    #[doc = "Gets or sets the target projection type."]
    #[serde(rename = "storageProjectionType", default, skip_serializing_if = "Option::is_none")]
    pub storage_projection_type: Option<target_storage_profile::StorageProjectionType>,
    #[doc = "Gets or sets the storage provider type on the target.\r\nApplicable when StorageProjectionType is not ContainerFileSystem."]
    #[serde(rename = "hydrationStorageProviderType", default, skip_serializing_if = "Option::is_none")]
    pub hydration_storage_provider_type: Option<target_storage_profile::HydrationStorageProviderType>,
    #[doc = "Gets or sets the target persistent volume id.\r\nApplicable when StorageProjectionType is PersistentVolume and on using an\r\nexisting PersistentVolume."]
    #[serde(rename = "persistentVolumeId", default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_id: Option<String>,
    #[doc = "Gets or sets the name of the projected volume on the target environment."]
    #[serde(rename = "targetName", default, skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    #[doc = "Gets or sets the storage size on the target.\r\nApplicable when StorageProjectionType is PersistentVolume and on creating a new\r\nPersistentVolume."]
    #[serde(rename = "targetSize", default, skip_serializing_if = "Option::is_none")]
    pub target_size: Option<String>,
    #[doc = "Azure file share profile for hydration of application folders not mounted on\r\nthe container file system."]
    #[serde(rename = "azureFileShareProfile", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_profile: Option<AzureFileShareHydrationProfile>,
}
impl TargetStorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod target_storage_profile {
    use super::*;
    #[doc = "Gets or sets the target storage access type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAccessType")]
    pub enum StorageAccessType {
        Shared,
        Exclusive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAccessType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAccessType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAccessType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Shared => serializer.serialize_unit_variant("StorageAccessType", 0u32, "Shared"),
                Self::Exclusive => serializer.serialize_unit_variant("StorageAccessType", 1u32, "Exclusive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the target projection type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageProjectionType")]
    pub enum StorageProjectionType {
        ContainerFileSystem,
        PersistentVolume,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageProjectionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageProjectionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageProjectionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ContainerFileSystem => serializer.serialize_unit_variant("StorageProjectionType", 0u32, "ContainerFileSystem"),
                Self::PersistentVolume => serializer.serialize_unit_variant("StorageProjectionType", 1u32, "PersistentVolume"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the storage provider type on the target.\r\nApplicable when StorageProjectionType is not ContainerFileSystem."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HydrationStorageProviderType")]
    pub enum HydrationStorageProviderType {
        AzureFileShare,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HydrationStorageProviderType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HydrationStorageProviderType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HydrationStorageProviderType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureFileShare => serializer.serialize_unit_variant("HydrationStorageProviderType", 0u32, "AzureFileShare"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Task model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskModel {
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskModelCustomProperties {
    #[doc = "Gets or sets the instance type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}
impl TaskModelCustomProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test migrate cleanup workflow model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestMigrateCleanupWorkflowModelCustomProperties {
    #[serde(flatten)]
    pub workflow_model_custom_properties: WorkflowModelCustomProperties,
    #[doc = "Gets or sets the test migrate cleanup comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}
impl TestMigrateCleanupWorkflowModelCustomProperties {
    pub fn new(workflow_model_custom_properties: WorkflowModelCustomProperties) -> Self {
        Self {
            workflow_model_custom_properties,
            comments: None,
        }
    }
}
#[doc = "Test migrate model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestMigrateModel {
    #[doc = "Test migrate model properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestMigrateModelProperties>,
}
impl TestMigrateModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test migrate model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestMigrateModelProperties {
    #[doc = "Workload deployment model custom properties."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<WorkloadDeploymentModelCustomProperties>,
}
impl TestMigrateModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test migrate workflow model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestMigrateWorkflowModelCustomProperties {
    #[serde(flatten)]
    pub workflow_model_custom_properties: WorkflowModelCustomProperties,
    #[serde(rename = "deployedResourcesProperties", default, skip_serializing_if = "Option::is_none")]
    pub deployed_resources_properties: Option<serde_json::Value>,
}
impl TestMigrateWorkflowModelCustomProperties {
    pub fn new(workflow_model_custom_properties: WorkflowModelCustomProperties) -> Self {
        Self {
            workflow_model_custom_properties,
            deployed_resources_properties: None,
        }
    }
}
#[doc = "Update ModernizeProject model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateModernizeProjectModel {
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
}
impl UpdateModernizeProjectModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VMware MigrateAgent model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareMigrateAgentModelCustomProperties {
    #[serde(flatten)]
    pub migrate_agent_model_custom_properties: MigrateAgentModelCustomProperties,
    #[doc = "Gets or sets the master Site Id of the Migrate Agent."]
    #[serde(rename = "vmwareSiteId", default, skip_serializing_if = "Option::is_none")]
    pub vmware_site_id: Option<String>,
    #[doc = "Gets or sets the friendly name of the,of the MigrateAgent fabric."]
    #[serde(rename = "fabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_friendly_name: Option<String>,
}
impl VMwareMigrateAgentModelCustomProperties {
    pub fn new(migrate_agent_model_custom_properties: MigrateAgentModelCustomProperties) -> Self {
        Self {
            migrate_agent_model_custom_properties,
            vmware_site_id: None,
            fabric_friendly_name: None,
        }
    }
}
#[doc = "Class for web application configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationConfiguration {
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the configuration name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the configuration file path."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "Gets or sets the configuration local file path."]
    #[serde(rename = "localFilePath", default, skip_serializing_if = "Option::is_none")]
    pub local_file_path: Option<String>,
    #[doc = "Gets or sets the configuration target file path."]
    #[serde(rename = "targetFilePath", default, skip_serializing_if = "Option::is_none")]
    pub target_file_path: Option<String>,
    #[doc = "Gets or sets the configuration section in the file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[doc = "Gets or sets the configuration type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<web_application_configuration::Type>,
    #[doc = "Gets or sets a value indicating whether the configuration is edited or not by the user."]
    #[serde(rename = "isDeploymentTimeEditable", default, skip_serializing_if = "Option::is_none")]
    pub is_deployment_time_editable: Option<bool>,
    #[doc = "Gets or sets the configuration value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Gets or sets the identifier for the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "secretStoreDetails", default, skip_serializing_if = "Option::is_none")]
    pub secret_store_details: Option<SecretStoreDetails>,
}
impl WebApplicationConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod web_application_configuration {
    use super::*;
    #[doc = "Gets or sets the configuration type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "IISConnectionString")]
        IisConnectionString,
        #[serde(rename = "IISAuthentication")]
        IisAuthentication,
        ApacheTomcatContextResource,
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
                Self::IisConnectionString => serializer.serialize_unit_variant("Type", 0u32, "IISConnectionString"),
                Self::IisAuthentication => serializer.serialize_unit_variant("Type", 1u32, "IISAuthentication"),
                Self::ApacheTomcatContextResource => serializer.serialize_unit_variant("Type", 2u32, "ApacheTomcatContextResource"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "WebApplication directory structure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationDirectory {
    #[doc = "Gets the unique id corresponding to the application directory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets a value indicating whether the directory object is editable.\r\nTrue when the directory is added as an optional directory, false when discovery is done\r\nmanually."]
    #[serde(rename = "isEditable", default, skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
    #[doc = "Gets or sets the paths of the directory on the source machine."]
    #[serde(
        rename = "sourcePaths",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_paths: Vec<String>,
    #[doc = "Gets or sets the size of the directory on the source machine."]
    #[serde(rename = "sourceSize", default, skip_serializing_if = "Option::is_none")]
    pub source_size: Option<String>,
    #[doc = "Storage profile for the directory on the target container."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<TargetStorageProfile>,
}
impl WebApplicationDirectory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Framework specific data for a web application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationFramework {
    #[doc = "Gets the ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets Name of the framework."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets Version of the framework."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl WebApplicationFramework {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workflow model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowModel {
    #[doc = "Workflow model properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowModelProperties>,
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
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl WorkflowModel {
    pub fn new() -> Self {
        Self::default()
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
        self.next_link.clone()
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
    #[doc = "Gets or sets the instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: workflow_model_custom_properties::InstanceType,
    #[doc = "Gets or sets any custom properties of the affected object."]
    #[serde(rename = "affectedObjectDetails", default, skip_serializing_if = "Option::is_none")]
    pub affected_object_details: Option<serde_json::Value>,
}
impl WorkflowModelCustomProperties {
    pub fn new(instance_type: workflow_model_custom_properties::InstanceType) -> Self {
        Self {
            instance_type,
            affected_object_details: None,
        }
    }
}
pub mod workflow_model_custom_properties {
    use super::*;
    #[doc = "Gets or sets the instance type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InstanceType")]
    pub enum InstanceType {
        SampleWorkflow,
        EnableReplicationWorkflow,
        StopReplicationWorkflow,
        BuildContainerImageWorkflow,
        MigrateWorkflow,
        TestMigrateWorkflow,
        TestMigrateCleanupWorkflow,
        CompleteMigrationWorkflow,
        DisableReplicationWorkflow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InstanceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InstanceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InstanceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SampleWorkflow => serializer.serialize_unit_variant("InstanceType", 0u32, "SampleWorkflow"),
                Self::EnableReplicationWorkflow => serializer.serialize_unit_variant("InstanceType", 1u32, "EnableReplicationWorkflow"),
                Self::StopReplicationWorkflow => serializer.serialize_unit_variant("InstanceType", 2u32, "StopReplicationWorkflow"),
                Self::BuildContainerImageWorkflow => serializer.serialize_unit_variant("InstanceType", 3u32, "BuildContainerImageWorkflow"),
                Self::MigrateWorkflow => serializer.serialize_unit_variant("InstanceType", 4u32, "MigrateWorkflow"),
                Self::TestMigrateWorkflow => serializer.serialize_unit_variant("InstanceType", 5u32, "TestMigrateWorkflow"),
                Self::TestMigrateCleanupWorkflow => serializer.serialize_unit_variant("InstanceType", 6u32, "TestMigrateCleanupWorkflow"),
                Self::CompleteMigrationWorkflow => serializer.serialize_unit_variant("InstanceType", 7u32, "CompleteMigrationWorkflow"),
                Self::DisableReplicationWorkflow => serializer.serialize_unit_variant("InstanceType", 8u32, "DisableReplicationWorkflow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workflow model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[doc = "Gets or sets the workload instance provider."]
    #[serde(rename = "workloadInstanceProviderId", default, skip_serializing_if = "Option::is_none")]
    pub workload_instance_provider_id: Option<String>,
    #[doc = "Gets or sets the workload deployment provider."]
    #[serde(rename = "workloadDeploymentProviderId", default, skip_serializing_if = "Option::is_none")]
    pub workload_deployment_provider_id: Option<String>,
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
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<WorkflowModelCustomProperties>,
}
impl WorkflowModelProperties {
    pub fn new() -> Self {
        Self::default()
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
        ModernizeProject,
        MigrateAgent,
        WorkloadInstance,
        WorkloadDeployment,
        ReplicationPostAction,
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
                Self::ModernizeProject => serializer.serialize_unit_variant("ObjectType", 0u32, "ModernizeProject"),
                Self::MigrateAgent => serializer.serialize_unit_variant("ObjectType", 1u32, "MigrateAgent"),
                Self::WorkloadInstance => serializer.serialize_unit_variant("ObjectType", 2u32, "WorkloadInstance"),
                Self::WorkloadDeployment => serializer.serialize_unit_variant("ObjectType", 3u32, "WorkloadDeployment"),
                Self::ReplicationPostAction => serializer.serialize_unit_variant("ObjectType", 4u32, "ReplicationPostAction"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workflow statistics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowStatisticsModel {
    #[doc = "Gets or sets the resource count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Gets or sets the categorized resource counts."]
    #[serde(rename = "categorizedCounts", default, skip_serializing_if = "Option::is_none")]
    pub categorized_counts: Option<serde_json::Value>,
}
impl WorkflowStatisticsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload deployment model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadDeploymentModel {
    #[doc = "Workload deployment model properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadDeploymentModelProperties>,
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
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl WorkloadDeploymentModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload deployment model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadDeploymentModelCollection {
    #[doc = "Gets or sets the list of workload deployments."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkloadDeploymentModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadDeploymentModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadDeploymentModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload deployment model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadDeploymentModelCustomProperties {
    #[doc = "Gets or sets the instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: workload_deployment_model_custom_properties::InstanceType,
}
impl WorkloadDeploymentModelCustomProperties {
    pub fn new(instance_type: workload_deployment_model_custom_properties::InstanceType) -> Self {
        Self { instance_type }
    }
}
pub mod workload_deployment_model_custom_properties {
    use super::*;
    #[doc = "Gets or sets the instance type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InstanceType")]
    pub enum InstanceType {
        #[serde(rename = "IISAKSWorkloadDeployment")]
        IisaksWorkloadDeployment,
        #[serde(rename = "ApacheTomcatAKSWorkloadDeployment")]
        ApacheTomcatAksWorkloadDeployment,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InstanceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InstanceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InstanceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IisaksWorkloadDeployment => serializer.serialize_unit_variant("InstanceType", 0u32, "IISAKSWorkloadDeployment"),
                Self::ApacheTomcatAksWorkloadDeployment => {
                    serializer.serialize_unit_variant("InstanceType", 1u32, "ApacheTomcatAKSWorkloadDeployment")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workload deployment model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadDeploymentModelProperties {
    #[doc = "Gets or sets the provisioning state of the workload deployment."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workload_deployment_model_properties::ProvisioningState>,
    #[doc = "Gets or sets the workload deployment status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<workload_deployment_model_properties::Status>,
    #[doc = "Gets or sets the workload deployment status description."]
    #[serde(rename = "statusDescription", default, skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[doc = "Gets or sets the test migrate state."]
    #[serde(rename = "testMigrationStatus", default, skip_serializing_if = "Option::is_none")]
    pub test_migration_status: Option<workload_deployment_model_properties::TestMigrationStatus>,
    #[doc = "Gets or sets the Test migrate state description."]
    #[serde(rename = "testMigrationStatusDescription", default, skip_serializing_if = "Option::is_none")]
    pub test_migration_status_description: Option<String>,
    #[doc = "Gets or sets the migrate state."]
    #[serde(rename = "migrationStatus", default, skip_serializing_if = "Option::is_none")]
    pub migration_status: Option<workload_deployment_model_properties::MigrationStatus>,
    #[doc = "Gets or sets the migrate state description."]
    #[serde(rename = "migrationStatusDescription", default, skip_serializing_if = "Option::is_none")]
    pub migration_status_description: Option<String>,
    #[doc = "Gets or sets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the deployment target platform."]
    #[serde(rename = "targetPlatform", default, skip_serializing_if = "Option::is_none")]
    pub target_platform: Option<workload_deployment_model_properties::TargetPlatform>,
    #[doc = "Workload instance model properties."]
    #[serde(rename = "workloadInstanceProperties", default, skip_serializing_if = "Option::is_none")]
    pub workload_instance_properties: Option<WorkloadInstanceModelProperties>,
    #[doc = "Gets or sets the workload deployment correlation Id."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Gets or sets the Last successful unplanned migrate time."]
    #[serde(rename = "lastSuccessfulMigrateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_successful_migrate_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the Last successful test migrate time."]
    #[serde(rename = "lastSuccessfulTestMigrateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_successful_test_migrate_time: Option<time::OffsetDateTime>,
    #[serde(rename = "currentJob", default, skip_serializing_if = "Option::is_none")]
    pub current_job: Option<serde_json::Value>,
    #[doc = "Gets or sets the allowed scenarios on the workload deployment."]
    #[serde(
        rename = "allowedOperations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_operations: Vec<String>,
    #[doc = "Gets or sets the list of health errors."]
    #[serde(
        rename = "healthErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_errors: Vec<HealthErrorModel>,
    #[doc = "Workload deployment model custom properties."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<WorkloadDeploymentModelCustomProperties>,
}
impl WorkloadDeploymentModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workload_deployment_model_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the workload deployment."]
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
    #[doc = "Gets or sets the workload deployment status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        None,
        InitialReplication,
        ReplicationFailed,
        ImageBuildPending,
        ImageBuildInProgress,
        ImageBuildFailed,
        TestMigrating,
        CleanUpPending,
        CleanUpInProgress,
        ReadyToMigrate,
        Migrating,
        Migrated,
        MigrationFailed,
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
                Self::None => serializer.serialize_unit_variant("Status", 0u32, "None"),
                Self::InitialReplication => serializer.serialize_unit_variant("Status", 1u32, "InitialReplication"),
                Self::ReplicationFailed => serializer.serialize_unit_variant("Status", 2u32, "ReplicationFailed"),
                Self::ImageBuildPending => serializer.serialize_unit_variant("Status", 3u32, "ImageBuildPending"),
                Self::ImageBuildInProgress => serializer.serialize_unit_variant("Status", 4u32, "ImageBuildInProgress"),
                Self::ImageBuildFailed => serializer.serialize_unit_variant("Status", 5u32, "ImageBuildFailed"),
                Self::TestMigrating => serializer.serialize_unit_variant("Status", 6u32, "TestMigrating"),
                Self::CleanUpPending => serializer.serialize_unit_variant("Status", 7u32, "CleanUpPending"),
                Self::CleanUpInProgress => serializer.serialize_unit_variant("Status", 8u32, "CleanUpInProgress"),
                Self::ReadyToMigrate => serializer.serialize_unit_variant("Status", 9u32, "ReadyToMigrate"),
                Self::Migrating => serializer.serialize_unit_variant("Status", 10u32, "Migrating"),
                Self::Migrated => serializer.serialize_unit_variant("Status", 11u32, "Migrated"),
                Self::MigrationFailed => serializer.serialize_unit_variant("Status", 12u32, "MigrationFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the test migrate state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TestMigrationStatus")]
    pub enum TestMigrationStatus {
        None,
        ImageBuildPending,
        ImageBuildInProgress,
        ImageBuildFailed,
        ReadyToTestMigrate,
        TestMigrationInProgress,
        TestMigrated,
        Failed,
        CleanupInProgress,
        CleanedUp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TestMigrationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TestMigrationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TestMigrationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("TestMigrationStatus", 0u32, "None"),
                Self::ImageBuildPending => serializer.serialize_unit_variant("TestMigrationStatus", 1u32, "ImageBuildPending"),
                Self::ImageBuildInProgress => serializer.serialize_unit_variant("TestMigrationStatus", 2u32, "ImageBuildInProgress"),
                Self::ImageBuildFailed => serializer.serialize_unit_variant("TestMigrationStatus", 3u32, "ImageBuildFailed"),
                Self::ReadyToTestMigrate => serializer.serialize_unit_variant("TestMigrationStatus", 4u32, "ReadyToTestMigrate"),
                Self::TestMigrationInProgress => serializer.serialize_unit_variant("TestMigrationStatus", 5u32, "TestMigrationInProgress"),
                Self::TestMigrated => serializer.serialize_unit_variant("TestMigrationStatus", 6u32, "TestMigrated"),
                Self::Failed => serializer.serialize_unit_variant("TestMigrationStatus", 7u32, "Failed"),
                Self::CleanupInProgress => serializer.serialize_unit_variant("TestMigrationStatus", 8u32, "CleanupInProgress"),
                Self::CleanedUp => serializer.serialize_unit_variant("TestMigrationStatus", 9u32, "CleanedUp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the migrate state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MigrationStatus")]
    pub enum MigrationStatus {
        None,
        ImageBuildPending,
        ImageBuildInProgress,
        ImageBuildFailed,
        ReadyToTestMigrate,
        ReadyToMigrate,
        TestMigrating,
        TestMigrated,
        TestMigrateCleanupInProgress,
        Migrating,
        Migrated,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MigrationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MigrationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MigrationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("MigrationStatus", 0u32, "None"),
                Self::ImageBuildPending => serializer.serialize_unit_variant("MigrationStatus", 1u32, "ImageBuildPending"),
                Self::ImageBuildInProgress => serializer.serialize_unit_variant("MigrationStatus", 2u32, "ImageBuildInProgress"),
                Self::ImageBuildFailed => serializer.serialize_unit_variant("MigrationStatus", 3u32, "ImageBuildFailed"),
                Self::ReadyToTestMigrate => serializer.serialize_unit_variant("MigrationStatus", 4u32, "ReadyToTestMigrate"),
                Self::ReadyToMigrate => serializer.serialize_unit_variant("MigrationStatus", 5u32, "ReadyToMigrate"),
                Self::TestMigrating => serializer.serialize_unit_variant("MigrationStatus", 6u32, "TestMigrating"),
                Self::TestMigrated => serializer.serialize_unit_variant("MigrationStatus", 7u32, "TestMigrated"),
                Self::TestMigrateCleanupInProgress => {
                    serializer.serialize_unit_variant("MigrationStatus", 8u32, "TestMigrateCleanupInProgress")
                }
                Self::Migrating => serializer.serialize_unit_variant("MigrationStatus", 9u32, "Migrating"),
                Self::Migrated => serializer.serialize_unit_variant("MigrationStatus", 10u32, "Migrated"),
                Self::Failed => serializer.serialize_unit_variant("MigrationStatus", 11u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the deployment target platform."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetPlatform")]
    pub enum TargetPlatform {
        AzureKubernetesService,
        AzureAppServiceContainer,
        AzureAppServiceNative,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetPlatform {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetPlatform {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetPlatform {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureKubernetesService => serializer.serialize_unit_variant("TargetPlatform", 0u32, "AzureKubernetesService"),
                Self::AzureAppServiceContainer => serializer.serialize_unit_variant("TargetPlatform", 1u32, "AzureAppServiceContainer"),
                Self::AzureAppServiceNative => serializer.serialize_unit_variant("TargetPlatform", 2u32, "AzureAppServiceNative"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workload deployment statistics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadDeploymentStatisticsModel {
    #[doc = "Gets or sets the list of health errors."]
    #[serde(
        rename = "healthErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_errors: Vec<HealthErrorModel>,
    #[doc = "Gets or sets the resource count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Gets or sets the categorized resource counts."]
    #[serde(rename = "categorizedCounts", default, skip_serializing_if = "Option::is_none")]
    pub categorized_counts: Option<serde_json::Value>,
}
impl WorkloadDeploymentStatisticsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload instance model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadInstanceModel {
    #[doc = "Workload instance model properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadInstanceModelProperties>,
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
    #[doc = "Gets or sets the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl WorkloadInstanceModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload instance model collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadInstanceModelCollection {
    #[doc = "Gets or sets the list of workload instances."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkloadInstanceModel>,
    #[doc = "Gets or sets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadInstanceModelCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkloadInstanceModelCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload instance model custom properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadInstanceModelCustomProperties {
    #[doc = "Gets or sets the instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: workload_instance_model_custom_properties::InstanceType,
    #[doc = "Gets or sets the Web application ARM id."]
    #[serde(rename = "webAppArmId", default, skip_serializing_if = "Option::is_none")]
    pub web_app_arm_id: Option<String>,
    #[doc = "Gets or sets the Web application site name."]
    #[serde(rename = "webAppSiteName", default, skip_serializing_if = "Option::is_none")]
    pub web_app_site_name: Option<String>,
}
impl WorkloadInstanceModelCustomProperties {
    pub fn new(instance_type: workload_instance_model_custom_properties::InstanceType) -> Self {
        Self {
            instance_type,
            web_app_arm_id: None,
            web_app_site_name: None,
        }
    }
}
pub mod workload_instance_model_custom_properties {
    use super::*;
    #[doc = "Gets or sets the instance type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InstanceType")]
    pub enum InstanceType {
        #[serde(rename = "IISWorkload")]
        IisWorkload,
        ApacheTomcatWorkload,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InstanceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InstanceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InstanceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IisWorkload => serializer.serialize_unit_variant("InstanceType", 0u32, "IISWorkload"),
                Self::ApacheTomcatWorkload => serializer.serialize_unit_variant("InstanceType", 1u32, "ApacheTomcatWorkload"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workload instance model properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadInstanceModelProperties {
    #[doc = "Gets or sets the workload instance name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the provisioning state of the workload instance."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workload_instance_model_properties::ProvisioningState>,
    #[doc = "Gets or sets the migrate agent id associated with the workload instance."]
    #[serde(rename = "migrateAgentId", default, skip_serializing_if = "Option::is_none")]
    pub migrate_agent_id: Option<String>,
    #[doc = "Gets or sets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the source platform."]
    #[serde(rename = "sourcePlatform", default, skip_serializing_if = "Option::is_none")]
    pub source_platform: Option<String>,
    #[doc = "Gets or sets the source name."]
    #[serde(rename = "sourceName", default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[doc = "Gets or sets the replication health of the workload instance."]
    #[serde(rename = "replicationHealth", default, skip_serializing_if = "Option::is_none")]
    pub replication_health: Option<workload_instance_model_properties::ReplicationHealth>,
    #[doc = "Gets or sets the replication state of the workload instance."]
    #[serde(rename = "replicationStatus", default, skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<workload_instance_model_properties::ReplicationStatus>,
    #[doc = "Gets or sets the workload replication state description."]
    #[serde(rename = "replicationStatusDescription", default, skip_serializing_if = "Option::is_none")]
    pub replication_status_description: Option<String>,
    #[doc = "Gets or sets the Last successful replication cycle time."]
    #[serde(rename = "lastSuccessfulReplicationCycleTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_successful_replication_cycle_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the list of health errors."]
    #[serde(
        rename = "healthErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_errors: Vec<HealthErrorModel>,
    #[serde(rename = "currentJob", default, skip_serializing_if = "Option::is_none")]
    pub current_job: Option<serde_json::Value>,
    #[doc = "Gets or sets the allowed scenarios on the workload instance."]
    #[serde(
        rename = "allowedOperations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_operations: Vec<String>,
    #[doc = "Gets or Sets the master site name."]
    #[serde(rename = "masterSiteName", default, skip_serializing_if = "Option::is_none")]
    pub master_site_name: Option<String>,
    #[doc = "Workload instance model custom properties."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<WorkloadInstanceModelCustomProperties>,
}
impl WorkloadInstanceModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workload_instance_model_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the workload instance."]
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
    #[doc = "Gets or sets the replication health of the workload instance."]
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
    #[doc = "Gets or sets the replication state of the workload instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationStatus")]
    pub enum ReplicationStatus {
        None,
        Scheduled,
        InitialSync,
        Completed,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ReplicationStatus", 0u32, "None"),
                Self::Scheduled => serializer.serialize_unit_variant("ReplicationStatus", 1u32, "Scheduled"),
                Self::InitialSync => serializer.serialize_unit_variant("ReplicationStatus", 2u32, "InitialSync"),
                Self::Completed => serializer.serialize_unit_variant("ReplicationStatus", 3u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("ReplicationStatus", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workload instance statistics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadInstanceStatisticsModel {
    #[doc = "Gets or sets the list of health errors."]
    #[serde(
        rename = "healthErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_errors: Vec<HealthErrorModel>,
    #[doc = "Gets or sets the resource count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Gets or sets the categorized resource counts."]
    #[serde(rename = "categorizedCounts", default, skip_serializing_if = "Option::is_none")]
    pub categorized_counts: Option<serde_json::Value>,
}
impl WorkloadInstanceStatisticsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadJobProperties {
    #[doc = "Gets or sets workload scenario name."]
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
impl WorkloadJobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
