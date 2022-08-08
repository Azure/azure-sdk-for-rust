#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "ApiKey authentication gives a name and a value that can be included in either the request header or query parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyAuthentication {
    #[serde(flatten)]
    pub rest_request_authentication: RestRequestAuthentication,
    #[doc = "The key name of the authentication key/value pair."]
    pub name: String,
    #[doc = "The location of the authentication key/value pair in the request."]
    #[serde(rename = "in")]
    pub in_: api_key_authentication::In,
    #[doc = "The value of the authentication key/value pair."]
    pub value: String,
}
impl ApiKeyAuthentication {
    pub fn new(
        rest_request_authentication: RestRequestAuthentication,
        name: String,
        in_: api_key_authentication::In,
        value: String,
    ) -> Self {
        Self {
            rest_request_authentication,
            name,
            in_,
            value,
        }
    }
}
pub mod api_key_authentication {
    use super::*;
    #[doc = "The location of the authentication key/value pair in the request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum In {
        Query,
        Header,
    }
}
#[doc = "The resource that defines the source location where the artifacts are located."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactSource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties that define the artifact source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ArtifactSource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
pub type ArtifactSourceListResult = Vec<ArtifactSource>;
#[doc = "The properties that define the source location where the artifacts are located."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactSourceProperties {
    #[doc = "The type of artifact source used."]
    #[serde(rename = "sourceType")]
    pub source_type: String,
    #[doc = "The path from the location that the 'authentication' property [say, a SAS URI to the blob container] refers to, to the location of the artifacts. This can be used to differentiate different versions of the artifacts. Or, different types of artifacts like binaries or templates. The location referenced by the authentication property concatenated with this optional artifactRoot path forms the artifact source location where the artifacts are expected to be found."]
    #[serde(rename = "artifactRoot", default, skip_serializing_if = "Option::is_none")]
    pub artifact_root: Option<String>,
    #[doc = "Defines the authentication method and properties to access the artifacts."]
    pub authentication: Authentication,
}
impl ArtifactSourceProperties {
    pub fn new(source_type: String, authentication: Authentication) -> Self {
        Self {
            source_type,
            artifact_root: None,
            authentication,
        }
    }
}
#[doc = "Defines the authentication method and properties to access the artifacts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authentication {
    #[doc = "The authentication type"]
    #[serde(rename = "type")]
    pub type_: String,
}
impl Authentication {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The error information object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Detailed error information of any failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detailed error information of any failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "Error code string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Descriptive error information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error target"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "More detailed error information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The attributes for the health check step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthCheckStepAttributes {
    #[doc = "The type of health check."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The duration in ISO 8601 format for which health check waits idly without any checks."]
    #[serde(rename = "waitDuration", default, skip_serializing_if = "Option::is_none")]
    pub wait_duration: Option<String>,
    #[doc = "The duration in ISO 8601 format for which the health check waits for the resource to become healthy. Health check fails if it doesn't. Health check starts to enforce healthyStateDuration once resource becomes healthy."]
    #[serde(rename = "maxElasticDuration", default, skip_serializing_if = "Option::is_none")]
    pub max_elastic_duration: Option<String>,
    #[doc = "The duration in ISO 8601 format for which the resource is expected to be continuously healthy. If maxElasticDuration is specified, healthy state duration is enforced after the detection of first healthy signal."]
    #[serde(rename = "healthyStateDuration")]
    pub healthy_state_duration: String,
}
impl HealthCheckStepAttributes {
    pub fn new(type_: String, healthy_state_duration: String) -> Self {
        Self {
            type_,
            wait_duration: None,
            max_elastic_duration: None,
            healthy_state_duration,
        }
    }
}
#[doc = "Defines the properties of a health check step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthCheckStepProperties {
    #[serde(flatten)]
    pub step_properties: StepProperties,
    #[doc = "The attributes for the health check step."]
    pub attributes: HealthCheckStepAttributes,
}
impl HealthCheckStepProperties {
    pub fn new(step_properties: StepProperties, attributes: HealthCheckStepAttributes) -> Self {
        Self {
            step_properties,
            attributes,
        }
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[doc = "The identity type."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The list of identities."]
    #[serde(rename = "identityIds")]
    pub identity_ids: Vec<String>,
}
impl Identity {
    pub fn new(type_: String, identity_ids: Vec<String>) -> Self {
        Self { type_, identity_ids }
    }
}
#[doc = "Supplementary contextual messages during a rollout."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Message {
    #[doc = "Time in UTC this message was provided."]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339::option")]
    pub time_stamp: Option<time::OffsetDateTime>,
    #[doc = "The actual message text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Message {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an operation that can be performed on the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The detail about an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDetail>,
    #[doc = "The origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The properties of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The detail about an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDetail {
    #[doc = "The name of the provider that supports the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource type on which this operation can be performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operations response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsList {
    #[doc = "Represents an operation that can be performed on the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Operation>,
}
impl OperationsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that define a step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrePostStep {
    #[doc = "The resource Id of the step to be run."]
    #[serde(rename = "stepId")]
    pub step_id: String,
}
impl PrePostStep {
    pub fn new(step_id: String) -> Self {
        Self { step_id }
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
#[doc = "Individual resource operation information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceOperation {
    #[doc = "Name of the resource as specified in the artifacts. For ARM resources, this is the name of the resource specified in the template."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Unique identifier of the operation. For ARM resources, this is the operationId obtained from ARM service."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "Type of the resource as specified in the artifacts. For ARM resources, this is the type of the resource specified in the template."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "State of the resource deployment. For ARM resources, this is the current provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Descriptive information of the resource operation."]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[doc = "Http status code of the operation."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}
impl ResourceOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A REST based health check"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestHealthCheck {
    #[doc = "A unique name for this check."]
    pub name: String,
    #[doc = "The properties that make up a REST request"]
    pub request: RestRequest,
    #[doc = "The properties that make up the expected REST response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<RestResponse>,
}
impl RestHealthCheck {
    pub fn new(name: String, request: RestRequest) -> Self {
        Self {
            name,
            request,
            response: None,
        }
    }
}
#[doc = "Defines the REST health check step properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestHealthCheckStepAttributes {
    #[serde(flatten)]
    pub health_check_step_attributes: HealthCheckStepAttributes,
    #[doc = "The parameters for the REST health check."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestParameters>,
}
impl RestHealthCheckStepAttributes {
    pub fn new(health_check_step_attributes: HealthCheckStepAttributes) -> Self {
        Self {
            health_check_step_attributes,
            properties: None,
        }
    }
}
#[doc = "The parameters for the REST health check."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestParameters {
    #[doc = "The list of checks that form the health check step."]
    #[serde(rename = "healthChecks")]
    pub health_checks: Vec<RestHealthCheck>,
}
impl RestParameters {
    pub fn new(health_checks: Vec<RestHealthCheck>) -> Self {
        Self { health_checks }
    }
}
#[doc = "The properties that make up a REST request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestRequest {
    #[doc = "The HTTP method to use for the request."]
    pub method: rest_request::Method,
    #[doc = "The HTTP URI to use for the request."]
    pub uri: String,
    #[doc = "The authentication information required in the REST health check request to the health provider."]
    pub authentication: RestRequestAuthentication,
}
impl RestRequest {
    pub fn new(method: rest_request::Method, uri: String, authentication: RestRequestAuthentication) -> Self {
        Self {
            method,
            uri,
            authentication,
        }
    }
}
pub mod rest_request {
    use super::*;
    #[doc = "The HTTP method to use for the request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Method {
        #[serde(rename = "GET")]
        Get,
        #[serde(rename = "POST")]
        Post,
    }
}
#[doc = "The authentication information required in the REST health check request to the health provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestRequestAuthentication {
    #[doc = "The authentication type."]
    #[serde(rename = "type")]
    pub type_: rest_request_authentication::Type,
}
impl RestRequestAuthentication {
    pub fn new(type_: rest_request_authentication::Type) -> Self {
        Self { type_ }
    }
}
pub mod rest_request_authentication {
    use super::*;
    #[doc = "The authentication type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        ApiKey,
        RolloutIdentity,
    }
}
#[doc = "The properties that make up the expected REST response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestResponse {
    #[doc = "The HTTP status codes expected in a successful health check response. The response is expected to match one of the given status codes. If no expected status codes are provided, default expected status code is 200 OK."]
    #[serde(rename = "successStatusCodes", default, skip_serializing_if = "Vec::is_empty")]
    pub success_status_codes: Vec<String>,
    #[doc = "The regular expressions to match the response content with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<rest_response::Regex>,
}
impl RestResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod rest_response {
    use super::*;
    #[doc = "The regular expressions to match the response content with."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Regex {
        #[doc = "The list of regular expressions."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub matches: Vec<String>,
        #[doc = "Indicates whether any or all of the expressions should match with the response content."]
        #[serde(rename = "matchQuantifier", default, skip_serializing_if = "Option::is_none")]
        pub match_quantifier: Option<regex::MatchQuantifier>,
    }
    impl Regex {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod regex {
        use super::*;
        #[doc = "Indicates whether any or all of the expressions should match with the response content."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum MatchQuantifier {
            All,
            Any,
        }
    }
}
#[doc = "Defines the rollout."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rollout {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "The properties that define a rollout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Rollout {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "RolloutIdentity uses the user-assigned managed identity authentication context specified in the Identity property during rollout creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutIdentityAuthentication {
    #[serde(flatten)]
    pub rest_request_authentication: RestRequestAuthentication,
}
impl RolloutIdentityAuthentication {
    pub fn new(rest_request_authentication: RestRequestAuthentication) -> Self {
        Self {
            rest_request_authentication,
        }
    }
}
pub type RolloutListResult = Vec<Rollout>;
#[doc = "Detailed runtime information of the rollout."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RolloutOperationInfo {
    #[doc = "The ordinal count of the number of retry attempts on a rollout. 0 if no retries of the rollout have been performed. If the rollout is updated with a PUT, this count is reset to 0."]
    #[serde(rename = "retryAttempt", default, skip_serializing_if = "Option::is_none")]
    pub retry_attempt: Option<i32>,
    #[doc = "True, if all steps that succeeded on the previous run/attempt were chosen to be skipped in this retry attempt. False, otherwise."]
    #[serde(rename = "skipSucceededOnRetry", default, skip_serializing_if = "Option::is_none")]
    pub skip_succeeded_on_retry: Option<bool>,
    #[doc = "The start time of the rollout in UTC."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The start time of the rollout in UTC. This property will not be set if the rollout has not completed yet."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Detailed error information of any failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl RolloutOperationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the properties of a rollout."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RolloutProperties {
    #[doc = "The current status of the rollout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The cardinal count of total number of retries performed on the rollout at a given time."]
    #[serde(rename = "totalRetryAttempts", default, skip_serializing_if = "Option::is_none")]
    pub total_retry_attempts: Option<i32>,
    #[doc = "Detailed runtime information of the rollout."]
    #[serde(rename = "operationInfo", default, skip_serializing_if = "Option::is_none")]
    pub operation_info: Option<RolloutOperationInfo>,
    #[doc = "The detailed information on the services being deployed."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<Service>,
}
impl RolloutProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the PUT rollout request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutRequest {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the resource."]
    pub identity: Identity,
    #[doc = "The properties for defining a rollout."]
    pub properties: RolloutRequestProperties,
}
impl RolloutRequest {
    pub fn new(tracked_resource: TrackedResource, identity: Identity, properties: RolloutRequestProperties) -> Self {
        Self {
            tracked_resource,
            identity,
            properties,
        }
    }
}
#[doc = "The properties for defining a rollout."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutRequestProperties {
    #[doc = "The version of the build being deployed."]
    #[serde(rename = "buildVersion")]
    pub build_version: String,
    #[doc = "The reference to the artifact source resource Id where the payload is located."]
    #[serde(rename = "artifactSourceId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_source_id: Option<String>,
    #[doc = "The resource Id of the service topology from which service units are being referenced in step groups to be deployed."]
    #[serde(rename = "targetServiceTopologyId")]
    pub target_service_topology_id: String,
    #[doc = "The list of step groups that define the orchestration."]
    #[serde(rename = "stepGroups")]
    pub step_groups: Vec<StepGroup>,
}
impl RolloutRequestProperties {
    pub fn new(build_version: String, target_service_topology_id: String, step_groups: Vec<StepGroup>) -> Self {
        Self {
            build_version,
            artifact_source_id: None,
            target_service_topology_id,
            step_groups,
        }
    }
}
#[doc = "Defines a specific step on a target service unit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutStep {
    #[doc = "Name of the step."]
    pub name: String,
    #[doc = "Current state of the step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The step group the current step is part of."]
    #[serde(rename = "stepGroup", default, skip_serializing_if = "Option::is_none")]
    pub step_group: Option<String>,
    #[doc = "Detailed information of a specific step run."]
    #[serde(rename = "operationInfo", default, skip_serializing_if = "Option::is_none")]
    pub operation_info: Option<StepOperationInfo>,
    #[doc = "Set of resource operations that were performed, if any, on an Azure resource."]
    #[serde(rename = "resourceOperations", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_operations: Vec<ResourceOperation>,
    #[doc = "Supplementary informative messages during rollout."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<Message>,
}
impl RolloutStep {
    pub fn new(name: String) -> Self {
        Self {
            name,
            status: None,
            step_group: None,
            operation_info: None,
            resource_operations: Vec::new(),
            messages: Vec::new(),
        }
    }
}
#[doc = "Defines the properties to access the artifacts using an Azure Storage SAS URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasAuthentication {
    #[serde(flatten)]
    pub authentication: Authentication,
    #[doc = "The properties that define SAS authentication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SasProperties>,
}
impl SasAuthentication {
    pub fn new(authentication: Authentication) -> Self {
        Self {
            authentication,
            properties: None,
        }
    }
}
#[doc = "The properties that define SAS authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasProperties {
    #[doc = "The SAS URI to the Azure Storage blob container. Any offset from the root of the container to where the artifacts are located can be defined in the artifactRoot."]
    #[serde(rename = "sasUri")]
    pub sas_uri: String,
}
impl SasProperties {
    pub fn new(sas_uri: String) -> Self {
        Self { sas_uri }
    }
}
#[doc = "Defines a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(flatten)]
    pub service_properties: ServiceProperties,
    #[doc = "Name of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The detailed information about the units that make up the service."]
    #[serde(rename = "serviceUnits", default, skip_serializing_if = "Vec::is_empty")]
    pub service_units: Vec<ServiceUnit>,
}
impl Service {
    pub fn new(service_properties: ServiceProperties) -> Self {
        Self {
            service_properties,
            name: None,
            service_units: Vec::new(),
        }
    }
}
pub type ServiceListResult = Vec<ServiceResource>;
#[doc = "The properties of a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceProperties {
    #[doc = "The Azure location to which the resources in the service belong to or should be deployed to."]
    #[serde(rename = "targetLocation")]
    pub target_location: String,
    #[doc = "The subscription to which the resources in the service belong to or should be deployed to."]
    #[serde(rename = "targetSubscriptionId")]
    pub target_subscription_id: String,
}
impl ServiceProperties {
    pub fn new(target_location: String, target_subscription_id: String) -> Self {
        Self {
            target_location,
            target_subscription_id,
        }
    }
}
#[doc = "The resource representation of a service in a service topology."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties that define a service in a service topology."]
    pub properties: serde_json::Value,
}
impl ServiceResource {
    pub fn new(tracked_resource: TrackedResource, properties: serde_json::Value) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
pub type ServiceTopologiesListResult = Vec<ServiceTopologyResource>;
#[doc = "The properties of a service topology."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTopologyProperties {
    #[doc = "The resource Id of the artifact source that contains the artifacts that can be referenced in the service units."]
    #[serde(rename = "artifactSourceId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_source_id: Option<String>,
}
impl ServiceTopologyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource representation of a service topology."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceTopologyResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties that define the service topology."]
    pub properties: serde_json::Value,
}
impl ServiceTopologyResource {
    pub fn new(tracked_resource: TrackedResource, properties: serde_json::Value) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "Defines a service unit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceUnit {
    #[serde(flatten)]
    pub service_unit_properties: ServiceUnitProperties,
    #[doc = "Name of the service unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Detailed step information, if present."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<RolloutStep>,
}
impl ServiceUnit {
    pub fn new(service_unit_properties: ServiceUnitProperties) -> Self {
        Self {
            service_unit_properties,
            name: None,
            steps: Vec::new(),
        }
    }
}
#[doc = "Defines the artifacts of a service unit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceUnitArtifacts {
    #[doc = "The full URI of the ARM template file with the SAS token."]
    #[serde(rename = "templateUri", default, skip_serializing_if = "Option::is_none")]
    pub template_uri: Option<String>,
    #[doc = "The full URI of the ARM parameters file with the SAS token."]
    #[serde(rename = "parametersUri", default, skip_serializing_if = "Option::is_none")]
    pub parameters_uri: Option<String>,
    #[doc = "The path to the ARM template file relative to the artifact source."]
    #[serde(rename = "templateArtifactSourceRelativePath", default, skip_serializing_if = "Option::is_none")]
    pub template_artifact_source_relative_path: Option<String>,
    #[doc = "The path to the ARM parameters file relative to the artifact source."]
    #[serde(rename = "parametersArtifactSourceRelativePath", default, skip_serializing_if = "Option::is_none")]
    pub parameters_artifact_source_relative_path: Option<String>,
}
impl ServiceUnitArtifacts {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceUnitListResult = Vec<ServiceUnitResource>;
#[doc = "Defines the properties of a service unit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceUnitProperties {
    #[doc = "The Azure Resource Group to which the resources in the service unit belong to or should be deployed to."]
    #[serde(rename = "targetResourceGroup")]
    pub target_resource_group: String,
    #[doc = "Describes the type of ARM deployment to be performed on the resource."]
    #[serde(rename = "deploymentMode")]
    pub deployment_mode: service_unit_properties::DeploymentMode,
    #[doc = "Defines the artifacts of a service unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<ServiceUnitArtifacts>,
}
impl ServiceUnitProperties {
    pub fn new(target_resource_group: String, deployment_mode: service_unit_properties::DeploymentMode) -> Self {
        Self {
            target_resource_group,
            deployment_mode,
            artifacts: None,
        }
    }
}
pub mod service_unit_properties {
    use super::*;
    #[doc = "Describes the type of ARM deployment to be performed on the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeploymentMode {
        Incremental,
        Complete,
    }
}
#[doc = "Represents the response of a service unit resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceUnitResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties that define the service unit."]
    pub properties: serde_json::Value,
}
impl ServiceUnitResource {
    pub fn new(tracked_resource: TrackedResource, properties: serde_json::Value) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "The properties that define a Step group in a rollout."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepGroup {
    #[doc = "The name of the step group."]
    pub name: String,
    #[doc = "The list of step group names on which this step group depends on."]
    #[serde(rename = "dependsOnStepGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on_step_groups: Vec<String>,
    #[doc = "The list of steps to be run before deploying the target."]
    #[serde(rename = "preDeploymentSteps", default, skip_serializing_if = "Vec::is_empty")]
    pub pre_deployment_steps: Vec<PrePostStep>,
    #[doc = "The resource Id of service unit to be deployed. The service unit should be from the service topology referenced in targetServiceTopologyId"]
    #[serde(rename = "deploymentTargetId")]
    pub deployment_target_id: String,
    #[doc = "The list of steps to be run after deploying the target."]
    #[serde(rename = "postDeploymentSteps", default, skip_serializing_if = "Vec::is_empty")]
    pub post_deployment_steps: Vec<PrePostStep>,
}
impl StepGroup {
    pub fn new(name: String, deployment_target_id: String) -> Self {
        Self {
            name,
            depends_on_step_groups: Vec::new(),
            pre_deployment_steps: Vec::new(),
            deployment_target_id,
            post_deployment_steps: Vec::new(),
        }
    }
}
#[doc = "Detailed information of a specific step run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StepOperationInfo {
    #[doc = "The name of the ARM deployment initiated as part of the step."]
    #[serde(rename = "deploymentName", default, skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
    #[doc = "Unique identifier to track the request for ARM-based resources."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Start time of the action in UTC."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the action in UTC."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Last time in UTC this operation was updated."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "Detailed error information of any failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl StepOperationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a step resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepProperties {
    #[doc = "The type of step."]
    #[serde(rename = "stepType")]
    pub step_type: step_properties::StepType,
}
impl StepProperties {
    pub fn new(step_type: step_properties::StepType) -> Self {
        Self { step_type }
    }
}
pub mod step_properties {
    use super::*;
    #[doc = "The type of step."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StepType {
        Wait,
        HealthCheck,
    }
}
#[doc = "The resource representation of a rollout step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of a step resource."]
    pub properties: StepProperties,
}
impl StepResource {
    pub fn new(tracked_resource: TrackedResource, properties: StepProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
pub type StepResourceListResult = Vec<StepResource>;
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
#[doc = "The parameters for the wait step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitStepAttributes {
    #[doc = "The duration in ISO 8601 format of how long the wait should be."]
    pub duration: String,
}
impl WaitStepAttributes {
    pub fn new(duration: String) -> Self {
        Self { duration }
    }
}
#[doc = "Defines the properties of a Wait step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitStepProperties {
    #[serde(flatten)]
    pub step_properties: StepProperties,
    #[doc = "The parameters for the wait step."]
    pub attributes: WaitStepAttributes,
}
impl WaitStepProperties {
    pub fn new(step_properties: StepProperties, attributes: WaitStepAttributes) -> Self {
        Self {
            step_properties,
            attributes,
        }
    }
}
