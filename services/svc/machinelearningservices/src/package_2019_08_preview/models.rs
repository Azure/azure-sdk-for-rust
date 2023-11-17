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
    #[doc = "The Model data collection properties."]
    #[serde(rename = "dataCollection", default, skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<ModelDataCollection>,
    #[doc = "The SSL certificate to use if SSL is enabled."]
    #[serde(rename = "sslCertificate", default, skip_serializing_if = "Option::is_none")]
    pub ssl_certificate: Option<String>,
    #[doc = "The SSL key for the certificate."]
    #[serde(rename = "sslKey", default, skip_serializing_if = "Option::is_none")]
    pub ssl_key: Option<String>,
    #[doc = "The CName for the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    #[doc = "The Dns label for the service."]
    #[serde(rename = "dnsNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub dns_name_label: Option<String>,
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
    #[doc = "The Id of the Image."]
    #[serde(rename = "imageId", default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "imageDetails", default, skip_serializing_if = "Option::is_none")]
    pub image_details: Option<DockerImageResponse>,
    #[doc = "The Uri for sending scoring requests."]
    #[serde(rename = "scoringUri", default, skip_serializing_if = "Option::is_none")]
    pub scoring_uri: Option<String>,
    #[doc = "The location of the service."]
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
    #[doc = "The Model data collection properties."]
    #[serde(rename = "dataCollection", default, skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<ModelDataCollection>,
    #[doc = "The SSL certificate to use if SSL is enabled."]
    #[serde(rename = "sslCertificate", default, skip_serializing_if = "Option::is_none")]
    pub ssl_certificate: Option<String>,
    #[doc = "The SSL key for the certificate."]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ModelEnvironmentDefinition>,
    #[doc = "The list of models."]
    #[serde(
        rename = "models",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub models_: Vec<Model>,
}
impl AciServiceResponse {
    pub fn new(service_response_base: ServiceResponseBase) -> Self {
        Self {
            service_response_base,
            container_resource_requirements: None,
            image_id: None,
            image_details: None,
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
            environment: None,
            models_: Vec::new(),
        }
    }
}
#[doc = "The response for an AKS Endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AksEndpointResponse {
    #[serde(flatten)]
    pub service_response_base: ServiceResponseBase,
    #[doc = "The name of the compute resource."]
    #[serde(rename = "computeName", default, skip_serializing_if = "Option::is_none")]
    pub compute_name: Option<String>,
    #[doc = "The Kubernetes namespace of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Whether or not Application Insights is enabled."]
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
    #[doc = "Whether or not authentication is enabled."]
    #[serde(rename = "authEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auth_enabled: Option<bool>,
    #[doc = "Whether or not AAD authentication is enabled."]
    #[serde(rename = "aadAuthEnabled", default, skip_serializing_if = "Option::is_none")]
    pub aad_auth_enabled: Option<bool>,
    #[doc = "The Uri for sending swagger requests."]
    #[serde(rename = "swaggerUri", default, skip_serializing_if = "Option::is_none")]
    pub swagger_uri: Option<String>,
    #[doc = "The Uri for sending scoring requests."]
    #[serde(rename = "scoringUri", default, skip_serializing_if = "Option::is_none")]
    pub scoring_uri: Option<String>,
    #[doc = "All the variants that belongs to this endpoint."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variants: Vec<AksServiceResponse>,
}
impl AksEndpointResponse {
    pub fn new(service_response_base: ServiceResponseBase) -> Self {
        Self {
            service_response_base,
            compute_name: None,
            namespace: None,
            app_insights_enabled: None,
            auth_enabled: None,
            aad_auth_enabled: None,
            swagger_uri: None,
            scoring_uri: None,
            variants: Vec::new(),
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
    #[doc = "The Model Management Service Error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ModelErrorResponse>,
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
    #[doc = "The Model data collection properties."]
    #[serde(rename = "dataCollection", default, skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<ModelDataCollection>,
    #[doc = "The name of the compute resource."]
    #[serde(rename = "computeName", default, skip_serializing_if = "Option::is_none")]
    pub compute_name: Option<String>,
    #[doc = "Whether or not Application Insights is enabled."]
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
    #[doc = "The Auto Scaler properties."]
    #[serde(rename = "autoScaler", default, skip_serializing_if = "Option::is_none")]
    pub auto_scaler: Option<AutoScaler>,
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
    pub liveness_probe_requirements: Option<LivenessProbeRequirements>,
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
    #[serde(rename = "imageDetails", default, skip_serializing_if = "Option::is_none")]
    pub image_details: Option<ImageResponseBaseUnion>,
    #[doc = "The Id of the Image."]
    #[serde(rename = "imageId", default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[doc = "The list of models."]
    #[serde(
        rename = "models",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub models_: Vec<Model>,
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
    #[doc = "The Model data collection properties."]
    #[serde(rename = "dataCollection", default, skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<ModelDataCollection>,
    #[doc = "Whether or not Application Insights is enabled."]
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
    #[doc = "The Auto Scaler properties."]
    #[serde(rename = "autoScaler", default, skip_serializing_if = "Option::is_none")]
    pub auto_scaler: Option<AutoScaler>,
    #[doc = "The Uri for sending scoring requests."]
    #[serde(rename = "scoringUri", default, skip_serializing_if = "Option::is_none")]
    pub scoring_uri: Option<String>,
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<AksReplicaStatus>,
    #[doc = "The scoring timeout in milliseconds."]
    #[serde(rename = "scoringTimeoutMs", default, skip_serializing_if = "Option::is_none")]
    pub scoring_timeout_ms: Option<i32>,
    #[doc = "The liveness probe requirements."]
    #[serde(rename = "livenessProbeRequirements", default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe_requirements: Option<LivenessProbeRequirements>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ModelEnvironmentDefinition>,
}
impl AksServiceResponse {
    pub fn new(aks_variant_response: AksVariantResponse) -> Self {
        Self {
            aks_variant_response,
            image_details: None,
            image_id: None,
            models_: Vec::new(),
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
            environment: None,
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
    pub enum Type {
        Control,
        Treatment,
    }
}
#[doc = "Details of an Artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[doc = "The identifier of an Artifact. Format of ArtifactId - {Origin}/{Container}/{Path}."]
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[doc = "The origin of the Artifact creation request. Available origins are 'ExperimentRun', 'LocalUpload', 'WebUpload', 'Dataset' and 'Unknown'."]
    pub origin: String,
    #[doc = "The name of container. Artifacts can be grouped by container."]
    pub container: String,
    #[doc = "The path to the Artifact in a container."]
    pub path: String,
    #[doc = "The Etag of the Artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The Date and Time at which the Artifact is created. The DateTime is in UTC."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[serde(rename = "dataPath", default, skip_serializing_if = "Option::is_none")]
    pub data_path: Option<DataPath>,
}
impl Artifact {
    pub fn new(origin: String, container: String, path: String) -> Self {
        Self {
            artifact_id: None,
            origin,
            container,
            path,
            etag: None,
            created_time: None,
            data_path: None,
        }
    }
}
#[doc = "Details of the Artifact Container's shared access signature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactContainerSas {
    #[doc = "The shared access signature of the Container."]
    #[serde(rename = "containerSas")]
    pub container_sas: String,
    #[doc = "The URI of the Container."]
    #[serde(rename = "containerUri")]
    pub container_uri: String,
    #[doc = "The Prefix to the Blobs in the Container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[doc = "The Prefix to the Artifact in the Blob."]
    #[serde(rename = "artifactPrefix", default, skip_serializing_if = "Option::is_none")]
    pub artifact_prefix: Option<String>,
}
impl ArtifactContainerSas {
    pub fn new(container_sas: String, container_uri: String) -> Self {
        Self {
            container_sas,
            container_uri,
            prefix: None,
            artifact_prefix: None,
        }
    }
}
#[doc = "Details of an Artifact Content Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactContentInformation {
    #[doc = "The URI of the content."]
    #[serde(rename = "contentUri", default, skip_serializing_if = "Option::is_none")]
    pub content_uri: Option<String>,
    #[doc = "The origin of the Artifact creation request. Available origins are 'ExperimentRun', 'LocalUpload', 'WebUpload', 'Dataset', 'ComputeRecord', 'Metric', and 'Unknown'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The name of container. Artifacts can be grouped by container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[doc = "The path to the Artifact in a container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl ArtifactContentInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}
impl ArtifactDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains list of Artifact Ids."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactIdList {
    #[doc = "List of Artifacts Ids."]
    #[serde(rename = "artifactIds")]
    pub artifact_ids: Vec<String>,
}
impl ArtifactIdList {
    pub fn new(artifact_ids: Vec<String>) -> Self {
        Self { artifact_ids }
    }
}
#[doc = "Details of an Artifact Path."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactPath {
    #[doc = "The path to the Artifact in a container."]
    pub path: String,
}
impl ArtifactPath {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}
#[doc = "Contains list of Artifact Paths."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactPathList {
    #[doc = "List of Artifact Paths."]
    pub paths: Vec<ArtifactPath>,
}
impl ArtifactPathList {
    pub fn new(paths: Vec<ArtifactPath>) -> Self {
        Self { paths }
    }
}
#[doc = "The Asset definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    #[doc = "The Asset Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the Asset."]
    pub name: String,
    #[doc = "The Asset description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A list of child artifacts."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<ArtifactDetails>,
    #[doc = "The Asset tag dictionary. Tags are mutable."]
    #[serde(rename = "kvTags", default, skip_serializing_if = "Option::is_none")]
    pub kv_tags: Option<serde_json::Value>,
    #[doc = "The Asset property dictionary. Properties are immutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The RunId associated with this Asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runid: Option<String>,
    #[doc = "A dictionary containing metadata about the Asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    #[doc = "The time the Asset was created in UTC."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
}
impl Asset {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            description: None,
            artifacts: Vec::new(),
            kv_tags: None,
            properties: None,
            runid: None,
            meta: None,
            created_time: None,
        }
    }
}
#[doc = "The async operation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncOperationDetails {
    #[doc = "The suboperation type."]
    #[serde(rename = "subOperationType", default, skip_serializing_if = "Option::is_none")]
    pub sub_operation_type: Option<String>,
    #[doc = "The suboperation state."]
    #[serde(rename = "subOperationState", default, skip_serializing_if = "Option::is_none")]
    pub sub_operation_state: Option<String>,
}
impl AsyncOperationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The async operation status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncOperationStatus {
    #[doc = "The async operation id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The async operation type."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[doc = "The async operation state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<async_operation_status::State>,
    #[doc = "The async operation creation time (UTC)."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The async operation end time (UTC)l"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The Model Management Service Error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ModelErrorResponse>,
    #[doc = "The resource created/updated by the async operation."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "The async operation log."]
    #[serde(rename = "operationLog", default, skip_serializing_if = "Option::is_none")]
    pub operation_log: Option<String>,
    #[doc = "The async operation details."]
    #[serde(rename = "operationDetails", default, skip_serializing_if = "Option::is_none")]
    pub operation_details: Option<AsyncOperationDetails>,
    #[doc = "The request id that created this operation"]
    #[serde(rename = "parentRequestId", default, skip_serializing_if = "Option::is_none")]
    pub parent_request_id: Option<String>,
}
impl AsyncOperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod async_operation_status {
    use super::*;
    #[doc = "The async operation state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        NotStarted,
        Running,
        Cancelled,
        Succeeded,
        Failed,
        TimedOut,
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
#[doc = "Service Token"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthToken {
    #[doc = "Access token."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[doc = "Access token type."]
    #[serde(rename = "tokenType", default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[doc = "Access token expiry time (UTC)."]
    #[serde(rename = "expiryOn", default, skip_serializing_if = "Option::is_none")]
    pub expiry_on: Option<i64>,
    #[doc = "Refresh access token after time (UTC)."]
    #[serde(rename = "refreshAfter", default, skip_serializing_if = "Option::is_none")]
    pub refresh_after: Option<i64>,
}
impl AuthToken {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDataLake {
    #[doc = "The Azure Data Lake store name"]
    #[serde(rename = "storeName", default, skip_serializing_if = "Option::is_none")]
    pub store_name: Option<String>,
    #[doc = "The Client ID/Application ID"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The ID of the tenant the service principal/app belongs to"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Is it using certificate to authenticate. If false then use client secret"]
    #[serde(rename = "isCertAuth", default, skip_serializing_if = "Option::is_none")]
    pub is_cert_auth: Option<bool>,
    #[doc = "The content of the certificate used for authentication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "The thumbprint of the certificate above"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The client secret"]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "The authority URL used for authentication"]
    #[serde(rename = "authorityUrl", default, skip_serializing_if = "Option::is_none")]
    pub authority_url: Option<String>,
    #[doc = "The resource the service principal/app has access to"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource Group Name"]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl AzureDataLake {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzurePostgreSql {
    #[doc = "The Azure PostgreSQL server name"]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The Azure PostgreSQL database name"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The Azure PostgreSQL user id"]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "The Azure PostgreSQL user password"]
    #[serde(rename = "userPassword", default, skip_serializing_if = "Option::is_none")]
    pub user_password: Option<String>,
    #[doc = "The Azure PostgreSQL port number"]
    #[serde(rename = "portNumber", default, skip_serializing_if = "Option::is_none")]
    pub port_number: Option<String>,
    #[doc = "The Azure PostgreSQL server host endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource Group Name"]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl AzurePostgreSql {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabase {
    #[doc = "The Azure SQL server name"]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The Azure SQL database name"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The server host endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "The Client ID/Application ID"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The ID of the tenant the service principal/app belongs to"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Is it using certificate to authenticate. If false then use client secret"]
    #[serde(rename = "isCertAuth", default, skip_serializing_if = "Option::is_none")]
    pub is_cert_auth: Option<bool>,
    #[doc = "The content of the certificate used for authentication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "The thumbprint of the certificate above"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The client secret"]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "The authority URL used for authentication"]
    #[serde(rename = "authorityUrl", default, skip_serializing_if = "Option::is_none")]
    pub authority_url: Option<String>,
    #[doc = "The resource the service principal/app has access to"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource Group Name"]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl AzureSqlDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorage {
    #[doc = "Storage Account Name"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The storage container name"]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "The host of the container"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "The protocol to use. Defaults to https"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "The credential type"]
    #[serde(rename = "credentialType", default, skip_serializing_if = "Option::is_none")]
    pub credential_type: Option<azure_storage::CredentialType>,
    #[doc = "The credential"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    #[serde(rename = "clientCredentials", default, skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<ClientCredentials>,
    #[doc = "If this is an Microsoft.MachineLearning.DataStore.Contracts.DataStoreType.AzureBlob, the length of time (in seconds) to cache files locally after they are accessed (downloaded)."]
    #[serde(rename = "blobCacheTimeout", default, skip_serializing_if = "Option::is_none")]
    pub blob_cache_timeout: Option<i32>,
    #[doc = "Indicate if we are using SAS token or Account Key (Deprecated)"]
    #[serde(rename = "isSas", default, skip_serializing_if = "Option::is_none")]
    pub is_sas: Option<bool>,
    #[doc = "Storage Account Key (Deprecated)"]
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[doc = "SAS Token for the container (Deprecated)"]
    #[serde(rename = "sasToken", default, skip_serializing_if = "Option::is_none")]
    pub sas_token: Option<String>,
    #[doc = "Indicate if we are using Workspace ManagedIdentities/MSI token"]
    #[serde(rename = "areWorkspaceManagedIdentitiesAllowed", default, skip_serializing_if = "Option::is_none")]
    pub are_workspace_managed_identities_allowed: Option<bool>,
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource Group Name"]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl AzureStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_storage {
    use super::*;
    #[doc = "The credential type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CredentialType {
        None,
        Sas,
        AccountKey,
        ClientCredentials,
    }
}
#[doc = "Base event is the envelope used to post event data to the Event controller"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseEvent {
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
impl BaseEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchAddOrModifyRunRequest {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub runs: Vec<CreateRun>,
}
impl BatchAddOrModifyRunRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchAddOrModifyRunResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}
impl BatchAddOrModifyRunResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the Batch Artifact Content Information request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchArtifactContentInformationResult {
    #[doc = "Artifact details of the Artifact Ids requested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<serde_json::Value>,
    #[doc = "Artifact Content Information details of the Artifact Ids requested."]
    #[serde(rename = "artifactContentInformation", default, skip_serializing_if = "Option::is_none")]
    pub artifact_content_information: Option<serde_json::Value>,
    #[doc = "Errors occurred while fetching the requested Artifact Ids."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}
impl BatchArtifactContentInformationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchEventCommand {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<BaseEvent>,
}
impl BatchEventCommand {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchEventCommandResult {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<KeyValuePairBaseEventErrorResponse>,
}
impl BatchEventCommandResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchMetric {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<Metric>,
}
impl BatchMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchServiceResponse {
    #[serde(flatten)]
    pub service_response_base: ServiceResponseBase,
    #[serde(
        rename = "modelIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub model_ids: Vec<String>,
    #[serde(rename = "computeName", default, skip_serializing_if = "Option::is_none")]
    pub compute_name: Option<String>,
    #[serde(rename = "environmentName", default, skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "environmentVersion", default, skip_serializing_if = "Option::is_none")]
    pub environment_version: Option<String>,
    #[serde(rename = "scoringUri", default, skip_serializing_if = "Option::is_none")]
    pub scoring_uri: Option<String>,
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
    #[doc = "The Model data collection properties."]
    #[serde(rename = "modelDataCollection", default, skip_serializing_if = "Option::is_none")]
    pub model_data_collection: Option<ModelDataCollection>,
    #[serde(rename = "entryScript", default, skip_serializing_if = "Option::is_none")]
    pub entry_script: Option<String>,
    #[serde(rename = "inputFormat", default, skip_serializing_if = "Option::is_none")]
    pub input_format: Option<String>,
    #[serde(rename = "outputAction", default, skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "miniBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub mini_batch_size: Option<i32>,
    #[serde(rename = "errorThreshold", default, skip_serializing_if = "Option::is_none")]
    pub error_threshold: Option<f64>,
    #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[serde(rename = "processCountPerNode", default, skip_serializing_if = "Option::is_none")]
    pub process_count_per_node: Option<i32>,
}
impl BatchServiceResponse {
    pub fn new(service_response_base: ServiceResponseBase) -> Self {
        Self {
            service_response_base,
            model_ids: Vec::new(),
            compute_name: None,
            environment_name: None,
            environment_version: None,
            scoring_uri: None,
            app_insights_enabled: None,
            model_data_collection: None,
            entry_script: None,
            input_format: None,
            output_action: None,
            mini_batch_size: None,
            error_threshold: None,
            node_count: None,
            process_count_per_node: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientCredentials {
    #[doc = "The Client ID/Application ID"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The ID of the tenant the service principal/app belongs to"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Is it using certificate to authenticate. If false then use client secret"]
    #[serde(rename = "isCertAuth", default, skip_serializing_if = "Option::is_none")]
    pub is_cert_auth: Option<bool>,
    #[doc = "The content of the certificate used for authentication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "The thumbprint of the certificate above"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The client secret"]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "The authority URL used for authentication"]
    #[serde(rename = "authorityUrl", default, skip_serializing_if = "Option::is_none")]
    pub authority_url: Option<String>,
    #[doc = "The resource the service principal/app has access to"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource Group Name"]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl ClientCredentials {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The request to create an Endpoint in the AKS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEndpointRequest {
    #[serde(flatten)]
    pub create_service_request: CreateServiceRequest,
    #[doc = "The name of the compute resource."]
    #[serde(rename = "computeName", default, skip_serializing_if = "Option::is_none")]
    pub compute_name: Option<String>,
    #[doc = "Whether or not Application Insights is enabled."]
    #[serde(rename = "appInsightsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_enabled: Option<bool>,
    #[doc = "Whether or not authentication is enabled."]
    #[serde(rename = "authEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auth_enabled: Option<bool>,
    #[doc = "Whether or not AAD authentication is enabled."]
    #[serde(rename = "aadAuthEnabled", default, skip_serializing_if = "Option::is_none")]
    pub aad_auth_enabled: Option<bool>,
    #[doc = "Kubernetes namespace for the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The service tag list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub variants: Vec<AksServiceCreateRequest>,
}
impl CreateEndpointRequest {
    pub fn new(create_service_request: CreateServiceRequest) -> Self {
        Self {
            create_service_request,
            compute_name: None,
            app_insights_enabled: None,
            auth_enabled: None,
            aad_auth_enabled: None,
            namespace: None,
            variants: Vec::new(),
        }
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
    pub enum Type {
        Control,
        Treatment,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateIotServiceRequest {
    #[serde(flatten)]
    pub create_service_request: CreateServiceRequest,
    #[serde(rename = "iotDeviceId", default, skip_serializing_if = "Option::is_none")]
    pub iot_device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<serde_json::Value>,
    #[serde(
        rename = "iotEdgeUserModule",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub iot_edge_user_module: Vec<IotModuleSettings>,
    #[serde(rename = "iotEdgeModule", default, skip_serializing_if = "Option::is_none")]
    pub iot_edge_module: Option<IotBaseModuleSettings>,
    #[serde(rename = "computeName", default, skip_serializing_if = "Option::is_none")]
    pub compute_name: Option<String>,
    #[serde(
        rename = "acrCredentials",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub acr_credentials: Vec<RegistryInfo>,
    #[serde(rename = "authEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auth_enabled: Option<bool>,
}
impl CreateIotServiceRequest {
    pub fn new(create_service_request: CreateServiceRequest) -> Self {
        Self {
            create_service_request,
            iot_device_id: None,
            routes: None,
            iot_edge_user_module: Vec::new(),
            iot_edge_module: None,
            compute_name: None,
            acr_credentials: Vec::new(),
            auth_enabled: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateRun {
    #[doc = "The identifier for the run. Run IDs must be less than 256 characters and contain only alphanumeric characters with dashes and underscores."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "The parent of the run if the run is hierarchical; otherwise, Null."]
    #[serde(rename = "parentRunId", default, skip_serializing_if = "Option::is_none")]
    pub parent_run_id: Option<String>,
    #[doc = "The status of the run. The Status string value maps to the RunStatus Enum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The start time of the run in UTC."]
    #[serde(rename = "startTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The end time of the run in UTC."]
    #[serde(rename = "endTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub end_time_utc: Option<time::OffsetDateTime>,
    #[serde(rename = "heartbeatEnabled", default, skip_serializing_if = "Option::is_none")]
    pub heartbeat_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<RunOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "dataContainerId", default, skip_serializing_if = "Option::is_none")]
    pub data_container_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "scriptName", default, skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "runDefinition", default, skip_serializing_if = "Option::is_none")]
    pub run_definition: Option<serde_json::Value>,
    #[serde(rename = "createdFrom", default, skip_serializing_if = "Option::is_none")]
    pub created_from: Option<CreatedFrom>,
    #[serde(rename = "cancelUri", default, skip_serializing_if = "Option::is_none")]
    pub cancel_uri: Option<String>,
    #[serde(rename = "diagnosticsUri", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_uri: Option<String>,
}
impl CreateRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The base class for creating a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateServiceRequest {
    #[doc = "The service name."]
    pub name: String,
    #[doc = "The description of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The service tag dictionary. Tags are mutable."]
    #[serde(rename = "kvTags", default, skip_serializing_if = "Option::is_none")]
    pub kv_tags: Option<serde_json::Value>,
    #[doc = "The service properties dictionary. Properties are immutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<AuthKeys>,
    #[doc = "The deployment type for the service."]
    #[serde(rename = "deploymentType", default, skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<create_service_request::DeploymentType>,
    #[doc = "The Image Id."]
    #[serde(rename = "imageId", default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[doc = "Request to create a Docker image based on Environment."]
    #[serde(rename = "environmentImageRequest", default, skip_serializing_if = "Option::is_none")]
    pub environment_image_request: Option<EnvironmentImageRequest>,
    #[doc = "The location of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl CreateServiceRequest {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            kv_tags: None,
            properties: None,
            keys: None,
            deployment_type: None,
            image_id: None,
            environment_image_request: None,
            location: None,
        }
    }
}
pub mod create_service_request {
    use super::*;
    #[doc = "The deployment type for the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeploymentType {
        #[serde(rename = "GRPCRealtimeEndpoint")]
        GrpcRealtimeEndpoint,
        HttpRealtimeEndpoint,
        Batch,
    }
}
#[doc = "The compute environment type for the service."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "computeType")]
pub enum CreateServiceRequestUnion {
    #[serde(rename = "ACI")]
    Aci(AciServiceCreateRequest),
    #[serde(rename = "AKS")]
    Aks(AksServiceCreateRequest),
    #[serde(rename = "AKSENDPOINT")]
    Aksendpoint(CreateEndpointRequest),
    #[serde(rename = "IOT")]
    Iot(CreateIotServiceRequest),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreatedBy {
    #[doc = "A user or service principal's object ID.\r\nThis is PII and should never be logged."]
    #[serde(rename = "userObjectId", default, skip_serializing_if = "Option::is_none")]
    pub user_object_id: Option<String>,
    #[doc = "A user or service principal's tenant ID."]
    #[serde(rename = "userTenantId", default, skip_serializing_if = "Option::is_none")]
    pub user_tenant_id: Option<String>,
    #[doc = "A user's full name or a service principal's app ID.\r\nThis is PII and should never be logged."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
impl CreatedBy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreatedFrom {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<created_from::Type>,
    #[serde(rename = "locationType", default, skip_serializing_if = "Option::is_none")]
    pub location_type: Option<created_from::LocationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl CreatedFrom {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod created_from {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Notebook,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LocationType {
        ArtifactId,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataPath {
    #[serde(rename = "dataStoreName", default, skip_serializing_if = "Option::is_none")]
    pub data_store_name: Option<String>,
    #[serde(rename = "relativePath", default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    #[serde(rename = "sqlDataPath", default, skip_serializing_if = "Option::is_none")]
    pub sql_data_path: Option<SqlDataPath>,
}
impl DataPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class for managing DataReferenceConfiguration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataReferenceConfiguration {
    #[doc = "The name of the data store."]
    #[serde(rename = "dataStoreName", default, skip_serializing_if = "Option::is_none")]
    pub data_store_name: Option<String>,
    #[doc = "Operation on the datastore, mount, download, upload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<data_reference_configuration::Mode>,
    #[doc = "Relative path on the datastore."]
    #[serde(rename = "pathOnDataStore", default, skip_serializing_if = "Option::is_none")]
    pub path_on_data_store: Option<String>,
    #[doc = "The path on the compute target."]
    #[serde(rename = "pathOnCompute", default, skip_serializing_if = "Option::is_none")]
    pub path_on_compute: Option<String>,
    #[doc = "Whether to overwrite the data if existing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
}
impl DataReferenceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_reference_configuration {
    use super::*;
    #[doc = "Operation on the datastore, mount, download, upload."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Mount,
        Download,
        Upload,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataStore {
    #[doc = "Name of the datastore"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Azure storage service this datastore points to."]
    #[serde(rename = "dataStoreType", default, skip_serializing_if = "Option::is_none")]
    pub data_store_type: Option<data_store::DataStoreType>,
    #[doc = "A read only property that denotes whether the service datastore has been validated with credentials."]
    #[serde(rename = "hasBeenValidated", default, skip_serializing_if = "Option::is_none")]
    pub has_been_validated: Option<bool>,
    #[doc = "Tags to datastore"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "azureStorageSection", default, skip_serializing_if = "Option::is_none")]
    pub azure_storage_section: Option<AzureStorage>,
    #[serde(rename = "azureDataLakeSection", default, skip_serializing_if = "Option::is_none")]
    pub azure_data_lake_section: Option<AzureDataLake>,
    #[serde(rename = "azureSqlDatabaseSection", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_database_section: Option<AzureSqlDatabase>,
    #[serde(rename = "azurePostgreSqlSection", default, skip_serializing_if = "Option::is_none")]
    pub azure_postgre_sql_section: Option<AzurePostgreSql>,
    #[serde(rename = "glusterFsSection", default, skip_serializing_if = "Option::is_none")]
    pub gluster_fs_section: Option<GlusterFs>,
}
impl DataStore {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_store {
    use super::*;
    #[doc = "The Azure storage service this datastore points to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataStoreType {
        AzureBlob,
        AzureFile,
        AzureDataLake,
        AzureSqlDatabase,
        AzurePostgreSql,
        #[serde(rename = "DBFS")]
        Dbfs,
        AzureDataLakeGen2,
        GlusterFs,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteTagsCommand {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
}
impl DeleteTagsCommand {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deployment summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentSummary {
    #[doc = "The number of successful deployments."]
    #[serde(rename = "successfulDeployments", default, skip_serializing_if = "Option::is_none")]
    pub successful_deployments: Option<i32>,
    #[doc = "The number of unsuccessful deployments."]
    #[serde(rename = "unsuccessfulDeployments", default, skip_serializing_if = "Option::is_none")]
    pub unsuccessful_deployments: Option<i32>,
}
impl DeploymentSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerImageResponse {
    #[serde(flatten)]
    pub image_response_base: ImageResponseBase,
    #[doc = "The name of the driver file."]
    #[serde(rename = "driverProgram", default, skip_serializing_if = "Option::is_none")]
    pub driver_program: Option<String>,
    #[doc = "The list of assets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assets: Vec<EnvironmentImageAsset>,
    #[doc = "The target runtime."]
    #[serde(rename = "targetRuntime", default, skip_serializing_if = "Option::is_none")]
    pub target_runtime: Option<TargetRuntime>,
    #[doc = "The Uri to the docker file."]
    #[serde(rename = "dockerFileUri", default, skip_serializing_if = "Option::is_none")]
    pub docker_file_uri: Option<String>,
    #[doc = "The Uri to the generated docker file."]
    #[serde(rename = "generatedDockerFileUri", default, skip_serializing_if = "Option::is_none")]
    pub generated_docker_file_uri: Option<String>,
}
impl DockerImageResponse {
    pub fn new(image_response_base: ImageResponseBase) -> Self {
        Self {
            image_response_base,
            driver_program: None,
            assets: Vec::new(),
            target_runtime: None,
            docker_file_uri: None,
            generated_docker_file_uri: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DockerSection {
    #[doc = "Base image used for Docker-based runs. Mutually exclusive with BaseDockerfile."]
    #[serde(rename = "baseImage", default, skip_serializing_if = "Option::is_none")]
    pub base_image: Option<String>,
    #[doc = "Base Dockerfile used for Docker-based runs. Mutually exclusive with BaseImage."]
    #[serde(rename = "baseDockerfile", default, skip_serializing_if = "Option::is_none")]
    pub base_dockerfile: Option<String>,
    #[doc = "Set true to perform this run inside a Docker container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Set false to disable AzureML's usage of the Docker shared volumes feature to work around bugs in certain versions of Docker for Windows."]
    #[serde(rename = "sharedVolumes", default, skip_serializing_if = "Option::is_none")]
    pub shared_volumes: Option<bool>,
    #[doc = "Extra arguments to the Docker run command."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub arguments: Vec<String>,
    #[serde(rename = "baseImageRegistry", default, skip_serializing_if = "Option::is_none")]
    pub base_image_registry: Option<ContainerRegistry>,
}
impl DockerSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDefinition {
    #[doc = "The name of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The environment version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub python: Option<PythonSection>,
    #[doc = "Definition of environment variables to be defined in the environment."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker: Option<DockerSection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spark: Option<SparkSection>,
    #[doc = "The inferencing stack version added to the image. To avoid adding an inferencing stack, do not set this value. Valid values: \"latest\"."]
    #[serde(rename = "inferencingStackVersion", default, skip_serializing_if = "Option::is_none")]
    pub inferencing_stack_version: Option<String>,
}
impl EnvironmentDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Image asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentImageAsset {
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
impl EnvironmentImageAsset {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to create a Docker image based on Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentImageRequest {
    #[doc = "The name of the driver file."]
    #[serde(rename = "driverProgram", default, skip_serializing_if = "Option::is_none")]
    pub driver_program: Option<String>,
    #[doc = "The list of assets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assets: Vec<EnvironmentImageAsset>,
    #[doc = "The list of model Ids."]
    #[serde(
        rename = "modelIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub model_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ModelEnvironmentDefinition>,
}
impl EnvironmentImageRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the error (e.g., the name of the property in error)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The root error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<RootError>,
    #[doc = "Dictionary containing correlation details for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation: Option<serde_json::Value>,
    #[doc = "The hosting environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[doc = "The Azure region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The time in UTC."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Experiment {
    #[serde(rename = "experimentId", default, skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdUtc", default, with = "azure_core::date::rfc3339::option")]
    pub created_utc: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "archivedTime", default, with = "azure_core::date::rfc3339::option")]
    pub archived_time: Option<time::OffsetDateTime>,
    #[serde(rename = "latestCreatedRunCreatedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub latest_created_run_created_utc: Option<time::OffsetDateTime>,
    #[serde(rename = "latestCreatedRunId", default, skip_serializing_if = "Option::is_none")]
    pub latest_created_run_id: Option<String>,
}
impl Experiment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The FPGA Docker Image response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FpgaDockerImageResponse {
    #[serde(flatten)]
    pub image_response_base: ImageResponseBase,
}
impl FpgaDockerImageResponse {
    pub fn new(image_response_base: ImageResponseBase) -> Self {
        Self { image_response_base }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlusterFs {
    #[doc = "The server address of one of the servers that hosts the GlusterFS. Can be either the IP address\r\nor server name."]
    #[serde(rename = "serverAddress", default, skip_serializing_if = "Option::is_none")]
    pub server_address: Option<String>,
    #[doc = "The name of the created GlusterFS volume."]
    #[serde(rename = "volumeName", default, skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
}
impl GlusterFs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HdiConfiguration {
    #[serde(rename = "yarnDeployMode", default, skip_serializing_if = "Option::is_none")]
    pub yarn_deploy_mode: Option<hdi_configuration::YarnDeployMode>,
}
impl HdiConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hdi_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum YarnDeployMode {
        None,
        Client,
        Cluster,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HistoryConfiguration {
    #[doc = "Set to true to collect outputs and store in run history."]
    #[serde(rename = "outputCollection", default, skip_serializing_if = "Option::is_none")]
    pub output_collection: Option<bool>,
    #[doc = "The list of directories to monitor and upload files from."]
    #[serde(
        rename = "directoriesToWatch",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories_to_watch: Vec<String>,
}
impl HistoryConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Bandit Policy configuration. Please refer https://docs.microsoft.com/en-us/python/api/azureml-train-core/azureml.train.hyperdrive.banditpolicy?view=azure-ml-py for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperDriveBanditPolicy {
    #[doc = "Policy configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<hyper_drive_bandit_policy::Properties>,
}
impl HyperDriveBanditPolicy {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
pub mod hyper_drive_bandit_policy {
    use super::*;
    #[doc = "Policy configuration properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Evaluation interval of the policy."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub evaluation_interval: Option<i32>,
        #[doc = "Value indicating the number of sequences for which the first evaluation is delayed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delay_evaluation: Option<i32>,
        #[doc = "Slack factor."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub slack_factor: Option<f32>,
        #[doc = "Slack amount."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub slack_amount: Option<f32>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Response for HyperDrive_CancelExperiment in case of success."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperDriveCancelExperimentResponse {
    #[doc = "Response code."]
    pub code: i32,
    #[doc = "Message indicating operation success."]
    pub result: String,
}
impl HyperDriveCancelExperimentResponse {
    pub fn new(code: i32, result: String) -> Self {
        Self { code, result }
    }
}
#[doc = "Part of request for HyperDrive_CreateExperiment. Contains configuration details required to create hyperdrive run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperDriveCreateExperiment {
    #[serde(flatten)]
    pub hyper_drive_experiment_base: HyperDriveExperimentBase,
    #[doc = "User who is creating the Hyperdrive run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl HyperDriveCreateExperiment {
    pub fn new(hyper_drive_experiment_base: HyperDriveExperimentBase) -> Self {
        Self {
            hyper_drive_experiment_base,
            user: None,
        }
    }
}
#[doc = "No early termination is applied in the case of DefaultPolicy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperDriveDefaultPolicy {}
impl HyperDriveDefaultPolicy {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "Response in case of an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperDriveErrorResponse {
    #[doc = "Error response code."]
    pub code: i32,
    #[doc = "Error message."]
    pub message: String,
}
impl HyperDriveErrorResponse {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "Part of response for HyperDrive_CreateExperiment in case of success. Contains details about the created hyperdrive run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperDriveExperiment {
    #[serde(flatten)]
    pub hyper_drive_experiment_base: HyperDriveExperimentBase,
    #[doc = "Indicates if all runs have been generated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_jobs_generated: Option<bool>,
    #[doc = "Indicates if cancellation has been requested for this Hyperdrive run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_requested: Option<bool>,
    #[doc = "Hyperdrive run creation time."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "Id of the user who created the Hyperdrive run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<i32>,
    #[doc = "Hyperdrive run id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    #[doc = "Hyperdrive run Uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experiment_uri: Option<String>,
    #[doc = "Hyperdrive run modification time."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub modified: Option<time::OffsetDateTime>,
    #[doc = "Hyperdrive run status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Study Uri of the Hyperdrive run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub study_uri: Option<String>,
    #[doc = "Hyperdrive run id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hyperdrive_run_id: Option<String>,
}
impl HyperDriveExperiment {
    pub fn new(hyper_drive_experiment_base: HyperDriveExperimentBase) -> Self {
        Self {
            hyper_drive_experiment_base,
            all_jobs_generated: None,
            cancellation_requested: None,
            created: None,
            created_by_user_id: None,
            experiment_id: None,
            experiment_uri: None,
            modified: None,
            status: None,
            study_uri: None,
            hyperdrive_run_id: None,
        }
    }
}
#[doc = "Base object for both request and response of HyperDrive_CreateExperiment api."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperDriveExperimentBase {
    #[doc = "The description for Hyperdrive run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Hyperparameter space and the sampling method configuration."]
    pub generator_config: hyper_drive_experiment_base::GeneratorConfig,
    #[doc = "Maximum number of runs to run concurrently."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_jobs: Option<i32>,
    #[doc = "Maximum duration of the Hyperdrive run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_duration_minutes: Option<i32>,
    #[doc = "Maximum number of runs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_total_jobs: Option<i32>,
    #[doc = "Name of the Hyperdrive run."]
    pub name: String,
    #[doc = "Platform of the Hyperdrive run."]
    pub platform: hyper_drive_experiment_base::Platform,
    #[doc = "Platform config object specifying the run definition structure."]
    pub platform_config: serde_json::Value,
    #[doc = "Early termination policy configuration."]
    pub policy_config: HyperDrivePolicyConfigBaseUnion,
    #[doc = "Name of the primary metric and goal of optimizing."]
    pub primary_metric_config: hyper_drive_experiment_base::PrimaryMetricConfig,
    #[doc = "Study Id of the Hyperdrive run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub study_id: Option<i32>,
}
impl HyperDriveExperimentBase {
    pub fn new(
        generator_config: hyper_drive_experiment_base::GeneratorConfig,
        name: String,
        platform: hyper_drive_experiment_base::Platform,
        platform_config: serde_json::Value,
        policy_config: HyperDrivePolicyConfigBaseUnion,
        primary_metric_config: hyper_drive_experiment_base::PrimaryMetricConfig,
    ) -> Self {
        Self {
            description: None,
            generator_config,
            max_concurrent_jobs: None,
            max_duration_minutes: None,
            max_total_jobs: None,
            name,
            platform,
            platform_config,
            policy_config,
            primary_metric_config,
            study_id: None,
        }
    }
}
pub mod hyper_drive_experiment_base {
    use super::*;
    #[doc = "Hyperparameter space and the sampling method configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct GeneratorConfig {
        #[doc = "Hyperparameter sampling method."]
        pub name: generator_config::Name,
        #[doc = "Dictionary specifying hyperparameter space."]
        pub parameter_space: serde_json::Value,
    }
    impl GeneratorConfig {
        pub fn new(name: generator_config::Name, parameter_space: serde_json::Value) -> Self {
            Self { name, parameter_space }
        }
    }
    pub mod generator_config {
        use super::*;
        #[doc = "Hyperparameter sampling method."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Name")]
        pub enum Name {
            #[serde(rename = "RANDOM")]
            Random,
            #[serde(rename = "GRID")]
            Grid,
            #[serde(rename = "BAYESIANOPTIMIZATION")]
            Bayesianoptimization,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Name {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Name {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Random => serializer.serialize_unit_variant("Name", 0u32, "RANDOM"),
                    Self::Grid => serializer.serialize_unit_variant("Name", 1u32, "GRID"),
                    Self::Bayesianoptimization => serializer.serialize_unit_variant("Name", 2u32, "BAYESIANOPTIMIZATION"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "Platform of the Hyperdrive run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Platform")]
    pub enum Platform {
        #[serde(rename = "AML")]
        Aml,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Platform {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Platform {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Platform {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Aml => serializer.serialize_unit_variant("Platform", 0u32, "AML"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Name of the primary metric and goal of optimizing."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PrimaryMetricConfig {
        #[doc = "Determines if the primary metric has to be minimized/maximized."]
        pub goal: primary_metric_config::Goal,
        #[doc = "Name of the primary metric reported by runs."]
        pub name: String,
    }
    impl PrimaryMetricConfig {
        pub fn new(goal: primary_metric_config::Goal, name: String) -> Self {
            Self { goal, name }
        }
    }
    pub mod primary_metric_config {
        use super::*;
        #[doc = "Determines if the primary metric has to be minimized/maximized."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Goal")]
        pub enum Goal {
            #[serde(rename = "MAXIMIZE")]
            Maximize,
            #[serde(rename = "MINIMIZE")]
            Minimize,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Goal {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Goal {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Goal {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Maximize => serializer.serialize_unit_variant("Goal", 0u32, "MAXIMIZE"),
                    Self::Minimize => serializer.serialize_unit_variant("Goal", 1u32, "MINIMIZE"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Response for HyperDrive_CreateExperiment in case of success."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperDriveExperimentResponse {
    #[doc = "Response code."]
    pub code: i32,
    #[doc = "Part of response for HyperDrive_CreateExperiment in case of success. Contains details about the created hyperdrive run."]
    pub result: HyperDriveExperiment,
}
impl HyperDriveExperimentResponse {
    pub fn new(code: i32, result: HyperDriveExperiment) -> Self {
        Self { code, result }
    }
}
#[doc = "Median stopping policy configuration. Please refer https://docs.microsoft.com/en-us/python/api/azureml-train-core/azureml.train.hyperdrive.medianstoppingpolicy?view=azure-ml-py for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperDriveMedianStoppingPolicy {
    #[doc = "Policy configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<hyper_drive_median_stopping_policy::Properties>,
}
impl HyperDriveMedianStoppingPolicy {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
pub mod hyper_drive_median_stopping_policy {
    use super::*;
    #[doc = "Policy configuration properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Evaluation interval of the policy."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub evaluation_interval: Option<i32>,
        #[doc = "Value indicating the number of sequences for which the first evaluation is delayed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delay_evaluation: Option<i32>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Type of early termination policy."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum HyperDrivePolicyConfigBaseUnion {
    Bandit(HyperDriveBanditPolicy),
    Default(HyperDriveDefaultPolicy),
    MedianStopping(HyperDriveMedianStoppingPolicy),
    TruncationSelection(HyperDriveTruncationSelectionPolicy),
}
#[doc = "Truncation selection policy configuration. Please refer https://docs.microsoft.com/en-us/python/api/azureml-train-core/azureml.train.hyperdrive.truncationselectionpolicy?view=azure-ml-py for more information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperDriveTruncationSelectionPolicy {
    #[doc = "Policy configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<hyper_drive_truncation_selection_policy::Properties>,
}
impl HyperDriveTruncationSelectionPolicy {
    pub fn new() -> Self {
        Self { properties: None }
    }
}
pub mod hyper_drive_truncation_selection_policy {
    use super::*;
    #[doc = "Policy configuration properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Evaluation interval of the policy."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub evaluation_interval: Option<i32>,
        #[doc = "Value indicating the number of sequences for which the first evaluation is delayed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delay_evaluation: Option<i32>,
        #[doc = "Truncation percentage value."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub truncation_percentage: Option<i32>,
        #[doc = "Boolean indicating if metrics from finished jobs should be excluded in the policy decision process."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub exclude_finished_jobs: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageResponseBase {
    #[doc = "The image Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The image name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The image version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "The image description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The image tag dictionary. Tags are mutable."]
    #[serde(rename = "kvTags", default, skip_serializing_if = "Option::is_none")]
    pub kv_tags: Option<serde_json::Value>,
    #[doc = "The image properties dictionary. Properties are immutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The time the image was created."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The time the image was last modified."]
    #[serde(rename = "modifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_time: Option<time::OffsetDateTime>,
    #[doc = "Whether the image will be automatically deleted with the last service using it."]
    #[serde(rename = "autoDelete", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete: Option<bool>,
    #[doc = "The type of the image."]
    #[serde(rename = "imageType", default, skip_serializing_if = "Option::is_none")]
    pub image_type: Option<image_response_base::ImageType>,
    #[doc = "The state of the operation."]
    #[serde(rename = "creationState", default, skip_serializing_if = "Option::is_none")]
    pub creation_state: Option<image_response_base::CreationState>,
    #[doc = "The Model Management Service Error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ModelErrorResponse>,
    #[doc = "The list of model Ids."]
    #[serde(
        rename = "modelIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub model_ids: Vec<String>,
    #[doc = "The list of models."]
    #[serde(
        rename = "modelDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub model_details: Vec<Model>,
    #[doc = "The Image location string."]
    #[serde(rename = "imageLocation", default, skip_serializing_if = "Option::is_none")]
    pub image_location: Option<String>,
    #[doc = "The Uri to the image build logs."]
    #[serde(rename = "imageBuildLogUri", default, skip_serializing_if = "Option::is_none")]
    pub image_build_log_uri: Option<String>,
    #[doc = "The ID of the asynchronous operation for this image."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}
impl ImageResponseBase {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            version: None,
            description: None,
            kv_tags: None,
            properties: None,
            created_time: None,
            modified_time: None,
            auto_delete: None,
            image_type: None,
            creation_state: None,
            error: None,
            model_ids: Vec::new(),
            model_details: Vec::new(),
            image_location: None,
            image_build_log_uri: None,
            operation_id: None,
        }
    }
}
pub mod image_response_base {
    use super::*;
    #[doc = "The type of the image."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ImageType {
        Docker,
    }
    #[doc = "The state of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreationState {
        NotStarted,
        Running,
        Cancelled,
        Succeeded,
        Failed,
        TimedOut,
    }
}
#[doc = "The flavor of the image."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "imageFlavor")]
pub enum ImageResponseBaseUnion {
    #[serde(rename = "WEBAPICONTAINER")]
    Webapicontainer(DockerImageResponse),
    #[serde(rename = "ACCELCONTAINER")]
    Accelcontainer(FpgaDockerImageResponse),
}
#[doc = "A nested structure of errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerErrorResponse {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A nested structure of errors."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<Box<InnerErrorResponse>>,
}
impl InnerErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotBaseModuleSettings {
    #[serde(rename = "moduleName", default, skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    #[serde(rename = "propertiesDesired", default, skip_serializing_if = "Option::is_none")]
    pub properties_desired: Option<serde_json::Value>,
    #[serde(rename = "createOptions", default, skip_serializing_if = "Option::is_none")]
    pub create_options: Option<String>,
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
}
impl IotBaseModuleSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotModuleSettings {
    #[serde(rename = "imageLocation", default, skip_serializing_if = "Option::is_none")]
    pub image_location: Option<String>,
    #[serde(rename = "moduleName", default, skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    #[serde(rename = "propertiesDesired", default, skip_serializing_if = "Option::is_none")]
    pub properties_desired: Option<serde_json::Value>,
    #[serde(rename = "createOptions", default, skip_serializing_if = "Option::is_none")]
    pub create_options: Option<String>,
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
}
impl IotModuleSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotServiceResponse {
    #[serde(flatten)]
    pub service_response_base: ServiceResponseBase,
    #[serde(rename = "iotDeviceId", default, skip_serializing_if = "Option::is_none")]
    pub iot_device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<serde_json::Value>,
    #[serde(rename = "computeName", default, skip_serializing_if = "Option::is_none")]
    pub compute_name: Option<String>,
    #[serde(
        rename = "iotEdgeModules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub iot_edge_modules: Vec<IotModuleSettings>,
    #[serde(rename = "authEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auth_enabled: Option<bool>,
    #[serde(rename = "imageDetails", default, skip_serializing_if = "Option::is_none")]
    pub image_details: Option<ImageResponseBaseUnion>,
    #[serde(rename = "imageId", default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}
impl IotServiceResponse {
    pub fn new(service_response_base: ServiceResponseBase) -> Self {
        Self {
            service_response_base,
            iot_device_id: None,
            routes: None,
            compute_name: None,
            iot_edge_modules: Vec::new(),
            auth_enabled: None,
            image_details: None,
            image_id: None,
        }
    }
}
#[doc = "The Json Patch definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonPatchOperation {
    #[doc = "The value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "The target location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[doc = "The source location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}
impl JsonPatchOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyValuePairBaseEventErrorResponse {
    #[doc = "Base event is the envelope used to post event data to the Event controller"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<BaseEvent>,
    #[doc = "The error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ErrorResponse>,
}
impl KeyValuePairBaseEventErrorResponse {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metric {
    #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    #[serde(rename = "dataContainerId", default, skip_serializing_if = "Option::is_none")]
    pub data_container_id: Option<String>,
    #[serde(rename = "metricType", default, skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    #[serde(rename = "createdUtc", default, with = "azure_core::date::rfc3339::option")]
    pub created_utc: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "numCells", default, skip_serializing_if = "Option::is_none")]
    pub num_cells: Option<i32>,
    #[serde(rename = "dataLocation", default, skip_serializing_if = "Option::is_none")]
    pub data_location: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cells: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<MetricSchema>,
}
impl Metric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSchema {
    #[serde(rename = "numProperties", default, skip_serializing_if = "Option::is_none")]
    pub num_properties: Option<i32>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub properties: Vec<MetricSchemaProperty>,
}
impl MetricSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSchemaProperty {
    #[serde(rename = "propertyId", default, skip_serializing_if = "Option::is_none")]
    pub property_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl MetricSchemaProperty {
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The Model last modified time (UTC)."]
    #[serde(rename = "modifiedTime", default, with = "azure_core::date::rfc3339::option")]
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
    #[doc = "Set True to perform this run inside a Docker container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Set False if necessary to work around shared volume bugs on Windows."]
    #[serde(rename = "sharedVolumes", default, skip_serializing_if = "Option::is_none")]
    pub shared_volumes: Option<bool>,
    #[doc = "Run with NVidia Docker extension to support GPUs."]
    #[serde(rename = "gpuSupport", default, skip_serializing_if = "Option::is_none")]
    pub gpu_support: Option<bool>,
    #[doc = "The shared memory size setting for NVidia GPUs."]
    #[serde(rename = "shmSize", default, skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<String>,
    #[doc = "Extra arguments to the Docker run command."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub arguments: Vec<String>,
    #[serde(rename = "baseImageRegistry", default, skip_serializing_if = "Option::is_none")]
    pub base_image_registry: Option<ContainerRegistry>,
}
impl ModelDockerSection {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub python: Option<ModelPythonSection>,
    #[doc = "Definition of environment variables to be defined in the environment."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker: Option<ModelDockerSection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spark: Option<ModelSparkSection>,
    #[doc = "The Inferencing stack version."]
    #[serde(rename = "inferencingStackVersion", default, skip_serializing_if = "Option::is_none")]
    pub inferencing_stack_version: Option<String>,
}
impl ModelEnvironmentDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Model Management Service Error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelErrorResponse {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The HTTP status code."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "An array of error detail objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetails>,
}
impl azure_core::Continuable for ModelErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ModelErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operational state of the Model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelOperationalState {
    #[doc = "The deployment summary."]
    #[serde(rename = "deploymentSummary", default, skip_serializing_if = "Option::is_none")]
    pub deployment_summary: Option<DeploymentSummary>,
    #[doc = "The deployment end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The deployment start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
impl ModelOperationalState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelPythonSection {
    #[doc = "The python interpreter path. This is only used when user_managed_dependencies=True."]
    #[serde(rename = "interpreterPath", default, skip_serializing_if = "Option::is_none")]
    pub interpreter_path: Option<String>,
    #[doc = "True means that AzureML reuses an existing python environment; False means that AzureML will create a python environment based on the Conda dependencies specification."]
    #[serde(rename = "userManagedDependencies", default, skip_serializing_if = "Option::is_none")]
    pub user_managed_dependencies: Option<bool>,
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub repositories: Vec<String>,
    #[doc = "The Spark packages to use."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModifyExperiment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<bool>,
}
impl ModifyExperiment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MpiConfiguration {
    #[doc = "Number of processes per node."]
    #[serde(rename = "processCountPerNode", default, skip_serializing_if = "Option::is_none")]
    pub process_count_per_node: Option<i32>,
}
impl MpiConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of ArtifactContentInformations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedArtifactContentInformationList {
    #[doc = "An array of objects of type ArtifactContentInformation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ArtifactContentInformation>,
    #[doc = "The token used in retrieving the next page.  If null, there are no additional pages."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "The link to the next page constructed using the continuationToken.  If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedArtifactContentInformationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedArtifactContentInformationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of Artifacts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedArtifactList {
    #[doc = "An array of objects of type Artifact."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Artifact>,
    #[doc = "The token used in retrieving the next page.  If null, there are no additional pages."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "The link to the next page constructed using the continuationToken.  If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedArtifactList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedArtifactList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of Assets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedAssetList {
    #[doc = "An array of objects of type Asset."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Asset>,
    #[doc = "A continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedAssetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedAssetList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of DataStores."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedDataStoreList {
    #[doc = "An array of objects of type DataStore."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DataStore>,
    #[doc = "The token used in retrieving the next page.  If null, there are no additional pages."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "The link to the next page constructed using the continuationToken.  If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedDataStoreList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedDataStoreList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of Experiments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedExperimentList {
    #[doc = "An array of objects of type Experiment."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Experiment>,
    #[doc = "The token used in retrieving the next page.  If null, there are no additional pages."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "The link to the next page constructed using the continuationToken.  If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedExperimentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedExperimentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of Models."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedModelList {
    #[doc = "An array of objects of type Model."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Model>,
    #[doc = "A continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedModelList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedModelList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of ProfileResponses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedProfileResponseList {
    #[doc = "An array of objects of type ProfileResponse."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProfileResponse>,
    #[doc = "A continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedProfileResponseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedProfileResponseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of Runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedRunList {
    #[doc = "An array of objects of type Run."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Run>,
    #[doc = "The token used in retrieving the next page.  If null, there are no additional pages."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "The link to the next page constructed using the continuationToken.  If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedRunList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedRunList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of RunMetrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedRunMetricList {
    #[doc = "An array of objects of type RunMetric."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RunMetric>,
    #[doc = "The token used in retrieving the next page.  If null, there are no additional pages."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "The link to the next page constructed using the continuationToken.  If null, there are no additional pages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedRunMetricList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedRunMetricList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of Services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedServiceList {
    #[doc = "An array of objects of type Service."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServiceResponseBaseUnion>,
    #[doc = "A continuation link (absolute URI) to the next page of results in the list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaginatedServiceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaginatedServiceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request for creating a profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileRequestBase {
    #[doc = "The profile name."]
    pub name: String,
    #[doc = "The profile description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The profile input data."]
    #[serde(rename = "inputData", default, skip_serializing_if = "Option::is_none")]
    pub input_data: Option<String>,
    #[doc = "The tags dictionary."]
    #[serde(rename = "kvTags", default, skip_serializing_if = "Option::is_none")]
    pub kv_tags: Option<serde_json::Value>,
    #[doc = "The properties dictionary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ProfileRequestBase {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            input_data: None,
            kv_tags: None,
            properties: None,
        }
    }
}
#[doc = "The profile response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileResponse {
    #[doc = "The profile name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The profile description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Image Id."]
    #[serde(rename = "imageId", default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[doc = "The input data."]
    #[serde(rename = "inputData", default, skip_serializing_if = "Option::is_none")]
    pub input_data: Option<String>,
    #[doc = "The state of the profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The profile creation time (UTC)."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The profile tags dictionary. Tags are mutable."]
    #[serde(rename = "kvTags", default, skip_serializing_if = "Option::is_none")]
    pub kv_tags: Option<serde_json::Value>,
    #[doc = "The profile properties dictionary. Properties are immutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The recommended amount of memory to allocate in GB."]
    #[serde(rename = "recommendedMemoryInGB", default, skip_serializing_if = "Option::is_none")]
    pub recommended_memory_in_gb: Option<f64>,
    #[doc = "The recommended CPU allocation."]
    #[serde(rename = "recommendedCpu", default, skip_serializing_if = "Option::is_none")]
    pub recommended_cpu: Option<f64>,
    #[doc = "Latency associated with the recommended memory/cpu config"]
    #[serde(rename = "recommendationLatencyInMs", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_latency_in_ms: Option<f64>,
    #[doc = "The profile run result."]
    #[serde(rename = "profileRunResult", default, skip_serializing_if = "Option::is_none")]
    pub profile_run_result: Option<String>,
    #[doc = "The Model Management Service Error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ModelErrorResponse>,
    #[doc = "The profiling error logs."]
    #[serde(rename = "profilingErrorLogs", default, skip_serializing_if = "Option::is_none")]
    pub profiling_error_logs: Option<String>,
}
impl ProfileResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PythonSection {
    #[doc = "The python interpreter path. This is only used when user_managed_dependencies=True."]
    #[serde(rename = "interpreterPath", default, skip_serializing_if = "Option::is_none")]
    pub interpreter_path: Option<String>,
    #[doc = "True means that AzureML reuses an existing python environment; False means that AzureML will create a python environment based on the Conda dependencies specification."]
    #[serde(rename = "userManagedDependencies", default, skip_serializing_if = "Option::is_none")]
    pub user_managed_dependencies: Option<bool>,
    #[serde(rename = "condaDependencies", default, skip_serializing_if = "Option::is_none")]
    pub conda_dependencies: Option<serde_json::Value>,
    #[serde(rename = "baseCondaEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub base_conda_environment: Option<String>,
}
impl PythonSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The set of supported filters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryParams {
    #[doc = "Allows for filtering the collection of resources.\r\nThe expression specified is evaluated for each resource in the collection, and only items where the expression evaluates to true are included in the response.\r\nSee https://docs.microsoft.com/en-us/azure/search/query-odata-filter-orderby-syntax for details on the expression syntax."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[doc = "The continuation token to use for getting the next set of resources."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "The comma separated list of resource properties to use for sorting the requested resources.\r\nOptionally, can be followed by either 'asc' or 'desc'"]
    #[serde(rename = "orderBy", default, skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[doc = "The maximum number of items in the resource collection to be included in the result.\r\nIf not specified, all items are returned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
}
impl QueryParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateServiceKeysRequest {
    #[doc = "Specification for which type of key to generate."]
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<regenerate_service_keys_request::KeyType>,
    #[doc = "The value the key is set to."]
    #[serde(rename = "keyValue", default, skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
}
impl RegenerateServiceKeysRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod regenerate_service_keys_request {
    use super::*;
    #[doc = "Specification for which type of key to generate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        Primary,
        Secondary,
    }
}
#[doc = "Contains registry information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryInfo {
    #[doc = "The user."]
    pub user: String,
    #[doc = "The location."]
    pub location: String,
    #[doc = "The password."]
    pub password: String,
}
impl RegistryInfo {
    pub fn new(user: String, location: String, password: String) -> Self {
        Self { user, location, password }
    }
}
#[doc = "The root error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RootError {
    #[doc = "The service-defined error code. Supported error codes: ServiceError, UserError, ValidationError, AzureStorageError, TransientError, RequestThrottled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the error (e.g., the name of the property in error)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The related errors that occurred during the request."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetails>,
    #[doc = "A nested structure of errors."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<InnerErrorResponse>,
}
impl RootError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of a Run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Run {
    #[serde(rename = "runNumber", default, skip_serializing_if = "Option::is_none")]
    pub run_number: Option<i32>,
    #[serde(rename = "rootRunId", default, skip_serializing_if = "Option::is_none")]
    pub root_run_id: Option<String>,
    #[doc = "The Id of the experiment that created this run."]
    #[serde(rename = "experimentId", default, skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    #[doc = "The time the run was created in UTC."]
    #[serde(rename = "createdUtc", default, with = "azure_core::date::rfc3339::option")]
    pub created_utc: Option<time::OffsetDateTime>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<CreatedBy>,
    #[doc = "The Id of the user that created the run."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "A token used for authenticating a run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "The Token expiration time in UTC."]
    #[serde(rename = "tokenExpiryTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub token_expiry_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
    #[doc = "A list of warnings that occurred during the run."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub warnings: Vec<RunDetailsWarning>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[doc = "The identifier for the run. Run IDs must be less than 256 characters and contain only alphanumeric characters with dashes and underscores."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "The parent of the run if the run is hierarchical; otherwise, Null."]
    #[serde(rename = "parentRunId", default, skip_serializing_if = "Option::is_none")]
    pub parent_run_id: Option<String>,
    #[doc = "The status of the run. The Status string value maps to the RunStatus Enum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The start time of the run in UTC."]
    #[serde(rename = "startTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The end time of the run in UTC."]
    #[serde(rename = "endTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub end_time_utc: Option<time::OffsetDateTime>,
    #[serde(rename = "heartbeatEnabled", default, skip_serializing_if = "Option::is_none")]
    pub heartbeat_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<RunOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "dataContainerId", default, skip_serializing_if = "Option::is_none")]
    pub data_container_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "scriptName", default, skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "runDefinition", default, skip_serializing_if = "Option::is_none")]
    pub run_definition: Option<serde_json::Value>,
    #[serde(rename = "createdFrom", default, skip_serializing_if = "Option::is_none")]
    pub created_from: Option<CreatedFrom>,
    #[serde(rename = "cancelUri", default, skip_serializing_if = "Option::is_none")]
    pub cancel_uri: Option<String>,
    #[serde(rename = "diagnosticsUri", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_uri: Option<String>,
}
impl Run {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunConfiguration {
    #[doc = "The relative path to the python script file. The file path is relative to the source_directory passed to submit run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[doc = "Command line arguments for the python script file."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub arguments: Vec<String>,
    #[doc = "The supported frameworks are Python, PySpark, CNTK, TensorFlow, and PyTorch. Use Tensorflow for AmlCompute clusters, and Python for distributed training jobs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework: Option<run_configuration::Framework>,
    #[doc = "The supported communicators are None, ParameterServer, OpenMpi, and IntelMpi Keep in mind that OpenMpi requires a custom image with OpenMpi installed.\r\nUse ParameterServer or OpenMpi for AmlCompute clusters. Use IntelMpi for distributed training jobs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communicator: Option<run_configuration::Communicator>,
    #[doc = "Target refers to compute where the job is scheduled for execution. The default target is \"local\" referring to the local machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "All the data sources are made available to the run during execution based on each configuration."]
    #[serde(rename = "dataReferences", default, skip_serializing_if = "Option::is_none")]
    pub data_references: Option<serde_json::Value>,
    #[doc = "This is primarily intended for notebooks to override the default job name.\r\nDefaults to ArgumentVector[0] if not specified."]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[doc = "Maximum allowed time for the run. The system will attempt to automatically cancel the run if it took longer than this value.\r\nMaxRunDurationSeconds=null means infinite duration."]
    #[serde(rename = "maxRunDurationSeconds", default, skip_serializing_if = "Option::is_none")]
    pub max_run_duration_seconds: Option<i64>,
    #[doc = "Number of compute nodes to run the job on. Only applies to AMLCompute."]
    #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<EnvironmentDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spark: Option<SparkConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tensorflow: Option<TensorflowConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mpi: Option<MpiConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hdi: Option<HdiConfiguration>,
}
impl RunConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod run_configuration {
    use super::*;
    #[doc = "The supported frameworks are Python, PySpark, CNTK, TensorFlow, and PyTorch. Use Tensorflow for AmlCompute clusters, and Python for distributed training jobs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Framework {
        Python,
        PySpark,
        Cntk,
        TensorFlow,
        PyTorch,
    }
    #[doc = "The supported communicators are None, ParameterServer, OpenMpi, and IntelMpi Keep in mind that OpenMpi requires a custom image with OpenMpi installed.\r\nUse ParameterServer or OpenMpi for AmlCompute clusters. Use IntelMpi for distributed training jobs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Communicator {
        None,
        ParameterServer,
        Gloo,
        Mpi,
        Nccl,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RunConfiguration>,
    #[doc = "Snapshots are user project folders that have been uploaded to the cloud for subsequent\r\nexecution. This field is required when executing against cloud-based compute targets\r\nunless the run submission was against the API endpoint that takes a zipped project folder\r\ninline with the request."]
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[doc = "Specifies that the run history entry for this execution should be scoped within\r\nan existing run as a child. Defaults to null, meaning the run has no parent.\r\nThis is intended for first-party service integration, not third-party API users."]
    #[serde(rename = "parentRunId", default, skip_serializing_if = "Option::is_none")]
    pub parent_run_id: Option<String>,
    #[doc = "Specifies the runsource property for this run. The default value is \"experiment\" if not specified."]
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
}
impl RunDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunDetails {
    #[doc = "The identifier for the run."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "The name of the compute target where the run is executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The status of the run. The Status string value maps to the RunStatus Enum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The parent of the run if the run is hierarchical."]
    #[serde(rename = "parentRunId", default, skip_serializing_if = "Option::is_none")]
    pub parent_run_id: Option<String>,
    #[doc = "The start time of the run in UTC."]
    #[serde(rename = "startTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The end time of the run in UTC."]
    #[serde(rename = "endTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub end_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
    #[doc = "A list of warnings that occurred during the run."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub warnings: Vec<RunDetailsWarning>,
    #[doc = "The tag dictionary for the run. Tags are mutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The properties dictionary for the run. Properties are immutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The run definition specification."]
    #[serde(rename = "runDefinition", default, skip_serializing_if = "Option::is_none")]
    pub run_definition: Option<serde_json::Value>,
    #[serde(rename = "logFiles", default, skip_serializing_if = "Option::is_none")]
    pub log_files: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl RunDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunDetailsWarning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl RunDetailsWarning {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunMetric {
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    #[serde(rename = "dataContainerId", default, skip_serializing_if = "Option::is_none")]
    pub data_container_id: Option<String>,
    #[serde(rename = "metricType", default, skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    #[serde(rename = "createdUtc", default, with = "azure_core::date::rfc3339::option")]
    pub created_utc: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "numCells", default, skip_serializing_if = "Option::is_none")]
    pub num_cells: Option<i32>,
    #[serde(rename = "dataLocation", default, skip_serializing_if = "Option::is_none")]
    pub data_location: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cells: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<MetricSchema>,
}
impl RunMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunOptions {
    #[serde(rename = "generateDataContainerIdIfNotSpecified", default, skip_serializing_if = "Option::is_none")]
    pub generate_data_container_id_if_not_specified: Option<bool>,
}
impl RunOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The base service response. The correct inherited response based on computeType will be returned (ex. ACIServiceResponse)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResponseBase {
    #[doc = "The service Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The service name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The service description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The service tag dictionary. Tags are mutable."]
    #[serde(rename = "kvTags", default, skip_serializing_if = "Option::is_none")]
    pub kv_tags: Option<serde_json::Value>,
    #[doc = "The service property dictionary. Properties are immutable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The ID of the latest asynchronous operation for this service."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "The current state of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<service_response_base::State>,
    #[doc = "The time the service was created."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The time the service was updated."]
    #[serde(rename = "updatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub updated_time: Option<time::OffsetDateTime>,
    #[doc = "The Model Management Service Error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ModelErrorResponse>,
    #[doc = "The deployment type for the service."]
    #[serde(rename = "deploymentType", default, skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<service_response_base::DeploymentType>,
}
impl ServiceResponseBase {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            description: None,
            kv_tags: None,
            properties: None,
            operation_id: None,
            state: None,
            created_time: None,
            updated_time: None,
            error: None,
            deployment_type: None,
        }
    }
}
pub mod service_response_base {
    use super::*;
    #[doc = "The current state of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Transitioning,
        Healthy,
        Unhealthy,
        Failed,
    }
    #[doc = "The deployment type for the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeploymentType {
        #[serde(rename = "GRPCRealtimeEndpoint")]
        GrpcRealtimeEndpoint,
        HttpRealtimeEndpoint,
        Batch,
    }
}
#[doc = "The compute environment type for the service."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "computeType")]
pub enum ServiceResponseBaseUnion {
    #[serde(rename = "ACI")]
    Aci(AciServiceResponse),
    #[serde(rename = "AKSENDPOINT")]
    Aksendpoint(AksEndpointResponse),
    #[serde(rename = "AKS")]
    Aks(AksServiceResponse),
    #[serde(rename = "AMLCOMPUTE")]
    Amlcompute(BatchServiceResponse),
    #[serde(rename = "IOT")]
    Iot(IotServiceResponse),
    #[serde(rename = "UNKNOWON")]
    Unknowon(UnknownServiceResponse),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
}
impl SparkConfiguration {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkSection {
    #[doc = "The list of spark repositories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub repositories: Vec<String>,
    #[doc = "The Spark packages to use."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub packages: Vec<SparkMavenPackage>,
    #[doc = "Whether to precache the packages."]
    #[serde(rename = "precachePackages", default, skip_serializing_if = "Option::is_none")]
    pub precache_packages: Option<bool>,
}
impl SparkSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDataPath {
    #[serde(rename = "sqlTableName", default, skip_serializing_if = "Option::is_none")]
    pub sql_table_name: Option<String>,
    #[serde(rename = "sqlQuery", default, skip_serializing_if = "Option::is_none")]
    pub sql_query: Option<String>,
    #[serde(rename = "sqlStoredProcedureName", default, skip_serializing_if = "Option::is_none")]
    pub sql_stored_procedure_name: Option<String>,
    #[serde(
        rename = "sqlStoredProcedureParams",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sql_stored_procedure_params: Vec<StoredProcedureParameter>,
}
impl SqlDataPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the details of a run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartRunResult {
    #[doc = "The identifier for a run."]
    #[serde(rename = "runId")]
    pub run_id: String,
}
impl StartRunResult {
    pub fn new(run_id: String) -> Self {
        Self { run_id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StoredProcedureParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<stored_procedure_parameter::Type>,
}
impl StoredProcedureParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod stored_procedure_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        String,
        Int,
        Decimal,
        Guid,
        Boolean,
        Date,
    }
}
#[doc = "The target runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetRuntime {
    #[doc = "The target architecture."]
    #[serde(rename = "targetArchitecture", default, skip_serializing_if = "Option::is_none")]
    pub target_architecture: Option<target_runtime::TargetArchitecture>,
    #[doc = "The target operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<target_runtime::OsType>,
    #[doc = "The target runtime type."]
    #[serde(rename = "runtimeType", default, skip_serializing_if = "Option::is_none")]
    pub runtime_type: Option<target_runtime::RuntimeType>,
    #[doc = "The properties dictionary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl TargetRuntime {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod target_runtime {
    use super::*;
    #[doc = "The target architecture."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetArchitecture {
        Amd64,
        Arm32v7,
    }
    #[doc = "The target operating system."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Linux,
        Windows,
    }
    #[doc = "The target runtime type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RuntimeType {
        SparkPython,
        Tlc37,
        Tlc38,
        Tlc310,
        Python,
        PythonSlim,
        PythonCustom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TensorflowConfiguration {
    #[doc = "The number of workers."]
    #[serde(rename = "workerCount", default, skip_serializing_if = "Option::is_none")]
    pub worker_count: Option<i32>,
    #[doc = "Number of parameter servers."]
    #[serde(rename = "parameterServerCount", default, skip_serializing_if = "Option::is_none")]
    pub parameter_server_count: Option<i32>,
}
impl TensorflowConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response for an unsupported Service\r\nDefines the basic service properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnknownServiceResponse {
    #[serde(flatten)]
    pub service_response_base: ServiceResponseBase,
}
impl UnknownServiceResponse {
    pub fn new(service_response_base: ServiceResponseBase) -> Self {
        Self { service_response_base }
    }
}
