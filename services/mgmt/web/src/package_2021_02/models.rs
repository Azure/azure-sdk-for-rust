#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Class representing Abnormal Time Period identified in diagnosis"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AbnormalTimePeriod {
    #[doc = "Start time of the downtime"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the downtime"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "List of Possible Cause of downtime"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<DetectorAbnormalTimePeriod>,
    #[doc = "List of proposed solutions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub solutions: Vec<Solution>,
}
impl AbnormalTimePeriod {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Address information for domain registration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    #[doc = "First line of an Address."]
    pub address1: String,
    #[doc = "The second line of the Address. Optional."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[doc = "The city for the address."]
    pub city: String,
    #[doc = "The country for the address."]
    pub country: String,
    #[doc = "The postal code for the address."]
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[doc = "The state or province for the address."]
    pub state: String,
}
impl Address {
    pub fn new(address1: String, city: String, country: String, postal_code: String, state: String) -> Self {
        Self {
            address1,
            address2: None,
            city,
            country,
            postal_code,
            state,
        }
    }
}
#[doc = "Describes main public IP address and any extra virtual IPs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressResponse {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "AddressResponse resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<address_response::Properties>,
}
impl AddressResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod address_response {
    use super::*;
    #[doc = "AddressResponse resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Main public virtual IP."]
        #[serde(rename = "serviceIpAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_ip_address: Option<String>,
        #[doc = "Virtual Network internal IP address of the App Service Environment if it is in internal load-balancing mode."]
        #[serde(rename = "internalIpAddress", default, skip_serializing_if = "Option::is_none")]
        pub internal_ip_address: Option<String>,
        #[doc = "IP addresses appearing on outbound connections."]
        #[serde(rename = "outboundIpAddresses", default, skip_serializing_if = "Vec::is_empty")]
        pub outbound_ip_addresses: Vec<String>,
        #[doc = "Additional virtual IPs."]
        #[serde(rename = "vipMappings", default, skip_serializing_if = "Vec::is_empty")]
        pub vip_mappings: Vec<VirtualIpMapping>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The configuration settings of the Allowed Audiences validation flow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllowedAudiencesValidation {
    #[doc = "The configuration settings of the allowed list of audiences from which to validate the JWT token."]
    #[serde(rename = "allowedAudiences", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_audiences: Vec<String>,
}
impl AllowedAudiencesValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the Azure Active Directory allowed principals."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllowedPrincipals {
    #[doc = "The list of the allowed groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    #[doc = "The list of the allowed identities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identities: Vec<String>,
}
impl AllowedPrincipals {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class Representing Detector Evidence used for analysis"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisData {
    #[doc = "Name of the Detector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Class representing detector definition"]
    #[serde(rename = "detectorDefinition", default, skip_serializing_if = "Option::is_none")]
    pub detector_definition: Option<DetectorDefinition>,
    #[doc = "Source Metrics"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<DiagnosticMetricSet>,
    #[doc = "Additional Source Data"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Vec<NameValuePair>>,
    #[serde(rename = "detectorMetaData", default, skip_serializing_if = "Option::is_none")]
    pub detector_meta_data: Option<ResponseMetaData>,
}
impl AnalysisData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of Analysis"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisDefinition {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "AnalysisDefinition resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<analysis_definition::Properties>,
}
impl AnalysisDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod analysis_definition {
    use super::*;
    #[doc = "AnalysisDefinition resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Description of the Analysis"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Information about the formal API definition for the app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiDefinitionInfo {
    #[doc = "The URL of the API definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ApiDefinitionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of site key vault references."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiKvReference {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "ApiKVReference resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<api_kv_reference::Properties>,
}
impl ApiKvReference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_kv_reference {
    use super::*;
    #[doc = "ApiKVReference resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reference: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[serde(rename = "vaultName", default, skip_serializing_if = "Option::is_none")]
        pub vault_name: Option<String>,
        #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
        pub secret_name: Option<String>,
        #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
        pub secret_version: Option<String>,
        #[doc = "Managed service identity."]
        #[serde(rename = "identityType", default, skip_serializing_if = "Option::is_none")]
        pub identity_type: Option<ManagedServiceIdentity>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub details: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source: Option<properties::Source>,
        #[serde(rename = "activeVersion", default, skip_serializing_if = "Option::is_none")]
        pub active_version: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Initialized,
            Resolved,
            InvalidSyntax,
            #[serde(rename = "MSINotEnabled")]
            MsiNotEnabled,
            VaultNotFound,
            SecretNotFound,
            SecretVersionNotFound,
            AccessToKeyVaultDenied,
            OtherReasons,
            FetchTimedOut,
            UnauthorizedClient,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Source {
            KeyVault,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKvReferenceCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ApiKvReference>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiKvReferenceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApiKvReferenceCollection {
    pub fn new(value: Vec<ApiKvReference>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Azure API management (APIM) configuration linked to the app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementConfig {
    #[doc = "APIM-Api Identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ApiManagementConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "App Insights Web App stack settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppInsightsWebAppStackSettings {
    #[doc = "<code>true</code> if remote Application Insights is supported for the stack; otherwise, <code>false</code>."]
    #[serde(rename = "isSupported", default, skip_serializing_if = "Option::is_none")]
    pub is_supported: Option<bool>,
    #[doc = "<code>true</code> if Application Insights is disabled by default for the stack; otherwise, <code>false</code>."]
    #[serde(rename = "isDefaultOff", default, skip_serializing_if = "Option::is_none")]
    pub is_default_off: Option<bool>,
}
impl AppInsightsWebAppStackSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppLogsConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "logAnalyticsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_configuration: Option<LogAnalyticsConfiguration>,
}
impl AppLogsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the app registration for providers that have app ids and app secrets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppRegistration {
    #[doc = "The App ID of the app used for login."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "The app setting name that contains the app secret."]
    #[serde(rename = "appSecretSettingName", default, skip_serializing_if = "Option::is_none")]
    pub app_secret_setting_name: Option<String>,
}
impl AppRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Key Vault container for a certificate that is purchased through Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServiceCertificate {
    #[doc = "Key Vault resource Id."]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
    #[doc = "Key Vault secret name."]
    #[serde(rename = "keyVaultSecretName", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_secret_name: Option<String>,
    #[doc = "Status of the Key Vault secret."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<app_service_certificate::ProvisioningState>,
}
impl AppServiceCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod app_service_certificate {
    use super::*;
    #[doc = "Status of the Key Vault secret."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Initialized,
        WaitingOnCertificateOrder,
        Succeeded,
        CertificateOrderFailed,
        OperationNotPermittedOnKeyVault,
        AzureServiceUnauthorizedToAccessKeyVault,
        KeyVaultDoesNotExist,
        KeyVaultSecretDoesNotExist,
        UnknownError,
        ExternalPrivateKey,
        Unknown,
    }
}
#[doc = "Collection of certificate order certificates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceCertificateCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<AppServiceCertificateResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AppServiceCertificateCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AppServiceCertificateCollection {
    pub fn new(value: Vec<AppServiceCertificateResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "SSL certificate purchase order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceCertificateOrder {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "AppServiceCertificateOrder resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<app_service_certificate_order::Properties>,
}
impl AppServiceCertificateOrder {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
pub mod app_service_certificate_order {
    use super::*;
    #[doc = "AppServiceCertificateOrder resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "State of the Key Vault secret."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub certificates: Option<serde_json::Value>,
        #[doc = "Certificate distinguished name."]
        #[serde(rename = "distinguishedName", default, skip_serializing_if = "Option::is_none")]
        pub distinguished_name: Option<String>,
        #[doc = "Domain verification token."]
        #[serde(rename = "domainVerificationToken", default, skip_serializing_if = "Option::is_none")]
        pub domain_verification_token: Option<String>,
        #[doc = "Duration in years (must be 1)."]
        #[serde(rename = "validityInYears", default, skip_serializing_if = "Option::is_none")]
        pub validity_in_years: Option<i32>,
        #[doc = "Certificate key size."]
        #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[doc = "Certificate product type."]
        #[serde(rename = "productType")]
        pub product_type: properties::ProductType,
        #[doc = "<code>true</code> if the certificate should be automatically renewed when it expires; otherwise, <code>false</code>."]
        #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
        pub auto_renew: Option<bool>,
        #[doc = "Status of certificate order."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Current order status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[doc = "SSL certificate details."]
        #[serde(rename = "signedCertificate", default, skip_serializing_if = "Option::is_none")]
        pub signed_certificate: Option<CertificateDetails>,
        #[doc = "Last CSR that was created for this order."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csr: Option<String>,
        #[doc = "SSL certificate details."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub intermediate: Option<CertificateDetails>,
        #[doc = "SSL certificate details."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub root: Option<CertificateDetails>,
        #[doc = "Current serial number of the certificate."]
        #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
        pub serial_number: Option<String>,
        #[doc = "Certificate last issuance time."]
        #[serde(rename = "lastCertificateIssuanceTime", default, with = "azure_core::date::rfc3339::option")]
        pub last_certificate_issuance_time: Option<time::OffsetDateTime>,
        #[doc = "Certificate expiration time."]
        #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
        pub expiration_time: Option<time::OffsetDateTime>,
        #[doc = "<code>true</code> if private key is external; otherwise, <code>false</code>."]
        #[serde(rename = "isPrivateKeyExternal", default, skip_serializing_if = "Option::is_none")]
        pub is_private_key_external: Option<bool>,
        #[doc = "Reasons why App Service Certificate is not renewable at the current moment."]
        #[serde(
            rename = "appServiceCertificateNotRenewableReasons",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub app_service_certificate_not_renewable_reasons: Vec<String>,
        #[doc = "Time stamp when the certificate would be auto renewed next"]
        #[serde(rename = "nextAutoRenewalTimeStamp", default, with = "azure_core::date::rfc3339::option")]
        pub next_auto_renewal_time_stamp: Option<time::OffsetDateTime>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub contact: Option<CertificateOrderContact>,
    }
    impl Properties {
        pub fn new(product_type: properties::ProductType) -> Self {
            Self {
                certificates: None,
                distinguished_name: None,
                domain_verification_token: None,
                validity_in_years: None,
                key_size: None,
                product_type,
                auto_renew: None,
                provisioning_state: None,
                status: None,
                signed_certificate: None,
                csr: None,
                intermediate: None,
                root: None,
                serial_number: None,
                last_certificate_issuance_time: None,
                expiration_time: None,
                is_private_key_external: None,
                app_service_certificate_not_renewable_reasons: Vec::new(),
                next_auto_renewal_time_stamp: None,
                contact: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Certificate product type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProductType {
            StandardDomainValidatedSsl,
            StandardDomainValidatedWildCardSsl,
        }
        #[doc = "Status of certificate order."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            InProgress,
            Deleting,
        }
        #[doc = "Current order status."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Pendingissuance,
            Issued,
            Revoked,
            Canceled,
            Denied,
            Pendingrevocation,
            PendingRekey,
            Unused,
            Expired,
            NotSubmitted,
        }
    }
}
#[doc = "Collection of certificate orders."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceCertificateOrderCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<AppServiceCertificateOrder>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AppServiceCertificateOrderCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AppServiceCertificateOrderCollection {
    pub fn new(value: Vec<AppServiceCertificateOrder>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "ARM resource for a certificate order that is purchased through Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServiceCertificateOrderPatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "AppServiceCertificateOrderPatchResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<app_service_certificate_order_patch_resource::Properties>,
}
impl AppServiceCertificateOrderPatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod app_service_certificate_order_patch_resource {
    use super::*;
    #[doc = "AppServiceCertificateOrderPatchResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "State of the Key Vault secret."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub certificates: Option<serde_json::Value>,
        #[doc = "Certificate distinguished name."]
        #[serde(rename = "distinguishedName", default, skip_serializing_if = "Option::is_none")]
        pub distinguished_name: Option<String>,
        #[doc = "Domain verification token."]
        #[serde(rename = "domainVerificationToken", default, skip_serializing_if = "Option::is_none")]
        pub domain_verification_token: Option<String>,
        #[doc = "Duration in years (must be 1)."]
        #[serde(rename = "validityInYears", default, skip_serializing_if = "Option::is_none")]
        pub validity_in_years: Option<i32>,
        #[doc = "Certificate key size."]
        #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[doc = "Certificate product type."]
        #[serde(rename = "productType")]
        pub product_type: properties::ProductType,
        #[doc = "<code>true</code> if the certificate should be automatically renewed when it expires; otherwise, <code>false</code>."]
        #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
        pub auto_renew: Option<bool>,
        #[doc = "Status of certificate order."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Current order status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[doc = "SSL certificate details."]
        #[serde(rename = "signedCertificate", default, skip_serializing_if = "Option::is_none")]
        pub signed_certificate: Option<CertificateDetails>,
        #[doc = "Last CSR that was created for this order."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csr: Option<String>,
        #[doc = "SSL certificate details."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub intermediate: Option<CertificateDetails>,
        #[doc = "SSL certificate details."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub root: Option<CertificateDetails>,
        #[doc = "Current serial number of the certificate."]
        #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
        pub serial_number: Option<String>,
        #[doc = "Certificate last issuance time."]
        #[serde(rename = "lastCertificateIssuanceTime", default, with = "azure_core::date::rfc3339::option")]
        pub last_certificate_issuance_time: Option<time::OffsetDateTime>,
        #[doc = "Certificate expiration time."]
        #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
        pub expiration_time: Option<time::OffsetDateTime>,
        #[doc = "<code>true</code> if private key is external; otherwise, <code>false</code>."]
        #[serde(rename = "isPrivateKeyExternal", default, skip_serializing_if = "Option::is_none")]
        pub is_private_key_external: Option<bool>,
        #[doc = "Reasons why App Service Certificate is not renewable at the current moment."]
        #[serde(
            rename = "appServiceCertificateNotRenewableReasons",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub app_service_certificate_not_renewable_reasons: Vec<String>,
        #[doc = "Time stamp when the certificate would be auto renewed next"]
        #[serde(rename = "nextAutoRenewalTimeStamp", default, with = "azure_core::date::rfc3339::option")]
        pub next_auto_renewal_time_stamp: Option<time::OffsetDateTime>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub contact: Option<CertificateOrderContact>,
    }
    impl Properties {
        pub fn new(product_type: properties::ProductType) -> Self {
            Self {
                certificates: None,
                distinguished_name: None,
                domain_verification_token: None,
                validity_in_years: None,
                key_size: None,
                product_type,
                auto_renew: None,
                provisioning_state: None,
                status: None,
                signed_certificate: None,
                csr: None,
                intermediate: None,
                root: None,
                serial_number: None,
                last_certificate_issuance_time: None,
                expiration_time: None,
                is_private_key_external: None,
                app_service_certificate_not_renewable_reasons: Vec::new(),
                next_auto_renewal_time_stamp: None,
                contact: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Certificate product type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProductType {
            StandardDomainValidatedSsl,
            StandardDomainValidatedWildCardSsl,
        }
        #[doc = "Status of certificate order."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            InProgress,
            Deleting,
        }
        #[doc = "Current order status."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Pendingissuance,
            Issued,
            Revoked,
            Canceled,
            Denied,
            Pendingrevocation,
            PendingRekey,
            Unused,
            Expired,
            NotSubmitted,
        }
    }
}
#[doc = "Key Vault container ARM resource for a certificate that is purchased through Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServiceCertificatePatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Key Vault container for a certificate that is purchased through Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppServiceCertificate>,
}
impl AppServiceCertificatePatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Key Vault container ARM resource for a certificate that is purchased through Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceCertificateResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Key Vault container for a certificate that is purchased through Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppServiceCertificate>,
}
impl AppServiceCertificateResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "Description of an App Service Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceEnvironment {
    #[doc = "Provisioning state of the App Service Environment."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<app_service_environment::ProvisioningState>,
    #[doc = "Current status of the App Service Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<app_service_environment::Status>,
    #[doc = "Specification for using a Virtual Network."]
    #[serde(rename = "virtualNetwork")]
    pub virtual_network: VirtualNetworkProfile,
    #[doc = "Specifies which endpoints to serve internally in the Virtual Network for the App Service Environment."]
    #[serde(rename = "internalLoadBalancingMode", default, skip_serializing_if = "Option::is_none")]
    pub internal_load_balancing_mode: Option<app_service_environment::InternalLoadBalancingMode>,
    #[doc = "Front-end VM size, e.g. \"Medium\", \"Large\"."]
    #[serde(rename = "multiSize", default, skip_serializing_if = "Option::is_none")]
    pub multi_size: Option<String>,
    #[doc = "Number of front-end instances."]
    #[serde(rename = "multiRoleCount", default, skip_serializing_if = "Option::is_none")]
    pub multi_role_count: Option<i32>,
    #[doc = "Number of IP SSL addresses reserved for the App Service Environment."]
    #[serde(rename = "ipsslAddressCount", default, skip_serializing_if = "Option::is_none")]
    pub ipssl_address_count: Option<i32>,
    #[doc = "DNS suffix of the App Service Environment."]
    #[serde(rename = "dnsSuffix", default, skip_serializing_if = "Option::is_none")]
    pub dns_suffix: Option<String>,
    #[doc = "Maximum number of VMs in the App Service Environment."]
    #[serde(rename = "maximumNumberOfMachines", default, skip_serializing_if = "Option::is_none")]
    pub maximum_number_of_machines: Option<i32>,
    #[doc = "Scale factor for front-ends."]
    #[serde(rename = "frontEndScaleFactor", default, skip_serializing_if = "Option::is_none")]
    pub front_end_scale_factor: Option<i32>,
    #[doc = "<code>true</code> if the App Service Environment is suspended; otherwise, <code>false</code>. The environment can be suspended, e.g. when the management endpoint is no longer available\n (most likely because NSG blocked the incoming traffic)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[doc = "Custom settings for changing the behavior of the App Service Environment."]
    #[serde(rename = "clusterSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_settings: Vec<NameValuePair>,
    #[doc = "User added ip ranges to whitelist on ASE db"]
    #[serde(rename = "userWhitelistedIpRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub user_whitelisted_ip_ranges: Vec<String>,
    #[doc = "Flag that displays whether an ASE has linux workers or not"]
    #[serde(rename = "hasLinuxWorkers", default, skip_serializing_if = "Option::is_none")]
    pub has_linux_workers: Option<bool>,
    #[doc = "Dedicated Host Count"]
    #[serde(rename = "dedicatedHostCount", default, skip_serializing_if = "Option::is_none")]
    pub dedicated_host_count: Option<i32>,
    #[doc = "Whether or not this App Service Environment is zone-redundant."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
}
impl AppServiceEnvironment {
    pub fn new(virtual_network: VirtualNetworkProfile) -> Self {
        Self {
            provisioning_state: None,
            status: None,
            virtual_network,
            internal_load_balancing_mode: None,
            multi_size: None,
            multi_role_count: None,
            ipssl_address_count: None,
            dns_suffix: None,
            maximum_number_of_machines: None,
            front_end_scale_factor: None,
            suspended: None,
            cluster_settings: Vec::new(),
            user_whitelisted_ip_ranges: Vec::new(),
            has_linux_workers: None,
            dedicated_host_count: None,
            zone_redundant: None,
        }
    }
}
pub mod app_service_environment {
    use super::*;
    #[doc = "Provisioning state of the App Service Environment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        InProgress,
        Deleting,
    }
    #[doc = "Current status of the App Service Environment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Preparing,
        Ready,
        Scaling,
        Deleting,
    }
    #[doc = "Specifies which endpoints to serve internally in the Virtual Network for the App Service Environment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InternalLoadBalancingMode")]
    pub enum InternalLoadBalancingMode {
        None,
        Web,
        Publishing,
        #[serde(rename = "Web, Publishing")]
        WebPublishing,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InternalLoadBalancingMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InternalLoadBalancingMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InternalLoadBalancingMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("InternalLoadBalancingMode", 0u32, "None"),
                Self::Web => serializer.serialize_unit_variant("InternalLoadBalancingMode", 1u32, "Web"),
                Self::Publishing => serializer.serialize_unit_variant("InternalLoadBalancingMode", 2u32, "Publishing"),
                Self::WebPublishing => serializer.serialize_unit_variant("InternalLoadBalancingMode", 3u32, "Web, Publishing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of App Service Environments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceEnvironmentCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<AppServiceEnvironmentResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AppServiceEnvironmentCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AppServiceEnvironmentCollection {
    pub fn new(value: Vec<AppServiceEnvironmentResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "ARM resource for a app service environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServiceEnvironmentPatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Description of an App Service Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppServiceEnvironment>,
}
impl AppServiceEnvironmentPatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "App Service Environment ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceEnvironmentResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Description of an App Service Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppServiceEnvironment>,
}
impl AppServiceEnvironmentResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "App Service plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServicePlan {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "AppServicePlan resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<app_service_plan::Properties>,
    #[doc = "Description of a SKU for a scalable resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuDescription>,
    #[doc = "Extended Location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl AppServicePlan {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            sku: None,
            extended_location: None,
        }
    }
}
pub mod app_service_plan {
    use super::*;
    #[doc = "AppServicePlan resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Target worker tier assigned to the App Service plan."]
        #[serde(rename = "workerTierName", default, skip_serializing_if = "Option::is_none")]
        pub worker_tier_name: Option<String>,
        #[doc = "App Service plan status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[doc = "App Service plan subscription."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subscription: Option<String>,
        #[doc = "Specification for an App Service Environment to use for this resource."]
        #[serde(rename = "hostingEnvironmentProfile", default, skip_serializing_if = "Option::is_none")]
        pub hosting_environment_profile: Option<HostingEnvironmentProfile>,
        #[doc = "Maximum number of instances that can be assigned to this App Service plan."]
        #[serde(rename = "maximumNumberOfWorkers", default, skip_serializing_if = "Option::is_none")]
        pub maximum_number_of_workers: Option<i32>,
        #[doc = "Geographical location for the App Service plan."]
        #[serde(rename = "geoRegion", default, skip_serializing_if = "Option::is_none")]
        pub geo_region: Option<String>,
        #[doc = "If <code>true</code>, apps assigned to this App Service plan can be scaled independently.\nIf <code>false</code>, apps assigned to this App Service plan will scale to all instances of the plan."]
        #[serde(rename = "perSiteScaling", default, skip_serializing_if = "Option::is_none")]
        pub per_site_scaling: Option<bool>,
        #[doc = "ServerFarm supports ElasticScale. Apps in this plan will scale as if the ServerFarm was ElasticPremium sku"]
        #[serde(rename = "elasticScaleEnabled", default, skip_serializing_if = "Option::is_none")]
        pub elastic_scale_enabled: Option<bool>,
        #[doc = "Maximum number of total workers allowed for this ElasticScaleEnabled App Service Plan"]
        #[serde(rename = "maximumElasticWorkerCount", default, skip_serializing_if = "Option::is_none")]
        pub maximum_elastic_worker_count: Option<i32>,
        #[doc = "Number of apps assigned to this App Service plan."]
        #[serde(rename = "numberOfSites", default, skip_serializing_if = "Option::is_none")]
        pub number_of_sites: Option<i32>,
        #[doc = "If <code>true</code>, this App Service Plan owns spot instances."]
        #[serde(rename = "isSpot", default, skip_serializing_if = "Option::is_none")]
        pub is_spot: Option<bool>,
        #[doc = "The time when the server farm expires. Valid only if it is a spot server farm."]
        #[serde(rename = "spotExpirationTime", default, with = "azure_core::date::rfc3339::option")]
        pub spot_expiration_time: Option<time::OffsetDateTime>,
        #[doc = "The time when the server farm free offer expires."]
        #[serde(rename = "freeOfferExpirationTime", default, with = "azure_core::date::rfc3339::option")]
        pub free_offer_expiration_time: Option<time::OffsetDateTime>,
        #[doc = "Resource group of the App Service plan."]
        #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
        pub resource_group: Option<String>,
        #[doc = "If Linux app service plan <code>true</code>, <code>false</code> otherwise."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reserved: Option<bool>,
        #[doc = "Obsolete: If Hyper-V container app service plan <code>true</code>, <code>false</code> otherwise."]
        #[serde(rename = "isXenon", default, skip_serializing_if = "Option::is_none")]
        pub is_xenon: Option<bool>,
        #[doc = "If Hyper-V container app service plan <code>true</code>, <code>false</code> otherwise."]
        #[serde(rename = "hyperV", default, skip_serializing_if = "Option::is_none")]
        pub hyper_v: Option<bool>,
        #[doc = "Scaling worker count."]
        #[serde(rename = "targetWorkerCount", default, skip_serializing_if = "Option::is_none")]
        pub target_worker_count: Option<i32>,
        #[doc = "Scaling worker size ID."]
        #[serde(rename = "targetWorkerSizeId", default, skip_serializing_if = "Option::is_none")]
        pub target_worker_size_id: Option<i32>,
        #[doc = "Provisioning state of the App Service Plan."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Specification for a Kubernetes Environment to use for this resource."]
        #[serde(rename = "kubeEnvironmentProfile", default, skip_serializing_if = "Option::is_none")]
        pub kube_environment_profile: Option<KubeEnvironmentProfile>,
        #[doc = "If <code>true</code>, this App Service Plan will perform availability zone balancing.\nIf <code>false</code>, this App Service Plan will not perform availability zone balancing."]
        #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
        pub zone_redundant: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "App Service plan status."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Ready,
            Pending,
            Creating,
        }
        #[doc = "Provisioning state of the App Service Plan."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            InProgress,
            Deleting,
        }
    }
}
#[doc = "Collection of App Service plans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServicePlanCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<AppServicePlan>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AppServicePlanCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AppServicePlanCollection {
    pub fn new(value: Vec<AppServicePlan>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "ARM resource for a app service plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServicePlanPatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "AppServicePlanPatchResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<app_service_plan_patch_resource::Properties>,
}
impl AppServicePlanPatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod app_service_plan_patch_resource {
    use super::*;
    #[doc = "AppServicePlanPatchResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Target worker tier assigned to the App Service plan."]
        #[serde(rename = "workerTierName", default, skip_serializing_if = "Option::is_none")]
        pub worker_tier_name: Option<String>,
        #[doc = "App Service plan status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[doc = "App Service plan subscription."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subscription: Option<String>,
        #[doc = "Specification for an App Service Environment to use for this resource."]
        #[serde(rename = "hostingEnvironmentProfile", default, skip_serializing_if = "Option::is_none")]
        pub hosting_environment_profile: Option<HostingEnvironmentProfile>,
        #[doc = "Maximum number of instances that can be assigned to this App Service plan."]
        #[serde(rename = "maximumNumberOfWorkers", default, skip_serializing_if = "Option::is_none")]
        pub maximum_number_of_workers: Option<i32>,
        #[doc = "Geographical location for the App Service plan."]
        #[serde(rename = "geoRegion", default, skip_serializing_if = "Option::is_none")]
        pub geo_region: Option<String>,
        #[doc = "If <code>true</code>, apps assigned to this App Service plan can be scaled independently.\nIf <code>false</code>, apps assigned to this App Service plan will scale to all instances of the plan."]
        #[serde(rename = "perSiteScaling", default, skip_serializing_if = "Option::is_none")]
        pub per_site_scaling: Option<bool>,
        #[doc = "ServerFarm supports ElasticScale. Apps in this plan will scale as if the ServerFarm was ElasticPremium sku"]
        #[serde(rename = "elasticScaleEnabled", default, skip_serializing_if = "Option::is_none")]
        pub elastic_scale_enabled: Option<bool>,
        #[doc = "Maximum number of total workers allowed for this ElasticScaleEnabled App Service Plan"]
        #[serde(rename = "maximumElasticWorkerCount", default, skip_serializing_if = "Option::is_none")]
        pub maximum_elastic_worker_count: Option<i32>,
        #[doc = "Number of apps assigned to this App Service plan."]
        #[serde(rename = "numberOfSites", default, skip_serializing_if = "Option::is_none")]
        pub number_of_sites: Option<i32>,
        #[doc = "If <code>true</code>, this App Service Plan owns spot instances."]
        #[serde(rename = "isSpot", default, skip_serializing_if = "Option::is_none")]
        pub is_spot: Option<bool>,
        #[doc = "The time when the server farm expires. Valid only if it is a spot server farm."]
        #[serde(rename = "spotExpirationTime", default, with = "azure_core::date::rfc3339::option")]
        pub spot_expiration_time: Option<time::OffsetDateTime>,
        #[doc = "The time when the server farm free offer expires."]
        #[serde(rename = "freeOfferExpirationTime", default, with = "azure_core::date::rfc3339::option")]
        pub free_offer_expiration_time: Option<time::OffsetDateTime>,
        #[doc = "Resource group of the App Service plan."]
        #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
        pub resource_group: Option<String>,
        #[doc = "If Linux app service plan <code>true</code>, <code>false</code> otherwise."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reserved: Option<bool>,
        #[doc = "Obsolete: If Hyper-V container app service plan <code>true</code>, <code>false</code> otherwise."]
        #[serde(rename = "isXenon", default, skip_serializing_if = "Option::is_none")]
        pub is_xenon: Option<bool>,
        #[doc = "If Hyper-V container app service plan <code>true</code>, <code>false</code> otherwise."]
        #[serde(rename = "hyperV", default, skip_serializing_if = "Option::is_none")]
        pub hyper_v: Option<bool>,
        #[doc = "Scaling worker count."]
        #[serde(rename = "targetWorkerCount", default, skip_serializing_if = "Option::is_none")]
        pub target_worker_count: Option<i32>,
        #[doc = "Scaling worker size ID."]
        #[serde(rename = "targetWorkerSizeId", default, skip_serializing_if = "Option::is_none")]
        pub target_worker_size_id: Option<i32>,
        #[doc = "Provisioning state of the App Service Plan."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Specification for a Kubernetes Environment to use for this resource."]
        #[serde(rename = "kubeEnvironmentProfile", default, skip_serializing_if = "Option::is_none")]
        pub kube_environment_profile: Option<KubeEnvironmentProfile>,
        #[doc = "If <code>true</code>, this App Service Plan will perform availability zone balancing.\nIf <code>false</code>, this App Service Plan will not perform availability zone balancing."]
        #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
        pub zone_redundant: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "App Service plan status."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Ready,
            Pending,
            Creating,
        }
        #[doc = "Provisioning state of the App Service Plan."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            InProgress,
            Deleting,
        }
    }
}
#[doc = "The configuration settings of the Apple provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Apple {
    #[doc = "<code>false</code> if the Apple provider should not be enabled despite the set registration; otherwise, <code>true</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The configuration settings of the registration for the Apple provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<AppleRegistration>,
    #[doc = "The configuration settings of the login flow, including the scopes that should be requested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<LoginScopes>,
}
impl Apple {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the registration for the Apple provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppleRegistration {
    #[doc = "The Client ID of the app used for login."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The app setting name that contains the client secret."]
    #[serde(rename = "clientSecretSettingName", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_setting_name: Option<String>,
}
impl AppleRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application logs configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationLogsConfig {
    #[doc = "Application logs to file system configuration."]
    #[serde(rename = "fileSystem", default, skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystemApplicationLogsConfig>,
    #[doc = "Application logs to Azure table storage configuration."]
    #[serde(rename = "azureTableStorage", default, skip_serializing_if = "Option::is_none")]
    pub azure_table_storage: Option<AzureTableStorageApplicationLogsConfig>,
    #[doc = "Application logs azure blob storage configuration."]
    #[serde(rename = "azureBlobStorage", default, skip_serializing_if = "Option::is_none")]
    pub azure_blob_storage: Option<AzureBlobStorageApplicationLogsConfig>,
}
impl ApplicationLogsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application stack."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationStack {
    #[doc = "Application stack name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Application stack display name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[doc = "Application stack dependency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency: Option<String>,
    #[doc = "List of major versions available."]
    #[serde(rename = "majorVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub major_versions: Vec<StackMajorVersion>,
    #[doc = "List of frameworks associated with application stack."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frameworks: Vec<ApplicationStack>,
    #[doc = "<code>true</code> if this is the stack is deprecated; otherwise, <code>false</code>."]
    #[serde(rename = "isDeprecated", default, skip_serializing_if = "Vec::is_empty")]
    pub is_deprecated: Vec<ApplicationStack>,
}
impl ApplicationStack {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Application Stacks"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationStackCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ApplicationStackResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationStackCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApplicationStackCollection {
    pub fn new(value: Vec<ApplicationStackResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "ARM resource for a ApplicationStack."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationStackResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Application stack."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationStack>,
}
impl ApplicationStackResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Github access token for Appservice CLI github integration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppserviceGithubToken {
    #[doc = "Github access token for Appservice CLI github integration"]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[doc = "Scope of the github access token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "token type"]
    #[serde(rename = "tokenType", default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[doc = "True if valid github token received, False otherwise"]
    #[serde(rename = "gotToken", default, skip_serializing_if = "Option::is_none")]
    pub got_token: Option<bool>,
    #[doc = "Error message if unable to get token"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl AppserviceGithubToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Appservice Github token request content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppserviceGithubTokenRequest {
    #[doc = "Code string to exchange for Github Access token"]
    pub code: String,
    #[doc = "State string used for verification."]
    pub state: String,
}
impl AppserviceGithubTokenRequest {
    pub fn new(code: String, state: String) -> Self {
        Self { code, state }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArcConfiguration {
    #[serde(rename = "artifactsStorageType", default, skip_serializing_if = "Option::is_none")]
    pub artifacts_storage_type: Option<arc_configuration::ArtifactsStorageType>,
    #[serde(rename = "artifactStorageClassName", default, skip_serializing_if = "Option::is_none")]
    pub artifact_storage_class_name: Option<String>,
    #[serde(rename = "artifactStorageMountPath", default, skip_serializing_if = "Option::is_none")]
    pub artifact_storage_mount_path: Option<String>,
    #[serde(rename = "artifactStorageNodeName", default, skip_serializing_if = "Option::is_none")]
    pub artifact_storage_node_name: Option<String>,
    #[serde(rename = "artifactStorageAccessMode", default, skip_serializing_if = "Option::is_none")]
    pub artifact_storage_access_mode: Option<String>,
    #[serde(rename = "frontEndServiceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub front_end_service_configuration: Option<FrontEndConfiguration>,
    #[serde(rename = "kubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub kube_config: Option<String>,
}
impl ArcConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod arc_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ArtifactsStorageType {
        LocalNode,
        NetworkFileSystem,
    }
}
#[doc = "A wrapper for an ARM resource id"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmIdWrapper {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ArmIdWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The plan object in Azure Resource Manager, represents a marketplace plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmPlan {
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "The promotion code."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "Version of product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ArmPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Full view of networking configuration for an ASE."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AseV3NetworkingConfiguration {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "AseV3NetworkingConfiguration resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ase_v3_networking_configuration::Properties>,
}
impl AseV3NetworkingConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ase_v3_networking_configuration {
    use super::*;
    #[doc = "AseV3NetworkingConfiguration resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "windowsOutboundIpAddresses", default, skip_serializing_if = "Vec::is_empty")]
        pub windows_outbound_ip_addresses: Vec<String>,
        #[serde(rename = "linuxOutboundIpAddresses", default, skip_serializing_if = "Vec::is_empty")]
        pub linux_outbound_ip_addresses: Vec<String>,
        #[serde(rename = "externalInboundIpAddresses", default, skip_serializing_if = "Vec::is_empty")]
        pub external_inbound_ip_addresses: Vec<String>,
        #[serde(rename = "internalInboundIpAddresses", default, skip_serializing_if = "Vec::is_empty")]
        pub internal_inbound_ip_addresses: Vec<String>,
        #[doc = "Property to enable and disable new private endpoint connection creation on ASE"]
        #[serde(rename = "allowNewPrivateEndpointConnections", default, skip_serializing_if = "Option::is_none")]
        pub allow_new_private_endpoint_connections: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The configuration settings of the platform of App Service Authentication/Authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthPlatform {
    #[doc = "<code>true</code> if the Authentication / Authorization feature is enabled for the current app; otherwise, <code>false</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The RuntimeVersion of the Authentication / Authorization feature in use for the current app.\nThe setting in this value can control the behavior of certain features in the Authentication / Authorization module."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[doc = "The path of the config file containing auth settings if they come from a file.\nIf the path is relative, base will the site's root directory."]
    #[serde(rename = "configFilePath", default, skip_serializing_if = "Option::is_none")]
    pub config_file_path: Option<String>,
}
impl AuthPlatform {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Actions which to take by the auto-heal module when a rule is triggered."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoHealActions {
    #[doc = "Predefined action to be taken."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<auto_heal_actions::ActionType>,
    #[doc = "Custom action to be executed\nwhen an auto heal rule is triggered."]
    #[serde(rename = "customAction", default, skip_serializing_if = "Option::is_none")]
    pub custom_action: Option<AutoHealCustomAction>,
    #[doc = "Minimum time the process must execute\nbefore taking the action"]
    #[serde(rename = "minProcessExecutionTime", default, skip_serializing_if = "Option::is_none")]
    pub min_process_execution_time: Option<String>,
}
impl AutoHealActions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_heal_actions {
    use super::*;
    #[doc = "Predefined action to be taken."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        Recycle,
        LogEvent,
        CustomAction,
    }
}
#[doc = "Custom action to be executed\nwhen an auto heal rule is triggered."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoHealCustomAction {
    #[doc = "Executable to be run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exe: Option<String>,
    #[doc = "Parameters for the executable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}
impl AutoHealCustomAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rules that can be defined for auto-heal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoHealRules {
    #[doc = "Triggers for auto-heal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggers: Option<AutoHealTriggers>,
    #[doc = "Actions which to take by the auto-heal module when a rule is triggered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<AutoHealActions>,
}
impl AutoHealRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Triggers for auto-heal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoHealTriggers {
    #[doc = "Trigger based on total requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<RequestsBasedTrigger>,
    #[doc = "A rule based on private bytes."]
    #[serde(rename = "privateBytesInKB", default, skip_serializing_if = "Option::is_none")]
    pub private_bytes_in_kb: Option<i32>,
    #[doc = "A rule based on status codes."]
    #[serde(rename = "statusCodes", default, skip_serializing_if = "Vec::is_empty")]
    pub status_codes: Vec<StatusCodesBasedTrigger>,
    #[doc = "Trigger based on request execution time."]
    #[serde(rename = "slowRequests", default, skip_serializing_if = "Option::is_none")]
    pub slow_requests: Option<SlowRequestsBasedTrigger>,
    #[doc = "A rule based on multiple Slow Requests Rule with path"]
    #[serde(rename = "slowRequestsWithPath", default, skip_serializing_if = "Vec::is_empty")]
    pub slow_requests_with_path: Vec<SlowRequestsBasedTrigger>,
    #[doc = "A rule based on status codes ranges."]
    #[serde(rename = "statusCodesRange", default, skip_serializing_if = "Vec::is_empty")]
    pub status_codes_range: Vec<StatusCodesRangeBasedTrigger>,
}
impl AutoHealTriggers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the Azure Active directory provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureActiveDirectory {
    #[doc = "<code>false</code> if the Azure Active Directory provider should not be enabled despite the set registration; otherwise, <code>true</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The configuration settings of the Azure Active Directory app registration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<AzureActiveDirectoryRegistration>,
    #[doc = "The configuration settings of the Azure Active Directory login flow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<AzureActiveDirectoryLogin>,
    #[doc = "The configuration settings of the Azure Active Directory token validation flow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<AzureActiveDirectoryValidation>,
    #[doc = "Gets a value indicating whether the Azure AD configuration was auto-provisioned using 1st party tooling.\nThis is an internal flag primarily intended to support the Azure Management Portal. Users should not\nread or write to this property."]
    #[serde(rename = "isAutoProvisioned", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_provisioned: Option<bool>,
}
impl AzureActiveDirectory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the Azure Active Directory login flow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureActiveDirectoryLogin {
    #[doc = "Login parameters to send to the OpenID Connect authorization endpoint when\na user logs in. Each parameter must be in the form \"key=value\"."]
    #[serde(rename = "loginParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub login_parameters: Vec<String>,
    #[doc = "<code>true</code> if the www-authenticate provider should be omitted from the request; otherwise, <code>false</code>."]
    #[serde(rename = "disableWWWAuthenticate", default, skip_serializing_if = "Option::is_none")]
    pub disable_www_authenticate: Option<bool>,
}
impl AzureActiveDirectoryLogin {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the Azure Active Directory app registration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureActiveDirectoryRegistration {
    #[doc = "The OpenID Connect Issuer URI that represents the entity which issues access tokens for this application.\nWhen using Azure Active Directory, this value is the URI of the directory tenant, e.g. https://login.microsoftonline.com/v2.0/{tenant-guid}/.\nThis URI is a case-sensitive identifier for the token issuer.\nMore information on OpenID Connect Discovery: http://openid.net/specs/openid-connect-discovery-1_0.html"]
    #[serde(rename = "openIdIssuer", default, skip_serializing_if = "Option::is_none")]
    pub open_id_issuer: Option<String>,
    #[doc = "The Client ID of this relying party application, known as the client_id.\nThis setting is required for enabling OpenID Connection authentication with Azure Active Directory or \nother 3rd party OpenID Connect providers.\nMore information on OpenID Connect: http://openid.net/specs/openid-connect-core-1_0.html"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The app setting name that contains the client secret of the relying party application."]
    #[serde(rename = "clientSecretSettingName", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_setting_name: Option<String>,
    #[doc = "An alternative to the client secret, that is the thumbprint of a certificate used for signing purposes. This property acts as\na replacement for the Client Secret. It is also optional."]
    #[serde(rename = "clientSecretCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_certificate_thumbprint: Option<String>,
    #[doc = "An alternative to the client secret thumbprint, that is the subject alternative name of a certificate used for signing purposes. This property acts as\na replacement for the Client Secret Certificate Thumbprint. It is also optional."]
    #[serde(
        rename = "clientSecretCertificateSubjectAlternativeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_secret_certificate_subject_alternative_name: Option<String>,
    #[doc = "An alternative to the client secret thumbprint, that is the issuer of a certificate used for signing purposes. This property acts as\na replacement for the Client Secret Certificate Thumbprint. It is also optional."]
    #[serde(rename = "clientSecretCertificateIssuer", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_certificate_issuer: Option<String>,
}
impl AzureActiveDirectoryRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the Azure Active Directory token validation flow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureActiveDirectoryValidation {
    #[doc = "The configuration settings of the checks that should be made while validating the JWT Claims."]
    #[serde(rename = "jwtClaimChecks", default, skip_serializing_if = "Option::is_none")]
    pub jwt_claim_checks: Option<JwtClaimChecks>,
    #[doc = "The list of audiences that can make successful authentication/authorization requests."]
    #[serde(rename = "allowedAudiences", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_audiences: Vec<String>,
    #[doc = "The configuration settings of the Azure Active Directory default authorization policy."]
    #[serde(rename = "defaultAuthorizationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub default_authorization_policy: Option<DefaultAuthorizationPolicy>,
}
impl AzureActiveDirectoryValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application logs azure blob storage configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBlobStorageApplicationLogsConfig {
    #[doc = "Log level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<azure_blob_storage_application_logs_config::Level>,
    #[doc = "SAS url to a azure blob container with read/write/list/delete permissions."]
    #[serde(rename = "sasUrl", default, skip_serializing_if = "Option::is_none")]
    pub sas_url: Option<String>,
    #[doc = "Retention in days.\nRemove blobs older than X days.\n0 or lower means no retention."]
    #[serde(rename = "retentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
}
impl AzureBlobStorageApplicationLogsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_blob_storage_application_logs_config {
    use super::*;
    #[doc = "Log level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        Off,
        Verbose,
        Information,
        Warning,
        Error,
    }
}
#[doc = "Http logs to azure blob storage configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBlobStorageHttpLogsConfig {
    #[doc = "SAS url to a azure blob container with read/write/list/delete permissions."]
    #[serde(rename = "sasUrl", default, skip_serializing_if = "Option::is_none")]
    pub sas_url: Option<String>,
    #[doc = "Retention in days.\nRemove blobs older than X days.\n0 or lower means no retention."]
    #[serde(rename = "retentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
    #[doc = "True if configuration is enabled, false if it is disabled and null if configuration is not set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AzureBlobStorageHttpLogsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the Azure Static Web Apps provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStaticWebApps {
    #[doc = "<code>false</code> if the Azure Static Web Apps provider should not be enabled despite the set registration; otherwise, <code>true</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The configuration settings of the registration for the Azure Static Web Apps provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<AzureStaticWebAppsRegistration>,
}
impl AzureStaticWebApps {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the registration for the Azure Static Web Apps provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStaticWebAppsRegistration {
    #[doc = "The Client ID of the app used for login."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl AzureStaticWebAppsRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Files or Blob Storage access information value for dictionary storage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageInfoValue {
    #[doc = "Type of storage."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<azure_storage_info_value::Type>,
    #[doc = "Name of the storage account."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Name of the file share (container name, for Blob storage)."]
    #[serde(rename = "shareName", default, skip_serializing_if = "Option::is_none")]
    pub share_name: Option<String>,
    #[doc = "Access key for the storage account."]
    #[serde(rename = "accessKey", default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[doc = "Path to mount the storage within the site's runtime environment."]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    #[doc = "State of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<azure_storage_info_value::State>,
}
impl AzureStorageInfoValue {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_storage_info_value {
    use super::*;
    #[doc = "Type of storage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        AzureFiles,
        AzureBlob,
    }
    #[doc = "State of the storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Ok,
        InvalidCredentials,
        InvalidShare,
        NotValidated,
    }
}
#[doc = "AzureStorageInfo dictionary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStoragePropertyDictionaryResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Azure storage accounts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AzureStoragePropertyDictionaryResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application logs to Azure table storage configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureTableStorageApplicationLogsConfig {
    #[doc = "Log level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<azure_table_storage_application_logs_config::Level>,
    #[doc = "SAS URL to an Azure table with add/query/delete permissions."]
    #[serde(rename = "sasUrl")]
    pub sas_url: String,
}
impl AzureTableStorageApplicationLogsConfig {
    pub fn new(sas_url: String) -> Self {
        Self { level: None, sas_url }
    }
}
pub mod azure_table_storage_application_logs_config {
    use super::*;
    #[doc = "Log level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        Off,
        Verbose,
        Information,
        Warning,
        Error,
    }
}
#[doc = "Backup description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupItem {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "BackupItem resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<backup_item::Properties>,
}
impl BackupItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_item {
    use super::*;
    #[doc = "BackupItem resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Id of the backup."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<i32>,
        #[doc = "SAS URL for the storage account container which contains this backup."]
        #[serde(rename = "storageAccountUrl", default, skip_serializing_if = "Option::is_none")]
        pub storage_account_url: Option<String>,
        #[doc = "Name of the blob which contains data for this backup."]
        #[serde(rename = "blobName", default, skip_serializing_if = "Option::is_none")]
        pub blob_name: Option<String>,
        #[doc = "Name of this backup."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "Backup status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[doc = "Size of the backup in bytes."]
        #[serde(rename = "sizeInBytes", default, skip_serializing_if = "Option::is_none")]
        pub size_in_bytes: Option<i64>,
        #[doc = "Timestamp of the backup creation."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        #[doc = "Details regarding this backup. Might contain an error message."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub log: Option<String>,
        #[doc = "List of databases included in the backup."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub databases: Vec<DatabaseBackupSetting>,
        #[doc = "True if this backup has been created due to a schedule being triggered."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scheduled: Option<bool>,
        #[doc = "Timestamp of a last restore operation which used this backup."]
        #[serde(rename = "lastRestoreTimeStamp", default, with = "azure_core::date::rfc3339::option")]
        pub last_restore_time_stamp: Option<time::OffsetDateTime>,
        #[doc = "Timestamp when this backup finished."]
        #[serde(rename = "finishedTimeStamp", default, with = "azure_core::date::rfc3339::option")]
        pub finished_time_stamp: Option<time::OffsetDateTime>,
        #[doc = "Unique correlation identifier. Please use this along with the timestamp while communicating with Azure support."]
        #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
        pub correlation_id: Option<String>,
        #[doc = "Size of the original web app which has been backed up."]
        #[serde(rename = "websiteSizeInBytes", default, skip_serializing_if = "Option::is_none")]
        pub website_size_in_bytes: Option<i64>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Backup status."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            InProgress,
            Failed,
            Succeeded,
            TimedOut,
            Created,
            Skipped,
            PartiallySucceeded,
            DeleteInProgress,
            DeleteFailed,
            Deleted,
        }
    }
}
#[doc = "Collection of backup items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupItemCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<BackupItem>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BackupItemCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BackupItemCollection {
    pub fn new(value: Vec<BackupItem>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Description of a backup which will be performed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupRequest {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "BackupRequest resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<backup_request::Properties>,
}
impl BackupRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_request {
    use super::*;
    #[doc = "BackupRequest resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Name of the backup."]
        #[serde(rename = "backupName", default, skip_serializing_if = "Option::is_none")]
        pub backup_name: Option<String>,
        #[doc = "True if the backup schedule is enabled (must be included in that case), false if the backup schedule should be disabled."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "SAS URL to the container."]
        #[serde(rename = "storageAccountUrl")]
        pub storage_account_url: String,
        #[doc = "Description of a backup schedule. Describes how often should be the backup performed and what should be the retention policy."]
        #[serde(rename = "backupSchedule", default, skip_serializing_if = "Option::is_none")]
        pub backup_schedule: Option<BackupSchedule>,
        #[doc = "Databases included in the backup."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub databases: Vec<DatabaseBackupSetting>,
    }
    impl Properties {
        pub fn new(storage_account_url: String) -> Self {
            Self {
                backup_name: None,
                enabled: None,
                storage_account_url,
                backup_schedule: None,
                databases: Vec::new(),
            }
        }
    }
}
#[doc = "Description of a backup schedule. Describes how often should be the backup performed and what should be the retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupSchedule {
    #[doc = "How often the backup should be executed (e.g. for weekly backup, this should be set to 7 and FrequencyUnit should be set to Day)"]
    #[serde(rename = "frequencyInterval")]
    pub frequency_interval: i32,
    #[doc = "The unit of time for how often the backup should be executed (e.g. for weekly backup, this should be set to Day and FrequencyInterval should be set to 7)"]
    #[serde(rename = "frequencyUnit")]
    pub frequency_unit: backup_schedule::FrequencyUnit,
    #[doc = "True if the retention policy should always keep at least one backup in the storage account, regardless how old it is; false otherwise."]
    #[serde(rename = "keepAtLeastOneBackup")]
    pub keep_at_least_one_backup: bool,
    #[doc = "After how many days backups should be deleted."]
    #[serde(rename = "retentionPeriodInDays")]
    pub retention_period_in_days: i32,
    #[doc = "When the schedule should start working."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Last time when this schedule was triggered."]
    #[serde(rename = "lastExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_execution_time: Option<time::OffsetDateTime>,
}
impl BackupSchedule {
    pub fn new(
        frequency_interval: i32,
        frequency_unit: backup_schedule::FrequencyUnit,
        keep_at_least_one_backup: bool,
        retention_period_in_days: i32,
    ) -> Self {
        Self {
            frequency_interval,
            frequency_unit,
            keep_at_least_one_backup,
            retention_period_in_days,
            start_time: None,
            last_execution_time: None,
        }
    }
}
pub mod backup_schedule {
    use super::*;
    #[doc = "The unit of time for how often the backup should be executed (e.g. for weekly backup, this should be set to Day and FrequencyInterval should be set to 7)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FrequencyUnit {
        Day,
        Hour,
    }
    impl Default for FrequencyUnit {
        fn default() -> Self {
            Self::Day
        }
    }
}
#[doc = "App Service billing entity that contains information about meter which the Azure billing system utilizes to charge users for services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingMeter {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "BillingMeter resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<billing_meter::Properties>,
}
impl BillingMeter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_meter {
    use super::*;
    #[doc = "BillingMeter resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Meter GUID onboarded in Commerce"]
        #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
        pub meter_id: Option<String>,
        #[doc = "Azure Location of billable resource"]
        #[serde(rename = "billingLocation", default, skip_serializing_if = "Option::is_none")]
        pub billing_location: Option<String>,
        #[doc = "Short Name from App Service Azure pricing Page"]
        #[serde(rename = "shortName", default, skip_serializing_if = "Option::is_none")]
        pub short_name: Option<String>,
        #[doc = "Friendly name of the meter"]
        #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
        pub friendly_name: Option<String>,
        #[doc = "App Service ResourceType meter used for"]
        #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
        pub resource_type: Option<String>,
        #[doc = "App Service OS type meter used for"]
        #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
        pub os_type: Option<String>,
        #[doc = "Meter Multiplier"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub multiplier: Option<f64>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Billing Meters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingMeterCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<BillingMeter>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BillingMeterCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BillingMeterCollection {
    pub fn new(value: Vec<BillingMeter>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The configuration settings of the storage of the tokens if blob storage is used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobStorageTokenStore {
    #[doc = "The name of the app setting containing the SAS URL of the blob storage containing the tokens."]
    #[serde(rename = "sasUrlSettingName", default, skip_serializing_if = "Option::is_none")]
    pub sas_url_setting_name: Option<String>,
}
impl BlobStorageTokenStore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the capabilities/features allowed for a specific SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Capability {
    #[doc = "Name of the SKU capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value of the SKU capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Reason of the SKU capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl Capability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SSL certificate for an app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Certificate resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<certificate::Properties>,
}
impl Certificate {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
pub mod certificate {
    use super::*;
    #[doc = "Certificate resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Certificate password."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        #[doc = "Friendly name of the certificate."]
        #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
        pub friendly_name: Option<String>,
        #[doc = "Subject name of the certificate."]
        #[serde(rename = "subjectName", default, skip_serializing_if = "Option::is_none")]
        pub subject_name: Option<String>,
        #[doc = "Host names the certificate applies to."]
        #[serde(rename = "hostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub host_names: Vec<String>,
        #[doc = "Pfx blob."]
        #[serde(rename = "pfxBlob", default, skip_serializing_if = "Option::is_none")]
        pub pfx_blob: Option<String>,
        #[doc = "App name."]
        #[serde(rename = "siteName", default, skip_serializing_if = "Option::is_none")]
        pub site_name: Option<String>,
        #[doc = "Self link."]
        #[serde(rename = "selfLink", default, skip_serializing_if = "Option::is_none")]
        pub self_link: Option<String>,
        #[doc = "Certificate issuer."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub issuer: Option<String>,
        #[doc = "Certificate issue Date."]
        #[serde(rename = "issueDate", default, with = "azure_core::date::rfc3339::option")]
        pub issue_date: Option<time::OffsetDateTime>,
        #[doc = "Certificate expiration date."]
        #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
        pub expiration_date: Option<time::OffsetDateTime>,
        #[doc = "Certificate thumbprint."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub thumbprint: Option<String>,
        #[doc = "Is the certificate valid?."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub valid: Option<bool>,
        #[doc = "Raw bytes of .cer file"]
        #[serde(rename = "cerBlob", default, skip_serializing_if = "Option::is_none")]
        pub cer_blob: Option<String>,
        #[doc = "Public key hash."]
        #[serde(rename = "publicKeyHash", default, skip_serializing_if = "Option::is_none")]
        pub public_key_hash: Option<String>,
        #[doc = "Specification for an App Service Environment to use for this resource."]
        #[serde(rename = "hostingEnvironmentProfile", default, skip_serializing_if = "Option::is_none")]
        pub hosting_environment_profile: Option<HostingEnvironmentProfile>,
        #[doc = "Key Vault Csm resource Id."]
        #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_id: Option<String>,
        #[doc = "Key Vault secret name."]
        #[serde(rename = "keyVaultSecretName", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_secret_name: Option<String>,
        #[doc = "Status of the Key Vault secret."]
        #[serde(rename = "keyVaultSecretStatus", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_secret_status: Option<properties::KeyVaultSecretStatus>,
        #[doc = "Resource ID of the associated App Service plan, formatted as: \"/subscriptions/{subscriptionID}/resourceGroups/{groupName}/providers/Microsoft.Web/serverfarms/{appServicePlanName}\"."]
        #[serde(rename = "serverFarmId", default, skip_serializing_if = "Option::is_none")]
        pub server_farm_id: Option<String>,
        #[doc = "CNAME of the certificate to be issued via free certificate"]
        #[serde(rename = "canonicalName", default, skip_serializing_if = "Option::is_none")]
        pub canonical_name: Option<String>,
        #[doc = "Method of domain validation for free cert"]
        #[serde(rename = "domainValidationMethod", default, skip_serializing_if = "Option::is_none")]
        pub domain_validation_method: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Status of the Key Vault secret."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum KeyVaultSecretStatus {
            Initialized,
            WaitingOnCertificateOrder,
            Succeeded,
            CertificateOrderFailed,
            OperationNotPermittedOnKeyVault,
            AzureServiceUnauthorizedToAccessKeyVault,
            KeyVaultDoesNotExist,
            KeyVaultSecretDoesNotExist,
            UnknownError,
            ExternalPrivateKey,
            Unknown,
        }
    }
}
#[doc = "Collection of certificates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Certificate>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CertificateCollection {
    pub fn new(value: Vec<Certificate>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "SSL certificate details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateDetails {
    #[doc = "Certificate Version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "Certificate Serial Number."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Certificate Thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Certificate Subject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Date Certificate is valid from."]
    #[serde(rename = "notBefore", default, with = "azure_core::date::rfc3339::option")]
    pub not_before: Option<time::OffsetDateTime>,
    #[doc = "Date Certificate is valid to."]
    #[serde(rename = "notAfter", default, with = "azure_core::date::rfc3339::option")]
    pub not_after: Option<time::OffsetDateTime>,
    #[doc = "Certificate Signature algorithm."]
    #[serde(rename = "signatureAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
    #[doc = "Certificate Issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "Raw certificate data."]
    #[serde(rename = "rawData", default, skip_serializing_if = "Option::is_none")]
    pub raw_data: Option<String>,
}
impl CertificateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SSL certificate email."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateEmail {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "CertificateEmail resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<certificate_email::Properties>,
}
impl CertificateEmail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_email {
    use super::*;
    #[doc = "CertificateEmail resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Email id."]
        #[serde(rename = "emailId", default, skip_serializing_if = "Option::is_none")]
        pub email_id: Option<String>,
        #[doc = "Time stamp."]
        #[serde(rename = "timeStamp", default, with = "azure_core::date::rfc3339::option")]
        pub time_stamp: Option<time::OffsetDateTime>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Certificate order action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateOrderAction {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "CertificateOrderAction resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<certificate_order_action::Properties>,
}
impl CertificateOrderAction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_order_action {
    use super::*;
    #[doc = "CertificateOrderAction resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Action type."]
        #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
        pub action_type: Option<properties::ActionType>,
        #[doc = "Time at which the certificate action was performed."]
        #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Action type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ActionType {
            CertificateIssued,
            CertificateOrderCanceled,
            CertificateOrderCreated,
            CertificateRevoked,
            DomainValidationComplete,
            FraudDetected,
            OrgNameChange,
            OrgValidationComplete,
            SanDrop,
            FraudCleared,
            CertificateExpired,
            CertificateExpirationWarning,
            FraudDocumentationRequired,
            Unknown,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateOrderContact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "nameFirst", default, skip_serializing_if = "Option::is_none")]
    pub name_first: Option<String>,
    #[serde(rename = "nameLast", default, skip_serializing_if = "Option::is_none")]
    pub name_last: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl CertificateOrderContact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM resource for a certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificatePatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "CertificatePatchResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<certificate_patch_resource::Properties>,
}
impl CertificatePatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_patch_resource {
    use super::*;
    #[doc = "CertificatePatchResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Certificate password."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        #[doc = "Friendly name of the certificate."]
        #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
        pub friendly_name: Option<String>,
        #[doc = "Subject name of the certificate."]
        #[serde(rename = "subjectName", default, skip_serializing_if = "Option::is_none")]
        pub subject_name: Option<String>,
        #[doc = "Host names the certificate applies to."]
        #[serde(rename = "hostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub host_names: Vec<String>,
        #[doc = "Pfx blob."]
        #[serde(rename = "pfxBlob", default, skip_serializing_if = "Option::is_none")]
        pub pfx_blob: Option<String>,
        #[doc = "App name."]
        #[serde(rename = "siteName", default, skip_serializing_if = "Option::is_none")]
        pub site_name: Option<String>,
        #[doc = "Self link."]
        #[serde(rename = "selfLink", default, skip_serializing_if = "Option::is_none")]
        pub self_link: Option<String>,
        #[doc = "Certificate issuer."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub issuer: Option<String>,
        #[doc = "Certificate issue Date."]
        #[serde(rename = "issueDate", default, with = "azure_core::date::rfc3339::option")]
        pub issue_date: Option<time::OffsetDateTime>,
        #[doc = "Certificate expiration date."]
        #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
        pub expiration_date: Option<time::OffsetDateTime>,
        #[doc = "Certificate thumbprint."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub thumbprint: Option<String>,
        #[doc = "Is the certificate valid?."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub valid: Option<bool>,
        #[doc = "Raw bytes of .cer file"]
        #[serde(rename = "cerBlob", default, skip_serializing_if = "Option::is_none")]
        pub cer_blob: Option<String>,
        #[doc = "Public key hash."]
        #[serde(rename = "publicKeyHash", default, skip_serializing_if = "Option::is_none")]
        pub public_key_hash: Option<String>,
        #[doc = "Specification for an App Service Environment to use for this resource."]
        #[serde(rename = "hostingEnvironmentProfile", default, skip_serializing_if = "Option::is_none")]
        pub hosting_environment_profile: Option<HostingEnvironmentProfile>,
        #[doc = "Key Vault Csm resource Id."]
        #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_id: Option<String>,
        #[doc = "Key Vault secret name."]
        #[serde(rename = "keyVaultSecretName", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_secret_name: Option<String>,
        #[doc = "Status of the Key Vault secret."]
        #[serde(rename = "keyVaultSecretStatus", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_secret_status: Option<properties::KeyVaultSecretStatus>,
        #[doc = "Resource ID of the associated App Service plan, formatted as: \"/subscriptions/{subscriptionID}/resourceGroups/{groupName}/providers/Microsoft.Web/serverfarms/{appServicePlanName}\"."]
        #[serde(rename = "serverFarmId", default, skip_serializing_if = "Option::is_none")]
        pub server_farm_id: Option<String>,
        #[doc = "CNAME of the certificate to be issued via free certificate"]
        #[serde(rename = "canonicalName", default, skip_serializing_if = "Option::is_none")]
        pub canonical_name: Option<String>,
        #[doc = "Method of domain validation for free cert"]
        #[serde(rename = "domainValidationMethod", default, skip_serializing_if = "Option::is_none")]
        pub domain_validation_method: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Status of the Key Vault secret."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum KeyVaultSecretStatus {
            Initialized,
            WaitingOnCertificateOrder,
            Succeeded,
            CertificateOrderFailed,
            OperationNotPermittedOnKeyVault,
            AzureServiceUnauthorizedToAccessKeyVault,
            KeyVaultDoesNotExist,
            KeyVaultSecretDoesNotExist,
            UnknownError,
            ExternalPrivateKey,
            Unknown,
        }
    }
}
#[doc = "The configuration settings of the app registration for providers that have client ids and client secrets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientRegistration {
    #[doc = "The Client ID of the app used for login."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The app setting name that contains the client secret."]
    #[serde(rename = "clientSecretSettingName", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_setting_name: Option<String>,
}
impl ClientRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information needed for cloning operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloningInfo {
    #[doc = "Correlation ID of cloning operation. This ID ties multiple cloning operations\ntogether to use the same snapshot."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "<code>true</code> to overwrite destination app; otherwise, <code>false</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    #[doc = "<code>true</code> to clone custom hostnames from source app; otherwise, <code>false</code>."]
    #[serde(rename = "cloneCustomHostNames", default, skip_serializing_if = "Option::is_none")]
    pub clone_custom_host_names: Option<bool>,
    #[doc = "<code>true</code> to clone source control from source app; otherwise, <code>false</code>."]
    #[serde(rename = "cloneSourceControl", default, skip_serializing_if = "Option::is_none")]
    pub clone_source_control: Option<bool>,
    #[doc = "ARM resource ID of the source app. App resource ID is of the form \n/subscriptions/{subId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{siteName} for production slots and \n/subscriptions/{subId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{siteName}/slots/{slotName} for other slots."]
    #[serde(rename = "sourceWebAppId")]
    pub source_web_app_id: String,
    #[doc = "Location of source app ex: West US or North Europe"]
    #[serde(rename = "sourceWebAppLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_web_app_location: Option<String>,
    #[doc = "App Service Environment."]
    #[serde(rename = "hostingEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub hosting_environment: Option<String>,
    #[doc = "Application setting overrides for cloned app. If specified, these settings override the settings cloned \nfrom source app. Otherwise, application settings from source app are retained."]
    #[serde(rename = "appSettingsOverrides", default, skip_serializing_if = "Option::is_none")]
    pub app_settings_overrides: Option<serde_json::Value>,
    #[doc = "<code>true</code> to configure load balancing for source and destination app."]
    #[serde(rename = "configureLoadBalancing", default, skip_serializing_if = "Option::is_none")]
    pub configure_load_balancing: Option<bool>,
    #[doc = "ARM resource ID of the Traffic Manager profile to use, if it exists. Traffic Manager resource ID is of the form \n/subscriptions/{subId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/trafficManagerProfiles/{profileName}."]
    #[serde(rename = "trafficManagerProfileId", default, skip_serializing_if = "Option::is_none")]
    pub traffic_manager_profile_id: Option<String>,
    #[doc = "Name of Traffic Manager profile to create. This is only needed if Traffic Manager profile does not already exist."]
    #[serde(rename = "trafficManagerProfileName", default, skip_serializing_if = "Option::is_none")]
    pub traffic_manager_profile_name: Option<String>,
}
impl CloningInfo {
    pub fn new(source_web_app_id: String) -> Self {
        Self {
            correlation_id: None,
            overwrite: None,
            clone_custom_host_names: None,
            clone_source_control: None,
            source_web_app_id,
            source_web_app_location: None,
            hosting_environment: None,
            app_settings_overrides: None,
            configure_load_balancing: None,
            traffic_manager_profile_id: None,
            traffic_manager_profile_name: None,
        }
    }
}
#[doc = "Database connection string information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnStringInfo {
    #[doc = "Name of connection string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Connection string value."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "Type of database."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<conn_string_info::Type>,
}
impl ConnStringInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod conn_string_info {
    use super::*;
    #[doc = "Type of database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        MySql,
        #[serde(rename = "SQLServer")]
        SqlServer,
        #[serde(rename = "SQLAzure")]
        SqlAzure,
        Custom,
        NotificationHub,
        ServiceBus,
        EventHub,
        ApiHub,
        DocDb,
        RedisCache,
        #[serde(rename = "PostgreSQL")]
        PostgreSql,
    }
}
#[doc = "Database connection string value to type pair."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnStringValueTypePair {
    #[doc = "Value of pair."]
    pub value: String,
    #[doc = "Type of database."]
    #[serde(rename = "type")]
    pub type_: conn_string_value_type_pair::Type,
}
impl ConnStringValueTypePair {
    pub fn new(value: String, type_: conn_string_value_type_pair::Type) -> Self {
        Self { value, type_ }
    }
}
pub mod conn_string_value_type_pair {
    use super::*;
    #[doc = "Type of database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        MySql,
        #[serde(rename = "SQLServer")]
        SqlServer,
        #[serde(rename = "SQLAzure")]
        SqlAzure,
        Custom,
        NotificationHub,
        ServiceBus,
        EventHub,
        ApiHub,
        DocDb,
        RedisCache,
        #[serde(rename = "PostgreSQL")]
        PostgreSql,
    }
}
#[doc = "String dictionary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionStringDictionary {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Connection strings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ConnectionStringDictionary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contact information for domain registration. If 'Domain Privacy' option is not selected then the contact information is made publicly available through the Whois \ndirectories as per ICANN requirements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    #[doc = "Address information for domain registration."]
    #[serde(rename = "addressMailing", default, skip_serializing_if = "Option::is_none")]
    pub address_mailing: Option<Address>,
    #[doc = "Email address."]
    pub email: String,
    #[doc = "Fax number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[doc = "Job title."]
    #[serde(rename = "jobTitle", default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[doc = "First name."]
    #[serde(rename = "nameFirst")]
    pub name_first: String,
    #[doc = "Last name."]
    #[serde(rename = "nameLast")]
    pub name_last: String,
    #[doc = "Middle name."]
    #[serde(rename = "nameMiddle", default, skip_serializing_if = "Option::is_none")]
    pub name_middle: Option<String>,
    #[doc = "Organization contact belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[doc = "Phone number."]
    pub phone: String,
}
impl Contact {
    pub fn new(email: String, name_first: String, name_last: String, phone: String) -> Self {
        Self {
            address_mailing: None,
            email,
            fax: None,
            job_title: None,
            name_first,
            name_last,
            name_middle: None,
            organization: None,
            phone,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerCpuStatistics {
    #[serde(rename = "cpuUsage", default, skip_serializing_if = "Option::is_none")]
    pub cpu_usage: Option<ContainerCpuUsage>,
    #[serde(rename = "systemCpuUsage", default, skip_serializing_if = "Option::is_none")]
    pub system_cpu_usage: Option<i64>,
    #[serde(rename = "onlineCpuCount", default, skip_serializing_if = "Option::is_none")]
    pub online_cpu_count: Option<i32>,
    #[serde(rename = "throttlingData", default, skip_serializing_if = "Option::is_none")]
    pub throttling_data: Option<ContainerThrottlingData>,
}
impl ContainerCpuStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerCpuUsage {
    #[serde(rename = "totalUsage", default, skip_serializing_if = "Option::is_none")]
    pub total_usage: Option<i64>,
    #[serde(rename = "perCpuUsage", default, skip_serializing_if = "Vec::is_empty")]
    pub per_cpu_usage: Vec<i64>,
    #[serde(rename = "kernelModeUsage", default, skip_serializing_if = "Option::is_none")]
    pub kernel_mode_usage: Option<i64>,
    #[serde(rename = "userModeUsage", default, skip_serializing_if = "Option::is_none")]
    pub user_mode_usage: Option<i64>,
}
impl ContainerCpuUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerInfo {
    #[serde(rename = "currentTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub current_time_stamp: Option<time::OffsetDateTime>,
    #[serde(rename = "previousTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub previous_time_stamp: Option<time::OffsetDateTime>,
    #[serde(rename = "currentCpuStats", default, skip_serializing_if = "Option::is_none")]
    pub current_cpu_stats: Option<ContainerCpuStatistics>,
    #[serde(rename = "previousCpuStats", default, skip_serializing_if = "Option::is_none")]
    pub previous_cpu_stats: Option<ContainerCpuStatistics>,
    #[serde(rename = "memoryStats", default, skip_serializing_if = "Option::is_none")]
    pub memory_stats: Option<ContainerMemoryStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eth0: Option<ContainerNetworkInterfaceStatistics>,
}
impl ContainerInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerMemoryStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
    #[serde(rename = "maxUsage", default, skip_serializing_if = "Option::is_none")]
    pub max_usage: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}
impl ContainerMemoryStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerNetworkInterfaceStatistics {
    #[serde(rename = "rxBytes", default, skip_serializing_if = "Option::is_none")]
    pub rx_bytes: Option<i64>,
    #[serde(rename = "rxPackets", default, skip_serializing_if = "Option::is_none")]
    pub rx_packets: Option<i64>,
    #[serde(rename = "rxErrors", default, skip_serializing_if = "Option::is_none")]
    pub rx_errors: Option<i64>,
    #[serde(rename = "rxDropped", default, skip_serializing_if = "Option::is_none")]
    pub rx_dropped: Option<i64>,
    #[serde(rename = "txBytes", default, skip_serializing_if = "Option::is_none")]
    pub tx_bytes: Option<i64>,
    #[serde(rename = "txPackets", default, skip_serializing_if = "Option::is_none")]
    pub tx_packets: Option<i64>,
    #[serde(rename = "txErrors", default, skip_serializing_if = "Option::is_none")]
    pub tx_errors: Option<i64>,
    #[serde(rename = "txDropped", default, skip_serializing_if = "Option::is_none")]
    pub tx_dropped: Option<i64>,
}
impl ContainerNetworkInterfaceStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerThrottlingData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periods: Option<i32>,
    #[serde(rename = "throttledPeriods", default, skip_serializing_if = "Option::is_none")]
    pub throttled_periods: Option<i32>,
    #[serde(rename = "throttledTime", default, skip_serializing_if = "Option::is_none")]
    pub throttled_time: Option<i32>,
}
impl ContainerThrottlingData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Continuous Web Job Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContinuousWebJob {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "ContinuousWebJob resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<continuous_web_job::Properties>,
}
impl ContinuousWebJob {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod continuous_web_job {
    use super::*;
    #[doc = "ContinuousWebJob resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Job status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[doc = "Detailed status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub detailed_status: Option<String>,
        #[doc = "Log URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub log_url: Option<String>,
        #[doc = "Run command."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub run_command: Option<String>,
        #[doc = "Job URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        #[doc = "Extra Info URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extra_info_url: Option<String>,
        #[doc = "Job type."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub web_job_type: Option<properties::WebJobType>,
        #[doc = "Error information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        #[doc = "Using SDK?"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub using_sdk: Option<bool>,
        #[doc = "Job settings."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub settings: Option<serde_json::Value>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Job status."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Initializing,
            Starting,
            Running,
            PendingRestart,
            Stopped,
        }
        #[doc = "Job type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum WebJobType {
            Continuous,
            Triggered,
        }
    }
}
#[doc = "Collection of Kudu continuous web job information elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinuousWebJobCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ContinuousWebJob>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ContinuousWebJobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ContinuousWebJobCollection {
    pub fn new(value: Vec<ContinuousWebJob>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The configuration settings of the session cookie's expiration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CookieExpiration {
    #[doc = "The convention used when determining the session cookie's expiration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convention: Option<cookie_expiration::Convention>,
    #[doc = "The time after the request is made when the session cookie should expire."]
    #[serde(rename = "timeToExpiration", default, skip_serializing_if = "Option::is_none")]
    pub time_to_expiration: Option<String>,
}
impl CookieExpiration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cookie_expiration {
    use super::*;
    #[doc = "The convention used when determining the session cookie's expiration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Convention {
        FixedTime,
        IdentityProviderDerived,
    }
}
#[doc = "Cross-Origin Resource Sharing (CORS) settings for the app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CorsSettings {
    #[doc = "Gets or sets the list of origins that should be allowed to make cross-origin\ncalls (for example: http://example.com:12345). Use \"*\" to allow all."]
    #[serde(rename = "allowedOrigins", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_origins: Vec<String>,
    #[doc = "Gets or sets whether CORS requests with credentials are allowed. See \nhttps://developer.mozilla.org/en-US/docs/Web/HTTP/CORS#Requests_with_credentials\nfor more details."]
    #[serde(rename = "supportCredentials", default, skip_serializing_if = "Option::is_none")]
    pub support_credentials: Option<bool>,
}
impl CorsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object with a list of the resources that need to be moved and the resource group they should be moved to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsmMoveResourceEnvelope {
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
}
impl CsmMoveResourceEnvelope {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Azure resource manager operation metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsmOperationCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<CsmOperationDescription>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CsmOperationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CsmOperationCollection {
    pub fn new(value: Vec<CsmOperationDescription>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Description of an operation available for Microsoft.Web resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsmOperationDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Meta data about operation used for display in portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<CsmOperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Properties available for a Microsoft.Web resource provider operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CsmOperationDescriptionProperties>,
}
impl CsmOperationDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties available for a Microsoft.Web resource provider operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsmOperationDescriptionProperties {
    #[doc = "Resource metrics service provided by Microsoft.Insights resource provider."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl CsmOperationDescriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Meta data about operation used for display in portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsmOperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CsmOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Publishing Credentials Policies parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsmPublishingCredentialsPoliciesEntity {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "CsmPublishingCredentialsPoliciesEntity resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<csm_publishing_credentials_policies_entity::Properties>,
}
impl CsmPublishingCredentialsPoliciesEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod csm_publishing_credentials_policies_entity {
    use super::*;
    #[doc = "CsmPublishingCredentialsPoliciesEntity resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "<code>true</code> to allow access to a publishing method; otherwise, <code>false</code>."]
        pub allow: bool,
    }
    impl Properties {
        pub fn new(allow: bool) -> Self {
            Self { allow }
        }
    }
}
#[doc = "Publishing options for requested profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsmPublishingProfileOptions {
    #[doc = "Name of the format. Valid values are: \nFileZilla3\nWebDeploy -- default\nFtp"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<csm_publishing_profile_options::Format>,
    #[doc = "Include the DisasterRecover endpoint if true"]
    #[serde(rename = "includeDisasterRecoveryEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub include_disaster_recovery_endpoints: Option<bool>,
}
impl CsmPublishingProfileOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod csm_publishing_profile_options {
    use super::*;
    #[doc = "Name of the format. Valid values are: \nFileZilla3\nWebDeploy -- default\nFtp"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Format")]
    pub enum Format {
        FileZilla3,
        WebDeploy,
        Ftp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Format {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Format {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Format {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::FileZilla3 => serializer.serialize_unit_variant("Format", 0u32, "FileZilla3"),
                Self::WebDeploy => serializer.serialize_unit_variant("Format", 1u32, "WebDeploy"),
                Self::Ftp => serializer.serialize_unit_variant("Format", 2u32, "Ftp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Deployment slot parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsmSlotEntity {
    #[doc = "Destination deployment slot during swap operation."]
    #[serde(rename = "targetSlot")]
    pub target_slot: String,
    #[doc = "<code>true</code> to preserve Virtual Network to the slot during swap; otherwise, <code>false</code>."]
    #[serde(rename = "preserveVnet")]
    pub preserve_vnet: bool,
}
impl CsmSlotEntity {
    pub fn new(target_slot: String, preserve_vnet: bool) -> Self {
        Self {
            target_slot,
            preserve_vnet,
        }
    }
}
#[doc = "Usage of the quota resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsmUsageQuota {
    #[doc = "Units of measurement for the quota resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Next reset time for the resource counter."]
    #[serde(rename = "nextResetTime", default, with = "azure_core::date::rfc3339::option")]
    pub next_reset_time: Option<time::OffsetDateTime>,
    #[doc = "The current value of the resource counter."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The resource limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "Localizable string object containing the name and a localized value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
}
impl CsmUsageQuota {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of CSM usage quotas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsmUsageQuotaCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<CsmUsageQuota>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CsmUsageQuotaCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CsmUsageQuotaCollection {
    pub fn new(value: Vec<CsmUsageQuota>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Custom domain analysis."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomHostnameAnalysisResult {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "CustomHostnameAnalysisResult resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<custom_hostname_analysis_result::Properties>,
}
impl CustomHostnameAnalysisResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod custom_hostname_analysis_result {
    use super::*;
    #[doc = "CustomHostnameAnalysisResult resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "<code>true</code> if hostname is already verified; otherwise, <code>false</code>."]
        #[serde(rename = "isHostnameAlreadyVerified", default, skip_serializing_if = "Option::is_none")]
        pub is_hostname_already_verified: Option<bool>,
        #[doc = "DNS verification test result."]
        #[serde(rename = "customDomainVerificationTest", default, skip_serializing_if = "Option::is_none")]
        pub custom_domain_verification_test: Option<properties::CustomDomainVerificationTest>,
        #[doc = "Body of the error response returned from the API."]
        #[serde(rename = "customDomainVerificationFailureInfo", default, skip_serializing_if = "Option::is_none")]
        pub custom_domain_verification_failure_info: Option<ErrorEntity>,
        #[doc = "<code>true</code> if there is a conflict on a scale unit; otherwise, <code>false</code>."]
        #[serde(rename = "hasConflictOnScaleUnit", default, skip_serializing_if = "Option::is_none")]
        pub has_conflict_on_scale_unit: Option<bool>,
        #[doc = "<code>true</code> if there is a conflict across subscriptions; otherwise, <code>false</code>."]
        #[serde(rename = "hasConflictAcrossSubscription", default, skip_serializing_if = "Option::is_none")]
        pub has_conflict_across_subscription: Option<bool>,
        #[doc = "Name of the conflicting app on scale unit if it's within the same subscription."]
        #[serde(rename = "conflictingAppResourceId", default, skip_serializing_if = "Option::is_none")]
        pub conflicting_app_resource_id: Option<String>,
        #[doc = "CName records controller can see for this hostname."]
        #[serde(rename = "cNameRecords", default, skip_serializing_if = "Vec::is_empty")]
        pub c_name_records: Vec<String>,
        #[doc = "TXT records controller can see for this hostname."]
        #[serde(rename = "txtRecords", default, skip_serializing_if = "Vec::is_empty")]
        pub txt_records: Vec<String>,
        #[doc = "A records controller can see for this hostname."]
        #[serde(rename = "aRecords", default, skip_serializing_if = "Vec::is_empty")]
        pub a_records: Vec<String>,
        #[doc = "Alternate CName records controller can see for this hostname."]
        #[serde(rename = "alternateCNameRecords", default, skip_serializing_if = "Vec::is_empty")]
        pub alternate_c_name_records: Vec<String>,
        #[doc = "Alternate TXT records controller can see for this hostname."]
        #[serde(rename = "alternateTxtRecords", default, skip_serializing_if = "Vec::is_empty")]
        pub alternate_txt_records: Vec<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "DNS verification test result."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum CustomDomainVerificationTest {
            Passed,
            Failed,
            Skipped,
        }
    }
}
#[doc = "The configuration settings of the custom Open ID Connect provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomOpenIdConnectProvider {
    #[doc = "<code>false</code> if the custom Open ID provider provider should not be enabled; otherwise, <code>true</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The configuration settings of the app registration for the custom Open ID Connect provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<OpenIdConnectRegistration>,
    #[doc = "The configuration settings of the login flow of the custom Open ID Connect provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<OpenIdConnectLogin>,
}
impl CustomOpenIdConnectProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional configuration for a data providers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataProviderMetadata {
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[doc = "Settings for the data provider"]
    #[serde(rename = "propertyBag", default, skip_serializing_if = "Vec::is_empty")]
    pub property_bag: Vec<KeyValuePairStringObject>,
}
impl DataProviderMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing data source used by the detectors"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSource {
    #[doc = "Instructions if any for the data source"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instructions: Vec<String>,
    #[doc = "Datasource Uri Links"]
    #[serde(rename = "dataSourceUri", default, skip_serializing_if = "Vec::is_empty")]
    pub data_source_uri: Vec<NameValuePair>,
}
impl DataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Column definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataTableResponseColumn {
    #[doc = "Name of the column"]
    #[serde(rename = "columnName", default, skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[doc = "Data type which looks like 'String' or 'Int32'."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[doc = "Column Type"]
    #[serde(rename = "columnType", default, skip_serializing_if = "Option::is_none")]
    pub column_type: Option<String>,
}
impl DataTableResponseColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Table which defines columns and raw row values"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataTableResponseObject {
    #[doc = "Name of the table"]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "List of columns with data types"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<DataTableResponseColumn>,
    #[doc = "Raw row values"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Vec<String>>,
}
impl DataTableResponseObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database backup settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseBackupSetting {
    #[doc = "Database type (e.g. SqlAzure / MySql)."]
    #[serde(rename = "databaseType")]
    pub database_type: database_backup_setting::DatabaseType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contains a connection string name that is linked to the SiteConfig.ConnectionStrings.\nThis is used during restore with overwrite connection strings options."]
    #[serde(rename = "connectionStringName", default, skip_serializing_if = "Option::is_none")]
    pub connection_string_name: Option<String>,
    #[doc = "Contains a connection string to a database which is being backed up or restored. If the restore should happen to a new database, the database name inside is the new one."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
}
impl DatabaseBackupSetting {
    pub fn new(database_type: database_backup_setting::DatabaseType) -> Self {
        Self {
            database_type,
            name: None,
            connection_string_name: None,
            connection_string: None,
        }
    }
}
pub mod database_backup_setting {
    use super::*;
    #[doc = "Database type (e.g. SqlAzure / MySql)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DatabaseType")]
    pub enum DatabaseType {
        SqlAzure,
        MySql,
        LocalMySql,
        PostgreSql,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DatabaseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DatabaseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DatabaseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SqlAzure => serializer.serialize_unit_variant("DatabaseType", 0u32, "SqlAzure"),
                Self::MySql => serializer.serialize_unit_variant("DatabaseType", 1u32, "MySql"),
                Self::LocalMySql => serializer.serialize_unit_variant("DatabaseType", 2u32, "LocalMySql"),
                Self::PostgreSql => serializer.serialize_unit_variant("DatabaseType", 3u32, "PostgreSql"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The configuration settings of the Azure Active Directory default authorization policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultAuthorizationPolicy {
    #[doc = "The configuration settings of the Azure Active Directory allowed principals."]
    #[serde(rename = "allowedPrincipals", default, skip_serializing_if = "Option::is_none")]
    pub allowed_principals: Option<AllowedPrincipals>,
    #[doc = "The configuration settings of the Azure Active Directory allowed applications."]
    #[serde(rename = "allowedApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_applications: Vec<String>,
}
impl DefaultAuthorizationPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "App Service error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultErrorResponse {
    #[doc = "Error model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<default_error_response::Error>,
}
impl azure_core::Continuable for DefaultErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DefaultErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod default_error_response {
    use super::*;
    #[doc = "Error model."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Standardized string to programmatically identify the error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Detailed error description and debugging information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "Detailed error description and debugging information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<serde_json::Value>,
        #[doc = "More information to debug error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub innererror: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Details about restoring a deleted app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedAppRestoreRequest {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "DeletedAppRestoreRequest resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<deleted_app_restore_request::Properties>,
}
impl DeletedAppRestoreRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deleted_app_restore_request {
    use super::*;
    #[doc = "DeletedAppRestoreRequest resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "ARM resource ID of the deleted app. Example:\n/subscriptions/{subId}/providers/Microsoft.Web/deletedSites/{deletedSiteId}"]
        #[serde(rename = "deletedSiteId", default, skip_serializing_if = "Option::is_none")]
        pub deleted_site_id: Option<String>,
        #[doc = "If true, deleted site configuration, in addition to content, will be restored."]
        #[serde(rename = "recoverConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub recover_configuration: Option<bool>,
        #[doc = "Point in time to restore the deleted app from, formatted as a DateTime string. \nIf unspecified, default value is the time that the app was deleted."]
        #[serde(rename = "snapshotTime", default, skip_serializing_if = "Option::is_none")]
        pub snapshot_time: Option<String>,
        #[doc = "If true, the snapshot is retrieved from DRSecondary endpoint."]
        #[serde(rename = "useDRSecondary", default, skip_serializing_if = "Option::is_none")]
        pub use_dr_secondary: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A deleted app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedSite {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "DeletedSite resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<deleted_site::Properties>,
}
impl DeletedSite {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deleted_site {
    use super::*;
    #[doc = "DeletedSite resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Numeric id for the deleted site"]
        #[serde(rename = "deletedSiteId", default, skip_serializing_if = "Option::is_none")]
        pub deleted_site_id: Option<i32>,
        #[doc = "Time in UTC when the app was deleted."]
        #[serde(rename = "deletedTimestamp", default, skip_serializing_if = "Option::is_none")]
        pub deleted_timestamp: Option<String>,
        #[doc = "Subscription containing the deleted site"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subscription: Option<String>,
        #[doc = "ResourceGroup that contained the deleted site"]
        #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
        pub resource_group: Option<String>,
        #[doc = "Name of the deleted site"]
        #[serde(rename = "deletedSiteName", default, skip_serializing_if = "Option::is_none")]
        pub deleted_site_name: Option<String>,
        #[doc = "Slot of the deleted site"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub slot: Option<String>,
        #[doc = "Kind of site that was deleted"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kind: Option<String>,
        #[doc = "Geo Region of the deleted site"]
        #[serde(rename = "geoRegionName", default, skip_serializing_if = "Option::is_none")]
        pub geo_region_name: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of deleted apps."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedWebAppCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<DeletedSite>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedWebAppCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeletedWebAppCollection {
    pub fn new(value: Vec<DeletedSite>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "User credentials used for publishing activity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Deployment {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Deployment resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<deployment::Properties>,
}
impl Deployment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment {
    use super::*;
    #[doc = "Deployment resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Deployment status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<i32>,
        #[doc = "Details about deployment status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "Who authored the deployment."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub author: Option<String>,
        #[doc = "Who performed the deployment."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deployer: Option<String>,
        #[doc = "Author email."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub author_email: Option<String>,
        #[doc = "Start time."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub start_time: Option<time::OffsetDateTime>,
        #[doc = "End time."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub end_time: Option<time::OffsetDateTime>,
        #[doc = "True if deployment is currently active, false if completed and null if not started."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
        #[doc = "Details on deployment."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub details: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of app deployments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Deployment>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeploymentCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeploymentCollection {
    pub fn new(value: Vec<Deployment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "List of available locations (regions or App Service Environments) for\ndeployment of App Service resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentLocations {
    #[doc = "Available regions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<GeoRegion>,
    #[doc = "Available App Service Environments with full descriptions of the environments."]
    #[serde(rename = "hostingEnvironments", default, skip_serializing_if = "Vec::is_empty")]
    pub hosting_environments: Vec<AppServiceEnvironment>,
    #[doc = "Available App Service Environments with basic information."]
    #[serde(rename = "hostingEnvironmentDeploymentInfos", default, skip_serializing_if = "Vec::is_empty")]
    pub hosting_environment_deployment_infos: Vec<HostingEnvironmentDeploymentInfo>,
}
impl DeploymentLocations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing Abnormal Time Period detected."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetectorAbnormalTimePeriod {
    #[doc = "Start time of the correlated event"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the correlated event"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Message describing the event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Represents the name of the Detector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Represents the rank of the Detector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
    #[doc = "Downtime metadata"]
    #[serde(rename = "metaData", default, skip_serializing_if = "Vec::is_empty")]
    pub meta_data: Vec<Vec<NameValuePair>>,
    #[doc = "Represents the type of the Detector"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<detector_abnormal_time_period::Type>,
    #[doc = "List of proposed solutions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub solutions: Vec<Solution>,
}
impl DetectorAbnormalTimePeriod {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod detector_abnormal_time_period {
    use super::*;
    #[doc = "Represents the type of the Detector"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        ServiceIncident,
        AppDeployment,
        AppCrash,
        RuntimeIssueDetected,
        AseDeployment,
        UserIssue,
        PlatformIssue,
        Other,
    }
}
#[doc = "Class representing detector definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetectorDefinition {
    #[doc = "Display name of the detector"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of the detector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Detector Rank"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<f64>,
    #[doc = "Flag representing whether detector is enabled or not."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}
impl DetectorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM resource for a detector definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetectorDefinitionResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Class representing detector definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DetectorDefinition>,
}
impl DetectorDefinitionResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of Detector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetectorInfo {
    #[doc = "Id of detector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of detector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Short description of the detector and its purpose."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Author of the detector."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "Problem category. This serves for organizing group for detectors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "List of Support Topics for which this detector is enabled."]
    #[serde(rename = "supportTopicList", default, skip_serializing_if = "Vec::is_empty")]
    pub support_topic_list: Vec<SupportTopic>,
    #[doc = "Analysis Types for which this detector should apply to."]
    #[serde(rename = "analysisType", default, skip_serializing_if = "Vec::is_empty")]
    pub analysis_type: Vec<String>,
    #[doc = "Whether this detector is an Analysis Detector or not."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<detector_info::Type>,
    #[doc = "Defines score of a detector to power ML based matching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}
impl DetectorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod detector_info {
    use super::*;
    #[doc = "Whether this detector is an Analysis Detector or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Detector,
        Analysis,
        CategoryOverview,
    }
}
#[doc = "Class representing Response from Detector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetectorResponse {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "DetectorResponse resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<detector_response::Properties>,
}
impl DetectorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod detector_response {
    use super::*;
    #[doc = "DetectorResponse resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Definition of Detector"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metadata: Option<DetectorInfo>,
        #[doc = "Data Set"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub dataset: Vec<DiagnosticData>,
        #[doc = "Identify the status of the most severe insight generated by the detector."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<Status>,
        #[doc = "Additional configuration for different data providers to be used by the UI"]
        #[serde(rename = "dataProvidersMetadata", default, skip_serializing_if = "Vec::is_empty")]
        pub data_providers_metadata: Vec<DataProviderMetadata>,
        #[doc = "Suggested utterances where the detector can be applicable"]
        #[serde(rename = "suggestedUtterances", default, skip_serializing_if = "Option::is_none")]
        pub suggested_utterances: Option<QueryUtterancesResults>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of detector responses"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetectorResponseCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<DetectorResponse>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DetectorResponseCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DetectorResponseCollection {
    pub fn new(value: Vec<DetectorResponse>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class representing a diagnostic analysis done on an application"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticAnalysis {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "DiagnosticAnalysis resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<diagnostic_analysis::Properties>,
}
impl DiagnosticAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnostic_analysis {
    use super::*;
    #[doc = "DiagnosticAnalysis resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Start time of the period"]
        #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
        pub start_time: Option<time::OffsetDateTime>,
        #[doc = "End time of the period"]
        #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
        pub end_time: Option<time::OffsetDateTime>,
        #[doc = "List of time periods."]
        #[serde(rename = "abnormalTimePeriods", default, skip_serializing_if = "Vec::is_empty")]
        pub abnormal_time_periods: Vec<AbnormalTimePeriod>,
        #[doc = "Data by each detector"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub payload: Vec<AnalysisData>,
        #[doc = "Data by each detector for detectors that did not corelate"]
        #[serde(rename = "nonCorrelatedDetectors", default, skip_serializing_if = "Vec::is_empty")]
        pub non_correlated_detectors: Vec<DetectorDefinition>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Diagnostic Analyses"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticAnalysisCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<AnalysisDefinition>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiagnosticAnalysisCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DiagnosticAnalysisCollection {
    pub fn new(value: Vec<AnalysisDefinition>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class representing detector definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticCategory {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "DiagnosticCategory resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<diagnostic_category::Properties>,
}
impl DiagnosticCategory {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnostic_category {
    use super::*;
    #[doc = "DiagnosticCategory resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Description of the diagnostic category"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Diagnostic Categories"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticCategoryCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<DiagnosticCategory>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiagnosticCategoryCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DiagnosticCategoryCollection {
    pub fn new(value: Vec<DiagnosticCategory>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Set of data with rendering instructions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticData {
    #[doc = "Data Table which defines columns and raw row values"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<DataTableResponseObject>,
    #[doc = "Instructions for rendering the data"]
    #[serde(rename = "renderingProperties", default, skip_serializing_if = "Option::is_none")]
    pub rendering_properties: Option<Rendering>,
}
impl DiagnosticData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Diagnostic Detectors"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticDetectorCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<DetectorDefinitionResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiagnosticDetectorCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DiagnosticDetectorCollection {
    pub fn new(value: Vec<DetectorDefinitionResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class representing Response from Diagnostic Detectors"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticDetectorResponse {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "DiagnosticDetectorResponse resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<diagnostic_detector_response::Properties>,
}
impl DiagnosticDetectorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnostic_detector_response {
    use super::*;
    #[doc = "DiagnosticDetectorResponse resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Start time of the period"]
        #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
        pub start_time: Option<time::OffsetDateTime>,
        #[doc = "End time of the period"]
        #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
        pub end_time: Option<time::OffsetDateTime>,
        #[doc = "Flag representing Issue was detected."]
        #[serde(rename = "issueDetected", default, skip_serializing_if = "Option::is_none")]
        pub issue_detected: Option<bool>,
        #[doc = "Class representing detector definition"]
        #[serde(rename = "detectorDefinition", default, skip_serializing_if = "Option::is_none")]
        pub detector_definition: Option<DetectorDefinition>,
        #[doc = "Metrics provided by the detector"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub metrics: Vec<DiagnosticMetricSet>,
        #[doc = "List of Correlated events found by the detector"]
        #[serde(rename = "abnormalTimePeriods", default, skip_serializing_if = "Vec::is_empty")]
        pub abnormal_time_periods: Vec<DetectorAbnormalTimePeriod>,
        #[doc = "Additional Data that detector wants to send."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub data: Vec<Vec<NameValuePair>>,
        #[serde(rename = "responseMetaData", default, skip_serializing_if = "Option::is_none")]
        pub response_meta_data: Option<ResponseMetaData>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Class representing Diagnostic Metric"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticMetricSample {
    #[doc = "Time at which metric is measured"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Role Instance. Null if this counter is not per instance \nThis is returned and should be whichever instance name we desire to be returned\ni.e. CPU and Memory return RDWORKERNAME (LargeDed..._IN_0) \nwhere RDWORKERNAME is Machine name below and RoleInstance name in parenthesis"]
    #[serde(rename = "roleInstance", default, skip_serializing_if = "Option::is_none")]
    pub role_instance: Option<String>,
    #[doc = "Total value of the metric. If multiple measurements are made this will have sum of all."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[doc = "Maximum of the metric sampled during the time period"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[doc = "Minimum of the metric sampled during the time period"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[doc = "Whether the values are aggregates across all workers or not"]
    #[serde(rename = "isAggregated", default, skip_serializing_if = "Option::is_none")]
    pub is_aggregated: Option<bool>,
}
impl DiagnosticMetricSample {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing Diagnostic Metric information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticMetricSet {
    #[doc = "Name of the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metric's unit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Start time of the period"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the period"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Presented time grain. Supported grains at the moment are PT1M, PT1H, P1D"]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "Collection of metric values for the selected period based on the {Microsoft.Web.Hosting.Administration.DiagnosticMetricSet.TimeGrain}"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<DiagnosticMetricSample>,
}
impl DiagnosticMetricSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dimension of a resource metric. For e.g. instance specific HTTP requests for a web app, \nwhere instance name is dimension of the metric HTTP request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Domain resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<domain::Properties>,
}
impl Domain {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
pub mod domain {
    use super::*;
    #[doc = "Domain resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Contact information for domain registration. If 'Domain Privacy' option is not selected then the contact information is made publicly available through the Whois \ndirectories as per ICANN requirements."]
        #[serde(rename = "contactAdmin")]
        pub contact_admin: Contact,
        #[doc = "Contact information for domain registration. If 'Domain Privacy' option is not selected then the contact information is made publicly available through the Whois \ndirectories as per ICANN requirements."]
        #[serde(rename = "contactBilling")]
        pub contact_billing: Contact,
        #[doc = "Contact information for domain registration. If 'Domain Privacy' option is not selected then the contact information is made publicly available through the Whois \ndirectories as per ICANN requirements."]
        #[serde(rename = "contactRegistrant")]
        pub contact_registrant: Contact,
        #[doc = "Contact information for domain registration. If 'Domain Privacy' option is not selected then the contact information is made publicly available through the Whois \ndirectories as per ICANN requirements."]
        #[serde(rename = "contactTech")]
        pub contact_tech: Contact,
        #[doc = "Domain registration status."]
        #[serde(rename = "registrationStatus", default, skip_serializing_if = "Option::is_none")]
        pub registration_status: Option<properties::RegistrationStatus>,
        #[doc = "Domain provisioning state."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Name servers."]
        #[serde(rename = "nameServers", default, skip_serializing_if = "Vec::is_empty")]
        pub name_servers: Vec<String>,
        #[doc = "<code>true</code> if domain privacy is enabled for this domain; otherwise, <code>false</code>."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub privacy: Option<bool>,
        #[doc = "Domain creation timestamp."]
        #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
        pub created_time: Option<time::OffsetDateTime>,
        #[doc = "Domain expiration timestamp."]
        #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
        pub expiration_time: Option<time::OffsetDateTime>,
        #[doc = "Timestamp when the domain was renewed last time."]
        #[serde(rename = "lastRenewedTime", default, with = "azure_core::date::rfc3339::option")]
        pub last_renewed_time: Option<time::OffsetDateTime>,
        #[doc = "<code>true</code> if the domain should be automatically renewed; otherwise, <code>false</code>."]
        #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
        pub auto_renew: Option<bool>,
        #[doc = "<code>true</code> if Azure can assign this domain to App Service apps; otherwise, <code>false</code>. This value will be <code>true</code> if domain registration status is active and \n it is hosted on name servers Azure has programmatic access to."]
        #[serde(rename = "readyForDnsRecordManagement", default, skip_serializing_if = "Option::is_none")]
        pub ready_for_dns_record_management: Option<bool>,
        #[doc = "All hostnames derived from the domain and assigned to Azure resources."]
        #[serde(rename = "managedHostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub managed_host_names: Vec<HostName>,
        #[doc = "Domain purchase consent object, representing acceptance of applicable legal agreements."]
        pub consent: DomainPurchaseConsent,
        #[doc = "Reasons why domain is not renewable."]
        #[serde(rename = "domainNotRenewableReasons", default, skip_serializing_if = "Vec::is_empty")]
        pub domain_not_renewable_reasons: Vec<String>,
        #[doc = "Current DNS type"]
        #[serde(rename = "dnsType", default, skip_serializing_if = "Option::is_none")]
        pub dns_type: Option<properties::DnsType>,
        #[doc = "Azure DNS Zone to use"]
        #[serde(rename = "dnsZoneId", default, skip_serializing_if = "Option::is_none")]
        pub dns_zone_id: Option<String>,
        #[doc = "Target DNS type (would be used for migration)"]
        #[serde(rename = "targetDnsType", default, skip_serializing_if = "Option::is_none")]
        pub target_dns_type: Option<properties::TargetDnsType>,
        #[serde(rename = "authCode", default, skip_serializing_if = "Option::is_none")]
        pub auth_code: Option<String>,
    }
    impl Properties {
        pub fn new(
            contact_admin: Contact,
            contact_billing: Contact,
            contact_registrant: Contact,
            contact_tech: Contact,
            consent: DomainPurchaseConsent,
        ) -> Self {
            Self {
                contact_admin,
                contact_billing,
                contact_registrant,
                contact_tech,
                registration_status: None,
                provisioning_state: None,
                name_servers: Vec::new(),
                privacy: None,
                created_time: None,
                expiration_time: None,
                last_renewed_time: None,
                auto_renew: None,
                ready_for_dns_record_management: None,
                managed_host_names: Vec::new(),
                consent,
                domain_not_renewable_reasons: Vec::new(),
                dns_type: None,
                dns_zone_id: None,
                target_dns_type: None,
                auth_code: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Domain registration status."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum RegistrationStatus {
            Active,
            Awaiting,
            Cancelled,
            Confiscated,
            Disabled,
            Excluded,
            Expired,
            Failed,
            Held,
            Locked,
            Parked,
            Pending,
            Reserved,
            Reverted,
            Suspended,
            Transferred,
            Unknown,
            Unlocked,
            Unparked,
            Updated,
            JsonConverterFailed,
        }
        #[doc = "Domain provisioning state."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            InProgress,
            Deleting,
        }
        #[doc = "Current DNS type"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum DnsType {
            AzureDns,
            DefaultDomainRegistrarDns,
        }
        #[doc = "Target DNS type (would be used for migration)"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum TargetDnsType {
            AzureDns,
            DefaultDomainRegistrarDns,
        }
    }
}
#[doc = "Domain availability check result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainAvailabilityCheckResult {
    #[doc = "Name of the domain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "<code>true</code> if domain can be purchased using CreateDomain API; otherwise, <code>false</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[doc = "Valid values are Regular domain: Azure will charge the full price of domain registration, SoftDeleted: Purchasing this domain will simply restore it and this operation will not cost anything."]
    #[serde(rename = "domainType", default, skip_serializing_if = "Option::is_none")]
    pub domain_type: Option<domain_availability_check_result::DomainType>,
}
impl DomainAvailabilityCheckResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod domain_availability_check_result {
    use super::*;
    #[doc = "Valid values are Regular domain: Azure will charge the full price of domain registration, SoftDeleted: Purchasing this domain will simply restore it and this operation will not cost anything."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DomainType {
        Regular,
        SoftDeleted,
    }
}
#[doc = "Collection of domains."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Domain>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DomainCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DomainCollection {
    pub fn new(value: Vec<Domain>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Single sign-on request information for domain management."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainControlCenterSsoRequest {
    #[doc = "URL where the single sign-on request is to be made."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Post parameter key."]
    #[serde(rename = "postParameterKey", default, skip_serializing_if = "Option::is_none")]
    pub post_parameter_key: Option<String>,
    #[doc = "Post parameter value. Client should use 'application/x-www-form-urlencoded' encoding for this value."]
    #[serde(rename = "postParameterValue", default, skip_serializing_if = "Option::is_none")]
    pub post_parameter_value: Option<String>,
}
impl DomainControlCenterSsoRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Domain ownership Identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainOwnershipIdentifier {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "DomainOwnershipIdentifier resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<domain_ownership_identifier::Properties>,
}
impl DomainOwnershipIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod domain_ownership_identifier {
    use super::*;
    #[doc = "DomainOwnershipIdentifier resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Ownership Id."]
        #[serde(rename = "ownershipId", default, skip_serializing_if = "Option::is_none")]
        pub ownership_id: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of domain ownership identifiers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainOwnershipIdentifierCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<DomainOwnershipIdentifier>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DomainOwnershipIdentifierCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DomainOwnershipIdentifierCollection {
    pub fn new(value: Vec<DomainOwnershipIdentifier>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "ARM resource for a domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainPatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "DomainPatchResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<domain_patch_resource::Properties>,
}
impl DomainPatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod domain_patch_resource {
    use super::*;
    #[doc = "DomainPatchResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Contact information for domain registration. If 'Domain Privacy' option is not selected then the contact information is made publicly available through the Whois \ndirectories as per ICANN requirements."]
        #[serde(rename = "contactAdmin")]
        pub contact_admin: Contact,
        #[doc = "Contact information for domain registration. If 'Domain Privacy' option is not selected then the contact information is made publicly available through the Whois \ndirectories as per ICANN requirements."]
        #[serde(rename = "contactBilling")]
        pub contact_billing: Contact,
        #[doc = "Contact information for domain registration. If 'Domain Privacy' option is not selected then the contact information is made publicly available through the Whois \ndirectories as per ICANN requirements."]
        #[serde(rename = "contactRegistrant")]
        pub contact_registrant: Contact,
        #[doc = "Contact information for domain registration. If 'Domain Privacy' option is not selected then the contact information is made publicly available through the Whois \ndirectories as per ICANN requirements."]
        #[serde(rename = "contactTech")]
        pub contact_tech: Contact,
        #[doc = "Domain registration status."]
        #[serde(rename = "registrationStatus", default, skip_serializing_if = "Option::is_none")]
        pub registration_status: Option<properties::RegistrationStatus>,
        #[doc = "Domain provisioning state."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Name servers."]
        #[serde(rename = "nameServers", default, skip_serializing_if = "Vec::is_empty")]
        pub name_servers: Vec<String>,
        #[doc = "<code>true</code> if domain privacy is enabled for this domain; otherwise, <code>false</code>."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub privacy: Option<bool>,
        #[doc = "Domain creation timestamp."]
        #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
        pub created_time: Option<time::OffsetDateTime>,
        #[doc = "Domain expiration timestamp."]
        #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
        pub expiration_time: Option<time::OffsetDateTime>,
        #[doc = "Timestamp when the domain was renewed last time."]
        #[serde(rename = "lastRenewedTime", default, with = "azure_core::date::rfc3339::option")]
        pub last_renewed_time: Option<time::OffsetDateTime>,
        #[doc = "<code>true</code> if the domain should be automatically renewed; otherwise, <code>false</code>."]
        #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
        pub auto_renew: Option<bool>,
        #[doc = "<code>true</code> if Azure can assign this domain to App Service apps; otherwise, <code>false</code>. This value will be <code>true</code> if domain registration status is active and \n it is hosted on name servers Azure has programmatic access to."]
        #[serde(rename = "readyForDnsRecordManagement", default, skip_serializing_if = "Option::is_none")]
        pub ready_for_dns_record_management: Option<bool>,
        #[doc = "All hostnames derived from the domain and assigned to Azure resources."]
        #[serde(rename = "managedHostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub managed_host_names: Vec<HostName>,
        #[doc = "Domain purchase consent object, representing acceptance of applicable legal agreements."]
        pub consent: DomainPurchaseConsent,
        #[doc = "Reasons why domain is not renewable."]
        #[serde(rename = "domainNotRenewableReasons", default, skip_serializing_if = "Vec::is_empty")]
        pub domain_not_renewable_reasons: Vec<String>,
        #[doc = "Current DNS type"]
        #[serde(rename = "dnsType", default, skip_serializing_if = "Option::is_none")]
        pub dns_type: Option<properties::DnsType>,
        #[doc = "Azure DNS Zone to use"]
        #[serde(rename = "dnsZoneId", default, skip_serializing_if = "Option::is_none")]
        pub dns_zone_id: Option<String>,
        #[doc = "Target DNS type (would be used for migration)"]
        #[serde(rename = "targetDnsType", default, skip_serializing_if = "Option::is_none")]
        pub target_dns_type: Option<properties::TargetDnsType>,
        #[serde(rename = "authCode", default, skip_serializing_if = "Option::is_none")]
        pub auth_code: Option<String>,
    }
    impl Properties {
        pub fn new(
            contact_admin: Contact,
            contact_billing: Contact,
            contact_registrant: Contact,
            contact_tech: Contact,
            consent: DomainPurchaseConsent,
        ) -> Self {
            Self {
                contact_admin,
                contact_billing,
                contact_registrant,
                contact_tech,
                registration_status: None,
                provisioning_state: None,
                name_servers: Vec::new(),
                privacy: None,
                created_time: None,
                expiration_time: None,
                last_renewed_time: None,
                auto_renew: None,
                ready_for_dns_record_management: None,
                managed_host_names: Vec::new(),
                consent,
                domain_not_renewable_reasons: Vec::new(),
                dns_type: None,
                dns_zone_id: None,
                target_dns_type: None,
                auth_code: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Domain registration status."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum RegistrationStatus {
            Active,
            Awaiting,
            Cancelled,
            Confiscated,
            Disabled,
            Excluded,
            Expired,
            Failed,
            Held,
            Locked,
            Parked,
            Pending,
            Reserved,
            Reverted,
            Suspended,
            Transferred,
            Unknown,
            Unlocked,
            Unparked,
            Updated,
            JsonConverterFailed,
        }
        #[doc = "Domain provisioning state."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            InProgress,
            Deleting,
        }
        #[doc = "Current DNS type"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum DnsType {
            AzureDns,
            DefaultDomainRegistrarDns,
        }
        #[doc = "Target DNS type (would be used for migration)"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum TargetDnsType {
            AzureDns,
            DefaultDomainRegistrarDns,
        }
    }
}
#[doc = "Domain purchase consent object, representing acceptance of applicable legal agreements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainPurchaseConsent {
    #[doc = "List of applicable legal agreement keys. This list can be retrieved using ListLegalAgreements API under <code>TopLevelDomain</code> resource."]
    #[serde(rename = "agreementKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub agreement_keys: Vec<String>,
    #[doc = "Client IP address."]
    #[serde(rename = "agreedBy", default, skip_serializing_if = "Option::is_none")]
    pub agreed_by: Option<String>,
    #[doc = "Timestamp when the agreements were accepted."]
    #[serde(rename = "agreedAt", default, with = "azure_core::date::rfc3339::option")]
    pub agreed_at: Option<time::OffsetDateTime>,
}
impl DomainPurchaseConsent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Domain recommendation search parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainRecommendationSearchParameters {
    #[doc = "Keywords to be used for generating domain recommendations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[doc = "Maximum number of recommendations."]
    #[serde(rename = "maxDomainRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub max_domain_recommendations: Option<i32>,
}
impl DomainRecommendationSearchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enabled configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnabledConfig {
    #[doc = "True if configuration is enabled, false if it is disabled and null if configuration is not set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl EnabledConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A domain name that a service is reached at, including details of the current connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDependency {
    #[doc = "The domain name of the dependency."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The IP Addresses and Ports used when connecting to DomainName."]
    #[serde(rename = "endpointDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_details: Vec<EndpointDetail>,
}
impl EndpointDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current TCP connectivity information from the App Service Environment to a single endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDetail {
    #[doc = "An IP Address that Domain Name currently resolves to."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The port an endpoint is connected to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The time in milliseconds it takes for a TCP connection to be created from the App Service Environment to this IpAddress at this Port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency: Option<f64>,
    #[doc = "Whether it is possible to create a TCP connection from the App Service Environment to this IpAddress at this Port."]
    #[serde(rename = "isAccessible", default, skip_serializing_if = "Option::is_none")]
    pub is_accessible: Option<bool>,
}
impl EndpointDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Body of the error response returned from the API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorEntity {
    #[doc = "Type of error."]
    #[serde(rename = "extendedCode", default, skip_serializing_if = "Option::is_none")]
    pub extended_code: Option<String>,
    #[doc = "Message template."]
    #[serde(rename = "messageTemplate", default, skip_serializing_if = "Option::is_none")]
    pub message_template: Option<String>,
    #[doc = "Parameters for the template."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,
    #[doc = "Inner errors."]
    #[serde(rename = "innerErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub inner_errors: Vec<ErrorEntity>,
    #[doc = "Error Details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorEntity>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Basic error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Any details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Routing rules in production experiments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Experiments {
    #[doc = "List of ramp-up rules."]
    #[serde(rename = "rampUpRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ramp_up_rules: Vec<RampUpRule>,
}
impl Experiments {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extended Location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "Name of extended location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of extended location."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the Facebook provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Facebook {
    #[doc = "<code>false</code> if the Facebook provider should not be enabled despite the set registration; otherwise, <code>true</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The configuration settings of the app registration for providers that have app ids and app secrets"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<AppRegistration>,
    #[doc = "The version of the Facebook api to be used while logging in."]
    #[serde(rename = "graphApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub graph_api_version: Option<String>,
    #[doc = "The configuration settings of the login flow, including the scopes that should be requested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<LoginScopes>,
}
impl Facebook {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application logs to file system configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileSystemApplicationLogsConfig {
    #[doc = "Log level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<file_system_application_logs_config::Level>,
}
impl FileSystemApplicationLogsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod file_system_application_logs_config {
    use super::*;
    #[doc = "Log level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        Off,
        Verbose,
        Information,
        Warning,
        Error,
    }
    impl Default for Level {
        fn default() -> Self {
            Self::Off
        }
    }
}
#[doc = "Http logs to file system configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileSystemHttpLogsConfig {
    #[doc = "Maximum size in megabytes that http log files can use.\nWhen reached old log files will be removed to make space for new ones.\nValue can range between 25 and 100."]
    #[serde(rename = "retentionInMb", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_mb: Option<i32>,
    #[doc = "Retention in days.\nRemove files older than X days.\n0 or lower means no retention."]
    #[serde(rename = "retentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
    #[doc = "True if configuration is enabled, false if it is disabled and null if configuration is not set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl FileSystemHttpLogsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the storage of the tokens if a file system is used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileSystemTokenStore {
    #[doc = "The directory in which the tokens will be stored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
}
impl FileSystemTokenStore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of a forward proxy used to make the requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForwardProxy {
    #[doc = "The convention used to determine the url of the request made."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convention: Option<forward_proxy::Convention>,
    #[doc = "The name of the header containing the host of the request."]
    #[serde(rename = "customHostHeaderName", default, skip_serializing_if = "Option::is_none")]
    pub custom_host_header_name: Option<String>,
    #[doc = "The name of the header containing the scheme of the request."]
    #[serde(rename = "customProtoHeaderName", default, skip_serializing_if = "Option::is_none")]
    pub custom_proto_header_name: Option<String>,
}
impl ForwardProxy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod forward_proxy {
    use super::*;
    #[doc = "The convention used to determine the url of the request made."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Convention {
        NoProxy,
        Standard,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontEndConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<front_end_configuration::Kind>,
}
impl FrontEndConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod front_end_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        NodePort,
        LoadBalancer,
    }
}
#[doc = "Function App stack major version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionAppMajorVersion {
    #[doc = "Function App stack major version (display only)."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "Function App stack major version name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Minor versions associated with the major version."]
    #[serde(rename = "minorVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub minor_versions: Vec<FunctionAppMinorVersion>,
}
impl FunctionAppMajorVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Function App stack minor version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionAppMinorVersion {
    #[doc = "Function App stack (display only)."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "Function App stack name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Function App stack runtimes."]
    #[serde(rename = "stackSettings", default, skip_serializing_if = "Option::is_none")]
    pub stack_settings: Option<FunctionAppRuntimes>,
}
impl FunctionAppMinorVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Function App runtime settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionAppRuntimeSettings {
    #[doc = "Function App stack minor version (runtime only)."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[doc = "<code>true</code> if remote debugging is supported for the stack; otherwise, <code>false</code>."]
    #[serde(rename = "remoteDebuggingSupported", default, skip_serializing_if = "Option::is_none")]
    pub remote_debugging_supported: Option<bool>,
    #[doc = "App Insights Web App stack settings."]
    #[serde(rename = "appInsightsSettings", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_settings: Option<AppInsightsWebAppStackSettings>,
    #[doc = "GitHub Actions Web App stack settings."]
    #[serde(rename = "gitHubActionSettings", default, skip_serializing_if = "Option::is_none")]
    pub git_hub_action_settings: Option<GitHubActionWebAppStackSettings>,
    #[doc = "Application settings associated with the minor version."]
    #[serde(rename = "appSettingsDictionary", default, skip_serializing_if = "Option::is_none")]
    pub app_settings_dictionary: Option<serde_json::Value>,
    #[doc = "Site config properties dictionary."]
    #[serde(rename = "siteConfigPropertiesDictionary", default, skip_serializing_if = "Option::is_none")]
    pub site_config_properties_dictionary: Option<SiteConfigPropertiesDictionary>,
    #[doc = "List of supported Functions extension versions."]
    #[serde(rename = "supportedFunctionsExtensionVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_functions_extension_versions: Vec<String>,
    #[doc = "<code>true</code> if the stack is in preview; otherwise, <code>false</code>."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[doc = "<code>true</code> if the stack is deprecated; otherwise, <code>false</code>."]
    #[serde(rename = "isDeprecated", default, skip_serializing_if = "Option::is_none")]
    pub is_deprecated: Option<bool>,
    #[doc = "<code>true</code> if the stack should be hidden; otherwise, <code>false</code>."]
    #[serde(rename = "isHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "End-of-life date for the minor version."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "<code>true</code> if the stack version is auto-updated; otherwise, <code>false</code>."]
    #[serde(rename = "isAutoUpdate", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_update: Option<bool>,
    #[doc = "<code>true</code> if the minor version is early-access; otherwise, <code>false</code>."]
    #[serde(rename = "isEarlyAccess", default, skip_serializing_if = "Option::is_none")]
    pub is_early_access: Option<bool>,
    #[doc = "<code>true</code> if the minor version the default; otherwise, <code>false</code>."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}
impl FunctionAppRuntimeSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Function App stack runtimes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionAppRuntimes {
    #[doc = "Function App runtime settings."]
    #[serde(rename = "linuxRuntimeSettings", default, skip_serializing_if = "Option::is_none")]
    pub linux_runtime_settings: Option<FunctionAppRuntimeSettings>,
    #[doc = "Function App runtime settings."]
    #[serde(rename = "windowsRuntimeSettings", default, skip_serializing_if = "Option::is_none")]
    pub windows_runtime_settings: Option<FunctionAppRuntimeSettings>,
}
impl FunctionAppRuntimes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Function App Stack."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionAppStack {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Function App stack location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "FunctionAppStack resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<function_app_stack::Properties>,
}
impl FunctionAppStack {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod function_app_stack {
    use super::*;
    #[doc = "FunctionAppStack resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Function App stack (display only)."]
        #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
        pub display_text: Option<String>,
        #[doc = "Function App stack name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[doc = "List of major versions available."]
        #[serde(rename = "majorVersions", default, skip_serializing_if = "Vec::is_empty")]
        pub major_versions: Vec<FunctionAppMajorVersion>,
        #[doc = "Function App stack preferred OS."]
        #[serde(rename = "preferredOs", default, skip_serializing_if = "Option::is_none")]
        pub preferred_os: Option<properties::PreferredOs>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Function App stack preferred OS."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum PreferredOs {
            Windows,
            Linux,
        }
    }
}
#[doc = "Collection of Function app Stacks"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionAppStackCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<FunctionAppStack>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FunctionAppStackCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FunctionAppStackCollection {
    pub fn new(value: Vec<FunctionAppStack>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Function information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionEnvelope {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "FunctionEnvelope resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<function_envelope::Properties>,
}
impl FunctionEnvelope {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod function_envelope {
    use super::*;
    #[doc = "FunctionEnvelope resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Function App ID."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub function_app_id: Option<String>,
        #[doc = "Script root path URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub script_root_path_href: Option<String>,
        #[doc = "Script URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub script_href: Option<String>,
        #[doc = "Config URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config_href: Option<String>,
        #[doc = "Test data URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub test_data_href: Option<String>,
        #[doc = "Secrets file URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secrets_file_href: Option<String>,
        #[doc = "Function URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub href: Option<String>,
        #[doc = "Config information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config: Option<serde_json::Value>,
        #[doc = "File list."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub files: Option<serde_json::Value>,
        #[doc = "Test data used when testing via the Azure Portal."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub test_data: Option<String>,
        #[doc = "The invocation URL"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub invoke_url_template: Option<String>,
        #[doc = "The function language"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub language: Option<String>,
        #[doc = "Gets or sets a value indicating whether the function is disabled"]
        #[serde(rename = "isDisabled", default, skip_serializing_if = "Option::is_none")]
        pub is_disabled: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Kudu function information elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionEnvelopeCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<FunctionEnvelope>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FunctionEnvelopeCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FunctionEnvelopeCollection {
    pub fn new(value: Vec<FunctionEnvelope>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Function secrets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionSecrets {
    #[doc = "Secret key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Trigger URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_url: Option<String>,
}
impl FunctionSecrets {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Geographical region."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GeoRegion {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "GeoRegion resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<geo_region::Properties>,
}
impl GeoRegion {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod geo_region {
    use super::*;
    #[doc = "GeoRegion resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Region description."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Display name for region."]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Display name for region."]
        #[serde(rename = "orgDomain", default, skip_serializing_if = "Option::is_none")]
        pub org_domain: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of geographical regions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoRegionCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<GeoRegion>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GeoRegionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GeoRegionCollection {
    pub fn new(value: Vec<GeoRegion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The configuration settings of the GitHub provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHub {
    #[doc = "<code>false</code> if the GitHub provider should not be enabled despite the set registration; otherwise, <code>true</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The configuration settings of the app registration for providers that have client ids and client secrets"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<ClientRegistration>,
    #[doc = "The configuration settings of the login flow, including the scopes that should be requested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<LoginScopes>,
}
impl GitHub {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The GitHub action code configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubActionCodeConfiguration {
    #[doc = "Runtime stack is used to determine the workflow file content for code base apps."]
    #[serde(rename = "runtimeStack", default, skip_serializing_if = "Option::is_none")]
    pub runtime_stack: Option<String>,
    #[doc = "Runtime version is used to determine what build version to set in the workflow file."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
}
impl GitHubActionCodeConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The GitHub action configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubActionConfiguration {
    #[doc = "The GitHub action code configuration."]
    #[serde(rename = "codeConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub code_configuration: Option<GitHubActionCodeConfiguration>,
    #[doc = "The GitHub action container configuration."]
    #[serde(rename = "containerConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub container_configuration: Option<GitHubActionContainerConfiguration>,
    #[doc = "This will help determine the workflow configuration to select."]
    #[serde(rename = "isLinux", default, skip_serializing_if = "Option::is_none")]
    pub is_linux: Option<bool>,
    #[doc = "Workflow option to determine whether the workflow file should be generated and written to the repository."]
    #[serde(rename = "generateWorkflowFile", default, skip_serializing_if = "Option::is_none")]
    pub generate_workflow_file: Option<bool>,
}
impl GitHubActionConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The GitHub action container configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubActionContainerConfiguration {
    #[doc = "The server URL for the container registry where the build will be hosted."]
    #[serde(rename = "serverUrl", default, skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    #[doc = "The image name for the build."]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "The username used to upload the image to the container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The password used to upload the image to the container registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl GitHubActionContainerConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitHub Actions Web App stack settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubActionWebAppStackSettings {
    #[doc = "<code>true</code> if GitHub Actions is supported for the stack; otherwise, <code>false</code>."]
    #[serde(rename = "isSupported", default, skip_serializing_if = "Option::is_none")]
    pub is_supported: Option<bool>,
    #[doc = "The minor version that is supported for GitHub Actions."]
    #[serde(rename = "supportedVersion", default, skip_serializing_if = "Option::is_none")]
    pub supported_version: Option<String>,
}
impl GitHubActionWebAppStackSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Global SKU Description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalCsmSkuDescription {
    #[doc = "Name of the resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Service Tier of the resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Size specifier of the resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "Family code of the resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "Description of the App Service plan scale options."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
    #[doc = "Locations of the SKU."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Capabilities of the SKU, e.g., is traffic manager enabled?"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
}
impl GlobalCsmSkuDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings that determines the validation flow of users using App Service Authentication/Authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalValidation {
    #[doc = "<code>true</code> if the authentication flow is required any request is made; otherwise, <code>false</code>."]
    #[serde(rename = "requireAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub require_authentication: Option<bool>,
    #[doc = "The action to take when an unauthenticated client attempts to access the app."]
    #[serde(rename = "unauthenticatedClientAction", default, skip_serializing_if = "Option::is_none")]
    pub unauthenticated_client_action: Option<global_validation::UnauthenticatedClientAction>,
    #[doc = "The default authentication provider to use when multiple providers are configured.\nThis setting is only needed if multiple providers are configured and the unauthenticated client\naction is set to \"RedirectToLoginPage\"."]
    #[serde(rename = "redirectToProvider", default, skip_serializing_if = "Option::is_none")]
    pub redirect_to_provider: Option<String>,
    #[doc = "The paths for which unauthenticated flow would not be redirected to the login page."]
    #[serde(rename = "excludedPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_paths: Vec<String>,
}
impl GlobalValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod global_validation {
    use super::*;
    #[doc = "The action to take when an unauthenticated client attempts to access the app."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UnauthenticatedClientAction {
        RedirectToLoginPage,
        AllowAnonymous,
        Return401,
        Return403,
    }
}
#[doc = "The configuration settings of the Google provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Google {
    #[doc = "<code>false</code> if the Google provider should not be enabled despite the set registration; otherwise, <code>true</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The configuration settings of the app registration for providers that have client ids and client secrets"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<ClientRegistration>,
    #[doc = "The configuration settings of the login flow, including the scopes that should be requested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<LoginScopes>,
    #[doc = "The configuration settings of the Allowed Audiences validation flow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<AllowedAudiencesValidation>,
}
impl Google {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The IIS handler mappings used to define which handler processes HTTP requests with certain extension. \nFor example, it is used to configure php-cgi.exe process to handle all HTTP requests with *.php extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HandlerMapping {
    #[doc = "Requests with this extension will be handled using the specified FastCGI application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[doc = "The absolute path to the FastCGI application."]
    #[serde(rename = "scriptProcessor", default, skip_serializing_if = "Option::is_none")]
    pub script_processor: Option<String>,
    #[doc = "Command-line arguments to be passed to the script processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}
impl HandlerMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Functions host level keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostKeys {
    #[doc = "Secret key."]
    #[serde(rename = "masterKey", default, skip_serializing_if = "Option::is_none")]
    pub master_key: Option<String>,
    #[doc = "Host level function keys."]
    #[serde(rename = "functionKeys", default, skip_serializing_if = "Option::is_none")]
    pub function_keys: Option<serde_json::Value>,
    #[doc = "System keys."]
    #[serde(rename = "systemKeys", default, skip_serializing_if = "Option::is_none")]
    pub system_keys: Option<serde_json::Value>,
}
impl HostKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a hostname derived from a domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostName {
    #[doc = "Name of the hostname."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of apps the hostname is assigned to. This list will have more than one app only if the hostname is pointing to a Traffic Manager."]
    #[serde(rename = "siteNames", default, skip_serializing_if = "Vec::is_empty")]
    pub site_names: Vec<String>,
    #[doc = "Name of the Azure resource the hostname is assigned to. If it is assigned to a Traffic Manager then it will be the Traffic Manager name otherwise it will be the app name."]
    #[serde(rename = "azureResourceName", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_name: Option<String>,
    #[doc = "Type of the Azure resource the hostname is assigned to."]
    #[serde(rename = "azureResourceType", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_type: Option<host_name::AzureResourceType>,
    #[doc = "Type of the DNS record."]
    #[serde(rename = "customHostNameDnsRecordType", default, skip_serializing_if = "Option::is_none")]
    pub custom_host_name_dns_record_type: Option<host_name::CustomHostNameDnsRecordType>,
    #[doc = "Type of the hostname."]
    #[serde(rename = "hostNameType", default, skip_serializing_if = "Option::is_none")]
    pub host_name_type: Option<host_name::HostNameType>,
}
impl HostName {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod host_name {
    use super::*;
    #[doc = "Type of the Azure resource the hostname is assigned to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AzureResourceType {
        Website,
        TrafficManager,
    }
    #[doc = "Type of the DNS record."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomHostNameDnsRecordType {
        CName,
        A,
    }
    #[doc = "Type of the hostname."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HostNameType {
        Verified,
        Managed,
    }
}
#[doc = "A hostname binding object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostNameBinding {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "HostNameBinding resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<host_name_binding::Properties>,
}
impl HostNameBinding {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod host_name_binding {
    use super::*;
    #[doc = "HostNameBinding resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "App Service app name."]
        #[serde(rename = "siteName", default, skip_serializing_if = "Option::is_none")]
        pub site_name: Option<String>,
        #[doc = "Fully qualified ARM domain resource URI."]
        #[serde(rename = "domainId", default, skip_serializing_if = "Option::is_none")]
        pub domain_id: Option<String>,
        #[doc = "Azure resource name."]
        #[serde(rename = "azureResourceName", default, skip_serializing_if = "Option::is_none")]
        pub azure_resource_name: Option<String>,
        #[doc = "Azure resource type."]
        #[serde(rename = "azureResourceType", default, skip_serializing_if = "Option::is_none")]
        pub azure_resource_type: Option<properties::AzureResourceType>,
        #[doc = "Custom DNS record type."]
        #[serde(rename = "customHostNameDnsRecordType", default, skip_serializing_if = "Option::is_none")]
        pub custom_host_name_dns_record_type: Option<properties::CustomHostNameDnsRecordType>,
        #[doc = "Hostname type."]
        #[serde(rename = "hostNameType", default, skip_serializing_if = "Option::is_none")]
        pub host_name_type: Option<properties::HostNameType>,
        #[doc = "SSL type"]
        #[serde(rename = "sslState", default, skip_serializing_if = "Option::is_none")]
        pub ssl_state: Option<properties::SslState>,
        #[doc = "SSL certificate thumbprint"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub thumbprint: Option<String>,
        #[doc = "Virtual IP address assigned to the hostname if IP based SSL is enabled."]
        #[serde(rename = "virtualIP", default, skip_serializing_if = "Option::is_none")]
        pub virtual_ip: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Azure resource type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum AzureResourceType {
            Website,
            TrafficManager,
        }
        #[doc = "Custom DNS record type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum CustomHostNameDnsRecordType {
            CName,
            A,
        }
        #[doc = "Hostname type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum HostNameType {
            Verified,
            Managed,
        }
        #[doc = "SSL type"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum SslState {
            Disabled,
            SniEnabled,
            IpBasedEnabled,
        }
    }
}
#[doc = "Collection of hostname bindings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostNameBindingCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<HostNameBinding>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HostNameBindingCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HostNameBindingCollection {
    pub fn new(value: Vec<HostNameBinding>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "SSL-enabled hostname."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostNameSslState {
    #[doc = "Hostname."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "SSL type."]
    #[serde(rename = "sslState", default, skip_serializing_if = "Option::is_none")]
    pub ssl_state: Option<host_name_ssl_state::SslState>,
    #[doc = "Virtual IP address assigned to the hostname if IP based SSL is enabled."]
    #[serde(rename = "virtualIP", default, skip_serializing_if = "Option::is_none")]
    pub virtual_ip: Option<String>,
    #[doc = "SSL certificate thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Set to <code>true</code> to update existing hostname."]
    #[serde(rename = "toUpdate", default, skip_serializing_if = "Option::is_none")]
    pub to_update: Option<bool>,
    #[doc = "Indicates whether the hostname is a standard or repository hostname."]
    #[serde(rename = "hostType", default, skip_serializing_if = "Option::is_none")]
    pub host_type: Option<host_name_ssl_state::HostType>,
}
impl HostNameSslState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod host_name_ssl_state {
    use super::*;
    #[doc = "SSL type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SslState {
        Disabled,
        SniEnabled,
        IpBasedEnabled,
    }
    #[doc = "Indicates whether the hostname is a standard or repository hostname."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HostType {
        Standard,
        Repository,
    }
}
#[doc = "Information needed to create resources on an App Service Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostingEnvironmentDeploymentInfo {
    #[doc = "Name of the App Service Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Location of the App Service Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl HostingEnvironmentDeploymentInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostics for an App Service Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostingEnvironmentDiagnostics {
    #[doc = "Name/identifier of the diagnostics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Diagnostics output."]
    #[serde(rename = "diagnosticsOutput", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_output: Option<String>,
}
impl HostingEnvironmentDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specification for an App Service Environment to use for this resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostingEnvironmentProfile {
    #[doc = "Resource ID of the App Service Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the App Service Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type of the App Service Environment."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl HostingEnvironmentProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Http logs configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpLogsConfig {
    #[doc = "Http logs to file system configuration."]
    #[serde(rename = "fileSystem", default, skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystemHttpLogsConfig>,
    #[doc = "Http logs to azure blob storage configuration."]
    #[serde(rename = "azureBlobStorage", default, skip_serializing_if = "Option::is_none")]
    pub azure_blob_storage: Option<AzureBlobStorageHttpLogsConfig>,
}
impl HttpLogsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the HTTP requests for authentication and authorization requests made against App Service Authentication/Authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpSettings {
    #[doc = "<code>false</code> if the authentication/authorization responses not having the HTTPS scheme are permissible; otherwise, <code>true</code>."]
    #[serde(rename = "requireHttps", default, skip_serializing_if = "Option::is_none")]
    pub require_https: Option<bool>,
    #[doc = "The configuration settings of the paths HTTP requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<HttpSettingsRoutes>,
    #[doc = "The configuration settings of a forward proxy used to make the requests."]
    #[serde(rename = "forwardProxy", default, skip_serializing_if = "Option::is_none")]
    pub forward_proxy: Option<ForwardProxy>,
}
impl HttpSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the paths HTTP requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpSettingsRoutes {
    #[doc = "The prefix that should precede all the authentication/authorization paths."]
    #[serde(rename = "apiPrefix", default, skip_serializing_if = "Option::is_none")]
    pub api_prefix: Option<String>,
}
impl HttpSettingsRoutes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Hybrid Connection contract. This is used to configure a Hybrid Connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnection {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "HybridConnection resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<hybrid_connection::Properties>,
}
impl HybridConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hybrid_connection {
    use super::*;
    #[doc = "HybridConnection resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The name of the Service Bus namespace."]
        #[serde(rename = "serviceBusNamespace", default, skip_serializing_if = "Option::is_none")]
        pub service_bus_namespace: Option<String>,
        #[doc = "The name of the Service Bus relay."]
        #[serde(rename = "relayName", default, skip_serializing_if = "Option::is_none")]
        pub relay_name: Option<String>,
        #[doc = "The ARM URI to the Service Bus relay."]
        #[serde(rename = "relayArmUri", default, skip_serializing_if = "Option::is_none")]
        pub relay_arm_uri: Option<String>,
        #[doc = "The hostname of the endpoint."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hostname: Option<String>,
        #[doc = "The port of the endpoint."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<i32>,
        #[doc = "The name of the Service Bus key which has Send permissions. This is used to authenticate to Service Bus."]
        #[serde(rename = "sendKeyName", default, skip_serializing_if = "Option::is_none")]
        pub send_key_name: Option<String>,
        #[doc = "The value of the Service Bus key. This is used to authenticate to Service Bus. In ARM this key will not be returned\nnormally, use the POST /listKeys API instead."]
        #[serde(rename = "sendKeyValue", default, skip_serializing_if = "Option::is_none")]
        pub send_key_value: Option<String>,
        #[doc = "The suffix for the service bus endpoint. By default this is .servicebus.windows.net"]
        #[serde(rename = "serviceBusSuffix", default, skip_serializing_if = "Option::is_none")]
        pub service_bus_suffix: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of hostname bindings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridConnectionCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<HybridConnection>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HybridConnectionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HybridConnectionCollection {
    pub fn new(value: Vec<HybridConnection>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Hybrid Connection key contract. This has the send key name and value for a Hybrid Connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnectionKey {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "HybridConnectionKey resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<hybrid_connection_key::Properties>,
}
impl HybridConnectionKey {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hybrid_connection_key {
    use super::*;
    #[doc = "HybridConnectionKey resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The name of the send key."]
        #[serde(rename = "sendKeyName", default, skip_serializing_if = "Option::is_none")]
        pub send_key_name: Option<String>,
        #[doc = "The value of the send key."]
        #[serde(rename = "sendKeyValue", default, skip_serializing_if = "Option::is_none")]
        pub send_key_value: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Hybrid Connection limits contract. This is used to return the plan limits of Hybrid Connections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnectionLimits {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "HybridConnectionLimits resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<hybrid_connection_limits::Properties>,
}
impl HybridConnectionLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hybrid_connection_limits {
    use super::*;
    #[doc = "HybridConnectionLimits resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The current number of Hybrid Connections."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub current: Option<i32>,
        #[doc = "The maximum number of Hybrid Connections allowed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maximum: Option<i32>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A domain specific resource identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identifier {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Identifier resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<identifier::Properties>,
}
impl Identifier {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identifier {
    use super::*;
    #[doc = "Identifier resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "String representation of the identity."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of identifiers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentifierCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Identifier>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IdentifierCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IdentifierCollection {
    pub fn new(value: Vec<Identifier>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The configuration settings of each of the identity providers used to configure App Service Authentication/Authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProviders {
    #[doc = "The configuration settings of the Azure Active directory provider."]
    #[serde(rename = "azureActiveDirectory", default, skip_serializing_if = "Option::is_none")]
    pub azure_active_directory: Option<AzureActiveDirectory>,
    #[doc = "The configuration settings of the Facebook provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facebook: Option<Facebook>,
    #[doc = "The configuration settings of the GitHub provider."]
    #[serde(rename = "gitHub", default, skip_serializing_if = "Option::is_none")]
    pub git_hub: Option<GitHub>,
    #[doc = "The configuration settings of the Google provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google: Option<Google>,
    #[doc = "The configuration settings of the legacy Microsoft Account provider."]
    #[serde(rename = "legacyMicrosoftAccount", default, skip_serializing_if = "Option::is_none")]
    pub legacy_microsoft_account: Option<LegacyMicrosoftAccount>,
    #[doc = "The configuration settings of the Twitter provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter: Option<Twitter>,
    #[doc = "The configuration settings of the Apple provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apple: Option<Apple>,
    #[doc = "The configuration settings of the Azure Static Web Apps provider."]
    #[serde(rename = "azureStaticWebApps", default, skip_serializing_if = "Option::is_none")]
    pub azure_static_web_apps: Option<AzureStaticWebApps>,
    #[doc = "The map of the name of the alias of each custom Open ID Connect provider to the\nconfiguration settings of the custom Open ID Connect provider."]
    #[serde(rename = "customOpenIdConnectProviders", default, skip_serializing_if = "Option::is_none")]
    pub custom_open_id_connect_providers: Option<serde_json::Value>,
}
impl IdentityProviders {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The IP Addresses and Ports that require inbound network access to and within the subnet of the App Service Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundEnvironmentEndpoint {
    #[doc = "Short text describing the purpose of the network traffic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The IP addresses that network traffic will originate from in cidr notation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<String>,
    #[doc = "The ports that network traffic will arrive to the App Service Environment at."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<String>,
}
impl InboundEnvironmentEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Inbound Environment Endpoints"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InboundEnvironmentEndpointCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<InboundEnvironmentEndpoint>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InboundEnvironmentEndpointCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InboundEnvironmentEndpointCollection {
    pub fn new(value: Vec<InboundEnvironmentEndpoint>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "IP security restriction on an app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpSecurityRestriction {
    #[doc = "IP address the security restriction is valid for.\nIt can be in form of pure ipv4 address (required SubnetMask property) or\nCIDR notation such as ipv4/mask (leading bit match). For CIDR,\nSubnetMask property must not be specified."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Subnet mask for the range of IP addresses the restriction is valid for."]
    #[serde(rename = "subnetMask", default, skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
    #[doc = "Virtual network resource id"]
    #[serde(rename = "vnetSubnetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_resource_id: Option<String>,
    #[doc = "(internal) Vnet traffic tag"]
    #[serde(rename = "vnetTrafficTag", default, skip_serializing_if = "Option::is_none")]
    pub vnet_traffic_tag: Option<i32>,
    #[doc = "(internal) Subnet traffic tag"]
    #[serde(rename = "subnetTrafficTag", default, skip_serializing_if = "Option::is_none")]
    pub subnet_traffic_tag: Option<i32>,
    #[doc = "Allow or Deny access for this IP range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "Defines what this IP filter will be used for. This is to support IP filtering on proxies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<ip_security_restriction::Tag>,
    #[doc = "Priority of IP restriction rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "IP restriction rule name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "IP restriction rule description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "IP restriction rule headers.\nX-Forwarded-Host (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-Host#Examples). \nThe matching logic is ..\n- If the property is null or empty (default), all hosts(or lack of) are allowed.\n- A value is compared using ordinal-ignore-case (excluding port number).\n- Subdomain wildcards are permitted but don't match the root domain. For example, *.contoso.com matches the subdomain foo.contoso.com\n but not the root domain contoso.com or multi-level foo.bar.contoso.com\n- Unicode host names are allowed but are converted to Punycode for matching.\n\nX-Forwarded-For (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For#Examples).\nThe matching logic is ..\n- If the property is null or empty (default), any forwarded-for chains (or lack of) are allowed.\n- If any address (excluding port number) in the chain (comma separated) matches the CIDR defined by the property.\n\nX-Azure-FDID and X-FD-HealthProbe.\nThe matching logic is exact match."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}
impl IpSecurityRestriction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ip_security_restriction {
    use super::*;
    #[doc = "Defines what this IP filter will be used for. This is to support IP filtering on proxies."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tag")]
    pub enum Tag {
        Default,
        XffProxy,
        ServiceTag,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tag {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tag {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tag {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("Tag", 0u32, "Default"),
                Self::XffProxy => serializer.serialize_unit_variant("Tag", 1u32, "XffProxy"),
                Self::ServiceTag => serializer.serialize_unit_variant("Tag", 2u32, "ServiceTag"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The configuration settings of the checks that should be made while validating the JWT Claims."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JwtClaimChecks {
    #[doc = "The list of the allowed groups."]
    #[serde(rename = "allowedGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_groups: Vec<String>,
    #[doc = "The list of the allowed client applications."]
    #[serde(rename = "allowedClientApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_client_applications: Vec<String>,
}
impl JwtClaimChecks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Function key info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyInfo {
    #[doc = "Key name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Key value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl KeyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyValuePairStringObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl KeyValuePairStringObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Kubernetes cluster specialized for web workloads by Azure App Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeEnvironment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "KubeEnvironment resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<kube_environment::Properties>,
    #[doc = "Extended Location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl KubeEnvironment {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            extended_location: None,
        }
    }
}
pub mod kube_environment {
    use super::*;
    #[doc = "KubeEnvironment resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the Kubernetes Environment."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Any errors that occurred during deployment or deployment validation"]
        #[serde(rename = "deploymentErrors", default, skip_serializing_if = "Option::is_none")]
        pub deployment_errors: Option<String>,
        #[doc = "Only visible within Vnet/Subnet"]
        #[serde(rename = "internalLoadBalancerEnabled", default, skip_serializing_if = "Option::is_none")]
        pub internal_load_balancer_enabled: Option<bool>,
        #[doc = "Default Domain Name for the cluster"]
        #[serde(rename = "defaultDomain", default, skip_serializing_if = "Option::is_none")]
        pub default_domain: Option<String>,
        #[doc = "Static IP of the KubeEnvironment"]
        #[serde(rename = "staticIp", default, skip_serializing_if = "Option::is_none")]
        pub static_ip: Option<String>,
        #[serde(rename = "arcConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub arc_configuration: Option<ArcConfiguration>,
        #[serde(rename = "appLogsConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub app_logs_configuration: Option<AppLogsConfiguration>,
        #[serde(rename = "aksResourceID", default, skip_serializing_if = "Option::is_none")]
        pub aks_resource_id: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Provisioning state of the Kubernetes Environment."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            Waiting,
            InitializationInProgress,
            InfrastructureSetupInProgress,
            InfrastructureSetupComplete,
            ScheduledForDelete,
            UpgradeRequested,
            UpgradeFailed,
        }
    }
}
#[doc = "Collection of Kubernetes Environments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeEnvironmentCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<KubeEnvironment>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KubeEnvironmentCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl KubeEnvironmentCollection {
    pub fn new(value: Vec<KubeEnvironment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "ARM resource for a KubeEnvironment when patching"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubeEnvironmentPatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "KubeEnvironmentPatchResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<kube_environment_patch_resource::Properties>,
}
impl KubeEnvironmentPatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod kube_environment_patch_resource {
    use super::*;
    #[doc = "KubeEnvironmentPatchResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the Kubernetes Environment."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Any errors that occurred during deployment or deployment validation"]
        #[serde(rename = "deploymentErrors", default, skip_serializing_if = "Option::is_none")]
        pub deployment_errors: Option<String>,
        #[doc = "Only visible within Vnet/Subnet"]
        #[serde(rename = "internalLoadBalancerEnabled", default, skip_serializing_if = "Option::is_none")]
        pub internal_load_balancer_enabled: Option<bool>,
        #[doc = "Default Domain Name for the cluster"]
        #[serde(rename = "defaultDomain", default, skip_serializing_if = "Option::is_none")]
        pub default_domain: Option<String>,
        #[doc = "Static IP of the KubeEnvironment"]
        #[serde(rename = "staticIp", default, skip_serializing_if = "Option::is_none")]
        pub static_ip: Option<String>,
        #[serde(rename = "arcConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub arc_configuration: Option<ArcConfiguration>,
        #[serde(rename = "appLogsConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub app_logs_configuration: Option<AppLogsConfiguration>,
        #[serde(rename = "aksResourceID", default, skip_serializing_if = "Option::is_none")]
        pub aks_resource_id: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Provisioning state of the Kubernetes Environment."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            Waiting,
            InitializationInProgress,
            InfrastructureSetupInProgress,
            InfrastructureSetupComplete,
            ScheduledForDelete,
            UpgradeRequested,
            UpgradeFailed,
        }
    }
}
#[doc = "Specification for a Kubernetes Environment to use for this resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubeEnvironmentProfile {
    #[doc = "Resource ID of the Kubernetes Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Kubernetes Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type of the Kubernetes Environment."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl KubeEnvironmentProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the legacy Microsoft Account provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LegacyMicrosoftAccount {
    #[doc = "<code>false</code> if the legacy Microsoft Account provider should not be enabled despite the set registration; otherwise, <code>true</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The configuration settings of the app registration for providers that have client ids and client secrets"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<ClientRegistration>,
    #[doc = "The configuration settings of the login flow, including the scopes that should be requested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<LoginScopes>,
    #[doc = "The configuration settings of the Allowed Audiences validation flow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<AllowedAudiencesValidation>,
}
impl LegacyMicrosoftAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Linux Java Container settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxJavaContainerSettings {
    #[doc = "Java 11 version (runtime only)."]
    #[serde(rename = "java11Runtime", default, skip_serializing_if = "Option::is_none")]
    pub java11_runtime: Option<String>,
    #[doc = "Java 8 version (runtime only)."]
    #[serde(rename = "java8Runtime", default, skip_serializing_if = "Option::is_none")]
    pub java8_runtime: Option<String>,
    #[doc = "<code>true</code> if the stack is in preview; otherwise, <code>false</code>."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[doc = "<code>true</code> if the stack is deprecated; otherwise, <code>false</code>."]
    #[serde(rename = "isDeprecated", default, skip_serializing_if = "Option::is_none")]
    pub is_deprecated: Option<bool>,
    #[doc = "<code>true</code> if the stack should be hidden; otherwise, <code>false</code>."]
    #[serde(rename = "isHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "End-of-life date for the minor version."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "<code>true</code> if the stack version is auto-updated; otherwise, <code>false</code>."]
    #[serde(rename = "isAutoUpdate", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_update: Option<bool>,
    #[doc = "<code>true</code> if the minor version is early-access; otherwise, <code>false</code>."]
    #[serde(rename = "isEarlyAccess", default, skip_serializing_if = "Option::is_none")]
    pub is_early_access: Option<bool>,
}
impl LinuxJavaContainerSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Localizable string object containing the name and a localized value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalizableString {
    #[doc = "Non-localized name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Localized name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl LocalizableString {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsConfiguration {
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "sharedKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_key: Option<String>,
}
impl LogAnalyticsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log Definition of a single resource metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
    #[serde(rename = "logFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub log_filter_pattern: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the login flow of users using App Service Authentication/Authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Login {
    #[doc = "The routes that specify the endpoints used for login and logout requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<LoginRoutes>,
    #[doc = "The configuration settings of the token store."]
    #[serde(rename = "tokenStore", default, skip_serializing_if = "Option::is_none")]
    pub token_store: Option<TokenStore>,
    #[doc = "<code>true</code> if the fragments from the request are preserved after the login request is made; otherwise, <code>false</code>."]
    #[serde(rename = "preserveUrlFragmentsForLogins", default, skip_serializing_if = "Option::is_none")]
    pub preserve_url_fragments_for_logins: Option<bool>,
    #[doc = "External URLs that can be redirected to as part of logging in or logging out of the app. Note that the query string part of the URL is ignored.\nThis is an advanced setting typically only needed by Windows Store application backends.\nNote that URLs within the current domain are always implicitly allowed."]
    #[serde(rename = "allowedExternalRedirectUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_external_redirect_urls: Vec<String>,
    #[doc = "The configuration settings of the session cookie's expiration."]
    #[serde(rename = "cookieExpiration", default, skip_serializing_if = "Option::is_none")]
    pub cookie_expiration: Option<CookieExpiration>,
    #[doc = "The configuration settings of the nonce used in the login flow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Nonce>,
}
impl Login {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The routes that specify the endpoints used for login and logout requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoginRoutes {
    #[doc = "The endpoint at which a logout request should be made."]
    #[serde(rename = "logoutEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub logout_endpoint: Option<String>,
}
impl LoginRoutes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the login flow, including the scopes that should be requested."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoginScopes {
    #[doc = "A list of the scopes that should be requested while authenticating."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}
impl LoginScopes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MSDeploy ARM PUT information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsDeploy {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "MSDeploy ARM PUT core information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MsDeployCore>,
}
impl MsDeploy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MSDeploy ARM PUT core information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsDeployCore {
    #[doc = "Package URI"]
    #[serde(rename = "packageUri", default, skip_serializing_if = "Option::is_none")]
    pub package_uri: Option<String>,
    #[doc = "SQL Connection String"]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "Database Type"]
    #[serde(rename = "dbType", default, skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    #[doc = "URI of MSDeploy Parameters file. Must not be set if SetParameters is used."]
    #[serde(rename = "setParametersXmlFileUri", default, skip_serializing_if = "Option::is_none")]
    pub set_parameters_xml_file_uri: Option<String>,
    #[doc = "MSDeploy Parameters. Must not be set if SetParametersXmlFileUri is used."]
    #[serde(rename = "setParameters", default, skip_serializing_if = "Option::is_none")]
    pub set_parameters: Option<serde_json::Value>,
    #[doc = "Controls whether the MSDeploy operation skips the App_Data directory.\nIf set to <code>true</code>, the existing App_Data directory on the destination\nwill not be deleted, and any App_Data directory in the source will be ignored.\nSetting is <code>false</code> by default."]
    #[serde(rename = "skipAppData", default, skip_serializing_if = "Option::is_none")]
    pub skip_app_data: Option<bool>,
    #[doc = "Sets the AppOffline rule while the MSDeploy operation executes.\nSetting is <code>false</code> by default."]
    #[serde(rename = "appOffline", default, skip_serializing_if = "Option::is_none")]
    pub app_offline: Option<bool>,
}
impl MsDeployCore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MSDeploy log"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsDeployLog {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "MSDeployLog resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ms_deploy_log::Properties>,
}
impl MsDeployLog {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ms_deploy_log {
    use super::*;
    #[doc = "MSDeployLog resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "List of log entry messages"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub entries: Vec<MsDeployLogEntry>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "MSDeploy log entry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsDeployLogEntry {
    #[doc = "Timestamp of log entry"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "Log entry type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ms_deploy_log_entry::Type>,
    #[doc = "Log entry message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl MsDeployLogEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ms_deploy_log_entry {
    use super::*;
    #[doc = "Log entry type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Message,
        Warning,
        Error,
    }
}
#[doc = "MSDeploy ARM response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsDeployStatus {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "MSDeployStatus resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ms_deploy_status::Properties>,
}
impl MsDeployStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ms_deploy_status {
    use super::*;
    #[doc = "MSDeployStatus resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Username of deployer"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deployer: Option<String>,
        #[doc = "Provisioning state"]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Start time of deploy operation"]
        #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
        pub start_time: Option<time::OffsetDateTime>,
        #[doc = "End time of deploy operation"]
        #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
        pub end_time: Option<time::OffsetDateTime>,
        #[doc = "Whether the deployment operation has completed"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub complete: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Provisioning state"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            #[serde(rename = "accepted")]
            Accepted,
            #[serde(rename = "running")]
            Running,
            #[serde(rename = "succeeded")]
            Succeeded,
            #[serde(rename = "failed")]
            Failed,
            #[serde(rename = "canceled")]
            Canceled,
        }
    }
}
#[doc = "Managed service identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServiceIdentity {
    #[doc = "Type of managed service identity."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<managed_service_identity::Type>,
    #[doc = "Tenant of managed service identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Principal Id of managed service identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The list of user assigned identities associated with the resource. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ManagedServiceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_service_identity {
    use super::*;
    #[doc = "Type of managed service identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "Retention policy of a resource metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl MetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of a single resource metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "supportsInstanceLevelAggregation", default, skip_serializing_if = "Option::is_none")]
    pub supports_instance_level_aggregation: Option<bool>,
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<bool>,
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[serde(rename = "isInternal", default, skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<MetricAvailability>,
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MySQL migration request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateMySqlRequest {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "MigrateMySqlRequest resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<migrate_my_sql_request::Properties>,
}
impl MigrateMySqlRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migrate_my_sql_request {
    use super::*;
    #[doc = "MigrateMySqlRequest resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Connection string to the remote MySQL database."]
        #[serde(rename = "connectionString")]
        pub connection_string: String,
        #[doc = "The type of migration operation to be done"]
        #[serde(rename = "migrationType")]
        pub migration_type: properties::MigrationType,
    }
    impl Properties {
        pub fn new(connection_string: String, migration_type: properties::MigrationType) -> Self {
            Self {
                connection_string,
                migration_type,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The type of migration operation to be done"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum MigrationType {
            LocalToRemote,
            RemoteToLocal,
        }
    }
}
#[doc = "MySQL migration status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateMySqlStatus {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "MigrateMySqlStatus resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<migrate_my_sql_status::Properties>,
}
impl MigrateMySqlStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migrate_my_sql_status {
    use super::*;
    #[doc = "MigrateMySqlStatus resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Status of the migration task."]
        #[serde(rename = "migrationOperationStatus", default, skip_serializing_if = "Option::is_none")]
        pub migration_operation_status: Option<properties::MigrationOperationStatus>,
        #[doc = "Operation ID for the migration task."]
        #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
        pub operation_id: Option<String>,
        #[doc = "True if the web app has in app MySql enabled"]
        #[serde(rename = "localMySqlEnabled", default, skip_serializing_if = "Option::is_none")]
        pub local_my_sql_enabled: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Status of the migration task."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum MigrationOperationStatus {
            InProgress,
            Failed,
            Succeeded,
            TimedOut,
            Created,
        }
    }
}
#[doc = "Identifies an object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameIdentifier {
    #[doc = "Name of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl NameIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of domain name identifiers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameIdentifierCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<NameIdentifier>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NameIdentifierCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NameIdentifierCollection {
    pub fn new(value: Vec<NameIdentifier>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Name value pair."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameValuePair {
    #[doc = "Pair name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Pair value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl NameValuePair {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Full view of network features for an app (presently VNET integration and Hybrid Connections)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFeatures {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "NetworkFeatures resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<network_features::Properties>,
}
impl NetworkFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_features {
    use super::*;
    #[doc = "NetworkFeatures resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The Virtual Network name."]
        #[serde(rename = "virtualNetworkName", default, skip_serializing_if = "Option::is_none")]
        pub virtual_network_name: Option<String>,
        #[doc = "Virtual Network information contract."]
        #[serde(rename = "virtualNetworkConnection", default, skip_serializing_if = "Option::is_none")]
        pub virtual_network_connection: Option<VnetInfo>,
        #[doc = "The Hybrid Connections summary view."]
        #[serde(rename = "hybridConnections", default, skip_serializing_if = "Vec::is_empty")]
        pub hybrid_connections: Vec<RelayServiceConnectionEntity>,
        #[doc = "The Hybrid Connection V2 (Service Bus) view."]
        #[serde(rename = "hybridConnectionsV2", default, skip_serializing_if = "Vec::is_empty")]
        pub hybrid_connections_v2: Vec<HybridConnection>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Network trace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkTrace {
    #[doc = "Local file path for the captured network trace file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Current status of the network trace operation, same as Operation.Status (InProgress/Succeeded/Failed)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Detailed message of a network trace operation, e.g. error message in case of failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NetworkTrace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the nonce used in the login flow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Nonce {
    #[doc = "<code>false</code> if the nonce should not be validated while completing the login flow; otherwise, <code>true</code>."]
    #[serde(rename = "validateNonce", default, skip_serializing_if = "Option::is_none")]
    pub validate_nonce: Option<bool>,
    #[doc = "The time after the request is made when the nonce should expire."]
    #[serde(rename = "nonceExpirationInterval", default, skip_serializing_if = "Option::is_none")]
    pub nonce_expiration_interval: Option<String>,
}
impl Nonce {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The authentication client credentials of the custom Open ID Connect provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenIdConnectClientCredential {
    #[doc = "The method that should be used to authenticate the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<open_id_connect_client_credential::Method>,
    #[doc = "The app setting that contains the client secret for the custom Open ID Connect provider."]
    #[serde(rename = "clientSecretSettingName", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_setting_name: Option<String>,
}
impl OpenIdConnectClientCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod open_id_connect_client_credential {
    use super::*;
    #[doc = "The method that should be used to authenticate the user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Method {
        ClientSecretPost,
    }
}
#[doc = "The configuration settings of the endpoints used for the custom Open ID Connect provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenIdConnectConfig {
    #[doc = "The endpoint to be used to make an authorization request."]
    #[serde(rename = "authorizationEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    #[doc = "The endpoint to be used to request a token."]
    #[serde(rename = "tokenEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
    #[doc = "The endpoint that issues the token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "The endpoint that provides the keys necessary to validate the token."]
    #[serde(rename = "certificationUri", default, skip_serializing_if = "Option::is_none")]
    pub certification_uri: Option<String>,
    #[doc = "The endpoint that contains all the configuration endpoints for the provider."]
    #[serde(rename = "wellKnownOpenIdConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub well_known_open_id_configuration: Option<String>,
}
impl OpenIdConnectConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the login flow of the custom Open ID Connect provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenIdConnectLogin {
    #[doc = "The name of the claim that contains the users name."]
    #[serde(rename = "nameClaimType", default, skip_serializing_if = "Option::is_none")]
    pub name_claim_type: Option<String>,
    #[doc = "A list of the scopes that should be requested while authenticating."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}
impl OpenIdConnectLogin {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the app registration for the custom Open ID Connect provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenIdConnectRegistration {
    #[doc = "The client id of the custom Open ID Connect provider."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The authentication client credentials of the custom Open ID Connect provider."]
    #[serde(rename = "clientCredential", default, skip_serializing_if = "Option::is_none")]
    pub client_credential: Option<OpenIdConnectClientCredential>,
    #[doc = "The configuration settings of the endpoints used for the custom Open ID Connect provider."]
    #[serde(rename = "openIdConnectConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub open_id_connect_configuration: Option<OpenIdConnectConfig>,
}
impl OpenIdConnectRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An operation on a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The current status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation::Status>,
    #[doc = "Any errors associate with the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorEntity>,
    #[doc = "Time when operation has started."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Time when operation has been updated."]
    #[serde(rename = "modifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub modified_time: Option<time::OffsetDateTime>,
    #[doc = "Time when operation will expire."]
    #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<time::OffsetDateTime>,
    #[doc = "Applicable only for stamp operation ids."]
    #[serde(rename = "geoMasterOperationId", default, skip_serializing_if = "Option::is_none")]
    pub geo_master_operation_id: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The current status of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        InProgress,
        Failed,
        Succeeded,
        TimedOut,
        Created,
    }
}
#[doc = "Endpoints accessed for a common purpose that the App Service Environment requires outbound network access to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpoint {
    #[doc = "The type of service accessed by the App Service Environment, e.g., Azure Storage, Azure SQL Database, and Azure Active Directory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The endpoints that the App Service Environment reaches the service at."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<EndpointDependency>,
}
impl OutboundEnvironmentEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Outbound Environment Endpoints"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundEnvironmentEndpointCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<OutboundEnvironmentEndpoint>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutboundEnvironmentEndpointCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OutboundEnvironmentEndpointCollection {
    pub fn new(value: Vec<OutboundEnvironmentEndpoint>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Collection of performance monitor counters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerfMonCounterCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<PerfMonResponse>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PerfMonCounterCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PerfMonCounterCollection {
    pub fn new(value: Vec<PerfMonResponse>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Performance monitor API response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerfMonResponse {
    #[doc = "The response code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Metric information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PerfMonSet>,
}
impl PerfMonResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Performance monitor sample in a set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerfMonSample {
    #[doc = "Point in time for which counter was measured."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "Name of the server on which the measurement is made."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "Value of counter at a certain time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl PerfMonSample {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerfMonSet {
    #[doc = "Unique key name of the counter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Start time of the period."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the period."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Presented time grain."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "Collection of workers that are active during this time."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<PerfMonSample>,
}
impl PerfMonSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Premier add-on."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PremierAddOn {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "PremierAddOn resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<premier_add_on::Properties>,
}
impl PremierAddOn {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
pub mod premier_add_on {
    use super::*;
    #[doc = "PremierAddOn resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Premier add on SKU."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sku: Option<String>,
        #[doc = "Premier add on Product."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub product: Option<String>,
        #[doc = "Premier add on Vendor."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vendor: Option<String>,
        #[doc = "Premier add on Marketplace publisher."]
        #[serde(rename = "marketplacePublisher", default, skip_serializing_if = "Option::is_none")]
        pub marketplace_publisher: Option<String>,
        #[doc = "Premier add on Marketplace offer."]
        #[serde(rename = "marketplaceOffer", default, skip_serializing_if = "Option::is_none")]
        pub marketplace_offer: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Premier add-on offer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PremierAddOnOffer {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "PremierAddOnOffer resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<premier_add_on_offer::Properties>,
}
impl PremierAddOnOffer {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod premier_add_on_offer {
    use super::*;
    #[doc = "PremierAddOnOffer resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Premier add on SKU."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sku: Option<String>,
        #[doc = "Premier add on offer Product."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub product: Option<String>,
        #[doc = "Premier add on offer Vendor."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vendor: Option<String>,
        #[doc = "<code>true</code> if promotion code is required; otherwise, <code>false</code>."]
        #[serde(rename = "promoCodeRequired", default, skip_serializing_if = "Option::is_none")]
        pub promo_code_required: Option<bool>,
        #[doc = "Premier add on offer Quota."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub quota: Option<i32>,
        #[doc = "App Service plans this offer is restricted to."]
        #[serde(rename = "webHostingPlanRestrictions", default, skip_serializing_if = "Option::is_none")]
        pub web_hosting_plan_restrictions: Option<properties::WebHostingPlanRestrictions>,
        #[doc = "Privacy policy URL."]
        #[serde(rename = "privacyPolicyUrl", default, skip_serializing_if = "Option::is_none")]
        pub privacy_policy_url: Option<String>,
        #[doc = "Legal terms URL."]
        #[serde(rename = "legalTermsUrl", default, skip_serializing_if = "Option::is_none")]
        pub legal_terms_url: Option<String>,
        #[doc = "Marketplace publisher."]
        #[serde(rename = "marketplacePublisher", default, skip_serializing_if = "Option::is_none")]
        pub marketplace_publisher: Option<String>,
        #[doc = "Marketplace offer."]
        #[serde(rename = "marketplaceOffer", default, skip_serializing_if = "Option::is_none")]
        pub marketplace_offer: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "App Service plans this offer is restricted to."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum WebHostingPlanRestrictions {
            None,
            Free,
            Shared,
            Basic,
            Standard,
            Premium,
        }
    }
}
#[doc = "Collection of premier add-on offers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PremierAddOnOfferCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<PremierAddOnOffer>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PremierAddOnOfferCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PremierAddOnOfferCollection {
    pub fn new(value: Vec<PremierAddOnOffer>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "ARM resource for a PremierAddOn."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PremierAddOnPatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "PremierAddOnPatchResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<premier_add_on_patch_resource::Properties>,
}
impl PremierAddOnPatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod premier_add_on_patch_resource {
    use super::*;
    #[doc = "PremierAddOnPatchResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Premier add on SKU."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sku: Option<String>,
        #[doc = "Premier add on Product."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub product: Option<String>,
        #[doc = "Premier add on Vendor."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vendor: Option<String>,
        #[doc = "Premier add on Marketplace publisher."]
        #[serde(rename = "marketplacePublisher", default, skip_serializing_if = "Option::is_none")]
        pub marketplace_publisher: Option<String>,
        #[doc = "Premier add on Marketplace offer."]
        #[serde(rename = "marketplaceOffer", default, skip_serializing_if = "Option::is_none")]
        pub marketplace_offer: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Description of the parameters of Private Access for a Web Site."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateAccess {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "PrivateAccess resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<private_access::Properties>,
}
impl PrivateAccess {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_access {
    use super::*;
    #[doc = "PrivateAccess resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Whether private access is enabled or not."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The Virtual Networks (and subnets) allowed to access the site privately."]
        #[serde(rename = "virtualNetworks", default, skip_serializing_if = "Vec::is_empty")]
        pub virtual_networks: Vec<PrivateAccessVirtualNetwork>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Description of a Virtual Network subnet that is useable for private site access."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateAccessSubnet {
    #[doc = "The name of the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The key (ID) of the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<i32>,
}
impl PrivateAccessSubnet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a Virtual Network that is useable for private site access."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateAccessVirtualNetwork {
    #[doc = "The name of the Virtual Network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The key (ID) of the Virtual Network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<i32>,
    #[doc = "The ARM uri of the Virtual Network"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "A List of subnets that access is allowed to on this Virtual Network. An empty array (but not null) is interpreted to mean that all subnets are allowed within this Virtual Network."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<PrivateAccessSubnet>,
}
impl PrivateAccessVirtualNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<RemotePrivateEndpointConnectionArmResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionCollection {
    pub fn new(value: Vec<RemotePrivateEndpointConnectionArmResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A request to approve or reject a private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkConnectionApprovalRequest {
    #[doc = "The state of a private link connection"]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkConnectionState>,
}
impl PrivateLinkConnectionApprovalRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private Endpoint Connection Approval ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkConnectionApprovalRequestResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "A request to approve or reject a private endpoint connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkConnectionApprovalRequest>,
}
impl PrivateLinkConnectionApprovalRequestResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of a private link connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkConnectionState {
    #[doc = "Status of a private link connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Description of a private link connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "ActionsRequired for a private link connection"]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    pub id: String,
    #[doc = "Name of a private link resource"]
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Properties of a private link resource"]
    pub properties: PrivateLinkResourceProperties,
}
impl PrivateLinkResource {
    pub fn new(id: String, name: String, type_: String, properties: PrivateLinkResourceProperties) -> Self {
        Self {
            id,
            name,
            type_,
            properties,
        }
    }
}
#[doc = "Properties of a private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "GroupId of a private link resource"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "RequiredMembers of a private link resource"]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "RequiredZoneNames of a private link resource"]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wrapper for a collection of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourcesWrapper {
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourcesWrapper {
    pub fn new(value: Vec<PrivateLinkResource>) -> Self {
        Self { value }
    }
}
#[doc = "Process Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessInfo {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "ProcessInfo resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<process_info::Properties>,
}
impl ProcessInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_info {
    use super::*;
    #[doc = "ProcessInfo resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "ARM Identifier for deployment."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub identifier: Option<i32>,
        #[doc = "Deployment name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deployment_name: Option<String>,
        #[doc = "HRef URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub href: Option<String>,
        #[doc = "Minidump URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub minidump: Option<String>,
        #[doc = "Is profile running?"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub is_profile_running: Option<bool>,
        #[doc = "Is the IIS Profile running?"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub is_iis_profile_running: Option<bool>,
        #[doc = "IIS Profile timeout (seconds)."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iis_profile_timeout_in_seconds: Option<f64>,
        #[doc = "Parent process."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parent: Option<String>,
        #[doc = "Child process list."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub children: Vec<String>,
        #[doc = "Thread list."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub threads: Vec<ProcessThreadInfo>,
        #[doc = "List of open files."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub open_file_handles: Vec<String>,
        #[doc = "List of modules."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub modules: Vec<ProcessModuleInfo>,
        #[doc = "File name of this process."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub file_name: Option<String>,
        #[doc = "Command line."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub command_line: Option<String>,
        #[doc = "User name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user_name: Option<String>,
        #[doc = "Handle count."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub handle_count: Option<i32>,
        #[doc = "Module count."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub module_count: Option<i32>,
        #[doc = "Thread count."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub thread_count: Option<i32>,
        #[doc = "Start time."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub start_time: Option<time::OffsetDateTime>,
        #[doc = "Total CPU time."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub total_cpu_time: Option<String>,
        #[doc = "User CPU time."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user_cpu_time: Option<String>,
        #[doc = "Privileged CPU time."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub privileged_cpu_time: Option<String>,
        #[doc = "Working set."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub working_set: Option<i64>,
        #[doc = "Peak working set."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub peak_working_set: Option<i64>,
        #[doc = "Private memory size."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub private_memory: Option<i64>,
        #[doc = "Virtual memory size."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub virtual_memory: Option<i64>,
        #[doc = "Peak virtual memory usage."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub peak_virtual_memory: Option<i64>,
        #[doc = "Paged system memory."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub paged_system_memory: Option<i64>,
        #[doc = "Non-paged system memory."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub non_paged_system_memory: Option<i64>,
        #[doc = "Paged memory."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub paged_memory: Option<i64>,
        #[doc = "Peak paged memory."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub peak_paged_memory: Option<i64>,
        #[doc = "Time stamp."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub time_stamp: Option<time::OffsetDateTime>,
        #[doc = "List of environment variables."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub environment_variables: Option<serde_json::Value>,
        #[doc = "Is this the SCM site?"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub is_scm_site: Option<bool>,
        #[doc = "Is this a Web Job?"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub is_webjob: Option<bool>,
        #[doc = "Description of process."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Kudu process information elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessInfoCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ProcessInfo>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProcessInfoCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProcessInfoCollection {
    pub fn new(value: Vec<ProcessInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Process Module Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessModuleInfo {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "ProcessModuleInfo resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<process_module_info::Properties>,
}
impl ProcessModuleInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_module_info {
    use super::*;
    #[doc = "ProcessModuleInfo resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Base address. Used as module identifier in ARM resource URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub base_address: Option<String>,
        #[doc = "File name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub file_name: Option<String>,
        #[doc = "HRef URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub href: Option<String>,
        #[doc = "File path."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub file_path: Option<String>,
        #[doc = "Module memory size."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub module_memory_size: Option<i32>,
        #[doc = "File version."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub file_version: Option<String>,
        #[doc = "File description."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub file_description: Option<String>,
        #[doc = "Product name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub product: Option<String>,
        #[doc = "Product version."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub product_version: Option<String>,
        #[doc = "Is debug?"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub is_debug: Option<bool>,
        #[doc = "Module language (locale)."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub language: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Kudu thread information elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessModuleInfoCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ProcessModuleInfo>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProcessModuleInfoCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProcessModuleInfoCollection {
    pub fn new(value: Vec<ProcessModuleInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Process Thread Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessThreadInfo {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "ProcessThreadInfo resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<process_thread_info::Properties>,
}
impl ProcessThreadInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_thread_info {
    use super::*;
    #[doc = "ProcessThreadInfo resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Site extension ID."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub identifier: Option<i32>,
        #[doc = "HRef URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub href: Option<String>,
        #[doc = "Process URI."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub process: Option<String>,
        #[doc = "Start address."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub start_address: Option<String>,
        #[doc = "Current thread priority."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub current_priority: Option<i32>,
        #[doc = "Thread priority level."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority_level: Option<String>,
        #[doc = "Base priority."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub base_priority: Option<i32>,
        #[doc = "Start time."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub start_time: Option<time::OffsetDateTime>,
        #[doc = "Total processor time."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub total_processor_time: Option<String>,
        #[doc = "User processor time."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user_processor_time: Option<String>,
        #[doc = "Thread state."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[doc = "Wait reason."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub wait_reason: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Kudu thread information elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessThreadInfoCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ProcessThreadInfo>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProcessThreadInfoCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProcessThreadInfoCollection {
    pub fn new(value: Vec<ProcessThreadInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Azure proxy only resource. This resource is not tracked by Azure Resource Manager."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyOnlyResource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Kind of resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyOnlyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Public certificate object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicCertificate {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "PublicCertificate resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<public_certificate::Properties>,
}
impl PublicCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod public_certificate {
    use super::*;
    #[doc = "PublicCertificate resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Public Certificate byte array"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub blob: Option<String>,
        #[doc = "Public Certificate Location"]
        #[serde(rename = "publicCertificateLocation", default, skip_serializing_if = "Option::is_none")]
        pub public_certificate_location: Option<properties::PublicCertificateLocation>,
        #[doc = "Certificate Thumbprint"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub thumbprint: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Public Certificate Location"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum PublicCertificateLocation {
            CurrentUserMy,
            LocalMachineMy,
            Unknown,
        }
    }
}
#[doc = "Collection of public certificates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicCertificateCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<PublicCertificate>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PublicCertificateCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PublicCertificateCollection {
    pub fn new(value: Vec<PublicCertificate>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Publishing Credentials Policies entity collection ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishingCredentialsPoliciesCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<CsmPublishingCredentialsPoliciesEntity>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PublishingCredentialsPoliciesCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PublishingCredentialsPoliciesCollection {
    pub fn new(value: Vec<CsmPublishingCredentialsPoliciesEntity>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Push settings for the App."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PushSettings {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "PushSettings resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<push_settings::Properties>,
}
impl PushSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod push_settings {
    use super::*;
    #[doc = "PushSettings resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Gets or sets a flag indicating whether the Push endpoint is enabled."]
        #[serde(rename = "isPushEnabled")]
        pub is_push_enabled: bool,
        #[doc = "Gets or sets a JSON string containing a list of tags that are whitelisted for use by the push registration endpoint."]
        #[serde(rename = "tagWhitelistJson", default, skip_serializing_if = "Option::is_none")]
        pub tag_whitelist_json: Option<String>,
        #[doc = "Gets or sets a JSON string containing a list of tags that require user authentication to be used in the push registration endpoint.\nTags can consist of alphanumeric characters and the following:\n'_', '@', '#', '.', ':', '-'. \nValidation should be performed at the PushRequestHandler."]
        #[serde(rename = "tagsRequiringAuth", default, skip_serializing_if = "Option::is_none")]
        pub tags_requiring_auth: Option<String>,
        #[doc = "Gets or sets a JSON string containing a list of dynamic tags that will be evaluated from user claims in the push registration endpoint."]
        #[serde(rename = "dynamicTagsJson", default, skip_serializing_if = "Option::is_none")]
        pub dynamic_tags_json: Option<String>,
    }
    impl Properties {
        pub fn new(is_push_enabled: bool) -> Self {
            Self {
                is_push_enabled,
                tag_whitelist_json: None,
                tags_requiring_auth: None,
                dynamic_tags_json: None,
            }
        }
    }
}
#[doc = "Result for utterances query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryUtterancesResult {
    #[doc = "Sample utterance."]
    #[serde(rename = "sampleUtterance", default, skip_serializing_if = "Option::is_none")]
    pub sample_utterance: Option<SampleUtterance>,
    #[doc = "Score of a sample utterance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}
impl QueryUtterancesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Suggested utterances where the detector can be applicable"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryUtterancesResults {
    #[doc = "Search Query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "Array of utterance results for search query."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<QueryUtterancesResult>,
}
impl QueryUtterancesResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Routing rules for ramp up testing. This rule allows to redirect static traffic % to a slot or to gradually change routing % based on performance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RampUpRule {
    #[doc = "Hostname of a slot to which the traffic will be redirected if decided to. E.g. myapp-stage.azurewebsites.net."]
    #[serde(rename = "actionHostName", default, skip_serializing_if = "Option::is_none")]
    pub action_host_name: Option<String>,
    #[doc = "Percentage of the traffic which will be redirected to <code>ActionHostName</code>."]
    #[serde(rename = "reroutePercentage", default, skip_serializing_if = "Option::is_none")]
    pub reroute_percentage: Option<f64>,
    #[doc = "In auto ramp up scenario this is the step to add/remove from <code>ReroutePercentage</code> until it reaches \\n<code>MinReroutePercentage</code> or \n<code>MaxReroutePercentage</code>. Site metrics are checked every N minutes specified in <code>ChangeIntervalInMinutes</code>.\\nCustom decision algorithm \ncan be provided in TiPCallback site extension which URL can be specified in <code>ChangeDecisionCallbackUrl</code>."]
    #[serde(rename = "changeStep", default, skip_serializing_if = "Option::is_none")]
    pub change_step: Option<f64>,
    #[doc = "Specifies interval in minutes to reevaluate ReroutePercentage."]
    #[serde(rename = "changeIntervalInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub change_interval_in_minutes: Option<i32>,
    #[doc = "Specifies lower boundary above which ReroutePercentage will stay."]
    #[serde(rename = "minReroutePercentage", default, skip_serializing_if = "Option::is_none")]
    pub min_reroute_percentage: Option<f64>,
    #[doc = "Specifies upper boundary below which ReroutePercentage will stay."]
    #[serde(rename = "maxReroutePercentage", default, skip_serializing_if = "Option::is_none")]
    pub max_reroute_percentage: Option<f64>,
    #[doc = "Custom decision algorithm can be provided in TiPCallback site extension which URL can be specified. See TiPCallback site extension for the scaffold and contracts.\nhttps://www.siteextensions.net/packages/TiPCallback/"]
    #[serde(rename = "changeDecisionCallbackUrl", default, skip_serializing_if = "Option::is_none")]
    pub change_decision_callback_url: Option<String>,
    #[doc = "Name of the routing rule. The recommended name would be to point to the slot which will receive the traffic in the experiment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl RampUpRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a recommendation result generated by the recommendation engine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Recommendation {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Recommendation resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<recommendation::Properties>,
}
impl Recommendation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recommendation {
    use super::*;
    #[doc = "Recommendation resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Timestamp when this instance was created."]
        #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
        pub creation_time: Option<time::OffsetDateTime>,
        #[doc = "A GUID value that each recommendation object is associated with."]
        #[serde(rename = "recommendationId", default, skip_serializing_if = "Option::is_none")]
        pub recommendation_id: Option<String>,
        #[doc = "Full ARM resource ID string that this recommendation object is associated with."]
        #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
        pub resource_id: Option<String>,
        #[doc = "Name of a resource type this recommendation applies, e.g. Subscription, ServerFarm, Site."]
        #[serde(rename = "resourceScope", default, skip_serializing_if = "Option::is_none")]
        pub resource_scope: Option<properties::ResourceScope>,
        #[doc = "Unique name of the rule."]
        #[serde(rename = "ruleName", default, skip_serializing_if = "Option::is_none")]
        pub rule_name: Option<String>,
        #[doc = "UI friendly name of the rule (may not be unique)."]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Recommendation text."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "Level indicating how critical this recommendation can impact."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<properties::Level>,
        #[doc = "List of channels that this recommendation can apply."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub channels: Option<properties::Channels>,
        #[doc = "The list of category tags that this recommendation belongs to."]
        #[serde(rename = "categoryTags", default, skip_serializing_if = "Vec::is_empty")]
        pub category_tags: Vec<String>,
        #[doc = "Name of action recommended by this object."]
        #[serde(rename = "actionName", default, skip_serializing_if = "Option::is_none")]
        pub action_name: Option<String>,
        #[doc = "True if this recommendation is still valid (i.e. \"actionable\"). False if it is invalid."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<i32>,
        #[doc = "The list of states of this recommendation. If it's null then it should be considered \"Active\"."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub states: Vec<String>,
        #[doc = "The beginning time in UTC of a range that the recommendation refers to."]
        #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
        pub start_time: Option<time::OffsetDateTime>,
        #[doc = "The end time in UTC of a range that the recommendation refers to."]
        #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
        pub end_time: Option<time::OffsetDateTime>,
        #[doc = "When to notify this recommendation next in UTC. Null means that this will never be notified anymore."]
        #[serde(rename = "nextNotificationTime", default, with = "azure_core::date::rfc3339::option")]
        pub next_notification_time: Option<time::OffsetDateTime>,
        #[doc = "Date and time in UTC when this notification expires."]
        #[serde(rename = "notificationExpirationTime", default, with = "azure_core::date::rfc3339::option")]
        pub notification_expiration_time: Option<time::OffsetDateTime>,
        #[doc = "Last timestamp in UTC this instance was actually notified. Null means that this recommendation hasn't been notified yet."]
        #[serde(rename = "notifiedTime", default, with = "azure_core::date::rfc3339::option")]
        pub notified_time: Option<time::OffsetDateTime>,
        #[doc = "A metric value measured by the rule."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub score: Option<f64>,
        #[doc = "True if this is associated with a dynamically added rule"]
        #[serde(rename = "isDynamic", default, skip_serializing_if = "Option::is_none")]
        pub is_dynamic: Option<bool>,
        #[doc = "Extension name of the portal if exists."]
        #[serde(rename = "extensionName", default, skip_serializing_if = "Option::is_none")]
        pub extension_name: Option<String>,
        #[doc = "Deep link to a blade on the portal."]
        #[serde(rename = "bladeName", default, skip_serializing_if = "Option::is_none")]
        pub blade_name: Option<String>,
        #[doc = "Forward link to an external document associated with the rule."]
        #[serde(rename = "forwardLink", default, skip_serializing_if = "Option::is_none")]
        pub forward_link: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Name of a resource type this recommendation applies, e.g. Subscription, ServerFarm, Site."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ResourceScope")]
        pub enum ResourceScope {
            ServerFarm,
            Subscription,
            WebSite,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for ResourceScope {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for ResourceScope {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for ResourceScope {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::ServerFarm => serializer.serialize_unit_variant("ResourceScope", 0u32, "ServerFarm"),
                    Self::Subscription => serializer.serialize_unit_variant("ResourceScope", 1u32, "Subscription"),
                    Self::WebSite => serializer.serialize_unit_variant("ResourceScope", 2u32, "WebSite"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Level indicating how critical this recommendation can impact."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Level {
            Critical,
            Warning,
            Information,
            NonUrgentSuggestion,
        }
        #[doc = "List of channels that this recommendation can apply."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Channels {
            Notification,
            Api,
            Email,
            Webhook,
            All,
        }
    }
}
#[doc = "Collection of recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendationCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Recommendation>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecommendationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RecommendationCollection {
    pub fn new(value: Vec<Recommendation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Represents a recommendation rule that the recommendation engine can perform."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendationRule {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "RecommendationRule resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<recommendation_rule::Properties>,
}
impl RecommendationRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recommendation_rule {
    use super::*;
    #[doc = "RecommendationRule resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Unique name of the rule."]
        #[serde(rename = "recommendationName", default, skip_serializing_if = "Option::is_none")]
        pub recommendation_name: Option<String>,
        #[doc = "UI friendly name of the rule."]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Localized name of the rule (Good for UI)."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "Recommendation ID of an associated recommendation object tied to the rule, if exists.\nIf such an object doesn't exist, it is set to null."]
        #[serde(rename = "recommendationId", default, skip_serializing_if = "Option::is_none")]
        pub recommendation_id: Option<String>,
        #[doc = "Localized detailed description of the rule."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Name of action that is recommended by this rule in string."]
        #[serde(rename = "actionName", default, skip_serializing_if = "Option::is_none")]
        pub action_name: Option<String>,
        #[doc = "Level of impact indicating how critical this rule is."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<properties::Level>,
        #[doc = "List of available channels that this rule applies."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub channels: Option<properties::Channels>,
        #[doc = "The list of category tags that this recommendation rule belongs to."]
        #[serde(rename = "categoryTags", default, skip_serializing_if = "Vec::is_empty")]
        pub category_tags: Vec<String>,
        #[doc = "True if this is associated with a dynamically added rule"]
        #[serde(rename = "isDynamic", default, skip_serializing_if = "Option::is_none")]
        pub is_dynamic: Option<bool>,
        #[doc = "Extension name of the portal if exists. Applicable to dynamic rule only."]
        #[serde(rename = "extensionName", default, skip_serializing_if = "Option::is_none")]
        pub extension_name: Option<String>,
        #[doc = "Deep link to a blade on the portal. Applicable to dynamic rule only."]
        #[serde(rename = "bladeName", default, skip_serializing_if = "Option::is_none")]
        pub blade_name: Option<String>,
        #[doc = "Forward link to an external document associated with the rule. Applicable to dynamic rule only."]
        #[serde(rename = "forwardLink", default, skip_serializing_if = "Option::is_none")]
        pub forward_link: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Level of impact indicating how critical this rule is."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Level {
            Critical,
            Warning,
            Information,
            NonUrgentSuggestion,
        }
        #[doc = "List of available channels that this rule applies."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Channels {
            Notification,
            Api,
            Email,
            Webhook,
            All,
        }
    }
}
#[doc = "Class representing certificate reissue request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReissueCertificateOrderRequest {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "ReissueCertificateOrderRequest resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<reissue_certificate_order_request::Properties>,
}
impl ReissueCertificateOrderRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod reissue_certificate_order_request {
    use super::*;
    #[doc = "ReissueCertificateOrderRequest resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Certificate Key Size."]
        #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[doc = "Delay in hours to revoke existing certificate after the new certificate is issued."]
        #[serde(rename = "delayExistingRevokeInHours", default, skip_serializing_if = "Option::is_none")]
        pub delay_existing_revoke_in_hours: Option<i32>,
        #[doc = "Csr to be used for re-key operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csr: Option<String>,
        #[doc = "Should we change the ASC type (from managed private key to external private key and vice versa)."]
        #[serde(rename = "isPrivateKeyExternal", default, skip_serializing_if = "Option::is_none")]
        pub is_private_key_external: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Hybrid Connection for an App Service app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelayServiceConnectionEntity {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "RelayServiceConnectionEntity resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<relay_service_connection_entity::Properties>,
}
impl RelayServiceConnectionEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod relay_service_connection_entity {
    use super::*;
    #[doc = "RelayServiceConnectionEntity resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "entityName", default, skip_serializing_if = "Option::is_none")]
        pub entity_name: Option<String>,
        #[serde(rename = "entityConnectionString", default, skip_serializing_if = "Option::is_none")]
        pub entity_connection_string: Option<String>,
        #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
        pub resource_type: Option<String>,
        #[serde(rename = "resourceConnectionString", default, skip_serializing_if = "Option::is_none")]
        pub resource_connection_string: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hostname: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<i32>,
        #[serde(rename = "biztalkUri", default, skip_serializing_if = "Option::is_none")]
        pub biztalk_uri: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A remote private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemotePrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "RemotePrivateEndpointConnection resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<remote_private_endpoint_connection::Properties>,
}
impl RemotePrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod remote_private_endpoint_connection {
    use super::*;
    #[doc = "RemotePrivateEndpointConnection resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<String>,
        #[doc = "A wrapper for an ARM resource id"]
        #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
        pub private_endpoint: Option<ArmIdWrapper>,
        #[doc = "The state of a private link connection"]
        #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
        pub private_link_service_connection_state: Option<PrivateLinkConnectionState>,
        #[doc = "Private IPAddresses mapped to the remote private endpoint"]
        #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_addresses: Vec<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Remote Private Endpoint Connection ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemotePrivateEndpointConnectionArmResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "RemotePrivateEndpointConnectionARMResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<remote_private_endpoint_connection_arm_resource::Properties>,
}
impl RemotePrivateEndpointConnectionArmResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod remote_private_endpoint_connection_arm_resource {
    use super::*;
    #[doc = "RemotePrivateEndpointConnectionARMResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<String>,
        #[doc = "A wrapper for an ARM resource id"]
        #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
        pub private_endpoint: Option<ArmIdWrapper>,
        #[doc = "The state of a private link connection"]
        #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
        pub private_link_service_connection_state: Option<PrivateLinkConnectionState>,
        #[doc = "Private IPAddresses mapped to the remote private endpoint"]
        #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_addresses: Vec<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Instructions for rendering the data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Rendering {
    #[doc = "Rendering Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<rendering::Type>,
    #[doc = "Title of data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Description of the data that will help it be interpreted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Rendering {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod rendering {
    use super::*;
    #[doc = "Rendering Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NoGraph,
        Table,
        TimeSeries,
        TimeSeriesPerInstance,
        PieChart,
        DataSummary,
        Email,
        Insights,
        DynamicInsight,
        Markdown,
        Detector,
        DropDown,
        Card,
        Solution,
        Guage,
        Form,
        ChangeSets,
        ChangeAnalysisOnboarding,
        ChangesView,
        AppInsight,
        DependencyGraph,
        DownTime,
        SummaryCard,
        SearchComponent,
        AppInsightEnablement,
    }
}
#[doc = "Class representing certificate renew request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewCertificateOrderRequest {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "RenewCertificateOrderRequest resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<renew_certificate_order_request::Properties>,
}
impl RenewCertificateOrderRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod renew_certificate_order_request {
    use super::*;
    #[doc = "RenewCertificateOrderRequest resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Certificate Key Size."]
        #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
        pub key_size: Option<i32>,
        #[doc = "Csr to be used for re-key operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csr: Option<String>,
        #[doc = "Should we change the ASC type (from managed private key to external private key and vice versa)."]
        #[serde(rename = "isPrivateKeyExternal", default, skip_serializing_if = "Option::is_none")]
        pub is_private_key_external: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Trigger based on total requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestsBasedTrigger {
    #[doc = "Request Count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Time interval."]
    #[serde(rename = "timeInterval", default, skip_serializing_if = "Option::is_none")]
    pub time_interval: Option<String>,
}
impl RequestsBasedTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure resource. This resource is tracked in Azure Resource Manager"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Kind of resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Resource Location."]
    pub location: String,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            kind: None,
            location,
            type_: None,
            tags: None,
        }
    }
}
#[doc = "Collection of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<String>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceCollection {
    pub fn new(value: Vec<String>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Used for getting ResourceHealthCheck settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceHealthMetadata {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "ResourceHealthMetadata resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<resource_health_metadata::Properties>,
}
impl ResourceHealthMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_health_metadata {
    use super::*;
    #[doc = "ResourceHealthMetadata resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The category that the resource matches in the RHC Policy File"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category: Option<String>,
        #[doc = "Is there a health signal for the resource"]
        #[serde(rename = "signalAvailability", default, skip_serializing_if = "Option::is_none")]
        pub signal_availability: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of resource health metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceHealthMetadataCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ResourceHealthMetadata>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceHealthMetadataCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceHealthMetadataCollection {
    pub fn new(value: Vec<ResourceHealthMetadata>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Metrics availability and retention."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceMetricAvailability {
    #[doc = "Time grain ."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "Retention period for the current time grain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
}
impl ResourceMetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for the metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceMetricDefinition {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "ResourceMetricDefinition resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<resource_metric_definition::Properties>,
}
impl ResourceMetricDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_metric_definition {
    use super::*;
    #[doc = "ResourceMetricDefinition resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Unit of the metric."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[doc = "Primary aggregation type."]
        #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
        pub primary_aggregation_type: Option<String>,
        #[doc = "List of time grains supported for the metric together with retention period."]
        #[serde(rename = "metricAvailabilities", default, skip_serializing_if = "Vec::is_empty")]
        pub metric_availabilities: Vec<ResourceMetricAvailability>,
        #[doc = "Resource URI."]
        #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
        pub resource_uri: Option<String>,
        #[doc = "Resource metric definition properties."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub properties: Option<serde_json::Value>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of metric definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceMetricDefinitionCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ResourceMetricDefinition>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceMetricDefinitionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceMetricDefinitionCollection {
    pub fn new(value: Vec<ResourceMetricDefinition>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Information regarding availability of a resource name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNameAvailability {
    #[doc = "<code>true</code> indicates name is valid and available. <code>false</code> indicates the name is invalid, unavailable, or both."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "<code>Invalid</code> indicates the name provided does not match Azure App Service naming requirements. <code>AlreadyExists</code> indicates that the name is already in use and is therefore unavailable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<resource_name_availability::Reason>,
    #[doc = "If reason == invalid, provide the user with the reason why the given name is invalid, and provide the resource naming requirements so that the user can select a valid name. If reason == AlreadyExists, explain that resource name is already in use, and direct them to select a different name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ResourceNameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_name_availability {
    use super::*;
    #[doc = "<code>Invalid</code> indicates the name provided does not match Azure App Service naming requirements. <code>AlreadyExists</code> indicates that the name is already in use and is therefore unavailable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Resource name availability request content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNameAvailabilityRequest {
    #[doc = "Resource name to verify."]
    pub name: String,
    #[doc = "Resource type used for verification."]
    #[serde(rename = "type")]
    pub type_: resource_name_availability_request::Type,
    #[doc = "Is fully qualified domain name."]
    #[serde(rename = "isFqdn", default, skip_serializing_if = "Option::is_none")]
    pub is_fqdn: Option<bool>,
}
impl ResourceNameAvailabilityRequest {
    pub fn new(name: String, type_: resource_name_availability_request::Type) -> Self {
        Self {
            name,
            type_,
            is_fqdn: None,
        }
    }
}
pub mod resource_name_availability_request {
    use super::*;
    #[doc = "Resource type used for verification."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Site,
        Slot,
        HostingEnvironment,
        PublishingUser,
        #[serde(rename = "Microsoft.Web/sites")]
        MicrosoftWebSites,
        #[serde(rename = "Microsoft.Web/sites/slots")]
        MicrosoftWebSitesSlots,
        #[serde(rename = "Microsoft.Web/hostingEnvironments")]
        MicrosoftWebHostingEnvironments,
        #[serde(rename = "Microsoft.Web/publishingUsers")]
        MicrosoftWebPublishingUsers,
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
                Self::Site => serializer.serialize_unit_variant("Type", 0u32, "Site"),
                Self::Slot => serializer.serialize_unit_variant("Type", 1u32, "Slot"),
                Self::HostingEnvironment => serializer.serialize_unit_variant("Type", 2u32, "HostingEnvironment"),
                Self::PublishingUser => serializer.serialize_unit_variant("Type", 3u32, "PublishingUser"),
                Self::MicrosoftWebSites => serializer.serialize_unit_variant("Type", 4u32, "Microsoft.Web/sites"),
                Self::MicrosoftWebSitesSlots => serializer.serialize_unit_variant("Type", 5u32, "Microsoft.Web/sites/slots"),
                Self::MicrosoftWebHostingEnvironments => {
                    serializer.serialize_unit_variant("Type", 6u32, "Microsoft.Web/hostingEnvironments")
                }
                Self::MicrosoftWebPublishingUsers => serializer.serialize_unit_variant("Type", 7u32, "Microsoft.Web/publishingUsers"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Message envelope that contains the common Azure resource manager properties and the resource provider specific content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseMessageEnvelopeRemotePrivateEndpointConnection {
    #[doc = "Resource Id. Typically ID is populated only for responses to GET requests. Caller is responsible for passing in this\nvalue for GET requests only.\nFor example: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupId}/providers/Microsoft.Web/sites/{sitename}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource e.g \"Microsoft.Web/sites\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Geographical region resource belongs to e.g. SouthCentralUS, SouthEastAsia."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags associated with resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The plan object in Azure Resource Manager, represents a marketplace plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<ArmPlan>,
    #[doc = "A remote private endpoint connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RemotePrivateEndpointConnection>,
    #[doc = "Description of a SKU for a scalable resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuDescription>,
    #[doc = "Azure-AsyncOperation Status info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Body of the error response returned from the API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorEntity>,
    #[doc = "Managed service identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Logical Availability Zones the service is hosted in"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl ResponseMessageEnvelopeRemotePrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseMetaData {
    #[doc = "Class representing data source used by the detectors"]
    #[serde(rename = "dataSource", default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}
impl ResponseMetaData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a restore request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreRequest {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "RestoreRequest resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<restore_request::Properties>,
}
impl RestoreRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restore_request {
    use super::*;
    #[doc = "RestoreRequest resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "SAS URL to the container."]
        #[serde(rename = "storageAccountUrl")]
        pub storage_account_url: String,
        #[doc = "Name of a blob which contains the backup."]
        #[serde(rename = "blobName", default, skip_serializing_if = "Option::is_none")]
        pub blob_name: Option<String>,
        #[doc = "<code>true</code> if the restore operation can overwrite target app; otherwise, <code>false</code>. <code>true</code> is needed if trying to restore over an existing app."]
        pub overwrite: bool,
        #[doc = "Name of an app."]
        #[serde(rename = "siteName", default, skip_serializing_if = "Option::is_none")]
        pub site_name: Option<String>,
        #[doc = "Collection of databases which should be restored. This list has to match the list of databases included in the backup."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub databases: Vec<DatabaseBackupSetting>,
        #[doc = "Changes a logic when restoring an app with custom domains. <code>true</code> to remove custom domains automatically. If <code>false</code>, custom domains are added to \nthe app's object when it is being restored, but that might fail due to conflicts during the operation."]
        #[serde(rename = "ignoreConflictingHostNames", default, skip_serializing_if = "Option::is_none")]
        pub ignore_conflicting_host_names: Option<bool>,
        #[doc = "Ignore the databases and only restore the site content"]
        #[serde(rename = "ignoreDatabases", default, skip_serializing_if = "Option::is_none")]
        pub ignore_databases: Option<bool>,
        #[doc = "Specify app service plan that will own restored site."]
        #[serde(rename = "appServicePlan", default, skip_serializing_if = "Option::is_none")]
        pub app_service_plan: Option<String>,
        #[doc = "Operation type."]
        #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
        pub operation_type: Option<properties::OperationType>,
        #[doc = "<code>true</code> if SiteConfig.ConnectionStrings should be set in new app; otherwise, <code>false</code>."]
        #[serde(rename = "adjustConnectionStrings", default, skip_serializing_if = "Option::is_none")]
        pub adjust_connection_strings: Option<bool>,
        #[doc = "App Service Environment name, if needed (only when restoring an app to an App Service Environment)."]
        #[serde(rename = "hostingEnvironment", default, skip_serializing_if = "Option::is_none")]
        pub hosting_environment: Option<String>,
    }
    impl Properties {
        pub fn new(storage_account_url: String, overwrite: bool) -> Self {
            Self {
                storage_account_url,
                blob_name: None,
                overwrite,
                site_name: None,
                databases: Vec::new(),
                ignore_conflicting_host_names: None,
                ignore_databases: None,
                app_service_plan: None,
                operation_type: None,
                adjust_connection_strings: None,
                hosting_environment: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Operation type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum OperationType {
            Default,
            Clone,
            Relocation,
            Snapshot,
            #[serde(rename = "CloudFS")]
            CloudFs,
        }
        impl Default for OperationType {
            fn default() -> Self {
                Self::Default
            }
        }
    }
}
#[doc = "Sample utterance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SampleUtterance {
    #[doc = "Text attribute of sample utterance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "Links attribute of sample utterance."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,
    #[doc = "Question id of sample utterance (for stackoverflow questions titles)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qid: Option<String>,
}
impl SampleUtterance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource metrics service provided by Microsoft.Insights resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A web app, a mobile app backend, or an API app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Site {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Site resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<site::Properties>,
    #[doc = "Managed service identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Extended Location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl Site {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            identity: None,
            extended_location: None,
        }
    }
}
pub mod site {
    use super::*;
    #[doc = "Site resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Current state of the app."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[doc = "Hostnames associated with the app."]
        #[serde(rename = "hostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub host_names: Vec<String>,
        #[doc = "Name of the repository site."]
        #[serde(rename = "repositorySiteName", default, skip_serializing_if = "Option::is_none")]
        pub repository_site_name: Option<String>,
        #[doc = "State indicating whether the app has exceeded its quota usage. Read-only."]
        #[serde(rename = "usageState", default, skip_serializing_if = "Option::is_none")]
        pub usage_state: Option<properties::UsageState>,
        #[doc = "<code>true</code> if the app is enabled; otherwise, <code>false</code>. Setting this value to false disables the app (takes the app offline)."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "Enabled hostnames for the app.Hostnames need to be assigned (see HostNames) AND enabled. Otherwise,\nthe app is not served on those hostnames."]
        #[serde(rename = "enabledHostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub enabled_host_names: Vec<String>,
        #[doc = "Management information availability state for the app."]
        #[serde(rename = "availabilityState", default, skip_serializing_if = "Option::is_none")]
        pub availability_state: Option<properties::AvailabilityState>,
        #[doc = "Hostname SSL states are used to manage the SSL bindings for app's hostnames."]
        #[serde(rename = "hostNameSslStates", default, skip_serializing_if = "Vec::is_empty")]
        pub host_name_ssl_states: Vec<HostNameSslState>,
        #[doc = "Resource ID of the associated App Service plan, formatted as: \"/subscriptions/{subscriptionID}/resourceGroups/{groupName}/providers/Microsoft.Web/serverfarms/{appServicePlanName}\"."]
        #[serde(rename = "serverFarmId", default, skip_serializing_if = "Option::is_none")]
        pub server_farm_id: Option<String>,
        #[doc = "<code>true</code> if reserved; otherwise, <code>false</code>."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reserved: Option<bool>,
        #[doc = "Obsolete: Hyper-V sandbox."]
        #[serde(rename = "isXenon", default, skip_serializing_if = "Option::is_none")]
        pub is_xenon: Option<bool>,
        #[doc = "Hyper-V sandbox."]
        #[serde(rename = "hyperV", default, skip_serializing_if = "Option::is_none")]
        pub hyper_v: Option<bool>,
        #[doc = "Last time the app was modified, in UTC. Read-only."]
        #[serde(rename = "lastModifiedTimeUtc", default, with = "azure_core::date::rfc3339::option")]
        pub last_modified_time_utc: Option<time::OffsetDateTime>,
        #[doc = "Configuration of an App Service app."]
        #[serde(rename = "siteConfig", default, skip_serializing_if = "Option::is_none")]
        pub site_config: Option<SiteConfig>,
        #[doc = "Azure Traffic Manager hostnames associated with the app. Read-only."]
        #[serde(rename = "trafficManagerHostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub traffic_manager_host_names: Vec<String>,
        #[doc = "<code>true</code> to stop SCM (KUDU) site when the app is stopped; otherwise, <code>false</code>. The default is <code>false</code>."]
        #[serde(rename = "scmSiteAlsoStopped", default, skip_serializing_if = "Option::is_none")]
        pub scm_site_also_stopped: Option<bool>,
        #[doc = "Specifies which deployment slot this app will swap into. Read-only."]
        #[serde(rename = "targetSwapSlot", default, skip_serializing_if = "Option::is_none")]
        pub target_swap_slot: Option<String>,
        #[doc = "Specification for an App Service Environment to use for this resource."]
        #[serde(rename = "hostingEnvironmentProfile", default, skip_serializing_if = "Option::is_none")]
        pub hosting_environment_profile: Option<HostingEnvironmentProfile>,
        #[doc = "<code>true</code> to enable client affinity; <code>false</code> to stop sending session affinity cookies, which route client requests in the same session to the same instance. Default is <code>true</code>."]
        #[serde(rename = "clientAffinityEnabled", default, skip_serializing_if = "Option::is_none")]
        pub client_affinity_enabled: Option<bool>,
        #[doc = "<code>true</code> to enable client certificate authentication (TLS mutual authentication); otherwise, <code>false</code>. Default is <code>false</code>."]
        #[serde(rename = "clientCertEnabled", default, skip_serializing_if = "Option::is_none")]
        pub client_cert_enabled: Option<bool>,
        #[doc = "This composes with ClientCertEnabled setting.\n- ClientCertEnabled: false means ClientCert is ignored.\n- ClientCertEnabled: true and ClientCertMode: Required means ClientCert is required.\n- ClientCertEnabled: true and ClientCertMode: Optional means ClientCert is optional or accepted."]
        #[serde(rename = "clientCertMode", default, skip_serializing_if = "Option::is_none")]
        pub client_cert_mode: Option<properties::ClientCertMode>,
        #[doc = "client certificate authentication comma-separated exclusion paths"]
        #[serde(rename = "clientCertExclusionPaths", default, skip_serializing_if = "Option::is_none")]
        pub client_cert_exclusion_paths: Option<String>,
        #[doc = "<code>true</code> to disable the public hostnames of the app; otherwise, <code>false</code>.\n If <code>true</code>, the app is only accessible via API management process."]
        #[serde(rename = "hostNamesDisabled", default, skip_serializing_if = "Option::is_none")]
        pub host_names_disabled: Option<bool>,
        #[doc = "Unique identifier that verifies the custom domains assigned to the app. Customer will add this id to a txt record for verification."]
        #[serde(rename = "customDomainVerificationId", default, skip_serializing_if = "Option::is_none")]
        pub custom_domain_verification_id: Option<String>,
        #[doc = "List of IP addresses that the app uses for outbound connections (e.g. database access). Includes VIPs from tenants that site can be hosted with current settings. Read-only."]
        #[serde(rename = "outboundIpAddresses", default, skip_serializing_if = "Option::is_none")]
        pub outbound_ip_addresses: Option<String>,
        #[doc = "List of IP addresses that the app uses for outbound connections (e.g. database access). Includes VIPs from all tenants except dataComponent. Read-only."]
        #[serde(rename = "possibleOutboundIpAddresses", default, skip_serializing_if = "Option::is_none")]
        pub possible_outbound_ip_addresses: Option<String>,
        #[doc = "Size of the function container."]
        #[serde(rename = "containerSize", default, skip_serializing_if = "Option::is_none")]
        pub container_size: Option<i32>,
        #[doc = "Maximum allowed daily memory-time quota (applicable on dynamic apps only)."]
        #[serde(rename = "dailyMemoryTimeQuota", default, skip_serializing_if = "Option::is_none")]
        pub daily_memory_time_quota: Option<i32>,
        #[doc = "App suspended till in case memory-time quota is exceeded."]
        #[serde(rename = "suspendedTill", default, with = "azure_core::date::rfc3339::option")]
        pub suspended_till: Option<time::OffsetDateTime>,
        #[doc = "Maximum number of workers.\nThis only applies to Functions container."]
        #[serde(rename = "maxNumberOfWorkers", default, skip_serializing_if = "Option::is_none")]
        pub max_number_of_workers: Option<i32>,
        #[doc = "Information needed for cloning operation."]
        #[serde(rename = "cloningInfo", default, skip_serializing_if = "Option::is_none")]
        pub cloning_info: Option<CloningInfo>,
        #[doc = "Name of the resource group the app belongs to. Read-only."]
        #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
        pub resource_group: Option<String>,
        #[doc = "<code>true</code> if the app is a default container; otherwise, <code>false</code>."]
        #[serde(rename = "isDefaultContainer", default, skip_serializing_if = "Option::is_none")]
        pub is_default_container: Option<bool>,
        #[doc = "Default hostname of the app. Read-only."]
        #[serde(rename = "defaultHostName", default, skip_serializing_if = "Option::is_none")]
        pub default_host_name: Option<String>,
        #[doc = "The status of the last successful slot swap operation."]
        #[serde(rename = "slotSwapStatus", default, skip_serializing_if = "Option::is_none")]
        pub slot_swap_status: Option<SlotSwapStatus>,
        #[doc = "HttpsOnly: configures a web site to accept only https requests. Issues redirect for\nhttp requests"]
        #[serde(rename = "httpsOnly", default, skip_serializing_if = "Option::is_none")]
        pub https_only: Option<bool>,
        #[doc = "Site redundancy mode"]
        #[serde(rename = "redundancyMode", default, skip_serializing_if = "Option::is_none")]
        pub redundancy_mode: Option<properties::RedundancyMode>,
        #[doc = "Specifies an operation id if this site has a pending operation."]
        #[serde(rename = "inProgressOperationId", default, skip_serializing_if = "Option::is_none")]
        pub in_progress_operation_id: Option<String>,
        #[doc = "Checks if Customer provided storage account is required"]
        #[serde(rename = "storageAccountRequired", default, skip_serializing_if = "Option::is_none")]
        pub storage_account_required: Option<bool>,
        #[doc = "Identity to use for Key Vault Reference authentication."]
        #[serde(rename = "keyVaultReferenceIdentity", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_reference_identity: Option<String>,
        #[doc = "Azure Resource Manager ID of the Virtual network and subnet to be joined by Regional VNET Integration.\nThis must be of the form /subscriptions/{subscriptionName}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{vnetName}/subnets/{subnetName}"]
        #[serde(rename = "virtualNetworkSubnetId", default, skip_serializing_if = "Option::is_none")]
        pub virtual_network_subnet_id: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "State indicating whether the app has exceeded its quota usage. Read-only."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum UsageState {
            Normal,
            Exceeded,
        }
        #[doc = "Management information availability state for the app."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum AvailabilityState {
            Normal,
            Limited,
            DisasterRecoveryMode,
        }
        #[doc = "This composes with ClientCertEnabled setting.\n- ClientCertEnabled: false means ClientCert is ignored.\n- ClientCertEnabled: true and ClientCertMode: Required means ClientCert is required.\n- ClientCertEnabled: true and ClientCertMode: Optional means ClientCert is optional or accepted."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ClientCertMode {
            Required,
            Optional,
            OptionalInteractiveUser,
        }
        #[doc = "Site redundancy mode"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum RedundancyMode {
            None,
            Manual,
            Failover,
            ActiveActive,
            GeoRedundant,
        }
    }
}
#[doc = "Configuration settings for the Azure App Service Authentication / Authorization feature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteAuthSettings {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SiteAuthSettings resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<site_auth_settings::Properties>,
}
impl SiteAuthSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod site_auth_settings {
    use super::*;
    #[doc = "SiteAuthSettings resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "<code>true</code> if the Authentication / Authorization feature is enabled for the current app; otherwise, <code>false</code>."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The RuntimeVersion of the Authentication / Authorization feature in use for the current app.\nThe setting in this value can control the behavior of certain features in the Authentication / Authorization module."]
        #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
        pub runtime_version: Option<String>,
        #[doc = "The action to take when an unauthenticated client attempts to access the app."]
        #[serde(rename = "unauthenticatedClientAction", default, skip_serializing_if = "Option::is_none")]
        pub unauthenticated_client_action: Option<properties::UnauthenticatedClientAction>,
        #[doc = "<code>true</code> to durably store platform-specific security tokens that are obtained during login flows; otherwise, <code>false</code>.\n The default is <code>false</code>."]
        #[serde(rename = "tokenStoreEnabled", default, skip_serializing_if = "Option::is_none")]
        pub token_store_enabled: Option<bool>,
        #[doc = "External URLs that can be redirected to as part of logging in or logging out of the app. Note that the query string part of the URL is ignored.\nThis is an advanced setting typically only needed by Windows Store application backends.\nNote that URLs within the current domain are always implicitly allowed."]
        #[serde(rename = "allowedExternalRedirectUrls", default, skip_serializing_if = "Vec::is_empty")]
        pub allowed_external_redirect_urls: Vec<String>,
        #[doc = "The default authentication provider to use when multiple providers are configured.\nThis setting is only needed if multiple providers are configured and the unauthenticated client\naction is set to \"RedirectToLoginPage\"."]
        #[serde(rename = "defaultProvider", default, skip_serializing_if = "Option::is_none")]
        pub default_provider: Option<properties::DefaultProvider>,
        #[doc = "The number of hours after session token expiration that a session token can be used to\ncall the token refresh API. The default is 72 hours."]
        #[serde(rename = "tokenRefreshExtensionHours", default, skip_serializing_if = "Option::is_none")]
        pub token_refresh_extension_hours: Option<f64>,
        #[doc = "The Client ID of this relying party application, known as the client_id.\nThis setting is required for enabling OpenID Connection authentication with Azure Active Directory or \nother 3rd party OpenID Connect providers.\nMore information on OpenID Connect: http://openid.net/specs/openid-connect-core-1_0.html"]
        #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[doc = "The Client Secret of this relying party application (in Azure Active Directory, this is also referred to as the Key).\nThis setting is optional. If no client secret is configured, the OpenID Connect implicit auth flow is used to authenticate end users.\nOtherwise, the OpenID Connect Authorization Code Flow is used to authenticate end users.\nMore information on OpenID Connect: http://openid.net/specs/openid-connect-core-1_0.html"]
        #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
        pub client_secret: Option<String>,
        #[doc = "The app setting name that contains the client secret of the relying party application."]
        #[serde(rename = "clientSecretSettingName", default, skip_serializing_if = "Option::is_none")]
        pub client_secret_setting_name: Option<String>,
        #[doc = "An alternative to the client secret, that is the thumbprint of a certificate used for signing purposes. This property acts as\na replacement for the Client Secret. It is also optional."]
        #[serde(rename = "clientSecretCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
        pub client_secret_certificate_thumbprint: Option<String>,
        #[doc = "The OpenID Connect Issuer URI that represents the entity which issues access tokens for this application.\nWhen using Azure Active Directory, this value is the URI of the directory tenant, e.g. https://sts.windows.net/{tenant-guid}/.\nThis URI is a case-sensitive identifier for the token issuer.\nMore information on OpenID Connect Discovery: http://openid.net/specs/openid-connect-discovery-1_0.html"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub issuer: Option<String>,
        #[doc = "Gets a value indicating whether the issuer should be a valid HTTPS url and be validated as such."]
        #[serde(rename = "validateIssuer", default, skip_serializing_if = "Option::is_none")]
        pub validate_issuer: Option<bool>,
        #[doc = "Allowed audience values to consider when validating JWTs issued by \nAzure Active Directory. Note that the <code>ClientID</code> value is always considered an\nallowed audience, regardless of this setting."]
        #[serde(rename = "allowedAudiences", default, skip_serializing_if = "Vec::is_empty")]
        pub allowed_audiences: Vec<String>,
        #[doc = "Login parameters to send to the OpenID Connect authorization endpoint when\na user logs in. Each parameter must be in the form \"key=value\"."]
        #[serde(rename = "additionalLoginParams", default, skip_serializing_if = "Vec::is_empty")]
        pub additional_login_params: Vec<String>,
        #[doc = "Gets a JSON string containing the Azure AD Acl settings."]
        #[serde(rename = "aadClaimsAuthorization", default, skip_serializing_if = "Option::is_none")]
        pub aad_claims_authorization: Option<String>,
        #[doc = "The OpenID Connect Client ID for the Google web application.\nThis setting is required for enabling Google Sign-In.\nGoogle Sign-In documentation: https://developers.google.com/identity/sign-in/web/"]
        #[serde(rename = "googleClientId", default, skip_serializing_if = "Option::is_none")]
        pub google_client_id: Option<String>,
        #[doc = "The client secret associated with the Google web application.\nThis setting is required for enabling Google Sign-In.\nGoogle Sign-In documentation: https://developers.google.com/identity/sign-in/web/"]
        #[serde(rename = "googleClientSecret", default, skip_serializing_if = "Option::is_none")]
        pub google_client_secret: Option<String>,
        #[doc = "The app setting name that contains the client secret associated with \nthe Google web application."]
        #[serde(rename = "googleClientSecretSettingName", default, skip_serializing_if = "Option::is_none")]
        pub google_client_secret_setting_name: Option<String>,
        #[doc = "The OAuth 2.0 scopes that will be requested as part of Google Sign-In authentication.\nThis setting is optional. If not specified, \"openid\", \"profile\", and \"email\" are used as default scopes.\nGoogle Sign-In documentation: https://developers.google.com/identity/sign-in/web/"]
        #[serde(rename = "googleOAuthScopes", default, skip_serializing_if = "Vec::is_empty")]
        pub google_o_auth_scopes: Vec<String>,
        #[doc = "The App ID of the Facebook app used for login.\nThis setting is required for enabling Facebook Login.\nFacebook Login documentation: https://developers.facebook.com/docs/facebook-login"]
        #[serde(rename = "facebookAppId", default, skip_serializing_if = "Option::is_none")]
        pub facebook_app_id: Option<String>,
        #[doc = "The App Secret of the Facebook app used for Facebook Login.\nThis setting is required for enabling Facebook Login.\nFacebook Login documentation: https://developers.facebook.com/docs/facebook-login"]
        #[serde(rename = "facebookAppSecret", default, skip_serializing_if = "Option::is_none")]
        pub facebook_app_secret: Option<String>,
        #[doc = "The app setting name that contains the app secret used for Facebook Login."]
        #[serde(rename = "facebookAppSecretSettingName", default, skip_serializing_if = "Option::is_none")]
        pub facebook_app_secret_setting_name: Option<String>,
        #[doc = "The OAuth 2.0 scopes that will be requested as part of Facebook Login authentication.\nThis setting is optional.\nFacebook Login documentation: https://developers.facebook.com/docs/facebook-login"]
        #[serde(rename = "facebookOAuthScopes", default, skip_serializing_if = "Vec::is_empty")]
        pub facebook_o_auth_scopes: Vec<String>,
        #[doc = "The Client Id of the GitHub app used for login.\nThis setting is required for enabling Github login"]
        #[serde(rename = "gitHubClientId", default, skip_serializing_if = "Option::is_none")]
        pub git_hub_client_id: Option<String>,
        #[doc = "The Client Secret of the GitHub app used for Github Login.\nThis setting is required for enabling Github login."]
        #[serde(rename = "gitHubClientSecret", default, skip_serializing_if = "Option::is_none")]
        pub git_hub_client_secret: Option<String>,
        #[doc = "The app setting name that contains the client secret of the Github\napp used for GitHub Login."]
        #[serde(rename = "gitHubClientSecretSettingName", default, skip_serializing_if = "Option::is_none")]
        pub git_hub_client_secret_setting_name: Option<String>,
        #[doc = "The OAuth 2.0 scopes that will be requested as part of GitHub Login authentication.\nThis setting is optional"]
        #[serde(rename = "gitHubOAuthScopes", default, skip_serializing_if = "Vec::is_empty")]
        pub git_hub_o_auth_scopes: Vec<String>,
        #[doc = "The OAuth 1.0a consumer key of the Twitter application used for sign-in.\nThis setting is required for enabling Twitter Sign-In.\nTwitter Sign-In documentation: https://dev.twitter.com/web/sign-in"]
        #[serde(rename = "twitterConsumerKey", default, skip_serializing_if = "Option::is_none")]
        pub twitter_consumer_key: Option<String>,
        #[doc = "The OAuth 1.0a consumer secret of the Twitter application used for sign-in.\nThis setting is required for enabling Twitter Sign-In.\nTwitter Sign-In documentation: https://dev.twitter.com/web/sign-in"]
        #[serde(rename = "twitterConsumerSecret", default, skip_serializing_if = "Option::is_none")]
        pub twitter_consumer_secret: Option<String>,
        #[doc = "The app setting name that contains the OAuth 1.0a consumer secret of the Twitter\napplication used for sign-in."]
        #[serde(rename = "twitterConsumerSecretSettingName", default, skip_serializing_if = "Option::is_none")]
        pub twitter_consumer_secret_setting_name: Option<String>,
        #[doc = "The OAuth 2.0 client ID that was created for the app used for authentication.\nThis setting is required for enabling Microsoft Account authentication.\nMicrosoft Account OAuth documentation: https://dev.onedrive.com/auth/msa_oauth.htm"]
        #[serde(rename = "microsoftAccountClientId", default, skip_serializing_if = "Option::is_none")]
        pub microsoft_account_client_id: Option<String>,
        #[doc = "The OAuth 2.0 client secret that was created for the app used for authentication.\nThis setting is required for enabling Microsoft Account authentication.\nMicrosoft Account OAuth documentation: https://dev.onedrive.com/auth/msa_oauth.htm"]
        #[serde(rename = "microsoftAccountClientSecret", default, skip_serializing_if = "Option::is_none")]
        pub microsoft_account_client_secret: Option<String>,
        #[doc = "The app setting name containing the OAuth 2.0 client secret that was created for the\napp used for authentication."]
        #[serde(
            rename = "microsoftAccountClientSecretSettingName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub microsoft_account_client_secret_setting_name: Option<String>,
        #[doc = "The OAuth 2.0 scopes that will be requested as part of Microsoft Account authentication.\nThis setting is optional. If not specified, \"wl.basic\" is used as the default scope.\nMicrosoft Account Scopes and permissions documentation: https://msdn.microsoft.com/en-us/library/dn631845.aspx"]
        #[serde(rename = "microsoftAccountOAuthScopes", default, skip_serializing_if = "Vec::is_empty")]
        pub microsoft_account_o_auth_scopes: Vec<String>,
        #[doc = "\"true\" if the auth config settings should be read from a file,\n\"false\" otherwise"]
        #[serde(rename = "isAuthFromFile", default, skip_serializing_if = "Option::is_none")]
        pub is_auth_from_file: Option<String>,
        #[doc = "The path of the config file containing auth settings.\nIf the path is relative, base will the site's root directory."]
        #[serde(rename = "authFilePath", default, skip_serializing_if = "Option::is_none")]
        pub auth_file_path: Option<String>,
        #[doc = "The ConfigVersion of the Authentication / Authorization feature in use for the current app.\nThe setting in this value can control the behavior of the control plane for Authentication / Authorization."]
        #[serde(rename = "configVersion", default, skip_serializing_if = "Option::is_none")]
        pub config_version: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The action to take when an unauthenticated client attempts to access the app."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum UnauthenticatedClientAction {
            RedirectToLoginPage,
            AllowAnonymous,
        }
        #[doc = "The default authentication provider to use when multiple providers are configured.\nThis setting is only needed if multiple providers are configured and the unauthenticated client\naction is set to \"RedirectToLoginPage\"."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum DefaultProvider {
            AzureActiveDirectory,
            Facebook,
            Google,
            MicrosoftAccount,
            Twitter,
            Github,
        }
    }
}
#[doc = "Configuration settings for the Azure App Service Authentication / Authorization V2 feature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteAuthSettingsV2 {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SiteAuthSettingsV2 resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<site_auth_settings_v2::Properties>,
}
impl SiteAuthSettingsV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod site_auth_settings_v2 {
    use super::*;
    #[doc = "SiteAuthSettingsV2 resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The configuration settings of the platform of App Service Authentication/Authorization."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub platform: Option<AuthPlatform>,
        #[doc = "The configuration settings that determines the validation flow of users using App Service Authentication/Authorization."]
        #[serde(rename = "globalValidation", default, skip_serializing_if = "Option::is_none")]
        pub global_validation: Option<GlobalValidation>,
        #[doc = "The configuration settings of each of the identity providers used to configure App Service Authentication/Authorization."]
        #[serde(rename = "identityProviders", default, skip_serializing_if = "Option::is_none")]
        pub identity_providers: Option<IdentityProviders>,
        #[doc = "The configuration settings of the login flow of users using App Service Authentication/Authorization."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub login: Option<Login>,
        #[doc = "The configuration settings of the HTTP requests for authentication and authorization requests made against App Service Authentication/Authorization."]
        #[serde(rename = "httpSettings", default, skip_serializing_if = "Option::is_none")]
        pub http_settings: Option<HttpSettings>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Represents whether or not an app is cloneable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteCloneability {
    #[doc = "Name of app."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<site_cloneability::Result>,
    #[doc = "List of features enabled on app that prevent cloning."]
    #[serde(rename = "blockingFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub blocking_features: Vec<SiteCloneabilityCriterion>,
    #[doc = "List of features enabled on app that are non-blocking but cannot be cloned. The app can still be cloned\nbut the features in this list will not be set up on cloned app."]
    #[serde(rename = "unsupportedFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub unsupported_features: Vec<SiteCloneabilityCriterion>,
    #[doc = "List of blocking application characteristics."]
    #[serde(rename = "blockingCharacteristics", default, skip_serializing_if = "Vec::is_empty")]
    pub blocking_characteristics: Vec<SiteCloneabilityCriterion>,
}
impl SiteCloneability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod site_cloneability {
    use super::*;
    #[doc = "Name of app."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        Cloneable,
        PartiallyCloneable,
        NotCloneable,
    }
}
#[doc = "An app cloneability criterion."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteCloneabilityCriterion {
    #[doc = "Name of criterion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of criterion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SiteCloneabilityCriterion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration of an App Service app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteConfig {
    #[doc = "Number of workers."]
    #[serde(rename = "numberOfWorkers", default, skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i32>,
    #[doc = "Default documents."]
    #[serde(rename = "defaultDocuments", default, skip_serializing_if = "Vec::is_empty")]
    pub default_documents: Vec<String>,
    #[doc = ".NET Framework version."]
    #[serde(rename = "netFrameworkVersion", default, skip_serializing_if = "Option::is_none")]
    pub net_framework_version: Option<String>,
    #[doc = "Version of PHP."]
    #[serde(rename = "phpVersion", default, skip_serializing_if = "Option::is_none")]
    pub php_version: Option<String>,
    #[doc = "Version of Python."]
    #[serde(rename = "pythonVersion", default, skip_serializing_if = "Option::is_none")]
    pub python_version: Option<String>,
    #[doc = "Version of Node.js."]
    #[serde(rename = "nodeVersion", default, skip_serializing_if = "Option::is_none")]
    pub node_version: Option<String>,
    #[doc = "Version of PowerShell."]
    #[serde(rename = "powerShellVersion", default, skip_serializing_if = "Option::is_none")]
    pub power_shell_version: Option<String>,
    #[doc = "Linux App Framework and version"]
    #[serde(rename = "linuxFxVersion", default, skip_serializing_if = "Option::is_none")]
    pub linux_fx_version: Option<String>,
    #[doc = "Xenon App Framework and version"]
    #[serde(rename = "windowsFxVersion", default, skip_serializing_if = "Option::is_none")]
    pub windows_fx_version: Option<String>,
    #[doc = "<code>true</code> if request tracing is enabled; otherwise, <code>false</code>."]
    #[serde(rename = "requestTracingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub request_tracing_enabled: Option<bool>,
    #[doc = "Request tracing expiration time."]
    #[serde(rename = "requestTracingExpirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub request_tracing_expiration_time: Option<time::OffsetDateTime>,
    #[doc = "<code>true</code> if remote debugging is enabled; otherwise, <code>false</code>."]
    #[serde(rename = "remoteDebuggingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub remote_debugging_enabled: Option<bool>,
    #[doc = "Remote debugging version."]
    #[serde(rename = "remoteDebuggingVersion", default, skip_serializing_if = "Option::is_none")]
    pub remote_debugging_version: Option<String>,
    #[doc = "<code>true</code> if HTTP logging is enabled; otherwise, <code>false</code>."]
    #[serde(rename = "httpLoggingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub http_logging_enabled: Option<bool>,
    #[doc = "Flag to use Managed Identity Creds for ACR pull"]
    #[serde(rename = "acrUseManagedIdentityCreds", default, skip_serializing_if = "Option::is_none")]
    pub acr_use_managed_identity_creds: Option<bool>,
    #[doc = "If using user managed identity, the user managed identity ClientId"]
    #[serde(rename = "acrUserManagedIdentityID", default, skip_serializing_if = "Option::is_none")]
    pub acr_user_managed_identity_id: Option<String>,
    #[doc = "HTTP logs directory size limit."]
    #[serde(rename = "logsDirectorySizeLimit", default, skip_serializing_if = "Option::is_none")]
    pub logs_directory_size_limit: Option<i32>,
    #[doc = "<code>true</code> if detailed error logging is enabled; otherwise, <code>false</code>."]
    #[serde(rename = "detailedErrorLoggingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub detailed_error_logging_enabled: Option<bool>,
    #[doc = "Publishing user name."]
    #[serde(rename = "publishingUsername", default, skip_serializing_if = "Option::is_none")]
    pub publishing_username: Option<String>,
    #[doc = "Application settings."]
    #[serde(rename = "appSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub app_settings: Vec<NameValuePair>,
    #[doc = "Connection strings."]
    #[serde(rename = "connectionStrings", default, skip_serializing_if = "Vec::is_empty")]
    pub connection_strings: Vec<ConnStringInfo>,
    #[doc = "MachineKey of an app."]
    #[serde(rename = "machineKey", default, skip_serializing_if = "Option::is_none")]
    pub machine_key: Option<SiteMachineKey>,
    #[doc = "Handler mappings."]
    #[serde(rename = "handlerMappings", default, skip_serializing_if = "Vec::is_empty")]
    pub handler_mappings: Vec<HandlerMapping>,
    #[doc = "Document root."]
    #[serde(rename = "documentRoot", default, skip_serializing_if = "Option::is_none")]
    pub document_root: Option<String>,
    #[doc = "SCM type."]
    #[serde(rename = "scmType", default, skip_serializing_if = "Option::is_none")]
    pub scm_type: Option<site_config::ScmType>,
    #[doc = "<code>true</code> to use 32-bit worker process; otherwise, <code>false</code>."]
    #[serde(rename = "use32BitWorkerProcess", default, skip_serializing_if = "Option::is_none")]
    pub use32_bit_worker_process: Option<bool>,
    #[doc = "<code>true</code> if WebSocket is enabled; otherwise, <code>false</code>."]
    #[serde(rename = "webSocketsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub web_sockets_enabled: Option<bool>,
    #[doc = "<code>true</code> if Always On is enabled; otherwise, <code>false</code>."]
    #[serde(rename = "alwaysOn", default, skip_serializing_if = "Option::is_none")]
    pub always_on: Option<bool>,
    #[doc = "Java version."]
    #[serde(rename = "javaVersion", default, skip_serializing_if = "Option::is_none")]
    pub java_version: Option<String>,
    #[doc = "Java container."]
    #[serde(rename = "javaContainer", default, skip_serializing_if = "Option::is_none")]
    pub java_container: Option<String>,
    #[doc = "Java container version."]
    #[serde(rename = "javaContainerVersion", default, skip_serializing_if = "Option::is_none")]
    pub java_container_version: Option<String>,
    #[doc = "App command line to launch."]
    #[serde(rename = "appCommandLine", default, skip_serializing_if = "Option::is_none")]
    pub app_command_line: Option<String>,
    #[doc = "Managed pipeline mode."]
    #[serde(rename = "managedPipelineMode", default, skip_serializing_if = "Option::is_none")]
    pub managed_pipeline_mode: Option<site_config::ManagedPipelineMode>,
    #[doc = "Virtual applications."]
    #[serde(rename = "virtualApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_applications: Vec<VirtualApplication>,
    #[doc = "Site load balancing."]
    #[serde(rename = "loadBalancing", default, skip_serializing_if = "Option::is_none")]
    pub load_balancing: Option<site_config::LoadBalancing>,
    #[doc = "Routing rules in production experiments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experiments: Option<Experiments>,
    #[doc = "Metric limits set on an app."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<SiteLimits>,
    #[doc = "<code>true</code> if Auto Heal is enabled; otherwise, <code>false</code>."]
    #[serde(rename = "autoHealEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auto_heal_enabled: Option<bool>,
    #[doc = "Rules that can be defined for auto-heal."]
    #[serde(rename = "autoHealRules", default, skip_serializing_if = "Option::is_none")]
    pub auto_heal_rules: Option<AutoHealRules>,
    #[doc = "Tracing options."]
    #[serde(rename = "tracingOptions", default, skip_serializing_if = "Option::is_none")]
    pub tracing_options: Option<String>,
    #[doc = "Virtual Network name."]
    #[serde(rename = "vnetName", default, skip_serializing_if = "Option::is_none")]
    pub vnet_name: Option<String>,
    #[doc = "Virtual Network Route All enabled. This causes all outbound traffic to have Virtual Network Security Groups and User Defined Routes applied."]
    #[serde(rename = "vnetRouteAllEnabled", default, skip_serializing_if = "Option::is_none")]
    pub vnet_route_all_enabled: Option<bool>,
    #[doc = "The number of private ports assigned to this app. These will be assigned dynamically on runtime."]
    #[serde(rename = "vnetPrivatePortsCount", default, skip_serializing_if = "Option::is_none")]
    pub vnet_private_ports_count: Option<i32>,
    #[doc = "Cross-Origin Resource Sharing (CORS) settings for the app."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<CorsSettings>,
    #[doc = "Push settings for the App."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<PushSettings>,
    #[doc = "Information about the formal API definition for the app."]
    #[serde(rename = "apiDefinition", default, skip_serializing_if = "Option::is_none")]
    pub api_definition: Option<ApiDefinitionInfo>,
    #[doc = "Azure API management (APIM) configuration linked to the app."]
    #[serde(rename = "apiManagementConfig", default, skip_serializing_if = "Option::is_none")]
    pub api_management_config: Option<ApiManagementConfig>,
    #[doc = "Auto-swap slot name."]
    #[serde(rename = "autoSwapSlotName", default, skip_serializing_if = "Option::is_none")]
    pub auto_swap_slot_name: Option<String>,
    #[doc = "<code>true</code> to enable local MySQL; otherwise, <code>false</code>."]
    #[serde(rename = "localMySqlEnabled", default, skip_serializing_if = "Option::is_none")]
    pub local_my_sql_enabled: Option<bool>,
    #[doc = "Managed Service Identity Id"]
    #[serde(rename = "managedServiceIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub managed_service_identity_id: Option<i32>,
    #[doc = "Explicit Managed Service Identity Id"]
    #[serde(rename = "xManagedServiceIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub x_managed_service_identity_id: Option<i32>,
    #[doc = "Identity to use for Key Vault Reference authentication."]
    #[serde(rename = "keyVaultReferenceIdentity", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_reference_identity: Option<String>,
    #[doc = "IP security restrictions for main."]
    #[serde(rename = "ipSecurityRestrictions", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_security_restrictions: Vec<IpSecurityRestriction>,
    #[doc = "IP security restrictions for scm."]
    #[serde(rename = "scmIpSecurityRestrictions", default, skip_serializing_if = "Vec::is_empty")]
    pub scm_ip_security_restrictions: Vec<IpSecurityRestriction>,
    #[doc = "IP security restrictions for scm to use main."]
    #[serde(rename = "scmIpSecurityRestrictionsUseMain", default, skip_serializing_if = "Option::is_none")]
    pub scm_ip_security_restrictions_use_main: Option<bool>,
    #[doc = "Http20Enabled: configures a web site to allow clients to connect over http2.0"]
    #[serde(rename = "http20Enabled", default, skip_serializing_if = "Option::is_none")]
    pub http20_enabled: Option<bool>,
    #[doc = "MinTlsVersion: configures the minimum version of TLS required for SSL requests"]
    #[serde(rename = "minTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub min_tls_version: Option<site_config::MinTlsVersion>,
    #[doc = "ScmMinTlsVersion: configures the minimum version of TLS required for SSL requests for SCM site"]
    #[serde(rename = "scmMinTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub scm_min_tls_version: Option<site_config::ScmMinTlsVersion>,
    #[doc = "State of FTP / FTPS service"]
    #[serde(rename = "ftpsState", default, skip_serializing_if = "Option::is_none")]
    pub ftps_state: Option<site_config::FtpsState>,
    #[doc = "Number of preWarmed instances.\nThis setting only applies to the Consumption and Elastic Plans"]
    #[serde(rename = "preWarmedInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub pre_warmed_instance_count: Option<i32>,
    #[doc = "Maximum number of workers that a site can scale out to.\nThis setting only applies to the Consumption and Elastic Premium Plans"]
    #[serde(rename = "functionAppScaleLimit", default, skip_serializing_if = "Option::is_none")]
    pub function_app_scale_limit: Option<i32>,
    #[doc = "Health check path"]
    #[serde(rename = "healthCheckPath", default, skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    #[doc = "Gets or sets a value indicating whether functions runtime scale monitoring is enabled. When enabled,\nthe ScaleController will not monitor event sources directly, but will instead call to the\nruntime to get scale status."]
    #[serde(
        rename = "functionsRuntimeScaleMonitoringEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub functions_runtime_scale_monitoring_enabled: Option<bool>,
    #[doc = "Sets the time zone a site uses for generating timestamps. Compatible with Linux and Windows App Service. Setting the WEBSITE_TIME_ZONE app setting takes precedence over this config. For Linux, expects tz database values https://www.iana.org/time-zones (for a quick reference see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). For Windows, expects one of the time zones listed under HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Time Zones"]
    #[serde(rename = "websiteTimeZone", default, skip_serializing_if = "Option::is_none")]
    pub website_time_zone: Option<String>,
    #[doc = "Number of minimum instance count for a site\nThis setting only applies to the Elastic Plans"]
    #[serde(rename = "minimumElasticInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub minimum_elastic_instance_count: Option<i32>,
    #[doc = "List of Azure Storage Accounts."]
    #[serde(rename = "azureStorageAccounts", default, skip_serializing_if = "Option::is_none")]
    pub azure_storage_accounts: Option<serde_json::Value>,
    #[doc = "Property to allow or block all public traffic."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,
}
impl SiteConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod site_config {
    use super::*;
    #[doc = "SCM type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScmType")]
    pub enum ScmType {
        None,
        Dropbox,
        Tfs,
        LocalGit,
        GitHub,
        CodePlexGit,
        CodePlexHg,
        BitbucketGit,
        BitbucketHg,
        ExternalGit,
        ExternalHg,
        OneDrive,
        #[serde(rename = "VSO")]
        Vso,
        #[serde(rename = "VSTSRM")]
        Vstsrm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScmType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScmType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScmType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ScmType", 0u32, "None"),
                Self::Dropbox => serializer.serialize_unit_variant("ScmType", 1u32, "Dropbox"),
                Self::Tfs => serializer.serialize_unit_variant("ScmType", 2u32, "Tfs"),
                Self::LocalGit => serializer.serialize_unit_variant("ScmType", 3u32, "LocalGit"),
                Self::GitHub => serializer.serialize_unit_variant("ScmType", 4u32, "GitHub"),
                Self::CodePlexGit => serializer.serialize_unit_variant("ScmType", 5u32, "CodePlexGit"),
                Self::CodePlexHg => serializer.serialize_unit_variant("ScmType", 6u32, "CodePlexHg"),
                Self::BitbucketGit => serializer.serialize_unit_variant("ScmType", 7u32, "BitbucketGit"),
                Self::BitbucketHg => serializer.serialize_unit_variant("ScmType", 8u32, "BitbucketHg"),
                Self::ExternalGit => serializer.serialize_unit_variant("ScmType", 9u32, "ExternalGit"),
                Self::ExternalHg => serializer.serialize_unit_variant("ScmType", 10u32, "ExternalHg"),
                Self::OneDrive => serializer.serialize_unit_variant("ScmType", 11u32, "OneDrive"),
                Self::Vso => serializer.serialize_unit_variant("ScmType", 12u32, "VSO"),
                Self::Vstsrm => serializer.serialize_unit_variant("ScmType", 13u32, "VSTSRM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Managed pipeline mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ManagedPipelineMode {
        Integrated,
        Classic,
    }
    #[doc = "Site load balancing."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoadBalancing {
        WeightedRoundRobin,
        LeastRequests,
        LeastResponseTime,
        WeightedTotalTraffic,
        RequestHash,
        PerSiteRoundRobin,
    }
    #[doc = "MinTlsVersion: configures the minimum version of TLS required for SSL requests"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MinTlsVersion")]
    pub enum MinTlsVersion {
        #[serde(rename = "1.0")]
        N1_0,
        #[serde(rename = "1.1")]
        N1_1,
        #[serde(rename = "1.2")]
        N1_2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MinTlsVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MinTlsVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MinTlsVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N1_0 => serializer.serialize_unit_variant("MinTlsVersion", 0u32, "1.0"),
                Self::N1_1 => serializer.serialize_unit_variant("MinTlsVersion", 1u32, "1.1"),
                Self::N1_2 => serializer.serialize_unit_variant("MinTlsVersion", 2u32, "1.2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "ScmMinTlsVersion: configures the minimum version of TLS required for SSL requests for SCM site"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScmMinTlsVersion")]
    pub enum ScmMinTlsVersion {
        #[serde(rename = "1.0")]
        N1_0,
        #[serde(rename = "1.1")]
        N1_1,
        #[serde(rename = "1.2")]
        N1_2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScmMinTlsVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScmMinTlsVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScmMinTlsVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N1_0 => serializer.serialize_unit_variant("ScmMinTlsVersion", 0u32, "1.0"),
                Self::N1_1 => serializer.serialize_unit_variant("ScmMinTlsVersion", 1u32, "1.1"),
                Self::N1_2 => serializer.serialize_unit_variant("ScmMinTlsVersion", 2u32, "1.2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "State of FTP / FTPS service"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FtpsState")]
    pub enum FtpsState {
        AllAllowed,
        FtpsOnly,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FtpsState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FtpsState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FtpsState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllAllowed => serializer.serialize_unit_variant("FtpsState", 0u32, "AllAllowed"),
                Self::FtpsOnly => serializer.serialize_unit_variant("FtpsState", 1u32, "FtpsOnly"),
                Self::Disabled => serializer.serialize_unit_variant("FtpsState", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Site config properties dictionary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteConfigPropertiesDictionary {
    #[doc = "<code>true</code> if use32BitWorkerProcess should be set to true for the stack; otherwise, <code>false</code>."]
    #[serde(rename = "use32BitWorkerProcess", default, skip_serializing_if = "Option::is_none")]
    pub use32_bit_worker_process: Option<bool>,
    #[doc = "LinuxFxVersion configuration setting."]
    #[serde(rename = "linuxFxVersion", default, skip_serializing_if = "Option::is_none")]
    pub linux_fx_version: Option<String>,
    #[doc = "JavaVersion configuration setting."]
    #[serde(rename = "javaVersion", default, skip_serializing_if = "Option::is_none")]
    pub java_version: Option<String>,
    #[doc = "PowerShellVersion configuration setting."]
    #[serde(rename = "powerShellVersion", default, skip_serializing_if = "Option::is_none")]
    pub power_shell_version: Option<String>,
}
impl SiteConfigPropertiesDictionary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web app configuration ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteConfigResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Configuration of an App Service app."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SiteConfig>,
}
impl SiteConfigResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of site configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteConfigResourceCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<SiteConfigResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SiteConfigResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SiteConfigResourceCollection {
    pub fn new(value: Vec<SiteConfigResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A snapshot of a web app configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteConfigurationSnapshotInfo {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SiteConfigurationSnapshotInfo resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<site_configuration_snapshot_info::Properties>,
}
impl SiteConfigurationSnapshotInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod site_configuration_snapshot_info {
    use super::*;
    #[doc = "SiteConfigurationSnapshotInfo resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The time the snapshot was taken."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub time: Option<time::OffsetDateTime>,
        #[doc = "The id of the snapshot"]
        #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
        pub snapshot_id: Option<i32>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of metadata for the app configuration snapshots that can be restored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteConfigurationSnapshotInfoCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<SiteConfigurationSnapshotInfo>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SiteConfigurationSnapshotInfoCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SiteConfigurationSnapshotInfoCollection {
    pub fn new(value: Vec<SiteConfigurationSnapshotInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Site Extension Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteExtensionInfo {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SiteExtensionInfo resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<site_extension_info::Properties>,
}
impl SiteExtensionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod site_extension_info {
    use super::*;
    #[doc = "SiteExtensionInfo resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Site extension ID."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extension_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[doc = "Site extension type."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extension_type: Option<properties::ExtensionType>,
        #[doc = "Summary description."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[doc = "Detailed description."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Version information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[doc = "Extension URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extension_url: Option<String>,
        #[doc = "Project URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub project_url: Option<String>,
        #[doc = "Icon URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon_url: Option<String>,
        #[doc = "License URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub license_url: Option<String>,
        #[doc = "Feed URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub feed_url: Option<String>,
        #[doc = "List of authors."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub authors: Vec<String>,
        #[doc = "Installer command line parameters."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub installer_command_line_params: Option<String>,
        #[doc = "Published timestamp."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub published_date_time: Option<time::OffsetDateTime>,
        #[doc = "Count of downloads."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub download_count: Option<i32>,
        #[doc = "<code>true</code> if the local version is the latest version; <code>false</code> otherwise."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub local_is_latest_version: Option<bool>,
        #[doc = "Local path."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub local_path: Option<String>,
        #[doc = "Installed timestamp."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub installed_date_time: Option<time::OffsetDateTime>,
        #[doc = "Provisioning state."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<String>,
        #[doc = "Site Extension comment."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub comment: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Site extension type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ExtensionType {
            Gallery,
            WebRoot,
        }
    }
}
#[doc = "Collection of Kudu site extension information elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteExtensionInfoCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<SiteExtensionInfo>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SiteExtensionInfoCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SiteExtensionInfoCollection {
    pub fn new(value: Vec<SiteExtensionInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Metric limits set on an app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteLimits {
    #[doc = "Maximum allowed CPU usage percentage."]
    #[serde(rename = "maxPercentageCpu", default, skip_serializing_if = "Option::is_none")]
    pub max_percentage_cpu: Option<f64>,
    #[doc = "Maximum allowed memory usage in MB."]
    #[serde(rename = "maxMemoryInMb", default, skip_serializing_if = "Option::is_none")]
    pub max_memory_in_mb: Option<i64>,
    #[doc = "Maximum allowed disk size usage in MB."]
    #[serde(rename = "maxDiskSizeInMb", default, skip_serializing_if = "Option::is_none")]
    pub max_disk_size_in_mb: Option<i64>,
}
impl SiteLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration of App Service site logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteLogsConfig {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SiteLogsConfig resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<site_logs_config::Properties>,
}
impl SiteLogsConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod site_logs_config {
    use super::*;
    #[doc = "SiteLogsConfig resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Application logs configuration."]
        #[serde(rename = "applicationLogs", default, skip_serializing_if = "Option::is_none")]
        pub application_logs: Option<ApplicationLogsConfig>,
        #[doc = "Http logs configuration."]
        #[serde(rename = "httpLogs", default, skip_serializing_if = "Option::is_none")]
        pub http_logs: Option<HttpLogsConfig>,
        #[doc = "Enabled configuration."]
        #[serde(rename = "failedRequestsTracing", default, skip_serializing_if = "Option::is_none")]
        pub failed_requests_tracing: Option<EnabledConfig>,
        #[doc = "Enabled configuration."]
        #[serde(rename = "detailedErrorMessages", default, skip_serializing_if = "Option::is_none")]
        pub detailed_error_messages: Option<EnabledConfig>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "MachineKey of an app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteMachineKey {
    #[doc = "MachineKey validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<String>,
    #[doc = "Validation key."]
    #[serde(rename = "validationKey", default, skip_serializing_if = "Option::is_none")]
    pub validation_key: Option<String>,
    #[doc = "Algorithm used for decryption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decryption: Option<String>,
    #[doc = "Decryption key."]
    #[serde(rename = "decryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub decryption_key: Option<String>,
}
impl SiteMachineKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM resource for a site."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SitePatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SitePatchResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<site_patch_resource::Properties>,
    #[doc = "Managed service identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl SitePatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod site_patch_resource {
    use super::*;
    #[doc = "SitePatchResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Current state of the app."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[doc = "Hostnames associated with the app."]
        #[serde(rename = "hostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub host_names: Vec<String>,
        #[doc = "Name of the repository site."]
        #[serde(rename = "repositorySiteName", default, skip_serializing_if = "Option::is_none")]
        pub repository_site_name: Option<String>,
        #[doc = "State indicating whether the app has exceeded its quota usage. Read-only."]
        #[serde(rename = "usageState", default, skip_serializing_if = "Option::is_none")]
        pub usage_state: Option<properties::UsageState>,
        #[doc = "<code>true</code> if the app is enabled; otherwise, <code>false</code>. Setting this value to false disables the app (takes the app offline)."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "Enabled hostnames for the app.Hostnames need to be assigned (see HostNames) AND enabled. Otherwise,\nthe app is not served on those hostnames."]
        #[serde(rename = "enabledHostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub enabled_host_names: Vec<String>,
        #[doc = "Management information availability state for the app."]
        #[serde(rename = "availabilityState", default, skip_serializing_if = "Option::is_none")]
        pub availability_state: Option<properties::AvailabilityState>,
        #[doc = "Hostname SSL states are used to manage the SSL bindings for app's hostnames."]
        #[serde(rename = "hostNameSslStates", default, skip_serializing_if = "Vec::is_empty")]
        pub host_name_ssl_states: Vec<HostNameSslState>,
        #[doc = "Resource ID of the associated App Service plan, formatted as: \"/subscriptions/{subscriptionID}/resourceGroups/{groupName}/providers/Microsoft.Web/serverfarms/{appServicePlanName}\"."]
        #[serde(rename = "serverFarmId", default, skip_serializing_if = "Option::is_none")]
        pub server_farm_id: Option<String>,
        #[doc = "<code>true</code> if reserved; otherwise, <code>false</code>."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reserved: Option<bool>,
        #[doc = "Obsolete: Hyper-V sandbox."]
        #[serde(rename = "isXenon", default, skip_serializing_if = "Option::is_none")]
        pub is_xenon: Option<bool>,
        #[doc = "Hyper-V sandbox."]
        #[serde(rename = "hyperV", default, skip_serializing_if = "Option::is_none")]
        pub hyper_v: Option<bool>,
        #[doc = "Last time the app was modified, in UTC. Read-only."]
        #[serde(rename = "lastModifiedTimeUtc", default, with = "azure_core::date::rfc3339::option")]
        pub last_modified_time_utc: Option<time::OffsetDateTime>,
        #[doc = "Configuration of an App Service app."]
        #[serde(rename = "siteConfig", default, skip_serializing_if = "Option::is_none")]
        pub site_config: Option<SiteConfig>,
        #[doc = "Azure Traffic Manager hostnames associated with the app. Read-only."]
        #[serde(rename = "trafficManagerHostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub traffic_manager_host_names: Vec<String>,
        #[doc = "<code>true</code> to stop SCM (KUDU) site when the app is stopped; otherwise, <code>false</code>. The default is <code>false</code>."]
        #[serde(rename = "scmSiteAlsoStopped", default, skip_serializing_if = "Option::is_none")]
        pub scm_site_also_stopped: Option<bool>,
        #[doc = "Specifies which deployment slot this app will swap into. Read-only."]
        #[serde(rename = "targetSwapSlot", default, skip_serializing_if = "Option::is_none")]
        pub target_swap_slot: Option<String>,
        #[doc = "Specification for an App Service Environment to use for this resource."]
        #[serde(rename = "hostingEnvironmentProfile", default, skip_serializing_if = "Option::is_none")]
        pub hosting_environment_profile: Option<HostingEnvironmentProfile>,
        #[doc = "<code>true</code> to enable client affinity; <code>false</code> to stop sending session affinity cookies, which route client requests in the same session to the same instance. Default is <code>true</code>."]
        #[serde(rename = "clientAffinityEnabled", default, skip_serializing_if = "Option::is_none")]
        pub client_affinity_enabled: Option<bool>,
        #[doc = "<code>true</code> to enable client certificate authentication (TLS mutual authentication); otherwise, <code>false</code>. Default is <code>false</code>."]
        #[serde(rename = "clientCertEnabled", default, skip_serializing_if = "Option::is_none")]
        pub client_cert_enabled: Option<bool>,
        #[doc = "This composes with ClientCertEnabled setting.\n- ClientCertEnabled: false means ClientCert is ignored.\n- ClientCertEnabled: true and ClientCertMode: Required means ClientCert is required.\n- ClientCertEnabled: true and ClientCertMode: Optional means ClientCert is optional or accepted."]
        #[serde(rename = "clientCertMode", default, skip_serializing_if = "Option::is_none")]
        pub client_cert_mode: Option<properties::ClientCertMode>,
        #[doc = "client certificate authentication comma-separated exclusion paths"]
        #[serde(rename = "clientCertExclusionPaths", default, skip_serializing_if = "Option::is_none")]
        pub client_cert_exclusion_paths: Option<String>,
        #[doc = "<code>true</code> to disable the public hostnames of the app; otherwise, <code>false</code>.\n If <code>true</code>, the app is only accessible via API management process."]
        #[serde(rename = "hostNamesDisabled", default, skip_serializing_if = "Option::is_none")]
        pub host_names_disabled: Option<bool>,
        #[doc = "Unique identifier that verifies the custom domains assigned to the app. Customer will add this id to a txt record for verification."]
        #[serde(rename = "customDomainVerificationId", default, skip_serializing_if = "Option::is_none")]
        pub custom_domain_verification_id: Option<String>,
        #[doc = "List of IP addresses that the app uses for outbound connections (e.g. database access). Includes VIPs from tenants that site can be hosted with current settings. Read-only."]
        #[serde(rename = "outboundIpAddresses", default, skip_serializing_if = "Option::is_none")]
        pub outbound_ip_addresses: Option<String>,
        #[doc = "List of IP addresses that the app uses for outbound connections (e.g. database access). Includes VIPs from all tenants except dataComponent. Read-only."]
        #[serde(rename = "possibleOutboundIpAddresses", default, skip_serializing_if = "Option::is_none")]
        pub possible_outbound_ip_addresses: Option<String>,
        #[doc = "Size of the function container."]
        #[serde(rename = "containerSize", default, skip_serializing_if = "Option::is_none")]
        pub container_size: Option<i32>,
        #[doc = "Maximum allowed daily memory-time quota (applicable on dynamic apps only)."]
        #[serde(rename = "dailyMemoryTimeQuota", default, skip_serializing_if = "Option::is_none")]
        pub daily_memory_time_quota: Option<i32>,
        #[doc = "App suspended till in case memory-time quota is exceeded."]
        #[serde(rename = "suspendedTill", default, with = "azure_core::date::rfc3339::option")]
        pub suspended_till: Option<time::OffsetDateTime>,
        #[doc = "Maximum number of workers.\nThis only applies to Functions container."]
        #[serde(rename = "maxNumberOfWorkers", default, skip_serializing_if = "Option::is_none")]
        pub max_number_of_workers: Option<i32>,
        #[doc = "Information needed for cloning operation."]
        #[serde(rename = "cloningInfo", default, skip_serializing_if = "Option::is_none")]
        pub cloning_info: Option<CloningInfo>,
        #[doc = "Name of the resource group the app belongs to. Read-only."]
        #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
        pub resource_group: Option<String>,
        #[doc = "<code>true</code> if the app is a default container; otherwise, <code>false</code>."]
        #[serde(rename = "isDefaultContainer", default, skip_serializing_if = "Option::is_none")]
        pub is_default_container: Option<bool>,
        #[doc = "Default hostname of the app. Read-only."]
        #[serde(rename = "defaultHostName", default, skip_serializing_if = "Option::is_none")]
        pub default_host_name: Option<String>,
        #[doc = "The status of the last successful slot swap operation."]
        #[serde(rename = "slotSwapStatus", default, skip_serializing_if = "Option::is_none")]
        pub slot_swap_status: Option<SlotSwapStatus>,
        #[doc = "HttpsOnly: configures a web site to accept only https requests. Issues redirect for\nhttp requests"]
        #[serde(rename = "httpsOnly", default, skip_serializing_if = "Option::is_none")]
        pub https_only: Option<bool>,
        #[doc = "Site redundancy mode"]
        #[serde(rename = "redundancyMode", default, skip_serializing_if = "Option::is_none")]
        pub redundancy_mode: Option<properties::RedundancyMode>,
        #[doc = "Specifies an operation id if this site has a pending operation."]
        #[serde(rename = "inProgressOperationId", default, skip_serializing_if = "Option::is_none")]
        pub in_progress_operation_id: Option<String>,
        #[doc = "Checks if Customer provided storage account is required"]
        #[serde(rename = "storageAccountRequired", default, skip_serializing_if = "Option::is_none")]
        pub storage_account_required: Option<bool>,
        #[doc = "Identity to use for Key Vault Reference authentication."]
        #[serde(rename = "keyVaultReferenceIdentity", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_reference_identity: Option<String>,
        #[doc = "Azure Resource Manager ID of the Virtual network and subnet to be joined by Regional VNET Integration.\nThis must be of the form /subscriptions/{subscriptionName}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{vnetName}/subnets/{subnetName}"]
        #[serde(rename = "virtualNetworkSubnetId", default, skip_serializing_if = "Option::is_none")]
        pub virtual_network_subnet_id: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "State indicating whether the app has exceeded its quota usage. Read-only."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum UsageState {
            Normal,
            Exceeded,
        }
        #[doc = "Management information availability state for the app."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum AvailabilityState {
            Normal,
            Limited,
            DisasterRecoveryMode,
        }
        #[doc = "This composes with ClientCertEnabled setting.\n- ClientCertEnabled: false means ClientCert is ignored.\n- ClientCertEnabled: true and ClientCertMode: Required means ClientCert is required.\n- ClientCertEnabled: true and ClientCertMode: Optional means ClientCert is optional or accepted."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ClientCertMode {
            Required,
            Optional,
            OptionalInteractiveUser,
        }
        #[doc = "Site redundancy mode"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum RedundancyMode {
            None,
            Manual,
            Failover,
            ActiveActive,
            GeoRedundant,
        }
    }
}
#[doc = "Used for getting PHP error logging flag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SitePhpErrorLogFlag {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SitePhpErrorLogFlag resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<site_php_error_log_flag::Properties>,
}
impl SitePhpErrorLogFlag {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod site_php_error_log_flag {
    use super::*;
    #[doc = "SitePhpErrorLogFlag resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Local log_errors setting."]
        #[serde(rename = "localLogErrors", default, skip_serializing_if = "Option::is_none")]
        pub local_log_errors: Option<String>,
        #[doc = "Master log_errors setting."]
        #[serde(rename = "masterLogErrors", default, skip_serializing_if = "Option::is_none")]
        pub master_log_errors: Option<String>,
        #[doc = "Local log_errors_max_len setting."]
        #[serde(rename = "localLogErrorsMaxLength", default, skip_serializing_if = "Option::is_none")]
        pub local_log_errors_max_length: Option<String>,
        #[doc = "Master log_errors_max_len setting."]
        #[serde(rename = "masterLogErrorsMaxLength", default, skip_serializing_if = "Option::is_none")]
        pub master_log_errors_max_length: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Site seal"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteSeal {
    #[doc = "HTML snippet"]
    pub html: String,
}
impl SiteSeal {
    pub fn new(html: String) -> Self {
        Self { html }
    }
}
#[doc = "Site seal request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteSealRequest {
    #[doc = "If <code>true</code> use the light color theme for site seal; otherwise, use the default color theme."]
    #[serde(rename = "lightTheme", default, skip_serializing_if = "Option::is_none")]
    pub light_theme: Option<bool>,
    #[doc = "Locale of site seal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}
impl SiteSealRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Source control configuration for an app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteSourceControl {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SiteSourceControl resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<site_source_control::Properties>,
}
impl SiteSourceControl {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod site_source_control {
    use super::*;
    #[doc = "SiteSourceControl resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Repository or source control URL."]
        #[serde(rename = "repoUrl", default, skip_serializing_if = "Option::is_none")]
        pub repo_url: Option<String>,
        #[doc = "Name of branch to use for deployment."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub branch: Option<String>,
        #[doc = "<code>true</code> to limit to manual integration; <code>false</code> to enable continuous integration (which configures webhooks into online repos like GitHub)."]
        #[serde(rename = "isManualIntegration", default, skip_serializing_if = "Option::is_none")]
        pub is_manual_integration: Option<bool>,
        #[doc = "<code>true</code> if this is deployed via GitHub action."]
        #[serde(rename = "isGitHubAction", default, skip_serializing_if = "Option::is_none")]
        pub is_git_hub_action: Option<bool>,
        #[doc = "<code>true</code> to enable deployment rollback; otherwise, <code>false</code>."]
        #[serde(rename = "deploymentRollbackEnabled", default, skip_serializing_if = "Option::is_none")]
        pub deployment_rollback_enabled: Option<bool>,
        #[doc = "<code>true</code> for a Mercurial repository; <code>false</code> for a Git repository."]
        #[serde(rename = "isMercurial", default, skip_serializing_if = "Option::is_none")]
        pub is_mercurial: Option<bool>,
        #[doc = "The GitHub action configuration."]
        #[serde(rename = "gitHubActionConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub git_hub_action_configuration: Option<GitHubActionConfiguration>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Description of the App Service plan scale options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapacity {
    #[doc = "Minimum number of workers for this App Service plan SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[doc = "Maximum number of workers for this App Service plan SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[doc = "Maximum number of Elastic workers for this App Service plan SKU."]
    #[serde(rename = "elasticMaximum", default, skip_serializing_if = "Option::is_none")]
    pub elastic_maximum: Option<i32>,
    #[doc = "Default number of workers for this App Service plan SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "Available scale configurations for an App Service plan."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<String>,
}
impl SkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a SKU for a scalable resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDescription {
    #[doc = "Name of the resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Service tier of the resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Size specifier of the resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "Family code of the resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "Current number of instances assigned to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[doc = "Description of the App Service plan scale options."]
    #[serde(rename = "skuCapacity", default, skip_serializing_if = "Option::is_none")]
    pub sku_capacity: Option<SkuCapacity>,
    #[doc = "Locations of the SKU."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Capabilities of the SKU, e.g., is traffic manager enabled?"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
}
impl SkuDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU discovery information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuInfo {
    #[doc = "Resource type that this SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Description of a SKU for a scalable resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuDescription>,
    #[doc = "Description of the App Service plan scale options."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
}
impl SkuInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of SKU information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuInfoCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<SkuInfo>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SkuInfoCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SkuInfoCollection {
    pub fn new(value: Vec<SkuInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Collection of SKU information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuInfos {
    #[doc = "Resource type that this SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "List of SKUs the subscription is able to use."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skus: Vec<GlobalCsmSkuDescription>,
}
impl SkuInfos {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Names for connection strings, application settings, and external Azure storage account configuration\nidentifiers to be marked as sticky to the deployment slot and not moved during a swap operation.\nThis is valid for all deployment slots in an app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SlotConfigNames {
    #[doc = "List of connection string names."]
    #[serde(rename = "connectionStringNames", default, skip_serializing_if = "Vec::is_empty")]
    pub connection_string_names: Vec<String>,
    #[doc = "List of application settings names."]
    #[serde(rename = "appSettingNames", default, skip_serializing_if = "Vec::is_empty")]
    pub app_setting_names: Vec<String>,
    #[doc = "List of external Azure storage account identifiers."]
    #[serde(rename = "azureStorageConfigNames", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_storage_config_names: Vec<String>,
}
impl SlotConfigNames {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Slot Config names azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SlotConfigNamesResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Names for connection strings, application settings, and external Azure storage account configuration\nidentifiers to be marked as sticky to the deployment slot and not moved during a swap operation.\nThis is valid for all deployment slots in an app."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SlotConfigNames>,
}
impl SlotConfigNamesResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A setting difference between two deployment slots of an app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SlotDifference {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SlotDifference resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<slot_difference::Properties>,
}
impl SlotDifference {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod slot_difference {
    use super::*;
    #[doc = "SlotDifference resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Level of the difference: Information, Warning or Error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<String>,
        #[doc = "The type of the setting: General, AppSetting or ConnectionString."]
        #[serde(rename = "settingType", default, skip_serializing_if = "Option::is_none")]
        pub setting_type: Option<String>,
        #[doc = "Rule that describes how to process the setting difference during a slot swap."]
        #[serde(rename = "diffRule", default, skip_serializing_if = "Option::is_none")]
        pub diff_rule: Option<String>,
        #[doc = "Name of the setting."]
        #[serde(rename = "settingName", default, skip_serializing_if = "Option::is_none")]
        pub setting_name: Option<String>,
        #[doc = "Value of the setting in the current slot."]
        #[serde(rename = "valueInCurrentSlot", default, skip_serializing_if = "Option::is_none")]
        pub value_in_current_slot: Option<String>,
        #[doc = "Value of the setting in the target slot."]
        #[serde(rename = "valueInTargetSlot", default, skip_serializing_if = "Option::is_none")]
        pub value_in_target_slot: Option<String>,
        #[doc = "Description of the setting difference."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of slot differences."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlotDifferenceCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<SlotDifference>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SlotDifferenceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SlotDifferenceCollection {
    pub fn new(value: Vec<SlotDifference>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The status of the last successful slot swap operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SlotSwapStatus {
    #[doc = "The time the last successful slot swap completed."]
    #[serde(rename = "timestampUtc", default, with = "azure_core::date::rfc3339::option")]
    pub timestamp_utc: Option<time::OffsetDateTime>,
    #[doc = "The source slot of the last swap operation."]
    #[serde(rename = "sourceSlotName", default, skip_serializing_if = "Option::is_none")]
    pub source_slot_name: Option<String>,
    #[doc = "The destination slot of the last swap operation."]
    #[serde(rename = "destinationSlotName", default, skip_serializing_if = "Option::is_none")]
    pub destination_slot_name: Option<String>,
}
impl SlotSwapStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Trigger based on request execution time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SlowRequestsBasedTrigger {
    #[doc = "Time taken."]
    #[serde(rename = "timeTaken", default, skip_serializing_if = "Option::is_none")]
    pub time_taken: Option<String>,
    #[doc = "Request Path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Request Count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Time interval."]
    #[serde(rename = "timeInterval", default, skip_serializing_if = "Option::is_none")]
    pub time_interval: Option<String>,
}
impl SlowRequestsBasedTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A snapshot of an app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Snapshot {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Snapshot resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<snapshot::Properties>,
}
impl Snapshot {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod snapshot {
    use super::*;
    #[doc = "Snapshot resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The time the snapshot was taken."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub time: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of snapshots which can be used to revert an app to a previous time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Snapshot>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SnapshotCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SnapshotCollection {
    pub fn new(value: Vec<Snapshot>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Specifies the web app that snapshot contents will be retrieved from."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotRecoverySource {
    #[doc = "Geographical location of the source web app, e.g. SouthEastAsia, SouthCentralUS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "ARM resource ID of the source app. \n/subscriptions/{subId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{siteName} for production slots and \n/subscriptions/{subId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{siteName}/slots/{slotName} for other slots."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SnapshotRecoverySource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about app recovery operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotRestoreRequest {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SnapshotRestoreRequest resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<snapshot_restore_request::Properties>,
}
impl SnapshotRestoreRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod snapshot_restore_request {
    use super::*;
    #[doc = "SnapshotRestoreRequest resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Point in time in which the app restore should be done, formatted as a DateTime string."]
        #[serde(rename = "snapshotTime", default, skip_serializing_if = "Option::is_none")]
        pub snapshot_time: Option<String>,
        #[doc = "Specifies the web app that snapshot contents will be retrieved from."]
        #[serde(rename = "recoverySource", default, skip_serializing_if = "Option::is_none")]
        pub recovery_source: Option<SnapshotRecoverySource>,
        #[doc = "If <code>true</code> the restore operation can overwrite source app; otherwise, <code>false</code>."]
        pub overwrite: bool,
        #[doc = "If true, site configuration, in addition to content, will be reverted."]
        #[serde(rename = "recoverConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub recover_configuration: Option<bool>,
        #[doc = "If true, custom hostname conflicts will be ignored when recovering to a target web app.\nThis setting is only necessary when RecoverConfiguration is enabled."]
        #[serde(rename = "ignoreConflictingHostNames", default, skip_serializing_if = "Option::is_none")]
        pub ignore_conflicting_host_names: Option<bool>,
        #[doc = "If true, the snapshot is retrieved from DRSecondary endpoint."]
        #[serde(rename = "useDRSecondary", default, skip_serializing_if = "Option::is_none")]
        pub use_dr_secondary: Option<bool>,
    }
    impl Properties {
        pub fn new(overwrite: bool) -> Self {
            Self {
                snapshot_time: None,
                recovery_source: None,
                overwrite,
                recover_configuration: None,
                ignore_conflicting_host_names: None,
                use_dr_secondary: None,
            }
        }
    }
}
#[doc = "Class Representing Solution for problems detected."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Solution {
    #[doc = "Solution Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "Display Name of the solution"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Order of the solution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<f64>,
    #[doc = "Description of the solution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Type of Solution"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<solution::Type>,
    #[doc = "Solution Data."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Vec<NameValuePair>>,
    #[doc = "Solution Metadata."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<Vec<NameValuePair>>,
}
impl Solution {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod solution {
    use super::*;
    #[doc = "Type of Solution"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        QuickSolution,
        DeepInvestigation,
        BestPractices,
    }
}
#[doc = "The source control OAuth token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControl {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SourceControl resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<source_control::Properties>,
}
impl SourceControl {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_control {
    use super::*;
    #[doc = "SourceControl resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "OAuth access token."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
        #[doc = "OAuth access token secret."]
        #[serde(rename = "tokenSecret", default, skip_serializing_if = "Option::is_none")]
        pub token_secret: Option<String>,
        #[doc = "OAuth refresh token."]
        #[serde(rename = "refreshToken", default, skip_serializing_if = "Option::is_none")]
        pub refresh_token: Option<String>,
        #[doc = "OAuth token expiration."]
        #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
        pub expiration_time: Option<time::OffsetDateTime>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of source controls."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceControlCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<SourceControl>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SourceControlCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SourceControlCollection {
    pub fn new(value: Vec<SourceControl>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Application stack major version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StackMajorVersion {
    #[doc = "Application stack major version (display only)."]
    #[serde(rename = "displayVersion", default, skip_serializing_if = "Option::is_none")]
    pub display_version: Option<String>,
    #[doc = "Application stack major version (runtime only)."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[doc = "<code>true</code> if this is the default major version; otherwise, <code>false</code>."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Minor versions associated with the major version."]
    #[serde(rename = "minorVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub minor_versions: Vec<StackMinorVersion>,
    #[doc = "<code>true</code> if this supports Application Insights; otherwise, <code>false</code>."]
    #[serde(rename = "applicationInsights", default, skip_serializing_if = "Option::is_none")]
    pub application_insights: Option<bool>,
    #[doc = "<code>true</code> if this stack is in Preview, otherwise <code>false</code>."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[doc = "<code>true</code> if this stack has been deprecated, otherwise <code>false</code>."]
    #[serde(rename = "isDeprecated", default, skip_serializing_if = "Option::is_none")]
    pub is_deprecated: Option<bool>,
    #[doc = "<code>true</code> if this stack should be hidden for new customers on portal, otherwise <code>false</code>."]
    #[serde(rename = "isHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "<appSettings>\n <appSetting name=\"FUNCTIONS_WORKER_RUNTIME\" value=\"dotnet\" />\n</appSettings>\n Example: All the function apps need AppSetting: \"FUNCTIONS_WORKER_RUNTIME\" to be set stack name"]
    #[serde(rename = "appSettingsDictionary", default, skip_serializing_if = "Option::is_none")]
    pub app_settings_dictionary: Option<serde_json::Value>,
    #[doc = "<siteConfigProperties>\n <siteConfigProperty name=\"Use32BitWorkerProcess\" value=\"false\" />\n</siteConfigProperties>\n Example: All Linux Function Apps, need Use32BitWorkerProcess to be set to 0"]
    #[serde(rename = "siteConfigPropertiesDictionary", default, skip_serializing_if = "Option::is_none")]
    pub site_config_properties_dictionary: Option<serde_json::Value>,
}
impl StackMajorVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application stack minor version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StackMinorVersion {
    #[doc = "Application stack minor version (display only)."]
    #[serde(rename = "displayVersion", default, skip_serializing_if = "Option::is_none")]
    pub display_version: Option<String>,
    #[doc = "Application stack minor version (runtime only)."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[doc = "<code>true</code> if this is the default minor version; otherwise, <code>false</code>."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "<code>true</code> if this supports Remote Debugging, otherwise <code>false</code>."]
    #[serde(rename = "isRemoteDebuggingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_remote_debugging_enabled: Option<bool>,
}
impl StackMinorVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stamp capacity information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StampCapacity {
    #[doc = "Name of the stamp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Available capacity (# of machines, bytes of storage etc...)."]
    #[serde(rename = "availableCapacity", default, skip_serializing_if = "Option::is_none")]
    pub available_capacity: Option<i64>,
    #[doc = "Total capacity (# of machines, bytes of storage etc...)."]
    #[serde(rename = "totalCapacity", default, skip_serializing_if = "Option::is_none")]
    pub total_capacity: Option<i64>,
    #[doc = "Name of the unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Shared/dedicated workers."]
    #[serde(rename = "computeMode", default, skip_serializing_if = "Option::is_none")]
    pub compute_mode: Option<stamp_capacity::ComputeMode>,
    #[doc = "Size of the machines."]
    #[serde(rename = "workerSize", default, skip_serializing_if = "Option::is_none")]
    pub worker_size: Option<stamp_capacity::WorkerSize>,
    #[doc = "Size ID of machines: \n0 - Small\n1 - Medium\n2 - Large"]
    #[serde(rename = "workerSizeId", default, skip_serializing_if = "Option::is_none")]
    pub worker_size_id: Option<i32>,
    #[doc = "If <code>true</code>, it includes basic apps.\nBasic apps are not used for capacity allocation."]
    #[serde(rename = "excludeFromCapacityAllocation", default, skip_serializing_if = "Option::is_none")]
    pub exclude_from_capacity_allocation: Option<bool>,
    #[doc = "<code>true</code> if capacity is applicable for all apps; otherwise, <code>false</code>."]
    #[serde(rename = "isApplicableForAllComputeModes", default, skip_serializing_if = "Option::is_none")]
    pub is_applicable_for_all_compute_modes: Option<bool>,
    #[doc = "Shared or Dedicated."]
    #[serde(rename = "siteMode", default, skip_serializing_if = "Option::is_none")]
    pub site_mode: Option<String>,
    #[doc = "Is this a linux stamp capacity"]
    #[serde(rename = "isLinux", default, skip_serializing_if = "Option::is_none")]
    pub is_linux: Option<bool>,
}
impl StampCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod stamp_capacity {
    use super::*;
    #[doc = "Shared/dedicated workers."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComputeMode {
        Shared,
        Dedicated,
        Dynamic,
    }
    #[doc = "Size of the machines."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkerSize {
        Small,
        Medium,
        Large,
        D1,
        D2,
        D3,
        SmallV3,
        MediumV3,
        LargeV3,
        NestedSmall,
        NestedSmallLinux,
        Default,
    }
}
#[doc = "Collection of stamp capacities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StampCapacityCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<StampCapacity>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StampCapacityCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StampCapacityCollection {
    pub fn new(value: Vec<StampCapacity>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A static site."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSite {
    #[doc = "The default autogenerated hostname for the static site."]
    #[serde(rename = "defaultHostname", default, skip_serializing_if = "Option::is_none")]
    pub default_hostname: Option<String>,
    #[doc = "URL for the repository of the static site."]
    #[serde(rename = "repositoryUrl", default, skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[doc = "The target branch in the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "The custom domains associated with this static site."]
    #[serde(rename = "customDomains", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_domains: Vec<String>,
    #[doc = "A user's github repository token. This is used to setup the Github Actions workflow file and API secrets."]
    #[serde(rename = "repositoryToken", default, skip_serializing_if = "Option::is_none")]
    pub repository_token: Option<String>,
    #[doc = "Build properties for the static site."]
    #[serde(rename = "buildProperties", default, skip_serializing_if = "Option::is_none")]
    pub build_properties: Option<StaticSiteBuildProperties>,
    #[doc = "Private endpoint connections"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<ResponseMessageEnvelopeRemotePrivateEndpointConnection>,
    #[doc = "State indicating whether staging environments are allowed or not allowed for a static web app."]
    #[serde(rename = "stagingEnvironmentPolicy", default, skip_serializing_if = "Option::is_none")]
    pub staging_environment_policy: Option<static_site::StagingEnvironmentPolicy>,
    #[doc = "<code>false</code> if config file is locked for this static web app; otherwise, <code>true</code>."]
    #[serde(rename = "allowConfigFileUpdates", default, skip_serializing_if = "Option::is_none")]
    pub allow_config_file_updates: Option<bool>,
    #[doc = "Template Options for the static site."]
    #[serde(rename = "templateProperties", default, skip_serializing_if = "Option::is_none")]
    pub template_properties: Option<StaticSiteTemplateOptions>,
    #[doc = "The content distribution endpoint for the static site."]
    #[serde(rename = "contentDistributionEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub content_distribution_endpoint: Option<String>,
    #[doc = "Identity to use for Key Vault Reference authentication."]
    #[serde(rename = "keyVaultReferenceIdentity", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_reference_identity: Option<String>,
    #[doc = "User provided function apps registered with the static site"]
    #[serde(rename = "userProvidedFunctionApps", default, skip_serializing_if = "Vec::is_empty")]
    pub user_provided_function_apps: Vec<StaticSiteUserProvidedFunctionApp>,
    #[doc = "The provider that submitted the last deployment to the primary environment of the static site."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl StaticSite {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site {
    use super::*;
    #[doc = "State indicating whether staging environments are allowed or not allowed for a static web app."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StagingEnvironmentPolicy {
        Enabled,
        Disabled,
    }
}
#[doc = "Static Site ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticSiteArmResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A static site."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StaticSite>,
    #[doc = "Description of a SKU for a scalable resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuDescription>,
    #[doc = "Managed service identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl StaticSiteArmResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            sku: None,
            identity: None,
        }
    }
}
#[doc = "Static Site Build ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteBuildArmResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSiteBuildARMResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_site_build_arm_resource::Properties>,
}
impl StaticSiteBuildArmResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site_build_arm_resource {
    use super::*;
    #[doc = "StaticSiteBuildARMResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "An identifier for the static site build."]
        #[serde(rename = "buildId", default, skip_serializing_if = "Option::is_none")]
        pub build_id: Option<String>,
        #[doc = "The source branch."]
        #[serde(rename = "sourceBranch", default, skip_serializing_if = "Option::is_none")]
        pub source_branch: Option<String>,
        #[doc = "The title of a pull request that a static site build is related to."]
        #[serde(rename = "pullRequestTitle", default, skip_serializing_if = "Option::is_none")]
        pub pull_request_title: Option<String>,
        #[doc = "The hostname for a static site build."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hostname: Option<String>,
        #[doc = "When this build was created."]
        #[serde(rename = "createdTimeUtc", default, with = "azure_core::date::rfc3339::option")]
        pub created_time_utc: Option<time::OffsetDateTime>,
        #[doc = "When this build was updated."]
        #[serde(rename = "lastUpdatedOn", default, with = "azure_core::date::rfc3339::option")]
        pub last_updated_on: Option<time::OffsetDateTime>,
        #[doc = "The status of the static site build."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[doc = "User provided function apps registered with the static site build"]
        #[serde(rename = "userProvidedFunctionApps", default, skip_serializing_if = "Vec::is_empty")]
        pub user_provided_function_apps: Vec<StaticSiteUserProvidedFunctionApp>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The status of the static site build."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Status")]
        pub enum Status {
            WaitingForDeployment,
            Uploading,
            Deploying,
            Ready,
            Failed,
            Deleting,
            Detached,
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
                    Self::WaitingForDeployment => serializer.serialize_unit_variant("Status", 0u32, "WaitingForDeployment"),
                    Self::Uploading => serializer.serialize_unit_variant("Status", 1u32, "Uploading"),
                    Self::Deploying => serializer.serialize_unit_variant("Status", 2u32, "Deploying"),
                    Self::Ready => serializer.serialize_unit_variant("Status", 3u32, "Ready"),
                    Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                    Self::Deleting => serializer.serialize_unit_variant("Status", 5u32, "Deleting"),
                    Self::Detached => serializer.serialize_unit_variant("Status", 6u32, "Detached"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Collection of static site builds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticSiteBuildCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<StaticSiteBuildArmResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StaticSiteBuildCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StaticSiteBuildCollection {
    pub fn new(value: Vec<StaticSiteBuildArmResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Build properties for the static site."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteBuildProperties {
    #[doc = "The path to the app code within the repository."]
    #[serde(rename = "appLocation", default, skip_serializing_if = "Option::is_none")]
    pub app_location: Option<String>,
    #[doc = "The path to the api code within the repository."]
    #[serde(rename = "apiLocation", default, skip_serializing_if = "Option::is_none")]
    pub api_location: Option<String>,
    #[doc = "Deprecated: The path of the app artifacts after building (deprecated in favor of OutputLocation)"]
    #[serde(rename = "appArtifactLocation", default, skip_serializing_if = "Option::is_none")]
    pub app_artifact_location: Option<String>,
    #[doc = "The output path of the app after building."]
    #[serde(rename = "outputLocation", default, skip_serializing_if = "Option::is_none")]
    pub output_location: Option<String>,
    #[doc = "A custom command to run during deployment of the static content application."]
    #[serde(rename = "appBuildCommand", default, skip_serializing_if = "Option::is_none")]
    pub app_build_command: Option<String>,
    #[doc = "A custom command to run during deployment of the Azure Functions API application."]
    #[serde(rename = "apiBuildCommand", default, skip_serializing_if = "Option::is_none")]
    pub api_build_command: Option<String>,
    #[doc = "Skip Github Action workflow generation."]
    #[serde(rename = "skipGithubActionWorkflowGeneration", default, skip_serializing_if = "Option::is_none")]
    pub skip_github_action_workflow_generation: Option<bool>,
    #[doc = "Github Action secret name override."]
    #[serde(rename = "githubActionSecretNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub github_action_secret_name_override: Option<String>,
}
impl StaticSiteBuildProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of static sites."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticSiteCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<StaticSiteArmResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StaticSiteCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StaticSiteCollection {
    pub fn new(value: Vec<StaticSiteArmResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Static Site Custom Domain Overview ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteCustomDomainOverviewArmResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSiteCustomDomainOverviewARMResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_site_custom_domain_overview_arm_resource::Properties>,
}
impl StaticSiteCustomDomainOverviewArmResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site_custom_domain_overview_arm_resource {
    use super::*;
    #[doc = "StaticSiteCustomDomainOverviewARMResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The domain name for the static site custom domain."]
        #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
        pub domain_name: Option<String>,
        #[doc = "The date and time on which the custom domain was created for the static site."]
        #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
        pub created_on: Option<time::OffsetDateTime>,
        #[doc = "The status of the custom domain"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[doc = "The TXT record validation token"]
        #[serde(rename = "validationToken", default, skip_serializing_if = "Option::is_none")]
        pub validation_token: Option<String>,
        #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The status of the custom domain"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Status")]
        pub enum Status {
            RetrievingValidationToken,
            Validating,
            Adding,
            Ready,
            Failed,
            Deleting,
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
                    Self::RetrievingValidationToken => serializer.serialize_unit_variant("Status", 0u32, "RetrievingValidationToken"),
                    Self::Validating => serializer.serialize_unit_variant("Status", 1u32, "Validating"),
                    Self::Adding => serializer.serialize_unit_variant("Status", 2u32, "Adding"),
                    Self::Ready => serializer.serialize_unit_variant("Status", 3u32, "Ready"),
                    Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                    Self::Deleting => serializer.serialize_unit_variant("Status", 5u32, "Deleting"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Collection of static site custom domains."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticSiteCustomDomainOverviewCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<StaticSiteCustomDomainOverviewArmResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StaticSiteCustomDomainOverviewCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StaticSiteCustomDomainOverviewCollection {
    pub fn new(value: Vec<StaticSiteCustomDomainOverviewArmResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Static Site Custom Domain Request Properties ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteCustomDomainRequestPropertiesArmResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSiteCustomDomainRequestPropertiesARMResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_site_custom_domain_request_properties_arm_resource::Properties>,
}
impl StaticSiteCustomDomainRequestPropertiesArmResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site_custom_domain_request_properties_arm_resource {
    use super::*;
    #[doc = "StaticSiteCustomDomainRequestPropertiesARMResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Validation method for adding a custom domain"]
        #[serde(rename = "validationMethod", default, skip_serializing_if = "Option::is_none")]
        pub validation_method: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Static Site Function Overview ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteFunctionOverviewArmResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSiteFunctionOverviewARMResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_site_function_overview_arm_resource::Properties>,
}
impl StaticSiteFunctionOverviewArmResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site_function_overview_arm_resource {
    use super::*;
    #[doc = "StaticSiteFunctionOverviewARMResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The name for the function"]
        #[serde(rename = "functionName", default, skip_serializing_if = "Option::is_none")]
        pub function_name: Option<String>,
        #[doc = "The trigger type of the function"]
        #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
        pub trigger_type: Option<properties::TriggerType>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The trigger type of the function"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "TriggerType")]
        pub enum TriggerType {
            HttpTrigger,
            Unknown,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for TriggerType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for TriggerType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for TriggerType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::HttpTrigger => serializer.serialize_unit_variant("TriggerType", 0u32, "HttpTrigger"),
                    Self::Unknown => serializer.serialize_unit_variant("TriggerType", 1u32, "Unknown"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Collection of static site functions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticSiteFunctionOverviewCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<StaticSiteFunctionOverviewArmResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StaticSiteFunctionOverviewCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StaticSiteFunctionOverviewCollection {
    pub fn new(value: Vec<StaticSiteFunctionOverviewArmResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "ARM resource for a static site when patching"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSitePatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "A static site."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StaticSite>,
}
impl StaticSitePatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Static Site Reset Properties ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteResetPropertiesArmResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSiteResetPropertiesARMResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_site_reset_properties_arm_resource::Properties>,
}
impl StaticSiteResetPropertiesArmResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site_reset_properties_arm_resource {
    use super::*;
    #[doc = "StaticSiteResetPropertiesARMResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The token which proves admin privileges to the repository."]
        #[serde(rename = "repositoryToken", default, skip_serializing_if = "Option::is_none")]
        pub repository_token: Option<String>,
        #[doc = "Determines whether the repository should be updated with the new properties."]
        #[serde(rename = "shouldUpdateRepository", default, skip_serializing_if = "Option::is_none")]
        pub should_update_repository: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Template Options for the static site."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteTemplateOptions {
    #[doc = "URL of the template repository. The newly generated repository will be based on this one."]
    #[serde(rename = "templateRepositoryUrl", default, skip_serializing_if = "Option::is_none")]
    pub template_repository_url: Option<String>,
    #[doc = "Owner of the newly generated repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "Name of the newly generated repository."]
    #[serde(rename = "repositoryName", default, skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[doc = "Description of the newly generated repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether or not the newly generated repository is a private repository. Defaults to false (i.e. public)."]
    #[serde(rename = "isPrivate", default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}
impl StaticSiteTemplateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Static Site User ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteUserArmResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSiteUserARMResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_site_user_arm_resource::Properties>,
}
impl StaticSiteUserArmResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site_user_arm_resource {
    use super::*;
    #[doc = "StaticSiteUserARMResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The identity provider for the static site user."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The user id for the static site user."]
        #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
        pub user_id: Option<String>,
        #[doc = "The display name for the static site user."]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "The roles for the static site user, in free-form string format"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub roles: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of static site custom users."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticSiteUserCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<StaticSiteUserArmResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StaticSiteUserCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StaticSiteUserCollection {
    pub fn new(value: Vec<StaticSiteUserArmResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Static sites user roles invitation resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteUserInvitationRequestResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSiteUserInvitationRequestResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_site_user_invitation_request_resource::Properties>,
}
impl StaticSiteUserInvitationRequestResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site_user_invitation_request_resource {
    use super::*;
    #[doc = "StaticSiteUserInvitationRequestResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The domain name for the static site custom domain."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<String>,
        #[doc = "The identity provider for the static site user."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The user id for the static site user."]
        #[serde(rename = "userDetails", default, skip_serializing_if = "Option::is_none")]
        pub user_details: Option<String>,
        #[doc = "The roles for the static site user, in free-form string format"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub roles: Option<String>,
        #[doc = "The number of hours the sas token stays valid"]
        #[serde(rename = "numHoursToExpiration", default, skip_serializing_if = "Option::is_none")]
        pub num_hours_to_expiration: Option<i32>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Static sites user roles invitation link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteUserInvitationResponseResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSiteUserInvitationResponseResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_site_user_invitation_response_resource::Properties>,
}
impl StaticSiteUserInvitationResponseResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site_user_invitation_response_resource {
    use super::*;
    #[doc = "StaticSiteUserInvitationResponseResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The expiration time of the invitation"]
        #[serde(rename = "expiresOn", default, with = "azure_core::date::rfc3339::option")]
        pub expires_on: Option<time::OffsetDateTime>,
        #[doc = "The url for the invitation link"]
        #[serde(rename = "invitationUrl", default, skip_serializing_if = "Option::is_none")]
        pub invitation_url: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A static site user provided function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteUserProvidedFunctionApp {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSiteUserProvidedFunctionApp resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_site_user_provided_function_app::Properties>,
}
impl StaticSiteUserProvidedFunctionApp {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site_user_provided_function_app {
    use super::*;
    #[doc = "StaticSiteUserProvidedFunctionApp resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The resource id of the function app registered with the static site"]
        #[serde(rename = "functionAppResourceId", default, skip_serializing_if = "Option::is_none")]
        pub function_app_resource_id: Option<String>,
        #[doc = "The region of the function app registered with the static site"]
        #[serde(rename = "functionAppRegion", default, skip_serializing_if = "Option::is_none")]
        pub function_app_region: Option<String>,
        #[doc = "The date and time on which the function app was registered with the static site."]
        #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
        pub created_on: Option<time::OffsetDateTime>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Static Site User Provided Function App ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteUserProvidedFunctionAppArmResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSiteUserProvidedFunctionAppARMResource resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_site_user_provided_function_app_arm_resource::Properties>,
}
impl StaticSiteUserProvidedFunctionAppArmResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_site_user_provided_function_app_arm_resource {
    use super::*;
    #[doc = "StaticSiteUserProvidedFunctionAppARMResource resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The resource id of the function app registered with the static site"]
        #[serde(rename = "functionAppResourceId", default, skip_serializing_if = "Option::is_none")]
        pub function_app_resource_id: Option<String>,
        #[doc = "The region of the function app registered with the static site"]
        #[serde(rename = "functionAppRegion", default, skip_serializing_if = "Option::is_none")]
        pub function_app_region: Option<String>,
        #[doc = "The date and time on which the function app was registered with the static site."]
        #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
        pub created_on: Option<time::OffsetDateTime>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of static site user provided function apps."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticSiteUserProvidedFunctionAppsCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<StaticSiteUserProvidedFunctionAppArmResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StaticSiteUserProvidedFunctionAppsCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StaticSiteUserProvidedFunctionAppsCollection {
    pub fn new(value: Vec<StaticSiteUserProvidedFunctionAppArmResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A static site zip deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteZipDeployment {
    #[doc = "URL for the zipped app content"]
    #[serde(rename = "appZipUrl", default, skip_serializing_if = "Option::is_none")]
    pub app_zip_url: Option<String>,
    #[doc = "URL for the zipped api content"]
    #[serde(rename = "apiZipUrl", default, skip_serializing_if = "Option::is_none")]
    pub api_zip_url: Option<String>,
    #[doc = "A title to label the deployment"]
    #[serde(rename = "deploymentTitle", default, skip_serializing_if = "Option::is_none")]
    pub deployment_title: Option<String>,
    #[doc = "The provider submitting this deployment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The language of the api content, if it exists"]
    #[serde(rename = "functionLanguage", default, skip_serializing_if = "Option::is_none")]
    pub function_language: Option<String>,
}
impl StaticSiteZipDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Static site zip deployment ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSiteZipDeploymentArmResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "A static site zip deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StaticSiteZipDeployment>,
}
impl StaticSiteZipDeploymentArmResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Preview for the Static Site Workflow to be generated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSitesWorkflowPreview {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSitesWorkflowPreview resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_sites_workflow_preview::Properties>,
}
impl StaticSitesWorkflowPreview {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_sites_workflow_preview {
    use super::*;
    #[doc = "StaticSitesWorkflowPreview resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The path for the workflow file to be generated"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[doc = "The contents for the workflow file to be generated"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub contents: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Request entity for previewing the Static Site workflow"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticSitesWorkflowPreviewRequest {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StaticSitesWorkflowPreviewRequest resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<static_sites_workflow_preview_request::Properties>,
}
impl StaticSitesWorkflowPreviewRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod static_sites_workflow_preview_request {
    use super::*;
    #[doc = "StaticSitesWorkflowPreviewRequest resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "URL for the repository of the static site."]
        #[serde(rename = "repositoryUrl", default, skip_serializing_if = "Option::is_none")]
        pub repository_url: Option<String>,
        #[doc = "The target branch in the repository."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub branch: Option<String>,
        #[doc = "Build properties for the static site."]
        #[serde(rename = "buildProperties", default, skip_serializing_if = "Option::is_none")]
        pub build_properties: Option<StaticSiteBuildProperties>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Identify the status of the most severe insight generated by the detector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Status {
    #[doc = "Descriptive message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Level of the most severe insight generated by the detector."]
    #[serde(rename = "statusId", default, skip_serializing_if = "Option::is_none")]
    pub status_id: Option<status::StatusId>,
}
impl Status {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod status {
    use super::*;
    #[doc = "Level of the most severe insight generated by the detector."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusId {
        Critical,
        Warning,
        Info,
        Success,
        None,
    }
}
#[doc = "Trigger based on status code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusCodesBasedTrigger {
    #[doc = "HTTP status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[doc = "Request Sub Status."]
    #[serde(rename = "subStatus", default, skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<i32>,
    #[doc = "Win32 error code."]
    #[serde(rename = "win32Status", default, skip_serializing_if = "Option::is_none")]
    pub win32_status: Option<i32>,
    #[doc = "Request Count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Time interval."]
    #[serde(rename = "timeInterval", default, skip_serializing_if = "Option::is_none")]
    pub time_interval: Option<String>,
    #[doc = "Request Path"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl StatusCodesBasedTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Trigger based on range of status codes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusCodesRangeBasedTrigger {
    #[doc = "HTTP status code."]
    #[serde(rename = "statusCodes", default, skip_serializing_if = "Option::is_none")]
    pub status_codes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Request Count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Time interval."]
    #[serde(rename = "timeInterval", default, skip_serializing_if = "Option::is_none")]
    pub time_interval: Option<String>,
}
impl StatusCodesRangeBasedTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Options for app content migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageMigrationOptions {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StorageMigrationOptions resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<storage_migration_options::Properties>,
}
impl StorageMigrationOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_migration_options {
    use super::*;
    #[doc = "StorageMigrationOptions resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "AzureFiles connection string."]
        #[serde(rename = "azurefilesConnectionString")]
        pub azurefiles_connection_string: String,
        #[doc = "AzureFiles share."]
        #[serde(rename = "azurefilesShare")]
        pub azurefiles_share: String,
        #[doc = "<code>true</code>if the app should be switched over; otherwise, <code>false</code>."]
        #[serde(rename = "switchSiteAfterMigration", default, skip_serializing_if = "Option::is_none")]
        pub switch_site_after_migration: Option<bool>,
        #[doc = "<code>true</code> if the app should be read only during copy operation; otherwise, <code>false</code>."]
        #[serde(rename = "blockWriteAccessToSite", default, skip_serializing_if = "Option::is_none")]
        pub block_write_access_to_site: Option<bool>,
    }
    impl Properties {
        pub fn new(azurefiles_connection_string: String, azurefiles_share: String) -> Self {
            Self {
                azurefiles_connection_string,
                azurefiles_share,
                switch_site_after_migration: None,
                block_write_access_to_site: None,
            }
        }
    }
}
#[doc = "Response for a migration of app content request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageMigrationResponse {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "StorageMigrationResponse resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<storage_migration_response::Properties>,
}
impl StorageMigrationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_migration_response {
    use super::*;
    #[doc = "StorageMigrationResponse resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "When server starts the migration process, it will return an operation ID identifying that particular migration operation."]
        #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
        pub operation_id: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "String dictionary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StringDictionary {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl StringDictionary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "String list resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StringList {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "List of string resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<String>,
}
impl StringList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines a unique Support Topic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportTopic {
    #[doc = "Support Topic Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Unique resource Id"]
    #[serde(rename = "pesId", default, skip_serializing_if = "Option::is_none")]
    pub pes_id: Option<String>,
}
impl SupportTopic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Swift Virtual Network Contract. This is used to enable the new Swift way of doing virtual network integration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwiftVirtualNetwork {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "SwiftVirtualNetwork resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<swift_virtual_network::Properties>,
}
impl SwiftVirtualNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod swift_virtual_network {
    use super::*;
    #[doc = "SwiftVirtualNetwork resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The Virtual Network subnet's resource ID. This is the subnet that this Web App will join. This subnet must have a delegation to Microsoft.Web/serverFarms defined first."]
        #[serde(rename = "subnetResourceId", default, skip_serializing_if = "Option::is_none")]
        pub subnet_resource_id: Option<String>,
        #[doc = "A flag that specifies if the scale unit this Web App is on supports Swift integration."]
        #[serde(rename = "swiftSupported", default, skip_serializing_if = "Option::is_none")]
        pub swift_supported: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Legal agreement for a top level domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TldLegalAgreement {
    #[doc = "Unique identifier for the agreement."]
    #[serde(rename = "agreementKey")]
    pub agreement_key: String,
    #[doc = "Agreement title."]
    pub title: String,
    #[doc = "Agreement details."]
    pub content: String,
    #[doc = "URL where a copy of the agreement details is hosted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl TldLegalAgreement {
    pub fn new(agreement_key: String, title: String, content: String) -> Self {
        Self {
            agreement_key,
            title,
            content,
            url: None,
        }
    }
}
#[doc = "Collection of top-level domain legal agreements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TldLegalAgreementCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<TldLegalAgreement>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TldLegalAgreementCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TldLegalAgreementCollection {
    pub fn new(value: Vec<TldLegalAgreement>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The configuration settings of the token store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenStore {
    #[doc = "<code>true</code> to durably store platform-specific security tokens that are obtained during login flows; otherwise, <code>false</code>.\n The default is <code>false</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The number of hours after session token expiration that a session token can be used to\ncall the token refresh API. The default is 72 hours."]
    #[serde(rename = "tokenRefreshExtensionHours", default, skip_serializing_if = "Option::is_none")]
    pub token_refresh_extension_hours: Option<f64>,
    #[doc = "The configuration settings of the storage of the tokens if a file system is used."]
    #[serde(rename = "fileSystem", default, skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystemTokenStore>,
    #[doc = "The configuration settings of the storage of the tokens if blob storage is used."]
    #[serde(rename = "azureBlobStorage", default, skip_serializing_if = "Option::is_none")]
    pub azure_blob_storage: Option<BlobStorageTokenStore>,
}
impl TokenStore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A top level domain object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopLevelDomain {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "TopLevelDomain resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<top_level_domain::Properties>,
}
impl TopLevelDomain {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod top_level_domain {
    use super::*;
    #[doc = "TopLevelDomain resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "If <code>true</code>, then the top level domain supports domain privacy; otherwise, <code>false</code>."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub privacy: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Options for retrieving the list of top level domain legal agreements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopLevelDomainAgreementOption {
    #[doc = "If <code>true</code>, then the list of agreements will include agreements for domain privacy as well; otherwise, <code>false</code>."]
    #[serde(rename = "includePrivacy", default, skip_serializing_if = "Option::is_none")]
    pub include_privacy: Option<bool>,
    #[doc = "If <code>true</code>, then the list of agreements will include agreements for domain transfer as well; otherwise, <code>false</code>."]
    #[serde(rename = "forTransfer", default, skip_serializing_if = "Option::is_none")]
    pub for_transfer: Option<bool>,
}
impl TopLevelDomainAgreementOption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Top-level domains."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopLevelDomainCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<TopLevelDomain>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TopLevelDomainCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TopLevelDomainCollection {
    pub fn new(value: Vec<TopLevelDomain>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Triggered Web Job History. List of Triggered Web Job Run Information elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggeredJobHistory {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "TriggeredJobHistory resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<triggered_job_history::Properties>,
}
impl TriggeredJobHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod triggered_job_history {
    use super::*;
    #[doc = "TriggeredJobHistory resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "List of triggered web job runs."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub runs: Vec<TriggeredJobRun>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Kudu continuous web job information elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggeredJobHistoryCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<TriggeredJobHistory>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TriggeredJobHistoryCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TriggeredJobHistoryCollection {
    pub fn new(value: Vec<TriggeredJobHistory>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Triggered Web Job Run Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggeredJobRun {
    #[doc = "Job ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_job_id: Option<String>,
    #[doc = "Job name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_job_name: Option<String>,
    #[doc = "Job status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<triggered_job_run::Status>,
    #[doc = "Start time."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Job duration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Output URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_url: Option<String>,
    #[doc = "Error URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_url: Option<String>,
    #[doc = "Job URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Job name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[doc = "Job trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}
impl TriggeredJobRun {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod triggered_job_run {
    use super::*;
    #[doc = "Job status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Success,
        Failed,
        Error,
    }
}
#[doc = "Triggered Web Job Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggeredWebJob {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "TriggeredWebJob resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<triggered_web_job::Properties>,
}
impl TriggeredWebJob {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod triggered_web_job {
    use super::*;
    #[doc = "TriggeredWebJob resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Triggered Web Job Run Information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub latest_run: Option<TriggeredJobRun>,
        #[doc = "History URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub history_url: Option<String>,
        #[doc = "Scheduler Logs URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scheduler_logs_url: Option<String>,
        #[doc = "Run command."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub run_command: Option<String>,
        #[doc = "Job URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        #[doc = "Extra Info URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extra_info_url: Option<String>,
        #[doc = "Job type."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub web_job_type: Option<properties::WebJobType>,
        #[doc = "Error information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        #[doc = "Using SDK?"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub using_sdk: Option<bool>,
        #[doc = "Job settings."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub settings: Option<serde_json::Value>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Job type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum WebJobType {
            Continuous,
            Triggered,
        }
    }
}
#[doc = "Collection of Kudu continuous web job information elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggeredWebJobCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<TriggeredWebJob>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TriggeredWebJobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TriggeredWebJobCollection {
    pub fn new(value: Vec<TriggeredWebJob>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The configuration settings of the Twitter provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Twitter {
    #[doc = "<code>false</code> if the Twitter provider should not be enabled despite the set registration; otherwise, <code>true</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The configuration settings of the app registration for the Twitter provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<TwitterRegistration>,
}
impl Twitter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the app registration for the Twitter provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TwitterRegistration {
    #[doc = "The OAuth 1.0a consumer key of the Twitter application used for sign-in.\nThis setting is required for enabling Twitter Sign-In.\nTwitter Sign-In documentation: https://dev.twitter.com/web/sign-in"]
    #[serde(rename = "consumerKey", default, skip_serializing_if = "Option::is_none")]
    pub consumer_key: Option<String>,
    #[doc = "The app setting name that contains the OAuth 1.0a consumer secret of the Twitter\napplication used for sign-in."]
    #[serde(rename = "consumerSecretSettingName", default, skip_serializing_if = "Option::is_none")]
    pub consumer_secret_setting_name: Option<String>,
}
impl TwitterRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Usage of the quota resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Usage resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<usage::Properties>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[doc = "Usage resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Friendly name shown in the UI."]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Name of the quota resource."]
        #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
        pub resource_name: Option<String>,
        #[doc = "Units of measurement for the quota resource."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[doc = "The current value of the resource counter."]
        #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
        pub current_value: Option<i64>,
        #[doc = "The resource limit."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        #[doc = "Next reset time for the resource counter."]
        #[serde(rename = "nextResetTime", default, with = "azure_core::date::rfc3339::option")]
        pub next_reset_time: Option<time::OffsetDateTime>,
        #[doc = "Compute mode used for this usage."]
        #[serde(rename = "computeMode", default, skip_serializing_if = "Option::is_none")]
        pub compute_mode: Option<properties::ComputeMode>,
        #[doc = "Site mode used for this usage."]
        #[serde(rename = "siteMode", default, skip_serializing_if = "Option::is_none")]
        pub site_mode: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Compute mode used for this usage."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ComputeMode {
            Shared,
            Dedicated,
            Dynamic,
        }
    }
}
#[doc = "Collection of usages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Usage>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UsageCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UsageCollection {
    pub fn new(value: Vec<Usage>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "User credentials used for publishing activity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct User {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "User resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<user::Properties>,
}
impl User {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod user {
    use super::*;
    #[doc = "User resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Username used for publishing."]
        #[serde(rename = "publishingUserName")]
        pub publishing_user_name: String,
        #[doc = "Password used for publishing."]
        #[serde(rename = "publishingPassword", default, skip_serializing_if = "Option::is_none")]
        pub publishing_password: Option<String>,
        #[doc = "Password hash used for publishing."]
        #[serde(rename = "publishingPasswordHash", default, skip_serializing_if = "Option::is_none")]
        pub publishing_password_hash: Option<String>,
        #[doc = "Password hash salt used for publishing."]
        #[serde(rename = "publishingPasswordHashSalt", default, skip_serializing_if = "Option::is_none")]
        pub publishing_password_hash_salt: Option<String>,
        #[doc = "Url of SCM site."]
        #[serde(rename = "scmUri", default, skip_serializing_if = "Option::is_none")]
        pub scm_uri: Option<String>,
    }
    impl Properties {
        pub fn new(publishing_user_name: String) -> Self {
            Self {
                publishing_user_name,
                publishing_password: None,
                publishing_password_hash: None,
                publishing_password_hash_salt: None,
                scm_uri: None,
            }
        }
    }
}
#[doc = "App properties used for validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateProperties {
    #[doc = "ARM resource ID of an App Service plan that would host the app."]
    #[serde(rename = "serverFarmId", default, skip_serializing_if = "Option::is_none")]
    pub server_farm_id: Option<String>,
    #[doc = "Name of the target SKU for the App Service plan."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[doc = "<code>true</code> if App Service plan is for Linux workers; otherwise, <code>false</code>."]
    #[serde(rename = "needLinuxWorkers", default, skip_serializing_if = "Option::is_none")]
    pub need_linux_workers: Option<bool>,
    #[doc = "<code>true</code> if App Service plan is for Spot instances; otherwise, <code>false</code>."]
    #[serde(rename = "isSpot", default, skip_serializing_if = "Option::is_none")]
    pub is_spot: Option<bool>,
    #[doc = "Target capacity of the App Service plan (number of VMs)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[doc = "Name of App Service Environment where app or App Service plan should be created."]
    #[serde(rename = "hostingEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub hosting_environment: Option<String>,
    #[doc = "<code>true</code> if App Service plan is running as a windows container"]
    #[serde(rename = "isXenon", default, skip_serializing_if = "Option::is_none")]
    pub is_xenon: Option<bool>,
    #[doc = "Base URL of the container registry"]
    #[serde(rename = "containerRegistryBaseUrl", default, skip_serializing_if = "Option::is_none")]
    pub container_registry_base_url: Option<String>,
    #[doc = "Username for to access the container registry"]
    #[serde(rename = "containerRegistryUsername", default, skip_serializing_if = "Option::is_none")]
    pub container_registry_username: Option<String>,
    #[doc = "Password for to access the container registry"]
    #[serde(rename = "containerRegistryPassword", default, skip_serializing_if = "Option::is_none")]
    pub container_registry_password: Option<String>,
    #[doc = "Repository name (image name)"]
    #[serde(rename = "containerImageRepository", default, skip_serializing_if = "Option::is_none")]
    pub container_image_repository: Option<String>,
    #[doc = "Image tag"]
    #[serde(rename = "containerImageTag", default, skip_serializing_if = "Option::is_none")]
    pub container_image_tag: Option<String>,
    #[doc = "Platform (windows or linux)"]
    #[serde(rename = "containerImagePlatform", default, skip_serializing_if = "Option::is_none")]
    pub container_image_platform: Option<String>,
    #[doc = "Description of an App Service Environment."]
    #[serde(rename = "appServiceEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub app_service_environment: Option<AppServiceEnvironment>,
}
impl ValidateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource validation request content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateRequest {
    #[doc = "Resource name to verify."]
    pub name: String,
    #[doc = "Resource type used for verification."]
    #[serde(rename = "type")]
    pub type_: validate_request::Type,
    #[doc = "Expected location of the resource."]
    pub location: String,
    #[doc = "App properties used for validation."]
    pub properties: ValidateProperties,
}
impl ValidateRequest {
    pub fn new(name: String, type_: validate_request::Type, location: String, properties: ValidateProperties) -> Self {
        Self {
            name,
            type_,
            location,
            properties,
        }
    }
}
pub mod validate_request {
    use super::*;
    #[doc = "Resource type used for verification."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        ServerFarm,
        Site,
        #[serde(rename = "Microsoft.Web/hostingEnvironments")]
        MicrosoftWebHostingEnvironments,
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
                Self::ServerFarm => serializer.serialize_unit_variant("Type", 0u32, "ServerFarm"),
                Self::Site => serializer.serialize_unit_variant("Type", 1u32, "Site"),
                Self::MicrosoftWebHostingEnvironments => {
                    serializer.serialize_unit_variant("Type", 2u32, "Microsoft.Web/hostingEnvironments")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the result of resource validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateResponse {
    #[doc = "Result of validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error details for when validation fails."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ValidateResponseError>,
}
impl ValidateResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details for when validation fails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateResponseError {
    #[doc = "Validation error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Validation error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidateResponseError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual application in an app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualApplication {
    #[doc = "Virtual path."]
    #[serde(rename = "virtualPath", default, skip_serializing_if = "Option::is_none")]
    pub virtual_path: Option<String>,
    #[doc = "Physical path."]
    #[serde(rename = "physicalPath", default, skip_serializing_if = "Option::is_none")]
    pub physical_path: Option<String>,
    #[doc = "<code>true</code> if preloading is enabled; otherwise, <code>false</code>."]
    #[serde(rename = "preloadEnabled", default, skip_serializing_if = "Option::is_none")]
    pub preload_enabled: Option<bool>,
    #[doc = "Virtual directories for virtual application."]
    #[serde(rename = "virtualDirectories", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_directories: Vec<VirtualDirectory>,
}
impl VirtualApplication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Directory for virtual application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualDirectory {
    #[doc = "Path to virtual application."]
    #[serde(rename = "virtualPath", default, skip_serializing_if = "Option::is_none")]
    pub virtual_path: Option<String>,
    #[doc = "Physical path."]
    #[serde(rename = "physicalPath", default, skip_serializing_if = "Option::is_none")]
    pub physical_path: Option<String>,
}
impl VirtualDirectory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual IP mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualIpMapping {
    #[doc = "Virtual IP address."]
    #[serde(rename = "virtualIP", default, skip_serializing_if = "Option::is_none")]
    pub virtual_ip: Option<String>,
    #[doc = "Internal HTTP port."]
    #[serde(rename = "internalHttpPort", default, skip_serializing_if = "Option::is_none")]
    pub internal_http_port: Option<i32>,
    #[doc = "Internal HTTPS port."]
    #[serde(rename = "internalHttpsPort", default, skip_serializing_if = "Option::is_none")]
    pub internal_https_port: Option<i32>,
    #[doc = "Is virtual IP mapping in use."]
    #[serde(rename = "inUse", default, skip_serializing_if = "Option::is_none")]
    pub in_use: Option<bool>,
    #[doc = "name of the service that virtual IP is assigned to"]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}
impl VirtualIpMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specification for using a Virtual Network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkProfile {
    #[doc = "Resource id of the Virtual Network."]
    pub id: String,
    #[doc = "Name of the Virtual Network (read-only)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type of the Virtual Network (read-only)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Subnet within the Virtual Network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
}
impl VirtualNetworkProfile {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: None,
            type_: None,
            subnet: None,
        }
    }
}
#[doc = "The Virtual Network gateway contract. This is used to give the Virtual Network gateway access to the VPN package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VnetGateway {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "VnetGateway resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<vnet_gateway::Properties>,
}
impl VnetGateway {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vnet_gateway {
    use super::*;
    #[doc = "VnetGateway resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "The Virtual Network name."]
        #[serde(rename = "vnetName", default, skip_serializing_if = "Option::is_none")]
        pub vnet_name: Option<String>,
        #[doc = "The URI where the VPN package can be downloaded."]
        #[serde(rename = "vpnPackageUri")]
        pub vpn_package_uri: String,
    }
    impl Properties {
        pub fn new(vpn_package_uri: String) -> Self {
            Self {
                vnet_name: None,
                vpn_package_uri,
            }
        }
    }
}
#[doc = "Virtual Network information contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VnetInfo {
    #[doc = "The Virtual Network's resource ID."]
    #[serde(rename = "vnetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vnet_resource_id: Option<String>,
    #[doc = "The client certificate thumbprint."]
    #[serde(rename = "certThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub cert_thumbprint: Option<String>,
    #[doc = "A certificate file (.cer) blob containing the public key of the private key used to authenticate a \nPoint-To-Site VPN connection."]
    #[serde(rename = "certBlob", default, skip_serializing_if = "Option::is_none")]
    pub cert_blob: Option<String>,
    #[doc = "The routes that this Virtual Network connection uses."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<VnetRoute>,
    #[doc = "<code>true</code> if a resync is required; otherwise, <code>false</code>."]
    #[serde(rename = "resyncRequired", default, skip_serializing_if = "Option::is_none")]
    pub resync_required: Option<bool>,
    #[doc = "DNS servers to be used by this Virtual Network. This should be a comma-separated list of IP addresses."]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Option::is_none")]
    pub dns_servers: Option<String>,
    #[doc = "Flag that is used to denote if this is VNET injection"]
    #[serde(rename = "isSwift", default, skip_serializing_if = "Option::is_none")]
    pub is_swift: Option<bool>,
}
impl VnetInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Network information ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VnetInfoResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Virtual Network information contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VnetInfo>,
}
impl VnetInfoResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The required set of inputs to validate a VNET"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VnetParameters {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "VnetParameters resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<vnet_parameters::Properties>,
}
impl VnetParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vnet_parameters {
    use super::*;
    #[doc = "VnetParameters resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The Resource Group of the VNET to be validated"]
        #[serde(rename = "vnetResourceGroup", default, skip_serializing_if = "Option::is_none")]
        pub vnet_resource_group: Option<String>,
        #[doc = "The name of the VNET to be validated"]
        #[serde(rename = "vnetName", default, skip_serializing_if = "Option::is_none")]
        pub vnet_name: Option<String>,
        #[doc = "The subnet name to be validated"]
        #[serde(rename = "vnetSubnetName", default, skip_serializing_if = "Option::is_none")]
        pub vnet_subnet_name: Option<String>,
        #[doc = "The ARM Resource ID of the subnet to validate"]
        #[serde(rename = "subnetResourceId", default, skip_serializing_if = "Option::is_none")]
        pub subnet_resource_id: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Virtual Network route contract used to pass routing information for a Virtual Network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VnetRoute {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "VnetRoute resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<vnet_route::Properties>,
}
impl VnetRoute {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vnet_route {
    use super::*;
    #[doc = "VnetRoute resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The starting address for this route. This may also include a CIDR notation, in which case the end address must not be specified."]
        #[serde(rename = "startAddress", default, skip_serializing_if = "Option::is_none")]
        pub start_address: Option<String>,
        #[doc = "The ending address for this route. If the start address is specified in CIDR notation, this must be omitted."]
        #[serde(rename = "endAddress", default, skip_serializing_if = "Option::is_none")]
        pub end_address: Option<String>,
        #[doc = "The type of route this is:\nDEFAULT - By default, every app has routes to the local address ranges specified by RFC1918\nINHERITED - Routes inherited from the real Virtual Network routes\nSTATIC - Static route set on the app only\n\nThese values will be used for syncing an app's routes with those from a Virtual Network."]
        #[serde(rename = "routeType", default, skip_serializing_if = "Option::is_none")]
        pub route_type: Option<properties::RouteType>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The type of route this is:\nDEFAULT - By default, every app has routes to the local address ranges specified by RFC1918\nINHERITED - Routes inherited from the real Virtual Network routes\nSTATIC - Static route set on the app only\n\nThese values will be used for syncing an app's routes with those from a Virtual Network."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "RouteType")]
        pub enum RouteType {
            #[serde(rename = "DEFAULT")]
            Default,
            #[serde(rename = "INHERITED")]
            Inherited,
            #[serde(rename = "STATIC")]
            Static,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for RouteType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for RouteType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for RouteType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Default => serializer.serialize_unit_variant("RouteType", 0u32, "DEFAULT"),
                    Self::Inherited => serializer.serialize_unit_variant("RouteType", 1u32, "INHERITED"),
                    Self::Static => serializer.serialize_unit_variant("RouteType", 2u32, "STATIC"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "A class that describes the reason for a validation failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VnetValidationFailureDetails {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "VnetValidationFailureDetails resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<vnet_validation_failure_details::Properties>,
}
impl VnetValidationFailureDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vnet_validation_failure_details {
    use super::*;
    #[doc = "VnetValidationFailureDetails resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Text describing the validation outcome."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "A flag describing whether or not validation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub failed: Option<bool>,
        #[doc = "A list of tests that failed in the validation."]
        #[serde(rename = "failedTests", default, skip_serializing_if = "Vec::is_empty")]
        pub failed_tests: Vec<VnetValidationTestFailure>,
        #[doc = "A list of warnings generated during validation."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub warnings: Vec<VnetValidationTestFailure>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A class that describes a test that failed during NSG and UDR validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VnetValidationTestFailure {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "VnetValidationTestFailure resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<vnet_validation_test_failure::Properties>,
}
impl VnetValidationTestFailure {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vnet_validation_test_failure {
    use super::*;
    #[doc = "VnetValidationTestFailure resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The name of the test that failed."]
        #[serde(rename = "testName", default, skip_serializing_if = "Option::is_none")]
        pub test_name: Option<String>,
        #[doc = "The details of what caused the failure, e.g. the blocking rule name, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub details: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of App Service apps."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAppCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Site>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebAppCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WebAppCollection {
    pub fn new(value: Vec<Site>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Collection of app instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAppInstanceStatusCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<WebSiteInstanceStatus>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebAppInstanceStatusCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WebAppInstanceStatusCollection {
    pub fn new(value: Vec<WebSiteInstanceStatus>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Web App stack major version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppMajorVersion {
    #[doc = "Web App stack major version (display only)."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "Web App stack major version name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Minor versions associated with the major version."]
    #[serde(rename = "minorVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub minor_versions: Vec<WebAppMinorVersion>,
}
impl WebAppMajorVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web App stack minor version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppMinorVersion {
    #[doc = "Web App stack minor version (display only)."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "Web App stack major version name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Web App stack runtimes."]
    #[serde(rename = "stackSettings", default, skip_serializing_if = "Option::is_none")]
    pub stack_settings: Option<WebAppRuntimes>,
}
impl WebAppMinorVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web App runtime settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppRuntimeSettings {
    #[doc = "Web App stack minor version (runtime only)."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[doc = "<code>true</code> if remote debugging is supported for the stack; otherwise, <code>false</code>."]
    #[serde(rename = "remoteDebuggingSupported", default, skip_serializing_if = "Option::is_none")]
    pub remote_debugging_supported: Option<bool>,
    #[doc = "App Insights Web App stack settings."]
    #[serde(rename = "appInsightsSettings", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_settings: Option<AppInsightsWebAppStackSettings>,
    #[doc = "GitHub Actions Web App stack settings."]
    #[serde(rename = "gitHubActionSettings", default, skip_serializing_if = "Option::is_none")]
    pub git_hub_action_settings: Option<GitHubActionWebAppStackSettings>,
    #[doc = "<code>true</code> if the stack is in preview; otherwise, <code>false</code>."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[doc = "<code>true</code> if the stack is deprecated; otherwise, <code>false</code>."]
    #[serde(rename = "isDeprecated", default, skip_serializing_if = "Option::is_none")]
    pub is_deprecated: Option<bool>,
    #[doc = "<code>true</code> if the stack should be hidden; otherwise, <code>false</code>."]
    #[serde(rename = "isHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "End-of-life date for the minor version."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "<code>true</code> if the stack version is auto-updated; otherwise, <code>false</code>."]
    #[serde(rename = "isAutoUpdate", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_update: Option<bool>,
    #[doc = "<code>true</code> if the minor version is early-access; otherwise, <code>false</code>."]
    #[serde(rename = "isEarlyAccess", default, skip_serializing_if = "Option::is_none")]
    pub is_early_access: Option<bool>,
}
impl WebAppRuntimeSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web App stack runtimes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppRuntimes {
    #[doc = "Web App runtime settings."]
    #[serde(rename = "linuxRuntimeSettings", default, skip_serializing_if = "Option::is_none")]
    pub linux_runtime_settings: Option<WebAppRuntimeSettings>,
    #[doc = "Web App runtime settings."]
    #[serde(rename = "windowsRuntimeSettings", default, skip_serializing_if = "Option::is_none")]
    pub windows_runtime_settings: Option<WebAppRuntimeSettings>,
    #[doc = "Linux Java Container settings."]
    #[serde(rename = "linuxContainerSettings", default, skip_serializing_if = "Option::is_none")]
    pub linux_container_settings: Option<LinuxJavaContainerSettings>,
    #[doc = "Windows Java Container settings."]
    #[serde(rename = "windowsContainerSettings", default, skip_serializing_if = "Option::is_none")]
    pub windows_container_settings: Option<WindowsJavaContainerSettings>,
}
impl WebAppRuntimes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web App stack."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppStack {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Web App stack location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "WebAppStack resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<web_app_stack::Properties>,
}
impl WebAppStack {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod web_app_stack {
    use super::*;
    #[doc = "WebAppStack resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Web App stack (display only)."]
        #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
        pub display_text: Option<String>,
        #[doc = "Web App stack name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[doc = "List of major versions available."]
        #[serde(rename = "majorVersions", default, skip_serializing_if = "Vec::is_empty")]
        pub major_versions: Vec<WebAppMajorVersion>,
        #[doc = "Web App stack preferred OS."]
        #[serde(rename = "preferredOs", default, skip_serializing_if = "Option::is_none")]
        pub preferred_os: Option<properties::PreferredOs>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Web App stack preferred OS."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum PreferredOs {
            Windows,
            Linux,
        }
    }
}
#[doc = "Collection of Web app Stacks"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAppStackCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<WebAppStack>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebAppStackCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WebAppStackCollection {
    pub fn new(value: Vec<WebAppStack>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Web Job Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebJob {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "WebJob resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<web_job::Properties>,
}
impl WebJob {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod web_job {
    use super::*;
    #[doc = "WebJob resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Run command."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub run_command: Option<String>,
        #[doc = "Job URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        #[doc = "Extra Info URL."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extra_info_url: Option<String>,
        #[doc = "Job type."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub web_job_type: Option<properties::WebJobType>,
        #[doc = "Error information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        #[doc = "Using SDK?"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub using_sdk: Option<bool>,
        #[doc = "Job settings."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub settings: Option<serde_json::Value>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Job type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum WebJobType {
            Continuous,
            Triggered,
        }
    }
}
#[doc = "Collection of Kudu web job information elements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebJobCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<WebJob>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebJobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WebJobCollection {
    pub fn new(value: Vec<WebJob>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebSiteInstanceStatus {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "WebSiteInstanceStatus resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<web_site_instance_status::Properties>,
}
impl WebSiteInstanceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod web_site_instance_status {
    use super::*;
    #[doc = "WebSiteInstanceStatus resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<properties::State>,
        #[doc = "Link to the GetStatusApi in Kudu"]
        #[serde(rename = "statusUrl", default, skip_serializing_if = "Option::is_none")]
        pub status_url: Option<String>,
        #[doc = "Link to the Diagnose and Solve Portal"]
        #[serde(rename = "detectorUrl", default, skip_serializing_if = "Option::is_none")]
        pub detector_url: Option<String>,
        #[doc = "Link to the console to web app instance"]
        #[serde(rename = "consoleUrl", default, skip_serializing_if = "Option::is_none")]
        pub console_url: Option<String>,
        #[doc = "Link to the console to web app instance"]
        #[serde(rename = "healthCheckUrl", default, skip_serializing_if = "Option::is_none")]
        pub health_check_url: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub containers: Option<serde_json::Value>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum State {
            #[serde(rename = "READY")]
            Ready,
            #[serde(rename = "STOPPED")]
            Stopped,
            #[serde(rename = "UNKNOWN")]
            Unknown,
        }
    }
}
#[doc = "Windows Java Container settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsJavaContainerSettings {
    #[doc = "Java container (runtime only)."]
    #[serde(rename = "javaContainer", default, skip_serializing_if = "Option::is_none")]
    pub java_container: Option<String>,
    #[doc = "Java container version (runtime only)."]
    #[serde(rename = "javaContainerVersion", default, skip_serializing_if = "Option::is_none")]
    pub java_container_version: Option<String>,
    #[doc = "<code>true</code> if the stack is in preview; otherwise, <code>false</code>."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[doc = "<code>true</code> if the stack is deprecated; otherwise, <code>false</code>."]
    #[serde(rename = "isDeprecated", default, skip_serializing_if = "Option::is_none")]
    pub is_deprecated: Option<bool>,
    #[doc = "<code>true</code> if the stack should be hidden; otherwise, <code>false</code>."]
    #[serde(rename = "isHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "End-of-life date for the minor version."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "<code>true</code> if the stack version is auto-updated; otherwise, <code>false</code>."]
    #[serde(rename = "isAutoUpdate", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_update: Option<bool>,
    #[doc = "<code>true</code> if the minor version is early-access; otherwise, <code>false</code>."]
    #[serde(rename = "isEarlyAccess", default, skip_serializing_if = "Option::is_none")]
    pub is_early_access: Option<bool>,
}
impl WindowsJavaContainerSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Worker pool of an App Service Environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkerPool {
    #[doc = "Worker size ID for referencing this worker pool."]
    #[serde(rename = "workerSizeId", default, skip_serializing_if = "Option::is_none")]
    pub worker_size_id: Option<i32>,
    #[doc = "Shared or dedicated app hosting."]
    #[serde(rename = "computeMode", default, skip_serializing_if = "Option::is_none")]
    pub compute_mode: Option<worker_pool::ComputeMode>,
    #[doc = "VM size of the worker pool instances."]
    #[serde(rename = "workerSize", default, skip_serializing_if = "Option::is_none")]
    pub worker_size: Option<String>,
    #[doc = "Number of instances in the worker pool."]
    #[serde(rename = "workerCount", default, skip_serializing_if = "Option::is_none")]
    pub worker_count: Option<i32>,
    #[doc = "Names of all instances in the worker pool (read only)."]
    #[serde(rename = "instanceNames", default, skip_serializing_if = "Vec::is_empty")]
    pub instance_names: Vec<String>,
}
impl WorkerPool {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod worker_pool {
    use super::*;
    #[doc = "Shared or dedicated app hosting."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComputeMode {
        Shared,
        Dedicated,
        Dynamic,
    }
}
#[doc = "Collection of worker pools."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkerPoolCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<WorkerPoolResource>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkerPoolCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkerPoolCollection {
    pub fn new(value: Vec<WorkerPoolResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Worker pool of an App Service Environment ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkerPoolResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Worker pool of an App Service Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkerPool>,
    #[doc = "Description of a SKU for a scalable resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuDescription>,
}
impl WorkerPoolResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User Assigned identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "Principal Id of user assigned identity"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Client Id of user assigned identity"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
