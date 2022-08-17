#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Description of a NotificationHub AdmCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdmCredential {
    #[doc = "Description of a NotificationHub AdmCredential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdmCredentialProperties>,
}
impl AdmCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub AdmCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdmCredentialProperties {
    #[doc = "The client identifier."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The credential secret access key."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "The URL of the authorization token."]
    #[serde(rename = "authTokenUrl", default, skip_serializing_if = "Option::is_none")]
    pub auth_token_url: Option<String>,
}
impl AdmCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub ApnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApnsCredential {
    #[doc = "Description of a NotificationHub ApnsCredential. Note that there is no explicit switch between Certificate and Token Authentication Modes. The mode is determined based on the properties passed in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApnsCredentialProperties>,
}
impl ApnsCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub ApnsCredential. Note that there is no explicit switch between Certificate and Token Authentication Modes. The mode is determined based on the properties passed in."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApnsCredentialProperties {
    #[doc = "The APNS certificate. Specify if using Certificate Authentication Mode."]
    #[serde(rename = "apnsCertificate", default, skip_serializing_if = "Option::is_none")]
    pub apns_certificate: Option<String>,
    #[doc = "The APNS certificate password if it exists."]
    #[serde(rename = "certificateKey", default, skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[doc = "The APNS endpoint of this credential. If using Certificate Authentication Mode and Sandbox specify 'gateway.sandbox.push.apple.com'. If using Certificate Authentication Mode and Production specify 'gateway.push.apple.com'. If using Token Authentication Mode and Sandbox specify 'https://api.development.push.apple.com:443/3/device'. If using Token Authentication Mode and Production specify 'https://api.push.apple.com:443/3/device'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "The APNS certificate thumbprint. Specify if using Certificate Authentication Mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "A 10-character key identifier (kid) key, obtained from your developer account. Specify if using Token Authentication Mode."]
    #[serde(rename = "keyId", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[doc = "The name of the application or BundleId. Specify if using Token Authentication Mode."]
    #[serde(rename = "appName", default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[doc = "The issuer (iss) registered claim key. The value is a 10-character TeamId, obtained from your developer account. Specify if using Token Authentication Mode."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "Provider Authentication Token, obtained through your developer account. Specify if using Token Authentication Mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl ApnsCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub BaiduCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaiduCredential {
    #[doc = "Description of a NotificationHub BaiduCredential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BaiduCredentialProperties>,
}
impl BaiduCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub BaiduCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaiduCredentialProperties {
    #[doc = "Baidu Api Key."]
    #[serde(rename = "baiduApiKey", default, skip_serializing_if = "Option::is_none")]
    pub baidu_api_key: Option<String>,
    #[doc = "Baidu Endpoint."]
    #[serde(rename = "baiduEndPoint", default, skip_serializing_if = "Option::is_none")]
    pub baidu_end_point: Option<String>,
    #[doc = "Baidu Secret Key"]
    #[serde(rename = "baiduSecretKey", default, skip_serializing_if = "Option::is_none")]
    pub baidu_secret_key: Option<String>,
}
impl BaiduCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Check Name Availability for Namespace and NotificationHubs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAvailabilityParameters {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    pub name: String,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The Sku description for a namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "True if the name is available and can be used to create new Namespace/NotificationHub. Otherwise false."]
    #[serde(rename = "isAvailiable", default, skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
}
impl CheckAvailabilityParameters {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            type_: None,
            location: None,
            tags: None,
            sku: None,
            is_availiable: None,
        }
    }
}
#[doc = "Description of a CheckAvailability resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckAvailabilityResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "True if the name is available and can be used to create new Namespace/NotificationHub. Otherwise false."]
    #[serde(rename = "isAvailiable", default, skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
}
impl CheckAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Debug Send payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebugSendParameters {}
impl DebugSendParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebugSendResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DebugSendResult>,
}
impl DebugSendResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebugSendResult {
    #[doc = "successful send"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<f64>,
    #[doc = "send failure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<f64>,
    #[doc = "actual failure description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<serde_json::Value>,
}
impl DebugSendResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates NotificationHubs service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code."]
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
#[doc = "Description of a NotificationHub GcmCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GcmCredential {
    #[doc = "Description of a NotificationHub GcmCredential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GcmCredentialProperties>,
}
impl GcmCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub GcmCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GcmCredentialProperties {
    #[doc = "The FCM legacy endpoint. Default value is 'https://fcm.googleapis.com/fcm/send'"]
    #[serde(rename = "gcmEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub gcm_endpoint: Option<String>,
    #[doc = "The Google API key."]
    #[serde(rename = "googleApiKey", default, skip_serializing_if = "Option::is_none")]
    pub google_api_key: Option<String>,
}
impl GcmCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub MpnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MpnsCredential {
    #[doc = "Description of a NotificationHub MpnsCredential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MpnsCredentialProperties>,
}
impl MpnsCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub MpnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MpnsCredentialProperties {
    #[doc = "The MPNS certificate."]
    #[serde(rename = "mpnsCertificate", default, skip_serializing_if = "Option::is_none")]
    pub mpns_certificate: Option<String>,
    #[doc = "The certificate key for this credential."]
    #[serde(rename = "certificateKey", default, skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[doc = "The MPNS certificate Thumbprint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
impl MpnsCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the CreateOrUpdate Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Namespace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
}
impl NamespaceCreateOrUpdateParameters {
    pub fn new() -> Self {
        Self {
            resource: Resource::default(),
            properties: None,
        }
    }
}
#[doc = "The response of the List Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceListResult {
    #[doc = "Result of the List Namespace operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NamespaceResource>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of Namespaces"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NamespaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NamespaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Patch Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespacePatchParameters {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The Sku description for a namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl NamespacePatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Namespace properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceProperties {
    #[doc = "The name of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Provisioning state of the Namespace."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Specifies the targeted region in which the namespace should be created. It can be any of the following values: Australia East, Australia Southeast, Central US, East US, East US 2, West US, North Central US, South Central US, East Asia, Southeast Asia, Brazil South, Japan East, Japan West, North Europe, West Europe"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Identifier for Azure Insights metrics"]
    #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    #[doc = "Status of the namespace. It can be any of these values:1 = Created/Active2 = Creating3 = Suspended4 = Deleting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The time the namespace was created."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The time the namespace was updated."]
    #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
    #[doc = "Endpoint you can use to perform NotificationHub operations."]
    #[serde(rename = "serviceBusEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_endpoint: Option<String>,
    #[doc = "The Id of the Azure subscription associated with the namespace."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "ScaleUnit where the namespace gets created"]
    #[serde(rename = "scaleUnit", default, skip_serializing_if = "Option::is_none")]
    pub scale_unit: Option<String>,
    #[doc = "Whether or not the namespace is currently enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Whether or not the namespace is set as Critical."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[doc = "Data center for the namespace"]
    #[serde(rename = "dataCenter", default, skip_serializing_if = "Option::is_none")]
    pub data_center: Option<String>,
    #[doc = "The namespace type."]
    #[serde(rename = "namespaceType", default, skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<namespace_properties::NamespaceType>,
}
impl NamespaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod namespace_properties {
    use super::*;
    #[doc = "The namespace type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NamespaceType {
        Messaging,
        NotificationHub,
    }
}
#[doc = "Description of a Namespace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Namespace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
}
impl NamespaceResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the CreateOrUpdate NotificationHub operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubCreateOrUpdateParameters {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "NotificationHub properties."]
    pub properties: NotificationHubProperties,
}
impl NotificationHubCreateOrUpdateParameters {
    pub fn new(properties: NotificationHubProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The response of the List NotificationHub operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationHubListResult {
    #[doc = "Result of the List NotificationHub operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NotificationHubResource>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of NotificationHub"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NotificationHubListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NotificationHubListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the patch NotificationHub operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationHubPatchParameters {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "NotificationHub properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotificationHubProperties>,
}
impl NotificationHubPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NotificationHub properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationHubProperties {
    #[doc = "The NotificationHub name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The RegistrationTtl of the created NotificationHub"]
    #[serde(rename = "registrationTtl", default, skip_serializing_if = "Option::is_none")]
    pub registration_ttl: Option<String>,
    #[doc = "The AuthorizationRules of the created NotificationHub"]
    #[serde(rename = "authorizationRules", default, skip_serializing_if = "Vec::is_empty")]
    pub authorization_rules: Vec<SharedAccessAuthorizationRuleProperties>,
    #[doc = "Description of a NotificationHub ApnsCredential."]
    #[serde(rename = "apnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub apns_credential: Option<ApnsCredential>,
    #[doc = "Description of a NotificationHub WnsCredential."]
    #[serde(rename = "wnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub wns_credential: Option<WnsCredential>,
    #[doc = "Description of a NotificationHub GcmCredential."]
    #[serde(rename = "gcmCredential", default, skip_serializing_if = "Option::is_none")]
    pub gcm_credential: Option<GcmCredential>,
    #[doc = "Description of a NotificationHub MpnsCredential."]
    #[serde(rename = "mpnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub mpns_credential: Option<MpnsCredential>,
    #[doc = "Description of a NotificationHub AdmCredential."]
    #[serde(rename = "admCredential", default, skip_serializing_if = "Option::is_none")]
    pub adm_credential: Option<AdmCredential>,
    #[doc = "Description of a NotificationHub BaiduCredential."]
    #[serde(rename = "baiduCredential", default, skip_serializing_if = "Option::is_none")]
    pub baidu_credential: Option<BaiduCredential>,
}
impl NotificationHubProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationHubResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "NotificationHub properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotificationHubProperties>,
}
impl NotificationHubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A NotificationHubs REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
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
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.NotificationHubs"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Invoice, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list NotificationHubs operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of NotificationHubs operations supported by the Microsoft.NotificationHubs resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
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
#[doc = "Description of a NotificationHub PNS Credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PnsCredentialsProperties {
    #[doc = "Description of a NotificationHub ApnsCredential."]
    #[serde(rename = "apnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub apns_credential: Option<ApnsCredential>,
    #[doc = "Description of a NotificationHub WnsCredential."]
    #[serde(rename = "wnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub wns_credential: Option<WnsCredential>,
    #[doc = "Description of a NotificationHub GcmCredential."]
    #[serde(rename = "gcmCredential", default, skip_serializing_if = "Option::is_none")]
    pub gcm_credential: Option<GcmCredential>,
    #[doc = "Description of a NotificationHub MpnsCredential."]
    #[serde(rename = "mpnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub mpns_credential: Option<MpnsCredential>,
    #[doc = "Description of a NotificationHub AdmCredential."]
    #[serde(rename = "admCredential", default, skip_serializing_if = "Option::is_none")]
    pub adm_credential: Option<AdmCredential>,
    #[doc = "Description of a NotificationHub BaiduCredential."]
    #[serde(rename = "baiduCredential", default, skip_serializing_if = "Option::is_none")]
    pub baidu_credential: Option<BaiduCredential>,
}
impl PnsCredentialsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub PNS Credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PnsCredentialsResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Description of a NotificationHub PNS Credentials."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PnsCredentialsProperties>,
}
impl PnsCredentialsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Namespace/NotificationHub Regenerate Keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicykeyResource {
    #[doc = "Name of the key that has to be regenerated for the Namespace/Notification Hub Authorization Rule. The value can be Primary Key/Secondary Key."]
    #[serde(rename = "policyKey", default, skip_serializing_if = "Option::is_none")]
    pub policy_key: Option<String>,
}
impl PolicykeyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The Sku description for a namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Namespace/NotificationHub Connection String"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceListKeys {
    #[doc = "PrimaryConnectionString of the AuthorizationRule."]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "SecondaryConnectionString of the created AuthorizationRule"]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[doc = "PrimaryKey of the created AuthorizationRule."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "SecondaryKey of the created AuthorizationRule"]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "KeyName of the created AuthorizationRule"]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
