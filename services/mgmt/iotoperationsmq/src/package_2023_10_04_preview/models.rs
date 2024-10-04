#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "BasicRules of AuthorizationConfig Polar rules are not supported"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationBasicRule {
    #[doc = "This subfield defines the broker resources that the Basic Rule is applied on."]
    #[serde(rename = "brokerResources")]
    pub broker_resources: Vec<ResourceInfoDefinition>,
    #[doc = "PrincipalDefinition properties of Basic Rule"]
    pub principals: PrincipalDefinition,
}
impl AuthorizationBasicRule {
    pub fn new(broker_resources: Vec<ResourceInfoDefinition>, principals: PrincipalDefinition) -> Self {
        Self {
            broker_resources,
            principals,
        }
    }
}
#[doc = "Broker AuthorizationConfig properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationConfig {
    #[doc = "Enable caching of the authorization rules."]
    #[serde(rename = "enableCache", default, skip_serializing_if = "Option::is_none")]
    pub enable_cache: Option<bool>,
    #[doc = "Authorization Rules to be used. If no rule is set, but Authorization Resource is used that would mean DenyAll."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rules: Vec<AuthorizationBasicRule>,
}
impl AuthorizationConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Automatic TLS server certificate management with cert-manager"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomaticCertMethod {
    #[doc = "Lifetime of automatically-managed certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Cert-Manager issuerRef properties"]
    #[serde(rename = "issuerRef")]
    pub issuer_ref: CertManagerIssuerRef,
    #[doc = "Cert Manager private key properties"]
    #[serde(rename = "privateKey", default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<CertManagerPrivateKey>,
    #[doc = "When to begin renewing automatically-managed certificate."]
    #[serde(rename = "renewBefore", default, skip_serializing_if = "Option::is_none")]
    pub renew_before: Option<String>,
    #[doc = "SANs for certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub san: Option<SanForCert>,
    #[doc = "Secret for storing server certificate. Any existing data will be overwritten."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    #[doc = "Certificate K8S namespace. Omit to use default namespace."]
    #[serde(rename = "secretNamespace", default, skip_serializing_if = "Option::is_none")]
    pub secret_namespace: Option<String>,
}
impl AutomaticCertMethod {
    pub fn new(issuer_ref: CertManagerIssuerRef) -> Self {
        Self {
            duration: None,
            issuer_ref,
            private_key: None,
            renew_before: None,
            san: None,
            secret_name: None,
            secret_namespace: None,
        }
    }
}
#[doc = "Automatic TLS server certificate management with cert-manager"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomaticCertMethodUpdate {
    #[doc = "Lifetime of automatically-managed certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Cert-Manager issuerRef properties"]
    #[serde(rename = "issuerRef", default, skip_serializing_if = "Option::is_none")]
    pub issuer_ref: Option<CertManagerIssuerRefUpdate>,
    #[doc = "Cert Manager private key properties"]
    #[serde(rename = "privateKey", default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<CertManagerPrivateKeyUpdate>,
    #[doc = "When to begin renewing automatically-managed certificate."]
    #[serde(rename = "renewBefore", default, skip_serializing_if = "Option::is_none")]
    pub renew_before: Option<String>,
    #[doc = "SANs for certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub san: Option<SanForCertUpdate>,
    #[doc = "Secret for storing server certificate. Any existing data will be overwritten."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    #[doc = "Certificate K8S namespace. Omit to use default namespace."]
    #[serde(rename = "secretNamespace", default, skip_serializing_if = "Option::is_none")]
    pub secret_namespace: Option<String>,
}
impl AutomaticCertMethodUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Desired properties of the Frontend Instances of the DMQTT Broker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendChain {
    #[doc = "Partitions is desired number of physical backend chains of the given distributed MQTT broker."]
    pub partitions: i32,
    #[doc = "Redundancy Factor is desired numbers of broker instances in one chain."]
    #[serde(rename = "redundancyFactor")]
    pub redundancy_factor: i32,
    #[doc = "Defines whether disk transfer is enabled or not."]
    #[serde(rename = "temporaryDiskTransferEnabled", default, skip_serializing_if = "Option::is_none")]
    pub temporary_disk_transfer_enabled: Option<bool>,
    #[doc = "Defines the percentage usage of buffer pool above which disk transfer will start."]
    #[serde(
        rename = "temporaryDiskTransferHighWatermarkPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub temporary_disk_transfer_high_watermark_percent: Option<i32>,
    #[doc = "Defines the percentage usage of buffer pool below which disk transfer will stop."]
    #[serde(
        rename = "temporaryDiskTransferLowWatermarkPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub temporary_disk_transfer_low_watermark_percent: Option<i32>,
    #[doc = "Defines the limits for memory usage percent of the backend instances of the MQTT broker."]
    #[serde(rename = "temporaryMaxBackendMemUsagePercent", default, skip_serializing_if = "Option::is_none")]
    pub temporary_max_backend_mem_usage_percent: Option<i32>,
    #[doc = "Internal knobs of Resource Limits for FE and BE"]
    #[serde(rename = "temporaryResourceLimits", default, skip_serializing_if = "Option::is_none")]
    pub temporary_resource_limits: Option<TemporaryResourceLimitsConfig>,
    #[doc = "Number of logical backend workers per pod."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workers: Option<i32>,
}
impl BackendChain {
    pub fn new(partitions: i32, redundancy_factor: i32) -> Self {
        Self {
            partitions,
            redundancy_factor,
            temporary_disk_transfer_enabled: None,
            temporary_disk_transfer_high_watermark_percent: None,
            temporary_disk_transfer_low_watermark_percent: None,
            temporary_max_backend_mem_usage_percent: None,
            temporary_resource_limits: None,
            workers: None,
        }
    }
}
#[doc = "Desired properties of the Frontend Instances of the DMQTT Broker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendChainUpdate {
    #[doc = "Partitions is desired number of physical backend chains of the given distributed MQTT broker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partitions: Option<i32>,
    #[doc = "Redundancy Factor is desired numbers of broker instances in one chain."]
    #[serde(rename = "redundancyFactor", default, skip_serializing_if = "Option::is_none")]
    pub redundancy_factor: Option<i32>,
    #[doc = "Defines whether disk transfer is enabled or not."]
    #[serde(rename = "temporaryDiskTransferEnabled", default, skip_serializing_if = "Option::is_none")]
    pub temporary_disk_transfer_enabled: Option<bool>,
    #[doc = "Defines the percentage usage of buffer pool above which disk transfer will start."]
    #[serde(
        rename = "temporaryDiskTransferHighWatermarkPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub temporary_disk_transfer_high_watermark_percent: Option<i32>,
    #[doc = "Defines the percentage usage of buffer pool below which disk transfer will stop."]
    #[serde(
        rename = "temporaryDiskTransferLowWatermarkPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub temporary_disk_transfer_low_watermark_percent: Option<i32>,
    #[doc = "Defines the limits for memory usage percent of the backend instances of the MQTT broker."]
    #[serde(rename = "temporaryMaxBackendMemUsagePercent", default, skip_serializing_if = "Option::is_none")]
    pub temporary_max_backend_mem_usage_percent: Option<i32>,
    #[doc = "Internal knobs of Resource Limits for FE and BE"]
    #[serde(rename = "temporaryResourceLimits", default, skip_serializing_if = "Option::is_none")]
    pub temporary_resource_limits: Option<TemporaryResourceLimitsConfigUpdate>,
    #[doc = "Number of logical backend workers per pod."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workers: Option<i32>,
}
impl BackendChainUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Broker Resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerAuthenticationProperties {
    #[doc = "The list of authentication methods supported by the Authentication Resource. For each array element, NOTE - Enum only authenticator type supported."]
    #[serde(rename = "authenticationMethods")]
    pub authentication_methods: Vec<BrokerAuthenticatorMethods>,
    #[doc = "The array of listener Resources it supports."]
    #[serde(rename = "listenerRef")]
    pub listener_ref: Vec<String>,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl BrokerAuthenticationProperties {
    pub fn new(authentication_methods: Vec<BrokerAuthenticatorMethods>, listener_ref: Vec<String>) -> Self {
        Self {
            authentication_methods,
            listener_ref,
            provisioning_state: None,
        }
    }
}
#[doc = "MQ broker/authentication resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerAuthenticationResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Broker Resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BrokerAuthenticationProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl BrokerAuthenticationResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a BrokerAuthenticationResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerAuthenticationResourceListResult {
    #[doc = "The BrokerAuthenticationResource items on this page"]
    pub value: Vec<BrokerAuthenticationResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BrokerAuthenticationResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BrokerAuthenticationResourceListResult {
    pub fn new(value: Vec<BrokerAuthenticationResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the BrokerAuthenticationResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerAuthenticationResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the BrokerAuthenticationResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BrokerAuthenticationResourceUpdateProperties>,
}
impl BrokerAuthenticationResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the BrokerAuthenticationResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerAuthenticationResourceUpdateProperties {
    #[doc = "The list of authentication methods supported by the Authentication Resource. For each array element, NOTE - Enum only authenticator type supported."]
    #[serde(
        rename = "authenticationMethods",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub authentication_methods: Vec<BrokerAuthenticatorMethods>,
    #[doc = "The array of listener Resources it supports."]
    #[serde(
        rename = "listenerRef",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub listener_ref: Vec<String>,
}
impl BrokerAuthenticationResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom Authentication properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerAuthenticatorCustomAuth {
    #[doc = "X509 Custom Authentication properties. NOTE - Enum only authenticator type supported at a time."]
    pub x509: BrokerAuthenticatorCustomAuthX509,
}
impl BrokerAuthenticatorCustomAuth {
    pub fn new(x509: BrokerAuthenticatorCustomAuthX509) -> Self {
        Self { x509 }
    }
}
#[doc = "X509 Custom Authentication properties. NOTE - Enum only authenticator type supported at a time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerAuthenticatorCustomAuthX509 {
    #[doc = "KeyVault certificate properties"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultCertificateProperties>,
    #[doc = "Secret where cert details are stored."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
impl BrokerAuthenticatorCustomAuthX509 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom method for BrokerAuthentication"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerAuthenticatorMethodCustom {
    #[doc = "Custom Authentication properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<BrokerAuthenticatorCustomAuth>,
    #[doc = "CA cert config map to use."]
    #[serde(rename = "caCertConfigMap", default, skip_serializing_if = "Option::is_none")]
    pub ca_cert_config_map: Option<String>,
    #[doc = "Endpoint to connect to."]
    pub endpoint: String,
    #[doc = "Configuration Headers to use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}
impl BrokerAuthenticatorMethodCustom {
    pub fn new(endpoint: String) -> Self {
        Self {
            auth: None,
            ca_cert_config_map: None,
            endpoint,
            headers: None,
        }
    }
}
#[doc = "Service Account Token for BrokerAuthentication"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerAuthenticatorMethodSat {
    #[doc = "List of allowed audience."]
    pub audiences: Vec<String>,
}
impl BrokerAuthenticatorMethodSat {
    pub fn new(audiences: Vec<String>) -> Self {
        Self { audiences }
    }
}
#[doc = "SVID for BrokerAuthentication"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerAuthenticatorMethodSvid {
    #[doc = "Mounted socket path for spiffe agent."]
    #[serde(rename = "agentSocketPath")]
    pub agent_socket_path: String,
    #[doc = "Maximum number of re-tries to fetch identity."]
    #[serde(rename = "identityMaxRetry", default, skip_serializing_if = "Option::is_none")]
    pub identity_max_retry: Option<i64>,
    #[doc = "Maximum time to wait before fetching identity again."]
    #[serde(rename = "identityWaitRetryMs", default, skip_serializing_if = "Option::is_none")]
    pub identity_wait_retry_ms: Option<i64>,
}
impl BrokerAuthenticatorMethodSvid {
    pub fn new(agent_socket_path: String) -> Self {
        Self {
            agent_socket_path,
            identity_max_retry: None,
            identity_wait_retry_ms: None,
        }
    }
}
#[doc = "UsernamePassword for BrokerAuthentication"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerAuthenticatorMethodUsernamePassword {
    #[doc = "KeyVault secret properties"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultSecretProperties>,
    #[doc = "Secret where username and password are stored."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
