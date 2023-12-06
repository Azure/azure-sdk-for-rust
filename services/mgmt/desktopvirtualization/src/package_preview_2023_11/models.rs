#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Active directory info. Only one should be populated based on the join type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryInfoPatchProperties {
    #[doc = "Credentials kept in the keyvault."]
    #[serde(rename = "domainCredentials", default, skip_serializing_if = "Option::is_none")]
    pub domain_credentials: Option<KeyVaultCredentialsPatchProperties>,
}
impl ActiveDirectoryInfoPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active directory info. Only one should be populated based on the join type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryInfoProperties {
    #[doc = "Credentials kept in the keyvault."]
    #[serde(rename = "domainCredentials")]
    pub domain_credentials: KeyVaultCredentialsProperties,
    #[doc = "The ou path."]
    #[serde(rename = "ouPath")]
    pub ou_path: String,
    #[doc = "The domain a virtual machine connected to a hostpool will join."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}
impl ActiveDirectoryInfoProperties {
    pub fn new(domain_credentials: KeyVaultCredentialsProperties, ou_path: String) -> Self {
        Self {
            domain_credentials,
            ou_path,
            domain_name: None,
        }
    }
}
#[doc = "Represents a ActiveSessionHostConfiguration definition. This has all of the sessionHostConfiguration properties except provisioningState"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveSessionHostConfiguration {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Session host configurations of HostPool."]
    pub properties: ActiveSessionHostConfigurationProperties,
}
impl ActiveSessionHostConfiguration {
    pub fn new(properties: ActiveSessionHostConfigurationProperties) -> Self {
        Self {
            resource: Resource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "List of ActiveSessionHostConfiguration definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveSessionHostConfigurationList {
    #[doc = "List of ActiveSessionHostConfiguration definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ActiveSessionHostConfiguration>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ActiveSessionHostConfigurationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ActiveSessionHostConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Session host configurations of HostPool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveSessionHostConfigurationProperties {
    #[doc = "The timestamp of the last update."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub version: Option<time::OffsetDateTime>,
    #[doc = "Friendly name to describe this version of the SessionHostConfiguration."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Hashtable that lists key/value pair tags to apply to the VMs"]
    #[serde(rename = "vmTags", default, skip_serializing_if = "Option::is_none")]
    pub vm_tags: Option<serde_json::Value>,
    #[doc = "The Location for the session host to be created in. It will default to the location of the hostpool if not provided."]
    #[serde(rename = "vmLocation", default, skip_serializing_if = "Option::is_none")]
    pub vm_location: Option<String>,
    #[doc = "The ResourceGroup for the session hosts to be created in. It will default to the ResourceGroup of the hostpool if not provided."]
    #[serde(rename = "vmResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_group: Option<String>,
    #[doc = "The prefix that should be associated with session host names"]
    #[serde(rename = "vmNamePrefix")]
    pub vm_name_prefix: String,
    #[doc = "Value for availability zones to be used by the session host. Should be from [1,2,3]."]
    #[serde(
        rename = "availabilityZones",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availability_zones: Vec<i32>,
    #[doc = "Network information."]
    #[serde(rename = "networkInfo")]
    pub network_info: NetworkInfoProperties,
    #[doc = "The id of the size of a virtual machine connected to a hostpool."]
    #[serde(rename = "vmSizeId")]
    pub vm_size_id: String,
    #[doc = "Disk information."]
    #[serde(rename = "diskInfo")]
    pub disk_info: DiskInfoProperties,
    #[doc = "The uri to the storage blob containing the arm template to be run on the virtual machine after provisioning."]
    #[serde(rename = "customConfigurationScriptUrl", default, skip_serializing_if = "Option::is_none")]
    pub custom_configuration_script_url: Option<String>,
    #[doc = "Image configurations of session host in a HostPool."]
    #[serde(rename = "imageInfo")]
    pub image_info: ImageInfoProperties,
    #[doc = "Domain configurations of session hosts."]
    #[serde(rename = "domainInfo")]
    pub domain_info: DomainInfoProperties,
    #[doc = "Security information."]
    #[serde(rename = "securityInfo", default, skip_serializing_if = "Option::is_none")]
    pub security_info: Option<SecurityInfoProperties>,
    #[doc = "Credentials kept in the keyvault."]
    #[serde(rename = "vmAdminCredentials")]
    pub vm_admin_credentials: KeyVaultCredentialsProperties,
    #[doc = "Boot Diagnostics is a debugging feature which allows you to view Console Output and Screenshot to diagnose VM status. <br><br> You can easily view the output of your console log. <br><br> Azure also enables you to see a screenshot of the VM from the hypervisor."]
    #[serde(rename = "bootDiagnosticsInfo", default, skip_serializing_if = "Option::is_none")]
    pub boot_diagnostics_info: Option<BootDiagnosticsInfoProperties>,
}
impl ActiveSessionHostConfigurationProperties {
    pub fn new(
        vm_name_prefix: String,
        network_info: NetworkInfoProperties,
        vm_size_id: String,
        disk_info: DiskInfoProperties,
        image_info: ImageInfoProperties,
        domain_info: DomainInfoProperties,
        vm_admin_credentials: KeyVaultCredentialsProperties,
    ) -> Self {
        Self {
            version: None,
            friendly_name: None,
            vm_tags: None,
            vm_location: None,
            vm_resource_group: None,
            vm_name_prefix,
            availability_zones: Vec::new(),
            network_info,
            vm_size_id,
            disk_info,
            custom_configuration_script_url: None,
            image_info,
            domain_info,
            security_info: None,
            vm_admin_credentials,
            boot_diagnostics_info: None,
        }
    }
}
#[doc = "The session host configuration for updating agent, monitoring agent, and stack component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentUpdatePatchProperties {
    #[doc = "The type of maintenance for session host components."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<agent_update_patch_properties::Type>,
    #[doc = "Whether to use localTime of the virtual machine."]
    #[serde(rename = "useSessionHostLocalTime", default, skip_serializing_if = "Option::is_none")]
    pub use_session_host_local_time: Option<bool>,
    #[doc = "Time zone for maintenance as defined in https://docs.microsoft.com/en-us/dotnet/api/system.timezoneinfo.findsystemtimezonebyid?view=net-5.0. Must be set if useLocalTime is true."]
    #[serde(rename = "maintenanceWindowTimeZone", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_time_zone: Option<String>,
    #[doc = "List of maintenance windows. Maintenance windows are 2 hours long."]
    #[serde(
        rename = "maintenanceWindows",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub maintenance_windows: Vec<MaintenanceWindowPatchProperties>,
}
impl AgentUpdatePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agent_update_patch_properties {
    use super::*;
    #[doc = "The type of maintenance for session host components."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Default,
        Scheduled,
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
                Self::Default => serializer.serialize_unit_variant("Type", 0u32, "Default"),
                Self::Scheduled => serializer.serialize_unit_variant("Type", 1u32, "Scheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The session host configuration for updating agent, monitoring agent, and stack component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentUpdateProperties {
    #[doc = "The type of maintenance for session host components."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<agent_update_properties::Type>,
    #[doc = "Whether to use localTime of the virtual machine."]
    #[serde(rename = "useSessionHostLocalTime", default, skip_serializing_if = "Option::is_none")]
    pub use_session_host_local_time: Option<bool>,
    #[doc = "Time zone for maintenance as defined in https://docs.microsoft.com/en-us/dotnet/api/system.timezoneinfo.findsystemtimezonebyid?view=net-5.0. Must be set if useLocalTime is true."]
    #[serde(rename = "maintenanceWindowTimeZone", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_time_zone: Option<String>,
    #[doc = "List of maintenance windows. Maintenance windows are 2 hours long."]
    #[serde(
        rename = "maintenanceWindows",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub maintenance_windows: Vec<MaintenanceWindowProperties>,
}
impl AgentUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agent_update_properties {
    use super::*;
    #[doc = "The type of maintenance for session host components."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Default,
        Scheduled,
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
                Self::Default => serializer.serialize_unit_variant("Type", 0u32, "Default"),
                Self::Scheduled => serializer.serialize_unit_variant("Type", 1u32, "Scheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema for App Attach Package properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppAttachPackage {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Schema for App Attach Package properties."]
    pub properties: AppAttachPackageProperties,
}
impl AppAttachPackage {
    pub fn new(tracked_resource: TrackedResource, properties: AppAttachPackageProperties) -> Self {
        Self {
            tracked_resource,
            system_data: None,
            properties,
        }
    }
}
#[doc = "Schema for Import Package Information properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppAttachPackageInfoProperties {
    #[doc = "Alias of App Attach Package. Assigned at import time"]
    #[serde(rename = "packageAlias", default, skip_serializing_if = "Option::is_none")]
    pub package_alias: Option<String>,
    #[doc = "VHD/CIM image path on Network Share."]
    #[serde(rename = "imagePath", default, skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[doc = "Package Name from appxmanifest.xml. "]
    #[serde(rename = "packageName", default, skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[doc = "Package Family Name from appxmanifest.xml. Contains Package Name and Publisher name. "]
    #[serde(rename = "packageFamilyName", default, skip_serializing_if = "Option::is_none")]
    pub package_family_name: Option<String>,
    #[doc = "Package Full Name from appxmanifest.xml. "]
    #[serde(rename = "packageFullName", default, skip_serializing_if = "Option::is_none")]
    pub package_full_name: Option<String>,
    #[doc = "User friendly Name to be displayed in the portal. "]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Relative Path to the package inside the image. "]
    #[serde(rename = "packageRelativePath", default, skip_serializing_if = "Option::is_none")]
    pub package_relative_path: Option<String>,
    #[doc = "Specifies how to register Package in feed."]
    #[serde(rename = "isRegularRegistration", default, skip_serializing_if = "Option::is_none")]
    pub is_regular_registration: Option<bool>,
    #[doc = "Make this version of the package the active one across the hostpool. "]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "List of package dependencies. "]
    #[serde(
        rename = "packageDependencies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_dependencies: Vec<MsixPackageDependencies>,
    #[doc = "Package Version found in the appxmanifest.xml. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Date Package was last updated, found in the appxmanifest.xml. "]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "List of package applications. "]
    #[serde(
        rename = "packageApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_applications: Vec<MsixPackageApplications>,
    #[doc = "Certificate name found in the appxmanifest.xml. "]
    #[serde(rename = "certificateName", default, skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    #[doc = "Date certificate expires, found in the appxmanifest.xml. "]
    #[serde(rename = "certificateExpiry", default, with = "azure_core::date::rfc3339::option")]
    pub certificate_expiry: Option<time::OffsetDateTime>,
    #[doc = "Is package timestamped so it can ignore the certificate expiry date"]
    #[serde(rename = "isPackageTimestamped", default, skip_serializing_if = "Option::is_none")]
    pub is_package_timestamped: Option<app_attach_package_info_properties::IsPackageTimestamped>,
}
impl AppAttachPackageInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod app_attach_package_info_properties {
    use super::*;
    #[doc = "Is package timestamped so it can ignore the certificate expiry date"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsPackageTimestamped")]
    pub enum IsPackageTimestamped {
        Timestamped,
        NotTimestamped,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsPackageTimestamped {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsPackageTimestamped {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsPackageTimestamped {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Timestamped => serializer.serialize_unit_variant("IsPackageTimestamped", 0u32, "Timestamped"),
                Self::NotTimestamped => serializer.serialize_unit_variant("IsPackageTimestamped", 1u32, "NotTimestamped"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of App Attach Package definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppAttachPackageList {
    #[doc = "List of App Attach Package definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AppAttachPackage>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AppAttachPackageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AppAttachPackageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for patchable App Attach Package properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppAttachPackagePatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "tags to be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Schema for patchable fields on an App Attach Package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppAttachPackagePatchProperties>,
}
impl AppAttachPackagePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for patchable fields on an App Attach Package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppAttachPackagePatchProperties {
    #[doc = "Schema for Import Package Information properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<AppAttachPackageInfoProperties>,
    #[doc = "List of Hostpool resource Ids."]
    #[serde(
        rename = "hostPoolReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub host_pool_references: Vec<String>,
    #[doc = "URL of keyvault location to store certificate"]
    #[serde(rename = "keyVaultURL", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_url: Option<String>,
    #[doc = "Parameter indicating how the health check should behave if this package fails staging"]
    #[serde(rename = "failHealthCheckOnStagingFailure", default, skip_serializing_if = "Option::is_none")]
    pub fail_health_check_on_staging_failure: Option<FailHealthCheckOnStagingFailureEnum>,
}
impl AppAttachPackagePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for App Attach Package properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppAttachPackageProperties {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Schema for Import Package Information properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<AppAttachPackageInfoProperties>,
    #[doc = "List of Hostpool resource Ids."]
    #[serde(
        rename = "hostPoolReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub host_pool_references: Vec<String>,
    #[doc = "URL of keyvault location to store certificate"]
    #[serde(rename = "keyVaultURL", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_url: Option<String>,
    #[doc = "Parameter indicating how the health check should behave if this package fails staging"]
    #[serde(rename = "failHealthCheckOnStagingFailure", default, skip_serializing_if = "Option::is_none")]
    pub fail_health_check_on_staging_failure: Option<FailHealthCheckOnStagingFailureEnum>,
}
impl AppAttachPackageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for Application properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Schema for Application properties."]
    pub properties: ApplicationProperties,
}
impl Application {
    pub fn new(properties: ApplicationProperties) -> Self {
        Self {
            resource: Resource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "Represents a ApplicationGroup definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGroup {
    #[serde(flatten)]
    pub resource_model_with_allowed_property_set: ResourceModelWithAllowedPropertySet,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Schema for ApplicationGroup properties."]
    pub properties: ApplicationGroupProperties,
}
impl ApplicationGroup {
    pub fn new(properties: ApplicationGroupProperties) -> Self {
        Self {
            resource_model_with_allowed_property_set: ResourceModelWithAllowedPropertySet::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "List of ApplicationGroup definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGroupList {
    #[doc = "List of ApplicationGroup definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ApplicationGroup>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationGroupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApplicationGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ApplicationGroup properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGroupPatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "tags to be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "ApplicationGroup properties that can be patched."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationGroupPatchProperties>,
}
impl ApplicationGroupPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ApplicationGroup properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationGroupPatchProperties {
    #[doc = "Description of ApplicationGroup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly name of ApplicationGroup."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Boolean representing whether the applicationGroup is show in the feed."]
    #[serde(rename = "showInFeed", default, skip_serializing_if = "Option::is_none")]
    pub show_in_feed: Option<bool>,
}
impl ApplicationGroupPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for ApplicationGroup properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGroupProperties {
    #[doc = "ObjectId of ApplicationGroup. (internal use)"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Description of ApplicationGroup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly name of ApplicationGroup."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "HostPool arm path of ApplicationGroup."]
    #[serde(rename = "hostPoolArmPath")]
    pub host_pool_arm_path: String,
    #[doc = "Workspace arm path of ApplicationGroup."]
    #[serde(rename = "workspaceArmPath", default, skip_serializing_if = "Option::is_none")]
    pub workspace_arm_path: Option<String>,
    #[doc = "Resource Type of ApplicationGroup."]
    #[serde(rename = "applicationGroupType")]
    pub application_group_type: application_group_properties::ApplicationGroupType,
    #[doc = "Is cloud pc resource."]
    #[serde(rename = "cloudPcResource", default, skip_serializing_if = "Option::is_none")]
    pub cloud_pc_resource: Option<bool>,
    #[doc = "Boolean representing whether the applicationGroup is show in the feed."]
    #[serde(rename = "showInFeed", default, skip_serializing_if = "Option::is_none")]
    pub show_in_feed: Option<bool>,
}
impl ApplicationGroupProperties {
    pub fn new(host_pool_arm_path: String, application_group_type: application_group_properties::ApplicationGroupType) -> Self {
        Self {
            object_id: None,
            description: None,
            friendly_name: None,
            host_pool_arm_path,
            workspace_arm_path: None,
            application_group_type,
            cloud_pc_resource: None,
            show_in_feed: None,
        }
    }
}
pub mod application_group_properties {
    use super::*;
    #[doc = "Resource Type of ApplicationGroup."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApplicationGroupType")]
    pub enum ApplicationGroupType {
        RemoteApp,
        Desktop,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApplicationGroupType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApplicationGroupType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApplicationGroupType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RemoteApp => serializer.serialize_unit_variant("ApplicationGroupType", 0u32, "RemoteApp"),
                Self::Desktop => serializer.serialize_unit_variant("ApplicationGroupType", 1u32, "Desktop"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of Application definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationList {
    #[doc = "List of Application definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Application>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApplicationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPatch {
    #[doc = "tags to be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Application properties that can be patched."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationPatchProperties>,
}
impl ApplicationPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPatchProperties {
    #[doc = "Description of Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly name of Application."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Specifies a path for the executable file for the application."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "Specifies whether this published application can be launched with command line arguments provided by the client, command line arguments specified at publish time, or no command line arguments at all."]
    #[serde(rename = "commandLineSetting", default, skip_serializing_if = "Option::is_none")]
    pub command_line_setting: Option<application_patch_properties::CommandLineSetting>,
    #[doc = "Command Line Arguments for Application."]
    #[serde(rename = "commandLineArguments", default, skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<String>,
    #[doc = "Specifies whether to show the RemoteApp program in the RD Web Access server."]
    #[serde(rename = "showInPortal", default, skip_serializing_if = "Option::is_none")]
    pub show_in_portal: Option<bool>,
    #[doc = "Path to icon."]
    #[serde(rename = "iconPath", default, skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[doc = "Index of the icon."]
    #[serde(rename = "iconIndex", default, skip_serializing_if = "Option::is_none")]
    pub icon_index: Option<i32>,
    #[doc = "Specifies the package family name for MSIX applications"]
    #[serde(rename = "msixPackageFamilyName", default, skip_serializing_if = "Option::is_none")]
    pub msix_package_family_name: Option<String>,
    #[doc = "Specifies the package application Id for MSIX applications"]
    #[serde(rename = "msixPackageApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub msix_package_application_id: Option<String>,
    #[doc = "Resource Type of Application."]
    #[serde(rename = "applicationType", default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<application_patch_properties::ApplicationType>,
}
impl ApplicationPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod application_patch_properties {
    use super::*;
    #[doc = "Specifies whether this published application can be launched with command line arguments provided by the client, command line arguments specified at publish time, or no command line arguments at all."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CommandLineSetting")]
    pub enum CommandLineSetting {
        DoNotAllow,
        Allow,
        Require,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CommandLineSetting {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CommandLineSetting {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CommandLineSetting {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DoNotAllow => serializer.serialize_unit_variant("CommandLineSetting", 0u32, "DoNotAllow"),
                Self::Allow => serializer.serialize_unit_variant("CommandLineSetting", 1u32, "Allow"),
                Self::Require => serializer.serialize_unit_variant("CommandLineSetting", 2u32, "Require"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Resource Type of Application."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApplicationType")]
    pub enum ApplicationType {
        InBuilt,
        MsixApplication,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApplicationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApplicationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApplicationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::InBuilt => serializer.serialize_unit_variant("ApplicationType", 0u32, "InBuilt"),
                Self::MsixApplication => serializer.serialize_unit_variant("ApplicationType", 1u32, "MsixApplication"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema for Application properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[doc = "ObjectId of Application. (internal use)"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Description of Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly name of Application."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Specifies a path for the executable file for the application."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "Specifies the package family name for MSIX applications"]
    #[serde(rename = "msixPackageFamilyName", default, skip_serializing_if = "Option::is_none")]
    pub msix_package_family_name: Option<String>,
    #[doc = "Specifies the package application Id for MSIX applications"]
    #[serde(rename = "msixPackageApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub msix_package_application_id: Option<String>,
    #[doc = "Resource Type of Application."]
    #[serde(rename = "applicationType", default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<application_properties::ApplicationType>,
    #[doc = "Specifies whether this published application can be launched with command line arguments provided by the client, command line arguments specified at publish time, or no command line arguments at all."]
    #[serde(rename = "commandLineSetting")]
    pub command_line_setting: application_properties::CommandLineSetting,
    #[doc = "Command Line Arguments for Application."]
    #[serde(rename = "commandLineArguments", default, skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<String>,
    #[doc = "Specifies whether to show the RemoteApp program in the RD Web Access server."]
    #[serde(rename = "showInPortal", default, skip_serializing_if = "Option::is_none")]
    pub show_in_portal: Option<bool>,
    #[doc = "Path to icon."]
    #[serde(rename = "iconPath", default, skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[doc = "Index of the icon."]
    #[serde(rename = "iconIndex", default, skip_serializing_if = "Option::is_none")]
    pub icon_index: Option<i32>,
    #[doc = "Hash of the icon."]
    #[serde(rename = "iconHash", default, skip_serializing_if = "Option::is_none")]
    pub icon_hash: Option<String>,
    #[doc = "the icon a 64 bit string as a byte array."]
    #[serde(rename = "iconContent", default, skip_serializing_if = "Option::is_none")]
    pub icon_content: Option<String>,
}
impl ApplicationProperties {
    pub fn new(command_line_setting: application_properties::CommandLineSetting) -> Self {
        Self {
            object_id: None,
            description: None,
            friendly_name: None,
            file_path: None,
            msix_package_family_name: None,
            msix_package_application_id: None,
            application_type: None,
            command_line_setting,
            command_line_arguments: None,
            show_in_portal: None,
            icon_path: None,
            icon_index: None,
            icon_hash: None,
            icon_content: None,
        }
    }
}
pub mod application_properties {
    use super::*;
    #[doc = "Resource Type of Application."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApplicationType")]
    pub enum ApplicationType {
        InBuilt,
        MsixApplication,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApplicationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApplicationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApplicationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::InBuilt => serializer.serialize_unit_variant("ApplicationType", 0u32, "InBuilt"),
                Self::MsixApplication => serializer.serialize_unit_variant("ApplicationType", 1u32, "MsixApplication"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies whether this published application can be launched with command line arguments provided by the client, command line arguments specified at publish time, or no command line arguments at all."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CommandLineSetting")]
    pub enum CommandLineSetting {
        DoNotAllow,
        Allow,
        Require,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CommandLineSetting {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CommandLineSetting {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CommandLineSetting {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DoNotAllow => serializer.serialize_unit_variant("CommandLineSetting", 0u32, "DoNotAllow"),
                Self::Allow => serializer.serialize_unit_variant("CommandLineSetting", 1u32, "Allow"),
                Self::Require => serializer.serialize_unit_variant("CommandLineSetting", 2u32, "Require"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure Active directory info. Only one should be populated based on the join type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureActiveDirectoryInfoProperties {
    #[doc = "The mdm guid."]
    #[serde(rename = "mdmProviderGuid")]
    pub mdm_provider_guid: String,
}
impl AzureActiveDirectoryInfoProperties {
    pub fn new(mdm_provider_guid: String) -> Self {
        Self { mdm_provider_guid }
    }
}
#[doc = "Boot Diagnostics is a debugging feature which allows you to view Console Output and Screenshot to diagnose VM status. <br><br> You can easily view the output of your console log. <br><br> Azure also enables you to see a screenshot of the VM from the hypervisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BootDiagnosticsInfoPatchProperties {
    #[doc = "Whether boot diagnostics should be enabled on the Virtual Machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Uri of the storage account to use for placing the console output and screenshot. <br><br>If storageUri is not specified while enabling boot diagnostics, managed storage will be used."]
    #[serde(rename = "storageUri", default, skip_serializing_if = "Option::is_none")]
    pub storage_uri: Option<String>,
}
impl BootDiagnosticsInfoPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Boot Diagnostics is a debugging feature which allows you to view Console Output and Screenshot to diagnose VM status. <br><br> You can easily view the output of your console log. <br><br> Azure also enables you to see a screenshot of the VM from the hypervisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BootDiagnosticsInfoProperties {
    #[doc = "Whether boot diagnostics should be enabled on the Virtual Machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Uri of the storage account to use for placing the console output and screenshot. <br><br>If storageUri is not specified while enabling boot diagnostics, managed storage will be used."]
    #[serde(rename = "storageUri", default, skip_serializing_if = "Option::is_none")]
    pub storage_uri: Option<String>,
}
impl BootDiagnosticsInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cloud error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Cloud error object properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorProperties>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cloud error object properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorProperties {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CloudErrorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom image information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomInfoPatchProperties {
    #[doc = "The resource id of the custom image."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl CustomInfoPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom image information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomInfoProperties {
    #[doc = "The resource id of the custom image."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}
impl CustomInfoProperties {
    pub fn new(resource_id: String) -> Self {
        Self { resource_id }
    }
}
#[doc = "Schema for Desktop properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Desktop {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Schema for Desktop properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DesktopProperties>,
}
impl Desktop {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Desktop definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DesktopList {
    #[doc = "List of Desktop definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Desktop>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DesktopList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DesktopList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Desktop properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DesktopPatch {
    #[doc = "tags to be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Desktop properties that can be patched."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DesktopPatchProperties>,
}
impl DesktopPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Desktop properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DesktopPatchProperties {
    #[doc = "Description of Desktop."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly name of Desktop."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl DesktopPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for Desktop properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DesktopProperties {
    #[doc = "ObjectId of Desktop. (internal use)"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Description of Desktop."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly name of Desktop."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Hash of the icon."]
    #[serde(rename = "iconHash", default, skip_serializing_if = "Option::is_none")]
    pub icon_hash: Option<String>,
    #[doc = "The icon a 64 bit string as a byte array."]
    #[serde(rename = "iconContent", default, skip_serializing_if = "Option::is_none")]
    pub icon_content: Option<String>,
}
impl DesktopProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disk information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskInfoPatchProperties {
    #[doc = "The disk type used by virtual machine in hostpool session host."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<disk_info_patch_properties::Type>,
}
impl DiskInfoPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disk_info_patch_properties {
    use super::*;
    #[doc = "The disk type used by virtual machine in hostpool session host."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
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
                Self::StandardLrs => serializer.serialize_unit_variant("Type", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("Type", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("Type", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Disk information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskInfoProperties {
    #[doc = "The disk type used by virtual machine in hostpool session host."]
    #[serde(rename = "type")]
    pub type_: disk_info_properties::Type,
}
impl DiskInfoProperties {
    pub fn new(type_: disk_info_properties::Type) -> Self {
        Self { type_ }
    }
}
pub mod disk_info_properties {
    use super::*;
    #[doc = "The disk type used by virtual machine in hostpool session host."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
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
                Self::StandardLrs => serializer.serialize_unit_variant("Type", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("Type", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("Type", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Domain configurations of session hosts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainInfoPatchProperties {
    #[doc = "Active directory info. Only one should be populated based on the join type."]
    #[serde(rename = "activeDirectoryInfo", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_info: Option<ActiveDirectoryInfoPatchProperties>,
}
impl DomainInfoPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Domain configurations of session hosts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainInfoProperties {
    #[doc = "The type of domain join done by the virtual machine."]
    #[serde(rename = "joinType")]
    pub join_type: domain_info_properties::JoinType,
    #[doc = "Active directory info. Only one should be populated based on the join type."]
    #[serde(rename = "activeDirectoryInfo", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_info: Option<ActiveDirectoryInfoProperties>,
    #[doc = "Azure Active directory info. Only one should be populated based on the join type."]
    #[serde(rename = "azureActiveDirectoryInfo", default, skip_serializing_if = "Option::is_none")]
    pub azure_active_directory_info: Option<AzureActiveDirectoryInfoProperties>,
}
impl DomainInfoProperties {
    pub fn new(join_type: domain_info_properties::JoinType) -> Self {
        Self {
            join_type,
            active_directory_info: None,
            azure_active_directory_info: None,
        }
    }
}
pub mod domain_info_properties {
    use super::*;
    #[doc = "The type of domain join done by the virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JoinType")]
    pub enum JoinType {
        ActiveDirectory,
        AzureActiveDirectory,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for JoinType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for JoinType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for JoinType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ActiveDirectory => serializer.serialize_unit_variant("JoinType", 0u32, "ActiveDirectory"),
                Self::AzureActiveDirectory => serializer.serialize_unit_variant("JoinType", 1u32, "AzureActiveDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Represents the definition of contents retrieved after expanding the MSIX Image. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpandMsixImage {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Schema for Expand MSIX Image properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExpandMsixImageProperties>,
}
impl ExpandMsixImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of MSIX package properties retrieved from MSIX Image expansion."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpandMsixImageList {
    #[doc = "List of MSIX package properties from give MSIX Image."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExpandMsixImage>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExpandMsixImageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ExpandMsixImageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for Expand MSIX Image properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpandMsixImageProperties {
    #[doc = "Alias of MSIX Package."]
    #[serde(rename = "packageAlias", default, skip_serializing_if = "Option::is_none")]
    pub package_alias: Option<String>,
    #[doc = "VHD/CIM image path on Network Share."]
    #[serde(rename = "imagePath", default, skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[doc = "Package Name from appxmanifest.xml. "]
    #[serde(rename = "packageName", default, skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[doc = "Package Family Name from appxmanifest.xml. Contains Package Name and Publisher name. "]
    #[serde(rename = "packageFamilyName", default, skip_serializing_if = "Option::is_none")]
    pub package_family_name: Option<String>,
    #[doc = "Package Full Name from appxmanifest.xml. "]
    #[serde(rename = "packageFullName", default, skip_serializing_if = "Option::is_none")]
    pub package_full_name: Option<String>,
    #[doc = "User friendly Name to be displayed in the portal. "]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Relative Path to the package inside the image. "]
    #[serde(rename = "packageRelativePath", default, skip_serializing_if = "Option::is_none")]
    pub package_relative_path: Option<String>,
    #[doc = "Specifies how to register Package in feed."]
    #[serde(rename = "isRegularRegistration", default, skip_serializing_if = "Option::is_none")]
    pub is_regular_registration: Option<bool>,
    #[doc = "Make this version of the package the active one across the hostpool. "]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "List of package dependencies. "]
    #[serde(
        rename = "packageDependencies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_dependencies: Vec<MsixPackageDependencies>,
    #[doc = "Package Version found in the appxmanifest.xml. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Date Package was last updated, found in the appxmanifest.xml. "]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "List of package applications. "]
    #[serde(
        rename = "packageApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_applications: Vec<MsixPackageApplications>,
    #[doc = "Certificate name found in the appxmanifest.xml. "]
    #[serde(rename = "certificateName", default, skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    #[doc = "Date certificate expires, found in the appxmanifest.xml. "]
    #[serde(rename = "certificateExpiry", default, with = "azure_core::date::rfc3339::option")]
    pub certificate_expiry: Option<time::OffsetDateTime>,
}
impl ExpandMsixImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameter indicating how the health check should behave if this package fails staging"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FailHealthCheckOnStagingFailureEnum")]
pub enum FailHealthCheckOnStagingFailureEnum {
    Unhealthy,
    NeedsAssistance,
    DoNotFail,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FailHealthCheckOnStagingFailureEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FailHealthCheckOnStagingFailureEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FailHealthCheckOnStagingFailureEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unhealthy => serializer.serialize_unit_variant("FailHealthCheckOnStagingFailureEnum", 0u32, "Unhealthy"),
            Self::NeedsAssistance => serializer.serialize_unit_variant("FailHealthCheckOnStagingFailureEnum", 1u32, "NeedsAssistance"),
            Self::DoNotFail => serializer.serialize_unit_variant("FailHealthCheckOnStagingFailureEnum", 2u32, "DoNotFail"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a HostPool definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPool {
    #[serde(flatten)]
    pub resource_model_with_allowed_property_set: ResourceModelWithAllowedPropertySet,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of HostPool."]
    pub properties: HostPoolProperties,
}
impl HostPool {
    pub fn new(properties: HostPoolProperties) -> Self {
        Self {
            resource_model_with_allowed_property_set: ResourceModelWithAllowedPropertySet::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "Represents properties for a hostpool update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPoolControlParameter {
    #[doc = "Action types for controlling hostpool update."]
    pub action: host_pool_control_parameter::Action,
    #[doc = "The cancel message sent to the user on the session host. This is can only be specified if the action is 'Cancel'."]
    #[serde(rename = "cancelMessage", default, skip_serializing_if = "Option::is_none")]
    pub cancel_message: Option<String>,
}
impl HostPoolControlParameter {
    pub fn new(action: host_pool_control_parameter::Action) -> Self {
        Self {
            action,
            cancel_message: None,
        }
    }
}
pub mod host_pool_control_parameter {
    use super::*;
    #[doc = "Action types for controlling hostpool update."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Start,
        Pause,
        Cancel,
        Retry,
        Resume,
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
                Self::Start => serializer.serialize_unit_variant("Action", 0u32, "Start"),
                Self::Pause => serializer.serialize_unit_variant("Action", 1u32, "Pause"),
                Self::Cancel => serializer.serialize_unit_variant("Action", 2u32, "Cancel"),
                Self::Retry => serializer.serialize_unit_variant("Action", 3u32, "Retry"),
                Self::Resume => serializer.serialize_unit_variant("Action", 4u32, "Resume"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of HostPool definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostPoolList {
    #[doc = "List of HostPool definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<HostPool>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HostPoolList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HostPoolList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HostPool properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostPoolPatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "tags to be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of HostPool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HostPoolPatchProperties>,
}
impl HostPoolPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of HostPool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostPoolPatchProperties {
    #[doc = "Friendly name of HostPool."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Description of HostPool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Custom rdp property of HostPool."]
    #[serde(rename = "customRdpProperty", default, skip_serializing_if = "Option::is_none")]
    pub custom_rdp_property: Option<String>,
    #[doc = "The max session limit of HostPool."]
    #[serde(rename = "maxSessionLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_session_limit: Option<i32>,
    #[doc = "PersonalDesktopAssignment type for HostPool."]
    #[serde(rename = "personalDesktopAssignmentType", default, skip_serializing_if = "Option::is_none")]
    pub personal_desktop_assignment_type: Option<host_pool_patch_properties::PersonalDesktopAssignmentType>,
    #[doc = "The type of the load balancer."]
    #[serde(rename = "loadBalancerType", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_type: Option<host_pool_patch_properties::LoadBalancerType>,
    #[doc = "The ring number of HostPool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ring: Option<i32>,
    #[doc = "Is validation environment."]
    #[serde(rename = "validationEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub validation_environment: Option<bool>,
    #[doc = "Represents a RegistrationInfo definition."]
    #[serde(rename = "registrationInfo", default, skip_serializing_if = "Option::is_none")]
    pub registration_info: Option<RegistrationInfoPatch>,
    #[doc = "VM template for sessionhosts configuration within hostpool."]
    #[serde(rename = "vmTemplate", default, skip_serializing_if = "Option::is_none")]
    pub vm_template: Option<String>,
    #[doc = "URL to customer ADFS server for signing WVD SSO certificates."]
    #[serde(rename = "ssoadfsAuthority", default, skip_serializing_if = "Option::is_none")]
    pub ssoadfs_authority: Option<String>,
    #[doc = "ClientId for the registered Relying Party used to issue WVD SSO certificates."]
    #[serde(rename = "ssoClientId", default, skip_serializing_if = "Option::is_none")]
    pub sso_client_id: Option<String>,
    #[doc = "Path to Azure KeyVault storing the secret used for communication to ADFS."]
    #[serde(rename = "ssoClientSecretKeyVaultPath", default, skip_serializing_if = "Option::is_none")]
    pub sso_client_secret_key_vault_path: Option<String>,
    #[doc = "The type of single sign on Secret Type."]
    #[serde(rename = "ssoSecretType", default, skip_serializing_if = "Option::is_none")]
    pub sso_secret_type: Option<host_pool_patch_properties::SsoSecretType>,
    #[doc = "The type of preferred application group type, default to Desktop Application Group"]
    #[serde(rename = "preferredAppGroupType", default, skip_serializing_if = "Option::is_none")]
    pub preferred_app_group_type: Option<host_pool_patch_properties::PreferredAppGroupType>,
    #[doc = "The flag to turn on/off StartVMOnConnect feature."]
    #[serde(rename = "startVMOnConnect", default, skip_serializing_if = "Option::is_none")]
    pub start_vm_on_connect: Option<bool>,
    #[doc = "Enabled to allow this resource to be access from the public network"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<host_pool_patch_properties::PublicNetworkAccess>,
    #[doc = "The session host configuration for updating agent, monitoring agent, and stack component."]
    #[serde(rename = "agentUpdate", default, skip_serializing_if = "Option::is_none")]
    pub agent_update: Option<AgentUpdatePatchProperties>,
}
impl HostPoolPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod host_pool_patch_properties {
    use super::*;
    #[doc = "PersonalDesktopAssignment type for HostPool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PersonalDesktopAssignmentType")]
    pub enum PersonalDesktopAssignmentType {
        Automatic,
        Direct,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PersonalDesktopAssignmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PersonalDesktopAssignmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PersonalDesktopAssignmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("PersonalDesktopAssignmentType", 0u32, "Automatic"),
                Self::Direct => serializer.serialize_unit_variant("PersonalDesktopAssignmentType", 1u32, "Direct"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the load balancer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LoadBalancerType")]
    pub enum LoadBalancerType {
        BreadthFirst,
        DepthFirst,
        Persistent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LoadBalancerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LoadBalancerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LoadBalancerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BreadthFirst => serializer.serialize_unit_variant("LoadBalancerType", 0u32, "BreadthFirst"),
                Self::DepthFirst => serializer.serialize_unit_variant("LoadBalancerType", 1u32, "DepthFirst"),
                Self::Persistent => serializer.serialize_unit_variant("LoadBalancerType", 2u32, "Persistent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of single sign on Secret Type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SsoSecretType")]
    pub enum SsoSecretType {
        SharedKey,
        Certificate,
        SharedKeyInKeyVault,
        CertificateInKeyVault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SsoSecretType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SsoSecretType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SsoSecretType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SharedKey => serializer.serialize_unit_variant("SsoSecretType", 0u32, "SharedKey"),
                Self::Certificate => serializer.serialize_unit_variant("SsoSecretType", 1u32, "Certificate"),
                Self::SharedKeyInKeyVault => serializer.serialize_unit_variant("SsoSecretType", 2u32, "SharedKeyInKeyVault"),
                Self::CertificateInKeyVault => serializer.serialize_unit_variant("SsoSecretType", 3u32, "CertificateInKeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of preferred application group type, default to Desktop Application Group"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredAppGroupType")]
    pub enum PreferredAppGroupType {
        None,
        Desktop,
        RailApplications,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredAppGroupType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredAppGroupType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredAppGroupType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PreferredAppGroupType", 0u32, "None"),
                Self::Desktop => serializer.serialize_unit_variant("PreferredAppGroupType", 1u32, "Desktop"),
                Self::RailApplications => serializer.serialize_unit_variant("PreferredAppGroupType", 2u32, "RailApplications"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enabled to allow this resource to be access from the public network"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        EnabledForSessionHostsOnly,
        EnabledForClientsOnly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::EnabledForSessionHostsOnly => {
                    serializer.serialize_unit_variant("PublicNetworkAccess", 2u32, "EnabledForSessionHostsOnly")
                }
                Self::EnabledForClientsOnly => serializer.serialize_unit_variant("PublicNetworkAccess", 3u32, "EnabledForClientsOnly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of HostPool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPoolProperties {
    #[doc = "ObjectId of HostPool. (internal use)"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Friendly name of HostPool."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Description of HostPool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "HostPool type for desktop."]
    #[serde(rename = "hostPoolType")]
    pub host_pool_type: host_pool_properties::HostPoolType,
    #[doc = "PersonalDesktopAssignment type for HostPool."]
    #[serde(rename = "personalDesktopAssignmentType", default, skip_serializing_if = "Option::is_none")]
    pub personal_desktop_assignment_type: Option<host_pool_properties::PersonalDesktopAssignmentType>,
    #[doc = "Custom rdp property of HostPool."]
    #[serde(rename = "customRdpProperty", default, skip_serializing_if = "Option::is_none")]
    pub custom_rdp_property: Option<String>,
    #[doc = "The max session limit of HostPool."]
    #[serde(rename = "maxSessionLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_session_limit: Option<i32>,
    #[doc = "The type of the load balancer."]
    #[serde(rename = "loadBalancerType")]
    pub load_balancer_type: host_pool_properties::LoadBalancerType,
    #[doc = "The ring number of HostPool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ring: Option<i32>,
    #[doc = "Is validation environment."]
    #[serde(rename = "validationEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub validation_environment: Option<bool>,
    #[doc = "Represents a RegistrationInfo definition."]
    #[serde(rename = "registrationInfo", default, skip_serializing_if = "Option::is_none")]
    pub registration_info: Option<RegistrationInfo>,
    #[doc = "VM template for sessionhosts configuration within hostpool."]
    #[serde(rename = "vmTemplate", default, skip_serializing_if = "Option::is_none")]
    pub vm_template: Option<String>,
    #[doc = "The type of management for this hostpool, Automated or Standard. The default value is Automated."]
    #[serde(rename = "managementType", default, skip_serializing_if = "Option::is_none")]
    pub management_type: Option<host_pool_properties::ManagementType>,
    #[doc = "List of applicationGroup links."]
    #[serde(
        rename = "applicationGroupReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub application_group_references: Vec<String>,
    #[doc = "List of App Attach Package links."]
    #[serde(
        rename = "appAttachPackageReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub app_attach_package_references: Vec<String>,
    #[doc = "URL to customer ADFS server for signing WVD SSO certificates."]
    #[serde(rename = "ssoadfsAuthority", default, skip_serializing_if = "Option::is_none")]
    pub ssoadfs_authority: Option<String>,
    #[doc = "ClientId for the registered Relying Party used to issue WVD SSO certificates."]
    #[serde(rename = "ssoClientId", default, skip_serializing_if = "Option::is_none")]
    pub sso_client_id: Option<String>,
    #[doc = "Path to Azure KeyVault storing the secret used for communication to ADFS."]
    #[serde(rename = "ssoClientSecretKeyVaultPath", default, skip_serializing_if = "Option::is_none")]
    pub sso_client_secret_key_vault_path: Option<String>,
    #[doc = "The type of single sign on Secret Type."]
    #[serde(rename = "ssoSecretType", default, skip_serializing_if = "Option::is_none")]
    pub sso_secret_type: Option<host_pool_properties::SsoSecretType>,
    #[doc = "The type of preferred application group type, default to Desktop Application Group"]
    #[serde(rename = "preferredAppGroupType")]
    pub preferred_app_group_type: host_pool_properties::PreferredAppGroupType,
    #[doc = "The flag to turn on/off StartVMOnConnect feature."]
    #[serde(rename = "startVMOnConnect", default, skip_serializing_if = "Option::is_none")]
    pub start_vm_on_connect: Option<bool>,
    #[doc = "Is cloud pc resource."]
    #[serde(rename = "cloudPcResource", default, skip_serializing_if = "Option::is_none")]
    pub cloud_pc_resource: Option<bool>,
    #[doc = "Enabled allows this resource to be accessed from both public and private networks, Disabled allows this resource to only be accessed via private endpoints"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<host_pool_properties::PublicNetworkAccess>,
    #[doc = "The session host configuration for updating agent, monitoring agent, and stack component."]
    #[serde(rename = "agentUpdate", default, skip_serializing_if = "Option::is_none")]
    pub agent_update: Option<AgentUpdateProperties>,
    #[doc = "List of private endpoint connection associated with the specified resource"]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl HostPoolProperties {
    pub fn new(
        host_pool_type: host_pool_properties::HostPoolType,
        load_balancer_type: host_pool_properties::LoadBalancerType,
        preferred_app_group_type: host_pool_properties::PreferredAppGroupType,
    ) -> Self {
        Self {
            object_id: None,
            friendly_name: None,
            description: None,
            host_pool_type,
            personal_desktop_assignment_type: None,
            custom_rdp_property: None,
            max_session_limit: None,
            load_balancer_type,
            ring: None,
            validation_environment: None,
            registration_info: None,
            vm_template: None,
            management_type: None,
            application_group_references: Vec::new(),
            app_attach_package_references: Vec::new(),
            ssoadfs_authority: None,
            sso_client_id: None,
            sso_client_secret_key_vault_path: None,
            sso_secret_type: None,
            preferred_app_group_type,
            start_vm_on_connect: None,
            cloud_pc_resource: None,
            public_network_access: None,
            agent_update: None,
            private_endpoint_connections: Vec::new(),
        }
    }
}
pub mod host_pool_properties {
    use super::*;
    #[doc = "HostPool type for desktop."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostPoolType")]
    pub enum HostPoolType {
        Personal,
        Pooled,
        #[serde(rename = "BYODesktop")]
        ByoDesktop,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostPoolType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostPoolType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostPoolType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Personal => serializer.serialize_unit_variant("HostPoolType", 0u32, "Personal"),
                Self::Pooled => serializer.serialize_unit_variant("HostPoolType", 1u32, "Pooled"),
                Self::ByoDesktop => serializer.serialize_unit_variant("HostPoolType", 2u32, "BYODesktop"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "PersonalDesktopAssignment type for HostPool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PersonalDesktopAssignmentType")]
    pub enum PersonalDesktopAssignmentType {
        Automatic,
        Direct,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PersonalDesktopAssignmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PersonalDesktopAssignmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PersonalDesktopAssignmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("PersonalDesktopAssignmentType", 0u32, "Automatic"),
                Self::Direct => serializer.serialize_unit_variant("PersonalDesktopAssignmentType", 1u32, "Direct"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the load balancer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LoadBalancerType")]
    pub enum LoadBalancerType {
        BreadthFirst,
        DepthFirst,
        Persistent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LoadBalancerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LoadBalancerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LoadBalancerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BreadthFirst => serializer.serialize_unit_variant("LoadBalancerType", 0u32, "BreadthFirst"),
                Self::DepthFirst => serializer.serialize_unit_variant("LoadBalancerType", 1u32, "DepthFirst"),
                Self::Persistent => serializer.serialize_unit_variant("LoadBalancerType", 2u32, "Persistent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of management for this hostpool, Automated or Standard. The default value is Automated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ManagementType")]
    pub enum ManagementType {
        Automated,
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ManagementType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ManagementType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ManagementType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automated => serializer.serialize_unit_variant("ManagementType", 0u32, "Automated"),
                Self::Standard => serializer.serialize_unit_variant("ManagementType", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of single sign on Secret Type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SsoSecretType")]
    pub enum SsoSecretType {
        SharedKey,
        Certificate,
        SharedKeyInKeyVault,
        CertificateInKeyVault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SsoSecretType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SsoSecretType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SsoSecretType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SharedKey => serializer.serialize_unit_variant("SsoSecretType", 0u32, "SharedKey"),
                Self::Certificate => serializer.serialize_unit_variant("SsoSecretType", 1u32, "Certificate"),
                Self::SharedKeyInKeyVault => serializer.serialize_unit_variant("SsoSecretType", 2u32, "SharedKeyInKeyVault"),
                Self::CertificateInKeyVault => serializer.serialize_unit_variant("SsoSecretType", 3u32, "CertificateInKeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of preferred application group type, default to Desktop Application Group"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredAppGroupType")]
    pub enum PreferredAppGroupType {
        None,
        Desktop,
        RailApplications,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredAppGroupType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredAppGroupType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredAppGroupType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PreferredAppGroupType", 0u32, "None"),
                Self::Desktop => serializer.serialize_unit_variant("PreferredAppGroupType", 1u32, "Desktop"),
                Self::RailApplications => serializer.serialize_unit_variant("PreferredAppGroupType", 2u32, "RailApplications"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enabled allows this resource to be accessed from both public and private networks, Disabled allows this resource to only be accessed via private endpoints"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        EnabledForSessionHostsOnly,
        EnabledForClientsOnly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::EnabledForSessionHostsOnly => {
                    serializer.serialize_unit_variant("PublicNetworkAccess", 2u32, "EnabledForSessionHostsOnly")
                }
                Self::EnabledForClientsOnly => serializer.serialize_unit_variant("PublicNetworkAccess", 3u32, "EnabledForClientsOnly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The configurations of a hostpool update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostPoolUpdateConfigurationPatchProperties {
    #[doc = "Whether not to save original disk. False by default."]
    #[serde(rename = "deleteOriginalVm", default, skip_serializing_if = "Option::is_none")]
    pub delete_original_vm: Option<bool>,
    #[doc = "The maximum number of virtual machines to be removed during hostpool update."]
    #[serde(rename = "maxVmsRemoved", default, skip_serializing_if = "Option::is_none")]
    pub max_vms_removed: Option<i32>,
    #[doc = "Grace period before logging off users in minutes."]
    #[serde(rename = "logOffDelayMinutes", default, skip_serializing_if = "Option::is_none")]
    pub log_off_delay_minutes: Option<i32>,
    #[doc = "Log off message sent to user for logoff."]
    #[serde(rename = "logOffMessage", default, skip_serializing_if = "Option::is_none")]
    pub log_off_message: Option<String>,
}
impl HostPoolUpdateConfigurationPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configurations of a hostpool update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostPoolUpdateConfigurationProperties {
    #[doc = "Whether not to save original disk. False by default."]
    #[serde(rename = "deleteOriginalVm", default, skip_serializing_if = "Option::is_none")]
    pub delete_original_vm: Option<bool>,
    #[doc = "The maximum number of virtual machines to be removed during hostpool update."]
    #[serde(rename = "maxVmsRemoved")]
    pub max_vms_removed: i32,
    #[doc = "Grace period before logging off users in minutes."]
    #[serde(rename = "logOffDelayMinutes")]
    pub log_off_delay_minutes: i32,
    #[doc = "Log off message sent to user for logoff. Default value is an empty string."]
    #[serde(rename = "logOffMessage", default, skip_serializing_if = "Option::is_none")]
    pub log_off_message: Option<String>,
}
impl HostPoolUpdateConfigurationProperties {
    pub fn new(max_vms_removed: i32, log_off_delay_minutes: i32) -> Self {
        Self {
            delete_original_vm: None,
            max_vms_removed,
            log_off_delay_minutes,
            log_off_message: None,
        }
    }
}
#[doc = "Hostpool update fault information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostPoolUpdateFault {
    #[doc = "Hostpool update fault type."]
    #[serde(rename = "faultType", default, skip_serializing_if = "Option::is_none")]
    pub fault_type: Option<host_pool_update_fault::FaultType>,
    #[doc = "Hostpool update fault code."]
    #[serde(rename = "faultCode", default, skip_serializing_if = "Option::is_none")]
    pub fault_code: Option<String>,
    #[doc = "Hostpool update fault text."]
    #[serde(rename = "faultText", default, skip_serializing_if = "Option::is_none")]
    pub fault_text: Option<String>,
    #[doc = "Hostpool update fault context."]
    #[serde(rename = "faultContext", default, skip_serializing_if = "Option::is_none")]
    pub fault_context: Option<String>,
}
impl HostPoolUpdateFault {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod host_pool_update_fault {
    use super::*;
    #[doc = "Hostpool update fault type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FaultType")]
    pub enum FaultType {
        ServiceError,
        UserError,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FaultType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FaultType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FaultType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ServiceError => serializer.serialize_unit_variant("FaultType", 0u32, "ServiceError"),
                Self::UserError => serializer.serialize_unit_variant("FaultType", 1u32, "UserError"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    }
}
#[doc = "Image configurations of session host in a HostPool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageInfoPatchProperties {
    #[doc = "The type of image session hosts use in the hostpool."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<image_info_patch_properties::Type>,
    #[doc = "Marketplace image information."]
    #[serde(rename = "marketplaceInfo", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_info: Option<MarketplaceInfoPatchProperties>,
    #[doc = "Custom image information."]
    #[serde(rename = "customInfo", default, skip_serializing_if = "Option::is_none")]
    pub custom_info: Option<CustomInfoPatchProperties>,
}
impl ImageInfoPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod image_info_patch_properties {
    use super::*;
    #[doc = "The type of image session hosts use in the hostpool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Marketplace,
        Custom,
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
                Self::Marketplace => serializer.serialize_unit_variant("Type", 0u32, "Marketplace"),
                Self::Custom => serializer.serialize_unit_variant("Type", 1u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Image configurations of session host in a HostPool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInfoProperties {
    #[doc = "The type of image session hosts use in the hostpool."]
    #[serde(rename = "type")]
    pub type_: image_info_properties::Type,
    #[doc = "Marketplace image information."]
    #[serde(rename = "marketplaceInfo", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_info: Option<MarketplaceInfoProperties>,
    #[doc = "Custom image information."]
    #[serde(rename = "customInfo", default, skip_serializing_if = "Option::is_none")]
    pub custom_info: Option<CustomInfoProperties>,
}
impl ImageInfoProperties {
    pub fn new(type_: image_info_properties::Type) -> Self {
        Self {
            type_,
            marketplace_info: None,
            custom_info: None,
        }
    }
}
pub mod image_info_properties {
    use super::*;
    #[doc = "The type of image session hosts use in the hostpool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Marketplace,
        Custom,
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
                Self::Marketplace => serializer.serialize_unit_variant("Type", 0u32, "Marketplace"),
                Self::Custom => serializer.serialize_unit_variant("Type", 1u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information to import app attach package"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportPackageInfoRequest {
    #[doc = "URI to Image"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Possible device architectures that an app attach package can be configured for"]
    #[serde(rename = "packageArchitecture", default, skip_serializing_if = "Option::is_none")]
    pub package_architecture: Option<import_package_info_request::PackageArchitecture>,
}
impl ImportPackageInfoRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod import_package_info_request {
    use super::*;
    #[doc = "Possible device architectures that an app attach package can be configured for"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PackageArchitecture")]
    pub enum PackageArchitecture {
        #[serde(rename = "ARM")]
        Arm,
        #[serde(rename = "ARM64")]
        Arm64,
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "x64")]
        X64,
        Neutral,
        #[serde(rename = "x86a64")]
        X86a64,
        #[serde(rename = "ALL")]
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PackageArchitecture {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PackageArchitecture {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PackageArchitecture {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Arm => serializer.serialize_unit_variant("PackageArchitecture", 0u32, "ARM"),
                Self::Arm64 => serializer.serialize_unit_variant("PackageArchitecture", 1u32, "ARM64"),
                Self::X86 => serializer.serialize_unit_variant("PackageArchitecture", 2u32, "x86"),
                Self::X64 => serializer.serialize_unit_variant("PackageArchitecture", 3u32, "x64"),
                Self::Neutral => serializer.serialize_unit_variant("PackageArchitecture", 4u32, "Neutral"),
                Self::X86a64 => serializer.serialize_unit_variant("PackageArchitecture", 5u32, "x86a64"),
                Self::All => serializer.serialize_unit_variant("PackageArchitecture", 6u32, "ALL"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Credentials kept in the keyvault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultCredentialsPatchProperties {
    #[doc = "The uri to access the secret that the username is stored in."]
    #[serde(rename = "usernameKeyVaultSecretUri", default, skip_serializing_if = "Option::is_none")]
    pub username_key_vault_secret_uri: Option<String>,
    #[doc = "The uri to access the secret that the password is stored in."]
    #[serde(rename = "passwordKeyVaultSecretUri", default, skip_serializing_if = "Option::is_none")]
    pub password_key_vault_secret_uri: Option<String>,
}
impl KeyVaultCredentialsPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Credentials kept in the keyvault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultCredentialsProperties {
    #[doc = "The uri to access the secret that the username is stored in."]
    #[serde(rename = "usernameKeyVaultSecretUri")]
    pub username_key_vault_secret_uri: String,
    #[doc = "The uri to access the secret that the password is stored in."]
    #[serde(rename = "passwordKeyVaultSecretUri")]
    pub password_key_vault_secret_uri: String,
}
impl KeyVaultCredentialsProperties {
    pub fn new(username_key_vault_secret_uri: String, password_key_vault_secret_uri: String) -> Self {
        Self {
            username_key_vault_secret_uri,
            password_key_vault_secret_uri,
        }
    }
}
#[doc = "Specifications of the Log for Azure Monitoring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of the log"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the log"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blob duration of the log"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents URI referring to MSIX Image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsixImageUri {
    #[doc = "URI to Image"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl MsixImageUri {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for MSIX Package properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsixPackage {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Schema for MSIX Package properties."]
    pub properties: MsixPackageProperties,
}
impl MsixPackage {
    pub fn new(properties: MsixPackageProperties) -> Self {
        Self {
            resource: Resource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "List of MSIX Package definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsixPackageList {
    #[doc = "List of MSIX Package definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<MsixPackage>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MsixPackageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MsixPackageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MSIX Package properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsixPackagePatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "MSIX Package properties that can be patched."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MsixPackagePatchProperties>,
}
impl MsixPackagePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MSIX Package properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsixPackagePatchProperties {
    #[doc = "Set a version of the package to be active across hostpool. "]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "Set Registration mode. Regular or Delayed."]
    #[serde(rename = "isRegularRegistration", default, skip_serializing_if = "Option::is_none")]
    pub is_regular_registration: Option<bool>,
    #[doc = "Display name for MSIX Package."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl MsixPackagePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for MSIX Package properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsixPackageProperties {
    #[doc = "VHD/CIM image path on Network Share."]
    #[serde(rename = "imagePath", default, skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[doc = "Package Name from appxmanifest.xml. "]
    #[serde(rename = "packageName", default, skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[doc = "Package Family Name from appxmanifest.xml. Contains Package Name and Publisher name. "]
    #[serde(rename = "packageFamilyName", default, skip_serializing_if = "Option::is_none")]
    pub package_family_name: Option<String>,
    #[doc = "User friendly Name to be displayed in the portal. "]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Relative Path to the package inside the image. "]
    #[serde(rename = "packageRelativePath", default, skip_serializing_if = "Option::is_none")]
    pub package_relative_path: Option<String>,
    #[doc = "Specifies how to register Package in feed."]
    #[serde(rename = "isRegularRegistration", default, skip_serializing_if = "Option::is_none")]
    pub is_regular_registration: Option<bool>,
    #[doc = "Make this version of the package the active one across the hostpool. "]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "List of package dependencies. "]
    #[serde(
        rename = "packageDependencies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_dependencies: Vec<MsixPackageDependencies>,
    #[doc = "Package Version found in the appxmanifest.xml. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Date Package was last updated, found in the appxmanifest.xml. "]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "List of package applications. "]
    #[serde(
        rename = "packageApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_applications: Vec<MsixPackageApplications>,
}
impl MsixPackageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance window starting hour and day of week."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindowPatchProperties {
    #[doc = "The update start hour of the day. (0 - 23)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[doc = "Day of the week."]
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<maintenance_window_patch_properties::DayOfWeek>,
}
impl MaintenanceWindowPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod maintenance_window_patch_properties {
    use super::*;
    #[doc = "Day of the week."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DayOfWeek {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
}
#[doc = "Maintenance window starting hour and day of week."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindowProperties {
    #[doc = "The update start hour of the day. (0 - 23)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[doc = "Day of the week."]
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<maintenance_window_properties::DayOfWeek>,
}
impl MaintenanceWindowProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod maintenance_window_properties {
    use super::*;
    #[doc = "Day of the week."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DayOfWeek {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
}
#[doc = "Marketplace image information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceInfoPatchProperties {
    #[doc = "The offer of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The publisher of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The sku of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The exact version of the image."]
    #[serde(rename = "exactVersion", default, skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
}
impl MarketplaceInfoPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Marketplace image information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceInfoProperties {
    #[doc = "The offer of the image."]
    pub offer: String,
    #[doc = "The publisher of the image."]
    pub publisher: String,
    #[doc = "The sku of the image."]
    pub sku: String,
    #[doc = "The exact version of the image."]
    #[serde(rename = "exactVersion")]
    pub exact_version: String,
}
impl MarketplaceInfoProperties {
    pub fn new(offer: String, publisher: String, sku: String, exact_version: String) -> Self {
        Self {
            offer,
            publisher,
            sku,
            exact_version,
        }
    }
}
#[doc = "Schema for MSIX Package Application properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsixPackageApplications {
    #[doc = "Package Application Id, found in appxmanifest.xml."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "Description of Package Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Used to activate Package Application. Consists of Package Name and ApplicationID. Found in appxmanifest.xml."]
    #[serde(rename = "appUserModelID", default, skip_serializing_if = "Option::is_none")]
    pub app_user_model_id: Option<String>,
    #[doc = "User friendly name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "User friendly name."]
    #[serde(rename = "iconImageName", default, skip_serializing_if = "Option::is_none")]
    pub icon_image_name: Option<String>,
    #[doc = "the icon a 64 bit string as a byte array."]
    #[serde(rename = "rawIcon", default, skip_serializing_if = "Option::is_none")]
    pub raw_icon: Option<String>,
    #[doc = "the icon a 64 bit string as a byte array."]
    #[serde(rename = "rawPng", default, skip_serializing_if = "Option::is_none")]
    pub raw_png: Option<String>,
}
impl MsixPackageApplications {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for MSIX Package Dependencies properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsixPackageDependencies {
    #[doc = "Name of package dependency."]
    #[serde(rename = "dependencyName", default, skip_serializing_if = "Option::is_none")]
    pub dependency_name: Option<String>,
    #[doc = "Name of dependency publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Dependency version required."]
    #[serde(rename = "minVersion", default, skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,
}
impl MsixPackageDependencies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInfoPatchProperties {
    #[doc = "The resource ID of the subnet."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "The resource ID of the security group. Any allowable/open ports should be specified in the NSG."]
    #[serde(rename = "securityGroupId", default, skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
}
impl NetworkInfoPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInfoProperties {
    #[doc = "The resource ID of the subnet."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "The resource ID of the security group. Any allowable/open ports should be specified in the NSG."]
    #[serde(rename = "securityGroupId", default, skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
}
impl NetworkInfoProperties {
    pub fn new(subnet_id: String) -> Self {
        Self {
            subnet_id,
            security_group_id: None,
        }
    }
}
#[doc = "Properties of the operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "Service specification payload"]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Plan for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[doc = "A user defined name of the 3rd Party Artifact that is being procured."]
    pub name: String,
    #[doc = "The publisher of the 3rd Party Artifact that is being bought. E.g. NewRelic"]
    pub publisher: String,
    #[doc = "The 3rd Party artifact that is being procured. E.g. NewRelic. Product maps to the OfferID specified for the artifact at the time of Data Market onboarding. "]
    pub product: String,
    #[doc = "A publisher provided promotion code as provisioned in Data Market for the said product/artifact."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "The version of the desired product/artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl Plan {
    pub fn new(name: String, publisher: String, product: String) -> Self {
        Self {
            name,
            publisher,
            product,
            promotion_code: None,
            version: None,
        }
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
#[doc = "List of private endpoint connection associated with the specified storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResultWithSystemData {
    #[doc = "Array of private endpoint connections"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateEndpointConnectionWithSystemData>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResultWithSystemData {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateEndpointConnectionListResultWithSystemData {
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
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionWithSystemData {
    #[serde(flatten)]
    pub private_endpoint_connection: PrivateEndpointConnection,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateEndpointConnectionWithSystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
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
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Provisioning,
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
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
            Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Provisioning"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Represents a RegistrationInfo definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationInfo {
    #[doc = "Expiration time of registration token."]
    #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<time::OffsetDateTime>,
    #[doc = "The registration token base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "The type of resetting the token."]
    #[serde(rename = "registrationTokenOperation", default, skip_serializing_if = "Option::is_none")]
    pub registration_token_operation: Option<registration_info::RegistrationTokenOperation>,
}
impl RegistrationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod registration_info {
    use super::*;
    #[doc = "The type of resetting the token."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RegistrationTokenOperation")]
    pub enum RegistrationTokenOperation {
        Delete,
        None,
        Update,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RegistrationTokenOperation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RegistrationTokenOperation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RegistrationTokenOperation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Delete => serializer.serialize_unit_variant("RegistrationTokenOperation", 0u32, "Delete"),
                Self::None => serializer.serialize_unit_variant("RegistrationTokenOperation", 1u32, "None"),
                Self::Update => serializer.serialize_unit_variant("RegistrationTokenOperation", 2u32, "Update"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a RegistrationInfo definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationInfoPatch {
    #[doc = "Expiration time of registration token."]
    #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<time::OffsetDateTime>,
    #[doc = "The type of resetting the token."]
    #[serde(rename = "registrationTokenOperation", default, skip_serializing_if = "Option::is_none")]
    pub registration_token_operation: Option<registration_info_patch::RegistrationTokenOperation>,
}
impl RegistrationInfoPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod registration_info_patch {
    use super::*;
    #[doc = "The type of resetting the token."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RegistrationTokenOperation")]
    pub enum RegistrationTokenOperation {
        Delete,
        None,
        Update,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RegistrationTokenOperation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RegistrationTokenOperation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RegistrationTokenOperation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Delete => serializer.serialize_unit_variant("RegistrationTokenOperation", 0u32, "Delete"),
                Self::None => serializer.serialize_unit_variant("RegistrationTokenOperation", 1u32, "None"),
                Self::Update => serializer.serialize_unit_variant("RegistrationTokenOperation", 2u32, "Update"),
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition containing the full set of allowed properties for a resource. Except properties bag, there cannot be a top level property outside of this set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceModelWithAllowedPropertySet {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The fully qualified resource ID of the resource that manages this resource. Indicates if this resource is managed by another Azure resource. If this is present, complete mode deployment will not delete the resource if it is removed from the template since it is managed by another resource."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The etag field is *not* required. If it is provided in the response body, it must also be provided as a header per the normal etag convention.  Entity tags are used for comparing two or more entities from the same requested resource. HTTP/1.1 uses entity tags in the etag (section 14.19), If-Match (section 14.24), If-None-Match (section 14.26), and If-Range (section 14.27) header fields. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<serde_json::Value>,
}
impl ResourceModelWithAllowedPropertySet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported operation of this resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[doc = "Operation name, in format of {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
    #[doc = "Is a data action."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Properties of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl ResourceProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Resource provider: Microsoft Desktop Virtualization."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of this operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationList {
    #[doc = "List of operations supported by this resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ResourceProviderOperation>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceProviderOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scaling plan reference to hostpool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingHostPoolReference {
    #[doc = "Arm path of referenced hostpool."]
    #[serde(rename = "hostPoolArmPath", default, skip_serializing_if = "Option::is_none")]
    pub host_pool_arm_path: Option<String>,
    #[doc = "Is the scaling plan enabled for this hostpool."]
    #[serde(rename = "scalingPlanEnabled", default, skip_serializing_if = "Option::is_none")]
    pub scaling_plan_enabled: Option<bool>,
}
impl ScalingHostPoolReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a scaling plan definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalingPlan {
    #[serde(flatten)]
    pub resource_model_with_allowed_property_set: ResourceModelWithAllowedPropertySet,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Scaling plan properties."]
    pub properties: ScalingPlanProperties,
}
impl ScalingPlan {
    pub fn new(properties: ScalingPlanProperties) -> Self {
        Self {
            resource_model_with_allowed_property_set: ResourceModelWithAllowedPropertySet::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "List of scaling plan definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingPlanList {
    #[doc = "List of scaling plan definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ScalingPlan>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScalingPlanList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScalingPlanList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scaling plan properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingPlanPatch {
    #[doc = "tags to be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Scaling plan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScalingPlanPatchProperties>,
}
impl ScalingPlanPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scaling plan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingPlanPatchProperties {
    #[doc = "Description of scaling plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "User friendly name of scaling plan."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Timezone of the scaling plan."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "Exclusion tag for scaling plan."]
    #[serde(rename = "exclusionTag", default, skip_serializing_if = "Option::is_none")]
    pub exclusion_tag: Option<String>,
    #[doc = "List of ScalingSchedule definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub schedules: Vec<ScalingSchedule>,
    #[doc = "List of ScalingHostPoolReference definitions."]
    #[serde(
        rename = "hostPoolReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub host_pool_references: Vec<ScalingHostPoolReference>,
}
impl ScalingPlanPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a ScalingPlanPersonalSchedule definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalingPlanPersonalSchedule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "A ScalingPlanPersonalSchedule."]
    pub properties: ScalingPlanPersonalScheduleProperties,
}
impl ScalingPlanPersonalSchedule {
    pub fn new(properties: ScalingPlanPersonalScheduleProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "List of ScalingPlanPersonalSchedule definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingPlanPersonalScheduleList {
    #[doc = "List of ScalingPlanPersonalSchedule definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ScalingPlanPersonalSchedule>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScalingPlanPersonalScheduleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScalingPlanPersonalScheduleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ScalingPlanPersonalSchedule properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingPlanPersonalSchedulePatch {
    #[doc = "A ScalingPlanPersonalSchedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScalingPlanPersonalScheduleProperties>,
}
impl ScalingPlanPersonalSchedulePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A ScalingPlanPersonalSchedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingPlanPersonalScheduleProperties {
    #[doc = "Set of days of the week on which this schedule is active."]
    #[serde(
        rename = "daysOfWeek",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub days_of_week: Vec<String>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "rampUpStartTime", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_start_time: Option<Time>,
    #[doc = "The desired startup behavior during the ramp up period for personal vms in the hostpool."]
    #[serde(rename = "rampUpAutoStartHosts", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_auto_start_hosts: Option<scaling_plan_personal_schedule_properties::RampUpAutoStartHosts>,
    #[doc = "The desired configuration of Start VM On Connect for the hostpool during the ramp up phase. If this is disabled, session hosts must be turned on using rampUpAutoStartHosts or by turning them on manually."]
    #[serde(rename = "rampUpStartVMOnConnect", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_start_vm_on_connect: Option<scaling_plan_personal_schedule_properties::RampUpStartVmOnConnect>,
    #[doc = "Action to be taken after a user disconnect during the ramp up period."]
    #[serde(rename = "rampUpActionOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_action_on_disconnect: Option<scaling_plan_personal_schedule_properties::RampUpActionOnDisconnect>,
    #[doc = "The time in minutes to wait before performing the desired session handling action when a user disconnects during the ramp up period."]
    #[serde(rename = "rampUpMinutesToWaitOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_minutes_to_wait_on_disconnect: Option<i32>,
    #[doc = "Action to be taken after a logoff during the ramp up period."]
    #[serde(rename = "rampUpActionOnLogoff", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_action_on_logoff: Option<scaling_plan_personal_schedule_properties::RampUpActionOnLogoff>,
    #[doc = "The time in minutes to wait before performing the desired session handling action when a user logs off during the ramp up period."]
    #[serde(rename = "rampUpMinutesToWaitOnLogoff", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_minutes_to_wait_on_logoff: Option<i32>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "peakStartTime", default, skip_serializing_if = "Option::is_none")]
    pub peak_start_time: Option<Time>,
    #[doc = "The desired configuration of Start VM On Connect for the hostpool during the peak phase."]
    #[serde(rename = "peakStartVMOnConnect", default, skip_serializing_if = "Option::is_none")]
    pub peak_start_vm_on_connect: Option<scaling_plan_personal_schedule_properties::PeakStartVmOnConnect>,
    #[doc = "Action to be taken after a user disconnect during the peak period."]
    #[serde(rename = "peakActionOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub peak_action_on_disconnect: Option<scaling_plan_personal_schedule_properties::PeakActionOnDisconnect>,
    #[doc = "The time in minutes to wait before performing the desired session handling action when a user disconnects during the peak period."]
    #[serde(rename = "peakMinutesToWaitOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub peak_minutes_to_wait_on_disconnect: Option<i32>,
    #[doc = "Action to be taken after a logoff during the peak period."]
    #[serde(rename = "peakActionOnLogoff", default, skip_serializing_if = "Option::is_none")]
    pub peak_action_on_logoff: Option<scaling_plan_personal_schedule_properties::PeakActionOnLogoff>,
    #[doc = "The time in minutes to wait before performing the desired session handling action when a user logs off during the peak period."]
    #[serde(rename = "peakMinutesToWaitOnLogoff", default, skip_serializing_if = "Option::is_none")]
    pub peak_minutes_to_wait_on_logoff: Option<i32>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "rampDownStartTime", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_start_time: Option<Time>,
    #[doc = "The desired configuration of Start VM On Connect for the hostpool during the ramp down phase."]
    #[serde(rename = "rampDownStartVMOnConnect", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_start_vm_on_connect: Option<scaling_plan_personal_schedule_properties::RampDownStartVmOnConnect>,
    #[doc = "Action to be taken after a user disconnect during the ramp down period."]
    #[serde(rename = "rampDownActionOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_action_on_disconnect: Option<scaling_plan_personal_schedule_properties::RampDownActionOnDisconnect>,
    #[doc = "The time in minutes to wait before performing the desired session handling action when a user disconnects during the ramp down period."]
    #[serde(rename = "rampDownMinutesToWaitOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_minutes_to_wait_on_disconnect: Option<i32>,
    #[doc = "Action to be taken after a logoff during the ramp down period."]
    #[serde(rename = "rampDownActionOnLogoff", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_action_on_logoff: Option<scaling_plan_personal_schedule_properties::RampDownActionOnLogoff>,
    #[doc = "The time in minutes to wait before performing the desired session handling action when a user logs off during the ramp down period."]
    #[serde(rename = "rampDownMinutesToWaitOnLogoff", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_minutes_to_wait_on_logoff: Option<i32>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "offPeakStartTime", default, skip_serializing_if = "Option::is_none")]
    pub off_peak_start_time: Option<Time>,
    #[doc = "The desired configuration of Start VM On Connect for the hostpool during the off-peak phase."]
    #[serde(rename = "offPeakStartVMOnConnect", default, skip_serializing_if = "Option::is_none")]
    pub off_peak_start_vm_on_connect: Option<scaling_plan_personal_schedule_properties::OffPeakStartVmOnConnect>,
    #[doc = "Action to be taken after a user disconnect during the off-peak period."]
    #[serde(rename = "offPeakActionOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub off_peak_action_on_disconnect: Option<scaling_plan_personal_schedule_properties::OffPeakActionOnDisconnect>,
    #[doc = "The time in minutes to wait before performing the desired session handling action when a user disconnects during the off-peak period."]
    #[serde(rename = "offPeakMinutesToWaitOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub off_peak_minutes_to_wait_on_disconnect: Option<i32>,
    #[doc = "Action to be taken after a logoff during the off-peak period."]
    #[serde(rename = "offPeakActionOnLogoff", default, skip_serializing_if = "Option::is_none")]
    pub off_peak_action_on_logoff: Option<scaling_plan_personal_schedule_properties::OffPeakActionOnLogoff>,
    #[doc = "The time in minutes to wait before performing the desired session handling action when a user logs off during the off-peak period."]
    #[serde(rename = "offPeakMinutesToWaitOnLogoff", default, skip_serializing_if = "Option::is_none")]
    pub off_peak_minutes_to_wait_on_logoff: Option<i32>,
}
impl ScalingPlanPersonalScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod scaling_plan_personal_schedule_properties {
    use super::*;
    #[doc = "The desired startup behavior during the ramp up period for personal vms in the hostpool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampUpAutoStartHosts")]
    pub enum RampUpAutoStartHosts {
        None,
        WithAssignedUser,
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampUpAutoStartHosts {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampUpAutoStartHosts {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampUpAutoStartHosts {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("RampUpAutoStartHosts", 0u32, "None"),
                Self::WithAssignedUser => serializer.serialize_unit_variant("RampUpAutoStartHosts", 1u32, "WithAssignedUser"),
                Self::All => serializer.serialize_unit_variant("RampUpAutoStartHosts", 2u32, "All"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The desired configuration of Start VM On Connect for the hostpool during the ramp up phase. If this is disabled, session hosts must be turned on using rampUpAutoStartHosts or by turning them on manually."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampUpStartVmOnConnect")]
    pub enum RampUpStartVmOnConnect {
        Enable,
        Disable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampUpStartVmOnConnect {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampUpStartVmOnConnect {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampUpStartVmOnConnect {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enable => serializer.serialize_unit_variant("RampUpStartVmOnConnect", 0u32, "Enable"),
                Self::Disable => serializer.serialize_unit_variant("RampUpStartVmOnConnect", 1u32, "Disable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for RampUpStartVmOnConnect {
        fn default() -> Self {
            Self::Enable
        }
    }
    #[doc = "Action to be taken after a user disconnect during the ramp up period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampUpActionOnDisconnect")]
    pub enum RampUpActionOnDisconnect {
        None,
        Deallocate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampUpActionOnDisconnect {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampUpActionOnDisconnect {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampUpActionOnDisconnect {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("RampUpActionOnDisconnect", 0u32, "None"),
                Self::Deallocate => serializer.serialize_unit_variant("RampUpActionOnDisconnect", 1u32, "Deallocate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Action to be taken after a logoff during the ramp up period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampUpActionOnLogoff")]
    pub enum RampUpActionOnLogoff {
        None,
        Deallocate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampUpActionOnLogoff {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampUpActionOnLogoff {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampUpActionOnLogoff {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("RampUpActionOnLogoff", 0u32, "None"),
                Self::Deallocate => serializer.serialize_unit_variant("RampUpActionOnLogoff", 1u32, "Deallocate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The desired configuration of Start VM On Connect for the hostpool during the peak phase."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PeakStartVmOnConnect")]
    pub enum PeakStartVmOnConnect {
        Enable,
        Disable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PeakStartVmOnConnect {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PeakStartVmOnConnect {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PeakStartVmOnConnect {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enable => serializer.serialize_unit_variant("PeakStartVmOnConnect", 0u32, "Enable"),
                Self::Disable => serializer.serialize_unit_variant("PeakStartVmOnConnect", 1u32, "Disable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PeakStartVmOnConnect {
        fn default() -> Self {
            Self::Enable
        }
    }
    #[doc = "Action to be taken after a user disconnect during the peak period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PeakActionOnDisconnect")]
    pub enum PeakActionOnDisconnect {
        None,
        Deallocate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PeakActionOnDisconnect {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PeakActionOnDisconnect {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PeakActionOnDisconnect {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PeakActionOnDisconnect", 0u32, "None"),
                Self::Deallocate => serializer.serialize_unit_variant("PeakActionOnDisconnect", 1u32, "Deallocate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Action to be taken after a logoff during the peak period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PeakActionOnLogoff")]
    pub enum PeakActionOnLogoff {
        None,
        Deallocate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PeakActionOnLogoff {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PeakActionOnLogoff {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PeakActionOnLogoff {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PeakActionOnLogoff", 0u32, "None"),
                Self::Deallocate => serializer.serialize_unit_variant("PeakActionOnLogoff", 1u32, "Deallocate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The desired configuration of Start VM On Connect for the hostpool during the ramp down phase."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampDownStartVmOnConnect")]
    pub enum RampDownStartVmOnConnect {
        Enable,
        Disable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampDownStartVmOnConnect {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampDownStartVmOnConnect {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampDownStartVmOnConnect {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enable => serializer.serialize_unit_variant("RampDownStartVmOnConnect", 0u32, "Enable"),
                Self::Disable => serializer.serialize_unit_variant("RampDownStartVmOnConnect", 1u32, "Disable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for RampDownStartVmOnConnect {
        fn default() -> Self {
            Self::Enable
        }
    }
    #[doc = "Action to be taken after a user disconnect during the ramp down period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampDownActionOnDisconnect")]
    pub enum RampDownActionOnDisconnect {
        None,
        Deallocate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampDownActionOnDisconnect {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampDownActionOnDisconnect {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampDownActionOnDisconnect {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("RampDownActionOnDisconnect", 0u32, "None"),
                Self::Deallocate => serializer.serialize_unit_variant("RampDownActionOnDisconnect", 1u32, "Deallocate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Action to be taken after a logoff during the ramp down period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampDownActionOnLogoff")]
    pub enum RampDownActionOnLogoff {
        None,
        Deallocate,
        Hibernate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampDownActionOnLogoff {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampDownActionOnLogoff {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampDownActionOnLogoff {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("RampDownActionOnLogoff", 0u32, "None"),
                Self::Deallocate => serializer.serialize_unit_variant("RampDownActionOnLogoff", 1u32, "Deallocate"),
                Self::Hibernate => serializer.serialize_unit_variant("RampDownActionOnLogoff", 2u32, "Hibernate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The desired configuration of Start VM On Connect for the hostpool during the off-peak phase."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OffPeakStartVmOnConnect")]
    pub enum OffPeakStartVmOnConnect {
        Enable,
        Disable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OffPeakStartVmOnConnect {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OffPeakStartVmOnConnect {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OffPeakStartVmOnConnect {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enable => serializer.serialize_unit_variant("OffPeakStartVmOnConnect", 0u32, "Enable"),
                Self::Disable => serializer.serialize_unit_variant("OffPeakStartVmOnConnect", 1u32, "Disable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for OffPeakStartVmOnConnect {
        fn default() -> Self {
            Self::Enable
        }
    }
    #[doc = "Action to be taken after a user disconnect during the off-peak period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OffPeakActionOnDisconnect")]
    pub enum OffPeakActionOnDisconnect {
        None,
        Deallocate,
        Hibernate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OffPeakActionOnDisconnect {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OffPeakActionOnDisconnect {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OffPeakActionOnDisconnect {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("OffPeakActionOnDisconnect", 0u32, "None"),
                Self::Deallocate => serializer.serialize_unit_variant("OffPeakActionOnDisconnect", 1u32, "Deallocate"),
                Self::Hibernate => serializer.serialize_unit_variant("OffPeakActionOnDisconnect", 2u32, "Hibernate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Action to be taken after a logoff during the off-peak period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OffPeakActionOnLogoff")]
    pub enum OffPeakActionOnLogoff {
        None,
        Deallocate,
        Hibernate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OffPeakActionOnLogoff {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OffPeakActionOnLogoff {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OffPeakActionOnLogoff {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("OffPeakActionOnLogoff", 0u32, "None"),
                Self::Deallocate => serializer.serialize_unit_variant("OffPeakActionOnLogoff", 1u32, "Deallocate"),
                Self::Hibernate => serializer.serialize_unit_variant("OffPeakActionOnLogoff", 2u32, "Hibernate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a ScalingPlanPooledSchedule definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalingPlanPooledSchedule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "A ScalingPlanPooledSchedule."]
    pub properties: ScalingPlanPooledScheduleProperties,
}
impl ScalingPlanPooledSchedule {
    pub fn new(properties: ScalingPlanPooledScheduleProperties) -> Self {
        Self {
            resource: Resource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "List of ScalingPlanPooledSchedule definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingPlanPooledScheduleList {
    #[doc = "List of ScalingPlanPooledSchedule definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ScalingPlanPooledSchedule>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScalingPlanPooledScheduleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScalingPlanPooledScheduleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ScalingPlanPooledSchedule properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingPlanPooledSchedulePatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A ScalingPlanPooledSchedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScalingPlanPooledScheduleProperties>,
}
impl ScalingPlanPooledSchedulePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A ScalingPlanPooledSchedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingPlanPooledScheduleProperties {
    #[doc = "Set of days of the week on which this schedule is active."]
    #[serde(
        rename = "daysOfWeek",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub days_of_week: Vec<String>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "rampUpStartTime", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_start_time: Option<Time>,
    #[doc = "Load balancing algorithm for ramp up period."]
    #[serde(rename = "rampUpLoadBalancingAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_load_balancing_algorithm: Option<scaling_plan_pooled_schedule_properties::RampUpLoadBalancingAlgorithm>,
    #[doc = "Minimum host percentage for ramp up period."]
    #[serde(rename = "rampUpMinimumHostsPct", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_minimum_hosts_pct: Option<i32>,
    #[doc = "Capacity threshold for ramp up period."]
    #[serde(rename = "rampUpCapacityThresholdPct", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_capacity_threshold_pct: Option<i32>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "peakStartTime", default, skip_serializing_if = "Option::is_none")]
    pub peak_start_time: Option<Time>,
    #[doc = "Load balancing algorithm for peak period."]
    #[serde(rename = "peakLoadBalancingAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub peak_load_balancing_algorithm: Option<scaling_plan_pooled_schedule_properties::PeakLoadBalancingAlgorithm>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "rampDownStartTime", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_start_time: Option<Time>,
    #[doc = "Load balancing algorithm for ramp down period."]
    #[serde(rename = "rampDownLoadBalancingAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_load_balancing_algorithm: Option<scaling_plan_pooled_schedule_properties::RampDownLoadBalancingAlgorithm>,
    #[doc = "Minimum host percentage for ramp down period."]
    #[serde(rename = "rampDownMinimumHostsPct", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_minimum_hosts_pct: Option<i32>,
    #[doc = "Capacity threshold for ramp down period."]
    #[serde(rename = "rampDownCapacityThresholdPct", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_capacity_threshold_pct: Option<i32>,
    #[doc = "Should users be logged off forcefully from hosts."]
    #[serde(rename = "rampDownForceLogoffUsers", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_force_logoff_users: Option<bool>,
    #[doc = "Specifies when to stop hosts during ramp down period."]
    #[serde(rename = "rampDownStopHostsWhen", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_stop_hosts_when: Option<scaling_plan_pooled_schedule_properties::RampDownStopHostsWhen>,
    #[doc = "Number of minutes to wait to stop hosts during ramp down period."]
    #[serde(rename = "rampDownWaitTimeMinutes", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_wait_time_minutes: Option<i32>,
    #[doc = "Notification message for users during ramp down period."]
    #[serde(rename = "rampDownNotificationMessage", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_notification_message: Option<String>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "offPeakStartTime", default, skip_serializing_if = "Option::is_none")]
    pub off_peak_start_time: Option<Time>,
    #[doc = "Load balancing algorithm for off-peak period."]
    #[serde(rename = "offPeakLoadBalancingAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub off_peak_load_balancing_algorithm: Option<scaling_plan_pooled_schedule_properties::OffPeakLoadBalancingAlgorithm>,
}
impl ScalingPlanPooledScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod scaling_plan_pooled_schedule_properties {
    use super::*;
    #[doc = "Load balancing algorithm for ramp up period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampUpLoadBalancingAlgorithm")]
    pub enum RampUpLoadBalancingAlgorithm {
        BreadthFirst,
        DepthFirst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampUpLoadBalancingAlgorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampUpLoadBalancingAlgorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampUpLoadBalancingAlgorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BreadthFirst => serializer.serialize_unit_variant("RampUpLoadBalancingAlgorithm", 0u32, "BreadthFirst"),
                Self::DepthFirst => serializer.serialize_unit_variant("RampUpLoadBalancingAlgorithm", 1u32, "DepthFirst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Load balancing algorithm for peak period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PeakLoadBalancingAlgorithm")]
    pub enum PeakLoadBalancingAlgorithm {
        BreadthFirst,
        DepthFirst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PeakLoadBalancingAlgorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PeakLoadBalancingAlgorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PeakLoadBalancingAlgorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BreadthFirst => serializer.serialize_unit_variant("PeakLoadBalancingAlgorithm", 0u32, "BreadthFirst"),
                Self::DepthFirst => serializer.serialize_unit_variant("PeakLoadBalancingAlgorithm", 1u32, "DepthFirst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Load balancing algorithm for ramp down period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampDownLoadBalancingAlgorithm")]
    pub enum RampDownLoadBalancingAlgorithm {
        BreadthFirst,
        DepthFirst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampDownLoadBalancingAlgorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampDownLoadBalancingAlgorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampDownLoadBalancingAlgorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BreadthFirst => serializer.serialize_unit_variant("RampDownLoadBalancingAlgorithm", 0u32, "BreadthFirst"),
                Self::DepthFirst => serializer.serialize_unit_variant("RampDownLoadBalancingAlgorithm", 1u32, "DepthFirst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies when to stop hosts during ramp down period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampDownStopHostsWhen")]
    pub enum RampDownStopHostsWhen {
        ZeroSessions,
        ZeroActiveSessions,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampDownStopHostsWhen {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampDownStopHostsWhen {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampDownStopHostsWhen {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ZeroSessions => serializer.serialize_unit_variant("RampDownStopHostsWhen", 0u32, "ZeroSessions"),
                Self::ZeroActiveSessions => serializer.serialize_unit_variant("RampDownStopHostsWhen", 1u32, "ZeroActiveSessions"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Load balancing algorithm for off-peak period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OffPeakLoadBalancingAlgorithm")]
    pub enum OffPeakLoadBalancingAlgorithm {
        BreadthFirst,
        DepthFirst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OffPeakLoadBalancingAlgorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OffPeakLoadBalancingAlgorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OffPeakLoadBalancingAlgorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BreadthFirst => serializer.serialize_unit_variant("OffPeakLoadBalancingAlgorithm", 0u32, "BreadthFirst"),
                Self::DepthFirst => serializer.serialize_unit_variant("OffPeakLoadBalancingAlgorithm", 1u32, "DepthFirst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Scaling plan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalingPlanProperties {
    #[doc = "ObjectId of scaling plan. (internal use)"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Description of scaling plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "User friendly name of scaling plan."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Timezone of the scaling plan."]
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[doc = "HostPool type for desktop."]
    #[serde(rename = "hostPoolType", default, skip_serializing_if = "Option::is_none")]
    pub host_pool_type: Option<scaling_plan_properties::HostPoolType>,
    #[doc = "Exclusion tag for scaling plan."]
    #[serde(rename = "exclusionTag", default, skip_serializing_if = "Option::is_none")]
    pub exclusion_tag: Option<String>,
    #[doc = "List of ScalingPlanPooledSchedule definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub schedules: Vec<ScalingSchedule>,
    #[doc = "List of ScalingHostPoolReference definitions."]
    #[serde(
        rename = "hostPoolReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub host_pool_references: Vec<ScalingHostPoolReference>,
}
impl ScalingPlanProperties {
    pub fn new(time_zone: String) -> Self {
        Self {
            object_id: None,
            description: None,
            friendly_name: None,
            time_zone,
            host_pool_type: None,
            exclusion_tag: None,
            schedules: Vec::new(),
            host_pool_references: Vec::new(),
        }
    }
}
pub mod scaling_plan_properties {
    use super::*;
    #[doc = "HostPool type for desktop."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostPoolType")]
    pub enum HostPoolType {
        Pooled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostPoolType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostPoolType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostPoolType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pooled => serializer.serialize_unit_variant("HostPoolType", 0u32, "Pooled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for HostPoolType {
        fn default() -> Self {
            Self::Pooled
        }
    }
}
#[doc = "A ScalingPlanPooledSchedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingSchedule {
    #[doc = "Name of the ScalingPlanPooledSchedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Set of days of the week on which this schedule is active."]
    #[serde(
        rename = "daysOfWeek",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub days_of_week: Vec<String>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "rampUpStartTime", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_start_time: Option<Time>,
    #[doc = "Load balancing algorithm for ramp up period."]
    #[serde(rename = "rampUpLoadBalancingAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_load_balancing_algorithm: Option<scaling_schedule::RampUpLoadBalancingAlgorithm>,
    #[doc = "Minimum host percentage for ramp up period."]
    #[serde(rename = "rampUpMinimumHostsPct", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_minimum_hosts_pct: Option<i32>,
    #[doc = "Capacity threshold for ramp up period."]
    #[serde(rename = "rampUpCapacityThresholdPct", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_capacity_threshold_pct: Option<i32>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "peakStartTime", default, skip_serializing_if = "Option::is_none")]
    pub peak_start_time: Option<Time>,
    #[doc = "Load balancing algorithm for peak period."]
    #[serde(rename = "peakLoadBalancingAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub peak_load_balancing_algorithm: Option<scaling_schedule::PeakLoadBalancingAlgorithm>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "rampDownStartTime", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_start_time: Option<Time>,
    #[doc = "Load balancing algorithm for ramp down period."]
    #[serde(rename = "rampDownLoadBalancingAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_load_balancing_algorithm: Option<scaling_schedule::RampDownLoadBalancingAlgorithm>,
    #[doc = "Minimum host percentage for ramp down period."]
    #[serde(rename = "rampDownMinimumHostsPct", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_minimum_hosts_pct: Option<i32>,
    #[doc = "Capacity threshold for ramp down period."]
    #[serde(rename = "rampDownCapacityThresholdPct", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_capacity_threshold_pct: Option<i32>,
    #[doc = "Should users be logged off forcefully from hosts."]
    #[serde(rename = "rampDownForceLogoffUsers", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_force_logoff_users: Option<bool>,
    #[doc = "Specifies when to stop hosts during ramp down period."]
    #[serde(rename = "rampDownStopHostsWhen", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_stop_hosts_when: Option<scaling_schedule::RampDownStopHostsWhen>,
    #[doc = "Number of minutes to wait to stop hosts during ramp down period."]
    #[serde(rename = "rampDownWaitTimeMinutes", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_wait_time_minutes: Option<i32>,
    #[doc = "Notification message for users during ramp down period."]
    #[serde(rename = "rampDownNotificationMessage", default, skip_serializing_if = "Option::is_none")]
    pub ramp_down_notification_message: Option<String>,
    #[doc = "The time for a scaling action to occur."]
    #[serde(rename = "offPeakStartTime", default, skip_serializing_if = "Option::is_none")]
    pub off_peak_start_time: Option<Time>,
    #[doc = "Load balancing algorithm for off-peak period."]
    #[serde(rename = "offPeakLoadBalancingAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub off_peak_load_balancing_algorithm: Option<scaling_schedule::OffPeakLoadBalancingAlgorithm>,
}
impl ScalingSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod scaling_schedule {
    use super::*;
    #[doc = "Load balancing algorithm for ramp up period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampUpLoadBalancingAlgorithm")]
    pub enum RampUpLoadBalancingAlgorithm {
        BreadthFirst,
        DepthFirst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampUpLoadBalancingAlgorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampUpLoadBalancingAlgorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampUpLoadBalancingAlgorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BreadthFirst => serializer.serialize_unit_variant("RampUpLoadBalancingAlgorithm", 0u32, "BreadthFirst"),
                Self::DepthFirst => serializer.serialize_unit_variant("RampUpLoadBalancingAlgorithm", 1u32, "DepthFirst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Load balancing algorithm for peak period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PeakLoadBalancingAlgorithm")]
    pub enum PeakLoadBalancingAlgorithm {
        BreadthFirst,
        DepthFirst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PeakLoadBalancingAlgorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PeakLoadBalancingAlgorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PeakLoadBalancingAlgorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BreadthFirst => serializer.serialize_unit_variant("PeakLoadBalancingAlgorithm", 0u32, "BreadthFirst"),
                Self::DepthFirst => serializer.serialize_unit_variant("PeakLoadBalancingAlgorithm", 1u32, "DepthFirst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Load balancing algorithm for ramp down period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampDownLoadBalancingAlgorithm")]
    pub enum RampDownLoadBalancingAlgorithm {
        BreadthFirst,
        DepthFirst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampDownLoadBalancingAlgorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampDownLoadBalancingAlgorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampDownLoadBalancingAlgorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BreadthFirst => serializer.serialize_unit_variant("RampDownLoadBalancingAlgorithm", 0u32, "BreadthFirst"),
                Self::DepthFirst => serializer.serialize_unit_variant("RampDownLoadBalancingAlgorithm", 1u32, "DepthFirst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies when to stop hosts during ramp down period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RampDownStopHostsWhen")]
    pub enum RampDownStopHostsWhen {
        ZeroSessions,
        ZeroActiveSessions,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RampDownStopHostsWhen {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RampDownStopHostsWhen {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RampDownStopHostsWhen {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ZeroSessions => serializer.serialize_unit_variant("RampDownStopHostsWhen", 0u32, "ZeroSessions"),
                Self::ZeroActiveSessions => serializer.serialize_unit_variant("RampDownStopHostsWhen", 1u32, "ZeroActiveSessions"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Load balancing algorithm for off-peak period."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OffPeakLoadBalancingAlgorithm")]
    pub enum OffPeakLoadBalancingAlgorithm {
        BreadthFirst,
        DepthFirst,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OffPeakLoadBalancingAlgorithm {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OffPeakLoadBalancingAlgorithm {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OffPeakLoadBalancingAlgorithm {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BreadthFirst => serializer.serialize_unit_variant("OffPeakLoadBalancingAlgorithm", 0u32, "BreadthFirst"),
                Self::DepthFirst => serializer.serialize_unit_variant("OffPeakLoadBalancingAlgorithm", 1u32, "DepthFirst"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Security information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityInfoPatchProperties {
    #[doc = "The security type used by virtual machine in hostpool session host."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<security_info_patch_properties::Type>,
    #[doc = "Whether to use secureBoot on the virtual machine."]
    #[serde(rename = "secureBootEnabled", default, skip_serializing_if = "Option::is_none")]
    pub secure_boot_enabled: Option<bool>,
    #[doc = "Whether to use vTPM on the virtual machine."]
    #[serde(rename = "vTpmEnabled", default, skip_serializing_if = "Option::is_none")]
    pub v_tpm_enabled: Option<bool>,
}
impl SecurityInfoPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod security_info_patch_properties {
    use super::*;
    #[doc = "The security type used by virtual machine in hostpool session host."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Standard,
        TrustedLaunch,
        #[serde(rename = "ConfidentialVM")]
        ConfidentialVm,
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
                Self::Standard => serializer.serialize_unit_variant("Type", 0u32, "Standard"),
                Self::TrustedLaunch => serializer.serialize_unit_variant("Type", 1u32, "TrustedLaunch"),
                Self::ConfidentialVm => serializer.serialize_unit_variant("Type", 2u32, "ConfidentialVM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Security information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityInfoProperties {
    #[doc = "The security type used by virtual machine in hostpool session host. Default is Standard."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<security_info_properties::Type>,
    #[doc = "Whether to use secureBoot on the virtual machine."]
    #[serde(rename = "secureBootEnabled", default, skip_serializing_if = "Option::is_none")]
    pub secure_boot_enabled: Option<bool>,
    #[doc = "Whether to use vTPM on the virtual machine."]
    #[serde(rename = "vTpmEnabled", default, skip_serializing_if = "Option::is_none")]
    pub v_tpm_enabled: Option<bool>,
}
impl SecurityInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod security_info_properties {
    use super::*;
    #[doc = "The security type used by virtual machine in hostpool session host. Default is Standard."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Standard,
        TrustedLaunch,
        #[serde(rename = "ConfidentialVM")]
        ConfidentialVm,
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
                Self::Standard => serializer.serialize_unit_variant("Type", 0u32, "Standard"),
                Self::TrustedLaunch => serializer.serialize_unit_variant("Type", 1u32, "TrustedLaunch"),
                Self::ConfidentialVm => serializer.serialize_unit_variant("Type", 2u32, "ConfidentialVM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::Standard
        }
    }
}
#[doc = "Represents message sent to a UserSession."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SendMessage {
    #[doc = "Title of message."]
    #[serde(rename = "messageTitle", default, skip_serializing_if = "Option::is_none")]
    pub message_title: Option<String>,
    #[doc = "Body of message."]
    #[serde(rename = "messageBody", default, skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
}
impl SendMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service specification payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Specifications of the Log for Azure Monitoring"]
    #[serde(
        rename = "logSpecifications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub log_specifications: Vec<LogSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a SessionHost definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHost {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Schema for SessionHost properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SessionHostProperties>,
}
impl SessionHost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a SessionHostConfiguration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHostConfiguration {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Session host configurations of HostPool."]
    pub properties: SessionHostConfigurationProperties,
}
impl SessionHostConfiguration {
    pub fn new(properties: SessionHostConfigurationProperties) -> Self {
        Self {
            resource: Resource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "List of SessionHostConfiguration definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostConfigurationList {
    #[doc = "List of SessionHostConfiguration definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SessionHostConfiguration>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SessionHostConfigurationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SessionHostConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current status of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHostConfigurationOperationStatus {
    #[doc = "Fully qualified ID for the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status. Current defined values are <UpdateFailed | Paused | Pausing | Cancelling | InProgress | Succeeded | Failed | Canceled>"]
    pub status: String,
    #[doc = "Percent of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The operations list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<SessionHostConfigurationOperationStatus>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl SessionHostConfigurationOperationStatus {
    pub fn new(status: String) -> Self {
        Self {
            id: None,
            name: None,
            status,
            percent_complete: None,
            start_time: None,
            end_time: None,
            operations: Vec::new(),
            error: None,
        }
    }
}
#[doc = "List of OperationStatus definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostConfigurationOperationStatusList {
    #[doc = "List of OperationStatus definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SessionHostConfigurationOperationStatus>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SessionHostConfigurationOperationStatusList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SessionHostConfigurationOperationStatusList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a SessionHostConfigurationPatch definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostConfigurationPatch {
    #[doc = "Session host configurations of HostPool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SessionHostConfigurationPatchProperties>,
}
impl SessionHostConfigurationPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Session host configurations of HostPool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostConfigurationPatchProperties {
    #[doc = "Friendly name to describe this version of the SessionHostConfiguration."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Hashtable that lists key/value pair tags to apply to the VMs"]
    #[serde(rename = "vmTags", default, skip_serializing_if = "Option::is_none")]
    pub vm_tags: Option<serde_json::Value>,
    #[doc = "The Location for the session host to be created in"]
    #[serde(rename = "vmLocation", default, skip_serializing_if = "Option::is_none")]
    pub vm_location: Option<String>,
    #[doc = "The ResourceGroup for the session hosts to be created in. It will default to the ResourceGroup of the hostpool if not provided."]
    #[serde(rename = "vmResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_group: Option<String>,
    #[doc = "The prefix that should be associated with session host names"]
    #[serde(rename = "vmNamePrefix", default, skip_serializing_if = "Option::is_none")]
    pub vm_name_prefix: Option<String>,
    #[doc = "Value for availability zones to be used by the session host. Should be from [1,2,3]."]
    #[serde(
        rename = "availabilityZones",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availability_zones: Vec<i32>,
    #[doc = "Network information."]
    #[serde(rename = "networkInfo", default, skip_serializing_if = "Option::is_none")]
    pub network_info: Option<NetworkInfoProperties>,
    #[doc = "The id of the size of a virtual machine connected to a hostpool."]
    #[serde(rename = "vmSizeId", default, skip_serializing_if = "Option::is_none")]
    pub vm_size_id: Option<String>,
    #[doc = "Disk information."]
    #[serde(rename = "diskInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_info: Option<DiskInfoPatchProperties>,
    #[doc = "The uri to the storage blob containing the arm template to be run on the virtual machine after provisioning."]
    #[serde(rename = "customConfigurationScriptUrl", default, skip_serializing_if = "Option::is_none")]
    pub custom_configuration_script_url: Option<String>,
    #[doc = "Image configurations of session host in a HostPool."]
    #[serde(rename = "imageInfo", default, skip_serializing_if = "Option::is_none")]
    pub image_info: Option<ImageInfoPatchProperties>,
    #[doc = "Domain configurations of session hosts."]
    #[serde(rename = "domainInfo", default, skip_serializing_if = "Option::is_none")]
    pub domain_info: Option<DomainInfoPatchProperties>,
    #[doc = "Security information."]
    #[serde(rename = "securityInfo", default, skip_serializing_if = "Option::is_none")]
    pub security_info: Option<SecurityInfoPatchProperties>,
    #[doc = "Credentials kept in the keyvault."]
    #[serde(rename = "vmAdminCredentials", default, skip_serializing_if = "Option::is_none")]
    pub vm_admin_credentials: Option<KeyVaultCredentialsPatchProperties>,
    #[doc = "Boot Diagnostics is a debugging feature which allows you to view Console Output and Screenshot to diagnose VM status. <br><br> You can easily view the output of your console log. <br><br> Azure also enables you to see a screenshot of the VM from the hypervisor."]
    #[serde(rename = "bootDiagnosticsInfo", default, skip_serializing_if = "Option::is_none")]
    pub boot_diagnostics_info: Option<BootDiagnosticsInfoPatchProperties>,
}
impl SessionHostConfigurationPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Session host configurations of HostPool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHostConfigurationProperties {
    #[doc = "The timestamp of the last update."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub version: Option<time::OffsetDateTime>,
    #[doc = "Friendly name to describe this version of the SessionHostConfiguration."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Provisioning state of the Session Host Configuration."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<session_host_configuration_properties::ProvisioningState>,
    #[doc = "Hashtable that lists key/value pair tags to apply to the VMs"]
    #[serde(rename = "vmTags", default, skip_serializing_if = "Option::is_none")]
    pub vm_tags: Option<serde_json::Value>,
    #[doc = "The Location for the session host to be created in. It will default to the location of the hostpool if not provided."]
    #[serde(rename = "vmLocation", default, skip_serializing_if = "Option::is_none")]
    pub vm_location: Option<String>,
    #[doc = "The ResourceGroup for the session hosts to be created in. It will default to the ResourceGroup of the hostpool if not provided."]
    #[serde(rename = "vmResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_group: Option<String>,
    #[doc = "The prefix that should be associated with session host names"]
    #[serde(rename = "vmNamePrefix")]
    pub vm_name_prefix: String,
    #[doc = "Value for availability zones to be used by the session host. Should be from [1,2,3]."]
    #[serde(
        rename = "availabilityZones",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availability_zones: Vec<i32>,
    #[doc = "Network information."]
    #[serde(rename = "networkInfo")]
    pub network_info: NetworkInfoProperties,
    #[doc = "The id of the size of a virtual machine connected to a hostpool."]
    #[serde(rename = "vmSizeId")]
    pub vm_size_id: String,
    #[doc = "Disk information."]
    #[serde(rename = "diskInfo")]
    pub disk_info: DiskInfoProperties,
    #[doc = "The uri to the storage blob containing the arm template to be run on the virtual machine after provisioning."]
    #[serde(rename = "customConfigurationScriptUrl", default, skip_serializing_if = "Option::is_none")]
    pub custom_configuration_script_url: Option<String>,
    #[doc = "Image configurations of session host in a HostPool."]
    #[serde(rename = "imageInfo")]
    pub image_info: ImageInfoProperties,
    #[doc = "Domain configurations of session hosts."]
    #[serde(rename = "domainInfo")]
    pub domain_info: DomainInfoProperties,
    #[doc = "Security information."]
    #[serde(rename = "securityInfo", default, skip_serializing_if = "Option::is_none")]
    pub security_info: Option<SecurityInfoProperties>,
    #[doc = "Credentials kept in the keyvault."]
    #[serde(rename = "vmAdminCredentials")]
    pub vm_admin_credentials: KeyVaultCredentialsProperties,
    #[doc = "Boot Diagnostics is a debugging feature which allows you to view Console Output and Screenshot to diagnose VM status. <br><br> You can easily view the output of your console log. <br><br> Azure also enables you to see a screenshot of the VM from the hypervisor."]
    #[serde(rename = "bootDiagnosticsInfo", default, skip_serializing_if = "Option::is_none")]
    pub boot_diagnostics_info: Option<BootDiagnosticsInfoProperties>,
}
impl SessionHostConfigurationProperties {
    pub fn new(
        vm_name_prefix: String,
        network_info: NetworkInfoProperties,
        vm_size_id: String,
        disk_info: DiskInfoProperties,
        image_info: ImageInfoProperties,
        domain_info: DomainInfoProperties,
        vm_admin_credentials: KeyVaultCredentialsProperties,
    ) -> Self {
        Self {
            version: None,
            friendly_name: None,
            provisioning_state: None,
            vm_tags: None,
            vm_location: None,
            vm_resource_group: None,
            vm_name_prefix,
            availability_zones: Vec::new(),
            network_info,
            vm_size_id,
            disk_info,
            custom_configuration_script_url: None,
            image_info,
            domain_info,
            security_info: None,
            vm_admin_credentials,
            boot_diagnostics_info: None,
        }
    }
}
pub mod session_host_configuration_properties {
    use super::*;
    #[doc = "Provisioning state of the Session Host Configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contains details on the failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostHealthCheckFailureDetails {
    #[doc = "Failure message: hints on what is wrong and how to recover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error code corresponding for the failure."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The timestamp of the last update."]
    #[serde(rename = "lastHealthCheckDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_health_check_date_time: Option<time::OffsetDateTime>,
}
impl SessionHostHealthCheckFailureDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The report for session host information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostHealthCheckReport {
    #[doc = "Represents the name of the health check operation performed."]
    #[serde(rename = "healthCheckName", default, skip_serializing_if = "Option::is_none")]
    pub health_check_name: Option<session_host_health_check_report::HealthCheckName>,
    #[doc = "Represents the Health state of the health check we performed."]
    #[serde(rename = "healthCheckResult", default, skip_serializing_if = "Option::is_none")]
    pub health_check_result: Option<session_host_health_check_report::HealthCheckResult>,
    #[doc = "Contains details on the failure."]
    #[serde(rename = "additionalFailureDetails", default, skip_serializing_if = "Option::is_none")]
    pub additional_failure_details: Option<SessionHostHealthCheckFailureDetails>,
}
impl SessionHostHealthCheckReport {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod session_host_health_check_report {
    use super::*;
    #[doc = "Represents the name of the health check operation performed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HealthCheckName")]
    pub enum HealthCheckName {
        DomainJoinedCheck,
        DomainTrustCheck,
        #[serde(rename = "FSLogixHealthCheck")]
        FsLogixHealthCheck,
        SxSStackListenerCheck,
        UrlsAccessibleCheck,
        MonitoringAgentCheck,
        DomainReachable,
        #[serde(rename = "WebRTCRedirectorCheck")]
        WebRtcRedirectorCheck,
        SupportedEncryptionCheck,
        MetaDataServiceCheck,
        AppAttachHealthCheck,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HealthCheckName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HealthCheckName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HealthCheckName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DomainJoinedCheck => serializer.serialize_unit_variant("HealthCheckName", 0u32, "DomainJoinedCheck"),
                Self::DomainTrustCheck => serializer.serialize_unit_variant("HealthCheckName", 1u32, "DomainTrustCheck"),
                Self::FsLogixHealthCheck => serializer.serialize_unit_variant("HealthCheckName", 2u32, "FSLogixHealthCheck"),
                Self::SxSStackListenerCheck => serializer.serialize_unit_variant("HealthCheckName", 3u32, "SxSStackListenerCheck"),
                Self::UrlsAccessibleCheck => serializer.serialize_unit_variant("HealthCheckName", 4u32, "UrlsAccessibleCheck"),
                Self::MonitoringAgentCheck => serializer.serialize_unit_variant("HealthCheckName", 5u32, "MonitoringAgentCheck"),
                Self::DomainReachable => serializer.serialize_unit_variant("HealthCheckName", 6u32, "DomainReachable"),
                Self::WebRtcRedirectorCheck => serializer.serialize_unit_variant("HealthCheckName", 7u32, "WebRTCRedirectorCheck"),
                Self::SupportedEncryptionCheck => serializer.serialize_unit_variant("HealthCheckName", 8u32, "SupportedEncryptionCheck"),
                Self::MetaDataServiceCheck => serializer.serialize_unit_variant("HealthCheckName", 9u32, "MetaDataServiceCheck"),
                Self::AppAttachHealthCheck => serializer.serialize_unit_variant("HealthCheckName", 10u32, "AppAttachHealthCheck"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Represents the Health state of the health check we performed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HealthCheckResult")]
    pub enum HealthCheckResult {
        Unknown,
        HealthCheckSucceeded,
        HealthCheckFailed,
        SessionHostShutdown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HealthCheckResult {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HealthCheckResult {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HealthCheckResult {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("HealthCheckResult", 0u32, "Unknown"),
                Self::HealthCheckSucceeded => serializer.serialize_unit_variant("HealthCheckResult", 1u32, "HealthCheckSucceeded"),
                Self::HealthCheckFailed => serializer.serialize_unit_variant("HealthCheckResult", 2u32, "HealthCheckFailed"),
                Self::SessionHostShutdown => serializer.serialize_unit_variant("HealthCheckResult", 3u32, "SessionHostShutdown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of SessionHost definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostList {
    #[doc = "List of SessionHost definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SessionHost>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SessionHostList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SessionHostList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a SessionHostManagement definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHostManagement {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Session host Managements of HostPool."]
    pub properties: SessionHostManagementProperties,
}
impl SessionHostManagement {
    pub fn new(properties: SessionHostManagementProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "List of SessionHostManagement definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostManagementList {
    #[doc = "List of SessionHostManagement definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SessionHostManagement>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SessionHostManagementList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SessionHostManagementList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SessionHostManagement Operation Progress information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostManagementOperationProgress {
    #[doc = "Time that the sessionHostManagement operation was created."]
    #[serde(rename = "executionStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub execution_start_time: Option<time::OffsetDateTime>,
    #[doc = "The number of sessionHosts to be started in the sessionHostManagement operation."]
    #[serde(rename = "totalSessionHosts", default, skip_serializing_if = "Option::is_none")]
    pub total_session_hosts: Option<i32>,
    #[doc = "The number of sessionHosts in progress in the sessionHostManagement operation."]
    #[serde(rename = "sessionHostsInProgress", default, skip_serializing_if = "Option::is_none")]
    pub session_hosts_in_progress: Option<i32>,
    #[doc = "The number of sessionHosts completed in the sessionHostManagement operation."]
    #[serde(rename = "sessionHostsCompleted", default, skip_serializing_if = "Option::is_none")]
    pub session_hosts_completed: Option<i32>,
    #[doc = "The number of sessionHosts rollback failed in the sessionHostManagement operation."]
    #[serde(rename = "sessionHostsRollbackFailed", default, skip_serializing_if = "Option::is_none")]
    pub session_hosts_rollback_failed: Option<i32>,
}
impl SessionHostManagementOperationProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current status of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHostManagementOperationStatus {
    #[doc = "Fully qualified ID for the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status. Current defined values are < Scheduled | UpdatingSessionHosts | ValidatingSessionHostUpdate | Paused | Pausing | Cancelling | Resuming | Starting > | Succeeded | Failed | Canceled"]
    pub status: String,
    #[doc = "Percent of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The operations list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<SessionHostManagementOperationStatus>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    #[doc = "Properties bag to hold custom RP properties for sessionHostManagement Operation Statuses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SessionHostManagementOperationStatusProperties>,
}
impl SessionHostManagementOperationStatus {
    pub fn new(status: String) -> Self {
        Self {
            id: None,
            name: None,
            status,
            percent_complete: None,
            start_time: None,
            end_time: None,
            operations: Vec::new(),
            error: None,
            properties: None,
        }
    }
}
#[doc = "List of SessionHostManagementOperationStatus definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostManagementOperationStatusList {
    #[doc = "List of SessionHostManagementOperationStatus definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SessionHostManagementOperationStatus>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SessionHostManagementOperationStatusList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SessionHostManagementOperationStatusList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties bag to hold custom RP properties for sessionHostManagement Operation Statuses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostManagementOperationStatusProperties {
    #[doc = "The Log Analytics."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<SessionHostManagementOperationType>,
    #[doc = "Action type of the current sessionHostManagement operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<session_host_management_operation_status_properties::Action>,
    #[doc = "SessionHostManagement Operation Progress information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<SessionHostManagementOperationProgress>,
    #[doc = "The timestamp that the update is scheduled for."]
    #[serde(rename = "scheduledDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_date_time: Option<time::OffsetDateTime>,
    #[doc = "Represents a SessionHostManagement definition."]
    #[serde(rename = "sessionHostManagement", default, skip_serializing_if = "Option::is_none")]
    pub session_host_management: Option<SessionHostManagement>,
    #[doc = "The resource ID of the operation that started this process."]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}
impl SessionHostManagementOperationStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod session_host_management_operation_status_properties {
    use super::*;
    #[doc = "Action type of the current sessionHostManagement operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Start,
        Retry,
        Pause,
        Resume,
        Cancel,
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
                Self::Start => serializer.serialize_unit_variant("Action", 0u32, "Start"),
                Self::Retry => serializer.serialize_unit_variant("Action", 1u32, "Retry"),
                Self::Pause => serializer.serialize_unit_variant("Action", 2u32, "Pause"),
                Self::Resume => serializer.serialize_unit_variant("Action", 3u32, "Resume"),
                Self::Cancel => serializer.serialize_unit_variant("Action", 4u32, "Cancel"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SessionHostManagementOperationType")]
pub enum SessionHostManagementOperationType {
    InitiateSessionHostUpdate,
    ValidateSessionHostUpdate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SessionHostManagementOperationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SessionHostManagementOperationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SessionHostManagementOperationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InitiateSessionHostUpdate => {
                serializer.serialize_unit_variant("SessionHostManagementOperationType", 0u32, "InitiateSessionHostUpdate")
            }
            Self::ValidateSessionHostUpdate => {
                serializer.serialize_unit_variant("SessionHostManagementOperationType", 1u32, "ValidateSessionHostUpdate")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a SessionHostManagementPatch definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostManagementPatch {
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Session host Managements of HostPool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SessionHostManagementPatchProperties>,
}
impl SessionHostManagementPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Session host Managements of HostPool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostManagementPatchProperties {
    #[doc = "Time zone for sessionHostManagement operations as defined in https://docs.microsoft.com/dotnet/api/system.timezoneinfo.findsystemtimezonebyid. Must be set if useLocalTime is true."]
    #[serde(rename = "scheduledDateTimeZone", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_date_time_zone: Option<String>,
    #[doc = "The configurations of a hostpool update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<HostPoolUpdateConfigurationPatchProperties>,
}
impl SessionHostManagementPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SessionHostManagement operation fault information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostManagementProgressError {
    #[doc = "SessionHostManagement operation fault type."]
    #[serde(rename = "faultType", default, skip_serializing_if = "Option::is_none")]
    pub fault_type: Option<session_host_management_progress_error::FaultType>,
    #[doc = "SessionHostManagement operation fault code."]
    #[serde(rename = "faultCode", default, skip_serializing_if = "Option::is_none")]
    pub fault_code: Option<String>,
    #[doc = "SessionHostManagement operation fault text."]
    #[serde(rename = "faultText", default, skip_serializing_if = "Option::is_none")]
    pub fault_text: Option<String>,
    #[doc = "SessionHostManagement operation fault context."]
    #[serde(rename = "faultContext", default, skip_serializing_if = "Option::is_none")]
    pub fault_context: Option<String>,
}
impl SessionHostManagementProgressError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod session_host_management_progress_error {
    use super::*;
    #[doc = "SessionHostManagement operation fault type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FaultType")]
    pub enum FaultType {
        ServiceError,
        UserError,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FaultType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FaultType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FaultType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ServiceError => serializer.serialize_unit_variant("FaultType", 0u32, "ServiceError"),
                Self::UserError => serializer.serialize_unit_variant("FaultType", 1u32, "UserError"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Session host Managements of HostPool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHostManagementProperties {
    #[doc = "Time zone for sessionHostManagement operations as defined in https://docs.microsoft.com/dotnet/api/system.timezoneinfo.findsystemtimezonebyid. Must be set if useLocalTime is true."]
    #[serde(rename = "scheduledDateTimeZone")]
    pub scheduled_date_time_zone: String,
    #[doc = "The configurations of a hostpool update."]
    pub update: HostPoolUpdateConfigurationProperties,
}
impl SessionHostManagementProperties {
    pub fn new(scheduled_date_time_zone: String, update: HostPoolUpdateConfigurationProperties) -> Self {
        Self {
            scheduled_date_time_zone,
            update,
        }
    }
}
#[doc = "SessionHost properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostPatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "SessionHost properties that can be patched."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SessionHostPatchProperties>,
}
impl SessionHostPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SessionHost properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostPatchProperties {
    #[doc = "Allow a new session."]
    #[serde(rename = "allowNewSession", default, skip_serializing_if = "Option::is_none")]
    pub allow_new_session: Option<bool>,
    #[doc = "User assigned to SessionHost."]
    #[serde(rename = "assignedUser", default, skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<String>,
    #[doc = "Friendly name of SessionHost"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl SessionHostPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for SessionHost properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionHostProperties {
    #[doc = "ObjectId of SessionHost. (internal use)"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Last heart beat from SessionHost."]
    #[serde(rename = "lastHeartBeat", default, with = "azure_core::date::rfc3339::option")]
    pub last_heart_beat: Option<time::OffsetDateTime>,
    #[doc = "Number of sessions on SessionHost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<i32>,
    #[doc = "Version of agent on SessionHost."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Allow a new session."]
    #[serde(rename = "allowNewSession", default, skip_serializing_if = "Option::is_none")]
    pub allow_new_session: Option<bool>,
    #[doc = "Virtual Machine Id of SessionHost's underlying virtual machine."]
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[doc = "Resource Id of SessionHost's underlying virtual machine."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "User assigned to SessionHost."]
    #[serde(rename = "assignedUser", default, skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<String>,
    #[doc = "Friendly name of SessionHost"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Status for a SessionHost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<session_host_properties::Status>,
    #[doc = "The timestamp of the status."]
    #[serde(rename = "statusTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub status_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The version of the OS on the session host."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "The version of the side by side stack on the session host."]
    #[serde(rename = "sxSStackVersion", default, skip_serializing_if = "Option::is_none")]
    pub sx_s_stack_version: Option<String>,
    #[doc = "Update state of a SessionHost."]
    #[serde(rename = "updateState", default, skip_serializing_if = "Option::is_none")]
    pub update_state: Option<session_host_properties::UpdateState>,
    #[doc = "The timestamp of the last update."]
    #[serde(rename = "lastUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_update_time: Option<time::OffsetDateTime>,
    #[doc = "The error message."]
    #[serde(rename = "updateErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub update_error_message: Option<String>,
    #[doc = "The last time update was completed."]
    #[serde(rename = "lastSessionHostUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_session_host_update_time: Option<time::OffsetDateTime>,
    #[doc = "SessionHostConfiguration version reference at the time the update is initiated, in the format of date time."]
    #[serde(rename = "sessionHostConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub session_host_configuration: Option<String>,
    #[doc = "List of SessionHostHealthCheckReports"]
    #[serde(
        rename = "sessionHostHealthCheckResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub session_host_health_check_results: Vec<SessionHostHealthCheckReport>,
}
impl SessionHostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod session_host_properties {
    use super::*;
    #[doc = "Status for a SessionHost."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Available,
        Unavailable,
        Shutdown,
        Disconnected,
        Upgrading,
        UpgradeFailed,
        NoHeartbeat,
        NotJoinedToDomain,
        DomainTrustRelationshipLost,
        SxSStackListenerNotReady,
        #[serde(rename = "FSLogixNotHealthy")]
        FsLogixNotHealthy,
        NeedsAssistance,
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
                Self::Available => serializer.serialize_unit_variant("Status", 0u32, "Available"),
                Self::Unavailable => serializer.serialize_unit_variant("Status", 1u32, "Unavailable"),
                Self::Shutdown => serializer.serialize_unit_variant("Status", 2u32, "Shutdown"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 3u32, "Disconnected"),
                Self::Upgrading => serializer.serialize_unit_variant("Status", 4u32, "Upgrading"),
                Self::UpgradeFailed => serializer.serialize_unit_variant("Status", 5u32, "UpgradeFailed"),
                Self::NoHeartbeat => serializer.serialize_unit_variant("Status", 6u32, "NoHeartbeat"),
                Self::NotJoinedToDomain => serializer.serialize_unit_variant("Status", 7u32, "NotJoinedToDomain"),
                Self::DomainTrustRelationshipLost => serializer.serialize_unit_variant("Status", 8u32, "DomainTrustRelationshipLost"),
                Self::SxSStackListenerNotReady => serializer.serialize_unit_variant("Status", 9u32, "SxSStackListenerNotReady"),
                Self::FsLogixNotHealthy => serializer.serialize_unit_variant("Status", 10u32, "FSLogixNotHealthy"),
                Self::NeedsAssistance => serializer.serialize_unit_variant("Status", 11u32, "NeedsAssistance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Update state of a SessionHost."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UpdateState")]
    pub enum UpdateState {
        Initial,
        Pending,
        Started,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UpdateState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UpdateState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UpdateState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Initial => serializer.serialize_unit_variant("UpdateState", 0u32, "Initial"),
                Self::Pending => serializer.serialize_unit_variant("UpdateState", 1u32, "Pending"),
                Self::Started => serializer.serialize_unit_variant("UpdateState", 2u32, "Started"),
                Self::Succeeded => serializer.serialize_unit_variant("UpdateState", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("UpdateState", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource model definition representing SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU. Ex - P3. It is typically a letter+number code"]
    pub name: String,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
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
impl Sku {
    pub fn new(name: String) -> Self {
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
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Basic,
        Standard,
        Premium,
    }
}
#[doc = "Represents a StartMenuItem definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartMenuItem {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Schema for StartMenuItem properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StartMenuItemProperties>,
}
impl StartMenuItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of StartMenuItem definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartMenuItemList {
    #[doc = "List of StartMenuItem definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<StartMenuItem>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StartMenuItemList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StartMenuItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for StartMenuItem properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartMenuItemProperties {
    #[doc = "Alias of StartMenuItem."]
    #[serde(rename = "appAlias", default, skip_serializing_if = "Option::is_none")]
    pub app_alias: Option<String>,
    #[doc = "Path to the file of StartMenuItem."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "Command line arguments for StartMenuItem."]
    #[serde(rename = "commandLineArguments", default, skip_serializing_if = "Option::is_none")]
    pub command_line_arguments: Option<String>,
    #[doc = "Path to the icon."]
    #[serde(rename = "iconPath", default, skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[doc = "Index of the icon."]
    #[serde(rename = "iconIndex", default, skip_serializing_if = "Option::is_none")]
    pub icon_index: Option<i32>,
}
impl StartMenuItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The time for a scaling action to occur."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Time {
    #[doc = "The hour."]
    pub hour: i32,
    #[doc = "The minute."]
    pub minute: i32,
}
impl Time {
    pub fn new(hour: i32, minute: i32) -> Self {
        Self { hour, minute }
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
#[doc = "Object containing the definition for properties to be used for a sessionHostUpdate operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSessionHostsRequestBody {
    #[doc = "The timestamp that the update validation is scheduled for. If none is provided, the update will be executed immediately"]
    #[serde(rename = "scheduledDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_date_time: Option<time::OffsetDateTime>,
    #[doc = "The timeZone as defined in https://docs.microsoft.com/dotnet/api/system.timezoneinfo.findsystemtimezonebyid."]
    #[serde(rename = "scheduledDateTimeZone", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_date_time_zone: Option<String>,
    #[doc = "The configurations of a hostpool update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<HostPoolUpdateConfigurationPatchProperties>,
}
impl UpdateSessionHostsRequestBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a UserSession definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSession {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Schema for UserSession properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserSessionProperties>,
}
impl UserSession {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of UserSession definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSessionList {
    #[doc = "List of UserSession definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<UserSession>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UserSessionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl UserSessionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for UserSession properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSessionProperties {
    #[doc = "ObjectId of user session. (internal use)"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The user principal name."]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[doc = "Application type of application."]
    #[serde(rename = "applicationType", default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<user_session_properties::ApplicationType>,
    #[doc = "State of user session."]
    #[serde(rename = "sessionState", default, skip_serializing_if = "Option::is_none")]
    pub session_state: Option<user_session_properties::SessionState>,
    #[doc = "The active directory user name."]
    #[serde(rename = "activeDirectoryUserName", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_user_name: Option<String>,
    #[doc = "The timestamp of the user session create."]
    #[serde(rename = "createTime", default, with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
}
impl UserSessionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod user_session_properties {
    use super::*;
    #[doc = "Application type of application."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApplicationType")]
    pub enum ApplicationType {
        RemoteApp,
        Desktop,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApplicationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApplicationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApplicationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RemoteApp => serializer.serialize_unit_variant("ApplicationType", 0u32, "RemoteApp"),
                Self::Desktop => serializer.serialize_unit_variant("ApplicationType", 1u32, "Desktop"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "State of user session."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SessionState")]
    pub enum SessionState {
        Unknown,
        Active,
        Disconnected,
        Pending,
        LogOff,
        UserProfileDiskMounted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SessionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SessionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SessionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("SessionState", 0u32, "Unknown"),
                Self::Active => serializer.serialize_unit_variant("SessionState", 1u32, "Active"),
                Self::Disconnected => serializer.serialize_unit_variant("SessionState", 2u32, "Disconnected"),
                Self::Pending => serializer.serialize_unit_variant("SessionState", 3u32, "Pending"),
                Self::LogOff => serializer.serialize_unit_variant("SessionState", 4u32, "LogOff"),
                Self::UserProfileDiskMounted => serializer.serialize_unit_variant("SessionState", 5u32, "UserProfileDiskMounted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Object containing the definition for properties to be used in the sessionHostUpdate validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateSessionHostUpdateRequestBody {
    #[doc = "Represents a SessionHostConfiguration definition."]
    #[serde(rename = "sessionHostConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub session_host_configuration: Option<SessionHostConfiguration>,
    #[doc = "Represents a SessionHostManagement definition."]
    #[serde(rename = "sessionHostManagement", default, skip_serializing_if = "Option::is_none")]
    pub session_host_management: Option<SessionHostManagement>,
    #[doc = "The timestamp that the update validation is scheduled for. If none is provided, the update will be executed immediately"]
    #[serde(rename = "scheduledDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_date_time: Option<time::OffsetDateTime>,
}
impl ValidateSessionHostUpdateRequestBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Workspace definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workspace {
    #[serde(flatten)]
    pub resource_model_with_allowed_property_set: ResourceModelWithAllowedPropertySet,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Schema for Workspace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Workspace definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceList {
    #[doc = "List of Workspace definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Workspace>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePatch {
    #[doc = "tags to be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Workspace properties that can be patched."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePatchProperties>,
}
impl WorkspacePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace properties that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePatchProperties {
    #[doc = "Description of Workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly name of Workspace."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "List of applicationGroup links."]
    #[serde(
        rename = "applicationGroupReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub application_group_references: Vec<String>,
    #[doc = "Enabled to allow this resource to be access from the public network"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<workspace_patch_properties::PublicNetworkAccess>,
}
impl WorkspacePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_patch_properties {
    use super::*;
    #[doc = "Enabled to allow this resource to be access from the public network"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema for Workspace properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceProperties {
    #[doc = "ObjectId of Workspace. (internal use)"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Description of Workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly name of Workspace."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "List of applicationGroup resource Ids."]
    #[serde(
        rename = "applicationGroupReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub application_group_references: Vec<String>,
    #[doc = "Is cloud pc resource."]
    #[serde(rename = "cloudPcResource", default, skip_serializing_if = "Option::is_none")]
    pub cloud_pc_resource: Option<bool>,
    #[doc = "Enabled allows this resource to be accessed from both public and private networks, Disabled allows this resource to only be accessed via private endpoints"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<workspace_properties::PublicNetworkAccess>,
    #[doc = "List of private endpoint connection associated with the specified resource"]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl WorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_properties {
    use super::*;
    #[doc = "Enabled allows this resource to be accessed from both public and private networks, Disabled allows this resource to only be accessed via private endpoints"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
