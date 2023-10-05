#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The configuration settings of the Allowed Audiences validation flow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllowedAudiencesValidation {
    #[doc = "The configuration settings of the allowed list of audiences from which to validate the JWT token."]
    #[serde(
        rename = "allowedAudiences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<String>,
    #[doc = "The list of the allowed identities."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identities: Vec<String>,
}
impl AllowedPrincipals {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration of application logs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppLogsConfiguration {
    #[doc = "Logs destination, can be 'log-analytics', 'azure-monitor' or 'none'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[doc = "Log Analytics configuration, must only be provided when destination is configured as 'log-analytics'"]
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
#[doc = "Configuration settings for the Azure ContainerApp Service Authentication / Authorization feature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "AuthConfig resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<auth_config::Properties>,
}
impl AuthConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auth_config {
    use super::*;
    #[doc = "AuthConfig resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The configuration settings of the platform of ContainerApp Service Authentication/Authorization."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub platform: Option<AuthPlatform>,
        #[doc = "The configuration settings that determines the validation flow of users using ContainerApp Service Authentication/Authorization."]
        #[serde(rename = "globalValidation", default, skip_serializing_if = "Option::is_none")]
        pub global_validation: Option<GlobalValidation>,
        #[doc = "The configuration settings of each of the identity providers used to configure ContainerApp Service Authentication/Authorization."]
        #[serde(rename = "identityProviders", default, skip_serializing_if = "Option::is_none")]
        pub identity_providers: Option<IdentityProviders>,
        #[doc = "The configuration settings of the login flow of users using ContainerApp Service Authentication/Authorization."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub login: Option<Login>,
        #[doc = "The configuration settings of the HTTP requests for authentication and authorization requests made against ContainerApp Service Authentication/Authorization."]
        #[serde(rename = "httpSettings", default, skip_serializing_if = "Option::is_none")]
        pub http_settings: Option<HttpSettings>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "AuthConfig collection ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthConfigCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<AuthConfig>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AuthConfigCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AuthConfigCollection {
    pub fn new(value: Vec<AuthConfig>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The configuration settings of the platform of ContainerApp Service Authentication/Authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthPlatform {
    #[doc = "<code>true</code> if the Authentication / Authorization feature is enabled for the current app; otherwise, <code>false</code>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The RuntimeVersion of the Authentication / Authorization feature in use for the current app.\nThe setting in this value can control the behavior of certain features in the Authentication / Authorization module."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
}
impl AuthPlatform {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operations of the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperations {
    #[doc = "Collection of available operation details"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationDetail>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailableOperations {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AvailableOperations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A workload profile with specific hardware configure to run container apps."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableWorkloadProfile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Region of the workload profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Revision resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<available_workload_profile::Properties>,
}
impl AvailableWorkloadProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod available_workload_profile {
    use super::*;
    #[doc = "Revision resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Used to categorize workload profiles."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category: Option<String>,
        #[doc = "indicates whether the profile is default for the location."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub applicability: Option<properties::Applicability>,
        #[doc = "Number of cores in CPU."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cores: Option<i32>,
        #[doc = "Memory in GiB."]
        #[serde(rename = "memoryGiB", default, skip_serializing_if = "Option::is_none")]
        pub memory_gi_b: Option<i32>,
        #[doc = "The everyday name of the workload profile."]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "indicates whether the profile is default for the location."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Applicability")]
        pub enum Applicability {
            LocationDefault,
            Custom,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Applicability {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Applicability {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Applicability {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::LocationDefault => serializer.serialize_unit_variant("Applicability", 0u32, "LocationDefault"),
                    Self::Custom => serializer.serialize_unit_variant("Applicability", 1u32, "Custom"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Collection of available workload profiles in the location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableWorkloadProfilesCollection {
    #[doc = "Collection of workload profiles."]
    pub value: Vec<AvailableWorkloadProfile>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailableWorkloadProfilesCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AvailableWorkloadProfilesCollection {
    pub fn new(value: Vec<AvailableWorkloadProfile>) -> Self {
        Self { value, next_link: None }
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
    #[serde(
        rename = "loginParameters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(
        rename = "allowedAudiences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Container App credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCredentials {
    #[doc = "Client Id."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Client Secret."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "Tenant Id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Kind of auth github does for deploying the template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Subscription Id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl AzureCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure File Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFileProperties {
    #[doc = "Storage account name for azure file."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Storage account key for azure file."]
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[doc = "Access mode for storage"]
    #[serde(rename = "accessMode", default, skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<azure_file_properties::AccessMode>,
    #[doc = "Azure file share name."]
    #[serde(rename = "shareName", default, skip_serializing_if = "Option::is_none")]
    pub share_name: Option<String>,
}
impl AzureFileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_file_properties {
    use super::*;
    #[doc = "Access mode for storage"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessMode")]
    pub enum AccessMode {
        ReadOnly,
        ReadWrite,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ReadOnly => serializer.serialize_unit_variant("AccessMode", 0u32, "ReadOnly"),
                Self::ReadWrite => serializer.serialize_unit_variant("AccessMode", 1u32, "ReadWrite"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Container App base container definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseContainer {
    #[doc = "Container image tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[doc = "Custom container name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Container start command."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub command: Vec<String>,
    #[doc = "Container start command arguments."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub args: Vec<String>,
    #[doc = "Container environment variables."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub env: Vec<EnvironmentVar>,
    #[doc = "Container App container resource requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ContainerResources>,
    #[doc = "Container volume mounts."]
    #[serde(
        rename = "volumeMounts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_mounts: Vec<VolumeMount>,
}
impl BaseContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing meter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingMeter {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Region for the billing meter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Revision resource specific properties"]
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
    #[doc = "Revision resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Used to categorize billing meters."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category: Option<String>,
        #[doc = "Billing meter type."]
        #[serde(rename = "meterType", default, skip_serializing_if = "Option::is_none")]
        pub meter_type: Option<String>,
        #[doc = "The everyday name of the billing meter."]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of billing meters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingMeterCollection {
    #[doc = "Collection of billing meters."]
    pub value: Vec<BillingMeter>,
}
impl BillingMeterCollection {
    pub fn new(value: Vec<BillingMeter>) -> Self {
        Self { value }
    }
}
#[doc = "Certificate used for Custom Domain bindings of Container Apps in a Managed Environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Certificate resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<certificate::Properties>,
}
impl Certificate {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
pub mod certificate {
    use super::*;
    #[doc = "Certificate resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the certificate."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Certificate password."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        #[doc = "Subject name of the certificate."]
        #[serde(rename = "subjectName", default, skip_serializing_if = "Option::is_none")]
        pub subject_name: Option<String>,
        #[doc = "Subject alternative names the certificate applies to."]
        #[serde(
            rename = "subjectAlternativeNames",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub subject_alternative_names: Vec<String>,
        #[doc = "PFX or PEM blob"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
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
        #[doc = "Public key hash."]
        #[serde(rename = "publicKeyHash", default, skip_serializing_if = "Option::is_none")]
        pub public_key_hash: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Provisioning state of the certificate."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            DeleteFailed,
            Pending,
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
                    Self::DeleteFailed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "DeleteFailed"),
                    Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Collection of Certificates."]
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CertificateCollection {
    pub fn new(value: Vec<Certificate>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A certificate to update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificatePatch {
    #[doc = "Application-specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CertificatePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "The name of the resource for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "Indicates if the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the given name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_response::Reason>,
    #[doc = "Detailed reason why the given name is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_response {
    use super::*;
    #[doc = "The reason why the given name is not available."]
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
#[doc = "Non versioned Container App configuration properties that define the mutable settings of a Container app"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[doc = "Collection of secrets used by a Container app"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub secrets: Vec<Secret>,
    #[doc = "ActiveRevisionsMode controls how active revisions are handled for the Container app:\n<list><item>Multiple: multiple revisions can be active.</item><item>Single: Only one revision can be active at a time. Revision weights can not be used in this mode. If no value if provided, this is the default.</item></list>"]
    #[serde(rename = "activeRevisionsMode", default, skip_serializing_if = "Option::is_none")]
    pub active_revisions_mode: Option<configuration::ActiveRevisionsMode>,
    #[doc = "Container App Ingress configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Ingress>,
    #[doc = "Collection of private container registry credentials for containers used by the Container app"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub registries: Vec<RegistryCredentials>,
    #[doc = "Container App Dapr configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dapr: Option<Dapr>,
    #[doc = "Optional. Max inactive revisions a Container App can have."]
    #[serde(rename = "maxInactiveRevisions", default, skip_serializing_if = "Option::is_none")]
    pub max_inactive_revisions: Option<i32>,
    #[doc = "Container App to be a dev service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}
impl Configuration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration {
    use super::*;
    #[doc = "ActiveRevisionsMode controls how active revisions are handled for the Container app:\n<list><item>Multiple: multiple revisions can be active.</item><item>Single: Only one revision can be active at a time. Revision weights can not be used in this mode. If no value if provided, this is the default.</item></list>"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActiveRevisionsMode")]
    pub enum ActiveRevisionsMode {
        Multiple,
        Single,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActiveRevisionsMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActiveRevisionsMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActiveRevisionsMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Multiple => serializer.serialize_unit_variant("ActiveRevisionsMode", 0u32, "Multiple"),
                Self::Single => serializer.serialize_unit_variant("ActiveRevisionsMode", 1u32, "Single"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ActiveRevisionsMode {
        fn default() -> Self {
            Self::Single
        }
    }
}
#[doc = "An environment for Kubernetes cluster specialized for web workloads by Azure App Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectedEnvironment {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "ConnectedEnvironment resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<connected_environment::Properties>,
}
impl ConnectedEnvironment {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            extended_location: None,
            properties: None,
        }
    }
}
pub mod connected_environment {
    use super::*;
    #[doc = "ConnectedEnvironment resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the Kubernetes Environment."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Any errors that occurred during deployment or deployment validation"]
        #[serde(rename = "deploymentErrors", default, skip_serializing_if = "Option::is_none")]
        pub deployment_errors: Option<String>,
        #[doc = "Default Domain Name for the cluster"]
        #[serde(rename = "defaultDomain", default, skip_serializing_if = "Option::is_none")]
        pub default_domain: Option<String>,
        #[doc = "Static IP of the connectedEnvironment"]
        #[serde(rename = "staticIp", default, skip_serializing_if = "Option::is_none")]
        pub static_ip: Option<String>,
        #[doc = "Application Insights connection string used by Dapr to export Service to Service communication telemetry"]
        #[serde(rename = "daprAIConnectionString", default, skip_serializing_if = "Option::is_none")]
        pub dapr_ai_connection_string: Option<String>,
        #[doc = "Configuration properties for apps environment custom domain"]
        #[serde(rename = "customDomainConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub custom_domain_configuration: Option<CustomDomainConfiguration>,
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
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            Waiting,
            InitializationInProgress,
            InfrastructureSetupInProgress,
            InfrastructureSetupComplete,
            ScheduledForDelete,
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
                    Self::Waiting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Waiting"),
                    Self::InitializationInProgress => {
                        serializer.serialize_unit_variant("ProvisioningState", 4u32, "InitializationInProgress")
                    }
                    Self::InfrastructureSetupInProgress => {
                        serializer.serialize_unit_variant("ProvisioningState", 5u32, "InfrastructureSetupInProgress")
                    }
                    Self::InfrastructureSetupComplete => {
                        serializer.serialize_unit_variant("ProvisioningState", 6u32, "InfrastructureSetupComplete")
                    }
                    Self::ScheduledForDelete => serializer.serialize_unit_variant("ProvisioningState", 7u32, "ScheduledForDelete"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Collection of connectedEnvironments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedEnvironmentCollection {
    #[doc = "Collection of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ConnectedEnvironment>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConnectedEnvironmentCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ConnectedEnvironmentCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage resource for connectedEnvironment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedEnvironmentStorage {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Storage properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<connected_environment_storage::Properties>,
}
impl ConnectedEnvironmentStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connected_environment_storage {
    use super::*;
    #[doc = "Storage properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Azure File Properties."]
        #[serde(rename = "azureFile", default, skip_serializing_if = "Option::is_none")]
        pub azure_file: Option<AzureFileProperties>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Storage for Environments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectedEnvironmentStoragesCollection {
    #[doc = "Collection of storage resources."]
    pub value: Vec<ConnectedEnvironmentStorage>,
}
impl ConnectedEnvironmentStoragesCollection {
    pub fn new(value: Vec<ConnectedEnvironmentStorage>) -> Self {
        Self { value }
    }
}
#[doc = "Container App container definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Container {
    #[serde(flatten)]
    pub base_container: BaseContainer,
    #[doc = "List of probes for the container."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub probes: Vec<ContainerAppProbe>,
}
impl Container {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerApp {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "The fully qualified resource ID of the resource that manages this resource. Indicates if this resource is managed by another Azure resource. If this is present, complete mode deployment will not delete the resource if it is removed from the template since it is managed by another resource."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "ContainerApp resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<container_app::Properties>,
}
impl ContainerApp {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            extended_location: None,
            identity: None,
            managed_by: None,
            properties: None,
        }
    }
}
pub mod container_app {
    use super::*;
    #[doc = "ContainerApp resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the Container App."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Deprecated. Resource ID of the Container App's environment."]
        #[serde(rename = "managedEnvironmentId", default, skip_serializing_if = "Option::is_none")]
        pub managed_environment_id: Option<String>,
        #[doc = "Resource ID of environment."]
        #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
        pub environment_id: Option<String>,
        #[doc = "Workload profile name for container apps to execute on."]
        #[serde(rename = "workloadProfileName", default, skip_serializing_if = "Option::is_none")]
        pub workload_profile_name: Option<WorkloadProfileName>,
        #[doc = "Name of the latest revision of the Container App."]
        #[serde(rename = "latestRevisionName", default, skip_serializing_if = "Option::is_none")]
        pub latest_revision_name: Option<String>,
        #[doc = "Name of the latest ready revision of the Container App."]
        #[serde(rename = "latestReadyRevisionName", default, skip_serializing_if = "Option::is_none")]
        pub latest_ready_revision_name: Option<String>,
        #[doc = "Fully Qualified Domain Name of the latest revision of the Container App."]
        #[serde(rename = "latestRevisionFqdn", default, skip_serializing_if = "Option::is_none")]
        pub latest_revision_fqdn: Option<String>,
        #[doc = "Id used to verify domain name ownership"]
        #[serde(rename = "customDomainVerificationId", default, skip_serializing_if = "Option::is_none")]
        pub custom_domain_verification_id: Option<String>,
        #[doc = "Non versioned Container App configuration properties that define the mutable settings of a Container app"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<Configuration>,
        #[doc = "Container App versioned application definition.\nDefines the desired state of an immutable revision.\nAny changes to this section Will result in a new revision being created"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub template: Option<Template>,
        #[doc = "Outbound IP Addresses for container app."]
        #[serde(
            rename = "outboundIpAddresses",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub outbound_ip_addresses: Vec<String>,
        #[doc = "The endpoint of the eventstream of the container app."]
        #[serde(rename = "eventStreamEndpoint", default, skip_serializing_if = "Option::is_none")]
        pub event_stream_endpoint: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Provisioning state of the Container App."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            InProgress,
            Succeeded,
            Failed,
            Canceled,
            Deleting,
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
                    Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 0u32, "InProgress"),
                    Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                    Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                    Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                    Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Container App Auth Token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerAppAuthToken {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Container App auth token resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<container_app_auth_token::Properties>,
}
impl ContainerAppAuthToken {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
pub mod container_app_auth_token {
    use super::*;
    #[doc = "Container App auth token resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Auth token value."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
        #[doc = "Token expiration date."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub expires: Option<time::OffsetDateTime>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Container App collection ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerAppCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ContainerApp>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ContainerAppCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ContainerAppCollection {
    pub fn new(value: Vec<ContainerApp>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Container App executions collection ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerAppJobExecutions {
    #[doc = "Collection of resources."]
    pub value: Vec<JobExecution>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ContainerAppJobExecutions {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ContainerAppJobExecutions {
    pub fn new(value: Vec<JobExecution>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerAppProbe {
    #[doc = "Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1. Maximum value is 10."]
    #[serde(rename = "failureThreshold", default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[doc = "HTTPGet specifies the http request to perform."]
    #[serde(rename = "httpGet", default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<container_app_probe::HttpGet>,
    #[doc = "Number of seconds after the container has started before liveness probes are initiated. Minimum value is 1. Maximum value is 60."]
    #[serde(rename = "initialDelaySeconds", default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    #[doc = "How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value is 240."]
    #[serde(rename = "periodSeconds", default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[doc = "Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1. Maximum value is 10."]
    #[serde(rename = "successThreshold", default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[doc = "TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported."]
    #[serde(rename = "tcpSocket", default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<container_app_probe::TcpSocket>,
    #[doc = "Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is an alpha field and requires enabling ProbeTerminationGracePeriod feature gate. Maximum value is 3600 seconds (1 hour)"]
    #[serde(rename = "terminationGracePeriodSeconds", default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    #[doc = "Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 240."]
    #[serde(rename = "timeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    #[doc = "The type of probe."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<container_app_probe::Type>,
}
impl ContainerAppProbe {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_app_probe {
    use super::*;
    #[doc = "HTTPGet specifies the http request to perform."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct HttpGet {
        #[doc = "Host name to connect to, defaults to the pod IP. You probably want to set \"Host\" in httpHeaders instead."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub host: Option<String>,
        #[doc = "Custom headers to set in the request. HTTP allows repeated headers."]
        #[serde(
            rename = "httpHeaders",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub http_headers: Vec<serde_json::Value>,
        #[doc = "Path to access on the HTTP server."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[doc = "Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME."]
        pub port: i32,
        #[doc = "Scheme to use for connecting to the host. Defaults to HTTP."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scheme: Option<http_get::Scheme>,
    }
    impl HttpGet {
        pub fn new(port: i32) -> Self {
            Self {
                host: None,
                http_headers: Vec::new(),
                path: None,
                port,
                scheme: None,
            }
        }
    }
    pub mod http_get {
        use super::*;
        #[doc = "Scheme to use for connecting to the host. Defaults to HTTP."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Scheme")]
        pub enum Scheme {
            #[serde(rename = "HTTP")]
            Http,
            #[serde(rename = "HTTPS")]
            Https,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Scheme {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Scheme {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Scheme {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Http => serializer.serialize_unit_variant("Scheme", 0u32, "HTTP"),
                    Self::Https => serializer.serialize_unit_variant("Scheme", 1u32, "HTTPS"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct TcpSocket {
        #[doc = "Optional: Host name to connect to, defaults to the pod IP."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub host: Option<String>,
        #[doc = "Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME."]
        pub port: i32,
    }
    impl TcpSocket {
        pub fn new(port: i32) -> Self {
            Self { host: None, port }
        }
    }
    #[doc = "The type of probe."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Liveness,
        Readiness,
        Startup,
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
                Self::Liveness => serializer.serialize_unit_variant("Type", 0u32, "Liveness"),
                Self::Readiness => serializer.serialize_unit_variant("Type", 1u32, "Readiness"),
                Self::Startup => serializer.serialize_unit_variant("Type", 2u32, "Startup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Container App Secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerAppSecret {
    #[doc = "Secret Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Secret Value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Resource ID of a managed identity to authenticate with Azure Key Vault, or System to use a system-assigned identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "Azure Key Vault URL pointing to the secret referenced by the container app."]
    #[serde(rename = "keyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_url: Option<String>,
}
impl ContainerAppSecret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App container resource requirements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerResources {
    #[doc = "Required CPU in cores, e.g. 0.5"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    #[doc = "Required memory, e.g. \"250Mb\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[doc = "Ephemeral Storage, e.g. \"1Gi\""]
    #[serde(rename = "ephemeralStorage", default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<String>,
}
impl ContainerResources {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Cross-Origin-Resource-Sharing policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsPolicy {
    #[doc = "Specifies the content for the access-control-allow-origins header"]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Vec<String>,
    #[doc = "Specifies the content for the access-control-allow-methods header"]
    #[serde(
        rename = "allowedMethods",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_methods: Vec<String>,
    #[doc = "Specifies the content for the access-control-allow-headers header"]
    #[serde(
        rename = "allowedHeaders",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_headers: Vec<String>,
    #[doc = "Specifies the content for the access-control-expose-headers header "]
    #[serde(
        rename = "exposeHeaders",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expose_headers: Vec<String>,
    #[doc = "Specifies the content for the access-control-max-age header"]
    #[serde(rename = "maxAge", default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
    #[doc = "Specifies whether the resource allows credentials"]
    #[serde(rename = "allowCredentials", default, skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
}
impl CorsPolicy {
    pub fn new(allowed_origins: Vec<String>) -> Self {
        Self {
            allowed_origins,
            allowed_methods: Vec::new(),
            allowed_headers: Vec::new(),
            expose_headers: Vec::new(),
            max_age: None,
            allow_credentials: None,
        }
    }
}
#[doc = "Custom Domain of a Container App"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    #[doc = "Hostname."]
    pub name: String,
    #[doc = "Custom Domain binding type."]
    #[serde(rename = "bindingType", default, skip_serializing_if = "Option::is_none")]
    pub binding_type: Option<custom_domain::BindingType>,
    #[doc = "Resource Id of the Certificate to be bound to this hostname. Must exist in the Managed Environment."]
    #[serde(rename = "certificateId", default, skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}
impl CustomDomain {
    pub fn new(name: String) -> Self {
        Self {
            name,
            binding_type: None,
            certificate_id: None,
        }
    }
}
pub mod custom_domain {
    use super::*;
    #[doc = "Custom Domain binding type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BindingType")]
    pub enum BindingType {
        Disabled,
        SniEnabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BindingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BindingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BindingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("BindingType", 0u32, "Disabled"),
                Self::SniEnabled => serializer.serialize_unit_variant("BindingType", 1u32, "SniEnabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Configuration properties for apps environment custom domain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainConfiguration {
    #[doc = "Id used to verify domain name ownership"]
    #[serde(rename = "customDomainVerificationId", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain_verification_id: Option<String>,
    #[doc = "Dns suffix for the environment domain"]
    #[serde(rename = "dnsSuffix", default, skip_serializing_if = "Option::is_none")]
    pub dns_suffix: Option<String>,
    #[doc = "PFX or PEM blob"]
    #[serde(rename = "certificateValue", default, skip_serializing_if = "Option::is_none")]
    pub certificate_value: Option<String>,
    #[doc = "Certificate password"]
    #[serde(rename = "certificatePassword", default, skip_serializing_if = "Option::is_none")]
    pub certificate_password: Option<String>,
    #[doc = "Certificate expiration date."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Certificate thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Subject name of the certificate."]
    #[serde(rename = "subjectName", default, skip_serializing_if = "Option::is_none")]
    pub subject_name: Option<String>,
}
impl CustomDomainConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom domain analysis."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomHostnameAnalysisResult {
    #[doc = "Host name that was analyzed"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "<code>true</code> if hostname is already verified; otherwise, <code>false</code>."]
    #[serde(rename = "isHostnameAlreadyVerified", default, skip_serializing_if = "Option::is_none")]
    pub is_hostname_already_verified: Option<bool>,
    #[doc = "DNS verification test result."]
    #[serde(rename = "customDomainVerificationTest", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain_verification_test: Option<custom_hostname_analysis_result::CustomDomainVerificationTest>,
    #[doc = "Raw failure information if DNS verification fails."]
    #[serde(rename = "customDomainVerificationFailureInfo", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain_verification_failure_info: Option<custom_hostname_analysis_result::CustomDomainVerificationFailureInfo>,
    #[doc = "<code>true</code> if there is a conflict on the Container App's managed environment; otherwise, <code>false</code>."]
    #[serde(rename = "hasConflictOnManagedEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub has_conflict_on_managed_environment: Option<bool>,
    #[doc = "<code>true</code> if there is a conflict on the Container App's managed environment level custom domain; otherwise, <code>false</code>."]
    #[serde(rename = "conflictWithEnvironmentCustomDomain", default, skip_serializing_if = "Option::is_none")]
    pub conflict_with_environment_custom_domain: Option<bool>,
    #[doc = "Name of the conflicting Container App on the Managed Environment if it's within the same subscription."]
    #[serde(rename = "conflictingContainerAppResourceId", default, skip_serializing_if = "Option::is_none")]
    pub conflicting_container_app_resource_id: Option<String>,
    #[doc = "CName records visible for this hostname."]
    #[serde(
        rename = "cNameRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub c_name_records: Vec<String>,
    #[doc = "TXT records visible for this hostname."]
    #[serde(
        rename = "txtRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub txt_records: Vec<String>,
    #[doc = "A records visible for this hostname."]
    #[serde(
        rename = "aRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub a_records: Vec<String>,
    #[doc = "Alternate CName records visible for this hostname."]
    #[serde(
        rename = "alternateCNameRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub alternate_c_name_records: Vec<String>,
    #[doc = "Alternate TXT records visible for this hostname."]
    #[serde(
        rename = "alternateTxtRecords",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub alternate_txt_records: Vec<String>,
}
impl CustomHostnameAnalysisResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod custom_hostname_analysis_result {
    use super::*;
    #[doc = "DNS verification test result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomDomainVerificationTest {
        Passed,
        Failed,
        Skipped,
    }
    #[doc = "Raw failure information if DNS verification fails."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct CustomDomainVerificationFailureInfo {
        #[doc = "Standardized string to programmatically identify the error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Detailed error description and debugging information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "Detailed error description and debugging information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[doc = "Details or the error"]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub details: Vec<serde_json::Value>,
    }
    impl CustomDomainVerificationFailureInfo {
        pub fn new() -> Self {
            Self::default()
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
#[doc = "Container App container Custom scaling rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomScaleRule {
    #[doc = "Type of the custom scale rule\neg: azure-servicebus, redis etc."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata properties to describe custom scale rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "Authentication secrets for the custom scale rule."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub auth: Vec<ScaleRuleAuth>,
}
impl CustomScaleRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App Dapr configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dapr {
    #[doc = "Boolean indicating if the Dapr side car is enabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Dapr application identifier"]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "Tells Dapr which protocol your application is using. Valid options are http and grpc. Default is http"]
    #[serde(rename = "appProtocol", default, skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<dapr::AppProtocol>,
    #[doc = "Tells Dapr which port your application is listening on"]
    #[serde(rename = "appPort", default, skip_serializing_if = "Option::is_none")]
    pub app_port: Option<i32>,
    #[doc = "Dapr max size of http header read buffer in KB to handle when sending multi-KB headers. Default is 65KB."]
    #[serde(rename = "httpReadBufferSize", default, skip_serializing_if = "Option::is_none")]
    pub http_read_buffer_size: Option<i32>,
    #[doc = "Increasing max size of request body http and grpc servers parameter in MB to handle uploading of big files. Default is 4 MB."]
    #[serde(rename = "httpMaxRequestSize", default, skip_serializing_if = "Option::is_none")]
    pub http_max_request_size: Option<i32>,
    #[doc = "Sets the log level for the Dapr sidecar. Allowed values are debug, info, warn, error. Default is info."]
    #[serde(rename = "logLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<dapr::LogLevel>,
    #[doc = "Enables API logging for the Dapr sidecar"]
    #[serde(rename = "enableApiLogging", default, skip_serializing_if = "Option::is_none")]
    pub enable_api_logging: Option<bool>,
}
impl Dapr {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dapr {
    use super::*;
    #[doc = "Tells Dapr which protocol your application is using. Valid options are http and grpc. Default is http"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AppProtocol")]
    pub enum AppProtocol {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "grpc")]
        Grpc,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AppProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AppProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AppProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("AppProtocol", 0u32, "http"),
                Self::Grpc => serializer.serialize_unit_variant("AppProtocol", 1u32, "grpc"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AppProtocol {
        fn default() -> Self {
            Self::Http
        }
    }
    #[doc = "Sets the log level for the Dapr sidecar. Allowed values are debug, info, warn, error. Default is info."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LogLevel")]
    pub enum LogLevel {
        #[serde(rename = "info")]
        Info,
        #[serde(rename = "debug")]
        Debug,
        #[serde(rename = "warn")]
        Warn,
        #[serde(rename = "error")]
        Error,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LogLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LogLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LogLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Info => serializer.serialize_unit_variant("LogLevel", 0u32, "info"),
                Self::Debug => serializer.serialize_unit_variant("LogLevel", 1u32, "debug"),
                Self::Warn => serializer.serialize_unit_variant("LogLevel", 2u32, "warn"),
                Self::Error => serializer.serialize_unit_variant("LogLevel", 3u32, "error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Dapr Component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DaprComponent {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Dapr Component resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<dapr_component::Properties>,
}
impl DaprComponent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dapr_component {
    use super::*;
    #[doc = "Dapr Component resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Component type"]
        #[serde(rename = "componentType", default, skip_serializing_if = "Option::is_none")]
        pub component_type: Option<String>,
        #[doc = "Component version"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[doc = "Boolean describing if the component errors are ignores"]
        #[serde(rename = "ignoreErrors", default, skip_serializing_if = "Option::is_none")]
        pub ignore_errors: Option<bool>,
        #[doc = "Initialization timeout"]
        #[serde(rename = "initTimeout", default, skip_serializing_if = "Option::is_none")]
        pub init_timeout: Option<String>,
        #[doc = "Collection of secrets used by a Dapr component"]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub secrets: Vec<Secret>,
        #[doc = "Name of a Dapr component to retrieve component secrets from"]
        #[serde(rename = "secretStoreComponent", default, skip_serializing_if = "Option::is_none")]
        pub secret_store_component: Option<String>,
        #[doc = "Component metadata"]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub metadata: Vec<DaprMetadata>,
        #[doc = "Names of container apps that can use this Dapr component"]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub scopes: Vec<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Dapr Components ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DaprComponentsCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<DaprComponent>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DaprComponentsCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DaprComponentsCollection {
    pub fn new(value: Vec<DaprComponent>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Configuration properties Dapr component"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DaprConfiguration {
    #[doc = "The version of Dapr"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl DaprConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dapr component metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DaprMetadata {
    #[doc = "Metadata property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metadata property value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Name of the Dapr Component secret from which to pull the metadata property value."]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<String>,
}
impl DaprMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dapr component Secret for ListSecrets Action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DaprSecret {
    #[doc = "Secret Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Secret Value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl DaprSecret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dapr component Secrets Collection for ListSecrets Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DaprSecretsCollection {
    #[doc = "Collection of secrets used by a Dapr component"]
    pub value: Vec<DaprSecret>,
}
impl DaprSecretsCollection {
    pub fn new(value: Vec<DaprSecret>) -> Self {
        Self { value }
    }
}
#[doc = "The configuration settings of the Azure Active Directory default authorization policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultAuthorizationPolicy {
    #[doc = "The configuration settings of the Azure Active Directory allowed principals."]
    #[serde(rename = "allowedPrincipals", default, skip_serializing_if = "Option::is_none")]
    pub allowed_principals: Option<AllowedPrincipals>,
    #[doc = "The configuration settings of the Azure Active Directory allowed applications."]
    #[serde(
        rename = "allowedApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
        #[doc = "Details or the error"]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
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
#[doc = "Details of a diagnostics data provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticDataProviderMetadata {
    #[doc = "Name of data provider"]
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[doc = "Collection of properties"]
    #[serde(
        rename = "propertyBag",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_bag: Vec<serde_json::Value>,
}
impl DiagnosticDataProviderMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostics data column"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticDataTableResponseColumn {
    #[doc = "Column name"]
    #[serde(rename = "columnName", default, skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[doc = "Data type of the column"]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[doc = "Column type"]
    #[serde(rename = "columnType", default, skip_serializing_if = "Option::is_none")]
    pub column_type: Option<String>,
}
impl DiagnosticDataTableResponseColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostics data table"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticDataTableResponseObject {
    #[doc = "Table name"]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "Columns in the table"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<DiagnosticDataTableResponseColumn>,
    #[doc = "Rows in the table"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rows: Vec<serde_json::Value>,
}
impl DiagnosticDataTableResponseObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rendering details of a diagnostics table"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticRendering {
    #[doc = "Rendering type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
    #[doc = "Title of the table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Description of the table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Flag if the table should be rendered"]
    #[serde(rename = "isVisible", default, skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
}
impl DiagnosticRendering {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Support topic information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSupportTopic {
    #[doc = "Unique topic identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "PES identifier"]
    #[serde(rename = "pesId", default, skip_serializing_if = "Option::is_none")]
    pub pes_id: Option<String>,
}
impl DiagnosticSupportTopic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostics data for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Diagnostics {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Diagnostics resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<diagnostics::Properties>,
}
impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnostics {
    use super::*;
    #[doc = "Diagnostics resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Metadata of the diagnostics response"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metadata: Option<DiagnosticsDefinition>,
        #[doc = "Set of data collections associated with the response."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub dataset: Vec<DiagnosticsDataApiResponse>,
        #[doc = "Rendering details of a diagnostics table"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<DiagnosticsStatus>,
        #[doc = "Details of a diagnostics data provider"]
        #[serde(rename = "dataProviderMetadata", default, skip_serializing_if = "Option::is_none")]
        pub data_provider_metadata: Option<DiagnosticDataProviderMetadata>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Diagnostics data collection for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsCollection {
    #[doc = "Collection of diagnostic data."]
    pub value: Vec<Diagnostics>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiagnosticsCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DiagnosticsCollection {
    pub fn new(value: Vec<Diagnostics>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Diagnostics data returned from a detector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticsDataApiResponse {
    #[doc = "Diagnostics data table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<DiagnosticDataTableResponseObject>,
    #[doc = "Rendering details of a diagnostics table"]
    #[serde(rename = "renderingProperties", default, skip_serializing_if = "Option::is_none")]
    pub rendering_properties: Option<DiagnosticRendering>,
}
impl DiagnosticsDataApiResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata of the diagnostics response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticsDefinition {
    #[doc = "Unique detector name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display Name of the detector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Details of the diagnostics info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Authors' names of the detector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "Category of the detector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "List of support topics"]
    #[serde(
        rename = "supportTopicList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub support_topic_list: Vec<DiagnosticSupportTopic>,
    #[doc = "List of analysis types"]
    #[serde(
        rename = "analysisTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub analysis_types: Vec<String>,
    #[doc = "Authors' names of the detector"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Authors' names of the detector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}
impl DiagnosticsDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rendering details of a diagnostics table"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticsStatus {
    #[doc = "Diagnostic message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Status"]
    #[serde(rename = "statusId", default, skip_serializing_if = "Option::is_none")]
    pub status_id: Option<i32>,
}
impl DiagnosticsStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Environment Auth Token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentAuthToken {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Environment auth token resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<environment_auth_token::Properties>,
}
impl EnvironmentAuthToken {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
pub mod environment_auth_token {
    use super::*;
    #[doc = "Environment auth token resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Auth token value."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
        #[doc = "Token expiration date."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub expires: Option<time::OffsetDateTime>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Container App container environment variable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentVar {
    #[doc = "Environment variable name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Non-secret environment variable value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Name of the Container App secret from which to pull the environment variable value."]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<String>,
}
impl EnvironmentVar {
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
#[doc = "Configuration properties that define the mutable settings of a Container App SourceControl"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GithubActionConfiguration {
    #[doc = "Container App registry information."]
    #[serde(rename = "registryInfo", default, skip_serializing_if = "Option::is_none")]
    pub registry_info: Option<RegistryInfo>,
    #[doc = "Container App credentials."]
    #[serde(rename = "azureCredentials", default, skip_serializing_if = "Option::is_none")]
    pub azure_credentials: Option<AzureCredentials>,
    #[doc = "Context path"]
    #[serde(rename = "contextPath", default, skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
    #[doc = "One time Github PAT to configure github environment"]
    #[serde(rename = "githubPersonalAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub github_personal_access_token: Option<String>,
    #[doc = "Image name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[doc = "Code or Image"]
    #[serde(rename = "publishType", default, skip_serializing_if = "Option::is_none")]
    pub publish_type: Option<String>,
    #[doc = "Operation system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[doc = "Runtime stack"]
    #[serde(rename = "runtimeStack", default, skip_serializing_if = "Option::is_none")]
    pub runtime_stack: Option<String>,
    #[doc = "Runtime version"]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
}
impl GithubActionConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings that determines the validation flow of users using ContainerApp Service Authentication/Authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalValidation {
    #[doc = "The action to take when an unauthenticated client attempts to access the app."]
    #[serde(rename = "unauthenticatedClientAction", default, skip_serializing_if = "Option::is_none")]
    pub unauthenticated_client_action: Option<global_validation::UnauthenticatedClientAction>,
    #[doc = "The default authentication provider to use when multiple providers are configured.\nThis setting is only needed if multiple providers are configured and the unauthenticated client\naction is set to \"RedirectToLoginPage\"."]
    #[serde(rename = "redirectToProvider", default, skip_serializing_if = "Option::is_none")]
    pub redirect_to_provider: Option<String>,
    #[doc = "The paths for which unauthenticated flow would not be redirected to the login page."]
    #[serde(
        rename = "excludedPaths",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Container App container Http scaling rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpScaleRule {
    #[doc = "Metadata properties to describe http scale rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "Authentication secrets for the custom scale rule."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub auth: Vec<ScaleRuleAuth>,
}
impl HttpScaleRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the HTTP requests for authentication and authorization requests made against ContainerApp Service Authentication/Authorization."]
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
#[doc = "The configuration settings of each of the identity providers used to configure ContainerApp Service Authentication/Authorization."]
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
#[doc = "Container App Ingress configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ingress {
    #[doc = "Hostname."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Bool indicating if app exposes an external http endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[doc = "Target Port in containers for traffic from ingress"]
    #[serde(rename = "targetPort", default, skip_serializing_if = "Option::is_none")]
    pub target_port: Option<i32>,
    #[doc = "Exposed Port in containers for TCP traffic from ingress"]
    #[serde(rename = "exposedPort", default, skip_serializing_if = "Option::is_none")]
    pub exposed_port: Option<i32>,
    #[doc = "Ingress transport protocol"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<ingress::Transport>,
    #[doc = "Traffic weights for app's revisions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub traffic: Vec<TrafficWeight>,
    #[doc = "custom domain bindings for Container Apps' hostnames."]
    #[serde(
        rename = "customDomains",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_domains: Vec<CustomDomain>,
    #[doc = "Bool indicating if HTTP connections to is allowed. If set to false HTTP connections are automatically redirected to HTTPS connections"]
    #[serde(rename = "allowInsecure", default, skip_serializing_if = "Option::is_none")]
    pub allow_insecure: Option<bool>,
    #[doc = "Rules to restrict incoming IP address."]
    #[serde(
        rename = "ipSecurityRestrictions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_security_restrictions: Vec<IpSecurityRestrictionRule>,
    #[doc = "Sticky Sessions for Single Revision Mode"]
    #[serde(rename = "stickySessions", default, skip_serializing_if = "Option::is_none")]
    pub sticky_sessions: Option<ingress::StickySessions>,
    #[doc = "Client certificate mode for mTLS authentication. Ignore indicates server drops client certificate on forwarding. Accept indicates server forwards client certificate but does not require a client certificate. Require indicates server requires a client certificate."]
    #[serde(rename = "clientCertificateMode", default, skip_serializing_if = "Option::is_none")]
    pub client_certificate_mode: Option<ingress::ClientCertificateMode>,
    #[doc = "Cross-Origin-Resource-Sharing policy"]
    #[serde(rename = "corsPolicy", default, skip_serializing_if = "Option::is_none")]
    pub cors_policy: Option<CorsPolicy>,
}
impl Ingress {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ingress {
    use super::*;
    #[doc = "Ingress transport protocol"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Transport")]
    pub enum Transport {
        #[serde(rename = "auto")]
        Auto,
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "http2")]
        Http2,
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Transport {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Transport {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Transport {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Auto => serializer.serialize_unit_variant("Transport", 0u32, "auto"),
                Self::Http => serializer.serialize_unit_variant("Transport", 1u32, "http"),
                Self::Http2 => serializer.serialize_unit_variant("Transport", 2u32, "http2"),
                Self::Tcp => serializer.serialize_unit_variant("Transport", 3u32, "tcp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Transport {
        fn default() -> Self {
            Self::Auto
        }
    }
    #[doc = "Sticky Sessions for Single Revision Mode"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct StickySessions {
        #[doc = "Sticky Session Affinity"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub affinity: Option<sticky_sessions::Affinity>,
    }
    impl StickySessions {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod sticky_sessions {
        use super::*;
        #[doc = "Sticky Session Affinity"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Affinity")]
        pub enum Affinity {
            #[serde(rename = "sticky")]
            Sticky,
            #[serde(rename = "none")]
            None,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Affinity {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Affinity {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Affinity {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Sticky => serializer.serialize_unit_variant("Affinity", 0u32, "sticky"),
                    Self::None => serializer.serialize_unit_variant("Affinity", 1u32, "none"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "Client certificate mode for mTLS authentication. Ignore indicates server drops client certificate on forwarding. Accept indicates server forwards client certificate but does not require a client certificate. Require indicates server requires a client certificate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClientCertificateMode")]
    pub enum ClientCertificateMode {
        #[serde(rename = "ignore")]
        Ignore,
        #[serde(rename = "accept")]
        Accept,
        #[serde(rename = "require")]
        Require,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClientCertificateMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClientCertificateMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClientCertificateMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ignore => serializer.serialize_unit_variant("ClientCertificateMode", 0u32, "ignore"),
                Self::Accept => serializer.serialize_unit_variant("ClientCertificateMode", 1u32, "accept"),
                Self::Require => serializer.serialize_unit_variant("ClientCertificateMode", 2u32, "require"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Container App init container definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InitContainer {
    #[serde(flatten)]
    pub base_container: BaseContainer,
}
impl InitContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rule to restrict incoming IP address."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpSecurityRestrictionRule {
    #[doc = "Name for the IP restriction rule."]
    pub name: String,
    #[doc = "Describe the IP restriction rule that is being sent to the container-app. This is an optional field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "CIDR notation to match incoming IP address"]
    #[serde(rename = "ipAddressRange")]
    pub ip_address_range: String,
    #[doc = "Allow or Deny rules to determine for incoming IP. Note: Rules can only consist of ALL Allow or ALL Deny"]
    pub action: ip_security_restriction_rule::Action,
}
impl IpSecurityRestrictionRule {
    pub fn new(name: String, ip_address_range: String, action: ip_security_restriction_rule::Action) -> Self {
        Self {
            name,
            description: None,
            ip_address_range,
            action,
        }
    }
}
pub mod ip_security_restriction_rule {
    use super::*;
    #[doc = "Allow or Deny rules to determine for incoming IP. Note: Rules can only consist of ALL Allow or ALL Deny"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Allow,
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Action", 0u32, "Allow"),
                Self::Deny => serializer.serialize_unit_variant("Action", 1u32, "Deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Container App Job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Container Apps Job resource specific properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<job::Properties>,
}
impl Job {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
        }
    }
}
pub mod job {
    use super::*;
    #[doc = "Container Apps Job resource specific properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the Container Apps Job."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Resource ID of environment."]
        #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
        pub environment_id: Option<String>,
        #[doc = "Workload profile name for container apps to execute on."]
        #[serde(rename = "workloadProfileName", default, skip_serializing_if = "Option::is_none")]
        pub workload_profile_name: Option<WorkloadProfileName>,
        #[doc = "Non versioned Container Apps Job configuration properties"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<JobConfiguration>,
        #[doc = "Container Apps Job versioned application definition. Defines the desired state of an immutable revision. Any changes to this section Will result in a new revision being created"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub template: Option<JobTemplate>,
        #[doc = "Outbound IP Addresses of a container apps job."]
        #[serde(
            rename = "outboundIpAddresses",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub outbound_ip_addresses: Vec<String>,
        #[doc = "The endpoint of the eventstream of the container apps job."]
        #[serde(rename = "eventStreamEndpoint", default, skip_serializing_if = "Option::is_none")]
        pub event_stream_endpoint: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Provisioning state of the Container Apps Job."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            InProgress,
            Succeeded,
            Failed,
            Canceled,
            Deleting,
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
                    Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 0u32, "InProgress"),
                    Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                    Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                    Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                    Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Non versioned Container Apps Job configuration properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobConfiguration {
    #[doc = "Collection of secrets used by a Container Apps Job"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub secrets: Vec<Secret>,
    #[doc = "Trigger type of the job"]
    #[serde(rename = "triggerType")]
    pub trigger_type: job_configuration::TriggerType,
    #[doc = "Maximum number of seconds a replica is allowed to run."]
    #[serde(rename = "replicaTimeout")]
    pub replica_timeout: i32,
    #[doc = "Maximum number of retries before failing the job."]
    #[serde(rename = "replicaRetryLimit", default, skip_serializing_if = "Option::is_none")]
    pub replica_retry_limit: Option<i32>,
    #[doc = "Manual trigger configuration for a single execution job. Properties replicaCompletionCount and parallelism would be set to 1 by default"]
    #[serde(rename = "manualTriggerConfig", default, skip_serializing_if = "Option::is_none")]
    pub manual_trigger_config: Option<job_configuration::ManualTriggerConfig>,
    #[doc = "Cron formatted repeating trigger schedule (\"* * * * *\") for cronjobs. Properties completions and parallelism would be set to 1 by default"]
    #[serde(rename = "scheduleTriggerConfig", default, skip_serializing_if = "Option::is_none")]
    pub schedule_trigger_config: Option<job_configuration::ScheduleTriggerConfig>,
    #[doc = "Trigger configuration of an event driven job."]
    #[serde(rename = "eventTriggerConfig", default, skip_serializing_if = "Option::is_none")]
    pub event_trigger_config: Option<job_configuration::EventTriggerConfig>,
    #[doc = "Collection of private container registry credentials used by a Container apps job"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub registries: Vec<RegistryCredentials>,
}
impl JobConfiguration {
    pub fn new(trigger_type: job_configuration::TriggerType, replica_timeout: i32) -> Self {
        Self {
            secrets: Vec::new(),
            trigger_type,
            replica_timeout,
            replica_retry_limit: None,
            manual_trigger_config: None,
            schedule_trigger_config: None,
            event_trigger_config: None,
            registries: Vec::new(),
        }
    }
}
pub mod job_configuration {
    use super::*;
    #[doc = "Trigger type of the job"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TriggerType")]
    pub enum TriggerType {
        Schedule,
        Event,
        Manual,
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
                Self::Schedule => serializer.serialize_unit_variant("TriggerType", 0u32, "Schedule"),
                Self::Event => serializer.serialize_unit_variant("TriggerType", 1u32, "Event"),
                Self::Manual => serializer.serialize_unit_variant("TriggerType", 2u32, "Manual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for TriggerType {
        fn default() -> Self {
            Self::Manual
        }
    }
    #[doc = "Manual trigger configuration for a single execution job. Properties replicaCompletionCount and parallelism would be set to 1 by default"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ManualTriggerConfig {
        #[doc = "Minimum number of successful replica completions before overall job completion."]
        #[serde(rename = "replicaCompletionCount", default, skip_serializing_if = "Option::is_none")]
        pub replica_completion_count: Option<ReplicaCompletionCount>,
        #[doc = "Number of parallel replicas of a job that can run at a given time."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parallelism: Option<Parallelism>,
    }
    impl ManualTriggerConfig {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Cron formatted repeating trigger schedule (\"* * * * *\") for cronjobs. Properties completions and parallelism would be set to 1 by default"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ScheduleTriggerConfig {
        #[doc = "Minimum number of successful replica completions before overall job completion."]
        #[serde(rename = "replicaCompletionCount", default, skip_serializing_if = "Option::is_none")]
        pub replica_completion_count: Option<ReplicaCompletionCount>,
        #[doc = "Cron formatted repeating schedule (\"* * * * *\") of a Cron Job."]
        #[serde(rename = "cronExpression")]
        pub cron_expression: String,
        #[doc = "Number of parallel replicas of a job that can run at a given time."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parallelism: Option<Parallelism>,
    }
    impl ScheduleTriggerConfig {
        pub fn new(cron_expression: String) -> Self {
            Self {
                replica_completion_count: None,
                cron_expression,
                parallelism: None,
            }
        }
    }
    #[doc = "Trigger configuration of an event driven job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct EventTriggerConfig {
        #[doc = "Minimum number of successful replica completions before overall job completion."]
        #[serde(rename = "replicaCompletionCount", default, skip_serializing_if = "Option::is_none")]
        pub replica_completion_count: Option<ReplicaCompletionCount>,
        #[doc = "Number of parallel replicas of a job that can run at a given time."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parallelism: Option<Parallelism>,
        #[doc = "Scaling configurations for event driven jobs."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scale: Option<JobScale>,
    }
    impl EventTriggerConfig {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Container Apps Job execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobExecution {
    #[doc = "Job execution Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Job execution Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Job Type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Current running State of the job"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_execution::Status>,
    #[doc = "Job execution start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job execution end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Job's execution template, containing container configuration for a job's execution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<JobExecutionTemplate>,
}
impl JobExecution {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_execution {
    use super::*;
    #[doc = "Current running State of the job"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Running,
        Processing,
        Stopped,
        Degraded,
        Failed,
        Unknown,
        Succeeded,
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
                Self::Running => serializer.serialize_unit_variant("Status", 0u32, "Running"),
                Self::Processing => serializer.serialize_unit_variant("Status", 1u32, "Processing"),
                Self::Stopped => serializer.serialize_unit_variant("Status", 2u32, "Stopped"),
                Self::Degraded => serializer.serialize_unit_variant("Status", 3u32, "Degraded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::Unknown => serializer.serialize_unit_variant("Status", 5u32, "Unknown"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 6u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Container App's Job execution name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobExecutionBase {
    #[doc = "Job execution name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Job execution Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl JobExecutionBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container Apps Jobs execution container definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobExecutionContainer {
    #[doc = "Container image tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[doc = "Custom container name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Container start command."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub command: Vec<String>,
    #[doc = "Container start command arguments."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub args: Vec<String>,
    #[doc = "Container environment variables."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub env: Vec<EnvironmentVar>,
    #[doc = "Container App container resource requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ContainerResources>,
}
impl JobExecutionContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App executions names list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobExecutionNamesCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<JobExecutionBase>,
}
impl JobExecutionNamesCollection {
    pub fn new(value: Vec<JobExecutionBase>) -> Self {
        Self { value }
    }
}
#[doc = "Job's execution template, containing container configuration for a job's execution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobExecutionTemplate {
    #[doc = "List of container definitions for the Container Apps Job."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub containers: Vec<JobExecutionContainer>,
    #[doc = "List of specialized containers that run before job containers."]
    #[serde(
        rename = "initContainers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub init_containers: Vec<JobExecutionContainer>,
}
impl JobExecutionTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container Apps Job resource specific properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobPatchProperties {
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<job_patch_properties::Properties>,
}
impl JobPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Resource ID of environment."]
        #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
        pub environment_id: Option<String>,
        #[doc = "Non versioned Container Apps Job configuration properties"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<JobConfiguration>,
        #[doc = "Container Apps Job versioned application definition. Defines the desired state of an immutable revision. Any changes to this section Will result in a new revision being created"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub template: Option<JobTemplate>,
        #[doc = "Outbound IP Addresses of a container apps job."]
        #[serde(
            rename = "outboundIpAddresses",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub outbound_ip_addresses: Vec<String>,
        #[doc = "The endpoint of the eventstream of the container apps job."]
        #[serde(rename = "eventStreamEndpoint", default, skip_serializing_if = "Option::is_none")]
        pub event_stream_endpoint: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Scaling configurations for event driven jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobScale {
    #[doc = "Interval to check each event source in seconds. Defaults to 30s"]
    #[serde(rename = "pollingInterval", default, skip_serializing_if = "Option::is_none")]
    pub polling_interval: Option<PollingInterval>,
    #[doc = "Minimum number of job executions that are created for a trigger, default 0"]
    #[serde(rename = "minExecutions", default, skip_serializing_if = "Option::is_none")]
    pub min_executions: Option<i32>,
    #[doc = "Maximum number of job executions that are created for a trigger, default 100."]
    #[serde(rename = "maxExecutions", default, skip_serializing_if = "Option::is_none")]
    pub max_executions: Option<i32>,
    #[doc = "Scaling rules."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rules: Vec<JobScaleRule>,
}
impl JobScale {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scaling rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobScaleRule {
    #[doc = "Scale Rule Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the scale rule\neg: azure-servicebus, redis etc."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata properties to describe the scale rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "Authentication secrets for the scale rule."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub auth: Vec<ScaleRuleAuth>,
}
impl JobScaleRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container Apps Job Secrets Collection ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobSecretsCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Secret>,
}
impl JobSecretsCollection {
    pub fn new(value: Vec<Secret>) -> Self {
        Self { value }
    }
}
#[doc = "Container Apps Job versioned application definition. Defines the desired state of an immutable revision. Any changes to this section Will result in a new revision being created"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobTemplate {
    #[doc = "List of specialized containers that run before app containers."]
    #[serde(
        rename = "initContainers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub init_containers: Vec<InitContainer>,
    #[doc = "List of container definitions for the Container App."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub containers: Vec<Container>,
    #[doc = "List of volume definitions for the Container App."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volumes: Vec<Volume>,
}
impl JobTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container Apps Jobs collection ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobsCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Job>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobsCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobsCollection {
    pub fn new(value: Vec<Job>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The configuration settings of the checks that should be made while validating the JWT Claims."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JwtClaimChecks {
    #[doc = "The list of the allowed groups."]
    #[serde(
        rename = "allowedGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_groups: Vec<String>,
    #[doc = "The list of the allowed client applications."]
    #[serde(
        rename = "allowedClientApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_client_applications: Vec<String>,
}
impl JwtClaimChecks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration properties Keda component"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KedaConfiguration {
    #[doc = "The version of Keda"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl KedaConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log Analytics configuration, must only be provided when destination is configured as 'log-analytics'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsConfiguration {
    #[doc = "Log analytics customer id"]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "Log analytics customer key"]
    #[serde(rename = "sharedKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_key: Option<String>,
}
impl LogAnalyticsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration settings of the login flow of users using ContainerApp Service Authentication/Authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Login {
    #[doc = "The routes that specify the endpoints used for login and logout requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<LoginRoutes>,
    #[doc = "<code>true</code> if the fragments from the request are preserved after the login request is made; otherwise, <code>false</code>."]
    #[serde(rename = "preserveUrlFragmentsForLogins", default, skip_serializing_if = "Option::is_none")]
    pub preserve_url_fragments_for_logins: Option<bool>,
    #[doc = "External URLs that can be redirected to as part of logging in or logging out of the app. Note that the query string part of the URL is ignored.\nThis is an advanced setting typically only needed by Windows Store application backends.\nNote that URLs within the current domain are always implicitly allowed."]
    #[serde(
        rename = "allowedExternalRedirectUrls",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<String>,
}
impl LoginScopes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed certificates used for Custom Domain bindings of Container Apps in a Managed Environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedCertificate {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Certificate resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<managed_certificate::Properties>,
}
impl ManagedCertificate {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
pub mod managed_certificate {
    use super::*;
    #[doc = "Certificate resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the certificate."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Subject name of the certificate."]
        #[serde(rename = "subjectName", default, skip_serializing_if = "Option::is_none")]
        pub subject_name: Option<String>,
        #[doc = "Any error occurred during the certificate provision."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        #[doc = "Selected type of domain control validation for managed certificates."]
        #[serde(rename = "domainControlValidation", default, skip_serializing_if = "Option::is_none")]
        pub domain_control_validation: Option<properties::DomainControlValidation>,
        #[doc = "A TXT token used for DNS TXT domain control validation when issuing this type of managed certificates."]
        #[serde(rename = "validationToken", default, skip_serializing_if = "Option::is_none")]
        pub validation_token: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Provisioning state of the certificate."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            DeleteFailed,
            Pending,
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
                    Self::DeleteFailed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "DeleteFailed"),
                    Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Selected type of domain control validation for managed certificates."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "DomainControlValidation")]
        pub enum DomainControlValidation {
            #[serde(rename = "CNAME")]
            Cname,
            #[serde(rename = "HTTP")]
            Http,
            #[serde(rename = "TXT")]
            Txt,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for DomainControlValidation {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for DomainControlValidation {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for DomainControlValidation {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Cname => serializer.serialize_unit_variant("DomainControlValidation", 0u32, "CNAME"),
                    Self::Http => serializer.serialize_unit_variant("DomainControlValidation", 1u32, "HTTP"),
                    Self::Txt => serializer.serialize_unit_variant("DomainControlValidation", 2u32, "TXT"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Collection of Managed Certificates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedCertificateCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ManagedCertificate>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedCertificateCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedCertificateCollection {
    pub fn new(value: Vec<ManagedCertificate>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A managed certificate to update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedCertificatePatch {
    #[doc = "Application-specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ManagedCertificatePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An environment for hosting container apps"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedEnvironment {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Kind of the Environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Managed environment resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<managed_environment::Properties>,
}
impl ManagedEnvironment {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            kind: None,
            properties: None,
        }
    }
}
pub mod managed_environment {
    use super::*;
    #[doc = "Managed environment resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the Environment."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Azure Monitor instrumentation key used by Dapr to export Service to Service communication telemetry"]
        #[serde(rename = "daprAIInstrumentationKey", default, skip_serializing_if = "Option::is_none")]
        pub dapr_ai_instrumentation_key: Option<String>,
        #[doc = "Application Insights connection string used by Dapr to export Service to Service communication telemetry"]
        #[serde(rename = "daprAIConnectionString", default, skip_serializing_if = "Option::is_none")]
        pub dapr_ai_connection_string: Option<String>,
        #[doc = "Configuration properties for apps environment to join a Virtual Network"]
        #[serde(rename = "vnetConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub vnet_configuration: Option<VnetConfiguration>,
        #[doc = "Any errors that occurred during deployment or deployment validation"]
        #[serde(rename = "deploymentErrors", default, skip_serializing_if = "Option::is_none")]
        pub deployment_errors: Option<String>,
        #[doc = "Default Domain Name for the cluster"]
        #[serde(rename = "defaultDomain", default, skip_serializing_if = "Option::is_none")]
        pub default_domain: Option<String>,
        #[doc = "Static IP of the Environment"]
        #[serde(rename = "staticIp", default, skip_serializing_if = "Option::is_none")]
        pub static_ip: Option<String>,
        #[doc = "Configuration of application logs"]
        #[serde(rename = "appLogsConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub app_logs_configuration: Option<AppLogsConfiguration>,
        #[doc = "Whether or not this Managed Environment is zone-redundant."]
        #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
        pub zone_redundant: Option<bool>,
        #[doc = "Configuration properties for apps environment custom domain"]
        #[serde(rename = "customDomainConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub custom_domain_configuration: Option<CustomDomainConfiguration>,
        #[doc = "The endpoint of the eventstream of the Environment."]
        #[serde(rename = "eventStreamEndpoint", default, skip_serializing_if = "Option::is_none")]
        pub event_stream_endpoint: Option<String>,
        #[doc = "Workload profiles configured for the Managed Environment."]
        #[serde(
            rename = "workloadProfiles",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub workload_profiles: Vec<WorkloadProfile>,
        #[doc = "Configuration properties Keda component"]
        #[serde(rename = "kedaConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub keda_configuration: Option<KedaConfiguration>,
        #[doc = "Configuration properties Dapr component"]
        #[serde(rename = "daprConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub dapr_configuration: Option<DaprConfiguration>,
        #[doc = "Name of the platform-managed resource group created for the Managed Environment to host infrastructure resources. If a subnet ID is provided, this resource group will be created in the same subscription as the subnet."]
        #[serde(rename = "infrastructureResourceGroup", default, skip_serializing_if = "Option::is_none")]
        pub infrastructure_resource_group: Option<String>,
        #[doc = "Peer authentication settings for the Managed Environment"]
        #[serde(rename = "peerAuthentication", default, skip_serializing_if = "Option::is_none")]
        pub peer_authentication: Option<properties::PeerAuthentication>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Provisioning state of the Environment."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
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
                    Self::Waiting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Waiting"),
                    Self::InitializationInProgress => {
                        serializer.serialize_unit_variant("ProvisioningState", 4u32, "InitializationInProgress")
                    }
                    Self::InfrastructureSetupInProgress => {
                        serializer.serialize_unit_variant("ProvisioningState", 5u32, "InfrastructureSetupInProgress")
                    }
                    Self::InfrastructureSetupComplete => {
                        serializer.serialize_unit_variant("ProvisioningState", 6u32, "InfrastructureSetupComplete")
                    }
                    Self::ScheduledForDelete => serializer.serialize_unit_variant("ProvisioningState", 7u32, "ScheduledForDelete"),
                    Self::UpgradeRequested => serializer.serialize_unit_variant("ProvisioningState", 8u32, "UpgradeRequested"),
                    Self::UpgradeFailed => serializer.serialize_unit_variant("ProvisioningState", 9u32, "UpgradeFailed"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Peer authentication settings for the Managed Environment"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct PeerAuthentication {
            #[doc = "Configuration properties for mutual TLS authentication"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub mtls: Option<Mtls>,
        }
        impl PeerAuthentication {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "Storage resource for managedEnvironment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedEnvironmentStorage {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Storage properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<managed_environment_storage::Properties>,
}
impl ManagedEnvironmentStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_environment_storage {
    use super::*;
    #[doc = "Storage properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Azure File Properties."]
        #[serde(rename = "azureFile", default, skip_serializing_if = "Option::is_none")]
        pub azure_file: Option<AzureFileProperties>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of Storage for Environments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedEnvironmentStoragesCollection {
    #[doc = "Collection of storage resources."]
    pub value: Vec<ManagedEnvironmentStorage>,
}
impl ManagedEnvironmentStoragesCollection {
    pub fn new(value: Vec<ManagedEnvironmentStorage>) -> Self {
        Self { value }
    }
}
#[doc = "Collection of Environments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedEnvironmentsCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ManagedEnvironment>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedEnvironmentsCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedEnvironmentsCollection {
    pub fn new(value: Vec<ManagedEnvironment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned,UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned,UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Configuration properties for mutual TLS authentication"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Mtls {
    #[doc = "Boolean indicating whether the mutual TLS authentication is enabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl Mtls {
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Operation detail payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDetail {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Operation display payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation display payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Resource provider of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Localized friendly name for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Localized friendly description for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Parallelism = i32;
pub type PollingInterval = i32;
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
#[doc = "Container App container Azure Queue based scaling rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueueScaleRule {
    #[doc = "Queue name."]
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[doc = "Queue length."]
    #[serde(rename = "queueLength", default, skip_serializing_if = "Option::is_none")]
    pub queue_length: Option<i32>,
    #[doc = "Authentication secrets for the queue scale rule."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub auth: Vec<ScaleRuleAuth>,
}
impl QueueScaleRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App Private Registry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryCredentials {
    #[doc = "Container Registry Server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "Container Registry Username"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The name of the Secret that contains the registry login password"]
    #[serde(rename = "passwordSecretRef", default, skip_serializing_if = "Option::is_none")]
    pub password_secret_ref: Option<String>,
    #[doc = "A Managed Identity to use to authenticate with Azure Container Registry. For user-assigned identities, use the full user-assigned identity Resource ID. For system-assigned identities, use 'system'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
}
impl RegistryCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App registry information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistryInfo {
    #[doc = "registry server Url."]
    #[serde(rename = "registryUrl", default, skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
    #[doc = "registry username."]
    #[serde(rename = "registryUserName", default, skip_serializing_if = "Option::is_none")]
    pub registry_user_name: Option<String>,
    #[doc = "registry secret."]
    #[serde(rename = "registryPassword", default, skip_serializing_if = "Option::is_none")]
    pub registry_password: Option<String>,
}
impl RegistryInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App Revision Replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Replica {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Replica resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<replica::Properties>,
}
impl Replica {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod replica {
    use super::*;
    #[doc = "Replica resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Timestamp describing when the pod was created by controller"]
        #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
        pub created_time: Option<time::OffsetDateTime>,
        #[doc = "Current running state of the replica"]
        #[serde(rename = "runningState", default, skip_serializing_if = "Option::is_none")]
        pub running_state: Option<properties::RunningState>,
        #[doc = "The details of replica current running state"]
        #[serde(rename = "runningStateDetails", default, skip_serializing_if = "Option::is_none")]
        pub running_state_details: Option<String>,
        #[doc = "The containers collection under a replica."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub containers: Vec<ReplicaContainer>,
        #[doc = "The init containers collection under a replica."]
        #[serde(
            rename = "initContainers",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub init_containers: Vec<ReplicaContainer>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Current running state of the replica"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "RunningState")]
        pub enum RunningState {
            Running,
            NotRunning,
            Unknown,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for RunningState {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for RunningState {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for RunningState {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Running => serializer.serialize_unit_variant("RunningState", 0u32, "Running"),
                    Self::NotRunning => serializer.serialize_unit_variant("RunningState", 1u32, "NotRunning"),
                    Self::Unknown => serializer.serialize_unit_variant("RunningState", 2u32, "Unknown"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Container App Revision Replicas collection ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicaCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Replica>,
}
impl ReplicaCollection {
    pub fn new(value: Vec<Replica>) -> Self {
        Self { value }
    }
}
pub type ReplicaCompletionCount = i32;
#[doc = "Container object under Container App Revision Replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicaContainer {
    #[doc = "The Name of the Container"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Id of the Container"]
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[doc = "The container ready status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[doc = "The container start status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<bool>,
    #[doc = "The container restart count"]
    #[serde(rename = "restartCount", default, skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    #[doc = "Current running state of the container"]
    #[serde(rename = "runningState", default, skip_serializing_if = "Option::is_none")]
    pub running_state: Option<replica_container::RunningState>,
    #[doc = "The details of container current running state"]
    #[serde(rename = "runningStateDetails", default, skip_serializing_if = "Option::is_none")]
    pub running_state_details: Option<String>,
    #[doc = "Log Stream endpoint"]
    #[serde(rename = "logStreamEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub log_stream_endpoint: Option<String>,
    #[doc = "Container exec endpoint"]
    #[serde(rename = "execEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub exec_endpoint: Option<String>,
}
impl ReplicaContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod replica_container {
    use super::*;
    #[doc = "Current running state of the container"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RunningState")]
    pub enum RunningState {
        Running,
        Terminated,
        Waiting,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RunningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RunningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RunningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Running => serializer.serialize_unit_variant("RunningState", 0u32, "Running"),
                Self::Terminated => serializer.serialize_unit_variant("RunningState", 1u32, "Terminated"),
                Self::Waiting => serializer.serialize_unit_variant("RunningState", 2u32, "Waiting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "Container App Revision."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Revision {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Revision resource specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<revision::Properties>,
}
impl Revision {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod revision {
    use super::*;
    #[doc = "Revision resource specific properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Timestamp describing when the revision was created\nby controller"]
        #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
        pub created_time: Option<time::OffsetDateTime>,
        #[doc = "Timestamp describing when the revision was last active. Only meaningful when revision is inactive"]
        #[serde(rename = "lastActiveTime", default, with = "azure_core::date::rfc3339::option")]
        pub last_active_time: Option<time::OffsetDateTime>,
        #[doc = "Fully qualified domain name of the revision"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fqdn: Option<String>,
        #[doc = "Container App versioned application definition.\nDefines the desired state of an immutable revision.\nAny changes to this section Will result in a new revision being created"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub template: Option<Template>,
        #[doc = "Boolean describing if the Revision is Active"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
        #[doc = "Number of pods currently running for this revision"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replicas: Option<i32>,
        #[doc = "Traffic weight assigned to this revision"]
        #[serde(rename = "trafficWeight", default, skip_serializing_if = "Option::is_none")]
        pub traffic_weight: Option<i32>,
        #[doc = "Optional Field - Platform Error Message"]
        #[serde(rename = "provisioningError", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_error: Option<String>,
        #[doc = "Current health State of the revision"]
        #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
        pub health_state: Option<properties::HealthState>,
        #[doc = "Current provisioning State of the revision"]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "Current running state of the revision"]
        #[serde(rename = "runningState", default, skip_serializing_if = "Option::is_none")]
        pub running_state: Option<properties::RunningState>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Current health State of the revision"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "HealthState")]
        pub enum HealthState {
            Healthy,
            Unhealthy,
            None,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for HealthState {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for HealthState {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for HealthState {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Healthy => serializer.serialize_unit_variant("HealthState", 0u32, "Healthy"),
                    Self::Unhealthy => serializer.serialize_unit_variant("HealthState", 1u32, "Unhealthy"),
                    Self::None => serializer.serialize_unit_variant("HealthState", 2u32, "None"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Current provisioning State of the revision"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            Provisioning,
            Provisioned,
            Failed,
            Deprovisioning,
            Deprovisioned,
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
                    Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Provisioning"),
                    Self::Provisioned => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Provisioned"),
                    Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                    Self::Deprovisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deprovisioning"),
                    Self::Deprovisioned => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deprovisioned"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Current running state of the revision"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "RunningState")]
        pub enum RunningState {
            Running,
            Processing,
            Stopped,
            Degraded,
            Failed,
            Unknown,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for RunningState {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for RunningState {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for RunningState {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Running => serializer.serialize_unit_variant("RunningState", 0u32, "Running"),
                    Self::Processing => serializer.serialize_unit_variant("RunningState", 1u32, "Processing"),
                    Self::Stopped => serializer.serialize_unit_variant("RunningState", 2u32, "Stopped"),
                    Self::Degraded => serializer.serialize_unit_variant("RunningState", 3u32, "Degraded"),
                    Self::Failed => serializer.serialize_unit_variant("RunningState", 4u32, "Failed"),
                    Self::Unknown => serializer.serialize_unit_variant("RunningState", 5u32, "Unknown"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Container App Revisions collection ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RevisionCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<Revision>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RevisionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RevisionCollection {
    pub fn new(value: Vec<Revision>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Container App scaling configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scale {
    #[doc = "Optional. Minimum number of container replicas."]
    #[serde(rename = "minReplicas", default, skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,
    #[doc = "Optional. Maximum number of container replicas. Defaults to 10 if not set."]
    #[serde(rename = "maxReplicas", default, skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i32>,
    #[doc = "Scaling rules."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rules: Vec<ScaleRule>,
}
impl Scale {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App container scaling rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScaleRule {
    #[doc = "Scale Rule Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Container App container Azure Queue based scaling rule."]
    #[serde(rename = "azureQueue", default, skip_serializing_if = "Option::is_none")]
    pub azure_queue: Option<QueueScaleRule>,
    #[doc = "Container App container Custom scaling rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomScaleRule>,
    #[doc = "Container App container Http scaling rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpScaleRule>,
    #[doc = "Container App container Tcp scaling rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<TcpScaleRule>,
}
impl ScaleRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Auth Secrets for Scale Rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScaleRuleAuth {
    #[doc = "Name of the secret from which to pull the auth params."]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<String>,
    #[doc = "Trigger Parameter that uses the secret"]
    #[serde(rename = "triggerParameter", default, skip_serializing_if = "Option::is_none")]
    pub trigger_parameter: Option<String>,
}
impl ScaleRuleAuth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Secret definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Secret {
    #[doc = "Secret Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Secret Value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Resource ID of a managed identity to authenticate with Azure Key Vault, or System to use a system-assigned identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "Azure Key Vault URL pointing to the secret referenced by the container app."]
    #[serde(rename = "keyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_url: Option<String>,
}
impl Secret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Secret to be added to volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretVolumeItem {
    #[doc = "Name of the Container App secret from which to pull the secret value."]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<String>,
    #[doc = "Path to project secret to. If no path is provided, path defaults to name of secret listed in secretRef."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl SecretVolumeItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App Secrets Collection ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretsCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<ContainerAppSecret>,
}
impl SecretsCollection {
    pub fn new(value: Vec<ContainerAppSecret>) -> Self {
        Self { value }
    }
}
#[doc = "Container App to be a dev service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[doc = "Dev ContainerApp service type"]
    #[serde(rename = "type")]
    pub type_: String,
}
impl Service {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Configuration to bind a ContainerApp to a dev ContainerApp Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBind {
    #[doc = "Resource id of the target service"]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "Name of the service bind"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ServiceBind {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App SourceControl."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControl {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
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
        #[doc = "Current provisioning State of the operation"]
        #[serde(rename = "operationState", default, skip_serializing_if = "Option::is_none")]
        pub operation_state: Option<properties::OperationState>,
        #[doc = "The repo url which will be integrated to ContainerApp."]
        #[serde(rename = "repoUrl", default, skip_serializing_if = "Option::is_none")]
        pub repo_url: Option<String>,
        #[doc = "The branch which will trigger the auto deployment"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub branch: Option<String>,
        #[doc = "Configuration properties that define the mutable settings of a Container App SourceControl"]
        #[serde(rename = "githubActionConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub github_action_configuration: Option<GithubActionConfiguration>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Current provisioning State of the operation"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "OperationState")]
        pub enum OperationState {
            InProgress,
            Succeeded,
            Failed,
            Canceled,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for OperationState {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for OperationState {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for OperationState {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::InProgress => serializer.serialize_unit_variant("OperationState", 0u32, "InProgress"),
                    Self::Succeeded => serializer.serialize_unit_variant("OperationState", 1u32, "Succeeded"),
                    Self::Failed => serializer.serialize_unit_variant("OperationState", 2u32, "Failed"),
                    Self::Canceled => serializer.serialize_unit_variant("OperationState", 3u32, "Canceled"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "SourceControl collection ARM resource."]
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SourceControlCollection {
    pub fn new(value: Vec<SourceControl>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Container App container Tcp scaling rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TcpScaleRule {
    #[doc = "Metadata properties to describe tcp scale rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "Authentication secrets for the tcp scale rule."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub auth: Vec<ScaleRuleAuth>,
}
impl TcpScaleRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container App versioned application definition.\nDefines the desired state of an immutable revision.\nAny changes to this section Will result in a new revision being created"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Template {
    #[doc = "User friendly suffix that is appended to the revision name"]
    #[serde(rename = "revisionSuffix", default, skip_serializing_if = "Option::is_none")]
    pub revision_suffix: Option<String>,
    #[doc = "Optional duration in seconds the Container App Instance needs to terminate gracefully. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). If this value is nil, the default grace period will be used instead. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds."]
    #[serde(rename = "terminationGracePeriodSeconds", default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    #[doc = "List of specialized containers that run before app containers."]
    #[serde(
        rename = "initContainers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub init_containers: Vec<InitContainer>,
    #[doc = "List of container definitions for the Container App."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub containers: Vec<Container>,
    #[doc = "Container App scaling configurations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,
    #[doc = "List of volume definitions for the Container App."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volumes: Vec<Volume>,
    #[doc = "List of container app services bound to the app"]
    #[serde(
        rename = "serviceBinds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_binds: Vec<ServiceBind>,
}
impl Template {
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
#[doc = "Traffic weight assigned to a revision"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrafficWeight {
    #[doc = "Name of a revision"]
    #[serde(rename = "revisionName", default, skip_serializing_if = "Option::is_none")]
    pub revision_name: Option<String>,
    #[doc = "Traffic weight assigned to a revision"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[doc = "Indicates that the traffic weight belongs to a latest stable revision"]
    #[serde(rename = "latestRevision", default, skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<bool>,
    #[doc = "Associates a traffic label with a revision"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
impl TrafficWeight {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration properties for apps environment to join a Virtual Network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VnetConfiguration {
    #[doc = "Boolean indicating the environment only has an internal load balancer. These environments do not have a public static IP resource. They must provide infrastructureSubnetId if enabling this property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[doc = "Resource ID of a subnet for infrastructure components. Must not overlap with any other provided IP ranges."]
    #[serde(rename = "infrastructureSubnetId", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_subnet_id: Option<String>,
    #[doc = "CIDR notation IP range assigned to the Docker bridge, network. Must not overlap with any other provided IP ranges."]
    #[serde(rename = "dockerBridgeCidr", default, skip_serializing_if = "Option::is_none")]
    pub docker_bridge_cidr: Option<String>,
    #[doc = "IP range in CIDR notation that can be reserved for environment infrastructure IP addresses. Must not overlap with any other provided IP ranges."]
    #[serde(rename = "platformReservedCidr", default, skip_serializing_if = "Option::is_none")]
    pub platform_reserved_cidr: Option<String>,
    #[doc = " An IP address from the IP range defined by platformReservedCidr that will be reserved for the internal DNS server."]
    #[serde(rename = "platformReservedDnsIP", default, skip_serializing_if = "Option::is_none")]
    pub platform_reserved_dns_ip: Option<String>,
}
impl VnetConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume definitions for the Container App."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Volume {
    #[doc = "Volume name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Storage type for the volume. If not provided, use EmptyDir."]
    #[serde(rename = "storageType", default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<volume::StorageType>,
    #[doc = "Name of storage resource. No need to provide for EmptyDir and Secret."]
    #[serde(rename = "storageName", default, skip_serializing_if = "Option::is_none")]
    pub storage_name: Option<String>,
    #[doc = "List of secrets to be added in volume. If no secrets are provided, all secrets in collection will be added to volume."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub secrets: Vec<SecretVolumeItem>,
    #[doc = "Mount options used while mounting the AzureFile. Must be a comma-separated string."]
    #[serde(rename = "mountOptions", default, skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<String>,
}
impl Volume {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod volume {
    use super::*;
    #[doc = "Storage type for the volume. If not provided, use EmptyDir."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageType")]
    pub enum StorageType {
        AzureFile,
        EmptyDir,
        Secret,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureFile => serializer.serialize_unit_variant("StorageType", 0u32, "AzureFile"),
                Self::EmptyDir => serializer.serialize_unit_variant("StorageType", 1u32, "EmptyDir"),
                Self::Secret => serializer.serialize_unit_variant("StorageType", 2u32, "Secret"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Volume mount for the Container App."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeMount {
    #[doc = "This must match the Name of a Volume."]
    #[serde(rename = "volumeName", default, skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
    #[doc = "Path within the container at which the volume should be mounted.Must not contain ':'."]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    #[doc = "Path within the volume from which the container's volume should be mounted. Defaults to \"\" (volume's root)."]
    #[serde(rename = "subPath", default, skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
}
impl VolumeMount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload profile to scope container app execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadProfile {
    #[doc = "Workload profile name for container apps to execute on."]
    pub name: WorkloadProfileName,
    #[doc = "Workload profile type for container apps to execute on."]
    #[serde(rename = "workloadProfileType")]
    pub workload_profile_type: WorkloadProfileType,
    #[doc = "The minimum capacity."]
    #[serde(rename = "minimumCount", default, skip_serializing_if = "Option::is_none")]
    pub minimum_count: Option<i32>,
    #[doc = "The maximum capacity."]
    #[serde(rename = "maximumCount", default, skip_serializing_if = "Option::is_none")]
    pub maximum_count: Option<i32>,
}
impl WorkloadProfile {
    pub fn new(name: WorkloadProfileName, workload_profile_type: WorkloadProfileType) -> Self {
        Self {
            name,
            workload_profile_type,
            minimum_count: None,
            maximum_count: None,
        }
    }
}
pub type WorkloadProfileName = String;
pub type WorkloadProfileType = String;
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
#[doc = "Collection of all the workload Profile States for a Managed Environment.."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadProfileStates {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Workload Profile resource specific properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<workload_profile_states::Properties>,
}
impl WorkloadProfileStates {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workload_profile_states {
    use super::*;
    #[doc = "Workload Profile resource specific properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Minimum count of instances."]
        #[serde(rename = "minimumCount", default, skip_serializing_if = "Option::is_none")]
        pub minimum_count: Option<i32>,
        #[doc = "Maximum count of nodes."]
        #[serde(rename = "maximumCount", default, skip_serializing_if = "Option::is_none")]
        pub maximum_count: Option<i32>,
        #[doc = "Current count of nodes."]
        #[serde(rename = "currentCount", default, skip_serializing_if = "Option::is_none")]
        pub current_count: Option<i32>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of workloadProfileStates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadProfileStatesCollection {
    #[doc = "Collection of resources."]
    pub value: Vec<WorkloadProfileStates>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadProfileStatesCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadProfileStatesCollection {
    pub fn new(value: Vec<WorkloadProfileStates>) -> Self {
        Self { value, next_link: None }
    }
}