impl BrokerAuthenticatorMethodUsernamePassword {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "X509 for BrokerAuthentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerAuthenticatorMethodX509 {
    #[doc = "BrokerAuthenticatorMethodX509Attributes properties. NOTE - Enum only type supported at a time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BrokerAuthenticatorMethodX509Attributes>,
    #[doc = "Trusted client ca cert config map."]
    #[serde(rename = "trustedClientCaCertConfigMap", default, skip_serializing_if = "Option::is_none")]
    pub trusted_client_ca_cert_config_map: Option<String>,
}
impl BrokerAuthenticatorMethodX509 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BrokerAuthenticatorMethodX509Attributes properties. NOTE - Enum only type supported at a time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerAuthenticatorMethodX509Attributes {
    #[doc = "KeyVault secret properties"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultSecretProperties>,
    #[doc = "Secret where x509 attributes are stored."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
impl BrokerAuthenticatorMethodX509Attributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of different CrdAuthenticator methods of Broker Resource. NOTE Enum - Only one method is supported for each entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerAuthenticatorMethods {
    #[doc = "Custom method for BrokerAuthentication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<BrokerAuthenticatorMethodCustom>,
    #[doc = "Service Account Token for BrokerAuthentication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sat: Option<BrokerAuthenticatorMethodSat>,
    #[doc = "SVID for BrokerAuthentication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svid: Option<BrokerAuthenticatorMethodSvid>,
    #[doc = "UsernamePassword for BrokerAuthentication"]
    #[serde(rename = "usernamePassword", default, skip_serializing_if = "Option::is_none")]
    pub username_password: Option<BrokerAuthenticatorMethodUsernamePassword>,
    #[doc = "X509 for BrokerAuthentication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<BrokerAuthenticatorMethodX509>,
}
impl BrokerAuthenticatorMethods {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Broker Resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerAuthorizationProperties {
    #[doc = "Broker AuthorizationConfig properties"]
    #[serde(rename = "authorizationPolicies")]
    pub authorization_policies: AuthorizationConfig,
    #[doc = "The array of listener Resources it supports."]
    #[serde(rename = "listenerRef")]
    pub listener_ref: Vec<String>,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl BrokerAuthorizationProperties {
    pub fn new(authorization_policies: AuthorizationConfig, listener_ref: Vec<String>) -> Self {
        Self {
            authorization_policies,
            listener_ref,
            provisioning_state: None,
        }
    }
}
#[doc = "MQ broker/authorization resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerAuthorizationResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Broker Resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BrokerAuthorizationProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl BrokerAuthorizationResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a BrokerAuthorizationResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerAuthorizationResourceListResult {
    #[doc = "The BrokerAuthorizationResource items on this page"]
    pub value: Vec<BrokerAuthorizationResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BrokerAuthorizationResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BrokerAuthorizationResourceListResult {
    pub fn new(value: Vec<BrokerAuthorizationResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the BrokerAuthorizationResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerAuthorizationResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the BrokerAuthorizationResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BrokerAuthorizationResourceUpdateProperties>,
}
impl BrokerAuthorizationResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the BrokerAuthorizationResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerAuthorizationResourceUpdateProperties {
    #[doc = "Broker AuthorizationConfig properties"]
    #[serde(rename = "authorizationPolicies", default, skip_serializing_if = "Option::is_none")]
    pub authorization_policies: Option<AuthorizationConfig>,
    #[doc = "The array of listener Resources it supports."]
    #[serde(
        rename = "listenerRef",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub listener_ref: Vec<String>,
}
impl BrokerAuthorizationResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostics setting specific to Broker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerDiagnostics {
    #[doc = "Diagnostic Service endpoint"]
    #[serde(rename = "diagnosticServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_service_endpoint: Option<String>,
    #[doc = "Knob to enable/disable metrics. Default = true"]
    #[serde(rename = "enableMetrics", default, skip_serializing_if = "Option::is_none")]
    pub enable_metrics: Option<bool>,
    #[doc = "Enable self check on Broker via Probe."]
    #[serde(rename = "enableSelfCheck", default, skip_serializing_if = "Option::is_none")]
    pub enable_self_check: Option<bool>,
    #[doc = "Enable self tracing on the Broker so that every selfCheckFrequencySeconds a random message is traced even if it didn't have trace context."]
    #[serde(rename = "enableSelfTracing", default, skip_serializing_if = "Option::is_none")]
    pub enable_self_tracing: Option<bool>,
    #[doc = "Knob to enable/disable entire tracing infrastructure."]
    #[serde(rename = "enableTracing", default, skip_serializing_if = "Option::is_none")]
    pub enable_tracing: Option<bool>,
    #[doc = "Format for the logs generated."]
    #[serde(rename = "logFormat", default, skip_serializing_if = "Option::is_none")]
    pub log_format: Option<String>,
    #[doc = "Log level for the Broker."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[doc = "Maximum time for the CellMap to live."]
    #[serde(rename = "maxCellMapLifetime", default, skip_serializing_if = "Option::is_none")]
    pub max_cell_map_lifetime: Option<i64>,
    #[doc = "Metric update frequency in seconds."]
    #[serde(rename = "metricUpdateFrequencySeconds", default, skip_serializing_if = "Option::is_none")]
    pub metric_update_frequency_seconds: Option<i64>,
    #[doc = "Probe Image to run."]
    #[serde(rename = "probeImage", default, skip_serializing_if = "Option::is_none")]
    pub probe_image: Option<String>,
    #[doc = "Frequency for the self check to run."]
    #[serde(rename = "selfCheckFrequencySeconds", default, skip_serializing_if = "Option::is_none")]
    pub self_check_frequency_seconds: Option<i64>,
    #[doc = "Time out period of the self check."]
    #[serde(rename = "selfCheckTimeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub self_check_timeout_seconds: Option<i64>,
    #[doc = "The frequency at which selfTrace should run."]
    #[serde(rename = "selfTraceFrequencySeconds", default, skip_serializing_if = "Option::is_none")]
    pub self_trace_frequency_seconds: Option<i64>,
    #[doc = "The number of the spans generated by the Tracing."]
    #[serde(rename = "spanChannelCapacity", default, skip_serializing_if = "Option::is_none")]
    pub span_channel_capacity: Option<i64>,
}
impl BrokerDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Broker Listener Resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerListenerProperties {
    #[doc = "The flag for enabling Authentication rules on Listener Port."]
    #[serde(rename = "authenticationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub authentication_enabled: Option<bool>,
    #[doc = "The flag for enabling Authorization policies on Listener Port. false - AllowAll, true - Use Authorization resource rules if present."]
    #[serde(rename = "authorizationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub authorization_enabled: Option<bool>,
    #[doc = "The k8s cr/resource reference of mq/broker."]
    #[serde(rename = "brokerRef")]
    pub broker_ref: String,
    #[doc = "The node port to use on the Host node."]
    #[serde(rename = "nodePort", default, skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i32>,
    #[doc = "The port to start Listening for connections on."]
    pub port: i32,
    #[doc = "The service name to expose Listener port on."]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[doc = "The Kubernetes Service type to deploy for Listener."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<broker_listener_properties::ServiceType>,
    #[doc = "Collection of different TLS types, NOTE- Enum at a time only one of them needs to be supported"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsCertMethod>,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl BrokerListenerProperties {
    pub fn new(broker_ref: String, port: i32) -> Self {
        Self {
            authentication_enabled: None,
            authorization_enabled: None,
            broker_ref,
            node_port: None,
            port,
            service_name: None,
            service_type: None,
            tls: None,
            provisioning_state: None,
        }
    }
}
pub mod broker_listener_properties {
    use super::*;
    #[doc = "The Kubernetes Service type to deploy for Listener."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceType")]
    pub enum ServiceType {
        #[serde(rename = "clusterIp")]
        ClusterIp,
        #[serde(rename = "loadBalancer")]
        LoadBalancer,
        #[serde(rename = "nodePort")]
        NodePort,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ClusterIp => serializer.serialize_unit_variant("ServiceType", 0u32, "clusterIp"),
                Self::LoadBalancer => serializer.serialize_unit_variant("ServiceType", 1u32, "loadBalancer"),
                Self::NodePort => serializer.serialize_unit_variant("ServiceType", 2u32, "nodePort"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ServiceType {
        fn default() -> Self {
            Self::ClusterIp
        }
    }
}
#[doc = "MQ broker/listener resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerListenerResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Broker Listener Resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BrokerListenerProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl BrokerListenerResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a BrokerListenerResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerListenerResourceListResult {
    #[doc = "The BrokerListenerResource items on this page"]
    pub value: Vec<BrokerListenerResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BrokerListenerResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BrokerListenerResourceListResult {
    pub fn new(value: Vec<BrokerListenerResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the BrokerListenerResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerListenerResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the BrokerListenerResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BrokerListenerResourceUpdateProperties>,
}
impl BrokerListenerResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the BrokerListenerResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerListenerResourceUpdateProperties {
    #[doc = "The flag for enabling Authentication rules on Listener Port."]
    #[serde(rename = "authenticationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub authentication_enabled: Option<bool>,
    #[doc = "The flag for enabling Authorization policies on Listener Port. false - AllowAll, true - Use Authorization resource rules if present."]
    #[serde(rename = "authorizationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub authorization_enabled: Option<bool>,
    #[doc = "The k8s cr/resource reference of mq/broker."]
    #[serde(rename = "brokerRef", default, skip_serializing_if = "Option::is_none")]
    pub broker_ref: Option<String>,
    #[doc = "The node port to use on the Host node."]
    #[serde(rename = "nodePort", default, skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i32>,
    #[doc = "The port to start Listening for connections on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The service name to expose Listener port on."]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[doc = "The Kubernetes Service type to deploy for Listener."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<broker_listener_resource_update_properties::ServiceType>,
    #[doc = "Collection of different TLS types, NOTE- Enum at a time only one of them needs to be supported"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsCertMethodUpdate>,
}
impl BrokerListenerResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod broker_listener_resource_update_properties {
    use super::*;
    #[doc = "The Kubernetes Service type to deploy for Listener."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceType")]
    pub enum ServiceType {
        #[serde(rename = "clusterIp")]
        ClusterIp,
        #[serde(rename = "loadBalancer")]
        LoadBalancer,
        #[serde(rename = "nodePort")]
        NodePort,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ClusterIp => serializer.serialize_unit_variant("ServiceType", 0u32, "clusterIp"),
                Self::LoadBalancer => serializer.serialize_unit_variant("ServiceType", 1u32, "loadBalancer"),
                Self::NodePort => serializer.serialize_unit_variant("ServiceType", 2u32, "nodePort"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ServiceType {
        fn default() -> Self {
            Self::ClusterIp
        }
    }
}
#[doc = "The memory profile settings of the Broker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BrokerMemoryProfile")]
pub enum BrokerMemoryProfile {
    #[serde(rename = "tiny")]
    Tiny,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BrokerMemoryProfile {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BrokerMemoryProfile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BrokerMemoryProfile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Tiny => serializer.serialize_unit_variant("BrokerMemoryProfile", 0u32, "tiny"),
            Self::Low => serializer.serialize_unit_variant("BrokerMemoryProfile", 1u32, "low"),
            Self::Medium => serializer.serialize_unit_variant("BrokerMemoryProfile", 2u32, "medium"),
            Self::High => serializer.serialize_unit_variant("BrokerMemoryProfile", 3u32, "high"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Broker Resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerProperties {
    #[doc = "Defines the Docker image details"]
    #[serde(rename = "authImage")]
    pub auth_image: ContainerImage,
    #[doc = "Defines the Docker image details"]
    #[serde(rename = "brokerImage")]
    pub broker_image: ContainerImage,
    #[doc = "Defines the Node Tolerations details"]
    #[serde(rename = "brokerNodeTolerations", default, skip_serializing_if = "Option::is_none")]
    pub broker_node_tolerations: Option<NodeTolerations>,
    #[doc = "Cardinality properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<Cardinality>,
    #[doc = "Diagnostics setting specific to Broker"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<BrokerDiagnostics>,
    #[doc = "DiskBackedMessageBufferSettings properties"]
    #[serde(rename = "diskBackedMessageBufferSettings", default, skip_serializing_if = "Option::is_none")]
    pub disk_backed_message_buffer_settings: Option<DiskBackedMessageBufferSettings>,
    #[doc = "The setting to enable or disable encryption of internal Traffic."]
    #[serde(rename = "encryptInternalTraffic", default, skip_serializing_if = "Option::is_none")]
    pub encrypt_internal_traffic: Option<bool>,
    #[doc = "Defines the Docker image details"]
    #[serde(rename = "healthManagerImage")]
    pub health_manager_image: ContainerImage,
    #[doc = "Defines the Node Tolerations details"]
    #[serde(rename = "healthManagerNodeTolerations", default, skip_serializing_if = "Option::is_none")]
    pub health_manager_node_tolerations: Option<NodeTolerations>,
    #[doc = "Cert Manager CA Cert properties"]
    #[serde(rename = "internalCerts", default, skip_serializing_if = "Option::is_none")]
    pub internal_certs: Option<CertManagerCertOptions>,
    #[doc = "Memory profile of broker."]
    #[serde(rename = "memoryProfile", default, skip_serializing_if = "Option::is_none")]
    pub memory_profile: Option<broker_properties::MemoryProfile>,
    #[doc = "The enum defining run mode of the broker deployment"]
    pub mode: RunMode,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl BrokerProperties {
    pub fn new(auth_image: ContainerImage, broker_image: ContainerImage, health_manager_image: ContainerImage, mode: RunMode) -> Self {
        Self {
            auth_image,
            broker_image,
            broker_node_tolerations: None,
            cardinality: None,
            diagnostics: None,
            disk_backed_message_buffer_settings: None,
            encrypt_internal_traffic: None,
            health_manager_image,
            health_manager_node_tolerations: None,
            internal_certs: None,
            memory_profile: None,
            mode,
            provisioning_state: None,
        }
    }
}
pub mod broker_properties {
    use super::*;
    #[doc = "Memory profile of broker."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemoryProfile")]
    pub enum MemoryProfile {
        #[serde(rename = "tiny")]
        Tiny,
        #[serde(rename = "low")]
        Low,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "high")]
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemoryProfile {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemoryProfile {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemoryProfile {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tiny => serializer.serialize_unit_variant("MemoryProfile", 0u32, "tiny"),
                Self::Low => serializer.serialize_unit_variant("MemoryProfile", 1u32, "low"),
                Self::Medium => serializer.serialize_unit_variant("MemoryProfile", 2u32, "medium"),
                Self::High => serializer.serialize_unit_variant("MemoryProfile", 3u32, "high"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for MemoryProfile {
        fn default() -> Self {
            Self::Medium
        }
    }
}
#[doc = "MQ broker resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Broker Resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BrokerProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl BrokerResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a BrokerResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerResourceListResult {
    #[doc = "The BrokerResource items on this page"]
    pub value: Vec<BrokerResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BrokerResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BrokerResourceListResult {
    pub fn new(value: Vec<BrokerResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the BrokerResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the BrokerResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BrokerResourceUpdateProperties>,
}
impl BrokerResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the BrokerResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrokerResourceUpdateProperties {
    #[doc = "Defines the Docker image details"]
    #[serde(rename = "authImage", default, skip_serializing_if = "Option::is_none")]
    pub auth_image: Option<ContainerImageUpdate>,
    #[doc = "Defines the Docker image details"]
    #[serde(rename = "brokerImage", default, skip_serializing_if = "Option::is_none")]
    pub broker_image: Option<ContainerImageUpdate>,
    #[doc = "Defines the Node Tolerations details"]
    #[serde(rename = "brokerNodeTolerations", default, skip_serializing_if = "Option::is_none")]
    pub broker_node_tolerations: Option<NodeTolerationsUpdate>,
    #[doc = "Cardinality properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<CardinalityUpdate>,
    #[doc = "Diagnostics setting specific to Broker"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<BrokerDiagnostics>,
    #[doc = "DiskBackedMessageBufferSettings properties"]
    #[serde(rename = "diskBackedMessageBufferSettings", default, skip_serializing_if = "Option::is_none")]
    pub disk_backed_message_buffer_settings: Option<DiskBackedMessageBufferSettingsUpdate>,
    #[doc = "The setting to enable or disable encryption of internal Traffic."]
    #[serde(rename = "encryptInternalTraffic", default, skip_serializing_if = "Option::is_none")]
    pub encrypt_internal_traffic: Option<bool>,
    #[doc = "Defines the Docker image details"]
    #[serde(rename = "healthManagerImage", default, skip_serializing_if = "Option::is_none")]
    pub health_manager_image: Option<ContainerImageUpdate>,
    #[doc = "Defines the Node Tolerations details"]
    #[serde(rename = "healthManagerNodeTolerations", default, skip_serializing_if = "Option::is_none")]
    pub health_manager_node_tolerations: Option<NodeTolerationsUpdate>,
    #[doc = "Cert Manager CA Cert properties"]
    #[serde(rename = "internalCerts", default, skip_serializing_if = "Option::is_none")]
    pub internal_certs: Option<CertManagerCertOptionsUpdate>,
    #[doc = "Memory profile of broker."]
    #[serde(rename = "memoryProfile", default, skip_serializing_if = "Option::is_none")]
    pub memory_profile: Option<broker_resource_update_properties::MemoryProfile>,
    #[doc = "The enum defining run mode of the broker deployment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<RunMode>,
}
impl BrokerResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod broker_resource_update_properties {
    use super::*;
    #[doc = "Memory profile of broker."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemoryProfile")]
    pub enum MemoryProfile {
        #[serde(rename = "tiny")]
        Tiny,
        #[serde(rename = "low")]
        Low,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "high")]
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemoryProfile {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemoryProfile {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemoryProfile {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tiny => serializer.serialize_unit_variant("MemoryProfile", 0u32, "tiny"),
                Self::Low => serializer.serialize_unit_variant("MemoryProfile", 1u32, "low"),
                Self::Medium => serializer.serialize_unit_variant("MemoryProfile", 2u32, "medium"),
                Self::High => serializer.serialize_unit_variant("MemoryProfile", 3u32, "high"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for MemoryProfile {
        fn default() -> Self {
            Self::Medium
        }
    }
}
#[doc = "Cardinality properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cardinality {
    #[doc = "Desired properties of the Frontend Instances of the DMQTT Broker"]
    #[serde(rename = "backendChain")]
    pub backend_chain: BackendChain,
    #[doc = "Desired properties of the Frontend Instances of the DMQTT Broker"]
    pub frontend: Frontend,
}
impl Cardinality {
    pub fn new(backend_chain: BackendChain, frontend: Frontend) -> Self {
        Self { backend_chain, frontend }
    }
}
#[doc = "Cardinality properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CardinalityUpdate {
    #[doc = "Desired properties of the Frontend Instances of the DMQTT Broker"]
    #[serde(rename = "backendChain", default, skip_serializing_if = "Option::is_none")]
    pub backend_chain: Option<BackendChainUpdate>,
    #[doc = "Desired properties of the Frontend Instances of the DMQTT Broker"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frontend: Option<FrontendUpdate>,
}
impl CardinalityUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cert Manager CA Cert properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertManagerCertOptions {
    #[doc = "Duration of CA cert."]
    pub duration: String,
    #[doc = "Cert Manager private key properties"]
    #[serde(rename = "privateKey")]
    pub private_key: CertManagerPrivateKey,
    #[doc = "Renew before time of CA cert."]
    #[serde(rename = "renewBefore")]
    pub renew_before: String,
}
impl CertManagerCertOptions {
    pub fn new(duration: String, private_key: CertManagerPrivateKey, renew_before: String) -> Self {
        Self {
            duration,
            private_key,
            renew_before,
        }
    }
}
#[doc = "Cert Manager CA Cert properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertManagerCertOptionsUpdate {
    #[doc = "Duration of CA cert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Cert Manager private key properties"]
    #[serde(rename = "privateKey", default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<CertManagerPrivateKeyUpdate>,
    #[doc = "Renew before time of CA cert."]
    #[serde(rename = "renewBefore", default, skip_serializing_if = "Option::is_none")]
    pub renew_before: Option<String>,
}
impl CertManagerCertOptionsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cert-Manager issuerRef properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertManagerIssuerRef {
    #[doc = "group of issuer."]
    pub group: String,
    #[doc = "kind of issuer (Issuer or ClusterIssuer)."]
    pub kind: String,
    #[doc = "name of issuer."]
    pub name: String,
}
impl CertManagerIssuerRef {
    pub fn new(group: String, kind: String, name: String) -> Self {
        Self { group, kind, name }
    }
}
#[doc = "Cert-Manager issuerRef properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertManagerIssuerRefUpdate {
    #[doc = "group of issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[doc = "kind of issuer (Issuer or ClusterIssuer)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "name of issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CertManagerIssuerRefUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cert Manager private key properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertManagerPrivateKey {
    #[doc = "algorithm for private key."]
    pub algorithm: String,
    #[doc = "cert-manager rotationPolicy."]
    #[serde(rename = "rotationPolicy")]
    pub rotation_policy: String,
    #[doc = "size of private key."]
    pub size: i32,
}
impl CertManagerPrivateKey {
    pub fn new(algorithm: String, rotation_policy: String, size: i32) -> Self {
        Self {
            algorithm,
            rotation_policy,
            size,
        }
    }
}
#[doc = "Cert Manager private key properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertManagerPrivateKeyUpdate {
    #[doc = "algorithm for private key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[doc = "cert-manager rotationPolicy."]
    #[serde(rename = "rotationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub rotation_policy: Option<String>,
    #[doc = "size of private key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
impl CertManagerPrivateKeyUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the Docker image details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImage {
    #[doc = "Image pull policy."]
    #[serde(rename = "pullPolicy", default, skip_serializing_if = "Option::is_none")]
    pub pull_policy: Option<String>,
    #[doc = "Image pull secrets."]
    #[serde(rename = "pullSecrets", default, skip_serializing_if = "Option::is_none")]
    pub pull_secrets: Option<String>,
    #[doc = "The Docker image name."]
    pub repository: String,
    #[doc = "The Docker  image tag."]
    pub tag: String,
}
impl ContainerImage {
    pub fn new(repository: String, tag: String) -> Self {
        Self {
            pull_policy: None,
            pull_secrets: None,
            repository,
            tag,
        }
    }
}
#[doc = "Defines the Docker image details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerImageUpdate {
    #[doc = "Image pull policy."]
    #[serde(rename = "pullPolicy", default, skip_serializing_if = "Option::is_none")]
    pub pull_policy: Option<String>,
    #[doc = "Image pull secrets."]
    #[serde(rename = "pullSecrets", default, skip_serializing_if = "Option::is_none")]
    pub pull_secrets: Option<String>,
    #[doc = "The Docker image name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[doc = "The Docker  image tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
impl ContainerImageUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DataLake connector map route properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeConnectorMap {
    #[doc = "Allowed latency for transferring data."]
    #[serde(rename = "allowedLatencySecs")]
    pub allowed_latency_secs: i32,
    #[doc = "Client Id to use."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Maximum messages to send per Batch."]
    #[serde(rename = "maxMessagesPerBatch")]
    pub max_messages_per_batch: i64,
    #[doc = "Message payload type."]
    #[serde(rename = "messagePayloadType")]
    pub message_payload_type: String,
    #[doc = "Mqtt source topic."]
    #[serde(rename = "mqttSourceTopic")]
    pub mqtt_source_topic: String,
    #[doc = "Quality of Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    #[doc = "Delta table properties"]
    pub table: DeltaTable,
}
impl DataLakeConnectorMap {
    pub fn new(
        allowed_latency_secs: i32,
        client_id: String,
        max_messages_per_batch: i64,
        message_payload_type: String,
        mqtt_source_topic: String,
        table: DeltaTable,
    ) -> Self {
        Self {
            allowed_latency_secs,
            client_id,
            max_messages_per_batch,
            message_payload_type,
            mqtt_source_topic,
            qos: None,
            table,
        }
    }
}
#[doc = "DataLake connector map route properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeConnectorMapUpdate {
    #[doc = "Allowed latency for transferring data."]
    #[serde(rename = "allowedLatencySecs", default, skip_serializing_if = "Option::is_none")]
    pub allowed_latency_secs: Option<i32>,
    #[doc = "Client Id to use."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Maximum messages to send per Batch."]
    #[serde(rename = "maxMessagesPerBatch", default, skip_serializing_if = "Option::is_none")]
    pub max_messages_per_batch: Option<i64>,
    #[doc = "Message payload type."]
    #[serde(rename = "messagePayloadType", default, skip_serializing_if = "Option::is_none")]
    pub message_payload_type: Option<String>,
    #[doc = "Mqtt source topic."]
    #[serde(rename = "mqttSourceTopic", default, skip_serializing_if = "Option::is_none")]
    pub mqtt_source_topic: Option<String>,
    #[doc = "Quality of Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    #[doc = "Delta table properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<DeltaTableUpdate>,
}
impl DataLakeConnectorMapUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MQ DataLakeConnector  Resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeConnectorProperties {
    #[doc = "DataLake possible database formats"]
    #[serde(rename = "databaseFormat")]
    pub database_format: DataLakeDatabaseFormat,
    #[doc = "Defines the Docker image details"]
    pub image: ContainerImage,
    #[doc = "The number of DataLakeConnector pods to spin up."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<i32>,
    #[doc = "Mqtt Local Broker ConnectionSpec details"]
    #[serde(rename = "localBrokerConnection", default, skip_serializing_if = "Option::is_none")]
    pub local_broker_connection: Option<LocalBrokerConnectionSpec>,
    #[doc = "The log level of the DataLake Connector instances."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[doc = "Defines the Node Tolerations details"]
    #[serde(rename = "nodeTolerations", default, skip_serializing_if = "Option::is_none")]
    pub node_tolerations: Option<NodeTolerations>,
    #[doc = "Mqtt Protocol types"]
    pub protocol: MqttProtocol,
    #[doc = "Target storage for the DataLake. NOTE - Enum only storage is supported at a time."]
    pub target: DataLakeTargetStorage,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DataLakeConnectorProperties {
    pub fn new(
        database_format: DataLakeDatabaseFormat,
        image: ContainerImage,
        protocol: MqttProtocol,
        target: DataLakeTargetStorage,
    ) -> Self {
        Self {
            database_format,
            image,
            instances: None,
            local_broker_connection: None,
            log_level: None,
            node_tolerations: None,
            protocol,
            target,
            provisioning_state: None,
        }
    }
}
#[doc = "MQ dataLakeConnector resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeConnectorResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "MQ DataLakeConnector  Resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeConnectorProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl DataLakeConnectorResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a DataLakeConnectorResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeConnectorResourceListResult {
    #[doc = "The DataLakeConnectorResource items on this page"]
    pub value: Vec<DataLakeConnectorResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataLakeConnectorResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataLakeConnectorResourceListResult {
    pub fn new(value: Vec<DataLakeConnectorResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the DataLakeConnectorResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeConnectorResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the DataLakeConnectorResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeConnectorResourceUpdateProperties>,
}
impl DataLakeConnectorResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the DataLakeConnectorResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeConnectorResourceUpdateProperties {
    #[doc = "DataLake possible database formats"]
    #[serde(rename = "databaseFormat", default, skip_serializing_if = "Option::is_none")]
    pub database_format: Option<DataLakeDatabaseFormat>,
    #[doc = "Defines the Docker image details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ContainerImageUpdate>,
    #[doc = "The number of DataLakeConnector pods to spin up."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<i32>,
    #[doc = "Mqtt Local Broker ConnectionSpec details"]
    #[serde(rename = "localBrokerConnection", default, skip_serializing_if = "Option::is_none")]
    pub local_broker_connection: Option<LocalBrokerConnectionSpecUpdate>,
    #[doc = "The log level of the DataLake Connector instances."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[doc = "Defines the Node Tolerations details"]
    #[serde(rename = "nodeTolerations", default, skip_serializing_if = "Option::is_none")]
    pub node_tolerations: Option<NodeTolerationsUpdate>,
    #[doc = "Mqtt Protocol types"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<MqttProtocol>,
    #[doc = "Target storage for the DataLake. NOTE - Enum only storage is supported at a time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<DataLakeTargetStorageUpdate>,
}
impl DataLakeConnectorResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MQ DataLakeConnector TopicMap Resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeConnectorTopicMapProperties {
    #[doc = "DataLake Connector CRD to use."]
    #[serde(rename = "dataLakeConnectorRef")]
    pub data_lake_connector_ref: String,
    #[doc = "DataLake connector map route properties"]
    pub mapping: DataLakeConnectorMap,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DataLakeConnectorTopicMapProperties {
    pub fn new(data_lake_connector_ref: String, mapping: DataLakeConnectorMap) -> Self {
        Self {
            data_lake_connector_ref,
            mapping,
            provisioning_state: None,
        }
    }
}
#[doc = "DataLake possible database formats"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataLakeDatabaseFormat")]
pub enum DataLakeDatabaseFormat {
    #[serde(rename = "delta")]
    Delta,
    #[serde(rename = "parquet")]
    Parquet,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataLakeDatabaseFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataLakeDatabaseFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataLakeDatabaseFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Delta => serializer.serialize_unit_variant("DataLakeDatabaseFormat", 0u32, "delta"),
            Self::Parquet => serializer.serialize_unit_variant("DataLakeDatabaseFormat", 1u32, "parquet"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DataLake Fabric Storage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeFabricStorage {
    #[doc = "DataLake Fabric Storage authentication details."]
    pub authentication: DataLakeFabricStorageAuthentication,
    #[doc = "DataLake fabric storage endpoint to use."]
    pub endpoint: String,
    #[doc = "Fabric path type to use."]
    #[serde(rename = "fabricPath")]
    pub fabric_path: FabricPathType,
    #[doc = "Fabric one lake guids."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guids: Option<FabricGuids>,
    #[doc = "Fabric one lake names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<FabricNames>,
}
impl DataLakeFabricStorage {
    pub fn new(authentication: DataLakeFabricStorageAuthentication, endpoint: String, fabric_path: FabricPathType) -> Self {
        Self {
            authentication,
            endpoint,
            fabric_path,
            guids: None,
            names: None,
        }
    }
}
#[doc = "DataLake Fabric Storage authentication details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeFabricStorageAuthentication {
    #[doc = "Managed identity authentication details."]
    #[serde(rename = "systemAssignedManagedIdentity")]
    pub system_assigned_managed_identity: ManagedIdentityAuthentication,
}
impl DataLakeFabricStorageAuthentication {
    pub fn new(system_assigned_managed_identity: ManagedIdentityAuthentication) -> Self {
        Self {
            system_assigned_managed_identity,
        }
    }
}
#[doc = "DataLake Fabric Storage authentication details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeFabricStorageAuthenticationUpdate {
    #[doc = "Managed identity authentication details."]
    #[serde(rename = "systemAssignedManagedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub system_assigned_managed_identity: Option<ManagedIdentityAuthenticationUpdate>,
}
impl DataLakeFabricStorageAuthenticationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DataLake Fabric Storage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeFabricStorageUpdate {
    #[doc = "DataLake Fabric Storage authentication details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<DataLakeFabricStorageAuthenticationUpdate>,
    #[doc = "DataLake fabric storage endpoint to use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Fabric path type to use."]
    #[serde(rename = "fabricPath", default, skip_serializing_if = "Option::is_none")]
    pub fabric_path: Option<FabricPathType>,
    #[doc = "Fabric one lake guids."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guids: Option<FabricGuidsUpdate>,
    #[doc = "Fabric one lake names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<FabricNamesUpdate>,
}
impl DataLakeFabricStorageUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DataLake Local Storage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeLocalStorage {
    #[doc = "Volume name to write to."]
    #[serde(rename = "volumeName")]
    pub volume_name: String,
}
impl DataLakeLocalStorage {
    pub fn new(volume_name: String) -> Self {
        Self { volume_name }
    }
}
#[doc = "DataLake Local Storage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeLocalStorageUpdate {
    #[doc = "Volume name to write to."]
    #[serde(rename = "volumeName", default, skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
}
impl DataLakeLocalStorageUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DataLake Service Storage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeServiceStorage {
    #[doc = "DataLake service storage endpoint to use."]
    pub endpoint: String,
    #[doc = "DataLake Service Storage authentication details. NOTE - Enum only one method is supported."]
    pub authentication: DataLakeServiceStorageAuthentication,
}
impl DataLakeServiceStorage {
    pub fn new(endpoint: String, authentication: DataLakeServiceStorageAuthentication) -> Self {
        Self { endpoint, authentication }
    }
}
#[doc = "DataLake Service Storage authentication details. NOTE - Enum only one method is supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeServiceStorageAuthentication {
    #[doc = "Access token secret name."]
    #[serde(rename = "accessTokenSecretName", default, skip_serializing_if = "Option::is_none")]
    pub access_token_secret_name: Option<String>,
    #[doc = "Managed identity authentication details."]
    #[serde(rename = "systemAssignedManagedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub system_assigned_managed_identity: Option<ManagedIdentityAuthentication>,
}
impl DataLakeServiceStorageAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DataLake Service Storage authentication details. NOTE - Enum only one method is supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeServiceStorageAuthenticationUpdate {
    #[doc = "Access token secret name."]
    #[serde(rename = "accessTokenSecretName", default, skip_serializing_if = "Option::is_none")]
    pub access_token_secret_name: Option<String>,
    #[doc = "Managed identity authentication details."]
    #[serde(rename = "systemAssignedManagedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub system_assigned_managed_identity: Option<ManagedIdentityAuthenticationUpdate>,
}
impl DataLakeServiceStorageAuthenticationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DataLake Service Storage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeServiceStorageUpdate {
    #[doc = "DataLake service storage endpoint to use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "DataLake Service Storage authentication details. NOTE - Enum only one method is supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<DataLakeServiceStorageAuthenticationUpdate>,
}
impl DataLakeServiceStorageUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Target storage for the DataLake. NOTE - Enum only storage is supported at a time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeTargetStorage {
    #[doc = "DataLake Service Storage details."]
    #[serde(rename = "datalakeStorage", default, skip_serializing_if = "Option::is_none")]
    pub datalake_storage: Option<DataLakeServiceStorage>,
    #[doc = "DataLake Local Storage details."]
    #[serde(rename = "localStorage", default, skip_serializing_if = "Option::is_none")]
    pub local_storage: Option<DataLakeLocalStorage>,
    #[doc = "DataLake Fabric Storage details."]
    #[serde(rename = "fabricOneLake", default, skip_serializing_if = "Option::is_none")]
    pub fabric_one_lake: Option<DataLakeFabricStorage>,
}
impl DataLakeTargetStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Target storage for the DataLake. NOTE - Enum only storage is supported at a time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeTargetStorageUpdate {
    #[doc = "DataLake Service Storage details."]
    #[serde(rename = "datalakeStorage", default, skip_serializing_if = "Option::is_none")]
    pub datalake_storage: Option<DataLakeServiceStorageUpdate>,
    #[doc = "DataLake Local Storage details."]
    #[serde(rename = "localStorage", default, skip_serializing_if = "Option::is_none")]
    pub local_storage: Option<DataLakeLocalStorageUpdate>,
    #[doc = "DataLake Fabric Storage details."]
    #[serde(rename = "fabricOneLake", default, skip_serializing_if = "Option::is_none")]
    pub fabric_one_lake: Option<DataLakeFabricStorageUpdate>,
}
impl DataLakeTargetStorageUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MQ dataLakeConnector/topicMap resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeTopicMapResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "MQ DataLakeConnector TopicMap Resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeConnectorTopicMapProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl DataLakeTopicMapResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a DataLakeTopicMapResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeTopicMapResourceListResult {
    #[doc = "The DataLakeTopicMapResource items on this page"]
    pub value: Vec<DataLakeTopicMapResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataLakeTopicMapResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataLakeTopicMapResourceListResult {
    pub fn new(value: Vec<DataLakeTopicMapResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the DataLakeTopicMapResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeTopicMapResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the DataLakeTopicMapResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeTopicMapResourceUpdateProperties>,
}
impl DataLakeTopicMapResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the DataLakeTopicMapResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeTopicMapResourceUpdateProperties {
    #[doc = "DataLake Connector CRD to use."]
    #[serde(rename = "dataLakeConnectorRef", default, skip_serializing_if = "Option::is_none")]
    pub data_lake_connector_ref: Option<String>,
    #[doc = "DataLake connector map route properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping: Option<DataLakeConnectorMapUpdate>,
}
impl DataLakeTopicMapResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Delta table properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeltaTable {
    #[doc = "Schema list supported."]
    pub schema: Vec<DeltaTableSchema>,
    #[doc = "Delta table name."]
    #[serde(rename = "tableName")]
    pub table_name: String,
    #[doc = "Delta table path."]
    #[serde(rename = "tablePath", default, skip_serializing_if = "Option::is_none")]
    pub table_path: Option<String>,
}
impl DeltaTable {
    pub fn new(schema: Vec<DeltaTableSchema>, table_name: String) -> Self {
        Self {
            schema,
            table_name,
            table_path: None,
        }
    }
}
#[doc = "Delta table format properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeltaTableFormatEnum")]
pub enum DeltaTableFormatEnum {
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "int8")]
    Int8,
    #[serde(rename = "int16")]
    Int16,
    #[serde(rename = "int32")]
    Int32,
    #[serde(rename = "uInt8")]
    UInt8,
    #[serde(rename = "uInt16")]
    UInt16,
    #[serde(rename = "uInt32")]
    UInt32,
    #[serde(rename = "uInt64")]
    UInt64,
    #[serde(rename = "float16")]
    Float16,
    #[serde(rename = "float32")]
    Float32,
    #[serde(rename = "float64")]
    Float64,
    #[serde(rename = "date32")]
    Date32,
    #[serde(rename = "date64")]
    Date64,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "utf8")]
    Utf8,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeltaTableFormatEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeltaTableFormatEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeltaTableFormatEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Boolean => serializer.serialize_unit_variant("DeltaTableFormatEnum", 0u32, "boolean"),
            Self::Int8 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 1u32, "int8"),
            Self::Int16 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 2u32, "int16"),
            Self::Int32 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 3u32, "int32"),
            Self::UInt8 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 4u32, "uInt8"),
            Self::UInt16 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 5u32, "uInt16"),
            Self::UInt32 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 6u32, "uInt32"),
            Self::UInt64 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 7u32, "uInt64"),
            Self::Float16 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 8u32, "float16"),
            Self::Float32 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 9u32, "float32"),
            Self::Float64 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 10u32, "float64"),
            Self::Date32 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 11u32, "date32"),
            Self::Date64 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 12u32, "date64"),
            Self::Binary => serializer.serialize_unit_variant("DeltaTableFormatEnum", 13u32, "binary"),
            Self::Utf8 => serializer.serialize_unit_variant("DeltaTableFormatEnum", 14u32, "utf8"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Delta table schema properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeltaTableSchema {
    #[doc = "Delta table format properties"]
    pub format: DeltaTableFormatEnum,
    #[doc = "Delta table schema mapping."]
    pub mapping: String,
    #[doc = "Delta table schema name."]
    pub name: String,
    #[doc = "Delta table schema optional."]
    pub optional: bool,
}
impl DeltaTableSchema {
    pub fn new(format: DeltaTableFormatEnum, mapping: String, name: String, optional: bool) -> Self {
        Self {
            format,
            mapping,
            name,
            optional,
        }
    }
}
#[doc = "Delta table properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeltaTableUpdate {
    #[doc = "Schema list supported."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub schema: Vec<DeltaTableSchema>,
    #[doc = "Delta table name."]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "Delta table path."]
    #[serde(rename = "tablePath", default, skip_serializing_if = "Option::is_none")]
    pub table_path: Option<String>,
}
impl DeltaTableUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MQ Diagnostic Services Resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticServiceProperties {
    #[doc = "The frequency at which the data will be exported."]
    #[serde(rename = "dataExportFrequencySeconds", default, skip_serializing_if = "Option::is_none")]
    pub data_export_frequency_seconds: Option<i32>,
    #[doc = "Defines the Docker image details"]
    pub image: ContainerImage,
    #[doc = "The format for the logs generated."]
    #[serde(rename = "logFormat", default, skip_serializing_if = "Option::is_none")]
    pub log_format: Option<String>,
    #[doc = "The format for the logs generated."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[doc = "The maximum data stored in MiB."]
    #[serde(rename = "maxDataStorageSize", default, skip_serializing_if = "Option::is_none")]
    pub max_data_storage_size: Option<i64>,
    #[doc = "The port at which metrics is exposed."]
    #[serde(rename = "metricsPort", default, skip_serializing_if = "Option::is_none")]
    pub metrics_port: Option<i32>,
    #[doc = "The destination to collect traces. Diagnostic service will push traces to this endpoint"]
    #[serde(rename = "openTelemetryTracesCollectorAddr", default, skip_serializing_if = "Option::is_none")]
    pub open_telemetry_traces_collector_addr: Option<String>,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Metric inactivity timeout."]
    #[serde(rename = "staleDataTimeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub stale_data_timeout_seconds: Option<i32>,
}
impl DiagnosticServiceProperties {
    pub fn new(image: ContainerImage) -> Self {
        Self {
            data_export_frequency_seconds: None,
            image,
            log_format: None,
            log_level: None,
            max_data_storage_size: None,
            metrics_port: None,
            open_telemetry_traces_collector_addr: None,
            provisioning_state: None,
            stale_data_timeout_seconds: None,
        }
    }
}
#[doc = "MQ diagnostic services resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticServiceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "MQ Diagnostic Services Resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticServiceProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl DiagnosticServiceResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a DiagnosticServiceResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticServiceResourceListResult {
    #[doc = "The DiagnosticServiceResource items on this page"]
    pub value: Vec<DiagnosticServiceResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiagnosticServiceResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DiagnosticServiceResourceListResult {
    pub fn new(value: Vec<DiagnosticServiceResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the DiagnosticServiceResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticServiceResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the DiagnosticServiceResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticServiceResourceUpdateProperties>,
}
impl DiagnosticServiceResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the DiagnosticServiceResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticServiceResourceUpdateProperties {
    #[doc = "The frequency at which the data will be exported."]
    #[serde(rename = "dataExportFrequencySeconds", default, skip_serializing_if = "Option::is_none")]
    pub data_export_frequency_seconds: Option<i32>,
    #[doc = "Defines the Docker image details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ContainerImageUpdate>,
    #[doc = "The format for the logs generated."]
    #[serde(rename = "logFormat", default, skip_serializing_if = "Option::is_none")]
    pub log_format: Option<String>,
    #[doc = "The format for the logs generated."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[doc = "The maximum data stored in MiB."]
    #[serde(rename = "maxDataStorageSize", default, skip_serializing_if = "Option::is_none")]
    pub max_data_storage_size: Option<i64>,
    #[doc = "The port at which metrics is exposed."]
    #[serde(rename = "metricsPort", default, skip_serializing_if = "Option::is_none")]
    pub metrics_port: Option<i32>,
    #[doc = "The destination to collect traces. Diagnostic service will push traces to this endpoint"]
    #[serde(rename = "openTelemetryTracesCollectorAddr", default, skip_serializing_if = "Option::is_none")]
    pub open_telemetry_traces_collector_addr: Option<String>,
    #[doc = "Metric inactivity timeout."]
    #[serde(rename = "staleDataTimeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub stale_data_timeout_seconds: Option<i32>,
}
impl DiagnosticServiceResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DiskBackedMessageBufferSettings properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskBackedMessageBufferSettings {
    #[doc = "VolumeClaimSpec properties"]
    #[serde(rename = "ephemeralVolumeClaimSpec", default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_volume_claim_spec: Option<VolumeClaimSpec>,
    #[doc = "The max size of the message buffer on disk. If a PVC template is specified using one of ephemeralVolumeClaimSpec or persistentVolumeClaimSpec, then this size is used as the request and limit sizes of that template. If neither ephemeralVolumeClaimSpec nor persistentVolumeClaimSpec are specified, then an emptyDir volume is mounted with this size as its limit. See <https://kubernetes.io/docs/concepts/storage/volumes/#emptydir> for details."]
    #[serde(rename = "maxSize")]
    pub max_size: String,
    #[doc = "VolumeClaimSpec properties"]
    #[serde(rename = "persistentVolumeClaimSpec", default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim_spec: Option<VolumeClaimSpec>,
}
impl DiskBackedMessageBufferSettings {
    pub fn new(max_size: String) -> Self {
        Self {
            ephemeral_volume_claim_spec: None,
            max_size,
            persistent_volume_claim_spec: None,
        }
    }
}
#[doc = "DiskBackedMessageBufferSettings properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskBackedMessageBufferSettingsUpdate {
    #[doc = "VolumeClaimSpec properties"]
    #[serde(rename = "ephemeralVolumeClaimSpec", default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_volume_claim_spec: Option<VolumeClaimSpecUpdate>,
    #[doc = "The max size of the message buffer on disk. If a PVC template is specified using one of ephemeralVolumeClaimSpec or persistentVolumeClaimSpec, then this size is used as the request and limit sizes of that template. If neither ephemeralVolumeClaimSpec nor persistentVolumeClaimSpec are specified, then an emptyDir volume is mounted with this size as its limit. See <https://kubernetes.io/docs/concepts/storage/volumes/#emptydir> for details."]
    #[serde(rename = "maxSize", default, skip_serializing_if = "Option::is_none")]
    pub max_size: Option<String>,
    #[doc = "VolumeClaimSpec properties"]
    #[serde(rename = "persistentVolumeClaimSpec", default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim_spec: Option<VolumeClaimSpecUpdate>,
}
impl DiskBackedMessageBufferSettingsUpdate {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "ExtendedLocation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedLocationProperty {
    #[doc = "The name of the extended location."]
    pub name: String,
    #[doc = "The enum defining type of ExtendedLocation accepted."]
    #[serde(rename = "type")]
    pub type_: ExtendedLocationType,
}
impl ExtendedLocationProperty {
    pub fn new(name: String, type_: ExtendedLocationType) -> Self {
        Self { name, type_ }
    }
}
#[doc = "The enum defining type of ExtendedLocation accepted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExtendedLocationType")]
pub enum ExtendedLocationType {
    CustomLocation,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExtendedLocationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExtendedLocationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExtendedLocationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CustomLocation => serializer.serialize_unit_variant("ExtendedLocationType", 0u32, "CustomLocation"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Fabric one lake guids."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricGuids {
    #[doc = "Fabric one lake house guid."]
    #[serde(rename = "lakehouseGuid")]
    pub lakehouse_guid: String,
    #[doc = "Fabric one lake workspace guid."]
    #[serde(rename = "workspaceGuid")]
    pub workspace_guid: String,
}
impl FabricGuids {
    pub fn new(lakehouse_guid: String, workspace_guid: String) -> Self {
        Self {
            lakehouse_guid,
            workspace_guid,
        }
    }
}
#[doc = "Fabric one lake guids."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricGuidsUpdate {
    #[doc = "Fabric one lake house guid."]
    #[serde(rename = "lakehouseGuid", default, skip_serializing_if = "Option::is_none")]
    pub lakehouse_guid: Option<String>,
    #[doc = "Fabric one lake workspace guid."]
    #[serde(rename = "workspaceGuid", default, skip_serializing_if = "Option::is_none")]
    pub workspace_guid: Option<String>,
}
impl FabricGuidsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Fabric one lake names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricNames {
    #[doc = "Fabric one lake house name."]
    #[serde(rename = "lakehouseName")]
    pub lakehouse_name: String,
    #[doc = "Fabric one lake workspace name."]
    #[serde(rename = "workspaceName")]
    pub workspace_name: String,
}
impl FabricNames {
    pub fn new(lakehouse_name: String, workspace_name: String) -> Self {
        Self {
            lakehouse_name,
            workspace_name,
        }
    }
}
#[doc = "Fabric one lake names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricNamesUpdate {
    #[doc = "Fabric one lake house name."]
    #[serde(rename = "lakehouseName", default, skip_serializing_if = "Option::is_none")]
    pub lakehouse_name: Option<String>,
    #[doc = "Fabric one lake workspace name."]
    #[serde(rename = "workspaceName", default, skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
}
impl FabricNamesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Fabric path type to use."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FabricPathType")]
pub enum FabricPathType {
    #[serde(rename = "files")]
    Files,
    #[serde(rename = "tables")]
    Tables,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FabricPathType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FabricPathType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FabricPathType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Files => serializer.serialize_unit_variant("FabricPathType", 0u32, "files"),
            Self::Tables => serializer.serialize_unit_variant("FabricPathType", 1u32, "tables"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Desired properties of the Frontend Instances of the DMQTT Broker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Frontend {
    #[doc = "Replicas is desired number of frontend replicas of the given distributed MQTT broker."]
    pub replicas: i32,
    #[doc = "Internal knobs of Resource Limits for FE and BE"]
    #[serde(rename = "temporaryResourceLimits", default, skip_serializing_if = "Option::is_none")]
    pub temporary_resource_limits: Option<TemporaryResourceLimitsConfig>,
    #[doc = "Number of logical frontend workers per pod."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workers: Option<i32>,
}
impl Frontend {
    pub fn new(replicas: i32) -> Self {
        Self {
            replicas,
            temporary_resource_limits: None,
            workers: None,
        }
    }
}
#[doc = "Desired properties of the Frontend Instances of the DMQTT Broker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontendUpdate {
    #[doc = "Replicas is desired number of frontend replicas of the given distributed MQTT broker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[doc = "Internal knobs of Resource Limits for FE and BE"]
    #[serde(rename = "temporaryResourceLimits", default, skip_serializing_if = "Option::is_none")]
    pub temporary_resource_limits: Option<TemporaryResourceLimitsConfigUpdate>,
    #[doc = "Number of logical frontend workers per pod."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workers: Option<i32>,
}
impl FrontendUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka Acks enum properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KafkaAcks")]
pub enum KafkaAcks {
    #[serde(rename = "zero")]
    Zero,
    #[serde(rename = "one")]
    One,
    #[serde(rename = "all")]
    All,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KafkaAcks {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KafkaAcks {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KafkaAcks {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Zero => serializer.serialize_unit_variant("KafkaAcks", 0u32, "zero"),
            Self::One => serializer.serialize_unit_variant("KafkaAcks", 1u32, "one"),
            Self::All => serializer.serialize_unit_variant("KafkaAcks", 2u32, "all"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "KafkaConnector Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaConnectorProperties {
    #[doc = "The client id prefix of the dynamically generated client ids."]
    #[serde(rename = "clientIdPrefix", default, skip_serializing_if = "Option::is_none")]
    pub client_id_prefix: Option<String>,
    #[doc = "Defines the Docker image details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ContainerImage>,
    #[doc = "The number of KafkaConnector pods to spin up."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<i32>,
    #[doc = "Kafka RemoteBrokerConnectionSpec details"]
    #[serde(rename = "kafkaConnection")]
    pub kafka_connection: KafkaRemoteBrokerConnectionSpec,
    #[doc = "Mqtt Local Broker ConnectionSpec details"]
    #[serde(rename = "localBrokerConnection", default, skip_serializing_if = "Option::is_none")]
    pub local_broker_connection: Option<LocalBrokerConnectionSpec>,
    #[doc = "The log level of the Bridge Connector instances."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[doc = "Defines the Node Tolerations details"]
    #[serde(rename = "nodeTolerations", default, skip_serializing_if = "Option::is_none")]
    pub node_tolerations: Option<NodeTolerations>,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl KafkaConnectorProperties {
    pub fn new(kafka_connection: KafkaRemoteBrokerConnectionSpec) -> Self {
        Self {
            client_id_prefix: None,
            image: None,
            instances: None,
            kafka_connection,
            local_broker_connection: None,
            log_level: None,
            node_tolerations: None,
            provisioning_state: None,
        }
    }
}
#[doc = "MQ kafkaConnector resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaConnectorResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "KafkaConnector Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KafkaConnectorProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl KafkaConnectorResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a KafkaConnectorResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaConnectorResourceListResult {
    #[doc = "The KafkaConnectorResource items on this page"]
    pub value: Vec<KafkaConnectorResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KafkaConnectorResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl KafkaConnectorResourceListResult {
    pub fn new(value: Vec<KafkaConnectorResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the KafkaConnectorResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaConnectorResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the KafkaConnectorResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KafkaConnectorResourceUpdateProperties>,
}
impl KafkaConnectorResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the KafkaConnectorResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaConnectorResourceUpdateProperties {
    #[doc = "The client id prefix of the dynamically generated client ids."]
    #[serde(rename = "clientIdPrefix", default, skip_serializing_if = "Option::is_none")]
    pub client_id_prefix: Option<String>,
    #[doc = "Defines the Docker image details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ContainerImageUpdate>,
    #[doc = "The number of KafkaConnector pods to spin up."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<i32>,
    #[doc = "Kafka RemoteBrokerConnectionSpec details"]
    #[serde(rename = "kafkaConnection", default, skip_serializing_if = "Option::is_none")]
    pub kafka_connection: Option<KafkaRemoteBrokerConnectionSpecUpdate>,
    #[doc = "Mqtt Local Broker ConnectionSpec details"]
    #[serde(rename = "localBrokerConnection", default, skip_serializing_if = "Option::is_none")]
    pub local_broker_connection: Option<LocalBrokerConnectionSpecUpdate>,
    #[doc = "The log level of the Bridge Connector instances."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[doc = "Defines the Node Tolerations details"]
    #[serde(rename = "nodeTolerations", default, skip_serializing_if = "Option::is_none")]
    pub node_tolerations: Option<NodeTolerationsUpdate>,
}
impl KafkaConnectorResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka Message compression enum properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KafkaMessageCompressionType")]
pub enum KafkaMessageCompressionType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "snappy")]
    Snappy,
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KafkaMessageCompressionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KafkaMessageCompressionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KafkaMessageCompressionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("KafkaMessageCompressionType", 0u32, "none"),
            Self::Gzip => serializer.serialize_unit_variant("KafkaMessageCompressionType", 1u32, "gzip"),
            Self::Snappy => serializer.serialize_unit_variant("KafkaMessageCompressionType", 2u32, "snappy"),
            Self::Lz4 => serializer.serialize_unit_variant("KafkaMessageCompressionType", 3u32, "lz4"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Kafka Partition Strategy enum properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KafkaPartitionStrategy")]
pub enum KafkaPartitionStrategy {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "topic")]
    Topic,
    #[serde(rename = "property")]
    Property,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KafkaPartitionStrategy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KafkaPartitionStrategy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KafkaPartitionStrategy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("KafkaPartitionStrategy", 0u32, "default"),
            Self::Static => serializer.serialize_unit_variant("KafkaPartitionStrategy", 1u32, "static"),
            Self::Topic => serializer.serialize_unit_variant("KafkaPartitionStrategy", 2u32, "topic"),
            Self::Property => serializer.serialize_unit_variant("KafkaPartitionStrategy", 3u32, "property"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Kafka RemoteBrokerConnection Authentication methods"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaRemoteBrokerAuthenticationProperties {
    #[doc = "Kafka RemoteBrokerConnection Authentication types. NOTE - Enum only one method is allowed to be passed."]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<KafkaRemoteBrokerAuthenticationTypes>,
    #[doc = "If authentication is enabled for Kafka remote broker."]
    pub enabled: bool,
}
impl KafkaRemoteBrokerAuthenticationProperties {
    pub fn new(enabled: bool) -> Self {
        Self { auth_type: None, enabled }
    }
}
#[doc = "Kafka RemoteBrokerConnection Authentication methods"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaRemoteBrokerAuthenticationPropertiesUpdate {
    #[doc = "Kafka RemoteBrokerConnection Authentication types. NOTE - Enum only one method is allowed to be passed."]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<KafkaRemoteBrokerAuthenticationTypesUpdate>,
    #[doc = "If authentication is enabled for Kafka remote broker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl KafkaRemoteBrokerAuthenticationPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka RemoteBrokerConnection Authentication types. NOTE - Enum only one method is allowed to be passed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaRemoteBrokerAuthenticationTypes {
    #[doc = "Kafka RemoteBrokerConnection Sasl Authentication properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sasl: Option<SaslRemoteBrokerBasicAuthentication>,
    #[doc = "Managed identity authentication details."]
    #[serde(rename = "systemAssignedManagedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub system_assigned_managed_identity: Option<ManagedIdentityAuthentication>,
    #[doc = "Kafka RemoteBrokerConnection X509 Authentication properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<KafkaX509Authentication>,
}
impl KafkaRemoteBrokerAuthenticationTypes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka RemoteBrokerConnection Authentication types. NOTE - Enum only one method is allowed to be passed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaRemoteBrokerAuthenticationTypesUpdate {
    #[doc = "Kafka RemoteBrokerConnection Sasl Authentication properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sasl: Option<SaslRemoteBrokerBasicAuthenticationUpdate>,
    #[doc = "Managed identity authentication details."]
    #[serde(rename = "systemAssignedManagedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub system_assigned_managed_identity: Option<ManagedIdentityAuthenticationUpdate>,
    #[doc = "Kafka RemoteBrokerConnection X509 Authentication properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<KafkaX509AuthenticationUpdate>,
}
impl KafkaRemoteBrokerAuthenticationTypesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka RemoteBrokerConnectionSpec details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaRemoteBrokerConnectionSpec {
    #[doc = "Kafka RemoteBrokerConnection Authentication methods"]
    pub authentication: KafkaRemoteBrokerAuthenticationProperties,
    #[doc = "The endpoint of remote broker to connect to."]
    pub endpoint: String,
    #[doc = "Kafka RemoteBrokerConnection TLS details"]
    pub tls: KafkaRemoteBrokerConnectionTls,
}
impl KafkaRemoteBrokerConnectionSpec {
    pub fn new(authentication: KafkaRemoteBrokerAuthenticationProperties, endpoint: String, tls: KafkaRemoteBrokerConnectionTls) -> Self {
        Self {
            authentication,
            endpoint,
            tls,
        }
    }
}
#[doc = "Kafka RemoteBrokerConnectionSpec details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaRemoteBrokerConnectionSpecUpdate {
    #[doc = "Kafka RemoteBrokerConnection Authentication methods"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<KafkaRemoteBrokerAuthenticationPropertiesUpdate>,
    #[doc = "The endpoint of remote broker to connect to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Kafka RemoteBrokerConnection TLS details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<KafkaRemoteBrokerConnectionTlsUpdate>,
}
impl KafkaRemoteBrokerConnectionSpecUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka RemoteBrokerConnection TLS details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaRemoteBrokerConnectionTls {
    #[doc = "Tls Enabled on Remote Broker Connection."]
    #[serde(rename = "tlsEnabled")]
    pub tls_enabled: bool,
    #[doc = "Trusted CA certificate name for Remote Broker."]
    #[serde(rename = "trustedCaCertificateConfigMap", default, skip_serializing_if = "Option::is_none")]
    pub trusted_ca_certificate_config_map: Option<String>,
}
impl KafkaRemoteBrokerConnectionTls {
    pub fn new(tls_enabled: bool) -> Self {
        Self {
            tls_enabled,
            trusted_ca_certificate_config_map: None,
        }
    }
}
#[doc = "Kafka RemoteBrokerConnection TLS details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaRemoteBrokerConnectionTlsUpdate {
    #[doc = "Tls Enabled on Remote Broker Connection."]
    #[serde(rename = "tlsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub tls_enabled: Option<bool>,
    #[doc = "Trusted CA certificate name for Remote Broker."]
    #[serde(rename = "trustedCaCertificateConfigMap", default, skip_serializing_if = "Option::is_none")]
    pub trusted_ca_certificate_config_map: Option<String>,
}
impl KafkaRemoteBrokerConnectionTlsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka Routes properties. NOTE - Enum only one method is allowed to be passed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaRoutes {
    #[doc = "Kafka to Mqtt route properties"]
    #[serde(rename = "kafkaToMqtt", default, skip_serializing_if = "Option::is_none")]
    pub kafka_to_mqtt: Option<KafkaToMqttRoutes>,
    #[doc = "Mqtt to Kafka route properties"]
    #[serde(rename = "mqttToKafka", default, skip_serializing_if = "Option::is_none")]
    pub mqtt_to_kafka: Option<MqttToKafkaRoutes>,
}
impl KafkaRoutes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka Sasl Authentication types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KafkaSaslType")]
pub enum KafkaSaslType {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "scramSha256")]
    ScramSha256,
    #[serde(rename = "scramSha512")]
    ScramSha512,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KafkaSaslType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KafkaSaslType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KafkaSaslType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Plain => serializer.serialize_unit_variant("KafkaSaslType", 0u32, "plain"),
            Self::ScramSha256 => serializer.serialize_unit_variant("KafkaSaslType", 1u32, "scramSha256"),
            Self::ScramSha512 => serializer.serialize_unit_variant("KafkaSaslType", 2u32, "scramSha512"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Kafka Shared Subscription properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaSharedSubscriptionProperties {
    #[doc = "The minimum number to use in a group for subscription."]
    #[serde(rename = "groupMinimumShareNumber")]
    pub group_minimum_share_number: i32,
    #[doc = "The name of the shared subscription."]
    #[serde(rename = "groupName")]
    pub group_name: String,
}
impl KafkaSharedSubscriptionProperties {
    pub fn new(group_minimum_share_number: i32, group_name: String) -> Self {
        Self {
            group_minimum_share_number,
            group_name,
        }
    }
}
#[doc = "Kafka to Mqtt route properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaToMqttRoutes {
    #[doc = "The consumer group id to use."]
    #[serde(rename = "consumerGroupId", default, skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    #[doc = "The kafka topic to pull from."]
    #[serde(rename = "kafkaTopic")]
    pub kafka_topic: String,
    #[doc = "The mqtt topic to publish to."]
    #[serde(rename = "mqttTopic")]
    pub mqtt_topic: String,
    #[doc = "The name of the route."]
    pub name: String,
    #[doc = "The qos to use for mqtt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
}
impl KafkaToMqttRoutes {
    pub fn new(kafka_topic: String, mqtt_topic: String, name: String) -> Self {
        Self {
            consumer_group_id: None,
            kafka_topic,
            mqtt_topic,
            name,
            qos: None,
        }
    }
}
#[doc = "Kafka Token KeyVault properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaTokenKeyVaultProperties {
    #[doc = "Username to connect with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "KeyVault properties"]
    pub vault: KeyVaultConnectionProperties,
    #[doc = "KeyVault secret object properties"]
    #[serde(rename = "vaultSecret")]
    pub vault_secret: KeyVaultSecretObject,
}
impl KafkaTokenKeyVaultProperties {
    pub fn new(vault: KeyVaultConnectionProperties, vault_secret: KeyVaultSecretObject) -> Self {
        Self {
            username: None,
            vault,
            vault_secret,
        }
    }
}
#[doc = "Kafka Token KeyVault properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaTokenKeyVaultPropertiesUpdate {
    #[doc = "Username to connect with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "KeyVault properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault: Option<KeyVaultConnectionPropertiesUpdate>,
    #[doc = "KeyVault secret object properties"]
    #[serde(rename = "vaultSecret", default, skip_serializing_if = "Option::is_none")]
    pub vault_secret: Option<KeyVaultSecretObjectUpdate>,
}
impl KafkaTokenKeyVaultPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka TopicMap Batching properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaTopicMapBatching {
    #[doc = "The setting to enable or disable batching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The latency of message batching."]
    #[serde(rename = "latencyMs", default, skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i32>,
    #[doc = "The maximum bytes to send in a batch."]
    #[serde(rename = "maxBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_bytes: Option<i64>,
    #[doc = "The maximum messages to send in a batch."]
    #[serde(rename = "maxMessages", default, skip_serializing_if = "Option::is_none")]
    pub max_messages: Option<i64>,
}
impl KafkaTopicMapBatching {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KafkaTopicMap Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaTopicMapProperties {
    #[doc = "Kafka TopicMap Batching properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batching: Option<KafkaTopicMapBatching>,
    #[doc = "The compression to use for kafka messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<kafka_topic_map_properties::Compression>,
    #[doc = "The flag to copy Mqtt properties."]
    #[serde(rename = "copyMqttProperties", default, skip_serializing_if = "Option::is_none")]
    pub copy_mqtt_properties: Option<String>,
    #[doc = "The kafkaConnector CRD it refers to."]
    #[serde(rename = "kafkaConnectorRef")]
    pub kafka_connector_ref: String,
    #[doc = "The partition to use for Kafka."]
    #[serde(rename = "partitionKeyProperty", default, skip_serializing_if = "Option::is_none")]
    pub partition_key_property: Option<String>,
    #[doc = "The partition strategy to use for Kafka."]
    #[serde(rename = "partitionStrategy", default, skip_serializing_if = "Option::is_none")]
    pub partition_strategy: Option<kafka_topic_map_properties::PartitionStrategy>,
    #[doc = "The route details for Kafka connector."]
    pub routes: Vec<KafkaRoutes>,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl KafkaTopicMapProperties {
    pub fn new(kafka_connector_ref: String, routes: Vec<KafkaRoutes>) -> Self {
        Self {
            batching: None,
            compression: None,
            copy_mqtt_properties: None,
            kafka_connector_ref,
            partition_key_property: None,
            partition_strategy: None,
            routes,
            provisioning_state: None,
        }
    }
}
pub mod kafka_topic_map_properties {
    use super::*;
    #[doc = "The compression to use for kafka messages."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Compression")]
    pub enum Compression {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "gzip")]
        Gzip,
        #[serde(rename = "snappy")]
        Snappy,
        #[serde(rename = "lz4")]
        Lz4,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Compression {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Compression {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Compression {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Compression", 0u32, "none"),
                Self::Gzip => serializer.serialize_unit_variant("Compression", 1u32, "gzip"),
                Self::Snappy => serializer.serialize_unit_variant("Compression", 2u32, "snappy"),
                Self::Lz4 => serializer.serialize_unit_variant("Compression", 3u32, "lz4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Compression {
        fn default() -> Self {
            Self::None
        }
    }
    #[doc = "The partition strategy to use for Kafka."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PartitionStrategy")]
    pub enum PartitionStrategy {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "static")]
        Static,
        #[serde(rename = "topic")]
        Topic,
        #[serde(rename = "property")]
        Property,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PartitionStrategy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PartitionStrategy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PartitionStrategy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("PartitionStrategy", 0u32, "default"),
                Self::Static => serializer.serialize_unit_variant("PartitionStrategy", 1u32, "static"),
                Self::Topic => serializer.serialize_unit_variant("PartitionStrategy", 2u32, "topic"),
                Self::Property => serializer.serialize_unit_variant("PartitionStrategy", 3u32, "property"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PartitionStrategy {
        fn default() -> Self {
            Self::Default
        }
    }
}
#[doc = "MQ kafkaConnector/topicMap resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaTopicMapResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "KafkaTopicMap Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KafkaTopicMapProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl KafkaTopicMapResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a KafkaTopicMapResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaTopicMapResourceListResult {
    #[doc = "The KafkaTopicMapResource items on this page"]
    pub value: Vec<KafkaTopicMapResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KafkaTopicMapResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl KafkaTopicMapResourceListResult {
    pub fn new(value: Vec<KafkaTopicMapResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the KafkaTopicMapResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaTopicMapResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the KafkaTopicMapResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KafkaTopicMapResourceUpdateProperties>,
}
impl KafkaTopicMapResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the KafkaTopicMapResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaTopicMapResourceUpdateProperties {
    #[doc = "Kafka TopicMap Batching properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batching: Option<KafkaTopicMapBatching>,
    #[doc = "The compression to use for kafka messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<kafka_topic_map_resource_update_properties::Compression>,
    #[doc = "The flag to copy Mqtt properties."]
    #[serde(rename = "copyMqttProperties", default, skip_serializing_if = "Option::is_none")]
    pub copy_mqtt_properties: Option<String>,
    #[doc = "The kafkaConnector CRD it refers to."]
    #[serde(rename = "kafkaConnectorRef", default, skip_serializing_if = "Option::is_none")]
    pub kafka_connector_ref: Option<String>,
    #[doc = "The partition to use for Kafka."]
    #[serde(rename = "partitionKeyProperty", default, skip_serializing_if = "Option::is_none")]
    pub partition_key_property: Option<String>,
    #[doc = "The partition strategy to use for Kafka."]
    #[serde(rename = "partitionStrategy", default, skip_serializing_if = "Option::is_none")]
    pub partition_strategy: Option<kafka_topic_map_resource_update_properties::PartitionStrategy>,
    #[doc = "The route details for Kafka connector."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub routes: Vec<KafkaRoutes>,
}
impl KafkaTopicMapResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod kafka_topic_map_resource_update_properties {
    use super::*;
    #[doc = "The compression to use for kafka messages."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Compression")]
    pub enum Compression {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "gzip")]
        Gzip,
        #[serde(rename = "snappy")]
        Snappy,
        #[serde(rename = "lz4")]
        Lz4,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Compression {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Compression {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Compression {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Compression", 0u32, "none"),
                Self::Gzip => serializer.serialize_unit_variant("Compression", 1u32, "gzip"),
                Self::Snappy => serializer.serialize_unit_variant("Compression", 2u32, "snappy"),
                Self::Lz4 => serializer.serialize_unit_variant("Compression", 3u32, "lz4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Compression {
        fn default() -> Self {
            Self::None
        }
    }
    #[doc = "The partition strategy to use for Kafka."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PartitionStrategy")]
    pub enum PartitionStrategy {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "static")]
        Static,
        #[serde(rename = "topic")]
        Topic,
        #[serde(rename = "property")]
        Property,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PartitionStrategy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PartitionStrategy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PartitionStrategy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("PartitionStrategy", 0u32, "default"),
                Self::Static => serializer.serialize_unit_variant("PartitionStrategy", 1u32, "static"),
                Self::Topic => serializer.serialize_unit_variant("PartitionStrategy", 2u32, "topic"),
                Self::Property => serializer.serialize_unit_variant("PartitionStrategy", 3u32, "property"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PartitionStrategy {
        fn default() -> Self {
            Self::Default
        }
    }
}
#[doc = "Kafka RemoteBrokerConnection X509 Authentication properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaX509Authentication {
    #[doc = "KeyVault certificate properties"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultCertificateProperties>,
    #[doc = "Secret where cert details are stored."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
impl KafkaX509Authentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka RemoteBrokerConnection X509 Authentication properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KafkaX509AuthenticationUpdate {
    #[doc = "KeyVault certificate properties"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultCertificatePropertiesUpdate>,
    #[doc = "Secret where cert details are stored."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
impl KafkaX509AuthenticationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KeyVault certificate properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultCertificateProperties {
    #[doc = "KeyVault properties"]
    pub vault: KeyVaultConnectionProperties,
    #[doc = "KeyVault secret object properties"]
    #[serde(rename = "vaultCaChainSecret", default, skip_serializing_if = "Option::is_none")]
    pub vault_ca_chain_secret: Option<KeyVaultSecretObject>,
    #[doc = "KeyVault secret object properties"]
    #[serde(rename = "vaultCert")]
    pub vault_cert: KeyVaultSecretObject,
}
impl KeyVaultCertificateProperties {
    pub fn new(vault: KeyVaultConnectionProperties, vault_cert: KeyVaultSecretObject) -> Self {
        Self {
            vault,
            vault_ca_chain_secret: None,
            vault_cert,
        }
    }
}
#[doc = "KeyVault certificate properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultCertificatePropertiesUpdate {
    #[doc = "KeyVault properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault: Option<KeyVaultConnectionPropertiesUpdate>,
    #[doc = "KeyVault secret object properties"]
    #[serde(rename = "vaultCaChainSecret", default, skip_serializing_if = "Option::is_none")]
    pub vault_ca_chain_secret: Option<KeyVaultSecretObjectUpdate>,
    #[doc = "KeyVault secret object properties"]
    #[serde(rename = "vaultCert", default, skip_serializing_if = "Option::is_none")]
    pub vault_cert: Option<KeyVaultSecretObjectUpdate>,
}
impl KeyVaultCertificatePropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KeyVault properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultConnectionProperties {
    #[doc = "KeyVault credentials properties. NOTE - Future this will be ENUM."]
    pub credentials: KeyVaultCredentialsProperties,
    #[doc = "KeyVault directoryId."]
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    #[doc = "KeyVault name."]
    pub name: String,
}
impl KeyVaultConnectionProperties {
    pub fn new(credentials: KeyVaultCredentialsProperties, directory_id: String, name: String) -> Self {
        Self {
            credentials,
            directory_id,
            name,
        }
    }
}
#[doc = "KeyVault properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultConnectionPropertiesUpdate {
    #[doc = "KeyVault credentials properties. NOTE - Future this will be ENUM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<KeyVaultCredentialsPropertiesUpdate>,
    #[doc = "KeyVault directoryId."]
    #[serde(rename = "directoryId", default, skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[doc = "KeyVault name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl KeyVaultConnectionPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KeyVault credentials properties. NOTE - Future this will be ENUM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultCredentialsProperties {
    #[doc = "KeyVault service principal local secret name."]
    #[serde(rename = "servicePrincipalLocalSecretName")]
    pub service_principal_local_secret_name: String,
}
impl KeyVaultCredentialsProperties {
    pub fn new(service_principal_local_secret_name: String) -> Self {
        Self {
            service_principal_local_secret_name,
        }
    }
}
#[doc = "KeyVault credentials properties. NOTE - Future this will be ENUM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultCredentialsPropertiesUpdate {
    #[doc = "KeyVault service principal local secret name."]
    #[serde(rename = "servicePrincipalLocalSecretName", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_local_secret_name: Option<String>,
}
impl KeyVaultCredentialsPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KeyVault secret object properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSecretObject {
    #[doc = "KeyVault secret name."]
    pub name: String,
    #[doc = "KeyVault secret version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl KeyVaultSecretObject {
    pub fn new(name: String) -> Self {
        Self { name, version: None }
    }
}
#[doc = "KeyVault secret object properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretObjectUpdate {
    #[doc = "KeyVault secret name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "KeyVault secret version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl KeyVaultSecretObjectUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KeyVault secret properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSecretProperties {
    #[doc = "KeyVault properties"]
    pub vault: KeyVaultConnectionProperties,
    #[doc = "KeyVault secret object properties"]
    #[serde(rename = "vaultSecret")]
    pub vault_secret: KeyVaultSecretObject,
}
impl KeyVaultSecretProperties {
    pub fn new(vault: KeyVaultConnectionProperties, vault_secret: KeyVaultSecretObject) -> Self {
        Self { vault, vault_secret }
    }
}
#[doc = "Mqtt Local Broker Authentication details. Only one method at a time is supported. Default - kubernetes authentication"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalBrokerAuthenticationMethods {
    #[doc = "Local Broker Kubernetes Authentication"]
    pub kubernetes: LocalBrokerKubernetesAuthentication,
}
impl LocalBrokerAuthenticationMethods {
    pub fn new(kubernetes: LocalBrokerKubernetesAuthentication) -> Self {
        Self { kubernetes }
    }
}
#[doc = "Mqtt Local Broker Authentication details. Only one method at a time is supported. Default - kubernetes authentication"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalBrokerAuthenticationMethodsUpdate {
    #[doc = "Local Broker Kubernetes Authentication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<LocalBrokerKubernetesAuthentication>,
}
impl LocalBrokerAuthenticationMethodsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mqtt Local Broker ConnectionSpec details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalBrokerConnectionSpec {
    #[doc = "Mqtt Local Broker Authentication details. Only one method at a time is supported. Default - kubernetes authentication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<LocalBrokerAuthenticationMethods>,
    #[doc = "The endpoint of local broker to connect to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Mqtt Local Broker Connection TLS details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<LocalBrokerConnectionTls>,
}
impl LocalBrokerConnectionSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mqtt Local Broker ConnectionSpec details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalBrokerConnectionSpecUpdate {
    #[doc = "Mqtt Local Broker Authentication details. Only one method at a time is supported. Default - kubernetes authentication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<LocalBrokerAuthenticationMethodsUpdate>,
    #[doc = "The endpoint of local broker to connect to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Mqtt Local Broker Connection TLS details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<LocalBrokerConnectionTls>,
}
impl LocalBrokerConnectionSpecUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mqtt Local Broker Connection TLS details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalBrokerConnectionTls {
    #[doc = "Tls Enabled on Local Broker Connection."]
    #[serde(rename = "tlsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub tls_enabled: Option<bool>,
    #[doc = "Trusted CA certificate config map name for Local Broker."]
    #[serde(rename = "trustedCaCertificateConfigMap", default, skip_serializing_if = "Option::is_none")]
    pub trusted_ca_certificate_config_map: Option<String>,
}
impl LocalBrokerConnectionTls {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Local Broker Kubernetes Authentication"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalBrokerKubernetesAuthentication {
    #[doc = "Secret Path where SAT is mounted."]
    #[serde(rename = "secretPath", default, skip_serializing_if = "Option::is_none")]
    pub secret_path: Option<String>,
    #[doc = "Token name where SAT is mounted on secret path."]
    #[serde(rename = "serviceAccountTokenName", default, skip_serializing_if = "Option::is_none")]
    pub service_account_token_name: Option<String>,
}
impl LocalBrokerKubernetesAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed identity authentication details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIdentityAuthentication {
    #[doc = "Token audience."]
    pub audience: String,
    #[doc = "Arc Extension name."]
    #[serde(rename = "extensionName", default, skip_serializing_if = "Option::is_none")]
    pub extension_name: Option<String>,
}
impl ManagedIdentityAuthentication {
    pub fn new(audience: String) -> Self {
        Self {
            audience,
            extension_name: None,
        }
    }
}
#[doc = "Managed identity authentication details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentityAuthenticationUpdate {
    #[doc = "Token audience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "Arc Extension name."]
    #[serde(rename = "extensionName", default, skip_serializing_if = "Option::is_none")]
    pub extension_name: Option<String>,
}
impl ManagedIdentityAuthenticationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Manual TLS server certificate management through a defined secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualCertMethod {
    #[doc = "secret containing TLS cert."]
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[doc = "namespace of secret; omit to use default namespace."]
    #[serde(rename = "secretNamespace", default, skip_serializing_if = "Option::is_none")]
    pub secret_namespace: Option<String>,
}
impl ManualCertMethod {
    pub fn new(secret_name: String) -> Self {
        Self {
            secret_name,
            secret_namespace: None,
        }
    }
}
#[doc = "Manual TLS server certificate management through a defined secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManualCertMethodUpdate {
    #[doc = "secret containing TLS cert."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    #[doc = "namespace of secret; omit to use default namespace."]
    #[serde(rename = "secretNamespace", default, skip_serializing_if = "Option::is_none")]
    pub secret_namespace: Option<String>,
}
impl ManualCertMethodUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MQ Resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqProperties {
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl MqProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MQ resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "MQ Resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MqProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl MqResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a MqResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqResourceListResult {
    #[doc = "The MqResource items on this page"]
    pub value: Vec<MqResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MqResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MqResourceListResult {
    pub fn new(value: Vec<MqResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the MqResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl MqResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MqttBridgeConnector Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttBridgeConnectorProperties {
    #[doc = "The number of instances to deploy for a bridge rollout."]
    #[serde(rename = "bridgeInstances", default, skip_serializing_if = "Option::is_none")]
    pub bridge_instances: Option<i32>,
    #[doc = "The client id prefix of the dynamically generated client ids."]
    #[serde(rename = "clientIdPrefix", default, skip_serializing_if = "Option::is_none")]
    pub client_id_prefix: Option<String>,
    #[doc = "Defines the Docker image details"]
    pub image: ContainerImage,
    #[doc = "Mqtt Local Broker ConnectionSpec details"]
    #[serde(rename = "localBrokerConnection", default, skip_serializing_if = "Option::is_none")]
    pub local_broker_connection: Option<LocalBrokerConnectionSpec>,
    #[doc = "The log level of the Bridge Connector instances."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[doc = "Defines the Node Tolerations details"]
    #[serde(rename = "nodeTolerations", default, skip_serializing_if = "Option::is_none")]
    pub node_tolerations: Option<NodeTolerations>,
    #[doc = "Mqtt Protocol types"]
    pub protocol: MqttProtocol,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "MqttBridge RemoteBrokerConnectionSpec details"]
    #[serde(rename = "remoteBrokerConnection")]
    pub remote_broker_connection: MqttBridgeRemoteBrokerConnectionSpec,
}
impl MqttBridgeConnectorProperties {
    pub fn new(image: ContainerImage, protocol: MqttProtocol, remote_broker_connection: MqttBridgeRemoteBrokerConnectionSpec) -> Self {
        Self {
            bridge_instances: None,
            client_id_prefix: None,
            image,
            local_broker_connection: None,
            log_level: None,
            node_tolerations: None,
            protocol,
            provisioning_state: None,
            remote_broker_connection,
        }
    }
}
#[doc = "MQ mqttBridgeConnector resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttBridgeConnectorResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "MqttBridgeConnector Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MqttBridgeConnectorProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl MqttBridgeConnectorResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a MqttBridgeConnectorResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttBridgeConnectorResourceListResult {
    #[doc = "The MqttBridgeConnectorResource items on this page"]
    pub value: Vec<MqttBridgeConnectorResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MqttBridgeConnectorResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MqttBridgeConnectorResourceListResult {
    pub fn new(value: Vec<MqttBridgeConnectorResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the MqttBridgeConnectorResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqttBridgeConnectorResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the MqttBridgeConnectorResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MqttBridgeConnectorResourceUpdateProperties>,
}
impl MqttBridgeConnectorResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the MqttBridgeConnectorResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqttBridgeConnectorResourceUpdateProperties {
    #[doc = "The number of instances to deploy for a bridge rollout."]
    #[serde(rename = "bridgeInstances", default, skip_serializing_if = "Option::is_none")]
    pub bridge_instances: Option<i32>,
    #[doc = "The client id prefix of the dynamically generated client ids."]
    #[serde(rename = "clientIdPrefix", default, skip_serializing_if = "Option::is_none")]
    pub client_id_prefix: Option<String>,
    #[doc = "Defines the Docker image details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ContainerImageUpdate>,
    #[doc = "Mqtt Local Broker ConnectionSpec details"]
    #[serde(rename = "localBrokerConnection", default, skip_serializing_if = "Option::is_none")]
    pub local_broker_connection: Option<LocalBrokerConnectionSpecUpdate>,
    #[doc = "The log level of the Bridge Connector instances."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[doc = "Defines the Node Tolerations details"]
    #[serde(rename = "nodeTolerations", default, skip_serializing_if = "Option::is_none")]
    pub node_tolerations: Option<NodeTolerationsUpdate>,
    #[doc = "Mqtt Protocol types"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<MqttProtocol>,
    #[doc = "MqttBridge RemoteBrokerConnectionSpec details"]
    #[serde(rename = "remoteBrokerConnection", default, skip_serializing_if = "Option::is_none")]
    pub remote_broker_connection: Option<MqttBridgeRemoteBrokerConnectionSpecUpdate>,
}
impl MqttBridgeConnectorResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MqttBridge RemoteBrokerConnection Authentication methods. NOTE - Enum only one is allowed to be passed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqttBridgeRemoteBrokerAuthenticationMethods {
    #[doc = "MqttBridge RemoteBroker X509 Authentication properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<MqttBridgeRemoteBrokerX509Authentication>,
    #[doc = "Managed identity authentication details."]
    #[serde(rename = "systemAssignedManagedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub system_assigned_managed_identity: Option<ManagedIdentityAuthentication>,
}
impl MqttBridgeRemoteBrokerAuthenticationMethods {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MqttBridge RemoteBrokerConnection Authentication methods. NOTE - Enum only one is allowed to be passed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqttBridgeRemoteBrokerAuthenticationMethodsUpdate {
    #[doc = "MqttBridge RemoteBroker X509 Authentication properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<MqttBridgeRemoteBrokerX509AuthenticationUpdate>,
    #[doc = "Managed identity authentication details."]
    #[serde(rename = "systemAssignedManagedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub system_assigned_managed_identity: Option<ManagedIdentityAuthenticationUpdate>,
}
impl MqttBridgeRemoteBrokerAuthenticationMethodsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MqttBridge RemoteBrokerConnectionSpec details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttBridgeRemoteBrokerConnectionSpec {
    #[doc = "MqttBridge RemoteBrokerConnection Authentication methods. NOTE - Enum only one is allowed to be passed."]
    pub authentication: MqttBridgeRemoteBrokerAuthenticationMethods,
    #[doc = "The endpoint of remote broker to connect to."]
    pub endpoint: String,
    #[doc = "Protocol for remote connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<mqtt_bridge_remote_broker_connection_spec::Protocol>,
    #[doc = "MqttBridge RemoteBrokerConnection TLS details"]
    pub tls: MqttBridgeRemoteBrokerConnectionTls,
}
impl MqttBridgeRemoteBrokerConnectionSpec {
    pub fn new(
        authentication: MqttBridgeRemoteBrokerAuthenticationMethods,
        endpoint: String,
        tls: MqttBridgeRemoteBrokerConnectionTls,
    ) -> Self {
        Self {
            authentication,
            endpoint,
            protocol: None,
            tls,
        }
    }
}
pub mod mqtt_bridge_remote_broker_connection_spec {
    use super::*;
    #[doc = "Protocol for remote connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "mqtt")]
        Mqtt,
        #[serde(rename = "webSocket")]
        WebSocket,
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
                Self::Mqtt => serializer.serialize_unit_variant("Protocol", 0u32, "mqtt"),
                Self::WebSocket => serializer.serialize_unit_variant("Protocol", 1u32, "webSocket"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Protocol {
        fn default() -> Self {
            Self::Mqtt
        }
    }
}
#[doc = "MqttBridge RemoteBrokerConnectionSpec details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqttBridgeRemoteBrokerConnectionSpecUpdate {
    #[doc = "MqttBridge RemoteBrokerConnection Authentication methods. NOTE - Enum only one is allowed to be passed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<MqttBridgeRemoteBrokerAuthenticationMethodsUpdate>,
    #[doc = "The endpoint of remote broker to connect to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Protocol for remote connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<mqtt_bridge_remote_broker_connection_spec_update::Protocol>,
    #[doc = "MqttBridge RemoteBrokerConnection TLS details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<MqttBridgeRemoteBrokerConnectionTlsUpdate>,
}
impl MqttBridgeRemoteBrokerConnectionSpecUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod mqtt_bridge_remote_broker_connection_spec_update {
    use super::*;
    #[doc = "Protocol for remote connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "mqtt")]
        Mqtt,
        #[serde(rename = "webSocket")]
        WebSocket,
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
                Self::Mqtt => serializer.serialize_unit_variant("Protocol", 0u32, "mqtt"),
                Self::WebSocket => serializer.serialize_unit_variant("Protocol", 1u32, "webSocket"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Protocol {
        fn default() -> Self {
            Self::Mqtt
        }
    }
}
#[doc = "MqttBridge RemoteBrokerConnection TLS details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttBridgeRemoteBrokerConnectionTls {
    #[doc = "Tls Enabled on Remote Broker Connection."]
    #[serde(rename = "tlsEnabled")]
    pub tls_enabled: bool,
    #[doc = "Trusted CA certificate name for Remote Broker."]
    #[serde(rename = "trustedCaCertificateConfigMap", default, skip_serializing_if = "Option::is_none")]
    pub trusted_ca_certificate_config_map: Option<String>,
}
impl MqttBridgeRemoteBrokerConnectionTls {
    pub fn new(tls_enabled: bool) -> Self {
        Self {
            tls_enabled,
            trusted_ca_certificate_config_map: None,
        }
    }
}
#[doc = "MqttBridge RemoteBrokerConnection TLS details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqttBridgeRemoteBrokerConnectionTlsUpdate {
    #[doc = "Tls Enabled on Remote Broker Connection."]
    #[serde(rename = "tlsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub tls_enabled: Option<bool>,
    #[doc = "Trusted CA certificate name for Remote Broker."]
    #[serde(rename = "trustedCaCertificateConfigMap", default, skip_serializing_if = "Option::is_none")]
    pub trusted_ca_certificate_config_map: Option<String>,
}
impl MqttBridgeRemoteBrokerConnectionTlsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protocol for remote connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MqttBridgeRemoteBrokerProtocol")]
pub enum MqttBridgeRemoteBrokerProtocol {
    #[serde(rename = "mqtt")]
    Mqtt,
    #[serde(rename = "webSocket")]
    WebSocket,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MqttBridgeRemoteBrokerProtocol {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MqttBridgeRemoteBrokerProtocol {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MqttBridgeRemoteBrokerProtocol {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Mqtt => serializer.serialize_unit_variant("MqttBridgeRemoteBrokerProtocol", 0u32, "mqtt"),
            Self::WebSocket => serializer.serialize_unit_variant("MqttBridgeRemoteBrokerProtocol", 1u32, "webSocket"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "MqttBridge RemoteBroker X509 Authentication properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqttBridgeRemoteBrokerX509Authentication {
    #[doc = "KeyVault certificate properties"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultCertificateProperties>,
    #[doc = "Secret where cert details are stored."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
impl MqttBridgeRemoteBrokerX509Authentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MqttBridge RemoteBroker X509 Authentication properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqttBridgeRemoteBrokerX509AuthenticationUpdate {
    #[doc = "KeyVault certificate properties"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultCertificatePropertiesUpdate>,
    #[doc = "Secret where cert details are stored."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
impl MqttBridgeRemoteBrokerX509AuthenticationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MqttBridgeRoute direction properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MqttBridgeRouteDirection")]
pub enum MqttBridgeRouteDirection {
    #[serde(rename = "remote-to-local")]
    RemoteToLocal,
    #[serde(rename = "local-to-remote")]
    LocalToRemote,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MqttBridgeRouteDirection {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MqttBridgeRouteDirection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MqttBridgeRouteDirection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RemoteToLocal => serializer.serialize_unit_variant("MqttBridgeRouteDirection", 0u32, "remote-to-local"),
            Self::LocalToRemote => serializer.serialize_unit_variant("MqttBridgeRouteDirection", 1u32, "local-to-remote"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "MqttBridgeRoute Shared subscription properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttBridgeRouteSharedSubscription {
    #[doc = "The group shared subscription minimum share number."]
    #[serde(rename = "groupMinimumShareNumber")]
    pub group_minimum_share_number: i32,
    #[doc = "The group name for Shared subscription."]
    #[serde(rename = "groupName")]
    pub group_name: String,
}
impl MqttBridgeRouteSharedSubscription {
    pub fn new(group_minimum_share_number: i32, group_name: String) -> Self {
        Self {
            group_minimum_share_number,
            group_name,
        }
    }
}
#[doc = "MqttBridgeTopicMap route properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttBridgeRoutes {
    #[doc = "MqttBridgeRoute direction properties"]
    pub direction: MqttBridgeRouteDirection,
    #[doc = "Name of the route."]
    pub name: String,
    #[doc = "Qos for MQTT connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    #[doc = "MqttBridgeRoute Shared subscription properties"]
    #[serde(rename = "sharedSubscription", default, skip_serializing_if = "Option::is_none")]
    pub shared_subscription: Option<MqttBridgeRouteSharedSubscription>,
    #[doc = "Source topic of the route."]
    pub source: String,
    #[doc = "Target topic of the route. Ignore if same as source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl MqttBridgeRoutes {
    pub fn new(direction: MqttBridgeRouteDirection, name: String, source: String) -> Self {
        Self {
            direction,
            name,
            qos: None,
            shared_subscription: None,
            source,
            target: None,
        }
    }
}
#[doc = "MqttBridgeTopicMap Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttBridgeTopicMapProperties {
    #[doc = "The MqttBridgeConnector CRD it refers to."]
    #[serde(rename = "mqttBridgeConnectorRef")]
    pub mqtt_bridge_connector_ref: String,
    #[doc = "The route details for MqttBridge connector."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub routes: Vec<MqttBridgeRoutes>,
    #[doc = "The enum defining status of resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl MqttBridgeTopicMapProperties {
    pub fn new(mqtt_bridge_connector_ref: String) -> Self {
        Self {
            mqtt_bridge_connector_ref,
            routes: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[doc = "MQ mqttBridgeTopicMap resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttBridgeTopicMapResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "MqttBridgeTopicMap Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MqttBridgeTopicMapProperties>,
    #[doc = "ExtendedLocation properties"]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocationProperty,
}
impl MqttBridgeTopicMapResource {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocationProperty) -> Self {
        Self {
            tracked_resource,
            properties: None,
            extended_location,
        }
    }
}
#[doc = "The response of a MqttBridgeTopicMapResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttBridgeTopicMapResourceListResult {
    #[doc = "The MqttBridgeTopicMapResource items on this page"]
    pub value: Vec<MqttBridgeTopicMapResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MqttBridgeTopicMapResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MqttBridgeTopicMapResourceListResult {
    pub fn new(value: Vec<MqttBridgeTopicMapResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the MqttBridgeTopicMapResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqttBridgeTopicMapResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the MqttBridgeTopicMapResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MqttBridgeTopicMapResourceUpdateProperties>,
}
impl MqttBridgeTopicMapResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the MqttBridgeTopicMapResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MqttBridgeTopicMapResourceUpdateProperties {
    #[doc = "The MqttBridgeConnector CRD it refers to."]
    #[serde(rename = "mqttBridgeConnectorRef", default, skip_serializing_if = "Option::is_none")]
    pub mqtt_bridge_connector_ref: Option<String>,
    #[doc = "The route details for MqttBridge connector."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub routes: Vec<MqttBridgeRoutes>,
}
impl MqttBridgeTopicMapResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mqtt Protocol types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MqttProtocol")]
pub enum MqttProtocol {
    #[serde(rename = "v3")]
    V3,
    #[serde(rename = "v5")]
    V5,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MqttProtocol {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MqttProtocol {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MqttProtocol {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::V3 => serializer.serialize_unit_variant("MqttProtocol", 0u32, "v3"),
            Self::V5 => serializer.serialize_unit_variant("MqttProtocol", 1u32, "v5"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Mqtt to Kafka route properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttToKafkaRoutes {
    #[doc = "Kafka Acks enum properties"]
    #[serde(rename = "kafkaAcks")]
    pub kafka_acks: KafkaAcks,
    #[doc = "The kafka topic to publish to."]
    #[serde(rename = "kafkaTopic")]
    pub kafka_topic: String,
    #[doc = "The mqtt topic to pull from."]
    #[serde(rename = "mqttTopic")]
    pub mqtt_topic: String,
    #[doc = "The name of the route."]
    pub name: String,
    #[doc = "The qos to use for mqtt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    #[doc = "Kafka Shared Subscription properties"]
    #[serde(rename = "sharedSubscription", default, skip_serializing_if = "Option::is_none")]
    pub shared_subscription: Option<KafkaSharedSubscriptionProperties>,
}
impl MqttToKafkaRoutes {
    pub fn new(kafka_acks: KafkaAcks, kafka_topic: String, mqtt_topic: String, name: String) -> Self {
        Self {
            kafka_acks,
            kafka_topic,
            mqtt_topic,
            name,
            qos: None,
            shared_subscription: None,
        }
    }
}
#[doc = "Defines the Node Tolerations details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeTolerations {
    #[doc = "Toleration effect."]
    pub effect: String,
    #[doc = "Toleration key."]
    pub key: String,
    #[doc = "Toleration operator like 'Exists', 'Equal' etc."]
    pub operator: String,
    #[doc = "Toleration Value."]
    pub value: String,
}
impl NodeTolerations {
    pub fn new(effect: String, key: String, operator: String, value: String) -> Self {
        Self {
            effect,
            key,
            operator,
            value,
        }
    }
}
#[doc = "Defines the Node Tolerations details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTolerationsUpdate {
    #[doc = "Toleration effect."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[doc = "Toleration key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Toleration operator like 'Exists', 'Equal' etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[doc = "Toleration Value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl NodeTolerationsUpdate {
    pub fn new() -> Self {
        Self::default()
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PrincipalDefinition properties of Basic Rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrincipalDefinition {
    #[doc = "A list of key-value pairs that match the attributes of the clients. The attributes are case-sensitive and must match the attributes provided by the clients during authentication."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attributes: Vec<serde_json::Value>,
    #[doc = "A list of client IDs that match the clients. The client IDs are case-sensitive and must match the client IDs provided by the clients during connection."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub clientids: Vec<String>,
    #[doc = "A list of usernames that match the clients. The usernames are case-sensitive and must match the usernames provided by the clients during authentication."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub usernames: Vec<String>,
}
impl PrincipalDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The enum defining status of resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    Updating,
    Deleting,
    Accepted,
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
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
            Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ResourceInfoDefinition properties of Basic Rule. This defines the objects that represent the actions or topics, such as - method.Connect, method.Publish, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceInfoDefinition {
    #[doc = "ResourceInfoDefinition methods allowed"]
    pub method: ResourceInfoDefinitionMethods,
    #[doc = "A list of topics or topic patterns that match the topics that the clients can publish or subscribe to. This subfield is required if the method is Publish or Subscribe."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub topics: Vec<String>,
}
impl ResourceInfoDefinition {
    pub fn new(method: ResourceInfoDefinitionMethods) -> Self {
        Self {
            method,
            topics: Vec::new(),
        }
    }
}
#[doc = "ResourceInfoDefinition methods allowed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceInfoDefinitionMethods")]
pub enum ResourceInfoDefinitionMethods {
    Connect,
    Publish,
    Subscribe,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceInfoDefinitionMethods {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceInfoDefinitionMethods {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceInfoDefinitionMethods {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Connect => serializer.serialize_unit_variant("ResourceInfoDefinitionMethods", 0u32, "Connect"),
            Self::Publish => serializer.serialize_unit_variant("ResourceInfoDefinitionMethods", 1u32, "Publish"),
            Self::Subscribe => serializer.serialize_unit_variant("ResourceInfoDefinitionMethods", 2u32, "Subscribe"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The enum defining run mode of the broker deployment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RunMode")]
pub enum RunMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "distributed")]
    Distributed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RunMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RunMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RunMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_unit_variant("RunMode", 0u32, "auto"),
            Self::Distributed => serializer.serialize_unit_variant("RunMode", 1u32, "distributed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SANs for certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SanForCert {
    #[doc = "DNS SANs."]
    pub dns: Vec<String>,
    #[doc = "IP address SANs."]
    pub ip: Vec<String>,
}
impl SanForCert {
    pub fn new(dns: Vec<String>, ip: Vec<String>) -> Self {
        Self { dns, ip }
    }
}
#[doc = "SANs for certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SanForCertUpdate {
    #[doc = "DNS SANs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns: Vec<String>,
    #[doc = "IP address SANs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip: Vec<String>,
}
impl SanForCertUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka RemoteBrokerConnection Sasl Authentication properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaslRemoteBrokerBasicAuthentication {
    #[doc = "Kafka Sasl Authentication types"]
    #[serde(rename = "saslType")]
    pub sasl_type: KafkaSaslType,
    #[doc = "Kafka RemoteBrokerConnection Sasl Authentication token properties. NOTE - Enum only one method is allowed to be passed."]
    pub token: SaslRemoteBrokerBasicAuthenticationToken,
}
impl SaslRemoteBrokerBasicAuthentication {
    pub fn new(sasl_type: KafkaSaslType, token: SaslRemoteBrokerBasicAuthenticationToken) -> Self {
        Self { sasl_type, token }
    }
}
#[doc = "Kafka RemoteBrokerConnection Sasl Authentication token properties. NOTE - Enum only one method is allowed to be passed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaslRemoteBrokerBasicAuthenticationToken {
    #[doc = "Kafka Token KeyVault properties."]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KafkaTokenKeyVaultProperties>,
    #[doc = "Secret where cert details are stored."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
impl SaslRemoteBrokerBasicAuthenticationToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka RemoteBrokerConnection Sasl Authentication token properties. NOTE - Enum only one method is allowed to be passed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaslRemoteBrokerBasicAuthenticationTokenUpdate {
    #[doc = "Kafka Token KeyVault properties."]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KafkaTokenKeyVaultPropertiesUpdate>,
    #[doc = "Secret where cert details are stored."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
impl SaslRemoteBrokerBasicAuthenticationTokenUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kafka RemoteBrokerConnection Sasl Authentication properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaslRemoteBrokerBasicAuthenticationUpdate {
    #[doc = "Kafka Sasl Authentication types"]
    #[serde(rename = "saslType", default, skip_serializing_if = "Option::is_none")]
    pub sasl_type: Option<KafkaSaslType>,
    #[doc = "Kafka RemoteBrokerConnection Sasl Authentication token properties. NOTE - Enum only one method is allowed to be passed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<SaslRemoteBrokerBasicAuthenticationTokenUpdate>,
}
impl SaslRemoteBrokerBasicAuthenticationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kubernetes Service Types supported by Listener"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceType")]
pub enum ServiceType {
    #[serde(rename = "clusterIp")]
    ClusterIp,
    #[serde(rename = "loadBalancer")]
    LoadBalancer,
    #[serde(rename = "nodePort")]
    NodePort,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ClusterIp => serializer.serialize_unit_variant("ServiceType", 0u32, "clusterIp"),
            Self::LoadBalancer => serializer.serialize_unit_variant("ServiceType", 1u32, "loadBalancer"),
            Self::NodePort => serializer.serialize_unit_variant("ServiceType", 2u32, "nodePort"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Internal knobs of Resource Limits for FE and BE"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemporaryResourceLimitsConfig {
    #[doc = "Maximum number of messages a client can have inflight."]
    #[serde(rename = "maxInflightMessages")]
    pub max_inflight_messages: i32,
    #[doc = "Maximum number of patch inflight per node."]
    #[serde(rename = "maxInflightPatches")]
    pub max_inflight_patches: i32,
    #[doc = "Maximum number of patch a client can have in flight."]
    #[serde(rename = "maxInflightPatchesPerClient")]
    pub max_inflight_patches_per_client: i32,
    #[doc = "Maximum message expiry interval, in seconds."]
    #[serde(rename = "maxMessageExpirySecs", default, skip_serializing_if = "Option::is_none")]
    pub max_message_expiry_secs: Option<i64>,
    #[doc = "Maximum receive for external clients."]
    #[serde(rename = "maxQueuedMessages")]
    pub max_queued_messages: i64,
    #[doc = "Maximum receive QoS0 for external clients."]
    #[serde(rename = "maxQueuedQos0Messages")]
    pub max_queued_qos0_messages: i64,
    #[doc = "Maximum session expiry interval, in seconds."]
    #[serde(rename = "maxSessionExpirySecs")]
    pub max_session_expiry_secs: i64,
}
impl TemporaryResourceLimitsConfig {
    pub fn new(
        max_inflight_messages: i32,
        max_inflight_patches: i32,
        max_inflight_patches_per_client: i32,
        max_queued_messages: i64,
        max_queued_qos0_messages: i64,
        max_session_expiry_secs: i64,
    ) -> Self {
        Self {
            max_inflight_messages,
            max_inflight_patches,
            max_inflight_patches_per_client,
            max_message_expiry_secs: None,
            max_queued_messages,
            max_queued_qos0_messages,
            max_session_expiry_secs,
        }
    }
}
#[doc = "Internal knobs of Resource Limits for FE and BE"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TemporaryResourceLimitsConfigUpdate {
    #[doc = "Maximum number of messages a client can have inflight."]
    #[serde(rename = "maxInflightMessages", default, skip_serializing_if = "Option::is_none")]
    pub max_inflight_messages: Option<i32>,
    #[doc = "Maximum number of patch inflight per node."]
    #[serde(rename = "maxInflightPatches", default, skip_serializing_if = "Option::is_none")]
    pub max_inflight_patches: Option<i32>,
    #[doc = "Maximum number of patch a client can have in flight."]
    #[serde(rename = "maxInflightPatchesPerClient", default, skip_serializing_if = "Option::is_none")]
    pub max_inflight_patches_per_client: Option<i32>,
    #[doc = "Maximum message expiry interval, in seconds."]
    #[serde(rename = "maxMessageExpirySecs", default, skip_serializing_if = "Option::is_none")]
    pub max_message_expiry_secs: Option<i64>,
    #[doc = "Maximum receive for external clients."]
    #[serde(rename = "maxQueuedMessages", default, skip_serializing_if = "Option::is_none")]
    pub max_queued_messages: Option<i64>,
    #[doc = "Maximum receive QoS0 for external clients."]
    #[serde(rename = "maxQueuedQos0Messages", default, skip_serializing_if = "Option::is_none")]
    pub max_queued_qos0_messages: Option<i64>,
    #[doc = "Maximum session expiry interval, in seconds."]
    #[serde(rename = "maxSessionExpirySecs", default, skip_serializing_if = "Option::is_none")]
    pub max_session_expiry_secs: Option<i64>,
}
impl TemporaryResourceLimitsConfigUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of different TLS types, NOTE- Enum at a time only one of them needs to be supported"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TlsCertMethod {
    #[doc = "Automatic TLS server certificate management with cert-manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic: Option<AutomaticCertMethod>,
    #[doc = "Manual TLS server certificate management through a defined secret"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual: Option<ManualCertMethod>,
    #[doc = "KeyVault certificate properties"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultCertificateProperties>,
}
impl TlsCertMethod {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of different TLS types, NOTE- Enum at a time only one of them needs to be supported"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TlsCertMethodUpdate {
    #[doc = "Automatic TLS server certificate management with cert-manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic: Option<AutomaticCertMethodUpdate>,
    #[doc = "Manual TLS server certificate management through a defined secret"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual: Option<ManualCertMethodUpdate>,
    #[doc = "KeyVault certificate properties"]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultCertificatePropertiesUpdate>,
}
impl TlsCertMethodUpdate {
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
#[doc = "VolumeClaimDataSource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeClaimDataSource {
    #[doc = "APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required."]
    #[serde(rename = "apiGroup", default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    #[doc = "Kind is the type of resource being referenced"]
    pub kind: String,
    #[doc = "Name is the name of resource being referenced"]
    pub name: String,
}
impl VolumeClaimDataSource {
    pub fn new(kind: String, name: String) -> Self {
        Self {
            api_group: None,
            kind,
            name,
        }
    }
}
#[doc = "VolumeClaimDataSourceRef properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeClaimDataSourceRef {
    #[doc = "APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required."]
    #[serde(rename = "apiGroup", default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    #[doc = "Kind is the type of resource being referenced"]
    pub kind: String,
    #[doc = "Name is the name of resource being referenced"]
    pub name: String,
}
impl VolumeClaimDataSourceRef {
    pub fn new(kind: String, name: String) -> Self {
        Self {
            api_group: None,
            kind,
            name,
        }
    }
}
#[doc = "VolumeClaimDataSourceRef properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeClaimDataSourceRefUpdate {
    #[doc = "APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required."]
    #[serde(rename = "apiGroup", default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    #[doc = "Kind is the type of resource being referenced"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Name is the name of resource being referenced"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl VolumeClaimDataSourceRefUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VolumeClaimDataSource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeClaimDataSourceUpdate {
    #[doc = "APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required."]
    #[serde(rename = "apiGroup", default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    #[doc = "Kind is the type of resource being referenced"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Name is the name of resource being referenced"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl VolumeClaimDataSourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VolumeClaimResourceRequirements properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeClaimResourceRequirements {
    #[doc = "Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<serde_json::Value>,
    #[doc = "Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<serde_json::Value>,
}
impl VolumeClaimResourceRequirements {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VolumeClaimSpec properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeClaimSpec {
    #[doc = "AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1"]
    #[serde(
        rename = "accessModes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub access_modes: Vec<String>,
    #[doc = "VolumeClaimDataSource properties"]
    #[serde(rename = "dataSource", default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<VolumeClaimDataSource>,
    #[doc = "VolumeClaimDataSourceRef properties"]
    #[serde(rename = "dataSourceRef", default, skip_serializing_if = "Option::is_none")]
    pub data_source_ref: Option<VolumeClaimDataSourceRef>,
    #[doc = "VolumeClaimResourceRequirements properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<VolumeClaimResourceRequirements>,
    #[doc = "VolumeClaimSpecSelector properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<VolumeClaimSpecSelector>,
    #[doc = "Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1"]
    #[serde(rename = "storageClassName", default, skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,
    #[doc = "volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec. This is a beta feature."]
    #[serde(rename = "volumeMode", default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<String>,
    #[doc = "VolumeName is the binding reference to the PersistentVolume backing this claim."]
    #[serde(rename = "volumeName", default, skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
}
impl VolumeClaimSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VolumeClaimSpecSelector properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeClaimSpecSelector {
    #[doc = "MatchExpressions is a list of label selector requirements. The requirements are ANDed."]
    #[serde(
        rename = "matchExpressions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_expressions: Vec<VolumeClaimSpecSelectorMatchExpressions>,
    #[doc = "MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is \"key\", the operator is \"In\", and the values array contains only \"value\". The requirements are ANDed."]
    #[serde(rename = "matchLabels", default, skip_serializing_if = "Option::is_none")]
    pub match_labels: Option<serde_json::Value>,
}
impl VolumeClaimSpecSelector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VolumeClaimSpecSelectorMatchExpressions properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeClaimSpecSelectorMatchExpressions {
    #[doc = "key is the label key that the selector applies to."]
    pub key: String,
    #[doc = "operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist."]
    pub operator: String,
    #[doc = "values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl VolumeClaimSpecSelectorMatchExpressions {
    pub fn new(key: String, operator: String) -> Self {
        Self {
            key,
            operator,
            values: Vec::new(),
        }
    }
}
#[doc = "VolumeClaimSpec properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeClaimSpecUpdate {
    #[doc = "AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1"]
    #[serde(
        rename = "accessModes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub access_modes: Vec<String>,
    #[doc = "VolumeClaimDataSource properties"]
    #[serde(rename = "dataSource", default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<VolumeClaimDataSourceUpdate>,
    #[doc = "VolumeClaimDataSourceRef properties"]
    #[serde(rename = "dataSourceRef", default, skip_serializing_if = "Option::is_none")]
    pub data_source_ref: Option<VolumeClaimDataSourceRefUpdate>,
    #[doc = "VolumeClaimResourceRequirements properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<VolumeClaimResourceRequirements>,
    #[doc = "VolumeClaimSpecSelector properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<VolumeClaimSpecSelector>,
    #[doc = "Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1"]
    #[serde(rename = "storageClassName", default, skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,
    #[doc = "volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec. This is a beta feature."]
    #[serde(rename = "volumeMode", default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<String>,
    #[doc = "VolumeName is the binding reference to the PersistentVolume backing this claim."]
    #[serde(rename = "volumeName", default, skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
}
impl VolumeClaimSpecUpdate {
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
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
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
