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
    #[doc = "Gets or sets the client identifier."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Gets or sets the credential secret access key."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "Gets or sets the URL of the authorization token."]
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
    #[doc = "Description of a NotificationHub ApnsCredential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApnsCredentialProperties>,
}
impl ApnsCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a NotificationHub ApnsCredential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApnsCredentialProperties {
    #[doc = "Gets or sets the APNS certificate."]
    #[serde(rename = "apnsCertificate", default, skip_serializing_if = "Option::is_none")]
    pub apns_certificate: Option<String>,
    #[doc = "Gets or sets the certificate key."]
    #[serde(rename = "certificateKey", default, skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[doc = "Gets or sets the endpoint of this credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Gets or sets the Apns certificate Thumbprint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
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
    #[doc = "Get or Set Baidu Api Key."]
    #[serde(rename = "baiduApiKey", default, skip_serializing_if = "Option::is_none")]
    pub baidu_api_key: Option<String>,
    #[doc = "Get or Set Baidu Endpoint."]
    #[serde(rename = "baiduEndPoint", default, skip_serializing_if = "Option::is_none")]
    pub baidu_end_point: Option<String>,
    #[doc = "Get or Set Baidu Secret Key"]
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
    #[doc = "Gets or sets name"]
    pub name: String,
    #[doc = "Gets or sets location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets true if the name is available and can be used to create new Namespace/NotificationHub. Otherwise false."]
    #[serde(rename = "isAvailiable", default, skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
}
impl CheckAvailabilityParameters {
    pub fn new(name: String) -> Self {
        Self {
            name,
            location: None,
            tags: None,
            is_availiable: None,
        }
    }
}
#[doc = "Description of a CheckAvailability resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckAvailabilityResource {
    #[doc = "Gets or sets the id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets datacenter location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets true if the name is available and can be used to create new Namespace/NotificationHub. Otherwise false."]
    #[serde(rename = "isAvailiable", default, skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
}
impl CheckAvailabilityResource {
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
    #[doc = "Gets or sets the GCM endpoint."]
    #[serde(rename = "gcmEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub gcm_endpoint: Option<String>,
    #[doc = "Gets or sets the Google API key."]
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
    #[doc = "Gets or sets the MPNS certificate."]
    #[serde(rename = "mpnsCertificate", default, skip_serializing_if = "Option::is_none")]
    pub mpns_certificate: Option<String>,
    #[doc = "Gets or sets the certificate key for this credential."]
    #[serde(rename = "certificateKey", default, skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[doc = "Gets or sets the Mpns certificate Thumbprint"]
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
    #[doc = "Gets or sets Namespace data center location."]
    pub location: String,
    #[doc = "Gets or sets Namespace tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Namespace properties."]
    pub properties: NamespaceProperties,
}
impl NamespaceCreateOrUpdateParameters {
    pub fn new(location: String, properties: NamespaceProperties) -> Self {
        Self {
            location,
            tags: None,
            properties,
        }
    }
}
#[doc = "The response of the List Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceListResult {
    #[doc = "Gets or sets result of the List Namespace operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NamespaceResource>,
    #[doc = "Gets or sets link to the next set of results. Not empty if Value contains incomplete list of Namespaces"]
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
#[doc = "Namespace properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceProperties {
    #[doc = "The name of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets provisioning state of the Namespace."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Specifies the targeted region in which the namespace should be created. It can be any of the following values: Australia East, Australia Southeast, Central US, East US, East US 2, West US, North Central US, South Central US, East Asia, Southeast Asia, Brazil South, Japan East, Japan West, North Europe, West Europe"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Status of the namespace. It can be any of these values:1 = Created/Active2 = Creating3 = Suspended4 = Deleting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The time the namespace was created."]
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
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
    #[doc = "Gets or sets the namespace type."]
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
    #[doc = "Gets or sets the namespace type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NamespaceType {
        Messaging,
        NotificationHub,
    }
}
#[doc = "Description of a Namespace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceResource {
    #[doc = "Gets or sets the id of the created Namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets datacenter location of the Namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets name of the Namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets resource type of the Namespace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets tags of the Namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
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
    #[doc = "Gets or sets NotificationHub data center location."]
    pub location: String,
    #[doc = "Gets or sets NotificationHub tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "NotificationHub properties."]
    pub properties: NotificationHubProperties,
}
impl NotificationHubCreateOrUpdateParameters {
    pub fn new(location: String, properties: NotificationHubProperties) -> Self {
        Self {
            location,
            tags: None,
            properties,
        }
    }
}
#[doc = "The response of the List NotificationHub operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationHubListResult {
    #[doc = "Gets or sets result of the List NotificationHub operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NotificationHubResource>,
    #[doc = "Gets or sets link to the next set of results. Not empty if Value contains incomplete list of NotificationHub"]
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
    #[doc = "Gets or sets the id of the created NotificationHub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets datacenter location of the NotificationHub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets name of the NotificationHub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets resource type of the NotificationHub."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets tags of the NotificationHub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "NotificationHub properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotificationHubProperties>,
}
impl NotificationHubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "Namespace/NotificationHub Connection String"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceListKeys {
    #[doc = "Gets or sets the primaryConnectionString of the created Namespace AuthorizationRule."]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "Gets or sets the secondaryConnectionString of the created Namespace AuthorizationRule"]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
}
impl ResourceListKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the CreateOrUpdate Namespace AuthorizationRules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleCreateOrUpdateParameters {
    #[doc = "Gets or sets Namespace data center location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets Name of the Namespace AuthorizationRule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "SharedAccessAuthorizationRule properties."]
    pub properties: SharedAccessAuthorizationRuleProperties,
}
impl SharedAccessAuthorizationRuleCreateOrUpdateParameters {
    pub fn new(properties: SharedAccessAuthorizationRuleProperties) -> Self {
        Self {
            location: None,
            name: None,
            properties,
        }
    }
}
#[doc = "The response of the List Namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedAccessAuthorizationRuleListResult {
    #[doc = "Gets or sets result of the List AuthorizationRules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedAccessAuthorizationRuleResource>,
    #[doc = "Gets or sets link to the next set of results. Not empty if Value contains incomplete list of AuthorizationRules"]
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
    #[doc = "The primary key that was used."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary key that was used."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "The name of the key that was used."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "The type of the claim."]
    #[serde(rename = "claimType", default, skip_serializing_if = "Option::is_none")]
    pub claim_type: Option<String>,
    #[doc = "The value of the claim."]
    #[serde(rename = "claimValue", default, skip_serializing_if = "Option::is_none")]
    pub claim_value: Option<String>,
    #[doc = "The rights associated with the rule."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rights: Vec<String>,
    #[doc = "The time at which the authorization rule was created."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The most recent time the rule was updated."]
    #[serde(rename = "modifiedTime", with = "azure_core::date::rfc3339::option")]
    pub modified_time: Option<time::OffsetDateTime>,
    #[doc = "The revision number for the rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}
impl SharedAccessAuthorizationRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a Namespace AuthorizationRules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedAccessAuthorizationRuleResource {
    #[doc = "Gets or sets the id of the created Namespace AuthorizationRules."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets datacenter location of the Namespace AuthorizationRules."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets name of the Namespace AuthorizationRules."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets resource type of the Namespace AuthorizationRules."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets tags of the Namespace AuthorizationRules."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "SharedAccessAuthorizationRule properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedAccessAuthorizationRuleProperties>,
}
impl SharedAccessAuthorizationRuleResource {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Gets or sets the package ID for this credential."]
    #[serde(rename = "packageSid", default, skip_serializing_if = "Option::is_none")]
    pub package_sid: Option<String>,
    #[doc = "Gets or sets the secret key."]
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[doc = "Gets or sets the Windows Live endpoint."]
    #[serde(rename = "windowsLiveEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub windows_live_endpoint: Option<String>,
}
impl WnsCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
