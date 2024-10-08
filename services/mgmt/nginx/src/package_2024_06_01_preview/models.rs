#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The request body for creating an analysis for an NGINX configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisCreate {
    pub config: analysis_create::Config,
}
impl AnalysisCreate {
    pub fn new(config: analysis_create::Config) -> Self {
        Self { config }
    }
}
pub mod analysis_create {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Config {
        #[doc = "The root file of the NGINX config file(s). It must match one of the files' filepath."]
        #[serde(rename = "rootFile", default, skip_serializing_if = "Option::is_none")]
        pub root_file: Option<String>,
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub files: Vec<NginxConfigurationFile>,
        #[serde(
            rename = "protectedFiles",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub protected_files: Vec<NginxConfigurationFile>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub package: Option<NginxConfigurationPackage>,
    }
    impl Config {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "An error object found during the analysis of an NGINX configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisDiagnostic {
    #[doc = "Unique identifier for the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub directive: String,
    pub description: String,
    #[doc = "the filepath of the most relevant config file"]
    pub file: String,
    pub line: f64,
    pub message: String,
    pub rule: String,
}
impl AnalysisDiagnostic {
    pub fn new(directive: String, description: String, file: String, line: f64, message: String, rule: String) -> Self {
        Self {
            id: None,
            directive,
            description,
            file,
            line,
            message,
            rule,
        }
    }
}
#[doc = "The response body for an analysis request. Contains the status of the analysis and any errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisResult {
    #[doc = "The status of the analysis."]
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<analysis_result::Data>,
}
impl AnalysisResult {
    pub fn new(status: String) -> Self {
        Self { status, data: None }
    }
}
pub mod analysis_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Data {
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub errors: Vec<AnalysisDiagnostic>,
    }
    impl Data {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Autoupgrade settings of a deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoUpgradeProfile {
    #[doc = "Channel used for autoupgrade."]
    #[serde(rename = "upgradeChannel")]
    pub upgrade_channel: String,
}
impl AutoUpgradeProfile {
    pub fn new(upgrade_channel: String) -> Self {
        Self { upgrade_channel }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProperties {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<IdentityType>,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl IdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IdentityType")]
pub enum IdentityType {
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned, UserAssigned")]
    SystemAssignedUserAssigned,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SystemAssigned => serializer.serialize_unit_variant("IdentityType", 0u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("IdentityType", 1u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("IdentityType", 2u32, "SystemAssigned, UserAssigned"),
            Self::None => serializer.serialize_unit_variant("IdentityType", 3u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxCertificate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NginxCertificateProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl NginxCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxCertificateErrorResponseBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NginxCertificateErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxCertificateListResponse {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NginxCertificate>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NginxCertificateListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NginxCertificateListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxCertificateProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "keyVirtualPath", default, skip_serializing_if = "Option::is_none")]
    pub key_virtual_path: Option<String>,
    #[serde(rename = "certificateVirtualPath", default, skip_serializing_if = "Option::is_none")]
    pub certificate_virtual_path: Option<String>,
    #[serde(rename = "keyVaultSecretId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_secret_id: Option<String>,
    #[serde(rename = "sha1Thumbprint", default, skip_serializing_if = "Option::is_none")]
    pub sha1_thumbprint: Option<String>,
    #[serde(rename = "keyVaultSecretVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_secret_version: Option<String>,
    #[serde(rename = "keyVaultSecretCreated", default, with = "azure_core::date::rfc3339::option")]
    pub key_vault_secret_created: Option<::time::OffsetDateTime>,
    #[serde(rename = "certificateError", default, skip_serializing_if = "Option::is_none")]
    pub certificate_error: Option<NginxCertificateErrorResponseBody>,
}
impl NginxCertificateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NginxConfigurationProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl NginxConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxConfigurationFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "virtualPath", default, skip_serializing_if = "Option::is_none")]
    pub virtual_path: Option<String>,
}
impl NginxConfigurationFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxConfigurationListResponse {
    #[doc = "Results of a list operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NginxConfiguration>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NginxConfigurationListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NginxConfigurationListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxConfigurationPackage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(
        rename = "protectedFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protected_files: Vec<String>,
}
impl NginxConfigurationPackage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxConfigurationProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<NginxConfigurationFile>,
    #[serde(
        rename = "protectedFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protected_files: Vec<NginxConfigurationFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<NginxConfigurationPackage>,
    #[serde(rename = "rootFile", default, skip_serializing_if = "Option::is_none")]
    pub root_file: Option<String>,
}
impl NginxConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxDeployment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NginxDeploymentProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl NginxDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxDeploymentListResponse {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NginxDeployment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NginxDeploymentListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NginxDeploymentListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxDeploymentProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "nginxVersion", default, skip_serializing_if = "Option::is_none")]
    pub nginx_version: Option<String>,
    #[doc = "The managed resource group to deploy VNet injection related network resources."]
    #[serde(rename = "managedResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group: Option<String>,
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NginxNetworkProfile>,
    #[doc = "The IP address of the deployment."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "enableDiagnosticsSupport", default, skip_serializing_if = "Option::is_none")]
    pub enable_diagnostics_support: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<NginxLogging>,
    #[doc = "Information on how the deployment will be scaled."]
    #[serde(rename = "scalingProperties", default, skip_serializing_if = "Option::is_none")]
    pub scaling_properties: Option<NginxDeploymentScalingProperties>,
    #[doc = "Autoupgrade settings of a deployment."]
    #[serde(rename = "autoUpgradeProfile", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_profile: Option<AutoUpgradeProfile>,
    #[serde(rename = "userProfile", default, skip_serializing_if = "Option::is_none")]
    pub user_profile: Option<NginxDeploymentUserProfile>,
    #[doc = "Settings for NGINX App Protect (NAP)"]
    #[serde(rename = "nginxAppProtect", default, skip_serializing_if = "Option::is_none")]
    pub nginx_app_protect: Option<nginx_deployment_properties::NginxAppProtect>,
}
impl NginxDeploymentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nginx_deployment_properties {
    use super::*;
    #[doc = "Settings for NGINX App Protect (NAP)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct NginxAppProtect {
        #[doc = "Settings for the NGINX App Protect Web Application Firewall (WAF)"]
        #[serde(rename = "webApplicationFirewallSettings")]
        pub web_application_firewall_settings: WebApplicationFirewallSettings,
        #[doc = "The status of the NGINX App Protect Web Application Firewall"]
        #[serde(rename = "webApplicationFirewallStatus", default, skip_serializing_if = "Option::is_none")]
        pub web_application_firewall_status: Option<WebApplicationFirewallStatus>,
    }
    impl NginxAppProtect {
        pub fn new(web_application_firewall_settings: WebApplicationFirewallSettings) -> Self {
            Self {
                web_application_firewall_settings,
                web_application_firewall_status: None,
            }
        }
    }
}
#[doc = "Information on how the deployment will be scaled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxDeploymentScalingProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[doc = "The settings for enabling automatic scaling of the deployment. If this field is specified, 'scale.capacity' must be empty."]
    #[serde(rename = "autoScaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_settings: Option<nginx_deployment_scaling_properties::AutoScaleSettings>,
}
impl NginxDeploymentScalingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nginx_deployment_scaling_properties {
    use super::*;
    #[doc = "The settings for enabling automatic scaling of the deployment. If this field is specified, 'scale.capacity' must be empty."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AutoScaleSettings {
        pub profiles: Vec<ScaleProfile>,
    }
    impl AutoScaleSettings {
        pub fn new(profiles: Vec<ScaleProfile>) -> Self {
            Self { profiles }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxDeploymentUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NginxDeploymentUpdateProperties>,
}
impl NginxDeploymentUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxDeploymentUpdateProperties {
    #[serde(rename = "enableDiagnosticsSupport", default, skip_serializing_if = "Option::is_none")]
    pub enable_diagnostics_support: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<NginxLogging>,
    #[doc = "Information on how the deployment will be scaled."]
    #[serde(rename = "scalingProperties", default, skip_serializing_if = "Option::is_none")]
    pub scaling_properties: Option<NginxDeploymentScalingProperties>,
    #[serde(rename = "userProfile", default, skip_serializing_if = "Option::is_none")]
    pub user_profile: Option<NginxDeploymentUserProfile>,
    #[doc = "Autoupgrade settings of a deployment."]
    #[serde(rename = "autoUpgradeProfile", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_profile: Option<AutoUpgradeProfile>,
    #[doc = "Update settings for NGINX App Protect (NAP)"]
    #[serde(rename = "nginxAppProtect", default, skip_serializing_if = "Option::is_none")]
    pub nginx_app_protect: Option<nginx_deployment_update_properties::NginxAppProtect>,
}
impl NginxDeploymentUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nginx_deployment_update_properties {
    use super::*;
    #[doc = "Update settings for NGINX App Protect (NAP)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct NginxAppProtect {
        #[doc = "Settings for the NGINX App Protect Web Application Firewall (WAF)"]
        #[serde(rename = "webApplicationFirewallSettings", default, skip_serializing_if = "Option::is_none")]
        pub web_application_firewall_settings: Option<WebApplicationFirewallSettings>,
    }
    impl NginxAppProtect {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxDeploymentUserProfile {
    #[doc = "The preferred support contact email address of the user used for sending alerts and notification. Can be an empty string or a valid email address."]
    #[serde(rename = "preferredEmail", default, skip_serializing_if = "Option::is_none")]
    pub preferred_email: Option<String>,
}
impl NginxDeploymentUserProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxFrontendIpConfiguration {
    #[serde(
        rename = "publicIPAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub public_ip_addresses: Vec<NginxPublicIpAddress>,
    #[serde(
        rename = "privateIPAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_ip_addresses: Vec<NginxPrivateIpAddress>,
}
impl NginxFrontendIpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxLogging {
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<NginxStorageAccount>,
}
impl NginxLogging {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxNetworkInterfaceConfiguration {
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}
impl NginxNetworkInterfaceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxNetworkProfile {
    #[serde(rename = "frontEndIPConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub front_end_ip_configuration: Option<NginxFrontendIpConfiguration>,
    #[serde(rename = "networkInterfaceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_interface_configuration: Option<NginxNetworkInterfaceConfiguration>,
}
impl NginxNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxPrivateIpAddress {
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "privateIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<NginxPrivateIpAllocationMethod>,
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}
impl NginxPrivateIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NginxPrivateIpAllocationMethod")]
pub enum NginxPrivateIpAllocationMethod {
    Static,
    Dynamic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NginxPrivateIpAllocationMethod {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NginxPrivateIpAllocationMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NginxPrivateIpAllocationMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Static => serializer.serialize_unit_variant("NginxPrivateIpAllocationMethod", 0u32, "Static"),
            Self::Dynamic => serializer.serialize_unit_variant("NginxPrivateIpAllocationMethod", 1u32, "Dynamic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxPublicIpAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl NginxPublicIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NginxStorageAccount {
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}
impl NginxStorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider: Nginx.NginxPlus"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Type on which the operation is performed, e.g., 'deployments'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation type, e.g., read, write, delete, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation, e.g., 'Write deployments'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of GET request to list Nginx.NginxPlus operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Nginx.NginxPlus provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationResult>,
    #[doc = "URL to get the next set of operation list results if there are any."]
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
#[doc = "A Nginx.NginxPlus REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    Deleted,
    NotSpecified,
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
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
            Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 8u32, "NotSpecified"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderDefaultErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for ResourceProviderDefaultErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ResourceProviderDefaultErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    #[doc = "Name of the SKU."]
    pub name: String,
}
impl ResourceSku {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "The autoscale profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleProfile {
    pub name: String,
    #[doc = "The capacity parameters of the profile."]
    pub capacity: scale_profile::Capacity,
}
impl ScaleProfile {
    pub fn new(name: String, capacity: scale_profile::Capacity) -> Self {
        Self { name, capacity }
    }
}
pub mod scale_profile {
    use super::*;
    #[doc = "The capacity parameters of the profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Capacity {
        #[doc = "The minimum number of NCUs the deployment can be autoscaled to."]
        pub min: i32,
        #[doc = "The maximum number of NCUs the deployment can be autoscaled to."]
        pub max: i32,
    }
    impl Capacity {
        pub fn new(min: i32, max: i32) -> Self {
            Self { min, max }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentityProperties {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserIdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Versions of the NGINX App Protect Web Application Firewall (WAF) components."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebApplicationFirewallComponentVersions {
    #[doc = "The version of the NGINX App Protect Web Application Firewall (WAF) engine."]
    #[serde(rename = "wafEngineVersion")]
    pub waf_engine_version: String,
    #[doc = "The version of the NGINX App Protect Web Application Firewall (WAF) module for NGINX."]
    #[serde(rename = "wafNginxVersion")]
    pub waf_nginx_version: String,
}
impl WebApplicationFirewallComponentVersions {
    pub fn new(waf_engine_version: String, waf_nginx_version: String) -> Self {
        Self {
            waf_engine_version,
            waf_nginx_version,
        }
    }
}
#[doc = "NGINX App Protect Web Application Firewall (WAF) Package. Contains the version and revision date of the package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebApplicationFirewallPackage {
    #[doc = "The version of the NGINX App Protect Web Application Firewall (WAF) package."]
    pub version: String,
    #[doc = "The date and time of the package revision."]
    #[serde(rename = "revisionDatetime", with = "azure_core::date::rfc3339")]
    pub revision_datetime: ::time::OffsetDateTime,
}
impl WebApplicationFirewallPackage {
    pub fn new(version: String, revision_datetime: ::time::OffsetDateTime) -> Self {
        Self {
            version,
            revision_datetime,
        }
    }
}
#[doc = "Settings for the NGINX App Protect Web Application Firewall (WAF)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationFirewallSettings {
    #[doc = "The activation state of the WAF. Use 'Enabled' to enable the WAF and 'Disabled' to disable it."]
    #[serde(rename = "activationState", default, skip_serializing_if = "Option::is_none")]
    pub activation_state: Option<web_application_firewall_settings::ActivationState>,
}
impl WebApplicationFirewallSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod web_application_firewall_settings {
    use super::*;
    #[doc = "The activation state of the WAF. Use 'Enabled' to enable the WAF and 'Disabled' to disable it."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActivationState")]
    pub enum ActivationState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActivationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActivationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActivationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ActivationState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ActivationState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The status of the NGINX App Protect Web Application Firewall"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationFirewallStatus {
    #[doc = "NGINX App Protect Web Application Firewall (WAF) Package. Contains the version and revision date of the package."]
    #[serde(rename = "attackSignaturesPackage", default, skip_serializing_if = "Option::is_none")]
    pub attack_signatures_package: Option<WebApplicationFirewallPackage>,
    #[doc = "NGINX App Protect Web Application Firewall (WAF) Package. Contains the version and revision date of the package."]
    #[serde(rename = "botSignaturesPackage", default, skip_serializing_if = "Option::is_none")]
    pub bot_signatures_package: Option<WebApplicationFirewallPackage>,
    #[doc = "NGINX App Protect Web Application Firewall (WAF) Package. Contains the version and revision date of the package."]
    #[serde(rename = "threatCampaignsPackage", default, skip_serializing_if = "Option::is_none")]
    pub threat_campaigns_package: Option<WebApplicationFirewallPackage>,
    #[doc = "Versions of the NGINX App Protect Web Application Firewall (WAF) components."]
    #[serde(rename = "componentVersions", default, skip_serializing_if = "Option::is_none")]
    pub component_versions: Option<WebApplicationFirewallComponentVersions>,
}
impl WebApplicationFirewallStatus {
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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
