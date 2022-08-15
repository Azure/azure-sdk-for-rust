#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "DNS server details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryConnectorDnsDetails {
    #[doc = "DNS domain name for which DNS lookups should be forwarded to the Active Directory DNS servers."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "List of Active Directory DNS server IP addresses."]
    #[serde(rename = "nameserverIPAddresses")]
    pub nameserver_ip_addresses: Vec<String>,
    #[doc = "Replica count for DNS proxy service. Default value is 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    #[doc = "Flag indicating whether to prefer Kubernetes DNS server response over AD DNS server response for IP address lookups."]
    #[serde(rename = "preferK8sDnsForPtrLookups", default, skip_serializing_if = "Option::is_none")]
    pub prefer_k8s_dns_for_ptr_lookups: Option<bool>,
}
impl ActiveDirectoryConnectorDnsDetails {
    pub fn new(nameserver_ip_addresses: Vec<String>) -> Self {
        Self {
            domain_name: None,
            nameserver_ip_addresses,
            replicas: None,
            prefer_k8s_dns_for_ptr_lookups: None,
        }
    }
}
#[doc = "Active Directory domain details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryConnectorDomainDetails {
    #[doc = "Name (uppercase) of the Active Directory domain that this AD connector will be associated with."]
    pub realm: String,
    #[doc = "NETBIOS name of the Active Directory domain."]
    #[serde(rename = "netbiosDomainName", default, skip_serializing_if = "Option::is_none")]
    pub netbios_domain_name: Option<String>,
    #[doc = "The service account provisioning mode for this Active Directory connector."]
    #[serde(rename = "serviceAccountProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub service_account_provisioning: Option<active_directory_connector_domain_details::ServiceAccountProvisioning>,
    #[doc = "The distinguished name of the Active Directory Organizational Unit."]
    #[serde(rename = "ouDistinguishedName", default, skip_serializing_if = "Option::is_none")]
    pub ou_distinguished_name: Option<String>,
    #[doc = "Details about the Active Directory domain controllers associated with this AD connector instance"]
    #[serde(rename = "domainControllers")]
    pub domain_controllers: ActiveDirectoryDomainControllers,
}
impl ActiveDirectoryConnectorDomainDetails {
    pub fn new(realm: String, domain_controllers: ActiveDirectoryDomainControllers) -> Self {
        Self {
            realm,
            netbios_domain_name: None,
            service_account_provisioning: None,
            ou_distinguished_name: None,
            domain_controllers,
        }
    }
}
pub mod active_directory_connector_domain_details {
    use super::*;
    #[doc = "The service account provisioning mode for this Active Directory connector."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceAccountProvisioning")]
    pub enum ServiceAccountProvisioning {
        #[serde(rename = "automatic")]
        Automatic,
        #[serde(rename = "manual")]
        Manual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceAccountProvisioning {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceAccountProvisioning {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceAccountProvisioning {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("ServiceAccountProvisioning", 0u32, "automatic"),
                Self::Manual => serializer.serialize_unit_variant("ServiceAccountProvisioning", 1u32, "manual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ServiceAccountProvisioning {
        fn default() -> Self {
            Self::Manual
        }
    }
}
#[doc = "A list of active directory connectors"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryConnectorListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActiveDirectoryConnectorResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ActiveDirectoryConnectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ActiveDirectoryConnectorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an Active Directory connector resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryConnectorProperties {
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "domainServiceAccountLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub domain_service_account_login_information: Option<BasicLoginInformation>,
    #[doc = "The provisioning state of the Active Directory connector resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The specifications of the AD Kubernetes resource."]
    pub spec: ActiveDirectoryConnectorSpec,
    #[doc = "The status of the Kubernetes custom resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ActiveDirectoryConnectorStatus>,
}
impl ActiveDirectoryConnectorProperties {
    pub fn new(spec: ActiveDirectoryConnectorSpec) -> Self {
        Self {
            domain_service_account_login_information: None,
            provisioning_state: None,
            spec,
            status: None,
        }
    }
}
#[doc = "Active directory connector resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryConnectorResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an Active Directory connector resource"]
    pub properties: ActiveDirectoryConnectorProperties,
}
impl ActiveDirectoryConnectorResource {
    pub fn new(properties: ActiveDirectoryConnectorProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The specifications of the AD Kubernetes resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryConnectorSpec {
    #[doc = "Active Directory domain details"]
    #[serde(rename = "activeDirectory")]
    pub active_directory: ActiveDirectoryConnectorDomainDetails,
    #[doc = "DNS server details"]
    pub dns: ActiveDirectoryConnectorDnsDetails,
}
impl ActiveDirectoryConnectorSpec {
    pub fn new(active_directory: ActiveDirectoryConnectorDomainDetails, dns: ActiveDirectoryConnectorDnsDetails) -> Self {
        Self { active_directory, dns }
    }
}
#[doc = "The status of the Kubernetes custom resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryConnectorStatus {
    #[doc = "The time that the custom resource was last updated."]
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[doc = "The version of the replicaSet associated with the AD connector custom resource."]
    #[serde(rename = "observedGeneration", default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[doc = "The state of the AD connector custom resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl ActiveDirectoryConnectorStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a domain controller in the AD domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryDomainController {
    #[doc = "Fully-qualified domain name of a domain controller in the AD domain."]
    pub hostname: String,
}
impl ActiveDirectoryDomainController {
    pub fn new(hostname: String) -> Self {
        Self { hostname }
    }
}
#[doc = "Details about the Active Directory domain controllers associated with this AD connector instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryDomainControllers {
    #[doc = "Information about a domain controller in the AD domain."]
    #[serde(rename = "primaryDomainController", default, skip_serializing_if = "Option::is_none")]
    pub primary_domain_controller: Option<ActiveDirectoryDomainController>,
    #[doc = "Information about the secondary domain controllers in the AD domain."]
    #[serde(rename = "secondaryDomainControllers", default, skip_serializing_if = "Option::is_none")]
    pub secondary_domain_controllers: Option<ActiveDirectorySecondaryDomainControllers>,
}
impl ActiveDirectoryDomainControllers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory information that related to the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryInformation {
    #[doc = "Keytab used for authenticate with Active Directory."]
    #[serde(rename = "keytabInformation", default, skip_serializing_if = "Option::is_none")]
    pub keytab_information: Option<KeytabInformation>,
}
impl ActiveDirectoryInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ActiveDirectorySecondaryDomainControllers = Vec<ActiveDirectoryDomainController>;
#[doc = "Username and password for basic login authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BasicLoginInformation {
    #[doc = "Login username."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Login password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl BasicLoginInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition representing SKU for ARM resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonSku {
    #[doc = "The name of the SKU.  It is typically a letter+number code"]
    pub name: String,
    #[doc = "Whether dev/test is enabled. When the dev field is set to true, the resource is used for dev/test purpose. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev: Option<bool>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl CommonSku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dev: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "The data controller properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataControllerProperties {
    #[doc = "The infrastructure the data controller is running on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<data_controller_properties::Infrastructure>,
    #[doc = "Properties from the Kubernetes data controller"]
    #[serde(rename = "onPremiseProperty", default, skip_serializing_if = "Option::is_none")]
    pub on_premise_property: Option<OnPremiseProperty>,
    #[doc = "The raw kubernetes information"]
    #[serde(rename = "k8sRaw", default, skip_serializing_if = "Option::is_none")]
    pub k8s_raw: Option<serde_json::Value>,
    #[doc = "Properties on upload watermark.  Mostly timestamp for each upload data type"]
    #[serde(rename = "uploadWatermark", default, skip_serializing_if = "Option::is_none")]
    pub upload_watermark: Option<UploadWatermark>,
    #[doc = "Last uploaded date from Kubernetes cluster. Defaults to current date time"]
    #[serde(rename = "lastUploadedDate", with = "azure_core::date::rfc3339::option")]
    pub last_uploaded_date: Option<time::OffsetDateTime>,
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "basicLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub basic_login_information: Option<BasicLoginInformation>,
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "metricsDashboardCredential", default, skip_serializing_if = "Option::is_none")]
    pub metrics_dashboard_credential: Option<BasicLoginInformation>,
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "logsDashboardCredential", default, skip_serializing_if = "Option::is_none")]
    pub logs_dashboard_credential: Option<BasicLoginInformation>,
    #[doc = "Log analytics workspace id and primary key"]
    #[serde(rename = "logAnalyticsWorkspaceConfig", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_workspace_config: Option<LogAnalyticsWorkspaceConfig>,
    #[doc = "Service principal for uploading billing, metrics and logs."]
    #[serde(rename = "uploadServicePrincipal", default, skip_serializing_if = "Option::is_none")]
    pub upload_service_principal: Option<UploadServicePrincipal>,
    #[doc = "The provisioning state of the Arc Data Controller resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "If a CustomLocation is provided, this contains the ARM id of the connected cluster the custom location belongs to."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "If a CustomLocation is provided, this contains the ARM id of the extension the custom location belongs to."]
    #[serde(rename = "extensionId", default, skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
}
impl DataControllerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_controller_properties {
    use super::*;
    #[doc = "The infrastructure the data controller is running on."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Infrastructure {
        #[serde(rename = "azure")]
        Azure,
        #[serde(rename = "gcp")]
        Gcp,
        #[serde(rename = "aws")]
        Aws,
        #[serde(rename = "alibaba")]
        Alibaba,
        #[serde(rename = "onpremises")]
        Onpremises,
        #[serde(rename = "other")]
        Other,
    }
    impl Default for Infrastructure {
        fn default() -> Self {
            Self::Other
        }
    }
}
#[doc = "Data controller resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataControllerResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "The data controller properties."]
    pub properties: DataControllerProperties,
}
impl DataControllerResource {
    pub fn new(tracked_resource: TrackedResource, properties: DataControllerProperties) -> Self {
        Self {
            tracked_resource,
            extended_location: None,
            properties,
        }
    }
}
#[doc = "Used for updating a data controller resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataControllerUpdate {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The data controller properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataControllerProperties>,
}
impl DataControllerUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Azure Data on Azure Arc service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "An error response from the Batch service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
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
#[doc = "An error response from the Batch service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponseBody>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The complex type of the extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "The name of the extended location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of extendedLocation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ExtendedLocationType>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of extendedLocation."]
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
#[doc = "The kubernetes resource limits and requests used to restrict or reserve resource usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8sResourceRequirements {
    #[doc = "Requests for a kubernetes resource type (e.g 'cpu', 'memory'). The 'cpu' request must be less than or equal to 'cpu' limit. Default 'cpu' is 2, minimum is 1. Default 'memory' is '4Gi', minimum is '2Gi. If sku.tier is GeneralPurpose, maximum 'cpu' is 24 and maximum 'memory' is '128Gi'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<serde_json::Value>,
    #[doc = "Limits for a kubernetes resource type (e.g 'cpu', 'memory'). The 'cpu' request must be less than or equal to 'cpu' limit. Default 'cpu' is 2, minimum is 1. Default 'memory' is '4Gi', minimum is '2Gi. If sku.tier is GeneralPurpose, maximum 'cpu' is 24 and maximum 'memory' is '128Gi'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<serde_json::Value>,
}
impl K8sResourceRequirements {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kubernetes scheduling information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8sScheduling {
    #[doc = "The kubernetes scheduling options. It describes restrictions used to help Kubernetes select appropriate nodes to host the database service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<K8sSchedulingOptions>,
}
impl K8sScheduling {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kubernetes scheduling options. It describes restrictions used to help Kubernetes select appropriate nodes to host the database service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8sSchedulingOptions {
    #[doc = "The kubernetes resource limits and requests used to restrict or reserve resource usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<K8sResourceRequirements>,
}
impl K8sSchedulingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Keytab used for authenticate with Active Directory."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeytabInformation {
    #[doc = "A base64-encoded keytab."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keytab: Option<String>,
}
impl KeytabInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log analytics workspace id and primary key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsWorkspaceConfig {
    #[doc = "Azure Log Analytics workspace ID"]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "Primary key of the workspace"]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
}
impl LogAnalyticsWorkspaceConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties from the Kubernetes data controller"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnPremiseProperty {
    #[doc = "A globally unique ID identifying the associated Kubernetes cluster"]
    pub id: String,
    #[doc = "Certificate that contains the Kubernetes cluster public key used to verify signing"]
    #[serde(rename = "publicSigningKey")]
    pub public_signing_key: String,
    #[doc = "Unique thumbprint returned to customer to verify the certificate being uploaded"]
    #[serde(rename = "signingCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub signing_certificate_thumbprint: Option<String>,
}
impl OnPremiseProperty {
    pub fn new(id: String, public_signing_key: String) -> Self {
        Self {
            id,
            public_signing_key,
            signing_certificate_thumbprint: None,
        }
    }
}
#[doc = "Azure Data Services on Azure Arc operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object."]
    pub name: String,
    #[doc = "Display metadata associated with the operation."]
    pub display: OperationDisplay,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction")]
    pub is_data_action: bool,
    #[doc = "Additional descriptions for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new(name: String, display: OperationDisplay, is_data_action: bool) -> Self {
        Self {
            name,
            display,
            origin: None,
            is_data_action,
            properties: None,
        }
    }
}
pub mod operation {
    use super::*;
    #[doc = "The intended executor of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Display metadata associated with the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[doc = "The localized friendly form of the resource provider name."]
    pub provider: String,
    #[doc = "The localized friendly form of the resource type related to this action/operation."]
    pub resource: String,
    #[doc = "The localized friendly name for the operation."]
    pub operation: String,
    #[doc = "The localized friendly description for the operation."]
    pub description: String,
}
impl OperationDisplay {
    pub fn new(provider: String, resource: String, operation: String, description: String) -> Self {
        Self {
            provider,
            resource,
            operation,
            description,
        }
    }
}
#[doc = "Result of the request to list Azure Data Services on Azure Arc operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "Link to retrieve next page of results."]
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
#[doc = "A list of data controllers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageOfDataControllerResource {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataControllerResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageOfDataControllerResource {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageOfDataControllerResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Postgres Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgresInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Postgres Instance properties."]
    pub properties: PostgresInstanceProperties,
    #[doc = "The resource model definition representing SKU for Azure Database for PostgresSQL - Azure Arc"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PostgresInstanceSku>,
}
impl PostgresInstance {
    pub fn new(tracked_resource: TrackedResource, properties: PostgresInstanceProperties) -> Self {
        Self {
            tracked_resource,
            extended_location: None,
            properties,
            sku: None,
        }
    }
}
#[doc = "A list of PostgresInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostgresInstanceListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PostgresInstance>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PostgresInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PostgresInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Postgres Instance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostgresInstanceProperties {
    #[doc = "The data controller id"]
    #[serde(rename = "dataControllerId", default, skip_serializing_if = "Option::is_none")]
    pub data_controller_id: Option<String>,
    #[doc = "The instance admin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<String>,
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "basicLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub basic_login_information: Option<BasicLoginInformation>,
    #[doc = "The raw kubernetes information"]
    #[serde(rename = "k8sRaw", default, skip_serializing_if = "Option::is_none")]
    pub k8s_raw: Option<serde_json::Value>,
    #[doc = "Last uploaded date from Kubernetes cluster. Defaults to current date time"]
    #[serde(rename = "lastUploadedDate", with = "azure_core::date::rfc3339::option")]
    pub last_uploaded_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the Azure Arc-enabled PostgreSQL instance."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PostgresInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition representing SKU for Azure Database for PostgresSQL - Azure Arc"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgresInstanceSku {
    #[serde(flatten)]
    pub common_sku: CommonSku,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<postgres_instance_sku::Tier>,
}
impl PostgresInstanceSku {
    pub fn new(common_sku: CommonSku) -> Self {
        Self { common_sku, tier: None }
    }
}
pub mod postgres_instance_sku {
    use super::*;
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Hyperscale,
    }
    impl Default for Tier {
        fn default() -> Self {
            Self::Hyperscale
        }
    }
}
#[doc = "An update to a Postgres Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostgresInstanceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Postgres Instance properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PostgresInstanceProperties>,
}
impl PostgresInstanceUpdate {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "A SqlManagedInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlManagedInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of sqlManagedInstance."]
    pub properties: SqlManagedInstanceProperties,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "The resource model definition representing SKU for Azure Managed Instance - Azure Arc"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SqlManagedInstanceSku>,
}
impl SqlManagedInstance {
    pub fn new(tracked_resource: TrackedResource, properties: SqlManagedInstanceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            extended_location: None,
            sku: None,
        }
    }
}
#[doc = "The raw kubernetes information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceK8sRaw {
    #[doc = "The kubernetes spec information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<SqlManagedInstanceK8sSpec>,
}
impl SqlManagedInstanceK8sRaw {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kubernetes spec information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceK8sSpec {
    #[doc = "The kubernetes scheduling information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<K8sScheduling>,
    #[doc = "This option specifies the number of SQL Managed Instance replicas that will be deployed in your Kubernetes cluster for high availability purposes. If sku.tier is BusinessCritical, allowed values are '2' or '3' with default of '3'. If sku.tier is GeneralPurpose, replicas must be '1'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}
impl SqlManagedInstanceK8sSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of SqlManagedInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlManagedInstance>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlManagedInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlManagedInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of sqlManagedInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceProperties {
    #[doc = "null"]
    #[serde(rename = "dataControllerId", default, skip_serializing_if = "Option::is_none")]
    pub data_controller_id: Option<String>,
    #[doc = "The instance admin user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<String>,
    #[doc = "The instance start time"]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The instance end time"]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The raw kubernetes information."]
    #[serde(rename = "k8sRaw", default, skip_serializing_if = "Option::is_none")]
    pub k8s_raw: Option<SqlManagedInstanceK8sRaw>,
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "basicLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub basic_login_information: Option<BasicLoginInformation>,
    #[doc = "Last uploaded date from Kubernetes cluster. Defaults to current date time"]
    #[serde(rename = "lastUploadedDate", with = "azure_core::date::rfc3339::option")]
    pub last_uploaded_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the Arc-enabled SQL Managed Instance resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Active Directory information that related to the resource."]
    #[serde(rename = "activeDirectoryInformation", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_information: Option<ActiveDirectoryInformation>,
    #[doc = "The license type to apply for this managed instance."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<sql_managed_instance_properties::LicenseType>,
    #[doc = "If a CustomLocation is provided, this contains the ARM id of the connected cluster the custom location belongs to."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "If a CustomLocation is provided, this contains the ARM id of the extension the custom location belongs to."]
    #[serde(rename = "extensionId", default, skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
}
impl SqlManagedInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_managed_instance_properties {
    use super::*;
    #[doc = "The license type to apply for this managed instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        BasePrice,
        LicenseIncluded,
        DisasterRecovery,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BasePrice => serializer.serialize_unit_variant("LicenseType", 0u32, "BasePrice"),
                Self::LicenseIncluded => serializer.serialize_unit_variant("LicenseType", 1u32, "LicenseIncluded"),
                Self::DisasterRecovery => serializer.serialize_unit_variant("LicenseType", 2u32, "DisasterRecovery"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for LicenseType {
        fn default() -> Self {
            Self::BasePrice
        }
    }
}
#[doc = "The resource model definition representing SKU for Azure Managed Instance - Azure Arc"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlManagedInstanceSku {
    #[doc = "The name of the SKU."]
    pub name: sql_managed_instance_sku::Name,
    #[doc = "The pricing tier for the instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sql_managed_instance_sku::Tier>,
    #[doc = "Whether dev/test is enabled. When the dev field is set to true, the resource is used for dev/test purpose. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev: Option<bool>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The SKU family"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The SKU capacity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl SqlManagedInstanceSku {
    pub fn new(name: sql_managed_instance_sku::Name) -> Self {
        Self {
            name,
            tier: None,
            dev: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
pub mod sql_managed_instance_sku {
    use super::*;
    #[doc = "The name of the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "vCore")]
        VCore,
    }
    #[doc = "The pricing tier for the instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        GeneralPurpose,
        BusinessCritical,
    }
    impl Default for Tier {
        fn default() -> Self {
            Self::GeneralPurpose
        }
    }
}
#[doc = "An update to a SQL Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlManagedInstanceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A SqlServerInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of SqlServerInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerInstanceProperties>,
}
impl SqlServerInstance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A list of SqlServerInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlServerInstance>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlServerInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlServerInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of SqlServerInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerInstanceProperties {
    #[doc = "SQL Server version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<sql_server_instance_properties::Version>,
    #[doc = "SQL Server edition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<sql_server_instance_properties::Edition>,
    #[doc = "ARM Resource id of the container resource (Azure Arc for Servers)."]
    #[serde(rename = "containerResourceId")]
    pub container_resource_id: String,
    #[doc = "The time when the resource was created."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[doc = "The number of logical processors used by the SQL Server instance."]
    #[serde(rename = "vCore", default, skip_serializing_if = "Option::is_none")]
    pub v_core: Option<String>,
    #[doc = "The cloud connectivity status."]
    pub status: sql_server_instance_properties::Status,
    #[doc = "SQL Server update level."]
    #[serde(rename = "patchLevel", default, skip_serializing_if = "Option::is_none")]
    pub patch_level: Option<String>,
    #[doc = "SQL Server collation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "SQL Server current version."]
    #[serde(rename = "currentVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[doc = "SQL Server instance name."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "Dynamic TCP ports used by SQL Server."]
    #[serde(rename = "tcpDynamicPorts", default, skip_serializing_if = "Option::is_none")]
    pub tcp_dynamic_ports: Option<String>,
    #[doc = "Static TCP ports used by SQL Server."]
    #[serde(rename = "tcpStaticPorts", default, skip_serializing_if = "Option::is_none")]
    pub tcp_static_ports: Option<String>,
    #[doc = "SQL Server product ID."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "SQL Server license type."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<sql_server_instance_properties::LicenseType>,
    #[doc = "Timestamp of last Azure Defender status update."]
    #[serde(rename = "azureDefenderStatusLastUpdated", with = "azure_core::date::rfc3339::option")]
    pub azure_defender_status_last_updated: Option<time::OffsetDateTime>,
    #[doc = "Status of Azure Defender."]
    #[serde(rename = "azureDefenderStatus", default, skip_serializing_if = "Option::is_none")]
    pub azure_defender_status: Option<sql_server_instance_properties::AzureDefenderStatus>,
    #[doc = "The provisioning state of the Arc-enabled SQL Server resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Type of host for Azure Arc SQL Server"]
    #[serde(rename = "hostType", default, skip_serializing_if = "Option::is_none")]
    pub host_type: Option<sql_server_instance_properties::HostType>,
}
impl SqlServerInstanceProperties {
    pub fn new(container_resource_id: String, status: sql_server_instance_properties::Status) -> Self {
        Self {
            version: None,
            edition: None,
            container_resource_id,
            create_time: None,
            v_core: None,
            status,
            patch_level: None,
            collation: None,
            current_version: None,
            instance_name: None,
            tcp_dynamic_ports: None,
            tcp_static_ports: None,
            product_id: None,
            license_type: None,
            azure_defender_status_last_updated: None,
            azure_defender_status: None,
            provisioning_state: None,
            host_type: None,
        }
    }
}
pub mod sql_server_instance_properties {
    use super::*;
    #[doc = "SQL Server version."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Version")]
    pub enum Version {
        #[serde(rename = "SQL Server 2012")]
        SqlServer2012,
        #[serde(rename = "SQL Server 2014")]
        SqlServer2014,
        #[serde(rename = "SQL Server 2016")]
        SqlServer2016,
        #[serde(rename = "SQL Server 2017")]
        SqlServer2017,
        #[serde(rename = "SQL Server 2019")]
        SqlServer2019,
        #[serde(rename = "SQL Server 2022")]
        SqlServer2022,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Version {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Version {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Version {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SqlServer2012 => serializer.serialize_unit_variant("Version", 0u32, "SQL Server 2012"),
                Self::SqlServer2014 => serializer.serialize_unit_variant("Version", 1u32, "SQL Server 2014"),
                Self::SqlServer2016 => serializer.serialize_unit_variant("Version", 2u32, "SQL Server 2016"),
                Self::SqlServer2017 => serializer.serialize_unit_variant("Version", 3u32, "SQL Server 2017"),
                Self::SqlServer2019 => serializer.serialize_unit_variant("Version", 4u32, "SQL Server 2019"),
                Self::SqlServer2022 => serializer.serialize_unit_variant("Version", 5u32, "SQL Server 2022"),
                Self::Unknown => serializer.serialize_unit_variant("Version", 6u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SQL Server edition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Edition")]
    pub enum Edition {
        Evaluation,
        Enterprise,
        Standard,
        Web,
        Developer,
        Express,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Edition {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Edition {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Edition {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Evaluation => serializer.serialize_unit_variant("Edition", 0u32, "Evaluation"),
                Self::Enterprise => serializer.serialize_unit_variant("Edition", 1u32, "Enterprise"),
                Self::Standard => serializer.serialize_unit_variant("Edition", 2u32, "Standard"),
                Self::Web => serializer.serialize_unit_variant("Edition", 3u32, "Web"),
                Self::Developer => serializer.serialize_unit_variant("Edition", 4u32, "Developer"),
                Self::Express => serializer.serialize_unit_variant("Edition", 5u32, "Express"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The cloud connectivity status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Connected,
        Disconnected,
        Registered,
        Unknown,
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
                Self::Connected => serializer.serialize_unit_variant("Status", 0u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 1u32, "Disconnected"),
                Self::Registered => serializer.serialize_unit_variant("Status", 2u32, "Registered"),
                Self::Unknown => serializer.serialize_unit_variant("Status", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        Paid,
        Free,
        #[serde(rename = "HADR")]
        Hadr,
        Undefined,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Paid => serializer.serialize_unit_variant("LicenseType", 0u32, "Paid"),
                Self::Free => serializer.serialize_unit_variant("LicenseType", 1u32, "Free"),
                Self::Hadr => serializer.serialize_unit_variant("LicenseType", 2u32, "HADR"),
                Self::Undefined => serializer.serialize_unit_variant("LicenseType", 3u32, "Undefined"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of Azure Defender."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AzureDefenderStatus")]
    pub enum AzureDefenderStatus {
        Protected,
        Unprotected,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AzureDefenderStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AzureDefenderStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AzureDefenderStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Protected => serializer.serialize_unit_variant("AzureDefenderStatus", 0u32, "Protected"),
                Self::Unprotected => serializer.serialize_unit_variant("AzureDefenderStatus", 1u32, "Unprotected"),
                Self::Unknown => serializer.serialize_unit_variant("AzureDefenderStatus", 2u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of host for Azure Arc SQL Server"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostType")]
    pub enum HostType {
        #[serde(rename = "Virtual Machine")]
        VirtualMachine,
        #[serde(rename = "Physical Server")]
        PhysicalServer,
        #[serde(rename = "AWS Virtual Machine")]
        AwsVirtualMachine,
        #[serde(rename = "GCP Virtual Machine")]
        GcpVirtualMachine,
        Other,
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
                Self::VirtualMachine => serializer.serialize_unit_variant("HostType", 0u32, "Virtual Machine"),
                Self::PhysicalServer => serializer.serialize_unit_variant("HostType", 1u32, "Physical Server"),
                Self::AwsVirtualMachine => serializer.serialize_unit_variant("HostType", 2u32, "AWS Virtual Machine"),
                Self::GcpVirtualMachine => serializer.serialize_unit_variant("HostType", 3u32, "GCP Virtual Machine"),
                Self::Other => serializer.serialize_unit_variant("HostType", 4u32, "Other"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An update to a SQL Server Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlServerInstanceUpdate {
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
#[doc = "Service principal for uploading billing, metrics and logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadServicePrincipal {
    #[doc = "Client ID of the service principal for uploading data."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Tenant ID of the service principal."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Authority for the service principal. Example: https://login.microsoftonline.com/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[doc = "Secret of the service principal"]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}
impl UploadServicePrincipal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties on upload watermark.  Mostly timestamp for each upload data type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadWatermark {
    #[doc = "Last uploaded date for metrics from kubernetes cluster. Defaults to current date time"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub metrics: Option<time::OffsetDateTime>,
    #[doc = "Last uploaded date for logs from kubernetes cluster. Defaults to current date time"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub logs: Option<time::OffsetDateTime>,
    #[doc = "Last uploaded date for usages from kubernetes cluster. Defaults to current date time"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub usages: Option<time::OffsetDateTime>,
}
impl UploadWatermark {
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
