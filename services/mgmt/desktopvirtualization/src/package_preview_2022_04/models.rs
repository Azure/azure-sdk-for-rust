#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
    #[serde(rename = "maintenanceWindows", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "maintenanceWindows", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationGroup>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationGroupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "Properties for arm migration."]
    #[serde(rename = "migrationRequest", default, skip_serializing_if = "Option::is_none")]
    pub migration_request: Option<MigrationRequestProperties>,
    #[doc = "Is cloud pc resource."]
    #[serde(rename = "cloudPcResource", default, skip_serializing_if = "Option::is_none")]
    pub cloud_pc_resource: Option<bool>,
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
            migration_request: None,
            cloud_pc_resource: None,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Desktop>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DesktopList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExpandMsixImage>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExpandMsixImageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "packageDependencies", default, skip_serializing_if = "Vec::is_empty")]
    pub package_dependencies: Vec<MsixPackageDependencies>,
    #[doc = "Package Version found in the appxmanifest.xml. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Date Package was last updated, found in the appxmanifest.xml. "]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "List of package applications. "]
    #[serde(rename = "packageApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub package_applications: Vec<MsixPackageApplications>,
}
impl ExpandMsixImageProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "List of HostPool definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostPoolList {
    #[doc = "List of HostPool definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HostPool>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HostPoolList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "List of applicationGroup links."]
    #[serde(rename = "applicationGroupReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_group_references: Vec<String>,
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
    #[doc = "Properties for arm migration."]
    #[serde(rename = "migrationRequest", default, skip_serializing_if = "Option::is_none")]
    pub migration_request: Option<MigrationRequestProperties>,
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
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
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
            application_group_references: Vec::new(),
            ssoadfs_authority: None,
            sso_client_id: None,
            sso_client_secret_key_vault_path: None,
            sso_secret_type: None,
            preferred_app_group_type,
            start_vm_on_connect: None,
            migration_request: None,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MsixPackage>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MsixPackageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "packageDependencies", default, skip_serializing_if = "Vec::is_empty")]
    pub package_dependencies: Vec<MsixPackageDependencies>,
    #[doc = "Package Version found in the appxmanifest.xml. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Date Package was last updated, found in the appxmanifest.xml. "]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "List of package applications. "]
    #[serde(rename = "packageApplications", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Properties for arm migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationRequestProperties {
    #[doc = "The type of operation for migration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<migration_request_properties::Operation>,
    #[doc = "The path to the legacy object to migrate."]
    #[serde(rename = "migrationPath", default, skip_serializing_if = "Option::is_none")]
    pub migration_path: Option<String>,
}
impl MigrationRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migration_request_properties {
    use super::*;
    #[doc = "The type of operation for migration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operation")]
    pub enum Operation {
        Start,
        Revoke,
        Complete,
        Hide,
        Unhide,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Start => serializer.serialize_unit_variant("Operation", 0u32, "Start"),
                Self::Revoke => serializer.serialize_unit_variant("Operation", 1u32, "Revoke"),
                Self::Complete => serializer.serialize_unit_variant("Operation", 2u32, "Complete"),
                Self::Hide => serializer.serialize_unit_variant("Operation", 3u32, "Hide"),
                Self::Unhide => serializer.serialize_unit_variant("Operation", 4u32, "Unhide"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnectionWithSystemData>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResultWithSystemData {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceProviderOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScalingPlan>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScalingPlanList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedules: Vec<ScalingSchedule>,
    #[doc = "List of ScalingHostPoolReference definitions."]
    #[serde(rename = "hostPoolReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub host_pool_references: Vec<ScalingHostPoolReference>,
}
impl ScalingPlanPatchProperties {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScalingPlanPooledSchedule>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScalingPlanPooledScheduleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "daysOfWeek", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedules: Vec<ScalingSchedule>,
    #[doc = "List of ScalingHostPoolReference definitions."]
    #[serde(rename = "hostPoolReferences", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "daysOfWeek", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SessionHost>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SessionHostList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SessionHostList {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "List of SessionHostHealthCheckReports"]
    #[serde(rename = "sessionHostHealthCheckResults", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StartMenuItem>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StartMenuItemList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UserSession>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UserSessionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "applicationGroupReferences", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "applicationGroupReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub application_group_references: Vec<String>,
    #[doc = "Is cloud pc resource."]
    #[serde(rename = "cloudPcResource", default, skip_serializing_if = "Option::is_none")]
    pub cloud_pc_resource: Option<bool>,
    #[doc = "Enabled allows this resource to be accessed from both public and private networks, Disabled allows this resource to only be accessed via private endpoints"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<workspace_properties::PublicNetworkAccess>,
    #[doc = "List of private endpoint connection associated with the specified resource"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
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