impl ResourceListKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the CreateOrUpdate Namespace AuthorizationRules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleCreateOrUpdateParameters {
    #[doc = "SharedAccessAuthorizationRule properties."]
    pub properties: SharedAccessAuthorizationRuleProperties,
}
impl SharedAccessAuthorizationRuleCreateOrUpdateParameters {
    pub fn new(properties: SharedAccessAuthorizationRuleProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The response of the List Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedAccessAuthorizationRuleListResult {
    #[doc = "Result of the List AuthorizationRules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedAccessAuthorizationRuleResource>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of AuthorizationRules"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SharedAccessAuthorizationRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SharedAccessAuthorizationRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SharedAccessAuthorizationRule properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedAccessAuthorizationRuleProperties {
    #[doc = "The rights associated with the rule."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rights: Vec<String>,
    #[doc = "A base64-encoded 256-bit primary key for signing and validating the SAS token."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "A base64-encoded 256-bit primary key for signing and validating the SAS token."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "A string that describes the authorization rule."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "A string that describes the claim type"]
    #[serde(rename = "claimType", default, skip_serializing_if = "Option::is_none")]
    pub claim_type: Option<String>,
    #[doc = "A string that describes the claim value"]
    #[serde(rename = "claimValue", default, skip_serializing_if = "Option::is_none")]
    pub claim_value: Option<String>,
    #[doc = "The last modified time for this rule"]
    #[serde(rename = "modifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    #[doc = "The created time for this rule"]
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[doc = "The revision number for the rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl SharedAccessAuthorizationRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a Namespace AuthorizationRules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedAccessAuthorizationRuleResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "SharedAccessAuthorizationRule properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedAccessAuthorizationRuleProperties>,
}
impl SharedAccessAuthorizationRuleResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Sku description for a namespace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Name of the notification hub sku"]
    pub name: sku::Name,
    #[doc = "The tier of particular sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The Sku size"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The Sku Family"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The capacity of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
pub mod sku {
    use super::*;
    #[doc = "Name of the notification hub sku"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Free,
        Basic,
        Standard,
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
                Self::Free => serializer.serialize_unit_variant("Name", 0u32, "Free"),
                Self::Basic => serializer.serialize_unit_variant("Name", 1u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Name", 2u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub WnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WnsCredential {
    #[doc = "Description of a NotificationHub WnsCredential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WnsCredentialProperties>,
}
impl WnsCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub WnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WnsCredentialProperties {
    #[doc = "The package ID for this credential."]
    #[serde(rename = "packageSid", default, skip_serializing_if = "Option::is_none")]
    pub package_sid: Option<String>,
    #[doc = "The secret key."]
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[doc = "The Windows Live endpoint."]
    #[serde(rename = "windowsLiveEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub windows_live_endpoint: Option<String>,
}
impl WnsCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
