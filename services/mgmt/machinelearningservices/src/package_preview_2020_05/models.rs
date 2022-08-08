#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AciServiceCreateRequest {
    #[serde(flatten)]
    pub create_service_request: CreateServiceRequest,
    #[doc = "The resource requirements for the container (cpu and memory)."]
    #[serde(rename = "containerResourceRequirements", default, skip_serializing_if = "Option::is_none")]
    pub container_resource_requirements: Option<ContainerResourceRequirements>,
    #[doc = "Whether or not authentication is enabled on the service."]
    #[serde(rename = "authEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auth_enabled: Option<bool>,
    #[doc = "Whether or not SSL is enabled."]
    #[serde(rename = "sslEnabled", default, skip_serializing_if = "Option::is_none")]
    pub ssl_enabled: Option<bool>,
    #[doc = "Whether or not Application Insights is enabled."]
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
    #[doc = "Details of the data collection options specified."]
    #[serde(rename = "dataCollection", default, skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<serde_json::Value>,
    #[doc = "The public SSL certificate in PEM format to use if SSL is enabled."]
    #[serde(rename = "sslCertificate", default, skip_serializing_if = "Option::is_none")]
    pub ssl_certificate: Option<String>,
    #[doc = "The public SSL key in PEM format for the certificate."]
    #[serde(rename = "sslKey", default, skip_serializing_if = "Option::is_none")]
    pub ssl_key: Option<String>,
    #[doc = "The CName for the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    #[doc = "The Dns label for the service."]
    #[serde(rename = "dnsNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub dns_name_label: Option<String>,
    #[doc = "The virtual network configuration."]
    #[serde(rename = "vnetConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub vnet_configuration: Option<serde_json::Value>,
    #[doc = "The encryption properties."]
    #[serde(rename = "encryptionProperties", default, skip_serializing_if = "Option::is_none")]
    pub encryption_properties: Option<serde_json::Value>,
}
impl AciServiceCreateRequest {
    pub fn new(create_service_request: CreateServiceRequest) -> Self {
        Self {
            create_service_request,
            container_resource_requirements: None,
            auth_enabled: None,
            ssl_enabled: None,
            app_insights_enabled: None,
            data_collection: None,
            ssl_certificate: None,
            ssl_key: None,
            cname: None,
            dns_name_label: None,
            vnet_configuration: None,
            encryption_properties: None,
        }
    }
}
#[doc = "The response for an ACI service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AciServiceResponse {
    #[serde(flatten)]
    pub service_response_base: ServiceResponseBase,
    #[doc = "The resource requirements for the container (cpu and memory)."]
    #[serde(rename = "containerResourceRequirements", default, skip_serializing_if = "Option::is_none")]
    pub container_resource_requirements: Option<ContainerResourceRequirements>,
    #[doc = "The Uri for sending scoring requests."]
    #[serde(rename = "scoringUri", default, skip_serializing_if = "Option::is_none")]
    pub scoring_uri: Option<String>,
    #[doc = "The name of the Azure location/region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Whether or not authentication is enabled on the service."]
    #[serde(rename = "authEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auth_enabled: Option<bool>,
    #[doc = "Whether or not SSL is enabled."]
    #[serde(rename = "sslEnabled", default, skip_serializing_if = "Option::is_none")]
    pub ssl_enabled: Option<bool>,
    #[doc = "Whether or not Application Insights is enabled."]
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
    #[doc = "Details of the data collection options specified."]
    #[serde(rename = "dataCollection", default, skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<serde_json::Value>,
    #[doc = "The public SSL certificate in PEM format to use if SSL is enabled."]
    #[serde(rename = "sslCertificate", default, skip_serializing_if = "Option::is_none")]
    pub ssl_certificate: Option<String>,
    #[doc = "The public SSL key in PEM format for the certificate."]
    #[serde(rename = "sslKey", default, skip_serializing_if = "Option::is_none")]
    pub ssl_key: Option<String>,
    #[doc = "The CName for the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    #[doc = "The public IP address for the service."]
    #[serde(rename = "publicIp", default, skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[doc = "The public Fqdn for the service."]
    #[serde(rename = "publicFqdn", default, skip_serializing_if = "Option::is_none")]
    pub public_fqdn: Option<String>,
    #[doc = "The Uri for sending swagger requests."]
    #[serde(rename = "swaggerUri", default, skip_serializing_if = "Option::is_none")]
    pub swagger_uri: Option<String>,
    #[doc = "Details on the models and configurations."]
    #[serde(rename = "modelConfigMap", default, skip_serializing_if = "Option::is_none")]
    pub model_config_map: Option<serde_json::Value>,
    #[doc = "The list of models."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub models: Vec<Model>,
    #[doc = "The Environment, models and assets used for inferencing."]
    #[serde(rename = "environmentImageRequest", default, skip_serializing_if = "Option::is_none")]
    pub environment_image_request: Option<serde_json::Value>,
    #[doc = "The virtual network configuration."]
    #[serde(rename = "vnetConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub vnet_configuration: Option<serde_json::Value>,
    #[doc = "The encryption properties."]
    #[serde(rename = "encryptionProperties", default, skip_serializing_if = "Option::is_none")]
    pub encryption_properties: Option<serde_json::Value>,
}
impl AciServiceResponse {
    pub fn new(service_response_base: ServiceResponseBase) -> Self {
        Self {
            service_response_base,
            container_resource_requirements: None,
            scoring_uri: None,
            location: None,
            auth_enabled: None,
            ssl_enabled: None,
            app_insights_enabled: None,
            data_collection: None,
            ssl_certificate: None,
            ssl_key: None,
            cname: None,
            public_ip: None,
            public_fqdn: None,
            swagger_uri: None,
            model_config_map: None,
            models: Vec::new(),
            environment_image_request: None,
            vnet_configuration: None,
            encryption_properties: None,
        }
    }
}
#[doc = "A Machine Learning compute based on AKS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aks {
    #[serde(flatten)]
    pub compute: Compute,
    #[doc = "AKS properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<aks::Properties>,
}
impl Aks {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod aks {
    use super::*;
    #[doc = "AKS properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Cluster full qualified domain name"]
        #[serde(rename = "clusterFqdn", default, skip_serializing_if = "Option::is_none")]
        pub cluster_fqdn: Option<String>,
        #[doc = "System services"]
        #[serde(rename = "systemServices", default, skip_serializing_if = "Vec::is_empty")]
        pub system_services: Vec<SystemService>,
        #[doc = "Number of agents"]
        #[serde(rename = "agentCount", default, skip_serializing_if = "Option::is_none")]
        pub agent_count: Option<i32>,
        #[doc = "Agent virtual machine size"]
        #[serde(rename = "agentVMSize", default, skip_serializing_if = "Option::is_none")]
        pub agent_vm_size: Option<String>,
        #[doc = "The ssl configuration for scoring"]
        #[serde(rename = "sslConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub ssl_configuration: Option<SslConfiguration>,
        #[doc = "Advance configuration for AKS networking"]
        #[serde(rename = "aksNetworkingConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub aks_networking_configuration: Option<AksNetworkingConfiguration>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AksReplicaStatus {
    #[doc = "The desired number of replicas."]
    #[serde(rename = "desiredReplicas", default, skip_serializing_if = "Option::is_none")]
    pub desired_replicas: Option<i32>,
    #[doc = "The number of updated replicas."]
    #[serde(rename = "updatedReplicas", default, skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
    #[doc = "The number of available replicas."]
    #[serde(rename = "availableReplicas", default, skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}
impl AksReplicaStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request to create an AKS service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AksServiceCreateRequest {
    #[serde(flatten)]
    pub create_endpoint_variant_request: CreateEndpointVariantRequest,
    #[doc = "The number of replicas on the cluster."]
    #[serde(rename = "numReplicas", default, skip_serializing_if = "Option::is_none")]
    pub num_replicas: Option<i32>,
    #[doc = "Details of the data collection options specified."]
    #[serde(rename = "dataCollection", default, skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<serde_json::Value>,
    #[doc = "The name of the compute resource."]
    #[serde(rename = "computeName", default, skip_serializing_if = "Option::is_none")]
    pub compute_name: Option<String>,
    #[doc = "Whether or not Application Insights is enabled."]
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
    #[doc = "The auto scaler properties."]
    #[serde(rename = "autoScaler", default, skip_serializing_if = "Option::is_none")]
    pub auto_scaler: Option<serde_json::Value>,
    #[doc = "The resource requirements for the container (cpu and memory)."]
    #[serde(rename = "containerResourceRequirements", default, skip_serializing_if = "Option::is_none")]
    pub container_resource_requirements: Option<ContainerResourceRequirements>,
    #[doc = "The maximum number of concurrent requests per container."]
    #[serde(rename = "maxConcurrentRequestsPerContainer", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_requests_per_container: Option<i32>,
    #[doc = "Maximum time a request will wait in the queue (in milliseconds). After this time, the service will return 503 (Service Unavailable)"]
    #[serde(rename = "maxQueueWaitMs", default, skip_serializing_if = "Option::is_none")]
    pub max_queue_wait_ms: Option<i32>,
    #[doc = "Kubernetes namespace for the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The scoring timeout in milliseconds."]
    #[serde(rename = "scoringTimeoutMs", default, skip_serializing_if = "Option::is_none")]
    pub scoring_timeout_ms: Option<i32>,
    #[doc = "Whether or not authentication is enabled."]
    #[serde(rename = "authEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auth_enabled: Option<bool>,
    #[doc = "The liveness probe requirements."]
    #[serde(rename = "livenessProbeRequirements", default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe_requirements: Option<serde_json::Value>,
    #[doc = "Whether or not AAD authentication is enabled."]
    #[serde(rename = "aadAuthEnabled", default, skip_serializing_if = "Option::is_none")]
    pub aad_auth_enabled: Option<bool>,
}
impl AksServiceCreateRequest {
    pub fn new(create_endpoint_variant_request: CreateEndpointVariantRequest) -> Self {
        Self {
            create_endpoint_variant_request,
            num_replicas: None,
            data_collection: None,
            compute_name: None,
            app_insights_enabled: None,
            auto_scaler: None,
            container_resource_requirements: None,
            max_concurrent_requests_per_container: None,
            max_queue_wait_ms: None,
            namespace: None,
            scoring_timeout_ms: None,
            auth_enabled: None,
            liveness_probe_requirements: None,
            aad_auth_enabled: None,
        }
    }
}
#[doc = "The response for an AKS service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AksServiceResponse {
    #[serde(flatten)]
    pub aks_variant_response: AksVariantResponse,
    #[doc = "The list of models."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub models: Vec<Model>,
    #[doc = "The resource requirements for the container (cpu and memory)."]
    #[serde(rename = "containerResourceRequirements", default, skip_serializing_if = "Option::is_none")]
    pub container_resource_requirements: Option<ContainerResourceRequirements>,
    #[doc = "The maximum number of concurrent requests per container."]
    #[serde(rename = "maxConcurrentRequestsPerContainer", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_requests_per_container: Option<i32>,
    #[doc = "Maximum time a request will wait in the queue (in milliseconds). After this time, the service will return 503 (Service Unavailable)"]
    #[serde(rename = "maxQueueWaitMs", default, skip_serializing_if = "Option::is_none")]
    pub max_queue_wait_ms: Option<i32>,
    #[doc = "The name of the compute resource."]
    #[serde(rename = "computeName", default, skip_serializing_if = "Option::is_none")]
    pub compute_name: Option<String>,
    #[doc = "The Kubernetes namespace of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The number of replicas on the cluster."]
    #[serde(rename = "numReplicas", default, skip_serializing_if = "Option::is_none")]
    pub num_replicas: Option<i32>,
    #[doc = "Details of the data collection options specified."]
    #[serde(rename = "dataCollection", default, skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<serde_json::Value>,
    #[doc = "Whether or not Application Insights is enabled."]
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
    #[doc = "The auto scaler properties."]
    #[serde(rename = "autoScaler", default, skip_serializing_if = "Option::is_none")]
    pub auto_scaler: Option<serde_json::Value>,
    #[doc = "The Uri for sending scoring requests."]
    #[serde(rename = "scoringUri", default, skip_serializing_if = "Option::is_none")]
    pub scoring_uri: Option<String>,
    #[doc = "The deployment status."]
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<serde_json::Value>,
    #[doc = "The scoring timeout in milliseconds."]
    #[serde(rename = "scoringTimeoutMs", default, skip_serializing_if = "Option::is_none")]
    pub scoring_timeout_ms: Option<i32>,
    #[doc = "The liveness probe requirements."]
    #[serde(rename = "livenessProbeRequirements", default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe_requirements: Option<serde_json::Value>,
    #[doc = "Whether or not authentication is enabled."]
    #[serde(rename = "authEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auth_enabled: Option<bool>,
    #[doc = "Whether or not AAD authentication is enabled."]
    #[serde(rename = "aadAuthEnabled", default, skip_serializing_if = "Option::is_none")]
    pub aad_auth_enabled: Option<bool>,
    #[doc = "The Uri for sending swagger requests."]
    #[serde(rename = "swaggerUri", default, skip_serializing_if = "Option::is_none")]
    pub swagger_uri: Option<String>,
    #[doc = "Details on the models and configurations."]
    #[serde(rename = "modelConfigMap", default, skip_serializing_if = "Option::is_none")]
    pub model_config_map: Option<serde_json::Value>,
    #[doc = "The Environment, models and assets used for inferencing."]
    #[serde(rename = "environmentImageRequest", default, skip_serializing_if = "Option::is_none")]
    pub environment_image_request: Option<serde_json::Value>,
}
impl AksServiceResponse {
    pub fn new(aks_variant_response: AksVariantResponse) -> Self {
        Self {
            aks_variant_response,
            models: Vec::new(),
            container_resource_requirements: None,
            max_concurrent_requests_per_container: None,
            max_queue_wait_ms: None,
            compute_name: None,
            namespace: None,
            num_replicas: None,
            data_collection: None,
            app_insights_enabled: None,
            auto_scaler: None,
            scoring_uri: None,
            deployment_status: None,
            scoring_timeout_ms: None,
            liveness_probe_requirements: None,
            auth_enabled: None,
            aad_auth_enabled: None,
            swagger_uri: None,
            model_config_map: None,
            environment_image_request: None,
        }
    }
}
#[doc = "The response for an AKS variant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AksVariantResponse {
    #[serde(flatten)]
    pub service_response_base: ServiceResponseBase,
    #[doc = "Is this the default variant."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "The amount of traffic variant receives."]
    #[serde(rename = "trafficPercentile", default, skip_serializing_if = "Option::is_none")]
    pub traffic_percentile: Option<f32>,
    #[doc = "The type of the variant."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<aks_variant_response::Type>,
}
impl AksVariantResponse {
    pub fn new(service_response_base: ServiceResponseBase) -> Self {
        Self {
            service_response_base,
            is_default: None,
            traffic_percentile: None,
            type_: None,
        }
    }
}
pub mod aks_variant_response {
    use super::*;
    #[doc = "The type of the variant."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Control,
        Treatment,
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
                Self::Control => serializer.serialize_unit_variant("Type", 0u32, "Control"),
                Self::Treatment => serializer.serialize_unit_variant("Type", 1u32, "Treatment"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Secrets related to a Machine Learning compute based on AKS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AksComputeSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[doc = "Content of kubeconfig file that can be used to connect to the Kubernetes cluster."]
    #[serde(rename = "userKubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub user_kube_config: Option<String>,
    #[doc = "Content of kubeconfig file that can be used to connect to the Kubernetes cluster."]
    #[serde(rename = "adminKubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub admin_kube_config: Option<String>,
    #[doc = "Image registry pull secret."]
    #[serde(rename = "imagePullSecretName", default, skip_serializing_if = "Option::is_none")]
    pub image_pull_secret_name: Option<String>,
}
impl AksComputeSecrets {
    pub fn new(compute_secrets: ComputeSecrets) -> Self {
        Self {
            compute_secrets,
            user_kube_config: None,
            admin_kube_config: None,
            image_pull_secret_name: None,
        }
    }
}
#[doc = "Advance configuration for AKS networking"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AksNetworkingConfiguration {
    #[doc = "Virtual network subnet resource ID the compute nodes belong to"]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "A CIDR notation IP range from which to assign service cluster IPs. It must not overlap with any Subnet IP ranges."]
    #[serde(rename = "serviceCidr", default, skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
    #[doc = "An IP address assigned to the Kubernetes DNS service. It must be within the Kubernetes service address range specified in serviceCidr."]
    #[serde(rename = "dnsServiceIP", default, skip_serializing_if = "Option::is_none")]
    pub dns_service_ip: Option<String>,
    #[doc = "A CIDR notation IP range assigned to the Docker bridge network. It must not overlap with any Subnet IP ranges or the Kubernetes service address range."]
    #[serde(rename = "dockerBridgeCidr", default, skip_serializing_if = "Option::is_none")]
    pub docker_bridge_cidr: Option<String>,
}
impl AksNetworkingConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Machine Learning compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlCompute {
    #[serde(flatten)]
    pub compute: Compute,
    #[doc = "AML Compute properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<aml_compute::Properties>,
}
impl AmlCompute {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod aml_compute {
    use super::*;
    #[doc = "AML Compute properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Compute OS Type"]
        #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
        pub os_type: Option<properties::OsType>,
        #[doc = "Virtual Machine Size"]
        #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
        pub vm_size: Option<String>,
        #[doc = "Virtual Machine priority"]
        #[serde(rename = "vmPriority", default, skip_serializing_if = "Option::is_none")]
        pub vm_priority: Option<properties::VmPriority>,
        #[doc = "Virtual Machine image for Windows AML Compute"]
        #[serde(rename = "virtualMachineImage", default, skip_serializing_if = "Option::is_none")]
        pub virtual_machine_image: Option<VirtualMachineImage>,
        #[doc = "Network is isolated or not"]
        #[serde(rename = "isolatedNetwork", default, skip_serializing_if = "Option::is_none")]
        pub isolated_network: Option<bool>,
        #[doc = "scale settings for AML Compute"]
        #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
        pub scale_settings: Option<ScaleSettings>,
        #[doc = "Settings for user account that gets created on each on the nodes of a compute."]
        #[serde(rename = "userAccountCredentials", default, skip_serializing_if = "Option::is_none")]
        pub user_account_credentials: Option<UserAccountCredentials>,
        #[doc = "Represents a resource ID. For example, for a subnet, it is the resource URL for the subnet."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subnet: Option<ResourceId>,
        #[doc = "State of the public SSH port. Possible values are: Disabled - Indicates that the public ssh port is closed on all nodes of the cluster. Enabled - Indicates that the public ssh port is open on all nodes of the cluster. NotSpecified - Indicates that the public ssh port is closed on all nodes of the cluster if VNet is defined, else is open all public nodes. It can be default only during cluster creation time, after creation it will be either enabled or disabled."]
        #[serde(rename = "remoteLoginPortPublicAccess", default, skip_serializing_if = "Option::is_none")]
        pub remote_login_port_public_access: Option<properties::RemoteLoginPortPublicAccess>,
        #[doc = "Allocation state of the compute. Possible values are: steady - Indicates that the compute is not resizing. There are no changes to the number of compute nodes in the compute in progress. A compute enters this state when it is created and when no operations are being performed on the compute to change the number of compute nodes. resizing - Indicates that the compute is resizing; that is, compute nodes are being added to or removed from the compute."]
        #[serde(rename = "allocationState", default, skip_serializing_if = "Option::is_none")]
        pub allocation_state: Option<properties::AllocationState>,
        #[doc = "The time at which the compute entered its current allocation state."]
        #[serde(rename = "allocationStateTransitionTime", with = "azure_core::date::rfc3339::option")]
        pub allocation_state_transition_time: Option<time::OffsetDateTime>,
        #[doc = "Collection of errors encountered by various compute nodes during node setup."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub errors: Vec<MachineLearningServiceError>,
        #[doc = "The number of compute nodes currently assigned to the compute."]
        #[serde(rename = "currentNodeCount", default, skip_serializing_if = "Option::is_none")]
        pub current_node_count: Option<i32>,
        #[doc = "The target number of compute nodes for the compute. If the allocationState is resizing, this property denotes the target node count for the ongoing resize operation. If the allocationState is steady, this property denotes the target node count for the previous resize operation."]
        #[serde(rename = "targetNodeCount", default, skip_serializing_if = "Option::is_none")]
        pub target_node_count: Option<i32>,
        #[doc = "Counts of various compute node states on the amlCompute."]
        #[serde(rename = "nodeStateCounts", default, skip_serializing_if = "Option::is_none")]
        pub node_state_counts: Option<NodeStateCounts>,
        #[doc = "Enable or disable node public IP address provisioning. Possible values are: Possible values are: true - Indicates that the compute nodes will have public IPs provisioned. false - Indicates that the compute nodes will have a private endpoint and no public IPs."]
        #[serde(rename = "enableNodePublicIp", default, skip_serializing_if = "Option::is_none")]
        pub enable_node_public_ip: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Compute OS Type"]
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
        #[doc = "Virtual Machine priority"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "VmPriority")]
        pub enum VmPriority {
            Dedicated,
            LowPriority,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for VmPriority {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for VmPriority {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for VmPriority {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Dedicated => serializer.serialize_unit_variant("VmPriority", 0u32, "Dedicated"),
                    Self::LowPriority => serializer.serialize_unit_variant("VmPriority", 1u32, "LowPriority"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "State of the public SSH port. Possible values are: Disabled - Indicates that the public ssh port is closed on all nodes of the cluster. Enabled - Indicates that the public ssh port is open on all nodes of the cluster. NotSpecified - Indicates that the public ssh port is closed on all nodes of the cluster if VNet is defined, else is open all public nodes. It can be default only during cluster creation time, after creation it will be either enabled or disabled."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "RemoteLoginPortPublicAccess")]
        pub enum RemoteLoginPortPublicAccess {
            Enabled,
            Disabled,
            NotSpecified,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for RemoteLoginPortPublicAccess {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for RemoteLoginPortPublicAccess {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for RemoteLoginPortPublicAccess {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Enabled => serializer.serialize_unit_variant("RemoteLoginPortPublicAccess", 0u32, "Enabled"),
                    Self::Disabled => serializer.serialize_unit_variant("RemoteLoginPortPublicAccess", 1u32, "Disabled"),
                    Self::NotSpecified => serializer.serialize_unit_variant("RemoteLoginPortPublicAccess", 2u32, "NotSpecified"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        impl Default for RemoteLoginPortPublicAccess {
            fn default() -> Self {
                Self::NotSpecified
            }
        }
        #[doc = "Allocation state of the compute. Possible values are: steady - Indicates that the compute is not resizing. There are no changes to the number of compute nodes in the compute in progress. A compute enters this state when it is created and when no operations are being performed on the compute to change the number of compute nodes. resizing - Indicates that the compute is resizing; that is, compute nodes are being added to or removed from the compute."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "AllocationState")]
        pub enum AllocationState {
            Steady,
            Resizing,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for AllocationState {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for AllocationState {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for AllocationState {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Steady => serializer.serialize_unit_variant("AllocationState", 0u32, "Steady"),
                    Self::Resizing => serializer.serialize_unit_variant("AllocationState", 1u32, "Resizing"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Compute node information related to a AmlCompute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmlComputeNodeInformation {
    #[doc = "ID of the compute node."]
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "Private IP address of the compute node."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Public IP address of the compute node."]
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[doc = "SSH port number of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<f64>,
    #[doc = "State of the compute node. Values are idle, running, preparing, unusable, leaving and preempted."]
    #[serde(rename = "nodeState", default, skip_serializing_if = "Option::is_none")]
    pub node_state: Option<aml_compute_node_information::NodeState>,
    #[doc = "ID of the Experiment running on the node, if any else null."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}
impl AmlComputeNodeInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aml_compute_node_information {
    use super::*;
    #[doc = "State of the compute node. Values are idle, running, preparing, unusable, leaving and preempted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NodeState")]
    pub enum NodeState {
        #[serde(rename = "idle")]
        Idle,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "preparing")]
        Preparing,
        #[serde(rename = "unusable")]
        Unusable,
        #[serde(rename = "leaving")]
        Leaving,
        #[serde(rename = "preempted")]
        Preempted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NodeState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NodeState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NodeState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Idle => serializer.serialize_unit_variant("NodeState", 0u32, "idle"),
                Self::Running => serializer.serialize_unit_variant("NodeState", 1u32, "running"),
                Self::Preparing => serializer.serialize_unit_variant("NodeState", 2u32, "preparing"),
                Self::Unusable => serializer.serialize_unit_variant("NodeState", 3u32, "unusable"),
                Self::Leaving => serializer.serialize_unit_variant("NodeState", 4u32, "leaving"),
                Self::Preempted => serializer.serialize_unit_variant("NodeState", 5u32, "preempted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Compute node information related to a AmlCompute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmlComputeNodesInformation {
    #[serde(flatten)]
    pub compute_nodes_information: ComputeNodesInformation,
    #[doc = "The collection of returned AmlCompute nodes details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<AmlComputeNodeInformation>,
}
impl AmlComputeNodesInformation {
    pub fn new(compute_nodes_information: ComputeNodesInformation) -> Self {
        Self {
            compute_nodes_information,
            nodes: Vec::new(),
        }
    }
}
#[doc = "Features enabled for a workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmlUserFeature {
    #[doc = "Specifies the feature ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the feature name "]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Describes the feature for user experience"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AmlUserFeature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthKeys {
    #[doc = "The primary key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary key."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl AuthKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Auto Scaler properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoScaler {
    #[doc = "Option to enable/disable auto scaling."]
    #[serde(rename = "autoscaleEnabled", default, skip_serializing_if = "Option::is_none")]
    pub autoscale_enabled: Option<bool>,
    #[doc = "The minimum number of replicas to scale down to."]
    #[serde(rename = "minReplicas", default, skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,
    #[doc = "The maximum number of replicas in the cluster."]
    #[serde(rename = "maxReplicas", default, skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i32>,
    #[doc = "The target utilization percentage to use for determining whether to scale the cluster."]
    #[serde(rename = "targetUtilization", default, skip_serializing_if = "Option::is_none")]
    pub target_utilization: Option<i32>,
    #[doc = "The amount of seconds to wait between auto scale updates."]
    #[serde(rename = "refreshPeriodInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub refresh_period_in_seconds: Option<i32>,
}
impl AutoScaler {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AmlCompute update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdateParameters {
    #[doc = "The properties of a amlCompute that need to be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterUpdateProperties>,
}
impl ClusterUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a amlCompute that need to be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdateProperties {
    #[doc = "scale settings for AML Compute"]
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<ScaleSettings>,
}
impl ClusterUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Machine Learning compute object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compute {
    #[doc = "The type of compute"]
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
    #[doc = "Location for the underlying compute"]
    #[serde(rename = "computeLocation", default, skip_serializing_if = "Option::is_none")]
    pub compute_location: Option<String>,
    #[doc = "The provision state of the cluster. Valid values are Unknown, Updating, Provisioning, Succeeded, and Failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<compute::ProvisioningState>,
    #[doc = "The description of the Machine Learning compute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The date and time when the compute was created."]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The date and time when the compute was last modified."]
    #[serde(rename = "modifiedOn", with = "azure_core::date::rfc3339::option")]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "ARM resource id of the underlying compute"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Errors during provisioning"]
    #[serde(rename = "provisioningErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub provisioning_errors: Vec<MachineLearningServiceError>,
    #[doc = "Indicating whether the compute was provisioned by user and brought from outside if true, or machine learning service provisioned it if false."]
    #[serde(rename = "isAttachedCompute", default, skip_serializing_if = "Option::is_none")]
    pub is_attached_compute: Option<bool>,
}
impl Compute {
    pub fn new(compute_type: ComputeType) -> Self {
        Self {
            compute_type,
            compute_location: None,
            provisioning_state: None,
            description: None,
            created_on: None,
            modified_on: None,
            resource_id: None,
            provisioning_errors: Vec::new(),
            is_attached_compute: None,
        }
    }
}
pub mod compute {
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
}
#[doc = "Compute nodes information related to a Machine Learning compute. Might differ for every type of compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeNodesInformation {
    #[doc = "The type of compute"]
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ComputeNodesInformation {
    pub fn new(compute_type: ComputeType) -> Self {
        Self {
            compute_type,
            next_link: None,
        }
    }
}
#[doc = "Machine Learning compute object wrapped into ARM resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Machine Learning compute object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Compute>,
}
impl ComputeResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Secrets related to a Machine Learning compute. Might differ for every type of compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeSecrets {
    #[doc = "The type of compute"]
    #[serde(rename = "computeType")]
    pub compute_type: ComputeType,
}
impl ComputeSecrets {
    pub fn new(compute_type: ComputeType) -> Self {
        Self { compute_type }
    }
}
#[doc = "The type of compute"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ComputeType")]
pub enum ComputeType {
    #[serde(rename = "AKS")]
    Aks,
    AmlCompute,
    DataFactory,
    VirtualMachine,
    #[serde(rename = "HDInsight")]
    HdInsight,
    Databricks,
    DataLakeAnalytics,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ComputeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ComputeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ComputeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Aks => serializer.serialize_unit_variant("ComputeType", 0u32, "AKS"),
            Self::AmlCompute => serializer.serialize_unit_variant("ComputeType", 1u32, "AmlCompute"),
            Self::DataFactory => serializer.serialize_unit_variant("ComputeType", 2u32, "DataFactory"),
            Self::VirtualMachine => serializer.serialize_unit_variant("ComputeType", 3u32, "VirtualMachine"),
            Self::HdInsight => serializer.serialize_unit_variant("ComputeType", 4u32, "HDInsight"),
            Self::Databricks => serializer.serialize_unit_variant("ComputeType", 5u32, "Databricks"),
            Self::DataLakeAnalytics => serializer.serialize_unit_variant("ComputeType", 6u32, "DataLakeAnalytics"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ContainerRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}
impl ContainerRegistryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource requirements for the container (cpu and memory)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerResourceRequirements {
    #[doc = "The number of CPU cores on the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    #[doc = "The amount of memory on the container in GB."]
    #[serde(rename = "memoryInGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_gb: Option<f64>,
    #[doc = "The number of GPU cores in the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpu: Option<i32>,
    #[doc = "The number of FPGA PCIE devices exposed to the container. Must be multiple of 2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fpga: Option<i32>,
}
impl ContainerResourceRequirements {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Variant properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEndpointVariantRequest {
    #[serde(flatten)]
    pub create_service_request: CreateServiceRequest,
    #[doc = "Is this the default variant."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "The amount of traffic variant receives."]
    #[serde(rename = "trafficPercentile", default, skip_serializing_if = "Option::is_none")]
    pub traffic_percentile: Option<f32>,
    #[doc = "The type of the variant."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<create_endpoint_variant_request::Type>,
}
impl CreateEndpointVariantRequest {
    pub fn new(create_service_request: CreateServiceRequest) -> Self {
        Self {
            create_service_request,
            is_default: None,
            traffic_percentile: None,
            type_: None,
        }
    }
}
pub mod create_endpoint_variant_request {
    use super::*;
    #[doc = "The type of the variant."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Control,
        Treatment,
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
                Self::Control => serializer.serialize_unit_variant("Type", 0u32, "Control"),
                Self::Treatment => serializer.serialize_unit_variant("Type", 1u32, "Treatment"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The base class for creating a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateServiceRequest {
    #[doc = "The description of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The service tag dictionary. Tags are mutable."]
    #[serde(rename = "kvTags", default, skip_serializing_if = "Option::is_none")]
    pub kv_tags: Option<serde_json::Value>,
    #[doc = "The service properties dictionary. Properties are immutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The authentication keys."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<serde_json::Value>,
    #[doc = "The compute environment type for the service."]
    #[serde(rename = "computeType")]
    pub compute_type: create_service_request::ComputeType,
    #[doc = "The Environment, models and assets needed for inferencing."]
    #[serde(rename = "environmentImageRequest", default, skip_serializing_if = "Option::is_none")]
    pub environment_image_request: Option<serde_json::Value>,
    #[doc = "The name of the Azure location/region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl CreateServiceRequest {
    pub fn new(compute_type: create_service_request::ComputeType) -> Self {
        Self {
            description: None,
            kv_tags: None,
            properties: None,
            keys: None,
            compute_type,
            environment_image_request: None,
            location: None,
        }
    }
}
pub mod create_service_request {
    use super::*;
    #[doc = "The compute environment type for the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComputeType")]
    pub enum ComputeType {
        #[serde(rename = "ACI")]
        Aci,
        #[serde(rename = "AKS")]
        Aks,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComputeType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComputeType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComputeType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Aci => serializer.serialize_unit_variant("ComputeType", 0u32, "ACI"),
                Self::Aks => serializer.serialize_unit_variant("ComputeType", 1u32, "AKS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A DataFactory compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataFactory {
    #[serde(flatten)]
    pub compute: Compute,
}
impl DataFactory {
    pub fn new(compute: Compute) -> Self {
        Self { compute }
    }
}
#[doc = "A DataLakeAnalytics compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalytics {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<data_lake_analytics::Properties>,
}
impl DataLakeAnalytics {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod data_lake_analytics {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "DataLake Store Account Name"]
        #[serde(rename = "dataLakeStoreAccountName", default, skip_serializing_if = "Option::is_none")]
        pub data_lake_store_account_name: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A DataFactory compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Databricks {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<databricks::Properties>,
}
impl Databricks {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod databricks {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Databricks access token"]
        #[serde(rename = "databricksAccessToken", default, skip_serializing_if = "Option::is_none")]
        pub databricks_access_token: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Secrets related to a Machine Learning compute based on Databricks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabricksComputeSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[doc = "access token for databricks account."]
    #[serde(rename = "databricksAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub databricks_access_token: Option<String>,
}
impl DatabricksComputeSecrets {
    pub fn new(compute_secrets: ComputeSecrets) -> Self {
        Self {
            compute_secrets,
            databricks_access_token: None,
        }
    }
}
#[doc = "The dataset reference object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatasetReference {
    #[doc = "The name of the dataset reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The id of the dataset reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl DatasetReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionProperties {
    #[doc = "vault base Url"]
    #[serde(rename = "vaultBaseUrl")]
    pub vault_base_url: String,
    #[doc = "Encryption Key name"]
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[doc = "Encryption Key Version"]
    #[serde(rename = "keyVersion")]
    pub key_version: String,
}
impl EncryptionProperties {
    pub fn new(vault_base_url: String, key_name: String, key_version: String) -> Self {
        Self {
            vault_base_url,
            key_name,
            key_version,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionProperty {
    #[doc = "Indicates whether or not the encryption is enabled for the workspace."]
    pub status: encryption_property::Status,
    #[serde(rename = "keyVaultProperties")]
    pub key_vault_properties: KeyVaultProperties,
}
impl EncryptionProperty {
    pub fn new(status: encryption_property::Status, key_vault_properties: KeyVaultProperties) -> Self {
        Self {
            status,
            key_vault_properties,
        }
    }
}
pub mod encryption_property {
    use super::*;
    #[doc = "Indicates whether or not the encryption is enabled for the workspace."]
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
}
#[doc = "Request to create a Docker image based on Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentImageRequest {
    #[doc = "The name of the driver file."]
    #[serde(rename = "driverProgram", default, skip_serializing_if = "Option::is_none")]
    pub driver_program: Option<String>,
    #[doc = "The list of assets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<ImageAsset>,
    #[doc = "The list of model Ids."]
    #[serde(rename = "modelIds", default, skip_serializing_if = "Vec::is_empty")]
    pub model_ids: Vec<String>,
    #[doc = "The list of models."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub models: Vec<Model>,
    #[doc = "The details of the AZURE ML environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
    #[doc = "The unique identifying details of the AZURE ML environment."]
    #[serde(rename = "environmentReference", default, skip_serializing_if = "Option::is_none")]
    pub environment_reference: Option<serde_json::Value>,
}
impl EnvironmentImageRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to create a Docker image based on Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentImageResponse {
    #[doc = "The name of the driver file."]
    #[serde(rename = "driverProgram", default, skip_serializing_if = "Option::is_none")]
    pub driver_program: Option<String>,
    #[doc = "The list of assets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<ImageAsset>,
    #[doc = "The list of model Ids."]
    #[serde(rename = "modelIds", default, skip_serializing_if = "Vec::is_empty")]
    pub model_ids: Vec<String>,
    #[doc = "The list of models."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub models: Vec<Model>,
    #[doc = "The details of the AZURE ML environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
    #[doc = "The unique identifying details of the AZURE ML environment."]
    #[serde(rename = "environmentReference", default, skip_serializing_if = "Option::is_none")]
    pub environment_reference: Option<serde_json::Value>,
}
impl EnvironmentImageResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentReference {
    #[doc = "Name of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl EnvironmentReference {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "An array of error detail objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The estimated price info for using a VM of a particular OS type, tier, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EstimatedVmPrice {
    #[doc = "The price charged for using the VM."]
    #[serde(rename = "retailPrice")]
    pub retail_price: f64,
    #[doc = "Operating system type used by the VM."]
    #[serde(rename = "osType")]
    pub os_type: estimated_vm_price::OsType,
    #[doc = "The type of the VM."]
    #[serde(rename = "vmTier")]
    pub vm_tier: estimated_vm_price::VmTier,
}
impl EstimatedVmPrice {
    pub fn new(retail_price: f64, os_type: estimated_vm_price::OsType, vm_tier: estimated_vm_price::VmTier) -> Self {
        Self {
            retail_price,
            os_type,
            vm_tier,
        }
    }
}
pub mod estimated_vm_price {
    use super::*;
    #[doc = "Operating system type used by the VM."]
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
    #[doc = "The type of the VM."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VmTier")]
    pub enum VmTier {
        Standard,
        LowPriority,
        Spot,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VmTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VmTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VmTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("VmTier", 0u32, "Standard"),
                Self::LowPriority => serializer.serialize_unit_variant("VmTier", 1u32, "LowPriority"),
                Self::Spot => serializer.serialize_unit_variant("VmTier", 2u32, "Spot"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The estimated price info for using a VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EstimatedVmPrices {
    #[doc = "Three lettered code specifying the currency of the VM price. Example: USD"]
    #[serde(rename = "billingCurrency")]
    pub billing_currency: estimated_vm_prices::BillingCurrency,
    #[doc = "The unit of time measurement for the specified VM price. Example: OneHour"]
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: estimated_vm_prices::UnitOfMeasure,
    #[doc = "The list of estimated prices for using a VM of a particular OS type, tier, etc."]
    pub values: Vec<EstimatedVmPrice>,
}
impl EstimatedVmPrices {
    pub fn new(
        billing_currency: estimated_vm_prices::BillingCurrency,
        unit_of_measure: estimated_vm_prices::UnitOfMeasure,
        values: Vec<EstimatedVmPrice>,
    ) -> Self {
        Self {
            billing_currency,
            unit_of_measure,
            values,
        }
    }
}
pub mod estimated_vm_prices {
    use super::*;
    #[doc = "Three lettered code specifying the currency of the VM price. Example: USD"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingCurrency")]
    pub enum BillingCurrency {
        #[serde(rename = "USD")]
        Usd,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingCurrency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingCurrency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingCurrency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Usd => serializer.serialize_unit_variant("BillingCurrency", 0u32, "USD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The unit of time measurement for the specified VM price. Example: OneHour"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UnitOfMeasure")]
    pub enum UnitOfMeasure {
        OneHour,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UnitOfMeasure {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UnitOfMeasure {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UnitOfMeasure {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OneHour => serializer.serialize_unit_variant("UnitOfMeasure", 0u32, "OneHour"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A HDInsight compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HdInsight {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<hd_insight::Properties>,
}
impl HdInsight {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod hd_insight {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Port open for ssh connections on the master node of the cluster."]
        #[serde(rename = "sshPort", default, skip_serializing_if = "Option::is_none")]
        pub ssh_port: Option<i32>,
        #[doc = "Public IP address of the master node of the cluster."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[doc = "Admin credentials for virtual machine"]
        #[serde(rename = "administratorAccount", default, skip_serializing_if = "Option::is_none")]
        pub administrator_account: Option<VirtualMachineSshCredentials>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[doc = "dictionary containing all the user assigned identities, with resourceId of the UAI as key."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        #[serde(rename = "SystemAssigned,UserAssigned")]
        SystemAssignedUserAssigned,
        UserAssigned,
        None,
    }
}
#[doc = "An Image asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageAsset {
    #[doc = "The Asset Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The mime type."]
    #[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[doc = "The Url of the Asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Whether the Asset is unpacked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpack: Option<bool>,
}
impl ImageAsset {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[doc = "The ArmId of the keyVault where the customer owned encryption key is present."]
    #[serde(rename = "keyVaultArmId")]
    pub key_vault_arm_id: String,
    #[doc = "Key vault uri to access the encryption key."]
    #[serde(rename = "keyIdentifier")]
    pub key_identifier: String,
    #[doc = "For future use - The client id of the identity which will be used to access key vault."]
    #[serde(rename = "identityClientId", default, skip_serializing_if = "Option::is_none")]
    pub identity_client_id: Option<String>,
}
impl KeyVaultProperties {
    pub fn new(key_vault_arm_id: String, key_identifier: String) -> Self {
        Self {
            key_vault_arm_id,
            key_identifier,
            identity_client_id: None,
        }
    }
}
#[doc = "Linked workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedWorkspace {
    #[doc = "ResourceId of the link of the linked workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Friendly name of the linked workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type of linked workspace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "LinkedWorkspace specific properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LinkedWorkspaceProps>,
}
impl LinkedWorkspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "object used for creating linked workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedWorkspaceDto {
    #[doc = "Friendly name of the linked workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "LinkedWorkspace specific properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LinkedWorkspaceProps>,
}
impl LinkedWorkspaceDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "LinkedWorkspace specific properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedWorkspaceProps {
    #[doc = "ResourceId of the link target of the linked workspace."]
    #[serde(rename = "linkedWorkspaceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub linked_workspace_resource_id: Option<String>,
    #[doc = "ResourceId of the user assigned identity for the linked workspace."]
    #[serde(rename = "userAssignedIdentityResourceId", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity_resource_id: Option<String>,
}
impl LinkedWorkspaceProps {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Aml user feature operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListAmlUserFeatureResult {
    #[doc = "The list of AML user facing features."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AmlUserFeature>,
    #[doc = "The URI to fetch the next page of AML user features information. Call ListNext() with this to fetch the next page of AML user features information."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListAmlUserFeatureResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListAmlUserFeatureResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Usages operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListUsagesResult {
    #[doc = "The list of AML resource usages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
    #[doc = "The URI to fetch the next page of AML resource usage information. Call ListNext() with this to fetch the next page of AML resource usage information."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListWorkspaceKeysResult {
    #[serde(rename = "userStorageKey", default, skip_serializing_if = "Option::is_none")]
    pub user_storage_key: Option<String>,
    #[serde(rename = "userStorageResourceId", default, skip_serializing_if = "Option::is_none")]
    pub user_storage_resource_id: Option<String>,
    #[serde(rename = "appInsightsInstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_instrumentation_key: Option<String>,
    #[serde(rename = "containerRegistryCredentials", default, skip_serializing_if = "Option::is_none")]
    pub container_registry_credentials: Option<RegistryListCredentialsResult>,
}
impl ListWorkspaceKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List WorkspaceQuotasByVMFamily operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListWorkspaceQuotas {
    #[doc = "The list of Workspace Quotas by VM Family"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceQuota>,
    #[doc = "The URI to fetch the next page of workspace quota information by VM Family. Call ListNext() with this to fetch the next page of Workspace Quota information."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListWorkspaceQuotas {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListWorkspaceQuotas {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The liveness probe requirements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LivenessProbeRequirements {
    #[doc = "The number of failures to allow before returning an unhealthy status."]
    #[serde(rename = "failureThreshold", default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[doc = "The number of successful probes before returning a healthy status."]
    #[serde(rename = "successThreshold", default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[doc = "The probe timeout in seconds."]
    #[serde(rename = "timeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    #[doc = "The length of time between probes in seconds."]
    #[serde(rename = "periodSeconds", default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[doc = "The delay before the first probe in seconds."]
    #[serde(rename = "initialDelaySeconds", default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
}
impl LivenessProbeRequirements {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wrapper for error response to follow ARM guidelines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineLearningServiceError {
    #[doc = "Error response information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl azure_core::Continuable for MachineLearningServiceError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MachineLearningServiceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Machine Learning Model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model {
    #[doc = "The Model Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The Model name."]
    pub name: String,
    #[doc = "The Model framework."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    #[doc = "The Model framework version."]
    #[serde(rename = "frameworkVersion", default, skip_serializing_if = "Option::is_none")]
    pub framework_version: Option<String>,
    #[doc = "The Model version assigned by Model Management Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "The list of datasets associated with the model."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub datasets: Vec<DatasetReference>,
    #[doc = "The URL of the Model. Usually a SAS URL."]
    pub url: String,
    #[doc = "The MIME type of Model content. For more details about MIME type, please open https://www.iana.org/assignments/media-types/media-types.xhtml"]
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[doc = "The Model description text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Model creation time (UTC)."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The Model last modified time (UTC)."]
    #[serde(rename = "modifiedTime", with = "azure_core::date::rfc3339::option")]
    pub modified_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates whether we need to unpack the Model during docker Image creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpack: Option<bool>,
    #[doc = "The Parent Model Id."]
    #[serde(rename = "parentModelId", default, skip_serializing_if = "Option::is_none")]
    pub parent_model_id: Option<String>,
    #[doc = "The RunId that created this model."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "The name of the experiment where this model was created."]
    #[serde(rename = "experimentName", default, skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    #[doc = "The Model tag dictionary. Items are mutable."]
    #[serde(rename = "kvTags", default, skip_serializing_if = "Option::is_none")]
    pub kv_tags: Option<serde_json::Value>,
    #[doc = "The Model property dictionary. Properties are immutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Models derived from this model"]
    #[serde(rename = "derivedModelIds", default, skip_serializing_if = "Vec::is_empty")]
    pub derived_model_ids: Vec<String>,
    #[doc = "Sample Input Data for the Model. A reference to a dataset in the workspace in the format aml://dataset/{datasetId}"]
    #[serde(rename = "sampleInputData", default, skip_serializing_if = "Option::is_none")]
    pub sample_input_data: Option<String>,
    #[doc = "Sample Output Data for the Model. A reference to a dataset in the workspace in the format aml://dataset/{datasetId}"]
    #[serde(rename = "sampleOutputData", default, skip_serializing_if = "Option::is_none")]
    pub sample_output_data: Option<String>,
    #[doc = "The resource requirements for the container (cpu and memory)."]
    #[serde(rename = "resourceRequirements", default, skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<ContainerResourceRequirements>,
}
impl Model {
    pub fn new(name: String, url: String, mime_type: String) -> Self {
        Self {
            id: None,
            name,
            framework: None,
            framework_version: None,
            version: None,
            datasets: Vec::new(),
            url,
            mime_type,
            description: None,
            created_time: None,
            modified_time: None,
            unpack: None,
            parent_model_id: None,
            run_id: None,
            experiment_name: None,
            kv_tags: None,
            properties: None,
            derived_model_ids: Vec::new(),
            sample_input_data: None,
            sample_output_data: None,
            resource_requirements: None,
        }
    }
}
#[doc = "The Model data collection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelDataCollection {
    #[doc = "Option for enabling/disabling Event Hub."]
    #[serde(rename = "eventHubEnabled", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_enabled: Option<bool>,
    #[doc = "Option for enabling/disabling storage."]
    #[serde(rename = "storageEnabled", default, skip_serializing_if = "Option::is_none")]
    pub storage_enabled: Option<bool>,
}
impl ModelDataCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelDockerSection {
    #[doc = "Base image used for Docker-based runs. Mutually exclusive with BaseDockerfile."]
    #[serde(rename = "baseImage", default, skip_serializing_if = "Option::is_none")]
    pub base_image: Option<String>,
    #[doc = "Base Dockerfile used for Docker-based runs. Mutually exclusive with BaseImage."]
    #[serde(rename = "baseDockerfile", default, skip_serializing_if = "Option::is_none")]
    pub base_dockerfile: Option<String>,
    #[doc = "Image registry that contains the base image."]
    #[serde(rename = "baseImageRegistry", default, skip_serializing_if = "Option::is_none")]
    pub base_image_registry: Option<serde_json::Value>,
}
impl ModelDockerSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelDockerSectionResponse {
    #[doc = "Base image used for Docker-based runs. Mutually exclusive with BaseDockerfile."]
    #[serde(rename = "baseImage", default, skip_serializing_if = "Option::is_none")]
    pub base_image: Option<String>,
    #[doc = "Base Dockerfile used for Docker-based runs. Mutually exclusive with BaseImage."]
    #[serde(rename = "baseDockerfile", default, skip_serializing_if = "Option::is_none")]
    pub base_dockerfile: Option<String>,
    #[doc = "Image registry that contains the base image."]
    #[serde(rename = "baseImageRegistry", default, skip_serializing_if = "Option::is_none")]
    pub base_image_registry: Option<serde_json::Value>,
}
impl ModelDockerSectionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelEnvironmentDefinition {
    #[doc = "The name of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The environment version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Settings for a Python environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub python: Option<serde_json::Value>,
    #[doc = "Definition of environment variables to be defined in the environment."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "The definition of a Docker container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker: Option<serde_json::Value>,
    #[doc = "The configuration for a Spark environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spark: Option<serde_json::Value>,
    #[doc = "Settings for a R environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<serde_json::Value>,
    #[doc = "The inferencing stack version added to the image. To avoid adding an inferencing stack, do not set this value. Valid values: \"latest\"."]
    #[serde(rename = "inferencingStackVersion", default, skip_serializing_if = "Option::is_none")]
    pub inferencing_stack_version: Option<String>,
}
impl ModelEnvironmentDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelEnvironmentDefinitionResponse {
    #[doc = "The name of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The environment version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Settings for a Python environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub python: Option<serde_json::Value>,
    #[doc = "Definition of environment variables to be defined in the environment."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "The definition of a Docker container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker: Option<serde_json::Value>,
    #[doc = "The configuration for a Spark environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spark: Option<serde_json::Value>,
    #[doc = "Settings for a R environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<serde_json::Value>,
    #[doc = "The inferencing stack version added to the image. To avoid adding an inferencing stack, do not set this value. Valid values: \"latest\"."]
    #[serde(rename = "inferencingStackVersion", default, skip_serializing_if = "Option::is_none")]
    pub inferencing_stack_version: Option<String>,
}
impl ModelEnvironmentDefinitionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelPythonSection {
    #[doc = "The python interpreter path to use if an environment build is not required. The path specified gets used to call the user script."]
    #[serde(rename = "interpreterPath", default, skip_serializing_if = "Option::is_none")]
    pub interpreter_path: Option<String>,
    #[doc = "True means that AzureML reuses an existing python environment; False means that AzureML will create a python environment based on the Conda dependencies specification."]
    #[serde(rename = "userManagedDependencies", default, skip_serializing_if = "Option::is_none")]
    pub user_managed_dependencies: Option<bool>,
    #[doc = "A JObject containing Conda dependencies."]
    #[serde(rename = "condaDependencies", default, skip_serializing_if = "Option::is_none")]
    pub conda_dependencies: Option<serde_json::Value>,
    #[serde(rename = "baseCondaEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub base_conda_environment: Option<String>,
}
impl ModelPythonSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelSparkSection {
    #[doc = "The list of spark repositories."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
    #[doc = "The Spark packages to use."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<SparkMavenPackage>,
    #[doc = "Whether to precache the packages."]
    #[serde(rename = "precachePackages", default, skip_serializing_if = "Option::is_none")]
    pub precache_packages: Option<bool>,
}
impl ModelSparkSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Counts of various compute node states on the amlCompute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeStateCounts {
    #[doc = "Number of compute nodes in idle state."]
    #[serde(rename = "idleNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub idle_node_count: Option<i32>,
    #[doc = "Number of compute nodes which are running jobs."]
    #[serde(rename = "runningNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub running_node_count: Option<i32>,
    #[doc = "Number of compute nodes which are being prepared."]
    #[serde(rename = "preparingNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub preparing_node_count: Option<i32>,
    #[doc = "Number of compute nodes which are in unusable state."]
    #[serde(rename = "unusableNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub unusable_node_count: Option<i32>,
    #[doc = "Number of compute nodes which are leaving the amlCompute."]
    #[serde(rename = "leavingNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub leaving_node_count: Option<i32>,
    #[doc = "Number of compute nodes which are in preempted state."]
    #[serde(rename = "preemptedNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub preempted_node_count: Option<i32>,
}
impl NodeStateCounts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Machine Learning workspace REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display name of operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The resource provider name: Microsoft.MachineLearningExperimentation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The operation that users can perform."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The description for the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "An array of operations supported by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of AML workspace operations supported by the AML workspace resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paginated list of Machine Learning compute objects wrapped in ARM resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedComputeResourcesList {
    #[doc = "An array of Machine Learning compute objects wrapped in ARM resource envelope."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComputeResource>,
    #[doc = "A continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedComputeResourcesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PaginatedComputeResourcesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paginated list of Machine Learning service objects wrapped in ARM resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedServiceList {
    #[doc = "An array of Machine Learning compute objects wrapped in ARM resource envelope."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceResource>,
    #[doc = "A continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedServiceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PaginatedServiceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Password {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Password {
    pub fn new() -> Self {
        Self::default()
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
    Disconnected,
    Timeout,
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
            Self::Disconnected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 3u32, "Disconnected"),
            Self::Timeout => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 4u32, "Timeout"),
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
#[doc = "The properties for Quota update or retrieval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaBaseProperties {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The maximum permitted quota of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "An enum describing the unit of quota measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<quota_base_properties::Unit>,
}
impl QuotaBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod quota_base_properties {
    use super::*;
    #[doc = "An enum describing the unit of quota measurement."]
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
#[doc = "Quota update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaUpdateParameters {
    #[doc = "The list for update quota."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QuotaBaseProperties>,
}
impl QuotaUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RCranPackage {
    #[doc = "The package name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The repository name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}
impl RCranPackage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RGitHubPackage {
    #[doc = "Repository address in the format username/repo[/subdir][@ref|#pull]."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[doc = "Personal access token to install from a private repo"]
    #[serde(rename = "authToken", default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
}
impl RGitHubPackage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RGitHubPackageResponse {
    #[doc = "Repository address in the format username/repo[/subdir][@ref|#pull]."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}
impl RGitHubPackageResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RSection {
    #[doc = "The version of R to be installed"]
    #[serde(rename = "rVersion", default, skip_serializing_if = "Option::is_none")]
    pub r_version: Option<String>,
    #[doc = "Indicates whether the environment is managed by user or by AzureML."]
    #[serde(rename = "userManaged", default, skip_serializing_if = "Option::is_none")]
    pub user_managed: Option<bool>,
    #[doc = "The Rscript path to use if an environment build is not required.\r\nThe path specified gets used to call the user script."]
    #[serde(rename = "rscriptPath", default, skip_serializing_if = "Option::is_none")]
    pub rscript_path: Option<String>,
    #[doc = "Date of MRAN snapshot to use in YYYY-MM-DD format, e.g. \"2019-04-17\""]
    #[serde(rename = "snapshotDate", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_date: Option<String>,
    #[doc = "The CRAN packages to use."]
    #[serde(rename = "cranPackages", default, skip_serializing_if = "Vec::is_empty")]
    pub cran_packages: Vec<RCranPackage>,
    #[doc = "The packages directly from GitHub."]
    #[serde(rename = "gitHubPackages", default, skip_serializing_if = "Vec::is_empty")]
    pub git_hub_packages: Vec<RGitHubPackage>,
    #[doc = "The packages from custom urls."]
    #[serde(rename = "customUrlPackages", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_url_packages: Vec<String>,
    #[doc = "The packages from Bioconductor."]
    #[serde(rename = "bioConductorPackages", default, skip_serializing_if = "Vec::is_empty")]
    pub bio_conductor_packages: Vec<String>,
}
impl RSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RSectionResponse {
    #[doc = "The version of R to be installed"]
    #[serde(rename = "rVersion", default, skip_serializing_if = "Option::is_none")]
    pub r_version: Option<String>,
    #[doc = "Indicates whether the environment is managed by user or by AzureML."]
    #[serde(rename = "userManaged", default, skip_serializing_if = "Option::is_none")]
    pub user_managed: Option<bool>,
    #[doc = "The Rscript path to use if an environment build is not required.\r\nThe path specified gets used to call the user script."]
    #[serde(rename = "rscriptPath", default, skip_serializing_if = "Option::is_none")]
    pub rscript_path: Option<String>,
    #[doc = "Date of MRAN snapshot to use in YYYY-MM-DD format, e.g. \"2019-04-17\""]
    #[serde(rename = "snapshotDate", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_date: Option<String>,
    #[doc = "The CRAN packages to use."]
    #[serde(rename = "cranPackages", default, skip_serializing_if = "Vec::is_empty")]
    pub cran_packages: Vec<RCranPackage>,
    #[doc = "The packages directly from GitHub."]
    #[serde(rename = "gitHubPackages", default, skip_serializing_if = "Vec::is_empty")]
    pub git_hub_packages: Vec<RGitHubPackageResponse>,
    #[doc = "The packages from custom urls."]
    #[serde(rename = "customUrlPackages", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_url_packages: Vec<String>,
    #[doc = "The packages from Bioconductor."]
    #[serde(rename = "bioConductorPackages", default, skip_serializing_if = "Vec::is_empty")]
    pub bio_conductor_packages: Vec<String>,
}
impl RSectionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryListCredentialsResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passwords: Vec<Password>,
}
impl RegistryListCredentialsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Resource Manager resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Specifies the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Specifies the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Contains resource tags defined as key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Sku of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a resource ID. For example, for a subnet, it is the resource URL for the subnet."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceId {
    #[doc = "The ID of the resource"]
    pub id: String,
}
impl ResourceId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The Resource Name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceName {
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The localized name of the resource."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl ResourceName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The quota assigned to a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceQuota {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The Resource Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "The maximum permitted quota of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "An enum describing the unit of quota measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<resource_quota::Unit>,
}
impl ResourceQuota {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_quota {
    use super::*;
    #[doc = "An enum describing the unit of quota measurement."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuLocationInfo {
    #[doc = "Location of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "List of availability zones where the SKU is supported."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Details of capabilities available to a SKU in specific zones."]
    #[serde(rename = "zoneDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub zone_details: Vec<ResourceSkuZoneDetails>,
}
impl ResourceSkuLocationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes The zonal capabilities of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuZoneDetails {
    #[doc = "The set of zones that the SKU is available in with the specified capabilities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    #[doc = "A list of capabilities that are available for the SKU in the specified list of zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
}
impl ResourceSkuZoneDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The restriction because of which SKU cannot be used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Restriction {
    #[doc = "The type of restrictions. As of now only possible value for this is location."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "The reason for the restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<restriction::ReasonCode>,
}
impl Restriction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restriction {
    use super::*;
    #[doc = "The reason for the restriction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReasonCode")]
    pub enum ReasonCode {
        NotSpecified,
        NotAvailableForRegion,
        NotAvailableForSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ReasonCode", 0u32, "NotSpecified"),
                Self::NotAvailableForRegion => serializer.serialize_unit_variant("ReasonCode", 1u32, "NotAvailableForRegion"),
                Self::NotAvailableForSubscription => serializer.serialize_unit_variant("ReasonCode", 2u32, "NotAvailableForSubscription"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Features/user capabilities associated with the sku"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapability {
    #[doc = "Capability/Feature ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Details about the feature/capability"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "scale settings for AML Compute"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleSettings {
    #[doc = "Max number of nodes to use"]
    #[serde(rename = "maxNodeCount")]
    pub max_node_count: i32,
    #[doc = "Min number of nodes to use"]
    #[serde(rename = "minNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub min_node_count: Option<i32>,
    #[doc = "Node Idle Time before scaling down amlCompute. This string needs to be in the RFC Format."]
    #[serde(rename = "nodeIdleTimeBeforeScaleDown", default, skip_serializing_if = "Option::is_none")]
    pub node_idle_time_before_scale_down: Option<String>,
}
impl ScaleSettings {
    pub fn new(max_node_count: i32) -> Self {
        Self {
            max_node_count,
            min_node_count: None,
            node_idle_time_before_scale_down: None,
        }
    }
}
#[doc = "Service principal credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalCredentials {
    #[doc = "Client Id"]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Client secret"]
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
}
impl ServicePrincipalCredentials {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self { client_id, client_secret }
    }
}
#[doc = "Machine Learning service object wrapped into ARM resource envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The base service response. The correct inherited response based on computeType will be returned (ex. ACIServiceResponse)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceResponseBase>,
}
impl ServiceResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The base service response. The correct inherited response based on computeType will be returned (ex. ACIServiceResponse)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResponseBase {
    #[doc = "The service description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The service tag dictionary. Tags are mutable."]
    #[serde(rename = "kvTags", default, skip_serializing_if = "Option::is_none")]
    pub kv_tags: Option<serde_json::Value>,
    #[doc = "The service property dictionary. Properties are immutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The current state of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<service_response_base::State>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[doc = "The compute environment type for the service."]
    #[serde(rename = "computeType")]
    pub compute_type: service_response_base::ComputeType,
    #[doc = "The deployment type for the service."]
    #[serde(rename = "deploymentType", default, skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<service_response_base::DeploymentType>,
}
impl ServiceResponseBase {
    pub fn new(compute_type: service_response_base::ComputeType) -> Self {
        Self {
            description: None,
            kv_tags: None,
            properties: None,
            state: None,
            error: None,
            compute_type,
            deployment_type: None,
        }
    }
}
pub mod service_response_base {
    use super::*;
    #[doc = "The current state of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Transitioning,
        Healthy,
        Unhealthy,
        Failed,
        Unschedulable,
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
                Self::Transitioning => serializer.serialize_unit_variant("State", 0u32, "Transitioning"),
                Self::Healthy => serializer.serialize_unit_variant("State", 1u32, "Healthy"),
                Self::Unhealthy => serializer.serialize_unit_variant("State", 2u32, "Unhealthy"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::Unschedulable => serializer.serialize_unit_variant("State", 4u32, "Unschedulable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The compute environment type for the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComputeType")]
    pub enum ComputeType {
        #[serde(rename = "ACI")]
        Aci,
        #[serde(rename = "AKS")]
        Aks,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComputeType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComputeType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComputeType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Aci => serializer.serialize_unit_variant("ComputeType", 0u32, "ACI"),
                Self::Aks => serializer.serialize_unit_variant("ComputeType", 1u32, "AKS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The deployment type for the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeploymentType")]
    pub enum DeploymentType {
        #[serde(rename = "GRPCRealtimeEndpoint")]
        GrpcRealtimeEndpoint,
        HttpRealtimeEndpoint,
        Batch,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeploymentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeploymentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeploymentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::GrpcRealtimeEndpoint => serializer.serialize_unit_variant("DeploymentType", 0u32, "GRPCRealtimeEndpoint"),
                Self::HttpRealtimeEndpoint => serializer.serialize_unit_variant("DeploymentType", 1u32, "HttpRealtimeEndpoint"),
                Self::Batch => serializer.serialize_unit_variant("DeploymentType", 2u32, "Batch"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResource {
    #[doc = "Unique name of the private link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of a shared private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedPrivateLinkResourceProperty>,
}
impl SharedPrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a shared private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPrivateLinkResourceProperty {
    #[doc = "The resource id that private link links to."]
    #[serde(rename = "privateLinkResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_resource_id: Option<String>,
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Request message."]
    #[serde(rename = "requestMessage", default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
}
impl SharedPrivateLinkResourceProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "Name of the sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Tier of the sku like Basic or Enterprise"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of skus with features"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkspaceSku>,
    #[doc = "The URI to fetch the next page of Workspace Skus. Call ListNext() with this URI to fetch the next page of Workspace Skus"]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkMavenPackage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SparkMavenPackage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ssl configuration for scoring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SslConfiguration {
    #[doc = "Enable or disable ssl for scoring"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ssl_configuration::Status>,
    #[doc = "Cert data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    #[doc = "Key data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "CNAME of the cert"]
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
    #[doc = "Enable or disable ssl for scoring"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Disabled,
        Enabled,
    }
}
#[doc = "A system service running on a compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemService {
    #[doc = "The type of this system service."]
    #[serde(rename = "systemServiceType", default, skip_serializing_if = "Option::is_none")]
    pub system_service_type: Option<String>,
    #[doc = "Public IP address"]
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[doc = "The version for this type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SystemService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for update Quota response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateWorkspaceQuotas {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The maximum permitted quota of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "An enum describing the unit of quota measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<update_workspace_quotas::Unit>,
    #[doc = "Status of update workspace quota."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<update_workspace_quotas::Status>,
}
impl UpdateWorkspaceQuotas {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_workspace_quotas {
    use super::*;
    #[doc = "An enum describing the unit of quota measurement."]
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
    #[doc = "Status of update workspace quota."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Undefined,
        Success,
        Failure,
        InvalidQuotaBelowClusterMinimum,
        InvalidQuotaExceedsSubscriptionLimit,
        #[serde(rename = "InvalidVMFamilyName")]
        InvalidVmFamilyName,
        OperationNotSupportedForSku,
        OperationNotEnabledForRegion,
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
                Self::Undefined => serializer.serialize_unit_variant("Status", 0u32, "Undefined"),
                Self::Success => serializer.serialize_unit_variant("Status", 1u32, "Success"),
                Self::Failure => serializer.serialize_unit_variant("Status", 2u32, "Failure"),
                Self::InvalidQuotaBelowClusterMinimum => {
                    serializer.serialize_unit_variant("Status", 3u32, "InvalidQuotaBelowClusterMinimum")
                }
                Self::InvalidQuotaExceedsSubscriptionLimit => {
                    serializer.serialize_unit_variant("Status", 4u32, "InvalidQuotaExceedsSubscriptionLimit")
                }
                Self::InvalidVmFamilyName => serializer.serialize_unit_variant("Status", 5u32, "InvalidVMFamilyName"),
                Self::OperationNotSupportedForSku => serializer.serialize_unit_variant("Status", 6u32, "OperationNotSupportedForSku"),
                Self::OperationNotEnabledForRegion => serializer.serialize_unit_variant("Status", 7u32, "OperationNotEnabledForRegion"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result of update workspace quota."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateWorkspaceQuotasResult {
    #[doc = "The list of workspace quota update result."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UpdateWorkspaceQuotas>,
    #[doc = "The URI to fetch the next page of workspace quota update result. Call ListNext() with this to fetch the next page of Workspace Quota update result."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl UpdateWorkspaceQuotasResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes AML Resource Usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "An enum describing the unit of usage measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
    #[doc = "The current usage of the resource."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The maximum permitted usage of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
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
    #[doc = "An enum describing the unit of usage measurement."]
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
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The localized name of the resource."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings for user account that gets created on each on the nodes of a compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccountCredentials {
    #[doc = "Name of the administrator user account which can be used to SSH to nodes."]
    #[serde(rename = "adminUserName")]
    pub admin_user_name: String,
    #[doc = "SSH public key of the administrator user account."]
    #[serde(rename = "adminUserSshPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_ssh_public_key: Option<String>,
    #[doc = "Password of the administrator user account."]
    #[serde(rename = "adminUserPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_password: Option<String>,
}
impl UserAccountCredentials {
    pub fn new(admin_user_name: String) -> Self {
        Self {
            admin_user_name,
            admin_user_ssh_public_key: None,
            admin_user_password: None,
        }
    }
}
#[doc = "dictionary containing all the user assigned identities, with resourceId of the UAI as key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User Assigned Identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the user assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the user assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The clientId(aka appId) of the user assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Machine Learning compute based on Azure Virtual Machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(flatten)]
    pub compute: Compute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<virtual_machine::Properties>,
}
impl VirtualMachine {
    pub fn new(compute: Compute) -> Self {
        Self { compute, properties: None }
    }
}
pub mod virtual_machine {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Virtual Machine size"]
        #[serde(rename = "virtualMachineSize", default, skip_serializing_if = "Option::is_none")]
        pub virtual_machine_size: Option<String>,
        #[doc = "Port open for ssh connections."]
        #[serde(rename = "sshPort", default, skip_serializing_if = "Option::is_none")]
        pub ssh_port: Option<i32>,
        #[doc = "Public IP address of the virtual machine."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[doc = "Admin credentials for virtual machine"]
        #[serde(rename = "administratorAccount", default, skip_serializing_if = "Option::is_none")]
        pub administrator_account: Option<VirtualMachineSshCredentials>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Virtual Machine image for Windows AML Compute"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineImage {
    #[doc = "Virtual Machine image path"]
    pub id: String,
}
impl VirtualMachineImage {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Secrets related to a Machine Learning compute based on AKS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineSecrets {
    #[serde(flatten)]
    pub compute_secrets: ComputeSecrets,
    #[doc = "Admin credentials for virtual machine"]
    #[serde(rename = "administratorAccount", default, skip_serializing_if = "Option::is_none")]
    pub administrator_account: Option<VirtualMachineSshCredentials>,
}
impl VirtualMachineSecrets {
    pub fn new(compute_secrets: ComputeSecrets) -> Self {
        Self {
            compute_secrets,
            administrator_account: None,
        }
    }
}
#[doc = "Describes the properties of a VM size."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSize {
    #[doc = "The name of the virtual machine size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The family name of the virtual machine size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The number of vCPUs supported by the virtual machine size."]
    #[serde(rename = "vCPUs", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us: Option<i32>,
    #[doc = "The number of gPUs supported by the virtual machine size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpus: Option<i32>,
    #[doc = "The OS VHD disk size, in MB, allowed by the virtual machine size."]
    #[serde(rename = "osVhdSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub os_vhd_size_mb: Option<i32>,
    #[doc = "The resource volume size, in MB, allowed by the virtual machine size."]
    #[serde(rename = "maxResourceVolumeMB", default, skip_serializing_if = "Option::is_none")]
    pub max_resource_volume_mb: Option<i32>,
    #[doc = "The amount of memory, in GB, supported by the virtual machine size."]
    #[serde(rename = "memoryGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_gb: Option<f64>,
    #[doc = "Specifies if the virtual machine size supports low priority VMs."]
    #[serde(rename = "lowPriorityCapable", default, skip_serializing_if = "Option::is_none")]
    pub low_priority_capable: Option<bool>,
    #[doc = "Specifies if the virtual machine size supports premium IO."]
    #[serde(rename = "premiumIO", default, skip_serializing_if = "Option::is_none")]
    pub premium_io: Option<bool>,
    #[doc = "The estimated price info for using a VM."]
    #[serde(rename = "estimatedVMPrices", default, skip_serializing_if = "Option::is_none")]
    pub estimated_vm_prices: Option<EstimatedVmPrices>,
}
impl VirtualMachineSize {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Virtual Machine size operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSizeListResult {
    #[doc = "The list of virtual machine sizes supported by AmlCompute."]
    #[serde(rename = "amlCompute", default, skip_serializing_if = "Vec::is_empty")]
    pub aml_compute: Vec<VirtualMachineSize>,
}
impl VirtualMachineSizeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Admin credentials for virtual machine"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSshCredentials {
    #[doc = "Username of admin account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Password of admin account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Public key data"]
    #[serde(rename = "publicKeyData", default, skip_serializing_if = "Option::is_none")]
    pub public_key_data: Option<String>,
    #[doc = "Private key data"]
    #[serde(rename = "privateKeyData", default, skip_serializing_if = "Option::is_none")]
    pub private_key_data: Option<String>,
}
impl VirtualMachineSshCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VnetConfiguration {
    #[doc = "The name of the virtual network."]
    #[serde(rename = "vnetName", default, skip_serializing_if = "Option::is_none")]
    pub vnet_name: Option<String>,
    #[doc = "The name of the virtual network subnet."]
    #[serde(rename = "subnetName", default, skip_serializing_if = "Option::is_none")]
    pub subnet_name: Option<String>,
}
impl VnetConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workspace {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list machine learning workspaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListResult {
    #[doc = "The list of machine learning workspaces. Since this list may be incomplete, the nextLink field should be used to request the next list of machine learning workspaces."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[doc = "The URI that can be used to request the next list of machine learning workspaces."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceProperties {
    #[doc = "The immutable id associated with this workspace."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The description of this workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The friendly name for this workspace. This name in mutable"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The creation time of the machine learning workspace in ISO8601 format."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "ARM id of the key vault associated with this workspace. This cannot be changed once the workspace has been created"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<String>,
    #[doc = "ARM id of the application insights associated with this workspace. This cannot be changed once the workspace has been created"]
    #[serde(rename = "applicationInsights", default, skip_serializing_if = "Option::is_none")]
    pub application_insights: Option<String>,
    #[doc = "ARM id of the container registry associated with this workspace. This cannot be changed once the workspace has been created"]
    #[serde(rename = "containerRegistry", default, skip_serializing_if = "Option::is_none")]
    pub container_registry: Option<String>,
    #[doc = "ARM id of the storage account associated with this workspace. This cannot be changed once the workspace has been created"]
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<String>,
    #[doc = "Url for the discovery service to identify regional endpoints for machine learning experimentation services"]
    #[serde(rename = "discoveryUrl", default, skip_serializing_if = "Option::is_none")]
    pub discovery_url: Option<String>,
    #[doc = "The current deployment state of workspace resource. The provisioningState is to indicate states for resource provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workspace_properties::ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperty>,
    #[doc = "The flag to signal HBI data in the workspace and reduce diagnostic data collected by the service"]
    #[serde(rename = "hbiWorkspace", default, skip_serializing_if = "Option::is_none")]
    pub hbi_workspace: Option<bool>,
    #[doc = "The name of the managed resource group created by workspace RP in customer subscription if the workspace is CMK workspace"]
    #[serde(rename = "serviceProvisionedResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub service_provisioned_resource_group: Option<String>,
    #[doc = "Count of private connections in the workspace"]
    #[serde(rename = "privateLinkCount", default, skip_serializing_if = "Option::is_none")]
    pub private_link_count: Option<i32>,
    #[doc = "The compute name for image build"]
    #[serde(rename = "imageBuildCompute", default, skip_serializing_if = "Option::is_none")]
    pub image_build_compute: Option<String>,
    #[doc = "The flag to indicate whether to allow public access when behind VNet."]
    #[serde(rename = "allowPublicAccessWhenBehindVnet", default, skip_serializing_if = "Option::is_none")]
    pub allow_public_access_when_behind_vnet: Option<bool>,
    #[doc = "The list of private endpoint connections in the workspace."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The list of shared private link resources in this workspace."]
    #[serde(rename = "sharedPrivateLinkResources", default, skip_serializing_if = "Vec::is_empty")]
    pub shared_private_link_resources: Vec<SharedPrivateLinkResource>,
}
impl WorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_properties {
    use super::*;
    #[doc = "The current deployment state of workspace resource. The provisioningState is to indicate states for resource provisioning."]
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
}
#[doc = "The parameters for updating the properties of a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePropertiesUpdateParameters {
    #[doc = "The description of this workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The friendly name for this workspace."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl WorkspacePropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes Workspace Sku details and features"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceSku {
    #[doc = "The set of locations that the SKU is available. This will be supported and registered Azure Geo Regions (e.g. West US, East US, Southeast Asia, etc.)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "A list of locations and availability zones in those locations where the SKU is available."]
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<ResourceSkuLocationInfo>,
    #[doc = "Sku Tier like Basic or Enterprise"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of features/user capabilities associated with the sku"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
    #[doc = "The restrictions because of which SKU cannot be used. This is empty if there are no restrictions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<Restriction>,
}
impl WorkspaceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for updating a machine learning workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceUpdateParameters {
    #[doc = "The resource tags for the machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Sku of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The parameters for updating the properties of a machine learning workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePropertiesUpdateParameters>,
}
impl WorkspaceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
